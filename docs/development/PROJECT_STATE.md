# Current development knowledge

This file stores durable facts and entry points. Chronological changes and their
validation belong in [CHANGELOG.md](../../CHANGELOG.md) and `docs/changes/`.
Read current source and manifests before relying on older build instructions.

## Architecture and current scope

- The root Cargo workspace and `AndroidPort/` are separate trees. A root fix
  does not automatically update AndroidPort.
- `crates/recomp/src/play.rs` connects the desktop event loop, input, menu and
  presentation. `crates/menu` owns UI state and rendering; `crates/input-config`
  owns persisted bindings and combo capture/latching. `crates/screen` presents
  the framebuffer and UI layers.
- `crates/gba-core` implements the emulated machine. `crates/recomp-core` emits
  native code; TinyCC compiles it. Runtime execution can still use interpreter
  fallbacks. Inspect cache revision/ABI requirements when changing emission or
  guest semantics; a host-only overlay does not itself change the guest ABI.
- Desktop packaging covers Windows and Linux. The release profile uses thin
  LTO and 16 codegen units; the Cargo manifest is authoritative.

## Container builds

- `Dockerfile.linux` and `scripts/build_linux_docker.sh` build Linux amd64 using
  Rust 1.94.0 / Debian Bookworm with ALSA/udev development dependencies inside
  the container. BuildKit caches Cargo/target separately from the host checkout.
  The scratch export stage writes only the portable tar.gz and BUILD-INFO.txt
  to `dist/docker-linux/`; runtime GUI/system libraries remain host dependencies.
  `.dockerignore` allows desktop build inputs and excludes game/personal data.
  Container export is gated by serial input/config, menu, core, runtime and
  screen suites; optional local-ROM/GPU cases remain separate. The exported
  runtime requires glibc 2.35 and previously passed a native Ubuntu Xvfb
  save/load/relocation check.
  Older-distro and physical-controller compatibility remain unverified.
  See [Docker workflow](../docker.md). Keep the direct Ubuntu-built archive
  distinct, and establish actual libc requirements for each build environment.

- `Dockerfile.windows` cross-compiles Windows GNU x86_64 with MinGW from the
  pinned Linux Rust base, builds TinyCC with distinct native/target compilers,
  and runs input/config, menu and runtime tests under container Wine.
  `scripts/build_docker.ps1` is the Windows-host entry point: default Windows,
  `-Platform linux` for Linux. Docker Desktop must use Linux containers.
  `scripts/build_windows_docker.sh` offers the Windows target from Bash.
  Export is `dist/docker-windows/`; it contains no game or personal data.
  Resource build scripts branch on Cargo's target OS rather than build-host cfg.
  The Windows builder reuses the pinned Rust image and copies its toolchain
  into Debian Trixie for MinGW 14 / Wine 10. Windows/Wine tests gate exports
  on serial core and screen suites as well as input/config, menu and runtime.
  Earlier packaged PE resource/import checks and a simulated-XInput portable
  save/load/move smoke also passed. PowerShell 7.4.6
  Linux mock-CLI validation passed; native Windows/Desktop/PowerShell 5.1 remain
  unverified.

## Desktop behavior

- Release desktop audio/presentation resources in `PlayApp::exiting`, before
  `EventLoop::run_app` consumes/drops the display connection. Release surfaces
  and their contexts before the window, including error exits. Holding them
  until PlayApp's later field destruction caused a reproducible Wayland/Mesa
  EGL SIGSEGV even when Vulkan rendered the game. Native Ubuntu 26.04 GNOME /
  AMD RADV close-event regression checks pass; see [portable notes](../portable.md).

