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

## 2026-10-20 — Validate supplied local saves and identify preliminary battle PPU cost

Type: docs. Platforms: Linux X11/Xvfb, Vulkan llvmpipe.

**Changes**

- Document isolated loading of the supplied cartridge save and snapshot, verified Git exclusions, and the distinction between valid resumed gameplay and paused automation attempts.
- Prioritize detailed CPU PPU and UI-extraction profiling in the performance TODO based on the exploratory battle capture; retain all acceptance criteria for controlled AI replay and optimization.

**Validation**

- git check-ignore confirms the root saves.zip and saves/ paths are excluded, alongside existing global .sav/.ffstate patterns; git ls-files contains none of these fixture files. ZIP CRC passes and the selected source copies still equal the original archive entries.
- The exported Docker Linux runtime successfully loaded the supplied slot, resumed the battle for 20 seconds and exited normally. The session includes startup: 1074 captured core frames, 171840 scanlines, no zoom calls or invalid PPU intervals; core/scanline p50 23.083840/17.545029 ms and p95 25.520037/19.434143 ms.
- A separate portable copy loaded the cartridge save through Saved Game > Load and visually reached the world map; observed overlay samples were about 59-60 internal and external FPS. The session closed with exit status 0; all 11659 captured frames had 160 scanlines and no invalid intervals. Whole-session statistics include boot/navigation and are not a world-map-only baseline.
- Project-memory validation passed for the working tree and --base HEAD; the diff whitespace check passed. No runtime code changed in this tranche.

**Limitations**

- Exploratory captures use Xvfb and CPU llvmpipe, include startup/navigation and have no paired baseline; they do not establish Radeon/Wayland performance, an AI-turn bottleneck or an optimization gain. Headless snapshot replay and clean post-load timing remain to be implemented.

Documentation: [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md), [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md).

[Change record](docs/changes/2026-10-20-validate-local-save-workloads.json)

## 2026-10-20 — Separate CPU PPU work from core frame time with scoped diagnostics

Type: added. Platforms: Root workspace core, desktop and headless native runners.

**Changes**

- Add scoped thread-local host timing for hardware scanlines and extra zoom rows outside machine/save state. Disabled entry points check capture state without clock reads; scopes restore prior capture on exit/unwind and stay on the emulation thread. Parallel zoom measures caller wall time including worker waits.
- Report bounded per-frame PPU and core-remainder distributions and call/invalid-interval counts on desktop profiling and opt-in headless capture. Discard incomplete guarded frames and exclude headless warm-up.
- Add --ppu-profile benchmark runs separated from ordinary timing/verification, reject malformed/truncated/inconsistent interval totals and timing contamination, and validate timing totals while retaining older frame-profile rows without totals.
- Exclude the incoming local save fixture archive from version control; its original bytes remain read-only and no save data enters the portable packages.

**Validation**

- 167 core and 46 runtime release tests passed serially. New checks cover disabled/thread-local capture, nested/unwinding cleanup, actual forced-blank and bitmap frame state equivalence, bounded retained samples and invalid-interval accounting. Nine Python benchmark tests passed, including malformed PPU metrics and contamination rejection.
- The explicit native 3000-frame replay with dispatch and PPU observation enabled passed exact frame/OAM/audio transcript and full serialized-state equality. Each captured frame contained 160 hardware scanlines and nonoverlapping PPU intervals.
- With PPU timing disabled, the same-host 3000-frame replay passed framebuffer/OAM, full final-state and execution-count equality over five alternating pairs pinned to CPU 15. Baseline/candidate medians: 2.430751/2.445480 ms/frame; the roughly 0.6 percent difference is within observed variability, not evidence of an AI gain.
- The actual --ppu-profile pipeline passed five alternating same-binary pairs at zoom level 2 on a 3000-frame intro. Both instrumented runs captured 2880 post-warmup frames, 460800 scanlines and 1837 zoom calls with valid disjoint intervals; full final state and dispatch counts matched ordinary runs. Estimated instrumentation overhead was 1.162/1.370 percent versus each uninstrumented median, a single-run estimate per side.
- In that zoomed introductory sample only, mean hardware-scanline time was 1.059022/1.071116 ms/frame, zoom time 0.072856/0.073398 and core remainder 1.493626/1.502000. Final stricter parser also accepted both saved captures; these are workload-specific CPU wall times, not AI or GPU measurements.
- Final expanded Docker suites passed 312 Linux tests (167 core, 26 input/config, 33 menu, 46 runtime, 40 screen) and 311 Windows/Wine tests (167 core, 26 input/config, 33 menu, 47 runtime, 38 screen); nine optional cases remained ignored per target. Both portable archives passed integrity, private-data/cache exclusion, library whitelist, production-marker and BUILD-INFO SHA-256 checks; Linux executable modes were preserved.
- The final exported Linux launcher passed isolated Xvfb slot save/load after folder relocation, cache reuse, two startup backups and unchanged per-user data. Both sessions emitted valid desktop PPU counts (160 hardware calls per captured frame) and complete bounded stage reports, including after state load; both closed normally.
- An incoming local save archive was inspected read-only. ZIP CRC, slot-preview and machine-payload checksums, snapshot version and ROM identity matched the current local EUR fixture. Deserialization and slow-scene classification remain pending; no game-progress contents were recorded in project memory.

