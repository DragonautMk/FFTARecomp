#![windows_subsystem = "windows"]
use std::{env, fs, path::PathBuf, process::Command};
use std::os::windows::process::CommandExt;

fn fail(dir: &PathBuf, msg: &str) {
    let _ = fs::write(dir.join("play-error.log"), msg);
}

fn main() {
    let exe_dir = env::current_exe().unwrap().parent().unwrap().to_path_buf();
    let recomp = exe_dir.join("recomp.exe");
    let toolchain = exe_dir.join("toolchain").join("bin");

    let local = PathBuf::from(env::var("LOCALAPPDATA").unwrap_or_default())
        .join("FFTARecomp").join("ffta.gba");
    let rom = if exe_dir.join("ffta.gba").exists() { exe_dir.join("ffta.gba") } else { local };

    if !recomp.exists() { return fail(&exe_dir, &format!("Falta {}", recomp.display())); }
    if !rom.exists()    { return fail(&exe_dir, &format!("Falta la ROM: {}", rom.display())); }

    let path = format!("{};{}", toolchain.display(), env::var("PATH").unwrap_or_default());
    let r = Command::new(&recomp)
        .arg("play")
        .arg("--gamedb").arg(exe_dir.join("gamedb.sqlite"))
        .arg(&rom)
        .current_dir(&exe_dir)
        .env("PATH", path)
        .env("RECOMP_JOBS", "2")
        .creation_flags(0x0800_0000)
        .output();

    match r {
        Err(e) => fail(&exe_dir, &format!("No se pudo lanzar recomp.exe: {e}")),
        Ok(o) if !o.status.success() => fail(
            &exe_dir,
            &format!(
                "recomp.exe termino con {}\n--- stderr ---\n{}\n--- stdout ---\n{}",
                o.status,
                String::from_utf8_lossy(&o.stderr),
                String::from_utf8_lossy(&o.stdout)
            ),
        ),
        Ok(_) => {}
    }
}