- [Desktop portable editions](../portable.md): `portable.txt` beside the actual
  executable opts into shared `input_config::storage::portable_root()` resolution.
  Settings/labels use `Data/config`, native translations `Data/cache`, cartridge
  progress `Data/saves/ffta.sav`; backups and slots remain beside that save.
  No AppData ROM/settings/save-pointer fallback in this mode. The wrapper copies
  a first-launch selected ROM to `ffta.gba`. Explicit environment path overrides
  still apply. Move the entire folder while closed; paths are resolved afresh.
  `scripts/package_ffta_windows.py` and Windows `build-all.ps1` create a portable
  ZIP by explicit file selection. Native Linux uses
  `scripts/package_ffta_linux.sh` and the shared
  `scripts/package_ffta_portable.py --platform linux` for a tar.gz with executable
  modes preserved and `tcc/libtcc.so`. The current archive targets the Ubuntu
  26.04 x86_64 host; broad older-distro/Steam Deck compatibility is unverified.
  Existing installed behavior needs no marker;
  migration is an explicit copy, never a silent move of personal data.
- Surface acquisition timeouts skip a host presentation while retaining the GPU.
  Lost/Outdated reconfigure and retry once per call; three failed recovery calls
  trigger software fallback. Successful acquisition resets the budget; memory
  exhaustion/unknown errors fall back immediately. Zero-sized surfaces skip
  acquisition, and skipped frames do not increment FPS EXT. Event-loop pacing
  remains responsible for retries; no tight retry loop or per-frame error log.
- The row-endpoint panel-bound scan is a test-only experiment. It preserves
  the full scan's results and reduces synthetic panel cost, but production
  integration was withheld after slower 3000-frame introductory A/B replays.
  Keep the current full scan until representative end-to-end benefit is shown;
  see the optional diagnostic in the performance measurement workflow.
- GPU UI layers retain LUT contents and skip identical uploads; FPS pixels use
  process-unique render revisions. Unversioned game UI/world data still updates
  normally. Resource recreation clears caches, and color comparison handles
  in-place changes. FIFO audio taps drain without discarding producer capacity;
  `FFTA_FRAME_PACING` is resolved at startup. See [FPS notes](../fps_overlay.md)
  and the performance TODO for remaining optimization work.
- [FPS overlay](../fps_overlay.md): Video > FPS counter; off/internal/external/
  both. INT measures completed guest frames per wall second; EXT measures host
  presentations, not display scanout. Configuration uses `video.fps` in
  `%APPDATA%\FFTARecomp\av.cfg` in installed Windows mode.
- [Menu shortcuts](../menu_shortcuts.md): configurable single controls/chords
  under Input > Keyboard, Mouse & Keyboard, or Gamepad > Open menu. Defaults
  Escape and Start+Back/Select. Escape remains a recovery shortcut; Guide/Mode
  works where exposed. Persistence uses `key.menu` and `pad.menu` in
  `%APPDATA%\FFTARecomp\input.cfg` in installed mode.
- Save resolution lives in `crates/recomp/src/save_path.rs`. `FFTA_SAVE_PATH`
  overrides the selected location. Windows remembers the shared save location
  under `%LOCALAPPDATA%\FFTARecomp\shared-save-path.txt`. Save files are raw
  cartridge `.sav` data (64 KiB for the tested EUR game), not emulator save
  states. Portability requires matching game edition/region and a tool/device
  that accepts that save format; no new conversion feature was implemented.

- [Save states](../save_states.md): pause menu > Save states, nine controller-
  operated slots with embedded previews and in-menu overwrite/error feedback.
  Storage: `SaveStates/<ROM SHA-256>/slot-1.ffstate` through `slot-9.ffstate`,
  relative to the selected cartridge save directory. A new session starts at
  slot 1; Left/Right wraps between all nine. Legacy raw states can be copied
  into an unused slot and loaded without an image until resaved.
  Files use `.ffstate`, carry ROM/BIOS identity and a versioned machine snapshot,
  and are separate from cartridge `.sav` files. Loading also rewinds cartridge
  memory, which normal autosave/exit later persists. Native translations and host
  resources are not stored; input recording must stop before loading.
  Undo last operation is one RAM-only session entry across all slots: undo save
  restores the previous file/preview (or an empty slot), undo load restores the
  pre-load machine. Failed operations preserve the entry; successful undo
  consumes it without redo. A detected externally changed/missing slot prevents
  undo save. Load and undo load both use the shared frontend restore reset.