**Limitations**

- CPU wall times are not GPU execution or summed worker CPU times. Detailed tile/sprite/composition timing, classification/replay of the newly available local snapshot, native Windows and physical-controller checks remain separate. AndroidPort is unchanged.
- The zoomed profiling run exercised the serial zoom path. Parallel zoom profiling and detailed GPU/PPU bottleneck analysis still require their own controlled validation.

Documentation: [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md), [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md).

[Change record](docs/changes/2026-10-20-separate-cpu-ppu-frame-timing.json)

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

## 2026-10-20 — Reuse unchanged color/FPS uploads and preserve audio producer buffers

Type: performance. Platforms: Desktop Linux and Windows.

**Changes**

- Retain a bounded per-layer LUT copy and upload colors only when actual contents differ, including in-place mutations. Give FPS textures process-unique render revisions so unchanged text avoids repeated uploads while layout uniforms and dynamic worlds remain current.
- Count submitted UI payload bytes in opt-in CPU profiling. Keep unversioned game UI updates unconditional, recreate empty caches with GPU resources, and preserve the CPU sampler and guest/save-state contracts.
- Drain FIFO audio events while retaining producer Vec capacity, preserving ordering and overflow/drop behavior; resolve FFTA_FRAME_PACING once at startup instead of querying the environment every gameplay tick.

**Validation**

- 44 native runtime tests and 36 screen tests passed; the runtime examples compile. The extended ignored GPU equivalence test passed explicitly on AMD Radeon 780M / RADV across nine update/cache/mutation/recreation cases at three world sizes, with pixel-exact CPU/GPU comparison and exact upload-byte assertions.
- For unchanged 480x160 UI with a 1x1 world, the GPU test verifies 438338 initial scheduled bytes versus 66 repeated bytes; changed UI/LUT inputs and recreated resources still upload their required bytes. This is payload reduction, not a frame-rate or AI-turn speedup.
- With the final buffer/pacing changes, 44 runtime and 36 screen tests passed. Profiling-enabled actual-launcher save/load/move smoke passed on Xvfb; native Xwayland/RADV window close also exited normally and emitted reduced UI payload counters.
- Same-host pre-upload baseline/candidate replay passed framebuffer/OAM and full final serialized-state equality on 3000 frames, plus five alternating timing pairs. Medians were 3.436691/3.470794 ms per frame, within the observed run variability; no headless speedup is claimed.
- Final GPU parity also covers changed layout with unchanged pixel revision. Native runtime/screen suites passed after test cleanup, and actual RADV launcher profiling reported 1063034 payload bytes across 157 layer-update calls with clean window close.

**Limitations**

- The slow AI-turn fixture is unavailable. PERF-05/06 remain partially open for broader UI/world reuse, bounds/packing work and controlled scene-level performance checks; no AI or overall FPS gain is claimed.

Documentation: [fps_overlay.md](docs/fps_overlay.md), [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md), [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md).

[Change record](docs/changes/2026-10-20-reuse-desktop-uploads-and-audio-buffers.json)

## 2026-10-20 — Resume native headless benchmarks from validated desktop snapshots

Type: added. Platforms: Root desktop CLI and Python benchmark runner; native Linux and Windows via Wine validation.

