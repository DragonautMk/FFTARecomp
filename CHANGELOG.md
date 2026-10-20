# Changelog

Unreleased development changes. Generated from `docs/changes/*.json`.
Edit the structured records, then run `python3 scripts/project_memory.py render`.

## 2026-10-20 — Reduce desktop runtime diagnostic overhead

Type: performance. Platforms: Windows (Wine), Linux.

**Changes**

- Hoist tracing configuration out of native dispatch and filter unrelated diagnostic addresses early, preserving guest behavior and cache ABI (commit 0f46623).
- Correct the optional native-observer test loader for TinyCC relocatable output and add a repeatable Windows runtime benchmark.

**Validation**

- 190 Windows tests passed through Wine; 159 Linux core tests passed serially before and after.
- Framebuffers and OAM matched over 6,000 replay frames; three timed runs after 120 warmup frames gave median 16.043990 to 4.092949 ms/frame (74.49% reduction under Wine).

**Limitations**

- Wine environment lookup overhead can amplify the result; native Windows speed and AI-turn performance remain unmeasured.
- The baseline parallel core suite has failures caused by shared display globals.

Documentation: [windows_runtime_performance.md](docs/windows_runtime_performance.md).

[Change record](docs/changes/2026-10-20-windows-runtime-performance.json)

## 2026-10-20 — Ship a portable Windows bundle with local settings, saves and cache

Type: changed. Platforms: Windows (Wine), Desktop storage.

**Changes**

- Enable executable-relative portable storage with portable.txt: Data/config for settings and labels, Data/cache for translations and Data/saves/ffta.sav for cartridge progress, with existing backup and nine-slot directories alongside it.
- Keep portable runs independent of installed AppData ROMs, remembered save pointers and settings; retain explicit environment overrides and installed behavior when the marker is absent.
- Offer a first-launch ROM picker and stage a complete copy beside the executable without overwriting a concurrently created destination; report startup errors in a dialog and play-error.log.
- Replace the default Windows installer packaging in build-all.ps1 with an explicit-input portable ZIP packager including runtime, TinyCC source/notices and migration documentation, excluding ROMs and personal Data.

**Validation**

- Windows GNU release build passed; 26 input/config and 42 runtime tests passed through Wine, including marker opt-in and folder relocation. Two optional ROM-specific runtime cases remained ignored.
- Actual wrapper plus simulated-XInput smoke passed from an unrelated working directory with spaces in the bundle path: created a preview slot and local native cache, moved the folder, loaded the slot and reused the unchanged cache; two startup backup copies retained.
- First-launch file-dialog selection copied the ROM byte-for-byte, launched gameplay and displayed the portable config internal-only FPS overlay. Installed AppData file hashes and modification times remained unchanged across all three launches.
- Final locked/offline Windows release build, archive CRC/input equality, TinyCC/source/license and private-data/test-double exclusion checks passed. Project memory coverage (including --base HEAD), updated skill validation and whitespace checks passed.

**Limitations**

- Native Windows and physical controllers were not tested; GUI validation used Wine and simulated XInput.
- The full PowerShell multi-platform build sequence was not executed on this Linux host. Linux/Steam Deck installer and Android packaging remain separate.
- Existing progress/settings require explicit copying as documented; explicit environment overrides may place files outside the portable folder. Keep the folder writable and move it only while the game is closed.

Documentation: [portable.md](docs/portable.md), [PROJECT_STATE.md](docs/development/PROJECT_STATE.md).

[Change record](docs/changes/2026-10-20-windows-portable-edition.json)

## 2026-10-20 — Build Windows portable releases from PowerShell with container-managed dependencies

Type: added. Platforms: Windows GNU x86_64 (Docker cross build), Windows host PowerShell entry point.

**Changes**

- Add a digest-pinned Rust 1.94.0 / Debian Trixie Windows builder with MinGW, the Windows Rust standard library and 64-bit Wine; export the standalone ZIP and build information.
- Add PowerShell Windows/Linux selection and a Windows-target Bash launcher, keeping Docker Desktop in Linux-container mode and development dependencies off the host.
- Compile TinyCC build-time helpers with an explicit host compiler and the Windows library with the target compiler; generate Windows icon resources based on the Cargo target OS during cross-compilation.
- Extend private-data context exclusions and Windows validation guidance, preserving the existing Linux builder and its separate output/cache.