- [Cartridge save backups](../save_backups.md): capture the selected `.sav` at
  desktop startup, retain two raw copies under `SaveBackups/<save-filename>/`,
  and restore via the controller-operated Save backups menu. Sequence/hash
  filenames identify order and integrity. Restore prepares a clean boot from
  the session's startup snapshot before atomic file replacement, resets the
  machine and frontend, and clears save-state undo. It preserves both backup
  copies and the nine state slots. Backup failure aborts before gameplay writes.
  Missing remembered/default primary files keep their recovery location when
  recognized backup copies are present, rather than selecting an unrelated save.

## Validation knowledge and remaining uncertainty

- [Desktop performance TODO](PERFORMANCE_TODO.md) tracks task completion separately
  from partial progress. Its first P0 tranche adds native Linux/Windows/Wine
  comparisons, optional final serialized-state verification, bounded headless
  frame-time capture and opt-in interactive CPU phase profiling, including
  separate CPU-side presenter setup/upload/acquire/encode/submit/present times. See the
  [measurement workflow](PERFORMANCE_WORKFLOW.md). These tools do not establish
  an AI-turn speedup or GPU execution timings. The frontend fails before advancing
  temporal history on an exhausted frame guard; stats display separate native
  block/fallback-step counts rather than their mixed-unit percentage.
  Remaining P0 work includes the identified slow-turn replay, detailed PPU
  internals and actual GPU execution timing. Start with a reproducible enemy-turn replay and
  stage/p95/p99 measurements, then bounded surface recovery and redundant
  upload/allocation reduction. GPU LUT/temporal processing is conditional on
  measured cost; preserve per-guest-frame history and the CPU reference path.
  Guest AI/GPU or full-PPU rewrites are not the first-line approach.
- CPU PPU timing uses scoped thread-local capture outside machine/save state.
  `FFTA_PROFILE=1` includes scanline/zoom/core-remainder rows on desktop;
  `--ppu-profile` enables separate headless diagnostic runs after warm-up.
  Disabled entry points check capture state without reading a clock. Capture
  is bounded, incomplete frames are discarded and parallel zoom measures the
  caller's wall time including worker waits. No guest or translation ABI change.
- `runc` guest-cycle profiling (`RECOMP_COST_FROM`) observes the shared native
  dispatch loop, preserving normal HLE/interpreter gates and strict fallback
  behavior. `COST_DISPATCH` separates native, missing, system and intentional
  hook dispatches. Its per-PC charged cycles are guest time, not host timings;
  keep the census disabled for timing comparisons. See the performance workflow.
- [Windows runtime performance](../windows_runtime_performance.md) documents
  the tested replay and matching framebuffer/OAM results. Timing improvements
  were measured under Wine, with environment lookup costs specific to that
  setup; they do not establish native Windows gains or AI-turn performance.
- Windows GNU tests and interactive FPS/menu checks ran through Wine. Menu
  gamepad smoke checks used a simulated XInput controller. Native Windows and
  physical controller coverage remain to be obtained.
- Some gba-core tests share display globals. Use serial execution for that
  suite; distinguish a baseline parallel failure from a regression.
- Optional TinyCC observer checks need its ELF relocatable object loader even
  for Windows. Record the backend and checks actually used.
- Keep test saves/config separate. Wine may replace APPDATA supplied by the
  caller; verify the effective location within the isolated prefix.
- Local `target/` and `dist/` files are ignored build artifacts, not portable
  prerequisites. Discover installed toolchains and rebuild when absent. Never
  include ROMs, saves, recordings or test controller DLLs in release archives.

## Maintaining this knowledge

Follow [the memory workflow](MEMORY_WORKFLOW.md) and the local skills routed
from [AGENTS.md](../../AGENTS.md). Update this page when facts or decisions
change, rather than adding a second chronological log here.
