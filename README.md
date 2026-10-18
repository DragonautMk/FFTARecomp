<p align="center">
  <img src="Image/logo.png" alt="FFTARecomp logo" width="50%">
</p>

<h1 align="center">FFTARecomp</h1>

<p align="center">
  <b>Final Fantasy Tactics Advance, running natively on your Windows PC.</b><br>
  Widescreen, video and audio options, and a one-click installer.
</p>

<p align="center">
  <a href="https://github.com/Jorge262007/FFTARecomp/releases"><img src="https://img.shields.io/github/v/release/Jorge262007/FFTARecomp?include_prereleases&label=release" alt="Release"></a>
  <img src="https://img.shields.io/badge/platform-Windows%2010%2F11-blue" alt="Windows">
  <img src="https://img.shields.io/badge/built%20with-Rust-orange" alt="Rust">
</p>

---

## ✨ What is this?

**FFTARecomp** turns *Final Fantasy Tactics Advance* (USA version) into a native Windows program. Instead of running in an emulator, the game is recompiled from your own ROM, so it runs smoothly, needs no BIOS, and gets modern options. It is based on [gba-recomp](https://github.com/JRickey/gba-recomp) by JRickey.

## 🎮 Features

- 🖥️ **Widescreen:** Auto, 16:10, 16:9 (default), 20:9, 21:9 and experimental 32:9.
- 🎨 **Video options:** screen model, grid, scaling and vsync.
- 🔊 **Audio options** and volume control.
- ⏩ **Fast forward.**
- 🕹️ **Remappable controls.**
- 📦 **One-click installer:** pick your ROM and you're done.
- 🚫 **No console BIOS needed.**

## 🕹️ Controls

> 💡 To open the **Options Menu**, press **Esc** while in-game.

| PC | Game Boy Advance |
|----|------------------|
| Z | A |
| X | B |
| A | L |
| S | R |
| Enter | Start |
| Right Shift | Select |
| Up Arrow | Up |
| Down Arrow | Down |
| Left Arrow | Left |
| Right Arrow | Right |
| Tab | Fast Forward |

Controls can be remapped from the Options Menu.

## 🚀 Installation

1. Download `FFTARecomp.rar` from [Releases](https://github.com/Jorge262007/FFTARecomp/releases) and extract it.
2. Run `FFTARecomp.exe`, choose your ROM and press **Recompile & Install**.
3. Wait for the five steps to finish, then press **Play**. ▶️

> 💡 You need your own copy of *Final Fantasy Tactics Advance (USA)*. This project does not include any game data.

## 🔨 Building from source

You need [Rust](https://rustup.rs) and Windows PowerShell. Then run a single script from the repo root:

```powershell
git clone https://github.com/Jorge262007/FFTARecomp.git
cd FFTARecomp
.\build-all.ps1
```

The script does everything in order:

1. Builds the whole workspace (`cargo build --release`).
2. Builds the installer (`cargo build --release -p installer`).
3. Copies `installer.exe` into `Installer\`, next to the static `payload` folder.
4. Downloads the compiler toolchain ([w64devkit](https://github.com/skeeto/w64devkit) 2.10.0) into `Installer\payload\toolchain`, only if it is missing.

When it finishes, the final installer is in:

```
FFTARecomp\Installer\
```

If PowerShell blocks the script, run it with:

```powershell
powershell -ExecutionPolicy Bypass -File .\build-all.ps1
```

To force a fresh toolchain download, delete `Installer\payload\toolchain` and run the script again.

> ⏱️ The first build can take several minutes.

## ⚠️ Good to know

- 🐢 The first installation builds the game on your PC and can take several minutes. It is normal for the PC to slow down and for CPU/RAM usage to rise. This only happens once.
- 💾 The install is larger than the game alone because it includes the compiler toolchain needed to build it.
- 🐛 This is a work in progress. If you find a bug, please open an [issue](https://github.com/Jorge262007/FFTARecomp/issues) and tell us what happened and your PC specs.

> ### 🛡️ Windows or your antivirus shows a warning?
> **The executables are not code-signed**, so Windows SmartScreen or your antivirus may show a warning. Click **More info → Run anyway**, or right-click the `.rar` → Properties → tick **Unblock** before extracting.
>
> If you have any doubts, you can check the **VirusTotal report** included in each release.

## 📜 License and credits

- License: MIT, see [LICENSE](LICENSE). It keeps the original author's copyright line.
- Based on [gba-recomp](https://github.com/JRickey/gba-recomp) by JRickey (MIT OR Apache-2.0).
- `gamedb.sqlite` is by JRickey, released under CC0 1.0.
- Includes [w64devkit](https://github.com/skeeto/w64devkit) (GCC, binutils and other tools under GPL and compatible licenses), unmodified. Their licenses are in `payload/toolchain`.
- *Final Fantasy Tactics Advance* is owned by Square Enix / Nintendo. This project is not affiliated with them and does not distribute ROMs or copyrighted game data.

---

<p align="center">Made by a fan, for fans.</p>
