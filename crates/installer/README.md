# FFTARecomp - instalador

Instalador de escritorio (Rust + egui) que toma **tu propia ROM** de Final Fantasy Tactics Advance
y la instala junto al reproductor y tus ajustes. Este repositorio **no contiene ninguna ROM, asset ni
archivo derivado del juego**,.



## Compilar (Windows)
Requisitos: Rust (rustup) y el toolchain MSVC.
Coloca tus propios `assets/logo.png` y `assets/icon.ico`, luego:

    cargo run --release

El instalador busca una carpeta `payload/` junto al `.exe` con `FFTARecomp.exe` y los ajustes
(widescreen, etc.); se copia entera a `%LOCALAPPDATA%\FFTARecomp`, junto con la ROM como `ffta.gba`.

## Aviso
Proyecto de hobby, sin afiliacion con Square Enix ni Nintendo. Final Fantasy Tactics Advance es
marca y propiedad de sus titulares. Usa solo una ROM de un cartucho que poseas.

## Linux

Build on Linux with Rust, clang, pkg-config, ALSA, udev, X11, XKB,
Wayland and GTK development libraries. On Ubuntu:

```sh
sudo apt-get install clang pkg-config libasound2-dev libudev-dev libx11-dev libxkbcommon-dev libwayland-dev libgtk-3-dev
bash scripts/package_ffta_linux.sh
```

Run this from the repository root. The archive is
`Installer/Linux/FFTAInstaller-linux-x86_64.tar.gz`. Extract it and run `./FFTAInstaller`.
The payload contains native Linux `recomp` and `FFTARecomp`; compilation of the
user ROM requires clang or gcc on PATH. No ROM, save, compiled ROM library or
runtime assets directory is included. Source assets remain necessary for builds.

The installer uses the same ROM hash/byte checks on Windows and Linux. Linux
shortcuts are desktop entries, and installed binaries receive execute permission.
File selection uses Windows native dialogs, KDE `kdialog`, or GTK `zenity`.
On Linux the desktop's selector is preferred when installed; if neither tool
can run, the installer opens its built-in browser. Cancelling closes the dialog.
The manual workflow `installer-linux.yml` builds and packages on Ubuntu; it does
not publish a release. Linux execution has not been tested on the Windows host.
