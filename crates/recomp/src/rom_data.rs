//! Source-authenticated extraction, preparation and verified dataset loading.
use gba_core::rom_data::{
    Extraction, Manifest, ObjDimensions, MANIFEST_FILE, MAX_FILE_BYTES, OBJ_DIMENSIONS_FILE,
};
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[cfg(test)]
mod native_tests;
#[cfg(test)]
mod validation_tests;
#[cfg(test)]
mod compression_tests;
mod preparation;
pub use preparation::prepare_startup;

fn read_bounded(path: &Path) -> Result<Vec<u8>, String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|e| format!("Cannot inspect {}: {e}", path.display()))?;
    if !metadata.file_type().is_file() || metadata.len() > MAX_FILE_BYTES {
        return Err(format!(
            "Resource must be a regular file of at most {MAX_FILE_BYTES} bytes: {}",
            path.display()
        ));
    }
    let mut bytes = Vec::new();
    fs::File::open(path)
        .and_then(|f| f.take(MAX_FILE_BYTES + 1).read_to_end(&mut bytes))
        .map_err(|e| format!("Cannot read {}: {e}", path.display()))?;
    if bytes.len() as u64 > MAX_FILE_BYTES {
        return Err("Resource grew beyond the file size limit".into());
    }
    Ok(bytes)
}

fn parse<T: serde::de::DeserializeOwned>(bytes: &[u8], label: &str) -> Result<T, String> {
    let value = std::str::from_utf8(bytes).map_err(|_| format!("{label} is not UTF-8"))?;
    toml::from_str(value).map_err(|e| format!("Invalid {label}: {e}"))
}

pub fn verify(path: &Path, source: &Extraction) -> Result<ObjDimensions, String> {
    let (manifest, dimensions, bytes) = read_dataset(path)?;
    source.validate_files(&manifest, &dimensions, &bytes)?;
    Ok(dimensions)
}

fn read_dataset(path: &Path) -> Result<(Manifest, ObjDimensions, Vec<u8>), String> {
    if !fs::symlink_metadata(path).is_ok_and(|m| m.file_type().is_dir()) {
        return Err(format!(
            "Dataset must be a directory, not a symlink: {}",
            path.display()
        ));
    }
    let manifest: Manifest = parse(
        &read_bounded(&path.join(MANIFEST_FILE))?,
        "resource manifest",
    )?;
    // Deliberately do not follow paths provided by a manifest. Version 1 has
    // exactly one resource and one fixed file name; source validation pins both.
    let bytes = read_bounded(&path.join(OBJ_DIMENSIONS_FILE))?;
    let dimensions: ObjDimensions = parse(&bytes, "OBJ dimensions")?;
    Ok((manifest, dimensions, bytes))
}

pub fn attach_from_directory(machine: &mut gba_core::Machine, path: &Path) -> Result<(), String> {
    let (manifest, dimensions, bytes) = read_dataset(path)?;
    let resources = gba_core::rom_data::RuntimeResources::from_base_files(
        &machine.bus.rom,
        &manifest,
        dimensions,
        &bytes,
    )?;
    let resident = resources.resident_bytes() + 2 * std::mem::size_of::<usize>();
    if resident > 1024 {
        return Err("Decoded OBJ metadata exceed the session memory budget".into());
    }
    machine.bus.set_rom_resources(Some(resources.clone()))?;
    eprintln!("ROM_DATA runtime=decoded resource={} source={} dataset={} file={} resident_bytes={resident}",
        resources.resource_id(), resources.source_sha256(), resources.dataset_sha256(), resources.file_sha256());
    Ok(())
}

pub fn prepare_and_attach(machine: &mut gba_core::Machine, status: bool) -> Result<(), String> {
    let prepared = prepare_startup(&machine.bus.rom, status)?;
    attach_from_directory(machine, &prepared.directory)
}