**Validation**

- PowerShell 7.4.6 Linux-container parsing and simulated-CLI tests passed for both platforms, checkout/output paths containing spaces, unrelated working directory, invalid inputs, missing Docker and nonzero Docker exit propagation. Bash and Python syntax checks passed.
- Actual Windows Docker build/export passed with Rust 1.94.0, MinGW GCC 14-win32, mingw-w64 12 headers and Wine 10 on Debian Trixie. 26 input/config, 33 menu and 42 runtime tests passed under container Wine; two optional runtime cases remained ignored.
- Final packaged executables are PE x86_64 with icon, group-icon and version resources. Imports contain no external GCC/libstdc++/winpthread runtime libraries; archive CRC, BUILD-INFO checksum, portable marker, TinyCC source/notices and private-data exclusion checks passed.
- Wine-host simulated-XInput smoke of the exported wrapper passed: preview slot save, normal close, folder relocation, slot load, unchanged native cache bytes/mtime, one then two startup backups and unchanged installed AppData files.
- The existing Linux Docker build/export passed 26 input/config and 41 runtime tests after the shared TinyCC/resource build-script changes.
- Final export passed the same 101 Wine tests and archive/PE checks; a second simulated-XInput smoke passed on the final rebuilt TinyCC DLL. Warm-cache invocation from /tmp with a space-containing output path exported byte-identical ZIP/build-info files. Project memory coverage, Windows skill validation and whitespace checks passed.

**Limitations**

- Native Windows/Docker Desktop, Windows PowerShell 5.1 and physical controllers were not available. Windows binary tests used Wine; PowerShell 7.4.6 argument/error handling was exercised on Linux with a simulated Docker CLI.
- Docker Desktop must use Linux containers; both builders target amd64, requiring supported emulation on other host architectures. Network is required for the initial toolchain and dependency downloads.
- Base/toolchain image digests and Cargo.lock are pinned, while Debian package updates and generated archive/PE timestamps prevent a claim of bit-for-bit reproducible builds. GUI smoke used a preplaced user ROM outside the container.

Documentation: [docker.md](docs/docker.md), [portable.md](docs/portable.md).

[Change record](docs/changes/2026-10-20-windows-docker-build.json)

## 2026-10-20 — Release desktop resources before the Wayland event loop is destroyed

Type: fixed. Platforms: Linux x86_64 / Ubuntu 26.04 GNOME Wayland.

**Changes**

- Stop the audio stream and drop GPU/software presentation resources and window in the final winit callback, on both successful and failed exits, while the display connection remains alive.
- Preserve final cartridge persistence and the common window-close/menu-Quit path; document the Mesa/EGL shutdown failure and portable update procedure.

**Validation**

- Original runtime reproduced SIGSEGV in libwayland-client through libEGL_mesa on native GNOME Wayland, AMD Radeon 780M / RADV Mesa 26.0.8, after saving the cartridge. A temporary local hook delivered WindowEvent::CloseRequested after 180 guest frames.
- With the fix, the same native Wayland test exited normally under GDB; two additional actual-launcher runs saved and exited with status 0. The real audio stream ran muted. The temporary hook was removed before packaging.
- Final Linux Docker build/export passed 26 input/config and 41 runtime tests; two optional ROM-specific runtime tests remained ignored. Archive checksum/build-info, executable modes, ELF binaries, bundled TinyCC, private-data exclusion and removal of the temporary close hook were verified.
- Final exported launcher passed Xvfb/Mesa llvmpipe preview-slot save, normal window close, folder relocation, slot load and another normal close; native cache bytes/mtime, two startup backups and installed per-user data remained correct.
- Final exported launcher also passed an actual X11 WM_DELETE_WINDOW request on the desktop with AMD Radeon 780M / RADV Vulkan: cartridge save completed and runtime exited with status 0.

**Limitations**

- Wayland close events were injected automatically rather than generated by a physical title-bar click. Native Windows and physical-controller shutdown were not tested for this fix.

Documentation: [portable.md](docs/portable.md), [PROJECT_STATE.md](docs/development/PROJECT_STATE.md).