**Changes**

- Add runc --state for read-only desktop slots with previews and legacy raw snapshots through the existing transactional menu validator; preserve state version, guest clocks and translation ABI.
- Replay N additional frames after loading, with input, zoom and benchmark warm-up relative to the loaded point. Preserve absolute guest counters and legacy guest-cycle/character diagnostics; no-snapshot runs retain boot frame zero.
- Reject conflicting cartridge/snapshot starts and diagnostic output over the snapshot path or its symlink. Extend benchmark input protection, checksums, initial-machine identity, exact frame-range validation and summary origin metadata.
- Record a reproducible idle battle baseline from the supplied local snapshot and prioritize detailed CPU PPU/UI extraction profiling; this change does not alter the renderer or claim an AI speedup.

**Validation**

- 47 native Linux runtime release tests passed serially, with three optional fixture cases ignored. The new command test rejects conflicting --sav/--state and snapshot output collisions, including symlinks on Unix, without changing the source file. Eleven Python behavioral tests passed, including nonzero origins, incomplete/duplicate captures, differing initial identities, output collisions and invalid start options.
- The supplied desktop slot passed the actual native headless pipeline over five alternating same-binary pairs, 600 additional frames and 120 warm-up frames, pinned to CPU 15 with RECOMP_MP2K=1. Per-frame framebuffer/OAM, full final machine state and execution counts matched; input hashes were unchanged. Median work times were 20.950177/20.856358 ms/frame, measuring variability rather than an optimization.
- Both separate instrumented battle runs captured 480 post-warmup frames, 76800 hardware scanlines, zero zoom calls and no invalid intervals. Mean scanline cost was 16.238810/15.286284 ms and core remainder 5.017627/4.764722 ms, about 76 percent scanlines. Instrumentation overhead estimates of +1.487/-3.837 percent show single-run variation and are not a speedup.
- A second snapshot pipeline applied zoom at relative frames 0 and 30 despite a nonzero guest origin. Both instrumented runs recorded 50 measured frames, 8000 hardware scanlines and 30 zoom calls with matching final state and execution counts.
- A legacy raw snapshot derived locally from the same slot loaded with the same machine identity; zero additional frames preserved the complete serialized state and original frame counter. An existing native binary and the new runtime also matched all 3000 introductory framebuffer/OAM captures, final machine state and execution counts without --state.
- Final Docker builds passed 313 Linux tests (167 core, 26 input/config, 33 menu, 47 runtime, 40 screen) and 312 Windows/Wine tests (167 core, 26 input/config, 33 menu, 48 runtime, 38 screen), with nine optional tests ignored on each target. Both portable archives passed integrity, private-data/library exclusion and BUILD-INFO SHA-256 checks; Linux executable modes were preserved.
- The actual exported Linux and Windows/Wine CLIs loaded the supplied desktop slot with zero additional frames. Both matched the native legacy-loader run on original frame counter, initial/final full machine identity and framebuffer result. This tests cross-platform loading without claiming cross-platform gameplay/audio equivalence.
- Original ZIP hash and selected source save/snapshot bytes remained unchanged after the tests. Project-memory checks passed for the working tree and --base HEAD, and the diff whitespace check passed.

**Limitations**

- The baseline is an idle battle snapshot, not an identified slow enemy-turn replay. Headless work excludes presentation, UI composition in the window and the audio-device callback; detailed PPU substage and GPU timings remain pending.
- Native Windows, physical controllers and AndroidPort are not validated by the Linux measurements. No native/generated guest semantics or snapshot format changed.

Documentation: [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md), [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md), [save_states.md](docs/save_states.md).

[Change record](docs/changes/2026-10-20-resume-headless-benchmarks-from-snapshots.json)

## 2026-10-20 — Retain GPU presentation across transient surface failures

Type: fixed. Platforms: Desktop Linux and Windows.

**Changes**

- Return an explicit presented/skipped outcome. Preserve GPU rendering on acquisition timeout, skip zero-sized surfaces, reconfigure Lost/Outdated once per frame and fall back after three failed recovery calls. Successful acquisition resets the budget; out-of-memory and unknown errors request immediate fallback.
- Count FPS EXT only on successful presentation and preserve the existing event-loop pacing and Wayland teardown order.

**Validation**

