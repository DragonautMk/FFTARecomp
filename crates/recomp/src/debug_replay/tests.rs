use super::*;

#[cfg(windows)]
#[test]
fn progress_publication_retries_a_short_windows_reader_sharing_denial() {
    use std::os::windows::ffi::OsStrExt;
    #[link(name = "kernel32")]
    extern "system" {
        fn CreateFileW(
            name: *const u16,
            access: u32,
            share: u32,
            security: usize,
            creation: u32,
            flags: u32,
            template: usize,
        ) -> usize;
        fn CloseHandle(handle: usize) -> i32;
    }
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("progress.json");
    json(&path, &serde_json::json!({"completed":1})).unwrap();
    let name: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    let handle = unsafe { CreateFileW(name.as_ptr(), 0x80000000, 1, 0, 3, 0, 0) };
    assert_ne!(handle, usize::MAX);
    let release = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(12));
        unsafe { CloseHandle(handle) };
    });
    json(&path, &serde_json::json!({"completed":2})).unwrap();
    release.join().unwrap();
    let result: serde_json::Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    assert_eq!(result["completed"], 2);
}

fn machine() -> Machine {
    let mut rom = vec![0; 0x200];
    rom[..4].copy_from_slice(&0xeaff_fffeu32.to_le_bytes());
    rom[0x100..0x106].copy_from_slice(b"SRAM_V");
    let mut m = crate::make_machine(rom, None);
    m.bus.frames = 17;
    m.bus.keys = 0x3fe;
    m
}
fn drain(m: &mut Machine) {
    m.bus.audio_buf.clear();
    m.bus.psg_tap.clear();
    m.bus.hle_tap.clear();
    m.bus.fifo_tap[0].clear();
    m.bus.fifo_tap[1].clear();
}
struct Fixture {
    temp: tempfile::TempDir,
    recording: PathBuf,
    session: PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let recording = temp.path().join("capture");
        let mut m = machine();
        let av = input_config::AvConfig::default();
        let input = input_config::InputConfig::default();
        let mut c = crate::debug_recording::Capture::start(&recording, &m, true, None, &av, &input)
            .unwrap();
        let initial = m.save_state().unwrap();
        for n in 0..4 {
            m.bus.keys = if n < 2 { 0x3fe } else { 0x3ff };
            c.before_frame(&m, &[]).unwrap();
            m.run_frame(5_000_000);
            drain(&mut m);
        }
        c.finish(&m, "fixture", true).unwrap();
        let session = crate::debug_session::create(
            &temp.path().join("source session"),
            crate::debug_session::Source {
                rom: &m.bus.rom,
                bios: None,
                save: m.bus.save_data(),
                state: Some(&initial),
                av,
                input,
                interp: true,
                dataset: None,
                executable: &std::env::current_exe().unwrap(),
            },
        )
        .unwrap();
        Self {
            temp,
            recording,
            session,
        }
    }
    fn create(&self) -> PathBuf {
        create(
            &self.recording,
            &self.session,
            &self.temp.path().join("replay case"),
            "DS-20-EUR",
            120000,
            &std::env::current_exe().unwrap(),
            None,
        )
        .unwrap()
    }
    fn inputs(&self) -> Vec<Vec<u8>> {
        [
            self.session.join("seed.ffstate"),
            self.session.join("Data/saves/ffta.sav"),
            self.recording.join("start.ffstate"),
            self.recording.join("input.input"),
            self.recording.join("end.ffstate"),
        ]
        .iter()
        .map(|p| fs::read(p).unwrap())
        .collect()
    }
}
fn run_fixture(path: &Path, control: impl FnMut(u64) -> Option<(&'static str, String)>) -> Report {
    let (root, p) = load(path).unwrap();
    let mut m = machine();
    m.load_state(&read(&root.join("seed.ffstate"), 64 * 1024 * 1024).unwrap())
        .unwrap();
    let bytes = read(&root.join("input.input"), MAX_INPUT).unwrap();
    let input = crate::debug_recording::script(&bytes, p.frames).unwrap();
    let end = read(&root.join("expected.ffstate"), 64 * 1024 * 1024).unwrap();
    execute(&p, "fixture-run", &mut m, &input, &end, None, None, control)
}
#[test]
fn profile_pins_selected_inputs_and_repeats_full_endpoint_without_save_writes() {
    let f = Fixture::new();
    let before = f.inputs();
    let path = f.create();
    for _ in 0..2 {
        let r = run_fixture(&path, |_| None);
        assert_eq!(r.outcome, "passed");
        assert_eq!(r.completed, 4);
        assert_eq!(r.code(), 0);
    }
    assert_eq!(f.inputs(), before);
    let (root, p) = load(&path).unwrap();
    assert_eq!(p.stop, "exact-endpoint");
    assert!(p.library_sha256.is_none());
    assert_eq!(
        checked(&root, "input.input", MAX_INPUT, &p.input_sha256).unwrap(),
        fs::read(f.recording.join("input.input")).unwrap()
    );
}
#[test]
fn timeout_and_interruption_report_distinct_codes_and_exact_progress() {
    let f = Fixture::new();
    let path = f.create();
    for (kind, code) in [("timeout", 4), ("interrupted", 5)] {
        let r = run_fixture(&path, |done| {
            if done == 2 {
                Some((kind, "fixture stop".into()))
            } else {
                None
            }
        });
        assert_eq!(r.outcome, kind);
        assert_eq!(r.code(), code);
        assert_eq!(r.completed, 2);
    }
    assert_eq!(run_fixture(&path, |_| None).outcome, "passed");
}
#[test]
fn endpoint_divergence_never_becomes_a_success() {
    let f = Fixture::new();
    let path = f.create();
    let (root, p) = load(&path).unwrap();
    let mut m = machine();
    m.load_state(&fs::read(root.join("seed.ffstate")).unwrap())
        .unwrap();
    let input =
        crate::debug_recording::script(&fs::read(root.join("input.input")).unwrap(), p.frames)
            .unwrap();
    let r = execute(
        &p,
        "fixture",
        &mut m,
        &input,
        b"different expected endpoint",
        None,
        None,
        |_| None,
    );
    assert_eq!(r.code(), 2);
    assert_eq!(r.outcome, "diverged");
    assert!(r.state_sha256.is_some());
}
#[test]
fn create_refuses_active_sessions_existing_destinations_and_corrupt_capture() {
    let f = Fixture::new();
    let path = f.create();
    assert!(create(
        &f.recording,
        &f.session,
        path.parent().unwrap(),
        "DS20",
        1000,
        &std::env::current_exe().unwrap(),
        None
    )
    .is_err());
    let _lease = crate::debug_session::open(&f.session).unwrap();
    assert!(create(
        &f.recording,
        &f.session,
        &f.temp.path().join("active"),
        "DS20",
        1000,
        &std::env::current_exe().unwrap(),
        None
    )
    .is_err());
    drop(_lease);
    fs::write(f.recording.join("input.input"), "changed").unwrap();
    assert!(create(
        &f.recording,
        &f.session,
        &f.temp.path().join("corrupt"),
        "DS20",
        1000,
        &std::env::current_exe().unwrap(),
        None
    )
    .unwrap_err()
    .contains("checksum"));
}
#[test]
fn profile_bounds_schema_and_changed_input_are_detected() {
    let f = Fixture::new();
    let path = f.create();
    let (root, p) = load(&path).unwrap();
    fs::write(root.join("input.input"), b"changed").unwrap();
    assert!(checked(&root, "input.input", MAX_INPUT, &p.input_sha256)
        .unwrap_err()
        .contains("changed"));
    for (timeout, frames, stop) in [
        (0, 4, "exact-endpoint"),
        (120001, 4, "exact-endpoint"),
        (1000, MAX_FRAMES + 1, "exact-endpoint"),
        (1000, 4, "unknown"),
    ] {
        let mut invalid = p.clone();
        invalid.timeout_ms = timeout;
        invalid.frames = frames;
        invalid.stop = stop.into();
        json(&path, &invalid).unwrap();
        assert!(load(&path).is_err());
    }
}
#[test]
fn moved_profile_preserves_relative_payloads_and_selected_binary() {
    let f = Fixture::new();
    let path = f.create();
    let moved = f.temp.path().join("moved profile");
    fs::rename(path.parent().unwrap(), &moved).unwrap();
    assert_eq!(
        run_fixture(&moved.join(PROFILE), |_| None).outcome,
        "passed"
    );
    let (_, p) = load(&moved.join(PROFILE)).unwrap();
    assert_eq!(
        digest(&read(&moved.join(executable()), 256 * 1024 * 1024).unwrap()),
        p.runtime_sha256
    );
}
#[test]
fn wrong_session_rom_and_malformed_origin_are_rejected_before_destination_creation() {
    let f = Fixture::new();
    let p = f.recording.join("recording.json");
    let mut manifest: serde_json::Value = serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
    manifest["rom_sha256"] = "wrong".into();
    json(&p, &manifest).unwrap();
    let destination = f.temp.path().join("wrong");
    assert!(create(
        &f.recording,
        &f.session,
        &destination,
        "DS20",
        1000,
        &std::env::current_exe().unwrap(),
        None
    )
    .unwrap_err()
    .contains("ROM/BIOS/dataset"));
    assert!(!destination.exists());
}