[Change record](docs/changes/2026-10-20-wayland-shutdown-crash.json)

## 2026-10-20 — Save and load persistent machine checkpoints from the desktop menu

Type: added. Platforms: Windows (Wine), Linux core.

**Changes**

- Add Save states > Save state... / Load state... to the pause menu with file selection, error feedback and persistent .ffstate files.
- Capture CPU, hardware scheduling, memory, cartridge protocol/data, graphics and enhanced audio state in a versioned ROM/BIOS-bound format; validate and decode before changing the running machine.
- Write checkpoints atomically and protect cartridge .sav destinations; loading cancels pointer requests, resets audio/presentation history and refuses active input recording.
- Exclude private state files from Git, source packages and release audits; document cartridge-save behavior and future state-format versioning requirements.

**Validation**

- Windows GNU release tests through Wine: 163 core, 27 menu and 29 runtime tests passed; four snapshot tests also passed after extending banked CPU register coverage.
- Optional native replay passed with enhanced audio taps: 120 framebuffer hashes and complete final machine state matched after rewind and after loading into a fresh machine.
- Linux core: 163 tests passed serially. Interactive Windows/Wine smoke created and loaded a checkpoint, loaded again after restart, resumed and exited cleanly using isolated test data.
- Five memory-tool tests, updated skill validation and whitespace checks passed. The Windows archive passed CRC checks, executable hash comparisons and private-data/test-double exclusion checks.

**Limitations**

- Native Windows and physical-controller manual validation remain outstanding; AndroidPort is a separate tree and is unchanged.
- Linux desktop compilation needs the missing ALSA development package on this host.
- States require identical ROM/BIOS content and a compatible state version; they are not cartridge saves or another emulator's format.
- Loading rewinds cartridge memory; normal autosave or clean exit subsequently persists it to the currently selected .sav.

Documentation: [save_states.md](docs/save_states.md), [PROJECT_STATE.md](docs/development/PROJECT_STATE.md).

[Change record](docs/changes/2026-10-20-save-states.json)

## 2026-10-20 — Undo the last successful save-state save or load from the controller menu

Type: added. Platforms: Windows (Wine), Desktop.

**Changes**

- Add Undo last operation to the slot picker, identifying the action and its slot independently of the currently selected slot.
- Keep one RAM-only undo entry per play session: restore exact previous slot bytes/preview or an empty slot after saving, and restore the complete pre-load machine after loading.
- Replace undo history only on successful saves/loads, consume it only on successful undo, preserve it across failures/menu reopen, and refuse detected external slot changes during undo save.
- Share audio/input/display restore handling between load and undo load; block undo load during input recording. Slot and machine formats and native cache ABI remain unchanged.

**Validation**

- Windows GNU release under Wine: 30 menu and 36 runtime tests passed, covering undo action routing, five-row UI geometry, empty/occupied/legacy slot restoration, machine restoration after further play, failed-operation retention, external changes, retry and no redo.
- Optional native replay passed: undoing a slot load restored the exact pre-load machine, and the next 120 framebuffer hashes and complete final state matched a reference, with enhanced audio taps enabled.
- Controller-only simulated-XInput smoke undid new-slot creation, replacement and loading; verified slot existence/hash changes, resumed and exited cleanly. Restart preserved the slot and cleared undo history.
- Five memory-tool tests, development skill validation and whitespace checks passed. Windows archive CRC, executable equality, TinyCC/notices presence and private-data/test-double exclusion checks passed.

**Limitations**

- Undo has one level in RAM for the current session, with no redo or recovery history after restart/game switch. A new successful save or load replaces it.
- Undo load also discards gameplay since that load and restores cartridge memory; ordinary .sav autosave/exit behavior still applies.
- Native Windows and physical gamepad validation remain outstanding; testing used Wine and simulated XInput. AndroidPort is unchanged.

Documentation: [save_states.md](docs/save_states.md), [PROJECT_STATE.md](docs/development/PROJECT_STATE.md).

[Change record](docs/changes/2026-10-20-save-state-undo.json)

## 2026-10-20 — Nine automatic save slots with previews and controller-only menu flow

