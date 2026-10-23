//! Opt-in measurements and complete recorded-scene differential checks.
//! Fixtures and machine-specific results remain outside the repository.
use super::*;
use crate::{
    arm_audio_hle, load_native, make_machine, run_frame_native, InputScript, FRAME_STEP_GUARD,
};
use std::time::Instant;

#[derive(serde::Serialize)]
struct StartupObservation {
    cached: bool,
    source_io_ms: f64,
    source_decode_ms: f64,
    preparation_ms: f64,
    activation_ms: f64,
    total_after_source_read_ms: f64,
    resident_bytes: usize,
    generation: String,
}

#[test]
#[ignore = "requires a private ROM and a new FFTA_STATE_TEST_COST_ROOT output directory"]
fn measure_decoded_startup_costs() {
    let rom_path = std::env::var_os("FFTA_STATE_TEST_ROM").expect("ROM fixture");
    let root = PathBuf::from(std::env::var_os("FFTA_STATE_TEST_COST_ROOT").expect("cost output"));
    assert!(
        !root.exists(),
        "preserve earlier measurements; select a new output root"
    );
    fs::create_dir_all(&root).unwrap();
    let mut observations = Vec::new();
    for index in 0..7 {
        let dataset_root = root.join(format!("sample-{index}"));
        for cached in [false, true] {
            let io_start = Instant::now();
            let rom = fs::read(&rom_path).unwrap();
            let source_io_ms = io_start.elapsed().as_secs_f64() * 1000.0;
            // ROM allocation and machine construction belong to both paths.
            let mut machine = make_machine(rom, None);
            let start = Instant::now();
            let mut source = Extraction::from_rom(&machine.bus.rom).unwrap();
            let decoded_at = start.elapsed();
            let prepared =
                preparation::prepare(&dataset_root, &mut source, &mut |_| Ok(())).unwrap();
            let prepared_at = start.elapsed();
            assert_eq!(prepared.action, if cached { "reused" } else { "created" });
            attach_from_directory(&mut machine, &prepared.directory).unwrap();
            let activated_at = start.elapsed();
            let resources = machine.bus.rom_resources().unwrap();
            observations.push(StartupObservation {
                cached,
                source_io_ms,
                source_decode_ms: decoded_at.as_secs_f64() * 1000.0,
                preparation_ms: (prepared_at - decoded_at).as_secs_f64() * 1000.0,
                activation_ms: (activated_at - prepared_at).as_secs_f64() * 1000.0,
                total_after_source_read_ms: activated_at.as_secs_f64() * 1000.0,
                resident_bytes: resources.resident_bytes() + 2 * std::mem::size_of::<usize>(),
                generation: prepared
                    .directory
                    .strip_prefix(&root)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
            });
        }
    }
    #[derive(serde::Serialize)]
    struct Report {
        observations: Vec<StartupObservation>,
    }
    // Save every observation, including slower results; evaluate budgets separately.
    fs::write(
        root.join("startup.toml"),
        toml::to_string_pretty(&Report { observations }).unwrap(),
    )
    .unwrap();
}