- 44 native runtime tests and 40 screen tests passed, including simulated prolonged timeout, lost/outdated recovery, success resetting the failure budget, bounded retries and fatal errors without reconfiguration.
- Native Ubuntu Xwayland/Vulkan on AMD Radeon 780M / RADV passed repeated resize, minimize/restore and WM_DELETE_WINDOW close using the isolated portable launcher and test save. Exit status was zero and the GPU presenter remained active.
- Windows Docker cross-build and Wine suites passed: 26 input/config, 33 menu and 45 runtime tests. Exported ZIP passed CRC, private-data exclusions, production-marker and BUILD-INFO SHA-256 checks; only empty portable Data directories are included.
- Linux Docker build and tests passed: 26 input/config and 44 runtime tests. Exported tar.gz passed integrity, executable-mode, private-data exclusion, production-marker and BUILD-INFO SHA-256 checks.

**Limitations**

- Physical monitor changes, native Windows and Wayland recovery/fallback paths remain unverified. PERF-04 acceptance remains open pending these interactive checks.

Documentation: [fps_overlay.md](docs/fps_overlay.md), [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md).

[Change record](docs/changes/2026-10-20-recover-transient-desktop-surfaces.json)

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

## 2026-10-20 — Preserve native runtime behavior during guest-cycle profiling

Type: fixed. Platforms: Desktop runtime and headless native runners.

**Changes**

- Replace the separate runc cost loop with an observer of the normal native dispatch loop, preserving sprite/mouse interpreter gates, audio HLE, BIOS/IRQ behavior and fallback tracing. Normal execution specializes the same loop with a no-op observer.
- Report native, missing-translation, system and intentional-hook dispatch counts separately while retaining legacy per-PC guest-cycle attribution. Add fixture-free guard/state/cycle equivalence and optional 3000-frame full-state/frame/OAM/audio transcript checks.

**Validation**

- 45 native runtime tests passed serially, with three optional fixture tests ignored. The synthetic test covers actual translated dispatch, missing translation, halted system steps, complete frames and exhausted zero/one-step guards.
- The initial 3000-frame fixture comparison passed equal frames/OAM/audio transcript, full serialized machine state and native/fallback counts. Observed 15470867 native blocks, 43198679 missing steps, 5984 system steps and 19823865 intentional-hook steps; attributed cycles summed to the actual 842604235 guest-clock advance.
- The strengthened fixture transcript includes per-frame guest clock and channel lengths. Final replay passed with 6582846 mixed samples, 6582846 PSG samples, 791212 events on each FIFO and 6561508 HLE samples, equal across observation modes.
- Actual runc CLI comparison with RECOMP_COST_FROM=240, RECOMP_MP2K=1 and fallback tracing passed all 3000 framebuffer/OAM hashes, final serialized-state SHA-256, execution totals and MP2K diagnostics. Both modes recorded 2990 live MP2K hooks and matching fallback census.
- Profiling-disabled same-host release A/B passed 3000-frame framebuffer/OAM, final-state and execution-count equality with five alternating timing pairs pinned to CPU 15. Baseline/candidate medians were 2.419853/2.411346 ms/frame (0.352 percent difference, within observed variability); no AI or whole-game speedup is claimed.
- Expanded final container suites passed 308 Linux tests (164 core, 26 input/config, 33 menu, 45 runtime, 40 screen) and 307 Windows/Wine tests (164 core, 26 input/config, 33 menu, 46 runtime, 38 screen); nine optional cases remained ignored per target. Both portable archives passed integrity, private-data and translation-cache exclusions, library whitelist and BUILD-INFO SHA-256 checks; Linux executable modes were preserved.
- The explicit native save-state replay/load/Undo fixture regression passed with the shared dispatcher.
- The final exported Linux portable launcher passed isolated Xvfb slot save/load after folder relocation, unchanged translation-cache bytes/mtime, two startup backups, unchanged per-user data and normal close. The loaded-state screenshot showed Slot 1 loaded and the undo-load entry.

**Limitations**

- Guest-cycle attribution is not host CPU timing. The introductory fixture does not establish AI-turn performance or coverage of every BIOS, audio-engine and input-hook path. Native Windows and physical-controller validation remain separate.

Documentation: [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md), [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md).

