use super::*;
fn wait(p: &mut Picker, m: &mut menu::Menu) {
    for _ in 0..2000 {
        p.poll(m);
        if p.job.is_none() {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    panic!("bounded synthetic worker did not finish");
}
fn fixture() -> (tempfile::TempDir, Picker, menu::Menu) {
    let t = tempfile::tempdir().unwrap();
    std::fs::write(t.path().join("a.html"), b"SYNTHETIC same").unwrap();
    std::fs::write(t.path().join("b.html"), b"SYNTHETIC same").unwrap();
    let mut p = Picker::default();
    p.roots = vec![Root::new(t.path(), false).unwrap()];
    let mut m = menu::Menu::new(true, true);
    p.open(&mut m, "a".repeat(64)).unwrap();
    (t, p, m)
}
#[test]
fn opening_cancelled_external_and_highlight_never_start_workers_or_select() {
    let (_t, mut p, mut m) = fixture();
    assert!(!p.status()["busy"].as_bool().unwrap());
    assert_eq!(p.status()["entries"], 0);
    assert!(p.selected.is_none());
    p.external_result(None, true, &mut m).unwrap();
    p.external_result(None, false, &mut m).unwrap();
    assert!(p.job.is_none());
    assert_eq!(p.roots.len(), 1);
    assert!(p.selected.is_none());
}
#[test]
fn refresh_select_recheck_and_generation_refusal_share_the_catalog_service() {
    let (t, mut p, mut m) = fixture();
    p.refresh(&mut m).unwrap();
    wait(&mut p, &mut m);
    assert_eq!(p.status()["entries"], 2);
    let gen = p.generation;
    p.choose(&mut m, 0, gen, false).unwrap();
    wait(&mut p, &mut m);
    let id = p.selected.as_ref().unwrap().entry.id.clone();
    assert!(m.content_active());
    assert!(p.choose(&mut m, 0, gen + 1, false).is_err());
    let path = p.selected.as_ref().unwrap().path.clone();
    std::fs::write(path, b"CHANGED").unwrap();
    p.choose(&mut m, 0, gen, false).unwrap();
    wait(&mut p, &mut m);
    assert_eq!(p.selected.as_ref().unwrap().entry.id, id);
    assert!(t.path().is_dir());
}
#[test]
fn late_selection_after_cancel_is_discarded_and_imported_file_does_not_scan_siblings() {
    let (_t, mut p, mut m) = fixture();
    p.refresh(&mut m).unwrap();
    wait(&mut p, &mut m);
    p.choose(&mut m, 0, p.generation, false).unwrap();
    p.cancel();
    wait(&mut p, &mut m);
    assert!(p.selected.is_none());
    let external = tempfile::tempdir().unwrap();
    let file = external.path().join("one.html");
    std::fs::write(&file, b"SYNTHETIC").unwrap();
    std::fs::write(
        external.path().join("NEVER.py"),
        b"raise RuntimeError('never')",
    )
    .unwrap();
    p.external_result(Some(file), true, &mut m).unwrap();
    wait(&mut p, &mut m);
    assert_eq!(p.status()["entries"], 3);
    assert!(p
        .index
        .as_ref()
        .unwrap()
        .entries
        .iter()
        .all(|e| e.kind != Kind::HostTool));
}

#[test]
#[ignore = "explicit public synthetic GUI fixture; never reads personal game inputs"]
pub(crate) fn write_synthetic_menu_fixture() {
    let root = PathBuf::from(
        std::env::var_os("FFTA_SELECTOR_TEST_ROOT").expect("explicit new fixture root"),
    );
    assert!(!root.exists());
    let executable = PathBuf::from(
        std::env::var_os("FFTA_SELECTOR_TEST_RUNTIME").expect("explicit candidate runtime"),
    );
    let mut rom = vec![0u8; 0x200];
    rom[..4].copy_from_slice(&0xeafffffeu32.to_le_bytes());
    rom[0xa0..0xac].copy_from_slice(b"DBG2-PICKER!");
    rom[0x100..0x106].copy_from_slice(b"SRAM_V");
    let machine = crate::make_machine(rom.clone(), None);
    let state = machine.save_state().unwrap();
    let mut av = input_config::AvConfig::default();
    av.audio_mute = true;
    av.video_aspect = "16:9".into();
    av.screen = "raw".into();
    let mut input = input_config::InputConfig::default();
    input.keys[input_config::Button::A.index()] = "Enter".into();
    input.keys[input_config::Button::Start.index()] = "Space".into();
    let directory = crate::debug_session::create(
        &root,
        crate::debug_session::Source {
            rom: &rom,
            bios: None,
            save: machine.bus.save_data(),
            state: Some(&state),
            av,
            input,
            interp: true,
            dataset: None,
            executable: &executable,
        },
    )
    .unwrap();
    let hash = gba_core::ffta_rom::digest(&rom);
    for name in ["First folder", "Second folder"] {
        let dir = directory.join("Data/config/replay").join(name);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("replay.json"),serde_json::to_vec(&json!({"schema":1,"kind":"ffta-debug-replay-profile","case":"Same name","rom_sha256":hash,"region":"SYNTHETIC","description":"Public host-menu fixture; no functional replay or game data."})).unwrap()).unwrap();
    }
    let mut slots = crate::save_state::Slots::new(&directory.join("Data/saves/ffta.sav"), &rom);
    let image = (0..240 * 160)
        .map(|i| [(i % 240) as u8, (i / 240) as u8, 90, 255])
        .collect::<Vec<_>>();
    slots.save(&machine, 2, &image, false).unwrap();
    std::fs::write(directory.join("fixture-identity.json"),serde_json::to_vec_pretty(&json!({"schema":1,"kind":"public-synthetic-selector-fixture","rom_sha256":hash,"state_sha256":gba_core::ffta_rom::digest(&state),"frame":machine.bus.frames,"clock":machine.bus.clock,"scope":"Host menu only; no copyrighted input or guest execution required"})).unwrap()).unwrap();
}

#[test]
fn completed_preview_never_leaves_a_working_status_even_after_highlight_changes() {
    fn rendered(menu: &menu::Menu) -> String {
        let mut px = vec![[0, 0, 0, 255]; 940 * 620];
        let keys = std::array::from_fn(|_| String::new());
        let text = menu::diagnostics::capture(|| {
            menu.draw(
                &input_config::AvConfig::default(),
                &keys,
                "",
                "",
                "",
                "",
                &mut px,
                940,
                620,
            )
        })
        .unwrap();
        text.runs
            .into_iter()
            .map(|r| r.text)
            .collect::<Vec<_>>()
            .join("\n")
    }
    let (_t, mut p, mut m) = fixture();
    p.refresh(&mut m).unwrap();
    wait(&mut p, &mut m);
    let id = m.content_highlight().unwrap().to_owned();
    let index = p
        .index
        .as_ref()
        .unwrap()
        .entries
        .iter()
        .position(|e| e.id == id)
        .unwrap();
    p.choose(&mut m, index, p.generation, true).unwrap();
    wait(&mut p, &mut m);
    let text = rendered(&m);
    assert!(!text.contains("Working on selected"));
    assert!(text.contains("No embedded"));
    assert!(p.selected.is_none());
    p.choose(&mut m, index, p.generation, true).unwrap();
    m.content_start_search();
    m.content_text("b.html");
    m.content_finish_search(false);
    wait(&mut p, &mut m);
    assert!(!rendered(&m).contains("Working on selected"));
    assert!(p.selected.is_none());
}

#[test]
fn typed_slot_results_preserve_prior_refs_and_never_leak_after_view_cancel() {
    let (t, mut p, mut m) = fixture();
    let rom = "a".repeat(64);
    let file = t.path().join("runtime.exe");
    std::fs::write(&file, b"PUBLIC synthetic runtime; never run").unwrap();
    p.open_for(
        &mut m,
        rom.clone(),
        ContentSlot::RuntimeA,
        "Choose only".into(),
    )
    .unwrap();
    p.refresh(&mut m).unwrap();
    wait(&mut p, &mut m);
    let row = p
        .index
        .as_ref()
        .unwrap()
        .entries
        .iter()
        .position(|e| e.kind == Kind::Runtime)
        .unwrap();
    p.choose(&mut m, row, p.generation, false).unwrap();
    wait(&mut p, &mut m);
    assert!(p.has_slot(ContentSlot::RuntimeA));
    let old = p.slot_path(ContentSlot::RuntimeA, &rom).unwrap();
    p.open_for(
        &mut m,
        rom.clone(),
        ContentSlot::Library,
        "Wrong types refused".into(),
    )
    .unwrap();
    p.choose(&mut m, row, p.generation, false).unwrap();
    wait(&mut p, &mut m);
    assert!(!p.has_slot(ContentSlot::Library));
    assert_eq!(p.slot_path(ContentSlot::RuntimeA, &rom).unwrap(), old);
    p.open_for(
        &mut m,
        rom.clone(),
        ContentSlot::RuntimeB,
        "Cancel pending".into(),
    )
    .unwrap();
    p.choose(&mut m, row, p.generation, false).unwrap();
    p.cancel();
    wait(&mut p, &mut m);
    assert!(!p.has_slot(ContentSlot::RuntimeB));
    std::fs::write(&file, b"CHANGED").unwrap();
    assert!(p.slot_path(ContentSlot::RuntimeA, &rom).is_err());
    assert!(p.has_slot(ContentSlot::RuntimeA));
}
