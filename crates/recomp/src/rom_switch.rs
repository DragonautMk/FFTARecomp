//! Native ROM picker; cancellation/validation failure never exits the running game.
use std::path::{Path,PathBuf};

pub fn pick(current_save:&str)->Option<PathBuf> {
    let mut dialog=rfd::FileDialog::new()
        .set_title("Choose FFTA ROM - USA or Europe")
        .add_filter("Game Boy Advance ROM",&["gba"]);
    if let Some(parent)=Path::new(current_save).parent().filter(|p|!p.as_os_str().is_empty()) {
        dialog=dialog.set_directory(parent);
    }
    dialog.pick_file()
}
pub fn validate(path:&Path)->Result<(),String> {
    let bytes=std::fs::read(path).map_err(|e|format!("Cannot read {}: {e}",path.display()))?;
    if gba_core::ffta_rom::version(&bytes).is_none() {
        return Err("Choose an original supported Final Fantasy Tactics Advance USA or Europe ROM. Its SHA256 must match the supported release.".into());
    }
    if let Some(manifest)=crate::packfile::load() {
        let manifest=manifest?;
        if crate::rom_sha256(&bytes)!=manifest.rom_sha256 {
            return Err("This packaged executable is pinned to another ROM. The current game will stay open.".into());
        }
    }
    Ok(())
}
pub fn launch(path:&Path, save:&str)->Result<(),String> {
    let exe=std::env::current_exe().map_err(|e|e.to_string())?;
    let mut command=std::process::Command::new(exe);
    command.arg("play").arg(path);
    // Keep the same progress even when the new ROM has another name or directory.
    let save=std::path::absolute(save).map_err(|e|format!("Cannot locate save: {e}"))?;
    command.env("FFTA_SAVE_PATH",&save);
    #[cfg(windows)] {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000); // No extra console window.
    }
    command.spawn().map(|_|crate::save_path::remember(&save)).map_err(|e|format!("Cannot start the selected ROM: {e}"))
}
pub fn show_error(message:&str) {
    rfd::MessageDialog::new().set_title("Switch ROM")
        .set_description(message).set_level(rfd::MessageLevel::Error)
        .set_buttons(rfd::MessageButtons::Ok).show();
}
#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "requires FFTA_TEST_ROM pointing to the user's ROM"]
    fn supported_user_rom_passes_validation_without_launching_or_saving() {
        let path=std::env::var_os("FFTA_TEST_ROM").expect("FFTA_TEST_ROM required");
        super::validate(std::path::Path::new(&path)).unwrap();
        println!("SWITCH_ROM validation=accepted launch=false save=false");
    }
    #[test]
    fn missing_and_non_rom_files_are_rejected_before_launch() {
        let dir=std::env::temp_dir().join(format!("ffta-switch-test-{}",std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let bad=dir.join("not a ROM.gba");
        assert!(super::validate(&bad).unwrap_err().contains("Cannot read"));
        std::fs::write(&bad,b"not a game image").unwrap();
        assert!(super::validate(&bad).unwrap_err().contains("SHA256"));
        std::fs::remove_file(bad).unwrap();std::fs::remove_dir(dir).unwrap();
    }
}