pub fn report_usage(machine: &gba_core::Machine) {
    if let Some(resources) = machine.bus.rom_resources() {
        let stats = resources.stats();
        eprintln!("ROM_DATA_USE resource={} dataset={} bus_reads={} host_rom_reads={} host_ram_reads={} dynamic_ram_reads={} validation_reads={} bus_source_mask={}",
            resources.resource_id(), resources.dataset_sha256(), stats.bus_reads, stats.host_rom_reads,
            stats.host_ram_reads, stats.dynamic_ram_reads, stats.validation_reads, stats.bus_source_mask);
    }
}

fn write_synced(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| format!("Cannot create {}: {e}", path.display()))?;
    file.write_all(bytes)
        .and_then(|_| file.sync_all())
        .map_err(|e| format!("Cannot finish {}: {e}", path.display()))
}

/// Publish only into an absent directory. An existing valid dataset is reused;
/// invalid data produce an explicit error, never a silent ROM fallback.
pub fn extract(path: &Path, source: &mut Extraction) -> Result<&'static str, String> {
    match fs::symlink_metadata(path) {
        Ok(_) => {
            verify(path, source)?;
            return Ok("reused");
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(format!("Cannot inspect output directory: {e}")),
    }
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    fs::create_dir_all(parent).map_err(|e| format!("Cannot create resources parent: {e}"))?;
    let temporary = tempfile::Builder::new()
        .prefix(".ffta-resources-")
        .tempdir_in(parent)
        .map_err(|e| format!("Cannot prepare resource directory: {e}"))?;
    write_dataset(temporary.path(), source, &mut write_synced)?;
    match fs::rename(temporary.path(), path) {
        Ok(()) => Ok("created"),
        // A concurrent writer may have published the same complete dataset.
        Err(_) if verify(path, source).is_ok() => Ok("reused"),
        Err(e) => Err(format!("Cannot publish resource directory: {e}")),
    }
}

fn write_dataset(
    path: &Path,
    source: &mut Extraction,
    write: &mut impl FnMut(&Path, &[u8]) -> Result<(), String>,
) -> Result<(), String> {
    let data = toml::to_string_pretty(&source.dimensions)
        .map_err(|e| format!("Cannot encode dimensions: {e}"))?;
    source.set_file_hash(data.as_bytes());
    let manifest = toml::to_string_pretty(&source.manifest)
        .map_err(|e| format!("Cannot encode manifest: {e}"))?;
    write(&path.join(OBJ_DIMENSIONS_FILE), data.as_bytes())?;
    write(&path.join(MANIFEST_FILE), manifest.as_bytes())?;
    verify(path, source)?;
    Ok(())
}