[Change record](docs/changes/2026-10-20-preserve-native-dispatch-in-cost-profiling.json)

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

## 2026-10-20 — Gate portable container exports on core and renderer regression suites

Type: changed. Platforms: Linux and Windows Docker builds.

**Changes**

- Run input/config, menu, core, screen and runtime tests serially in both container recipes before exporting portable artifacts. This includes panel-bound equivalence and surface-recovery tests and respects shared core display globals.

**Validation**

- Windows Docker cross-build/Wine passed 306 tests: 164 core, 26 input/config, 33 menu, 45 runtime and 38 screen. Eight optional cases stayed ignored. Exported ZIP passed CRC, private-data exclusions, production-marker and BUILD-INFO SHA-256 checks.
- Linux Docker passed 307 tests: 164 core, 26 input/config, 33 menu, 44 runtime and 40 screen. Eight optional cases stayed ignored. Exported tar.gz passed integrity, executable-mode, private-data exclusion, production-marker and BUILD-INFO SHA-256 checks.

**Limitations**

- Optional local-ROM, GPU readback and microbenchmark cases remain ignored in the default recipes; native GPU, physical controllers and native Windows require separate validation.

Documentation: [docker.md](docs/docker.md).

[Change record](docs/changes/2026-10-20-gate-docker-on-core-and-screen-tests.json)

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

## 2026-10-20 — Evaluate and withhold panel-bound optimization after end-to-end regression

Type: performance. Platforms: Root workspace core and desktop.

**Changes**

- Preserve a test-only row-endpoint scan candidate with full-scan equivalence coverage and an opt-in release microbenchmark using five alternating pairs. Inputs cover empty, dense, sparse and rectangular-panel layers.
- Keep production pixel_bounds unchanged. Although the candidate reduced synthetic panel-bound cost, integrated 3000-frame replays were slower, so the runtime optimization was withheld pending representative end-to-end benefit. No guest state, serialization or native translation ABI changes.

**Validation**

- All 164 core tests passed serially. Equivalence coverage includes empty layers, partially filled final rows, dense data, varying blend weights/colors, individual endpoints and inputs beyond the standard canvas.
- Release diagnostic medians in microseconds per call (reference/candidate): empty 31.922/32.422, dense 118.480/0.240, sparse 33.309/9.211, rectangular panel 42.388/28.798. Empty input was about 1.6 percent slower; gains are function-level synthetic measurements.
- Integrated candidate 3000-frame comparison (bounds-headless) passed framebuffer/OAM, full serialized final-state and execution-count equality with five alternating timing pairs. Baseline/candidate medians: 2.370355/2.422693 ms/frame; emulation-time reduction -2.208 percent. Pinned comparisons used CPU 15; the negative 3000-frame results prevented production integration.
- Integrated candidate 1200-frame comparison (bounds-headless-pinned) passed framebuffer/OAM, full serialized final-state and execution-count equality with five alternating timing pairs. Baseline/candidate medians: 1.197299/1.195935 ms/frame; emulation-time reduction 0.114 percent. Pinned comparisons used CPU 15; the negative 3000-frame results prevented production integration.
- Integrated candidate 3000-frame comparison (bounds-headless-pinned-3000) passed framebuffer/OAM, full serialized final-state and execution-count equality with five alternating timing pairs. Baseline/candidate medians: 2.424893/2.464699 ms/frame; emulation-time reduction -1.642 percent. Pinned comparisons used CPU 15; the negative 3000-frame results prevented production integration.
- After withholding the production change, the rebuilt native release runtime is byte-identical by SHA-256 to the saved pre-experiment binary. Only test/diagnostic code remains in gba-core.
- Final test-only candidate and reference equivalence coverage passed in the expanded Windows/Wine core suite: 164 passed, five optional cases ignored.
- Final test-only candidate also passed the expanded Linux container core suite: 164 passed, five optional cases ignored.

**Limitations**

- Enemy-turn fixtures remain unavailable. Microbenchmark results do not establish whole-game FPS or AI-turn gains; broader panel packing and revision caching remain pending.
- The integrated candidate was tested in an introductory headless replay, which does not measure desktop panel packing. The experiment remains disabled; synthetic function-level savings are not sufficient to ship it.

Documentation: [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md), [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md).

