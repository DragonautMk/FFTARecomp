# Project scripts

Platform-specific entry points live in these folders:

- `linux/`: Linux Docker build/extraction entry points and the native Linux
  packager. `build_windows_docker.sh` builds the Windows package from a Linux host.
- `windows/`: PowerShell entry points, the Windows portable-packager wrapper,
  MSYS2 TinyCC builder and Windows-to-Linux cross-build helper with its C linker.
- `android/`: Android APK builder and Android-device camera helper. The current
  APK builder requires a Windows SDK/NDK toolchain as documented in its header.
- `unix/`: TinyCC builder shared by Linux and macOS.
- `tests/`: shared Python tooling tests.

Cross-platform extraction, packaging, release, analysis and project-memory
tools stay directly in `scripts/`. For example,
`benchmark_windows_runtime.py` supports native and Wine runtimes despite its
historical filename; `package-release.sh` runs on all three CI host platforms.
The root `build-all.ps1` remains the Windows all-platform build entry point.
It launches the Linux cross-build helper with the current PowerShell executable,
so PowerShell 7 does not try to find the Windows PowerShell `powershell.exe`
inside its own installation directory.
`AndroidPort/` has its own scripts and is managed separately.

From the repository root, extract readable ROM data with Docker:

```bash
bash scripts/linux/extract_rom_data_docker.sh
```

```powershell
.\scripts\windows\extract_rom_data_docker.ps1
```

For portable builds:

```bash
bash scripts/linux/build_linux_docker.sh
bash scripts/linux/build_windows_docker.sh
```

```powershell
.\scripts\windows\build_docker.ps1
```

Both extraction entry points use
`rom/1194 - Final Fantasy Tactics Advance (E)(Surplus).gba` relative to the
repository root by default. Override it with a first ROM argument on Bash or
`-RomPath` on PowerShell; absolute ROM paths are accepted and relative ROM paths
always start at the repository root, even when launched from another directory.

Scripts resolve their repository from their own location. Explicit output
paths remain relative to the calling directory. See
[Docker workflows](../docs/docker.md) and [extraction options](../docs/rom_data.md).

The Linux Docker builder saves each run's output and errors under
`target/build-logs/linux-*.log`. On failure it waits for Enter in an interactive
terminal; noninteractive automation receives the original exit code immediately.