#[test]
#[ignore = "requires private ROM/native/input fixtures and explicit scene length; optional state/save"]
fn decoded_base_matches_recorded_scene_at_every_frame() {
    crate::configure_headless_widescreen();
    let rom = fs::read(std::env::var_os("FFTA_STATE_TEST_ROM").expect("ROM fixture")).unwrap();
    let library = std::env::var_os("FFTA_STATE_TEST_NATIVE").expect("native fixture");
    let state = std::env::var_os("FFTA_STATE_TEST_STATE");
    let save = std::env::var_os("FFTA_STATE_TEST_SAVE");
    assert!(
        !(state.is_some() && save.is_some()),
        "choose a snapshot or cartridge save"
    );
    let input_path = std::env::var("FFTA_STATE_TEST_INPUT").expect("input fixture");
    let frames: u64 = std::env::var("FFTA_STATE_TEST_FRAMES")
        .expect("explicit frame count")
        .parse()
        .unwrap();
    assert!(frames > 0);
    let input = InputScript::load(&input_path).unwrap();
    let (_library, table) = load_native(Path::new(&library)).unwrap();
    let mut legacy = make_machine(rom.clone(), None);
    let mut decoded = make_machine(rom.clone(), None);
    let directory = tempfile::tempdir().unwrap();
    let mut source = Extraction::from_rom(&rom).unwrap();
    let dataset = match std::env::var_os("FFTA_STATE_TEST_DATASET") {
        Some(path) => PathBuf::from(path),
        None => {
            let dataset = directory.path().join("decoded");
            extract(&dataset, &mut source).unwrap();
            dataset
        }
    };
    attach_from_directory(&mut decoded, &dataset).unwrap();
    for machine in [&mut legacy, &mut decoded] {
        if let Some(path) = &state {
            crate::save_state::load(machine, Path::new(path)).unwrap();
        }
        if let Some(path) = &save {
            machine.bus.load_save_data(&fs::read(path).unwrap());
        }
    }
    // Compare the unmodified snapshot before configuring audio observation.
    assert_eq!(legacy.save_state().unwrap(), decoded.save_state().unwrap());
    for machine in [&mut legacy, &mut decoded] {
        // Loading a snapshot can overwrite these host shadows. Arm identically
        // AFTER loading so a zero-length HLE tap cannot masquerade as coverage.
        arm_audio_hle(machine, None);
        machine.bus.tap_channels = true;
        clear_audio(machine);
    }
    assert_eq!(legacy.save_state().unwrap(), decoded.save_state().unwrap());
    let alternate_mode = std::env::var("FFTA_STATE_TEST_ALTERNATE_MODE")
        .ok()
        .map(|value| value.parse::<u8>().unwrap());
    if let Some(mode) = alternate_mode {
        assert!(matches!(mode, 1 | 2), "select an original alternate Color Mode");
        assert_eq!(decoded.bus.ewram[0x2fc2] & 3, mode, "loaded Color Mode");
    }
    let first = legacy.bus.frames;
    let mut dispatch = [0u64; 2];
    let mut audio = [0u64; 5];
    let mut source_mask = 0;
    let mut first_alternate_read = None;
    for relative in 0..frames {
        let mut counts = [(0, 0); 2];
        for (index, machine) in [&mut legacy, &mut decoded].into_iter().enumerate() {
            machine.bus.keys = input.keys_at(relative);
            input.apply_mouse(&mut machine.bus, relative).unwrap();
            let pointer = machine as *mut gba_core::Machine as *mut std::ffi::c_void;
            counts[index] = run_frame_native(machine, &table, pointer, FRAME_STEP_GUARD);
            assert_eq!(machine.bus.frames, first + relative + 1, "incomplete frame");
            assert!(machine.bus.frame_ready);
        }
        assert_eq!(counts[0], counts[1], "dispatch at {relative}");
        dispatch[0] += counts[0].0;
        dispatch[1] += counts[0].1;
        assert_eq!(
            legacy.bus.framebuffer, decoded.bus.framebuffer,
            "central pixels at {relative}"
        );
        assert_eq!(
            legacy.bus.wide_framebuffer, decoded.bus.wide_framebuffer,
            "wide pixels at {relative}"
        );
        assert_eq!(legacy.bus.oam, decoded.bus.oam, "OAM at {relative}");
        assert_eq!(
            legacy.bus.audio_buf, decoded.bus.audio_buf,
            "mixed audio at {relative}"
        );
        assert_eq!(legacy.bus.psg_tap, decoded.bus.psg_tap, "PSG at {relative}");
        assert_eq!(
            legacy.bus.fifo_tap, decoded.bus.fifo_tap,
            "FIFO at {relative}"
        );
        assert_eq!(legacy.bus.hle_tap.len(), decoded.bus.hle_tap.len());
        assert!(
            legacy
                .bus
                .hle_tap
                .iter()
                .zip(&decoded.bus.hle_tap)
                .all(|(a, b)| a.to_bits() == b.to_bits()),
            "HLE at {relative}"
        );
        assert!(
            legacy.save_state().unwrap() == decoded.save_state().unwrap(),
            "full state at {relative}"
        );
        if let Some(mode) = alternate_mode {
            assert_eq!(decoded.bus.ewram[0x2fc2] & 3, mode, "Color Mode at {relative}");
            let alternate = decoded.bus.rom_resources().unwrap().alternate_palette_stats().unwrap();
            if alternate.decoded_literal_bytes > 0 && first_alternate_read.is_none() {
                let id = u16::from_le_bytes(decoded.bus.ewram[0x7f10..0x7f12].try_into().unwrap());
                first_alternate_read = Some((relative, decoded.bus.frames, id));
            }
        }
        for (sum, len) in audio.iter_mut().zip([
            decoded.bus.audio_buf.len(),
            decoded.bus.psg_tap.len(),
            decoded.bus.fifo_tap[0].len(),
            decoded.bus.fifo_tap[1].len(),
            decoded.bus.hle_tap.len(),
        ]) {
            *sum += len as u64;
        }
        let mask = decoded.bus.rom_resources().unwrap().stats().bus_source_mask;
        if mask != source_mask {
            eprintln!(
                "ROM_DATA_SOURCE first_frame={} clock={} mask={mask}",
                decoded.bus.frames, decoded.bus.clock
            );
            source_mask = mask;
        }
        if relative % 500 == 499 {
            eprintln!("ROM_DATA_SCENE_PROGRESS frames={}/{}", relative + 1, frames);
        }
        clear_audio(&mut legacy);
        clear_audio(&mut decoded);
    }
    assert!(dispatch[0] > 0, "native dispatch must execute");
    assert!(
        audio.iter().all(|&count| count > 0),
        "all audio streams need actual samples: {audio:?}"
    );
    let stats = decoded.bus.rom_resources().unwrap().stats();
    let bg = decoded
        .bus
        .rom_resources()
        .unwrap()
        .background_stats()
        .unwrap();
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_BG").is_some() {
        assert!(
            bg.bus_reads > 0,
            "this scene must exercise BG metadata reads"
        );
    }
    let palette = decoded
        .bus
        .rom_resources()
        .unwrap()
        .palette_stats()
        .unwrap();
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_PALETTE").is_some() {
        assert!(
            palette.lz77_calls > 0 && palette.decoded_bytes > 0,
            "this scene must consume decoded palette banks"
        );
    }
    eprintln!("ROM_DATA_PALETTE_SCENE frames={frames} bus_reads={} source_bytes={} lz77_calls={} decoded_bytes={}", palette.bus_reads, palette.source_bytes, palette.lz77_calls, palette.decoded_bytes);
    if let Some(mode) = alternate_mode {
        let alternate = decoded.bus.rom_resources().unwrap().alternate_palette_stats().unwrap();
        assert!(alternate.table_bytes > 0 && alternate.decoded_literal_bytes > 0,
            "this scene must read alternate links and decoded literal banks");
        let (relative, absolute, id) = first_alternate_read.expect("an actual alternate scene");
        eprintln!("ROM_DATA_ALTERNATE_SCENE mode={mode} frames={frames} first_relative={relative} first_absolute={absolute} background_id={id} bus_reads={} source_bytes={} table_bytes={} decoded_literal_bytes={}", alternate.bus_reads, alternate.source_bytes, alternate.table_bytes, alternate.decoded_literal_bytes);
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_PACKET").is_some() {
        let packet = decoded.bus.rom_resources().unwrap().packet_stats().expect("decoded EUR packets");
        assert!(packet.typed_value_bytes + packet.typed_hle_bytes > 0,
            "this scene must consume decoded packet values");
        eprintln!("ROM_DATA_PACKET_SCENE frames={frames} bus_reads={} source_bytes={} typed_value_bytes={} lz77_calls={} decoded_body_bytes={} typed_hle_bytes={}", packet.bus_reads, packet.source_bytes, packet.typed_value_bytes, packet.lz77_calls, packet.decoded_body_bytes, packet.typed_hle_bytes);
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_GRAPHICS").is_some() {
        let graphics = decoded.bus.rom_resources().unwrap().graphics_stats().expect("decoded EUR graphics");
        assert!(graphics.decoded_literal_bytes > 0, "this scene must consume decoded main graphics words");
        eprintln!("ROM_DATA_GRAPHICS_SCENE frames={frames} bus_reads={} source_bytes={} decoded_literal_bytes={}",
            graphics.bus_reads, graphics.source_bytes, graphics.decoded_literal_bytes);
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_TRANSFER").is_some() {
        let transfer = decoded.bus.rom_resources().unwrap().transfer_stats().expect("decoded EUR transfers");
        assert!(transfer.control_bytes > 0 && transfer.tile_bytes > 0, "this scene must use typed transfer controls and tile words");
        eprintln!("ROM_DATA_TRANSFER_SCENE frames={frames} bus_reads={} source_bytes={} control_bytes={} tile_bytes={}",
            transfer.bus_reads, transfer.source_bytes, transfer.control_bytes, transfer.tile_bytes);
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_TEXT_CATALOG").is_some() {
        let t = decoded.bus.rom_resources().unwrap().text_catalog_stats().expect("decoded EUR text catalogs");
        let complete_font = decoded.bus.rom_resources().unwrap().font_keyboard_stats();
        assert!(t.archive_index_bytes > 0 && t.decoded_program_bytes > 0
            && t.glyph_bytes + complete_font.map_or(0, |f| f.glyph_bytes) > 0,
            "this scene must consume typed archive programs and font words");
        eprintln!("ROM_DATA_TEXT_CATALOG_SCENE frames={frames} bus_reads={} source_bytes={} pointer_table_bytes={} string_bytes={} archive_index_bytes={} header_bytes={} encoded_control_bytes={} decoded_program_bytes={} glyph_bytes={} width_bytes={}",
            t.bus_reads,t.source_bytes,t.pointer_table_bytes,t.string_bytes,t.archive_index_bytes,t.header_bytes,t.encoded_control_bytes,t.decoded_program_bytes,t.glyph_bytes,t.width_bytes);
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_COMPLETE_FONT").is_some() {
        let f = decoded.bus.rom_resources().unwrap().font_keyboard_stats().expect("decoded complete EUR font");
        assert!(f.glyph_bytes > 0 && f.width_bytes > 0, "this scene must consume complete-bank glyphs and metrics");
        eprintln!("ROM_DATA_FONT_KEYBOARD_SCENE frames={frames} bus_reads={} glyph_bytes={} width_bytes={} index_bytes={} program_bytes={} row_width_bytes={}",
            f.bus_reads,f.glyph_bytes,f.width_bytes,f.keyboard_index_bytes,f.keyboard_program_bytes,f.keyboard_row_width_bytes);
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_OBJ_LISTS").is_some() {
        let l = decoded.bus.rom_resources().unwrap().obj_list_stats().expect("decoded EUR static OBJ lists");
        assert!(l.count_bytes > 0 && l.attribute_bytes > 0 && l.host_reads > 0,
            "this scene must consume file-derived OBJ counts/attributes in guest and host paths");
        eprintln!("ROM_DATA_OBJ_LIST_SCENE frames={frames} bus_reads={} source_bytes={} count_bytes={} attribute_bytes={} host_reads={}",
            l.bus_reads, l.source_bytes, l.count_bytes, l.attribute_bytes, l.host_reads);
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_EXTRA_OBJ_LISTS").is_some() {
        let resources = decoded.bus.rom_resources().unwrap();
        let touched: Vec<_> = [0x394f34,0x3950fa,0x395102,0x399628,0x3adfc4,0x3adfd2,0x3adfda].into_iter()
            .filter(|&offset| resources.obj_list_source_used(offset) == Some((true,true))).collect();
        eprintln!("ROM_DATA_EXTRA_OBJ_SCENE frames={frames} guest_and_host_sources={touched:x?}");
        assert!(!touched.is_empty(), "this scene must consume a newly added OBJ list through guest and host readers");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_OBJ_ANIMATION").is_some() {
        let a = decoded.bus.rom_resources().unwrap().obj_animation_stats().expect("decoded EUR OBJ animation banks");
        eprintln!("ROM_DATA_OBJ_ANIMATION_SCENE frames={frames} bus_reads={} source_bytes={} index_bytes={} count_bytes={} attribute_bytes={} sequence_header_bytes={} frame_list_bytes={} duration_bytes={} host_reads={}",
            a.bus_reads, a.source_bytes, a.index_bytes, a.count_bytes, a.attribute_bytes, a.sequence_header_bytes, a.frame_list_bytes, a.duration_bytes, a.host_reads);
        assert!(a.index_bytes > 0 && a.count_bytes > 0 && a.attribute_bytes > 0 && a.frame_list_bytes > 0 && a.duration_bytes > 0 && a.host_reads > 0,
            "this scene must consume typed OBJ animation indices/list fields/frame selectors/durations and host lists");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_ZERO_COUNT_OBJ").is_some() {
        let a = decoded.bus.rom_resources().unwrap().obj_animation_stats().unwrap();
        eprintln!("ROM_DATA_ZERO_COUNT_SCENE frames={frames} selector_bytes={} host_reads={}",a.zero_count_selector_bytes,a.zero_count_host_reads);
        assert!(a.zero_count_selector_bytes > 0 && a.zero_count_host_reads > 0,
            "ordinary scene must use the newly rooted zero-count selectors in guest and host paths");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_LOCALIZED_OBJ").is_some() {
        let a = decoded.bus.rom_resources().unwrap().obj_animation_stats().unwrap();
        eprintln!("ROM_DATA_LOCALIZED_OBJ_SCENE frames={frames} selector_bytes={} source_bytes={} host_reads={}",
            a.localized_selector_bytes,a.localized_source_bytes,a.localized_host_reads);
        assert!(a.localized_selector_bytes>0 && a.localized_source_bytes>0 && a.localized_host_reads>0,
            "ordinary scene must use the new language-selected OBJ tables, bank and host lists");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_OBJ_GRAPHICS").is_some() {
        let g = decoded.bus.rom_resources().unwrap().obj_graphics_stats().expect("decoded EUR OBJ custom graphics");
        eprintln!("ROM_DATA_OBJ_GRAPHICS_SCENE frames={frames} bus_reads={} source_bytes={} decoded_literal_bytes={} selector_bytes={}",g.bus_reads,g.source_bytes,g.decoded_literal_bytes,g.selector_bytes);
        assert!(g.source_bytes > 0 && g.decoded_literal_bytes > 0 && g.selector_bytes > 0,
            "ordinary scene must consume custom OBJ bank literals and language selectors");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_UI_PALETTES").is_some() {
        let p = decoded.bus.rom_resources().unwrap().ui_palette_stats().expect("decoded EUR raw UI palettes");
        eprintln!("ROM_DATA_UI_PALETTE_SCENE frames={frames} bus_reads={} source_bytes={}",p.bus_reads,p.source_bytes);
        assert!(p.source_bytes > 0,"ordinary scene must read new file-derived UI palette colors");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_UI_GRAPHICS").is_some() {
        let owner = decoded.bus.rom_resources().unwrap();
        let g = owner.ui_graphics_stats().expect("decoded EUR custom UI graphics");
        eprintln!("ROM_DATA_UI_GRAPHICS_SCENE frames={frames} bus_reads={} source_bytes={} decoded_literal_bytes={} sources={:x?}",g.bus_reads,g.source_bytes,g.decoded_literal_bytes,owner.ui_graphics_used_sources());
        assert!(g.source_bytes>0 && g.decoded_literal_bytes>0,"ordinary scene must consume newly migrated custom UI literals");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_METADATA16").is_some() {
        let i=decoded.bus.rom_resources().unwrap().metadata16_stats().expect("decoded EUR metadata16");
        eprintln!("ROM_DATA_METADATA16_SCENE frames={frames} bus_reads={} source_bytes={} alias_bytes={} display_bytes={} flag_bytes={} value_bytes={} opaque_bytes={}",i.bus_reads,i.source_bytes,i.alias_bytes,i.display_bytes,i.flag_bytes,i.value_bytes,i.opaque_bytes);
        assert!(i.source_bytes>0,"ordinary scene must consume newly migrated metadata16fields");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_METADATA4").is_some() {
        let i=decoded.bus.rom_resources().unwrap().metadata4_stats().expect("decoded EUR metadata4");
        eprintln!("ROM_DATA_METADATA4_SCENE frames={frames} bus_reads={} source_bytes={} selector_bytes={} value_bytes={}",i.bus_reads,i.source_bytes,i.selector_bytes,i.value_bytes);
        assert!(i.source_bytes>0,"ordinary scene must consume newly migrated metadata4 fields");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_METADATA2").is_some() {
        let i=decoded.bus.rom_resources().unwrap().metadata2_stats().expect("decoded EUR metadata2");
        eprintln!("ROM_DATA_METADATA2_SCENE frames={frames} bus_reads={} source_bytes={} key_bytes={} value_bytes={}",i.bus_reads,i.source_bytes,i.key_bytes,i.value_bytes);
        assert!(i.source_bytes>0,"ordinary scene must consume newly migrated metadata2 fields");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_DEFAULT48").is_some() {
        let i=decoded.bus.rom_resources().unwrap().default48_stats().expect("decoded EUR default48");
        eprintln!("ROM_DATA_DEFAULT48_SCENE frames={frames} bus_reads={} source_bytes={} header_bytes={} argument_bytes={} halfword_bytes={} flag_bytes={} tail_bytes={}",i.bus_reads,i.source_bytes,i.header_bytes,i.argument_bytes,i.halfword_bytes,i.flag_bytes,i.tail_bytes);
        assert!(i.source_bytes>0,"ordinary scene must consume newly migrated default48 fields");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_CONSTRUCTOR_ALTERNATES").is_some() {
        let i=decoded.bus.rom_resources().unwrap().constructor_alternates_stats().expect("decoded EUR constructor_alternates");
        eprintln!("ROM_DATA_CONSTRUCTOR_ALTERNATES_SCENE frames={frames} bus_reads={} source_bytes={} pointer_bytes={} record_mask={} header_bytes={} argument_bytes={} halfword_bytes={} flag_bytes={} tail_bytes={}",i.bus_reads,i.source_bytes,i.pointer_bytes,i.record_mask,i.header_bytes,i.argument_bytes,i.halfword_bytes,i.flag_bytes,i.tail_bytes);
        assert!(i.source_bytes>i.pointer_bytes && i.pointer_bytes>0 && i.record_mask>0,"ordinary scene must consume newly migrated constructor_alternates fields");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_METADATA14").is_some() {
        let i=decoded.bus.rom_resources().unwrap().metadata14_stats().expect("decoded EUR metadata14");
        eprintln!("ROM_DATA_METADATA14_SCENE frames={frames} bus_reads={} source_bytes={} records_used={} byte_fields={} halfword_bytes={} packed_bytes={} tail_bytes={}",i.bus_reads,i.source_bytes,i.records_used(),i.byte_fields,i.halfword_bytes,i.packed_bytes,i.tail_bytes);
        assert!(i.source_bytes>0 && i.records_used()>0,"ordinary scene must consume newly migrated metadata14 fields");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_METADATA12").is_some() {
        let i=decoded.bus.rom_resources().unwrap().metadata12_stats().expect("decoded EUR metadata12");
        eprintln!("ROM_DATA_METADATA12_SCENE frames={frames} bus_reads={} source_bytes={} records_used={} primary_bytes={} secondary_bytes={}",i.bus_reads,i.source_bytes,i.records_used(),i.primary_bytes,i.secondary_bytes);
        assert!(i.source_bytes>0 && i.records_used()>0,"ordinary scene must consume newly migrated metadata12 fields");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_GLOBAL_PAIR_FLAGS").is_some() {
        let i=decoded.bus.rom_resources().unwrap().global_pair_flags_stats().expect("decoded EUR global_pair_flags");
        eprintln!("ROM_DATA_GLOBAL_PAIR_FLAGS_SCENE frames={frames} bus_reads={} source_bytes={} bytes_used={} byte_mask={}",i.bus_reads,i.source_bytes,i.bytes_used(),i.byte_mask);
        assert!(i.source_bytes>0 && i.bytes_used()>0,"ordinary scene must consume newly migrated global_pair_flags fields");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_EVALUATION_PARAMETERS").is_some() {
        let i=decoded.bus.rom_resources().unwrap().evaluation_parameters_stats().expect("decoded EUR evaluation_parameters");
        eprintln!("ROM_DATA_EVALUATION_PARAMETERS_SCENE frames={frames} bus_reads={} source_bytes={} divisor_bytes={} weight_bytes={} bonus_bytes={} score_opaque_bytes={} threshold_bytes={} threshold_opaque_bytes={} threshold_mask={}",i.bus_reads,i.source_bytes,i.divisor_bytes,i.weight_bytes,i.bonus_bytes,i.score_opaque_bytes,i.threshold_bytes,i.threshold_opaque_bytes,i.threshold_mask);
        assert!(i.source_bytes>0 && i.divisor_bytes+i.weight_bytes+i.bonus_bytes+i.threshold_bytes>0,"ordinary scene must consume newly migrated evaluation_parameters fields");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_PROGRESSION_REWARDS").is_some() {
        let i=decoded.bus.rom_resources().unwrap().progression_rewards_stats().expect("decoded EUR progression_rewards");
        eprintln!("ROM_DATA_PROGRESSION_REWARDS_SCENE frames={frames} bus_reads={} source_bytes={} delta_bytes={} discount_bytes={} opaque_bytes={} record_mask_lo={} record_mask_hi={}",i.bus_reads,i.source_bytes,i.delta_bytes,i.discount_bytes,i.opaque_bytes,i.record_mask[0],i.record_mask[1]);
        assert!(i.source_bytes>0 && i.delta_bytes+i.discount_bytes>0,"ordinary scene must consume newly migrated progression_rewards fields");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_PROGRESSION_GATES").is_some() {
        let i=decoded.bus.rom_resources().unwrap().progression_gates_stats().expect("decoded EUR progression_gates");
        eprintln!("ROM_DATA_PROGRESSION_GATES_SCENE frames={frames} bus_reads={} source_bytes={} requirement_bytes={} return_bytes={} output_bytes={} unused_bytes={} record_mask={}",i.bus_reads,i.source_bytes,i.requirement_bytes,i.return_bytes,i.output_bytes,i.unused_bytes,i.record_mask);
        assert!(i.source_bytes>0 && i.requirement_bytes+i.return_bytes+i.output_bytes>0,"ordinary scene must consume newly migrated progression_gates fields");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_PROGRESSION_SELECTION_GATES").is_some() {
        let i=decoded.bus.rom_resources().unwrap().progression_selection_gates_stats().expect("decoded EUR selection gates");
        eprintln!("ROM_DATA_PROGRESSION_SELECTION_GATES_SCENE frames={frames} bus_reads={} source_bytes={} selector_bytes={} minimum01_bytes={} minimum02_bytes={} minimum04_bytes={} any_minima_bytes={} flag_bytes={} record_mask={}",i.bus_reads,i.source_bytes,i.selector_bytes,i.minimum01_bytes,i.minimum02_bytes,i.minimum04_bytes,i.any_minima_bytes,i.flag_bytes,i.record_mask);
        assert!(i.source_bytes>0,"ordinary scene must consume newly migrated selection gates");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_AGGREGATE_FLAG_KEYS").is_some() {
        let i=decoded.bus.rom_resources().unwrap().aggregate_flag_keys_stats().expect("decoded EUR aggregate keys");
        eprintln!("ROM_DATA_AGGREGATE_FLAG_KEYS_SCENE frames={frames} bus_reads={} source_bytes={} key_bytes={} terminator_bytes={} entry_mask={:?}",i.bus_reads,i.source_bytes,i.key_bytes,i.terminator_bytes,i.entry_mask);
        assert!(i.key_bytes>0 && i.terminator_bytes>0,"ordinary scene must consume aggregate keys and terminal");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_METADATA52").is_some() {
        let i=decoded.bus.rom_resources().unwrap().metadata52_stats().expect("decoded EUR metadata52");
        eprintln!("ROM_DATA_METADATA52_SCENE frames={frames} bus_reads={} source_bytes={} header_bytes={} routing_bytes={} packed_bytes={} tail_bytes={} opaque_bytes={}",i.bus_reads,i.source_bytes,i.header_bytes,i.routing_bytes,i.packed_bytes,i.tail_bytes,i.opaque_bytes);
        assert!(i.source_bytes>0,"ordinary scene must consume newly migrated metadata52fields");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_ITEM_LINKS").is_some() {
        let i=decoded.bus.rom_resources().unwrap().item_link_stats().expect("decoded EUR item links");
        eprintln!("ROM_DATA_ITEM_LINK_SCENE frames={frames} bus_reads={} source_bytes={} count_bytes={} selector_bytes={} value_bytes={} opaque_bytes={}",i.bus_reads,i.source_bytes,i.count_bytes,i.selector_bytes,i.value_bytes,i.opaque_bytes);
        assert!(i.source_bytes>0 && i.count_bytes>0 && i.selector_bytes>0 && i.value_bytes>0,"ordinary scene must consume newly migrated item-link fields");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_ITEMS").is_some() {
        let i=decoded.bus.rom_resources().unwrap().item_stats().expect("decoded EUR item records");
        eprintln!("ROM_DATA_ITEM_SCENE frames={frames} bus_reads={} source_bytes={} halfword_bytes={} signed_bytes={} other_bytes={}",i.bus_reads,i.source_bytes,i.halfword_bytes,i.signed_bytes,i.other_bytes);
        assert!(i.source_bytes>0,"ordinary scene must read new file-derived item fields");
    }
    if std::env::var_os("FFTA_STATE_TEST_REQUIRE_UI_GRAPHICS_SELECTOR").is_some() {
        let g=decoded.bus.rom_resources().unwrap().ui_graphics_stats().expect("decoded EUR localized UI graphics");
        eprintln!("ROM_DATA_UI_GRAPHICS_SELECTOR_SCENE frames={frames} selector_bytes={} localized_source_bytes={} localized_literal_bytes={}",g.selector_bytes,g.localized_source_bytes,g.localized_literal_bytes);
        assert!(g.selector_bytes>0,"ordinary scene must read a new file-derived UI language pointer");
    }
    if let Ok(mask) = std::env::var("FFTA_STATE_TEST_REQUIRE_SOURCE_MASK") {
        let required: u8 = mask.parse().unwrap();
        assert_eq!(
            stats.bus_source_mask & required,
            required,
            "required source ranges did not execute"
        );
    }
    assert!(
        stats.host_rom_reads + stats.host_ram_reads > 0,
        "typed consumers must execute"
    );
    eprintln!("ROM_DATA_SCENE frames={frames} first={first} native={} fallback={} full_states={frames} audio={audio:?} bus_reads={} host_rom_reads={} host_ram_reads={} dynamic_ram_reads={} bus_source_mask={}", dispatch[0], dispatch[1], stats.bus_reads, stats.host_rom_reads, stats.host_ram_reads, stats.dynamic_ram_reads, stats.bus_source_mask);
    eprintln!(
        "ROM_DATA_BG_SCENE frames={frames} bus_reads={} projected_bytes={}",
        bg.bus_reads, bg.projected_bytes
    );
}