[Change record](docs/changes/2026-10-20-evaluate-panel-row-endpoint-scans.json)

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

## 2026-10-20 — Plan measured desktop optimizations with correctness and compatibility gates

Type: docs. Platforms: Desktop Linux and Windows.

**Changes**

- Add a repository TODO with 13 prioritized tasks: reproducible AI-turn workloads, profiling and equivalence gates, transient GPU recovery, upload/buffer reuse, CPU/native/PPU/audio/pacing improvements, and conditional GPU temporal processing and zoom parallelism.
- Document code evidence, risk, dependencies and completion criteria; defer guest-AI and full-PPU GPU rewrites until measurements and equivalence justify their complexity. Link the plan from README and project state.

**Validation**

- Reviewed the current runtime, native dispatch, CPU PPU, temporal filters, GPU presenter/UI uploads, FPS overlay, audio handoff and existing benchmark/equivalence tests; checked wgpu 26 upload/timestamp documentation.
- All 13 task IDs are unique and pending, local source links resolve, project-memory checks including --base HEAD pass, and the diff passes whitespace checks. No runtime tests were needed for this documentation-only change.

**Limitations**

- Planning/documentation only: no runtime optimization, new benchmark measurement or native AI-turn speedup is claimed. Task priorities after baseline collection depend on the measured bottleneck.

Documentation: [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md).

[Change record](docs/changes/2026-10-20-desktop-performance-todo.json)

## 2026-10-20 — Begin performance TODO with native comparisons and bounded CPU profiling

Type: added. Platforms: Desktop Linux x86_64; Windows GNU compatibility.

**Changes**

- Extend the existing benchmark to detect native Linux/macOS and Windows/Wine binaries, alternate five timing pairs, validate complete unique frame captures and warm-up scenes, protect inputs, apply per-process timeouts and publish explicit running/failed/passed summaries.
- Add opt-in full final-machine-state hashing and bounded headless per-frame CPU times with separate diagnostic runs; add opt-in interactive CPU phase summaries, leaving disabled collection allocation-free and avoiding per-frame logging.
- Replace mixed-unit native percentages with separate block/step counts and reject incomplete guarded frames before advancing frontend temporal history. Keep PERF-01/02/03 open for missing workload, PPU/GPU and complete validation requirements.
- Split opt-in CPU presenter times into setup/upload, acquire/retry, encoding, submission and present, with UI layout/uploads measured separately; preserve the existing presenter API and expose no mandatory GPU features or timestamp requirements.

**Validation**

- Eight Python rejection/input-protection tests and 43 native Linux runtime tests passed; two fixture-dependent runtime tests were initially ignored.
- The optional native save-state replay/load/Undo integration test passed with the local ROM and TinyCC translation, including exact machine-state replay.
- Preliminary packaged-baseline comparison on 3000 intro frames passed framebuffer/OAM verification and five alternating native Linux timing pairs. A final-runtime self-comparison on 1200 frames passed full serialized final-state equality, five timing pairs and separate bounded frame-time diagnostic runs.
- Actual launcher GUI save/load/move smoke passed with profiling enabled on Xvfb/llvmpipe; cache, two backups and per-user data remained correct. Xwayland/AMD RADV Vulkan profiling emitted expected CPU phases and closed with exit status 0.
- Windows Docker cross-build/export passed 26 input/config, 33 menu and 44 runtime tests under container Wine; two optional runtime tests remained ignored. Exported ZIP CRC, PE signature, BUILD-INFO checksum and private-data exclusion checks passed.
- Final native runtime also passed a fresh packaged-baseline comparison on 3000 intro frames with matching framebuffer/OAM and dispatch counts, plus five alternating timing pairs. Baseline/candidate medians were 3.238056/3.203595 ms per frame; this instrumentation change claims no speedup.

**Limitations**

- P0 remains in progress: no identified slow enemy-turn replay, separate PPU/GPU execution timing, full-stream audio equivalence or native Windows/macOS performance result is established.
- Intro timings and short GUI profiles are diagnostic validation, not an optimization speedup claim. The preliminary baseline used a Docker-built package while the candidate used the native host toolchain; single-run overhead estimates are noisy.

Documentation: [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md), [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md).

[Change record](docs/changes/2026-10-20-desktop-performance-baseline-tools.json)

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