pub fn cmd(args: &[String]) -> Result<(), String> {
    let operation = args.first().map(String::as_str);
    if !matches!(
        (operation, args.len()),
        (Some("extract" | "verify"), 3) | (Some("prepare"), 2 | 3)
    ) {
        return Err("usage: recomp rom-data <extract|verify> <rom> <dataset-directory> OR rom-data prepare <rom> [resources-root]".into());
    }
    let rom = fs::read(&args[1]).map_err(|e| format!("Cannot read ROM: {e}"))?;
    let mut source = Extraction::from_rom(&rom)?;
    if operation == Some("prepare") {
        let root = args
            .get(2)
            .map(PathBuf::from)
            .map(Ok)
            .unwrap_or_else(preparation::storage_root)?;
        let prepared = preparation::prepare(
            &root,
            &mut source,
            &mut preparation::console_progress(false),
        )?;
        println!("ROM_DATA action={} profile={} source={} schema={} decoded={} runtime=offline directory={}",
            prepared.action, source.manifest.profile, source.manifest.source_rom_sha256,
            source.manifest.schema, source.manifest.coverage.join(","), prepared.directory.display());
        return Ok(());
    }
    let path = Path::new(&args[2]);
    let action = if args[0] == "extract" {
        extract(path, &mut source)?
    } else {
        verify(path, &source)?;
        "verified"
    };
    println!("ROM_DATA action={action} profile={} source={} schema={} decoded={} files=2 runtime=offline",
        source.manifest.profile, source.manifest.source_rom_sha256, source.manifest.schema,
        source.manifest.coverage.join(","));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Synthetic structure, deliberately not a copy of a game's dimension table.
    pub(super) fn fixture() -> Extraction {
        use gba_core::rom_data::*;
        let dimensions = ObjDimensions {
            entries: std::array::from_fn(|i| ObjSize {
                width: 8 << (i % 4),
                height: 8 << ((i + 1) % 4),
            }),
        };
        Extraction {
            manifest: Manifest {
                schema: SCHEMA,
                extractor_revision: EXTRACTOR_REV,
                decoder_revision: DECODER_REV,
                source_rom_sha256: "synthetic-source".into(),
                source_rom_size: 48,
                profile: "synthetic".into(),
                coverage: vec![OBJ_DIMENSIONS_ID.into()],
                resources: vec![Resource {
                    id: OBJ_DIMENSIONS_ID.into(),
                    path: OBJ_DIMENSIONS_FILE.into(),
                    format: "obj-dimensions-v1".into(),
                    sha256: String::new(),
                    dependencies: vec![],
                    sources: vec![SourceRange {
                        offset: 0,
                        length: 48,
                        sha256: gba_core::ffta_rom::digest(&dimensions.source_bytes()),
                        role: "synthetic".into(),
                    }],
                }],
            },
            dimensions,
        }
    }

    #[test]
    fn deterministic_publication_reuse_and_relocation() {
        let temp = tempfile::tempdir().unwrap();
        let a = temp.path().join("first folder/data");
        let b = temp.path().join("second");
        let mut source = fixture();
        assert_eq!(extract(&a, &mut source).unwrap(), "created");
        let before = fs::read(a.join(MANIFEST_FILE)).unwrap();
        assert_eq!(extract(&a, &mut source).unwrap(), "reused");
        assert_eq!(before, fs::read(a.join(MANIFEST_FILE)).unwrap());
        assert_eq!(extract(&b, &mut source).unwrap(), "created");
        for name in [MANIFEST_FILE, OBJ_DIMENSIONS_FILE] {
            assert_eq!(
                fs::read(a.join(name)).unwrap(),
                fs::read(b.join(name)).unwrap()
            );
        }
        let moved = temp.path().join("moved folder");
        fs::rename(&a, &moved).unwrap();
        assert_eq!(verify(&moved, &source).unwrap(), source.dimensions);
        assert!(!String::from_utf8(before)
            .unwrap()
            .contains(&temp.path().display().to_string()));
    }

    #[test]
    fn corruption_missing_files_and_edits_fail_without_overwriting() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("data");
        let mut source = fixture();
        extract(&path, &mut source).unwrap();
        let original = fs::read(path.join(OBJ_DIMENSIONS_FILE)).unwrap();
        fs::write(path.join(OBJ_DIMENSIONS_FILE), b"broken").unwrap();
        assert!(verify(&path, &source).is_err());
        assert!(extract(&path, &mut source).is_err());
        assert_eq!(fs::read(path.join(OBJ_DIMENSIONS_FILE)).unwrap(), b"broken");
        fs::write(path.join(OBJ_DIMENSIONS_FILE), &original).unwrap();
        let mut changed = source.dimensions.clone();
        changed.entries[0].width = 64;
        let bytes = toml::to_string_pretty(&changed).unwrap();
        let mut manifest = source.manifest.clone();
        manifest.resources[0].sha256 = gba_core::ffta_rom::digest(bytes.as_bytes());
        fs::write(path.join(OBJ_DIMENSIONS_FILE), &bytes).unwrap();
        fs::write(
            path.join(MANIFEST_FILE),
            toml::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();
        assert!(verify(&path, &source).is_err());
        fs::remove_file(path.join(OBJ_DIMENSIONS_FILE)).unwrap();
        assert!(verify(&path, &source).is_err());
        fs::write(
            path.join(OBJ_DIMENSIONS_FILE),
            [0; MAX_FILE_BYTES as usize + 1],
        )
        .unwrap();
        assert!(verify(&path, &source).is_err());
    }

    #[test]
    fn strict_parser_and_fixed_paths() {
        let source = fixture();
        let bytes = toml::to_string_pretty(&source.dimensions).unwrap();
        let wrong = format!("unknown = 1\n{bytes}");
        assert!(parse::<ObjDimensions>(wrong.as_bytes(), "test").is_err());
        let wrong = bytes.replacen("width = 8", "width = 65536", 1);
        assert!(parse::<ObjDimensions>(wrong.as_bytes(), "test").is_err());
        assert!(parse::<ObjDimensions>(&[255], "test").is_err());
        let fewer = ObjDimensions {
            entries: source.dimensions.entries,
        };
        let text = toml::to_string_pretty(&fewer).unwrap();
        let cut = text.rfind("[[entries]]").unwrap();
        assert!(parse::<ObjDimensions>(text[..cut].as_bytes(), "test").is_err());
        assert!(cmd(&[]).is_err());
        assert!(cmd(&["unknown".into(), "rom".into(), "out".into()]).is_err());
    }

    #[test]
    fn competing_publishers_leave_one_verified_dataset() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("data");
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(4));
        let handles: Vec<_> = (0..4)
            .map(|_| {
                let path = path.clone();
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    barrier.wait();
                    extract(&path, &mut fixture()).unwrap()
                })
            })
            .collect();
        let outcomes: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        assert_eq!(outcomes.iter().filter(|&&v| v == "created").count(), 1);
        verify(&path, &fixture()).unwrap();
        assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 1);
    }

    #[test]
    #[ignore = "requires the user's exact USA and EUR ROMs; no game data is bundled"]
    fn regional_cli_extraction_verification_and_identity() {
        let temp = tempfile::tempdir().unwrap();
        let mut decoded = Vec::new();
        let mut manifests = Vec::new();
        for (env, profile) in [
            ("FFTA_TEST_ROM_USA", "usa"),
            ("FFTA_TEST_ROM_EUR", "europe"),
        ] {
            let rom_path = std::env::var(env).unwrap();
            let output = temp.path().join(profile);
            let output_arg = output.to_str().unwrap().to_owned();
            cmd(&["extract".into(), rom_path.clone(), output_arg.clone()]).unwrap();
            let manifest = fs::read(output.join(MANIFEST_FILE)).unwrap();
            cmd(&["extract".into(), rom_path.clone(), output_arg.clone()]).unwrap();
            assert_eq!(manifest, fs::read(output.join(MANIFEST_FILE)).unwrap());
            cmd(&["verify".into(), rom_path.clone(), output_arg]).unwrap();
            let source = Extraction::from_rom(&fs::read(&rom_path).unwrap()).unwrap();
            assert_eq!(verify(&output, &source).unwrap(), source.dimensions);
            decoded.push(fs::read(output.join(OBJ_DIMENSIONS_FILE)).unwrap());
            manifests.push(manifest);
        }
        assert_eq!(decoded[0], decoded[1]);
        assert_ne!(manifests[0], manifests[1]);
        let usa = fs::read(std::env::var("FFTA_TEST_ROM_USA").unwrap()).unwrap();
        assert!(verify(
            &temp.path().join("europe"),
            &Extraction::from_rom(&usa).unwrap()
        )
        .is_err());
    }

    #[test]
    #[ignore = "requires the user's exact USA and EUR ROMs; no game data is bundled"]
    fn runtime_files_bind_transactionally_and_survive_slots_undo_and_backup_reboot() {
        use gba_core::Bus;
        use std::sync::Arc;
        for env in ["FFTA_TEST_ROM_USA", "FFTA_TEST_ROM_EUR"] {
            let temp = tempfile::tempdir().unwrap();
            let rom = fs::read(std::env::var(env).unwrap()).unwrap();
            let mut source = Extraction::from_rom(&rom).unwrap();
            let path = temp.path().join("decoded");
            extract(&path, &mut source).unwrap();
            let mut machine = super::super::make_machine(rom, None);
            let boot = machine.save_state().unwrap();
            attach_from_directory(&mut machine, &path).unwrap();
            let resources = machine.bus.rom_resources().unwrap().clone();
            assert_eq!(
                resources.file_sha256(),
                gba_core::ffta_rom::digest(&fs::read(path.join(OBJ_DIMENSIONS_FILE)).unwrap())
            );
            assert_eq!(boot, machine.save_state().unwrap());
            let offset = source.manifest.resources[0].sources[0].offset;
            let expected = machine.bus.read32(0x08000000 + offset);
            let save = temp.path().join("saves/test.sav");
            fs::create_dir_all(save.parent().unwrap()).unwrap();
            let mut slots = super::super::save_state::Slots::new(&save, &machine.bus.rom);
            let preview = vec![[0; 4]; 240 * 160];
            slots.save(&machine, 1, &preview, false).unwrap();
            machine.cpu.regs[4] = 37;
            let before = machine.save_state().unwrap();
            slots.load(&mut machine, 1).unwrap();
            assert_eq!(machine.cpu.regs[4], 0);
            assert!(Arc::ptr_eq(
                machine.bus.rom_resources().unwrap(),
                &resources
            ));
            slots.undo(&mut machine).unwrap();
            assert_eq!(machine.save_state().unwrap(), before);
            assert!(Arc::ptr_eq(
                machine.bus.rom_resources().unwrap(),
                &resources
            ));
            // A new activation must reject absent files; a running session
            // retains its already verified immutable in-memory resources.
            fs::remove_file(path.join(OBJ_DIMENSIONS_FILE)).unwrap();
            assert!(attach_from_directory(&mut machine, &path).is_err());
            assert!(Arc::ptr_eq(
                machine.bus.rom_resources().unwrap(),
                &resources
            ));
            let mut fresh = super::super::make_machine(machine.bus.rom.clone(), None);
            assert!(attach_from_directory(&mut fresh, &path).is_err());
            assert!(fresh.bus.rom_resources().is_none());
            assert_eq!(machine.bus.read32(0x08000000 + offset), expected);
            let count = resources.stats();
            machine.load_state(&boot).unwrap();
            assert!(Arc::ptr_eq(
                machine.bus.rom_resources().unwrap(),
                &resources
            ));
            assert_eq!(resources.stats(), count);
            let length = machine.bus.save_data().unwrap().len();
            let backups = super::super::save_backups::Backups::new(&save);
            fs::write(&save, vec![7; length]).unwrap();
            backups.capture_startup().unwrap();
            let generation = backups.choices(length)[0].generation.unwrap();
            machine.cpu.regs[4] = 99;
            backups.restore(generation, &mut machine, &boot).unwrap();
            assert_eq!(machine.cpu.regs[4], 0);
            assert!(Arc::ptr_eq(
                machine.bus.rom_resources().unwrap(),
                &resources
            ));
            assert_eq!(machine.bus.save_data().unwrap(), vec![7; length]);
            assert_eq!(machine.bus.read32(0x08000000 + offset), expected);
            assert!(resources.stats().bus_reads > 0);
        }
    }

    #[cfg(unix)]
    #[test]
    fn symbolic_resources_are_rejected() {
        let temp = tempfile::tempdir().unwrap();
        let real = temp.path().join("real");
        let alias = temp.path().join("alias");
        let mut source = fixture();
        extract(&real, &mut source).unwrap();
        std::os::unix::fs::symlink(&real, &alias).unwrap();
        assert!(verify(&alias, &source).is_err());
        fs::rename(real.join(OBJ_DIMENSIONS_FILE), temp.path().join("outside")).unwrap();
        std::os::unix::fs::symlink(temp.path().join("outside"), real.join(OBJ_DIMENSIONS_FILE))
            .unwrap();
        assert!(verify(&real, &source).is_err());
    }
}