Type: changed. Platforms: Windows (Wine), Desktop.

**Changes**

- Replace save/load file dialogs with nine numbered slots, wraparound selection, screenshot previews and in-menu success/error feedback.
- Store slot-1.ffstate through slot-9.ffstate under SaveStates/<ROM SHA-256>/ beside the selected cartridge save, isolating different ROMs and preserving slots across restarts.
- Commit the 240x160 game-view/native-UI preview and machine snapshot as one atomic file; require explicit in-menu replacement confirmation and protect newly occupied slots from unconfirmed writes.
- Keep loading compatibility for original raw .ffstate checkpoints copied into a numbered slot; retain the existing core state version and native cache ABI.

**Validation**

- Windows GNU release under Wine: 29 menu tests and 31 runtime tests passed, covering all nine paths, ROM isolation, fresh-machine loading, no-clobber/confirmed replacement, matching previews, damaged files and legacy states.
- Preview drawing and mouse hit geometry passed at 240x160, 640x480, 1280x720 and 3840x2160. Controller navigation tests passed for slot wraparound, empty loads and safe confirmation defaults.
- Interactive simulated-XInput smoke saved slot 9, displayed its preview, cancelled and confirmed replacement, loaded it, exercised empty-slot feedback and loaded slot 1 after restart; both sessions resumed and exited cleanly.
- Five memory-tool tests and development skill validation passed. Release archive CRC, executable equality, TinyCC/notices presence and private-data/test-double exclusion checks passed.

**Limitations**

- Validation used Wine and simulated XInput, not physical controllers or native Windows. AndroidPort remains unchanged.
- Original files outside the numbered slot folder are preserved; importing one requires copying it into an unused slot. A legacy preview appears after resaving.
- Slot location follows the selected cartridge save directory. Loading also restores cartridge memory, subsequently persisted by normal autosave or exit.

Documentation: [save_states.md](docs/save_states.md), [PROJECT_STATE.md](docs/development/PROJECT_STATE.md).

[Change record](docs/changes/2026-10-20-save-state-slots.json)

## 2026-10-20 — Add project skills and checked development memory

Type: docs. Platforms: Project-wide.

**Changes**

- Add project instructions and local skills for implementation, Windows validation and evidence-based change recording.
- Backfill performance, FPS and menu shortcut history; keep durable project knowledge separate from chronological records.
- Generate CHANGELOG.md from structured JSON records and check that changed implementation files have a new or updated record.
- Add a read-only CI workflow for change coverage and memory tooling tests; preserve existing uncommitted feature work.

**Validation**

- Five Python unittest cases passed in temporary Git repositories, including stale output, duplicate records, unsafe paths, invalid dates, missing references, commit ranges, deleted files and staged changes.
- All three local skills passed the skill-creator validator.

**Limitations**

- Agents must read AGENTS.md or discover the local skills; no global client settings or background services are installed.
- The checker verifies file coverage and metadata, not the truth of descriptions; change summaries and validation evidence require review.
- The CI workflow has been added locally but has not run on GitHub; remote branch protection settings are unchanged.

Documentation: [MEMORY_WORKFLOW.md](docs/development/MEMORY_WORKFLOW.md), [PROJECT_STATE.md](docs/development/PROJECT_STATE.md), [AGENTS.md](AGENTS.md).

[Change record](docs/changes/2026-10-20-project-memory.json)

## 2026-10-20 — Add configurable keyboard and gamepad menu combinations

Type: added. Platforms: Windows (Wine), Desktop.

**Changes**

- Add Open menu binding capture under keyboard, mouse/keyboard and gamepad input pages; persist key.menu and pad.menu.
- Default to Escape and Start+Back/Select; preserve Escape recovery and Guide/Mode where available.
- Capture the largest simultaneous combination and consume a completed shortcut through partial release, across desktop controller backends.

**Validation**

- 78 Windows tests passed through Wine: 25 configuration, 26 menu and 27 runtime tests.
- Interactive Wine smoke with an isolated XInput test double verified default/custom gamepad shortcuts, partial release, keyboard capture and persistence.

**Limitations**

- Physical controllers and native Windows manual behavior remain unverified.
- An individual button retains its game action before the full combination is recognized.