fn clear_audio(machine: &mut gba_core::Machine) {
    machine.bus.audio_buf.clear();
    machine.bus.psg_tap.clear();
    machine.bus.hle_tap.clear();
    machine.bus.fifo_tap.iter_mut().for_each(Vec::clear);
}

#[test]
#[ignore = "requires the exact private USA ROM; follows clean-boot demo input to the original copier"]
fn usa_copied_obj_initializer_from_clean_boot_uses_decoded_source() {
    crate::configure_headless_widescreen();
    let rom = fs::read(std::env::var_os("FFTA_STATE_TEST_ROM").expect("USA ROM fixture")).unwrap();
    assert_eq!(gba_core::ffta_rom::digest(&rom), gba_core::ffta_rom::SHA256);
    // Independently located USA literals and function entry. No EUR offset
    // arithmetic or copied guest instructions are used to construct this test.
    const ENTRY: u32 = 0x0808_f74c;
    const START: usize = 0xa38adc;
    const END: usize = 0xa3991c;
    const ALIAS: usize = 0xa398dc;
    const DEST: usize = 0x5f28;
    let word = |offset| u32::from_le_bytes(rom[offset..offset + 4].try_into().unwrap());
    assert_eq!(word(0x8f764), 0x0800_0000 + END as u32);
    assert_eq!(word(0x8f768), 0x0800_0000 + START as u32);
    assert_eq!(word(0x8f76c), 0x0300_0000 + DEST as u32);
    assert_eq!(DEST + ALIAS - START, 0x6d28);

    let mut source = Extraction::from_rom(&rom).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let dataset = directory.path().join("decoded");
    extract(&dataset, &mut source).unwrap();
    let mut decoded = make_machine(rom.clone(), None);
    attach_from_directory(&mut decoded, &dataset).unwrap();
    let max_frames: u64 = std::env::var("FFTA_STATE_TEST_FRAMES")
        .unwrap_or_else(|_| "12000".into())
        .parse()
        .unwrap();
    assert!((1..=30000).contains(&max_frames));
    for frame in 0..max_frames {
        decoded.bus.keys = crate::demo_keys(frame);
        decoded.bus.frame_ready = false;
        for _ in 0..FRAME_STEP_GUARD {
            if decoded.cpu.regs[15] == ENTRY {
                assert!(decoded.cpu.thumb());
                let checkpoint = decoded.save_state().unwrap();
                let entered_frame = decoded.bus.frames;
                let entered_clock = decoded.bus.clock;
                let return_pc = decoded.cpu.regs[14] & !1;
                let before = decoded.bus.rom_resources().unwrap().stats();
                let mut legacy = make_machine(rom.clone(), None);
                legacy.load_state(&checkpoint).unwrap();
                assert_eq!(legacy.save_state().unwrap(), checkpoint);
                let mut count = 0;
                let mut read_pc = None;
                while decoded.cpu.regs[15] != return_pc || count == 0 {
                    assert!(count < 10000, "original initializer did not return");
                    let pc = decoded.cpu.regs[15];
                    let reads = decoded.bus.rom_resources().unwrap().stats().bus_reads;
                    legacy.step();
                    decoded.step();
                    assert!(
                        legacy.save_state().unwrap() == decoded.save_state().unwrap(),
                        "initializer full state at step {count}/PC {pc:08x}"
                    );
                    if decoded.bus.rom_resources().unwrap().stats().bus_reads > reads {
                        assert!(
                            read_pc.replace(pc).is_none(),
                            "unexpected multiple copy instructions"
                        );
                    }
                    count += 1;
                }
                assert_eq!(
                    &decoded.bus.iwram[DEST..DEST + END - START],
                    &rom[START..END]
                );
                assert_eq!(
                    &decoded.bus.iwram[0x6d28..0x6d58],
                    &source.dimensions.source_bytes()
                );
                let after = decoded.bus.rom_resources().unwrap().stats();
                assert_eq!(after.bus_reads - before.bus_reads, 24);
                assert_eq!(after.bus_source_mask & 2, 2);
                let read_pc = read_pc.expect("original copier must read the file-backed alias");

                // A separate provenance probe poisons only the old raw table
                // AFTER authenticated activation. Original code, other source
                // bytes and the caller's real checkpoint are left intact.
                let mut file_only = make_machine(rom.clone(), None);
                attach_from_directory(&mut file_only, &dataset).unwrap();
                file_only.load_state(&checkpoint).unwrap();
                file_only.bus.rom[ALIAS..ALIAS + 48].fill(0xa5);
                for _ in 0..count {
                    file_only.step();
                }
                assert_eq!(
                    &file_only.bus.iwram[DEST..DEST + END - START],
                    &rom[START..END]
                );
                assert_eq!(file_only.bus.rom_resources().unwrap().stats().bus_reads, 24);
                assert_eq!(file_only.cpu.regs[15], return_pc);
                // Restore the test poison only after the guest returned, so
                // snapshot ROM identity can be compared without masking state.
                file_only.bus.rom[ALIAS..ALIAS + 48].copy_from_slice(&rom[ALIAS..ALIAS + 48]);
                assert!(file_only.save_state().unwrap() == legacy.save_state().unwrap());
                eprintln!("ROM_DATA_USA_INITIALIZER frame={entered_frame} clock={entered_clock} entry={ENTRY:08x} return={return_pc:08x} read_pc={read_pc:08x} steps={count} copied_bytes={} table_bytes=48 decoded_reads=24 poisoned_rom_probe=true full_states={count}", END-START);
                return;
            }
            decoded.step();
            if decoded.bus.frame_ready {
                break;
            }
        }
        assert_eq!(decoded.bus.frames, frame + 1, "incomplete boot frame");
        clear_audio(&mut decoded);
        if frame % 500 == 499 {
            eprintln!(
                "ROM_DATA_USA_BOOT frames={} pc={:08x}",
                decoded.bus.frames, decoded.cpu.regs[15]
            );
        }
    }
    panic!("original USA initializer was not reached in {max_frames} clean-boot frames");
}