Documentation: [menu_shortcuts.md](docs/menu_shortcuts.md).

[Change record](docs/changes/2026-10-20-menu-shortcuts.json)

## 2026-10-20 — Package a native Linux portable edition with executable archive permissions

Type: added. Platforms: Ubuntu 26.04 x86_64, Windows packaging compatibility.

**Changes**

- Build and package native recomp/ffta-play plus TinyCC as dist/FFTARecomp-linux-portable.tar.gz, with executable launchers, local portable marker and empty Data directories; replace the native Linux installer packaging entry point.
- Share explicit package input selection and staged atomic publication between Linux tar.gz and Windows ZIP, retaining the existing Windows script interface and TinyCC source/licenses while excluding game and personal data.
- Document Ubuntu startup, native dependencies, glibc 2.43 requirement for this host-built runtime, portable migration and observed validation; extend project guidance for Linux packaging.

**Validation**

- Native locked/offline release build and TinyCC build passed on Ubuntu 26.04 x86_64 using locally extracted ALSA development metadata. All runtime shared libraries resolved with ldd; ELF inspection established glibc 2.43 runtime and 2.39 wrapper requirements.
- 26 input/config and 41 runtime tests passed natively; two optional runtime tests were ignored.
- Observed native X11 gameplay/FPS on the local desktop. An isolated Xvfb/Mesa llvmpipe keyboard smoke saved a preview slot, closed normally, moved the folder, loaded the same slot and closed normally again; translation cache bytes/mtime were reused, startup backups counted one then two and external per-user files remained unchanged.
- The refactored Windows compatibility packaging entry point produced a ZIP with archive CRC and input-equality verification.
- The native Linux packaging shell script completed end to end. Final gzip/tar integrity, executable mode 0755, runtime/TinyCC input equality, source/notices presence and private-data exclusion checks passed. Project memory coverage, skill validation and whitespace checks passed.

**Limitations**

- This native archive was built for Ubuntu 26.04 x86_64 and requires glibc 2.43; older Ubuntu versions and Steam Deck compatibility were not established.
- Physical controllers, native Wayland interaction and first-launch ROM selection were not tested on Linux; the Linux smoke used a preplaced user ROM and Xvfb software rendering for menu automation.
- Application data is portable; Linux graphical, ALSA, udev and libc runtime dependencies remain supplied by the host. Windows-hosted Linux cross-build and Android outputs remain separate.

Documentation: [portable.md](docs/portable.md), [PROJECT_STATE.md](docs/development/PROJECT_STATE.md).

[Change record](docs/changes/2026-10-20-linux-native-portable.json)

## 2026-10-20 — Add configurable internal and external FPS overlay

Type: added. Platforms: Windows (Wine), Desktop.

**Changes**

- Add Video > FPS counter with Off, Internal, External and Both modes, persisted as video.fps in av.cfg (commit 401922d).
- Show a compact high-contrast HUD with guest frame and host presentation rates, independent of game zoom; reset sampling after pause or mode changes.

**Validation**

- 75 Windows tests passed through Wine across runtime, menu, configuration and GPU layer equivalence.
- Interactive Wine smoke verified menu selection, persistence, normal INT/EXT rates and different rates under fast forward.

**Limitations**

- External FPS counts successful host presentations, not physical display scanouts.
- Native Windows interactive behavior remains unverified.

Documentation: [fps_overlay.md](docs/fps_overlay.md).

[Change record](docs/changes/2026-10-20-fps-overlay.json)

## 2026-10-20 — Build Linux portable releases with container-managed dependencies

Type: added. Platforms: Linux amd64 (Docker).

**Changes**

- Add a Rust 1.94.0/Debian Bookworm build image pinned by digest with GCC, Python, pkg-config and ALSA/udev development packages installed inside the container.
- Export only the portable tar.gz and compiler/libc/library/hash information through a scratch stage; keep BuildKit registry/git/target caches separate from the checkout and run runtime/input tests before export.
- Add a one-command build/export wrapper and an allowlisted Docker context excluding ROMs, personal saves, state slots, agent settings and host build outputs; document the distinction between contained build dependencies and host GUI/runtime dependencies.
- Include Docker workflow documentation in portable packages and retain the separate direct Ubuntu-built archive.

**Validation**

- Shell syntax and Python packaging syntax checks passed. Excess arguments, comma-containing and newline-containing output paths were rejected from an unrelated working directory before invoking Docker.
- Actual Docker build/export completed: Rust 1.94.0 and Debian Bookworm glibc 2.36; 26 input/config and 41 runtime tests passed inside the container, with two optional runtime cases ignored.
- All exported runtime dependencies resolved on Ubuntu; ELF inspection found maximum required GLIBC versions 2.35 (runtime), 2.34 (wrapper) and 2.34 (TinyCC), compared with 2.43 for the earlier direct host runtime.
- The exported package passed an Ubuntu-host Xvfb/Mesa llvmpipe native keyboard smoke: preview slot save, normal close, full folder relocation, slot load, unchanged cache bytes/mtime, one then two startup copies and unchanged external per-user files.
- Final digest-pinned build/export passed. A second invocation from /tmp with a space-containing output path reused cached build layers and exported identical archive/build-info bytes. Final archive checks passed for executable modes, source/notices/docs, checksum, host ownership, equality with the native-tested binaries and private-data exclusion.

**Limitations**

- The image digest and Cargo.lock are pinned; Debian package updates mean bit-for-bit reproducible builds are not guaranteed.
- The build is linux/amd64. Containerized compilation does not bundle the host desktop graphics stack, libc, ALSA or udev runtime libraries into the portable archive.
- GUI validation used Ubuntu 26.04 Xvfb and software rendering with a preplaced user ROM; older-distro/Steam Deck, physical controllers and native Wayland interaction remain unverified.

Documentation: [docker.md](docs/docker.md), [portable.md](docs/portable.md).

[Change record](docs/changes/2026-10-20-docker-linux-build.json)

## 2026-10-20 — Keep two automatic cartridge-save backups and restore them from the controller menu

Type: added. Platforms: Windows (Wine), Desktop.

**Changes**

- Capture the selected raw .sav before gameplay/autosave on desktop launch, loading exactly those captured bytes and retaining the two most recent complete startup copies.
- Store raw copies under SaveBackups/<save-filename>/ with sequence and SHA-256 filenames; publish a complete copy before pruning managed older files, preserving unrelated files and aborting startup on backup failure.
- Add controller-operated Save backups with latest/previous choices, safe Cancel-default confirmation and pinned-generation restoration. Validate content/size and prepare a clean boot before atomically replacing the current file and machine.
- Preserve both copies and all nine state slots during restore, reset the frontend and state undo, and prevent stale gameplay RAM from overwriting the recovered cartridge data. Missing remembered/default primary saves retain their recovery location when copies exist.
- Fit ten main-menu rows and distinguish main/video layouts with equal row counts; retain ordinary menu navigation and input capture behavior.

**Validation**

- Windows GNU release through Wine: 33 menu and 42 runtime tests passed, covering retention, exact raw data, safe confirmation, generation pinning, clean-boot restore, subsequent save, damaged/missing copies, failed publication, missing primary recovery and failure isolation.
- Controller-only simulated-XInput smoke exercised three starts, two-copy rotation, default cancellation, older-copy restore and clean exit without stale-RAM overwrite. Both remaining raw-copy hashes matched their encoded checksums.
- A separate final-build simulated-XInput smoke restored a missing primary from existing backups, retained the recovered file on exit and left both copies unchanged.
- Five memory-tool tests, development skill validation and whitespace checks passed. Archive CRC, executable equality, TinyCC/notices presence and private-data/test-double exclusion checks passed.

**Limitations**

- Copies are the two most recent startup versions, so unchanged launches can yield identical copies; an absent primary does not create a fake backup.
- Restore replaces session progress and clears save-state undo. It is blocked during input recording. Hash checks validate backup storage integrity, not game-specific save checksums.
- Validation used Wine and simulated XInput, not native Windows or physical controllers. AndroidPort remains unchanged.

Documentation: [save_backups.md](docs/save_backups.md), [save_states.md](docs/save_states.md), [PROJECT_STATE.md](docs/development/PROJECT_STATE.md).

[Change record](docs/changes/2026-10-20-cartridge-save-backups.json)
