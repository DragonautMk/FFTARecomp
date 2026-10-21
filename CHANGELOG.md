# Changelog

Unreleased development changes. Generated from `docs/changes/*.json`.
Edit the structured records, then run `python3 scripts/project_memory.py render`.

## 2026-10-21 — Add observed window-manager minimize and restore regression coverage

Type: added. Platforms: Linux X11, Openbox and llvmpipe Vulkan, Windows GNU/MinGW13 under Wine10 with Vulkan.

**Changes**

- Add an optional no-ROM event-loop Presenter diagnostic that observes real window-manager minimize/restore transitions, requires rendering after restore, checks three sizes and VSync ON/OFF, and preserves surface-before-window cleanup.
- Require observable minimized state and bounded phase deadlines; unsupported state, premature close or incomplete teardown fails instead of claiming lifecycle success from explicit zero-size API calls.

**Validation**

- Native release diagnostic compiled and passed three actual minimize/restore cycles, resize to 640x360/960x540/852x480, VSync ON/OFF and clean teardown on Xvfb/Openbox/llvmpipe Vulkan: 27 Presented, zero Skipped. Independent xprop observed three Iconic and four Normal states.
- The unchanged canonical Linux portable passed three real WM minimize/restore cycles while running an isolated battle copy, followed by Undo/reload/resume/WM-close. Independent states and exit status checked; restored battle/HUD screenshot inspected. Runtime hash matches the current inspected Linux export.
- The new example cross-compiled for Windows GNU using existing local MinGW13. Wine first selected its synthetic D3D12 adapter and failed device creation; this is not passing coverage. A retry with D3D12 DLLs disabled only in the isolated test prefix passed on llvmpipe Vulkan: three observed minimize/restore cycles, resize/VSync and clean exit, 18 Presented/9 Skipped (minimized sizes were zero). Independent xprop observed three Iconic/four Normal states. No production backend choice was changed.
- The first Wine wrapper cleanup used an unavailable wineserver command name; corrected its path and cleaned up the owned Xvfb/prefix before retry. The final wrapper finished normally and left no test display/process running.
- project_memory.py render, check and check --base HEAD, rustfmt --check on the new example and Git whitespace validation passed before handoff.

**Limitations**

- No native Windows GPU, Wayland minimize/restore, physical monitor switch or forced runtime device-loss claim. Runtime and canonical portable archives are unchanged; no existing benchmark or fixture was recreated.

Documentation: [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md).

[Change record](docs/changes/2026-10-21-validate-window-minimize-restore.json)

## 2026-10-21 — Verify renderer transitions between distinct virtual monitor outputs

Type: added. Platforms: Native Linux X11 Xorg dummy/Openbox/Vulkan, Windows GNU under Wine10/Vulkan.

**Changes**

- Extend the no-ROM window diagnostic with a separate --monitors mode: require two separately positioned enumerated monitor handles, observe A/B/A/B/A changes, resize to bounded monitor-relative dimensions, verify VSync ON/OFF presentation and clean teardown.
- Keep the completed default minimize diagnostic and all runtime/shader/backend/guest behavior unchanged; no archive rebuild or benchmark calibration is required for this diagnostic-only addition.

**Validation**

- Native and Windows GNU release diagnostic builds passed. Native same-size outputs then unequal 1280x720/1280x960 CRTC outputs passed four monitor transitions, five observed placements and VSync checks: 20 Presented, zero Skipped. Independent xwininfo positions confirm A/B/A/B/A; unequal outputs cause actual 640x360/640x480 surface-size changes.
- Windows GNU/Wine10 with the previously established isolated Vulkan/D3D12-disabled prefix setup passed the unequal-output mode: two distinct DISPLAY handles, A/B/A/B/A, 20 Presented, zero Skipped, resize/VSync and clean exit. Independent X window positions also match.
- Capability inspection found Xvfb reset monitor areas between clients; -noreset preserves them, but winit enumerates CRTC outputs rather than those areas. Locally extracted Xorg/dummy provided distinct active outputs with automatic physical input/GPU addition disabled; no system packages or real display configuration changed.
- First portable move script reached the intro rather than battle; screenshots prevented a false load/Undo claim. Explicit evdev US synthetic-keyboard setup passed a focused menu/slot probe, then the corrected actual portable scenario passed battle load/zoom, A/B/A/B/A moves, Undo/reload/resume and WM-close. Battle-on-second-monitor, Undo and reloaded-battle captures inspected; positions/exit and current canonical runtime hash checked. Old input-unverified artifacts remain labeled as such.
- Verified all current shipping-source hashes and both canonical archive hashes unchanged. No runtime package rebuild, default minimize retest, scene recording or benchmark timing was repeated for this diagnostic mode.
- Native/Windows builds, rustfmt --check, project_memory.py render/check/check --base HEAD and Git whitespace validation passed before handoff.

**Limitations**

- No physical-monitor, GPU migration, mixed-DPI/color calibration, hotplug, Wayland or native Windows coverage. The virtual pair is same GPU and scale 1. Canonical runtime and portable archives remain unchanged; no performance speedup is claimed.

Documentation: [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md).

[Change record](docs/changes/2026-10-21-validate-virtual-monitor-transitions.json)

## 2026-10-21 — Require an explicit task selection checkpoint after context resumption

Type: changed. Platforms: Project agent workflow.

**Changes**

- Extend the existing resume rules in all three local skills and AGENTS with a task selection checkpoint before mutations or expensive commands: current request, stable task ID, previous outcome, missing deliverable and next action.
- Require a concrete reason to reopen completed or rejected work and validate already implemented requests without duplicating implementation or records.
- Mark the workflow request complete in the handoff and replace stale running-benchmark guidance with the observed completed timing artifact; preserve the unaccepted runtime candidate as pending.
- Require checking the selected ID against newer terminal records and immediately closing the handoff selection before moving on; remove the stale pre-wait-composition next action and preserve its recorded rejection without repeating runtime work.

**Validation**

- All three local skills passed skill-creator quick_validate.py.
- Read Git diff and existing memory records before editing; inspected the existing timing-summary status without restarting benchmarks or builds.
- project_memory.py render, check and check --base HEAD passed; git diff whitespace check passed with core.whitespace=cr-at-eol.
- On request reconciliation, verified the already implemented guidance without rewriting skills or repeating runtime validation; corrected the handoff latest-request checkpoint to keep pending performance work separate.
- Revalidated all three skills with quick_validate.py after adding immediate terminal closure; reconciled the stale selection against the existing rejected-composition record without running builds, replays or benchmarks.

**Limitations**

- These instructions require the agent to read and maintain the handoff; no compaction hook enforces task selection automatically. Runtime candidate acceptance is outside this workflow change.

Documentation: [MEMORY_WORKFLOW.md](docs/development/MEMORY_WORKFLOW.md), [TASK_HANDOFF.md](docs/development/TASK_HANDOFF.md).

[Change record](docs/changes/2026-10-21-require-resume-selection-checkpoint.json)

## 2026-10-21 — Reconcile performance TODO criteria with completed evidence

Type: docs. Platforms: Project development workflow.

**Changes**

- Close PERF-03 correctness/A-B controls and PERF-08 CPU PPU reductions against their stated acceptance criteria and existing implementation/evidence; preserve the remaining original-lag, hardware and native Windows requirements under their open items.
- Replace stale current-state claims that frame-window gating, enhanced-source counters and backend/cadence diagnostics are missing; remove the rejected pre-wait composition next action and already completed world-only UI next candidate.
- Summarize the status of all13 parent criteria without rerunning completed validation or claiming optional GPU/parallel experiments are implemented.

**Validation**

- Inspected current frontend guard ordering: incomplete frames return before completed-frame FPS, temporal history and frontend frame count updates. Read existing dispatch guard/full-state/audio equivalence records and benchmark protection/rollback evidence.
- Read window-reference, per-scanline BG, empty-row, panel-row and world-only pixel records; inspected current local world-only headless, GUI and zoom6 result JSONs with passed status. Existing native/Wine174 core checks,3530-frame full-state/count parity and five headless/GUI timing pairs support PERF-08 closure.
- Inspected the13 TODO criteria and current handoff; no tests, builds, replay or profiling runs repeated for this audit.

**Limitations**

- The original rare slow episode remains unisolated. Closing controls and accepted PPU reductions does not prove the overall performance objective complete or establish native Windows/physical hardware results.

Documentation: [2026-10-20-preserve-native-dispatch-in-cost-profiling.json](docs/changes/2026-10-20-preserve-native-dispatch-in-cost-profiling.json), [2026-10-21-evaluate-world-only-ui-pixels.json](docs/changes/2026-10-21-evaluate-world-only-ui-pixels.json), [2026-10-21-evaluate-composition-before-deadline.json](docs/changes/2026-10-21-evaluate-composition-before-deadline.json).

[Change record](docs/changes/2026-10-21-reconcile-performance-todo-criteria.json)

## 2026-10-21 — Observe realtime audio callback work and faithful source holds without changing DSP

Type: added. Platforms: Linux desktop, Windows GNU under Wine.

**Changes**

- Add an opt-in FFTA_PROFILE audio observer under the existing ring mutex: bounded lock/render/total/arrival timing and playing/priming/paused/fast-forward counters, independent of legacy alarm resets. Report outside the callback after stream teardown.
- Extract the unchanged DSP callback into a device-independent factory for parity and starvation tests; observer off adds no clock reads or allocation, and observer on adds no callback lock, logging or IO. Track producer dropped items and CPAL error notifications separately.
- Give each context its own fixed sample allowance, keep counts after caps, reset arrival history across contexts/device rebuilds, and label replacement-stream initial buffers as priming before watchdog updates. No core, save format, translation ABI or audio policy change.

**Validation**

- 58 native runtime tests passed (4 optional ignored), including new bounded-context/counter tests and real callback on/off parity in f32/i16/u16 at 44.1/48 kHz with mono/stereo/six-channel output, starvation, mute, HLE/enhanced and speed transitions.
- Normalized DSP statements match the preserved preimage exactly. Five alternating synthetic empty-ring observer pairs passed: median 38220.580 to 38349.723 ns/callback (+129.143 ns, +0.3379%); not hardware/game timing.
- Native CPAL stereo 44100 Hz session exercised all four contexts, load/zoom/fast-forward/Undo/reload/resume/close and inspected captures. Playing p99 total 0.500062 ms, mutex wait 0.000912 ms, 0 callbacks over own period, 0 producer drops; 1790 faithful holds in 6 callbacks. Private source fixtures unchanged. This GUI binary predates only the added attach-time priming-label safeguard covered by the final unit test.
- Final Docker exports passed 329 Linux and 328 Windows/Wine tests (12 optional ignored each); archive integrity/privacy, feature-doc identity, unchanged PE DLL imports/resources and GLIBC 2.35 checks passed.
- Actual final Linux portable passed cold-cache profile-on GUI all contexts/load/zoom/fast-forward/Undo/reload/resume/close with inspected captures. Playing 511 callbacks: p99 total 0.486576 ms, lock wait 0.000972 ms, no callback over its period or producer drops; 1868 faithful holds in 8 callbacks, including lifecycle transitions. All timing metrics finite/ordered, counts reconcile, source fixtures unchanged.
- Actual exported runtime passed default CPAL startup/close without diagnostic rows and 240 reference benchmark frames with no callback/audio profile or private writes. Updated canonical local portable archives after all checks and preserved previous exports; source/artifact hashes stored locally.
- project_memory.py render/check/check --base HEAD and git whitespace check passed.

**Limitations**

- Faithful source holds are not enhanced PSG/HLE/FIFO starvation or backend XRUN telemetry and can occur during refill/resume. Work times exclude profiler bookkeeping and callback unlock/return; arrival spacing is not CPU time.
- The capture is a whole interactive session with lifecycle labels, not a controlled AI-turn or scene/warm-up interval. Synthetic overhead is not native Windows/device latency. Existing callback backlog IO/error handling remains unchanged.
- Native Windows GUI/device timing, enhanced-path starvation/backend XRUNs and original slow AI-turn attribution remain unverified. Earlier GUI overhead calibrations remain scoped to their binaries and unpaced callback-free mode.

Documentation: [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md), [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md), [fps_overlay.md](docs/fps_overlay.md).

[Change record](docs/changes/2026-10-21-profile-realtime-audio-callbacks.json)

## 2026-10-21 — Service timer and audio events chronologically across native tick batches

Type: fixed. Platforms: Linux desktop, Windows GNU / Wine.

**Changes**

- Process scheduled events at their own master-clock timestamps before completing a tick batch, keeping audio-grid/timer tie ordering consistent with single-cycle execution.
- Route halt/IntrWait skips through the same clock path and service timers/audio during DMA without reentering video and restarting an active transfer. Extend the endpoint by stolen DMA cycles rather than replacing requested CPU work.
- Keep serialized fields, state schema, native ABI, translation coverage and intentional interpreter hooks unchanged. Existing version 1 slots still load, undo and reload; older executables may produce different subsequent audio.

**Validation**

- The new ROM-free batching regression fails on the old implementation. Five new regressions cover 24 batch/period combinations, both timers and every prescaler/cascade with HBlank DMA, exact sample/timer ties, sound refill/stolen-cycle accounting and skip equivalence.
- Final native and Windows GNU/Wine core suites each pass 179 tests, with 5 optional ignored; native runtime passes 66 with 4 ignored. Optional 3000-frame frame/OAM/audio/full-state observer regression also passes on the corrected core.
- Actual corrected native CLI preserves all 3530 old framebuffer/OAM hashes, initial snapshot/scene and dispatch counts. Full final state matches the isolated corrected baseline. Old/corrected baseline final CPU and clock are equal, with only mixed audio, HLE and timer bookkeeping fields differing intentionally.
- Corrected extra-seed coverage remains excluded: first 211 frames now match completely, but CPU/clock diverge at 212 and final IWRAM/HLE differ. The continuous-audio pair case did not run; no failed-coverage timing or production acceptance.
- Five alternating native uninstrumented headless pairs pinned to CPU 15 preserve each build's expected full final state and all protected inputs. Median mean work is 5.756460 to 5.823067 ms (+1.157%, +0.066607 ms). This is an audio correctness fix with measured cost, not a speedup.
- Native isolated GUI load of an old slot, zoom, Undo, reload and WM-close pass on Xvfb/Vulkan, without unexpected software fallback; battle and Undo images inspected. Canonical native binaries restored after isolating the candidate.
- Docker exports pass 349 Windows/Wine and 350 Linux tests, 15 optional ignored each. Archive integrity, private-data exclusion, bundled feature docs, source hashes, PE resources/import sets and GLIBC <= 2.35 checks pass.
- The extracted Linux export passes all 3530 old framebuffer/OAM hashes and counts, with full state exactly matching the corrected native reference. Actual cold-cache portable GUI rebuilds 28859 native blocks and passes old-slot load, zoom, Undo, reload and clean WM-close on Vulkan without fallback; battle/Undo captures inspected.
- Published the validated local Linux/Windows archives only after export gates, preserving previous archives under ignored timer-audio-order-exports/previous-exports. Installed the exact validated native runtime/launcher candidates so target/release agrees with current source. No commit, push or public release.

**Limitations**

- The original rare AI stall remains unisolated. Extra ROM seed coverage still fails full-state correctness and remains disabled. Wine/Xvfb checks do not establish native Windows, physical-controller or all hardware/DMA corner cases.

Documentation: [2026-10-21-isolate-rom-coverage-audio-divergence.json](docs/changes/2026-10-21-isolate-rom-coverage-audio-divergence.json), [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md), [save_states.md](docs/save_states.md).

[Change record](docs/changes/2026-10-21-order-timer-audio-events.json)

## 2026-10-21 — Observe successful host presentation intervals without changing pacing

Type: added. Platforms: Native Linux, Windows GNU / Wine / Docker.

**Changes**

- Add bounded opt-in host presentation cadence and guest-frame advance counts, reusing the FPS timestamp. Single-traversal frame range and lifecycle anchors exclude paused/priming/FF/zero-size boundaries while surface skips remain visible as longer intervals.
- Extract the existing pure diagnostic frame-window helper for audio/cadence reuse. Keep the observer off in default play and callback-free benchmarks, with unchanged guest state, rendering/DSP/scheduler policy and FPS behavior.

**Validation**

- Native runtime66 tests pass,4 optional ignored, including cadence skip/boundary/backend/window/cap/off cases and the existing audio gate and sample/queue parity tests. Initial helper initializer syntax error was caught by Rust parsing, corrected before successful validation; first test build failed, corrected build passed.
- Native CPAL/Xvfb/Vulkan llvmpipe matrix on existing scene window8596:9196 passes four clock/audio and requested-VSync cases: clock600successes/599intervals with one guest advance, mean16.7445/16.7427ms; audio237/236 with multi-guest advances, mean42.6843/42.6692ms. No skips/backend changes/omissions, all windows close normally and private slots unchanged.
- Strengthened skip/boundary test through the actual PlayProfile API and reran native suite:66passed,4ignored. Matrix release behavior is unchanged by that test-only edit. Silent fallback GUI using a process-local invalid ALSA config also passes a closed window with>500normal presentations and no multi-guest advances; no system settings changed. Clock/audio and silent-fallback captures inspected, FPS behavior consistent with measured cadence.
- Final container exports passed344 Windows/Wine and345 Linux tests,15 optional ignored each. Frozen source/doc hashes, archive privacy/integrity, resources/imported DLL set and GLIBC2.35 checks pass. Actual Linux archive cold-cache load/zoom/Undo/reload/resume/WM-close passes, battle/FPS/Undo captures inspected, Vulkan/no fallback/exit0.
- Actual archive cadence capture has566successes,8anchors,558intervals/guest advances,0skips/backend changes/repeated or multi guest frames. Undo interrupts both audio/cadence ranges and reload does not reopen them. Lifecycle audio capture retains1762faithful holds in4callbacks,0drops/over-period; not a steady-scene performance result. Default-off ignores malformed flags with no diagnostic rows; benchmark ignores them/no cadence rows and matches240reference frames without private writes.
- Updated canonical local portable archives only after checks, preserving previous exports. Existing clock/audio pacing, GPU backend, guest/save/cache ABI and default configuration remain unchanged.

**Limitations**

- Host API-success intervals are not GPU completion or scanout. Virtual requested-VSync coverage does not prove physical60/high-Hz/native Windows behavior or original slow enemy episode. CPU/GPU phases remain whole-session; cadence range applies only to the new observer. No scheduler optimization is claimed.

Documentation: [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md), [fps_overlay.md](docs/fps_overlay.md), [TASK_HANDOFF.md](docs/development/TASK_HANDOFF.md).

[Change record](docs/changes/2026-10-21-observe-successful-presentation-cadence.json)

## 2026-10-21 — Separate enhanced audio source demand from configured targets

Type: added. Platforms: Native Linux, Windows GNU / Wine / Docker.

**Changes**

- Count per-context PSG/HLE pair and FIFO event requests, empty reads and configured-target requests under the existing frame gate, without labelling inactive warm consumers as audible underruns.
- Compile per-request counting out of the normal DSP specialization; select once per stream with a callback dispatch, without new callback locks, logging, IO or allocation. Preserve queue policy, arithmetic, DAC holds and crossfades.

**Validation**

- Native runtime63 tests passed,4 optional ignored. New tests verify source counts including partial pairs/DAC holds, configured HLE/FIFO routing/backlog/mute labels, capped/gated totals, and exact output/queue/interpolator state parity; existing f32/i16/u16 callback parity passes.
- Five alternating synthetic profile off/on pairs passed, median40666.215/40696.723ns per512-frame callback (+0.0750%); not a game performance or default-before/after comparison.
- Inspected pinned CPAL0.16.0 ALSA XRun recovery: some EPIPE events are recovered without notifying the application, so device error counts cannot prove absence of backend XRUNs.
- Native CPAL idle-battle frame8596:9196 pair passes accounting/closed-window/slot-privacy checks. Faithful and enhanced each236 callbacks; all PSG/HLE/FIFO empty-read counters zero. Enhanced targets PSG/HLE while FIFO stays warm; faithful has no configured enhanced target. Both scene captures inspected. Initial harness variable-shadowing aborted before enhanced measurement; faithful reused and only missing enhanced measured after fixing the driver.
- Final container exports passed341 Windows/Wine and342 Linux tests,15 optional ignored each. Source/doc/archive privacy/integrity, PE resources/imported DLL set and GLIBC<=2.35 checks pass. Actual extracted Linux archive cold-cache lifecycle source accounting/load/zoom/Undo/reload/resume/WM-close passes with inspected battle/FPS and Undo captures, Vulkan/no unexpected fallback/exit0. Default-off produces no diagnostic rows and ignores invalid gate; callback-free benchmark ignores it and matches240reference frames without private writes.
- Lifecycle playing capture contains2741PSG/HLE empty-pair requests and660empty events per FIFO, all in one callback, plus2741faithful holds; all enhanced configured-target requests are0 with faithful selected. Retain this transition observation separately from the zero-empty idle window; it is not evidence of audible enhanced starvation.
- Updated canonical local archives only after final archive GUI checks and preserved old versions. No core/save/cache ABI, DSP policy, backend recovery or system audio configuration changed.

**Limitations**

- Configured-target counts do not represent live crossfade weight, voice activity or output audibility. One idle-battle pair does not establish AI-turn/native Windows performance or resolution of the original stall. Backend XRUNs require independent telemetry because pinned CPAL/ALSA can recover without app error notification.

Documentation: [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md), [TASK_HANDOFF.md](docs/development/TASK_HANDOFF.md).

[Change record](docs/changes/2026-10-21-observe-enhanced-audio-source-requests.json)

## 2026-10-21 — Isolate native coverage divergence to audio and timer batching

Type: docs. Platforms: Native Linux diagnosis.

**Changes**

- Complete the separate first-state-divergence investigation using existing baseline/candidate translations in an ignored standalone copy of the current runtime; no production source or package changes.
- Identify mixed-audio/HLE-verifier divergence and demonstrate ROM-free tick batch sensitivity, establishing a focused event-order correctness criterion before accepting further native coverage.

**Validation**

- Snapshot comparisons match exactly through37frames. First difference is relative38/guest8514 at equal clock2391464782 with equal CPU: two mixed samples and HLE verifier fields differ. Bounded JSON comparison reports22 difference entries, below its40-entry cap.
- Continued the standalone diagnosis to3530frames and reproduced both original final-state SHA-256 hashes exactly. Protected production source/runtime/original-library/fixture hashes remain unchanged; no benchmark timing repeated.
- ROM-free FIFO-A/128-cycle-timer fixture compares tick600 and600*tick1: equal final clock600 and DAC, different mixed samples and timer bookkeeping. It provides focused batch-sensitivity evidence without ROM input.
- Initial isolated build failed without pkg-config metadata; retained the failure log. Corrected build reused the existing Linux dependency sysroot and passed. Build26038, diagnosis24808 and timer fixture1662 are terminal; all helpers, snapshots and logs are ignored local artifacts.

**Limitations**

- The complete causal chain for the battle divergence and safe correction across DMA, skip, timers and video are not yet proved. No core fix or additional ROM seeds accepted; no speedup or original rare-stall resolution claimed. The helper is local diagnostic evidence, not a shipped command.

Documentation: [2026-10-21-evaluate-immutable-rom-entry-seeds.json](docs/changes/2026-10-21-evaluate-immutable-rom-entry-seeds.json), [mem.rs](crates/gba-core/src/mem.rs), [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md).

[Change record](docs/changes/2026-10-21-isolate-rom-coverage-audio-divergence.json)

## 2026-10-21 — Fall back cleanly when GPU surface configuration fails

Type: fixed. Platforms: Desktop Linux and Windows.

**Changes**

- Guard surface configuration at startup, resize, VSync switch and acquisition recovery with balanced validation/internal/out-of-memory error scopes. Return errors to existing software fallback instead of the default wgpu panic handler; do not replace the global error handler or retry configuration in a tight loop.
- Make the VSync setter return a configuration Result and drop a failed presenter in the menu caller. Preserve surface/context-before-window teardown and the existing bounded acquisition recovery policy. No guest/state/translation ABI changes.

**Validation**

- Release native screen/runtime suites passed 41/53 tests (3 optional ignored in each). Explicit graphics test passed real device error-scope capture, subsequent valid operations and preservation of caller-owned outer error scopes.
- Candidate in the previously failing Xvfb/Radeon-GL configuration now logs the returned Invalid surface error, activates software rendering and passes load/Undo/zoom/reload/resume/clean close. Resumed battle screenshot inspected at 59.7 INT/EXT; this is a functional check, not a comparative performance claim.
- Successful GPU path passed Xvfb/llvmpipe Vulkan resize, load/Undo/zoom/reload/close. A dedicated Right/Left menu test shows VSync ON then OFF with no GPU fallback; screenshots inspected. Initial Return-only captures were both OFF and were not treated as VSync coverage.
- Physical Radeon 780M/RADV Vulkan native Wayland replay passed all 3530 framebuffer/OAM hashes, final machine state and dispatch counts against the existing verified GUI reference, with private files unchanged and clean automatic close. No timing comparison was repeated for this configuration-only change.
- Final Docker gates passed 323 Windows/Wine and 324 Linux tests (11 optional ignored each). Archives passed integrity/private-data exclusion, updated FPS documentation identity, BUILD-INFO hashes, PE resource/unchanged import checks and Linux GLIBC 2.35 verification. The actual Linux export passed cold-cache software fallback, load/Undo/zoom/reload/resume/clean close with inspected screenshot.
- The actual exported benchmark in the same failed-GPU setup returns controlled exit 1, no panic or false success report, and no private-data writes. Original fixtures and source hashes remained unchanged; validated canonical local exports replaced only after checks, with prior versions preserved.

**Limitations**

- Actual memory exhaustion, runtime device loss, failed VSync reconfiguration, monitor switching and native Windows GPU/GUI behavior remain unverified. The shared scoped conversion is exercised with validation errors and a real initial configuration failure; these results do not establish all GPU failure modes.
- These are local portable exports, not a published release. No performance speedup is claimed for this configuration-error fix.

Documentation: [fps_overlay.md](docs/fps_overlay.md).

[Change record](docs/changes/2026-10-21-handle-surface-configuration-errors.json)

## 2026-10-21 — Return owned GPU device failures before resource creation can panic

Type: fixed. Platforms: Desktop Linux and Windows.

**Changes**

- Latch the first uncaptured error or loss callback on the Device owned by Presenter using Arc/OnceLock; return it to the existing software fallback instead of panicking when new UI or resized textures are created after device loss.
- Check failure state around uploads, acquisition, encoding/submission and VSync; suppress further UI writes on a latched failure. Preserve local scoped configuration errors, guest timing, rendering equations and normal backend selection. No per-frame allocation or GPU wait is added.
- Add a private ignored Linux/Windows device-destruction regression with existing workspace winit/softbuffer as dev-only dependencies. No public fault API, runtime feature or environment injection control is shipped.

**Validation**

- Baseline plain/cached UI destruction returned errors and software presentation succeeded, but expanded test reproduced a wgpu panic creating native-ui texture views after actual Device::destroy. Original failing log retained; resize was not reached in that failed run.
- Initial corrected native Xvfb/llvmpipe Vulkan test passed all four actual destruction cases (plain/cached/new UI/resize), controlled errors, software presentation and teardown. Added first-error retention, suppressed-write/VSync and scoped-error-isolation assertions; final native/Windows validation pending.
- Final native release suites passed58 runtime/46 screen tests (4/6 optional ignored). Expanded actual destruction regression passes all four scenarios plus scoped-error isolation, first-error retention, suppressed writes and VSync rejection. Windows GNU build and explicit Wine10/Vulkan regression also passed all assertions and software surface presentation.
- Final native default GUI with profiling off passed load/zoom/Undo/reload/resume/WM close with resumed battle/HUD capture inspected; log confirms Vulkan and no unexpected software fallback. Physical Radeon Vulkan/Wayland replay matched all3530 image/OAM hashes, full state and native/fallback counts with unchanged private data and clean close. Functional verification overlapped container builds; its timings are excluded from performance claims.
- Final Docker gates passed333 Windows/Wine and334 Linux tests (15 optional ignored each); the new optional device-destruction regression was separately executed on native Linux and Wine. Archives pass integrity/private-data exclusion, updated bundled doc identity, final source hashes, PE resources/imported DLL set and GLIBC2.35 checks.
- Actual final Linux archive with cold cache passed normal Vulkan load/zoom/Undo/reload/resume/WM close, no unexpected software fallback, inspected battle/HUD and Undo captures. Updated local dist archives and BUILD-INFO after checks, preserving prior exports; no public release. Windows archive SHA25685a012f6eff45d618d3b654db9cf1839682db6f73cefe89e434c99e96709b05d; Linux1bf8ca4afd0e364a7e315ec63b29ab15bebfa857ff147fe97a9b14f82643c70d.
- project_memory.py render/check/check --base HEAD, rustfmt --check on the private regression and Git whitespace validation passed before handoff.

**Limitations**

- Explicit Device::destroy proves deterministic invalidation/new-resource recovery, not every asynchronous hardware-reset race, physical GPU fault or native Windows/Wayland device-loss scenario. The software surface is exercised directly after Presenter Err; app fallback control flow is unchanged.
- No AI-turn or normal-frame speedup claim. Physical Wayland verification overlapped builds and is correctness evidence only. Guest/save formats, backend selection and AndroidPort are unchanged; the broader performance TODO remains open.

Documentation: [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md).

[Change record](docs/changes/2026-10-21-handle-owned-gpu-device-failures.json)

## 2026-10-21 — Select one realtime audio profile interval by published guest frame

Type: added. Platforms: Native Linux, Windows GNU / Wine / Docker.

**Changes**

- Add optional FFTA_AUDIO_PROFILE_FRAMES=START:END to profile-enabled normal play, retaining context-separated callback timing and counts only for one producer-published frame traversal; close on end or timeline restore/rewind.
- Keep DSP, callback code, queues and audio policy unchanged; no new callback lock, IO, allocation or clocks. Default play and callback-free benchmarks ignore the optional window.

**Validation**

- Native runtime suite passed60 tests,4 optional ignored, including gate boundary/rewind/restore and actual callback output/queue/counter parity. Normal CPAL GUI same idle-battle600-frame window captures236/235 callbacks for faithful/enhanced, no faithful holds/drops/over-period callbacks; screenshots inspected.
- Final Docker exports passed338 Windows/Wine and339 Linux tests,15 optional ignored each; archive integrity/privacy/docs/source hashes, resources/imported DLL set and GLIBC2.35 checks pass. Actual extracted Linux archive cold-cache GUI passes load/zoom/Undo/reload/resume/close; inspected battle/FPS/Undo captures, Vulkan without unexpected fallback. Undo interrupts the window and reload does not reopen it. Default-off ignores invalid window with no audio diagnostic rows; callback-free benchmark ignores it and matches240 reference frames without private writes.
- The main.rs callback/DSP source is byte-identical to the preserved preimage. Native scene measurements predate only a guard avoiding optional no-stream profiling mutex work in callback-free benchmarks; the measured normal-play path is unchanged.
- The lifecycle-window test observes1309 faithful holds in7 of222 playing callbacks, with0 drops/over-period callbacks; it includes transitions and concurrent native compilation and is not a steady-scene timing comparison. Retain this observation rather than asserting that all audio holds are absent.
- Final native runtime/launcher build passes. Canonical local Windows/Linux archives updated only after checks; previous archives preserved under the current case.

**Limitations**

- Native Windows/device timing, enhanced-source shortages and driver XRUNs remain unverified. One idle-scene pair is not an AI-turn, original-stall resolution or mode-performance comparison. Frame publication attributes callbacks to producer boundaries, not per-sample guest provenance; CPU/GPU profiles remain whole-session.

Documentation: [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md), [TASK_HANDOFF.md](docs/development/TASK_HANDOFF.md).

[Change record](docs/changes/2026-10-21-gate-realtime-audio-by-guest-frame.json)

## 2026-10-21 — Flush pending GPU uploads when presentation is skipped

Type: fixed. Platforms: Desktop Linux and Windows.

**Changes**

- Track pending UI/framebuffer/uniform Queue writes across updates and flush them with transfer-only submissions when presentation is skipped, preserving revision caches and normal draw/FPS/timestamp behavior.
- Skip frontend presentation preparation for zero-size windows, avoiding new UI uploads and software-surface creation without altering guest/audio progress.
- Add an optional no-ROM Presenter lifecycle example covering 1200 zero-size batches, hidden/reactivated UI, LUT/world changes, preserved pixel revisions and restored normal presentation.
- Preserve the benchmark contract on zero-size early returns by reporting a skipped presentation instead of silently accepting a frame.

**Validation**

- Native release suites passed 58 runtime and 46 screen tests (4/5 optional ignored), including batch consumption and existing bounded acquisition recovery cases; native release runtime/launcher and the diagnostic example build passed.
- Actual Presenter diagnostic under Xvfb/llvmpipe Vulkan passed 1200 zero-size UI batches, normal recovery and teardown. Scheduled 492179776 logical bytes; RSS 130672 -> 133232 KiB, unchanged from batch 600 to 1200. No blocking GPU poll was added.
- Normal native GUI load/zoom/Undo/reload/resume/WM close passed with inspected resumed-battle HUD capture. That executable predates only the subsequently added zero-size benchmark error guard; normal window behavior is unchanged.
- Final review found the early zero-size return initially bypassed the benchmark skip error. Windows export build was interrupted and Linux had already completed; these preliminary outputs were not promoted. Updated the guard and source manifest before final builds.
- After the zero-size benchmark guard fix, native release suites again passed 58 runtime/46 screen tests (4/5 optional ignored) and the final native runtime/launcher build passed. No guest code, shader or save-format changes required new scene recordings or A/B calibration.
- Final corrected Docker builds passed 333 Windows/Wine and 334 Linux tests, 14 optional ignored each. Archives passed integrity/private-data exclusion, updated bundled feature-doc identity, unchanged final source manifest, PE resources/imports and GLIBC 2.35 verification. Running Linux archive hashes match the inspected export.
- Actual final Linux portable with cold cache passed load, zoom-6 running, Undo load, reload/resume and WM close with profiling off and both FPS counters. Inspected battle/HUD and Undo captures. Promoted inspected Windows/Linux archives and BUILD-INFO locally, preserving previous exports; no public release.
- project_memory.py render, check and check --base HEAD and the Git whitespace check passed before handoff.

**Limitations**

- Real compositor minimization, monitor switching, forced runtime device loss and native Windows GPU/GUI remain unverified; the zero-size API diagnostic and simulated acquisition tests do not establish those cases. The broader performance TODO remains open.
- No AI-turn speedup or comparative normal-frame timing claim. Linux RSS observations describe one llvmpipe Vulkan run, not a universal GPU memory budget. No guest/state/save-format or shader change.

Documentation: [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md).

[Change record](docs/changes/2026-10-21-flush-skipped-presentation-uploads.json)

## 2026-10-21 — Skip world-only sorting in native UI capture

Type: performance. Platforms: Native Linux, Windows GNU / Wine / Docker.

**Changes**

- Track actually admitted UI BG/portrait candidates per pixel. With none, keep the default destination write to clear stale data while bypassing world-candidate sorting and blending; retain the full loop as reference.
- Keep row classification cache, real UI ordering/blending, windows, modal/backing and capture/anchor behavior; no state/cache ABI or guest timing change. Retain after headless and complete GUI work improvements with exact replay parity.

**Validation**

- Native serial core suite passed174 tests,5 optional ignored. New mixed-row full-state comparisons exercise world-only holes, stale destinations, world alpha and portrait-only Stats UI, asserting the fixture actually reaches each case; existing full panel reference tests pass.
- Native runtime/launcher and Windows GNU builds pass; Windows/Wine serial suite passed174 core tests,5 optional ignored. Headless replay matches all3530 framebuffer/OAM hashes, full final state and execution counts. Five alternating pairs improve median5.846082 to5.600360ms (4.2032%); separate profile p95/p99 also lower.
- Native GUI correctness replay passed all3530 framebuffer/OAM matches, state/counts and private-data checks. Five alternating full-GUI pairs reduce median mean7.584102 to7.381503ms (2.6714%); all pairs improve2.018-2.953%.
- Worldmap240-frame image/state/count parity and two1200-frame diagnostic pairs pass without regression. Zoom0:6 matches all3530 framebuffer/OAM hashes, full state and counts. Default GUI load/zoom/Undo/reload/resume/WM-close passes with Vulkan and no unexpected software fallback; battle/FPS and Undo captures visually inspected.
- Container exports passed336 Windows/Wine and337 Linux tests,15 optional ignored each. Archive integrity/privacy, bundled docs, source hashes, PE resources/imported DLL set and GLIBC<=2.35 checks pass. Actual extracted Linux cold-cache GUI load/Undo/zoom/reload/resume/WM-close passes; battle/FPS and Undo captures inspected, Vulkan/no fallback/exit0 confirmed. Local dist archives replaced only after validation, previous archives preserved.
- GUI median p95/p99 improve9.172928/9.698431 to8.846837/9.317414ms; scheduled UI payload stays316299008bytes. All source hashes remained frozen during validation.

**Limitations**

- Native Linux unpaced work timing does not establish native Windows performance or resolution of the original rare AI stall. Physical controller, hardware fault and broader interactive pacing/audio coverage are unchanged.

Documentation: [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md), [TASK_HANDOFF.md](docs/development/TASK_HANDOFF.md), [fps_overlay.md](docs/fps_overlay.md).

[Change record](docs/changes/2026-10-21-evaluate-world-only-ui-pixels.json)

## 2026-10-21 — Cache native UI panel membership per row and eligible pixel

Type: performance. Platforms: Native Linux evaluation.

**Changes**

- Implement a per-call panel rule snapshot with cached vertical membership and lazy per-pixel classification shared across eligible BG candidates. Preserve banner precedence, missing-bounds laziness and the original uncached policy in a test specialization.
- Preserve sorting, blending, existing window-register cache, serialized state and translation ABI. Retain after exact replay/panel checks and five-pair improvements in headless and complete GUI work.

**Validation**

- Native serial core suite passed173 tests,5 optional ignored. New exhaustive old-policy comparisons cover action/town/shop flags, panel kinds, banners, rectangles, vertical/horizontal edges and primary/extra panels; missing bounds remain lazy for overriding banners and rows without eligible BG. Existing full panel-state comparisons now use the uncached policy reference.
- Native runtime/launcher and Windows GNU core tests compile; Windows/Wine serial core suite also passed173 tests,5 optional ignored. Headless evaluation matches all3530frame/OAM hashes, full state and dispatch counts; five timing pairs/diagnostics still running.
- Completed five native headless3530-frame A/B pairs with120 warm-up: all hashes/state/counts equal, all pairs faster. Median6.001925 ->5.840709ms (-2.6861%). Separate instrumented p95/p99/max7.081627/7.251019/8.735002 ->6.864737/6.994522/7.344674ms; PPU mean3.806301 ->3.650077ms, core remainder2.196837 ->2.191594ms. No over-budget frames in that diagnostic pair.
- Complete unpaced native Linux llvmpipe GUI five pairs pass: median mean7.767086 ->7.591386ms(-2.2621%), all pairs improve2.108-2.641%; median p95/p99/max9.397574/9.895277/11.371842 ->9.136884/9.641691/11.062622ms. All3530frames/state/counts/private data match; UI payload316299008bytes unchanged.
- World-map setup initially omitted the existing empty.input file and failed before guest execution. Copied the existing fixture; rerun only that uncovered scene, not completed timing pairs.
- Corrected world-map run passed240frame parity/state/counts and two alternating1200frame diagnostic pairs: candidate3.024185/3.024873ms, baseline3.027517/3.087637ms, private data unchanged. No world-map speedup claim.
- Zoom0:6 verification passed all3530frame/OAM hashes, full state and dispatch counts against original baseline; inputs unchanged. Normal default GUI with profiling off passed load/zoom/Undo/reload/resume/WM close on Vulkan without fallback; resumed battle/HUD capture inspected.
- Final Docker exports passed335 Windows/Wine and336 Linux tests (15 optional ignored each), archive integrity/privacy/doc/source hashes, PE resources/imported DLL set and GLIBC2.35. Actual extracted Linux portable passed cold-cache load/zoom/Undo/reload/resume/WM close with profiling off, Vulkan without fallback and inspected battle/HUD/Undo captures.
- Updated local dist archives and BUILD-INFO after checks, preserving previous versions: Windows SHA256edb9a43aceea8c673937f5cdfb2431dc0da080ac600339570b92d1f6b93d669c; Linuxacbcf0e556be1f340a18c38300650874c70de0b8c02a01e19fe748bb114b7a09. No public release.
- project_memory.py render/check/check --base HEAD and Git whitespace validation passed; all evaluation/build/GUI handles terminal, no duplicate calibration or fixture recording.

**Limitations**

- Measurements describe work on the supplied native Linux replay, including unpaced llvmpipe GUI with CPU profiling enabled; no audio callback/GPU timestamp/per-frame hash I/O in timing. No original rare-stall, native Windows hardware/performance or AndroidPort claim.
- Cache is local to one capture call/pixel; no cross-frame invalidation or state/translation ABI change. Earlier lifecycle/device-loss validation remains applicable; those unrelated tests were not repeated.

Documentation: [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md).

[Change record](docs/changes/2026-10-21-evaluate-ui-panel-row-classification.json)

## 2026-10-21 — Reuse native UI packing for unchanged complete inputs

Type: performance. Platforms: Linux desktop, Windows GNU under Wine.

**Changes**

- Implement an input cache keyed by raw source pixels, backing and exact resolved packing mode/length/right parameters. Hits skip pure packing and use the existing GPU revision contract; world, palette, layout and Stats background stay dynamic.
- Use owned input snapshots with an exact bijective 32-bit field representation, exhaustive UiPixel destructuring and no unsafe layout/endianness casts. Changes conservatively invalidate packed output and clear/restore resets all host cache data; FPS and native producers share unique revision allocation.

**Validation**

- Native runtime/screen suites passed 55/41 tests (3 optional ignored each), including no repacking on identical keys, in-place source/backing changes, mode/length/anchor changes, clear/reactivation, and direct packing parity for blends and backing variants.
- Initial struct-equality trial was too costly in UI preparation (0.203124 ms diagnostic) and was replaced before acceptance timing by the exact word representation; original artifacts are preserved locally.
- Final exact-word implementation matched all 3530 reference framebuffer/OAM hashes, full final state and dispatch counts with private files unchanged. Five alternating A/B pairs reduced median mean host work 8.339864 to 8.222313 ms (-1.4095%); all pairs improve 1.39-1.92%, and median p95/p99 improve. UI logical payload 2011423488 to 530412288 bytes.
- World-map new-scene check matched all 240 framebuffer/OAM hashes, final state and counts; two alternating 1200-frame diagnostic pairs had comparable means (baseline 3.055/3.106 ms, candidate 3.077/3.064 ms), private files unchanged.
- New native Linux GUI load, six zoom steps, Undo load, reload, resume and close passed; inspected captures show clean preview and gameplay with FPS HUD. Physical Radeon/Vulkan Wayland replay matched all 3530 frame/OAM hashes, final state and counts and exited successfully.
- GUI text-panel length changed from 100% to 150% on the action-menu snapshot; inspected before/menu/after captures confirm repacking/layout update and clean exit. This run overlapped builds and is not timing evidence.
- Docker exports passed 325 Windows/Wine and 326 Linux tests (11 optional ignored each). Archives passed integrity/privacy, bundled feature-doc identity, PE resources and unchanged DLL-import sets, and Linux GLIBC 2.35 checks.
- Actual extracted Linux export passed isolated cold-cache GUI load/zoom6/Undo/reload/resume/close with profiling disabled and inspected screenshots. Initial harness setup failed before app launch on an existing empty Data directory/missing working directory; corrected only the isolated setup, then the check passed.
- Canonical local portable archives and BUILD-INFO were updated after validation; prior exports and source/artifact hashes are preserved in ignored target/desktop-perf/ui-input-cache-exports. project_memory.py render/check/check --base HEAD and whitespace check passed.

**Limitations**

- Timing benefit is measured on an unpaced llvmpipe Vulkan workload, not native Windows performance or proof of an AI-turn speedup. Native Windows GUI/physical-controller coverage remains absent.
- Owned source/backing snapshots retain additional memory proportional to current layer sizes, with at most two source/backing pairs and no historical cache entries. No serialized state or translation ABI change; shared world-texture uploads remain separate PERF-05 work.

Documentation: [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md), [fps_overlay.md](docs/fps_overlay.md).

[Change record](docs/changes/2026-10-21-evaluate-ui-packing-input-cache.json)

## 2026-10-21 — Share one world upload per native UI source and presentation

Type: performance. Platforms: Linux desktop, Windows GNU under Wine.

**Changes**

- Implement scoped reuse that shares GPU world handles only for identical immutable source slices and dimensions in one set_ui_layers call; upload each distinct world again on every call. Keep independent LUT/layout/UI revisions and Stats/FPS backgrounds.
- Reuse existing bound resources where safe, split previously shared handles before divergent writes, and rebind when source groups or dimensions change. No cross-frame address-based content cache or guest/serialization/translation change.

**Validation**

- Native 58 runtime/41 screen suites passed (4 optional ignored each) and the added CPU source-key test passed separately. Same extent/source/dimensions are required; layout/view differences do not prevent same-call sharing.
- Explicit old single-layer and new shared-layer GPU tests passed on AMD Radeon 780M/RADV. New test checks CPU/pixel equality and exact upload bytes across source mutation, split/merge/reordering, absent/reactivated layers, 1x1/320x160/640x320 dimensions and layer recreation.
- Candidate passed all existing 3530 framebuffer/OAM hashes, final state and dispatch counts, private files unchanged. Logical payload is 316299008 bytes versus 530412288 for the prior cache implementation; verification timing is excluded because compilation overlapped.
- Extended distinct per-layer pixel/LUT/layout/crop test exposed a fractional CPU/WGSL texel-boundary mismatch. Exact same-case GPU output matches independent per-layer world uploads, including all fractional crops; integer crops also match CPU. The timing command chained after the failed strict-CPU check did not start. After adding independent-path verification, five new-code A/B pairs completed (87605), all paired means improve 0.336-0.773%. Median mean 8.251393 -> 8.218974 ms (-0.3929%); median p95/p99 10.068110/10.571226 -> 10.015975/10.458356 ms.
- World-map check passed all 240 reference image/OAM hashes, full state/counts and two alternating 1200-frame diagnostic pairs (baseline 3.022/3.084 ms, candidate 3.045/3.024 ms), private data unchanged. No systematic regression established by this short check.
- Native GUI load/zoom6/Undo/reload/resume/close passed with profiling off and FPS HUD active; inspected captures show correct preview/gameplay. Physical Radeon/Vulkan Wayland replay matches all 3530 reference hashes, full state/counts and exits cleanly.
- Final Docker suites passed 330 Linux and 329 Windows/Wine tests (13 optional ignored each). Archives passed integrity/privacy and bundled-doc identity, PE resource/import compatibility and Linux GLIBC 2.35 checks.
- Actual new Linux portable passed isolated cold-cache GUI load/zoom6/Undo/reload/resume/close with FPS HUD, profiling off and inspected captures. Canonical local exports and BUILD-INFO updated after validation; previous archives preserved.
- Compacted the task handoff into one current queue and a closed/rejected ID index with evidence links; retained artifact locators and scope limits instead of duplicate historical live-process guidance. project_memory.py render/check/check --base HEAD and whitespace check passed.

**Limitations**

- Small unpaced llvmpipe timing gain; no isolated AI-turn or native Windows performance claim. Native Windows GPU/physical-controller and full device/lifecycle fault coverage remain unavailable.
- Fractional world-view widths can select neighboring texels in CPU versus WGSL even with independent uploads. The new sharing path must match independent GPU output; broad arbitrary-float CPU/GPU equality is not established.

Documentation: [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md).

[Change record](docs/changes/2026-10-21-evaluate-shared-world-uploads.json)

## 2026-10-21 — Reject packed-byte UI upload caching after end-to-end evaluation

Type: docs. Platforms: Native Linux desktop evaluation.

**Changes**

- Evaluated then removed a candidate that packed into reusable scratch buffers, compared complete output bytes and supplied process-unique UI revisions. No production runtime change is retained; existing unversioned/native UI, FPS caching and validated portable exports remain the baseline.
- Record the rejected result so context recovery does not repeat this byte-comparison-only design. Any follow-up must address the measured preparation cost, for example by avoiding packing when its complete input tuple is unchanged, rather than only avoiding transfer after packing.

**Validation**

- Native runtime/screen suites passed 55/41 tests (3 optional ignored each). New tests compare packed panel output across lengths/anchors, content changes, reusable capacity, clears/reactivation and independent producers; existing GPU pixel/byte-equivalence test passed on Radeon 780M/Vulkan.
- Baseline and candidate passed the existing 3530-frame GUI verification: all image/OAM hashes, final machine state and dispatch counts equal the established reference, private data unchanged. Logical UI upload payload fell from 2011423488 to 530412288 bytes over 3410 measured updates. Verification timings are not used to accept the optimization.
- Five alternating new-code A/B pairs passed with 3530 additional frames and 120 warm-up per run. Median run means baseline 8.328947 ms, candidate 8.339867 ms (+0.131109%); paired changes range -0.008598% to +0.840389%. All final states, dispatch counts, inputs and private files agree.
- Median UI preparation mean rose 0.156532 -> 0.179629 ms; upload mean fell 0.129143 -> 0.103490 ms. Median per-run combined UI work only fell 0.285154 -> 0.281867 ms, not a useful end-to-end improvement despite about 74% fewer logical upload bytes.
- Preserved candidate source/binaries/logs under ignored target paths, then restored only the four experiment preimages and the prior native executable byte-for-byte. Removed the experiment-owned new module. Published portable archives were never replaced for this rejected candidate.

**Limitations**

- This is a scoped unpaced llvmpipe/Vulkan evaluation, not native Windows or hardware GPU performance evidence. Logical scheduled bytes do not establish PCIe traffic, power savings or a speedup.
- The rejected design has no retained implementation. Previous baseline validation remains applicable because source/binary restoration is exact; no duplicate build or calibration was run merely to recover the baseline.

Documentation: [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md).

[Change record](docs/changes/2026-10-21-evaluate-native-ui-upload-revisions.json)

## 2026-10-21 — Reject additional ROM entry coverage on full-state divergence

Type: docs. Platforms: Native Linux headless / TinyCC.

**Changes**

- Evaluate18 already observed immutable ROM entry labels with the existing isolated build path; no mutable battle RAM, intentional hook, production runtime, game database or package change.
- Reject production acceptance when full final machine state differs despite equal3530-frame framebuffer/OAM; stop before timing and preserve diagnostic evidence.

**Validation**

- Read the existing census top missing-entry list; SQLite read-only lookup confirms zero mapped functions for the fixture image. Reused that census without rerunning it.
- Sequential matched TinyCC builds pass. Reconstructed baseline library is byte-identical to the existing translation; candidate grows28859 to29196blocks. Both use the same isolated runtime, boot/RAM discovery, exact ROM and compiler inputs.
- Two verification runs match all3530 framebuffer/OAM hashes, initial snapshot identity, warm-up scene and dimensions. Full final serialized state differs: baseline38cc54374bbc23dafd697492c2600697013a06e2d75dd68a31c5f1f9d0e2a9ac, candidate7393d554c2ec878bc0d8beb5fd0655b332f986c3de45050c680e1363a46f0b01. Verification fails as intended; zero timing pairs performed.
- Baseline/candidate dispatch counts23355688/83764597 and24121477/61353461 are not host timing evidence. Verified protected fixture/runtime/original-library hashes unchanged after failure. Build session74952 and evaluation4376 terminal; local artifacts remain ignored.

**Limitations**

- The differing serialized component and first divergence are not yet isolated. Equal images do not establish guest equivalence; no speedup, audible defect or original rare-stall diagnosis is claimed. No new translation accepted or portable archive rebuilt.

Documentation: [2026-10-21-characterize-enemy-fallback-host-cost.json](docs/changes/2026-10-21-characterize-enemy-fallback-host-cost.json), [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md).

[Change record](docs/changes/2026-10-21-evaluate-immutable-rom-entry-seeds.json)

## 2026-10-21 — Reject pre-wait composition after contrary presentation-tail measurements

Type: performance. Platforms: Native Linux evaluation.

**Changes**

- Audit the temporal/LUT composition as host-only immutable-input work and evaluate moving it before the existing deadline wait without changing guest/history/audio production or deadline policy.
- Reject this reorder after five alternating realtime cadence pairs show worse p99 in4/5pairs. Restore exact accepted play.rs and native runtime, preserving candidate source/binary and results; portable exports unchanged.

**Validation**

- Audited Temporal::compose and ColorLut use: immutable Temporal/history/LUT inputs, writes only the private RGBA buffer, no clock, IO or shared mutable state. Sleep pumps no events; temporal advancement remains in push before both orders.
- Native runtime66 tests and screen46 tests pass,4/6optional ignored, native runtime/launcher build passes. Five alternating realtime clock/immediate pairs each capture600successes/599intervals over the same guest window,1anchor,0skips/repeats/multi-guest/backend changes/omissions and0faithful holds, with private slot hashes unchanged. Baseline/candidate scene captures inspected; Vulkan/no fallback/clean exit.
- Median interval means16.742582/16.742562ms; p95 improves17.509748 to17.464329ms but p99 worsens17.959623 to18.020603ms and median maximum18.189055 to18.331738ms. Four of five paired p99 results worse; no cadence benefit accepted.
- Source/input hashes verified before preserving trial artifacts. Source restored by exact bytes with fresh mtime for Cargo invalidation; normal runtime restored by exact bytes/hash. No additional Windows/export gates or unpaced benchmark used to justify the rejected normal-loop reorder.

**Limitations**

- One idle scene with native CPAL/Xvfb/llvmpipe, not physical scanout/high-Hz/native Windows or original AI-stall evidence. Results do not prove universal regression; they do not justify this pacing change. Further validation of rejected code is not pending work.

Documentation: [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md), [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md), [blend.rs](crates/screen/src/blend.rs).

[Change record](docs/changes/2026-10-21-evaluate-composition-before-deadline.json)

## 2026-10-21 — Reject compact UI layer IDs after modest inconsistent GUI benefit

Type: performance. Platforms: Native Linux evaluation, Windows GNU / Wine core tests.

**Changes**

- Complete current optimized host CPU triage after retained PPU changes, reusing the existing enemy replay and preserving the normal runtime.
- Evaluate u8 instead of usize temporary UI IDs with unchanged sorting/blending/serialization. Reject the candidate after modest inconsistent GUI benefit and contrary tail observations; restore source and native runtime byte-for-byte from their exact accepted baselines. Preserve trial binaries/source/evidence; no export or ABI change.

**Validation**

- New199Hz user CPU sample after120-frame warm-up captured3798 samples with0 lost; final full state and execution counts match. Source/fixture and restored normal runtime hashes match. UI capture31.20%, sprite renderer9.19%, composition8.85%, interpreter7.64% self samples; this is attribution, not speedup.
- Compact-ID candidate passes174 serial core tests,5 optional ignored and native runtime/launcher build. All3530frame/OAM hashes, final state and counts match. Five headless pairs favor candidate, median5.756819 to5.679220ms (-1.3480%). Separate instrumented profile run is slower5.755066 to5.833724ms; do not infer tail improvements or hide this contrary observation. GUI evaluation pending.
- Windows GNU build and Wine174core tests pass,5 optional ignored. GUI verification matches all3530 framebuffer/OAM lines, full final state and execution counts against the existing reference and preserves private Data. GUI timing pending; no concurrent compilation during timing.
- Five GUI CPU-profile-on pairs pass state/count/input/private-data checks: median7.552612 to7.518334ms (-0.4539%), one pair slower; median maximum11.518421 to15.911304ms. Diagnostic-off comparison, required by contrary instrumented core data, also passes:7.563614 to7.505102ms (-0.7736%), one pair slower, paired differences range+0.1469% to-1.4330%; median maximum12.100617 to16.226909ms. These observations do not prove a universal causal regression or robust useful normal-path gain.
- Verified frozen candidate source/binary/input hashes before preserving the evaluated candidate, then restored ppu.rs and normal native runtime exactly by bytes/hashes. Completed jobs are terminal; no prior benchmark/fixture recreated and canonical portable archives unchanged.
- Refreshed restored source mtime to invalidate stale Cargo candidate artifacts on future builds while verifying exact baseline bytes. Memory/changelog checks and whitespace verification complete at handoff.

**Limitations**

- Current host sample excludes window/audio callbacks and does not isolate missing versus intentional interpreter work or diagnose the original rare stall. Native Windows performance was not measured. The rejected layout must not be reactivated without distinct evidence. GUI worldmap/zoom/lifecycle/export gates were intentionally not run after the acceptance decision failed.

Documentation: [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md), [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md).

[Change record](docs/changes/2026-10-21-evaluate-compact-ui-layer-ids.json)

## 2026-10-21 — Observe native audio stream and sink errors independently through PipeWire

Type: docs. Platforms: Linux native CPAL / PipeWire / Xvfb Vulkan.

**Changes**

- Document a bounded read-only backend observation with one line-buffered pw-top, PID-to-client-to-stream/link/sink identity checks, stable object serials and validated sample cadence; preserve existing runtime and system audio policy.

**Validation**

- Accepted capture has10/3/6 running samples over approximately9/2/5seconds for idle battle/pause/resumed play, maximum sample gap below1.001s; stream and sink ERR deltas0 in each observed interval. Sink ERR already1 at first idle sample, with no causal or hardware-XRUN inference.
- Native portable runtime hash equals the previously validated canonical Linux export. CPAL frame window8596:9196 completes235playing callbacks without faithful holds/drops/over-period or device error notifications. Idle/resumed screenshots inspected; Vulkan/no fallback/clean exit and slot privacy pass.
- First pw-dump sandbox read was unable to connect in the sandbox; approved read-only elevated inventory succeeds. First GUI capture completes but piped pw-top batches paused rows at one read time, invalidating phase timing. Preserved it and repeated only backend collection with stdbuf -oL; the corrected cadence/identity analysis passes.

**Limitations**

- Shared graph ERR is not a dedicated ALSA hardware-XRUN counter or proof of game causality. Creating-node zeros do not establish a valid running baseline. These are short idle-scene wall intervals, not exact app frame-window boundaries, AI-turn or native Windows/hardware coverage.

Documentation: [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md), [TASK_HANDOFF.md](docs/development/TASK_HANDOFF.md).

[Change record](docs/changes/2026-10-21-characterize-pipewire-backend-errors.json)

## 2026-10-21 — Attribute observed enemy replay dispatches and remaining CPU cost

Type: docs. Platforms: Native Linux headless.

**Changes**

- Complete fallback-candidate triage using existing observed enemy snapshot/input and native translation. No production translation, lookup, hook, guest ABI, runtime or package change accepted from counts alone.
- Prioritize remaining native UI panel extraction work from a new host CPU sample, while preserving completed/rejected optimizations and the broader open performance TODO.

**Validation**

- Confirmed the only existing COST_DISPATCH log covered the intro; collected one new3530-frame enemy-scene census after120 warm-up frames. All3530 framebuffer/OAM hashes, full state and native/fallback counts match the existing headless reference; ROM/state/input/library/runtime hashes unchanged.
- Warm-up-excluded dispatches:22626079 native,79140200 missing,6820 system,55906060 intentional hooks;3804 fallback entry keys across the full replay (3729ROM/75IWRAM). These are counts and guest-cycle attribution, not host CPU shares.
- Built only the final optimized recomp crate with strip disabled/debug1 for symbols, isolated it under ignored target storage and restored the original native binary by hash. One199Hz user CPU sample enabled after the8596 warm-up marker collected3928 samples with zero lost; final guest state/counts match reference. No census or per-frame hash I/O during sampling.
- Leaf self samples: capture_dialogue_part31.52%, render_sprites_extent8.63%, compose_into7.76%, exec::step_inner6.21%, render_text_bg_hardware5.14%, Machine::step4.56%. Leaf percentages are not inclusive timing or an isolated AI interval; no speedup is claimed.
- project_memory.py render/check/check --base HEAD and Git whitespace validation passed. Profiling process handles are terminal and original target/release/recomp hash is restored; shipping archives were not rebuilt.

**Limitations**

- Call-chain reconstruction does not isolate all missing versus intentional interpreter work. One headless native workload excludes realtime audio/window behavior and the originally reported rare slow episode; no native Windows performance claim.

Documentation: [PROJECT_STATE.md](docs/development/PROJECT_STATE.md).

[Change record](docs/changes/2026-10-21-characterize-enemy-fallback-host-cost.json)

## 2026-10-21 — Align native UI texel selection with the software sampling policy

Type: fixed. Platforms: Native Linux, Linux portable (Docker Bookworm), Windows GNU/MinGW (Docker, Wine tests).

**Changes**

- Implement canonical sampling maps: compute integer UI/world indices using the existing CPU formulas, cache a bounded two-row RG32Uint map by exact layout/clip/view/dimension bits, and upload only changed maps. Shader lookup uses integer pixel indices instead of backend-dependent float floor expressions.
- Retain the existing software policy, guest state, color blending, world-sharing and pixel/LUT revisions. Map origins/extents are bounded by device limits, with invalid UI entries for clipping and complete cache invalidation at resource recreation.
- Conservatively bound UI rasterization using the exact transparent-pixel predicate and the integer sampling maps. Recompute source bounds only on pixel uploads and scissor only on bounds/geometry changes; preserve active source ownership even when no pixels draw.

**Validation**

- Existing runtime/screen suites passed 58/44 tests (4/5 optional ignored), including two new map tests for legacy boundaries, clipping/clamping, complete key invalidation, cache hits and oversized-layout storage bounds.
- On Radeon 780M/Vulkan, all three explicit native UI GPU tests passed: formerly fractional cases now match CPU for every layer; runtime 852x480 zoom 0-6 and exact-x2 variant have zero mismatches, plus previous revision/color/world-sharing byte checks.
- New baseline runtime test found 43098 differing pixels at zoom 6 (852x480 synthetic patterned data), zero at zoom 0-5. Exact-x2 clamp still differed (86474); no production zoom clamp is proposed. Native baseline executable/source is reused from accepted shared-world implementation.
- Expanded resize/DPI reuse matrix initially failed in test comparison indexing (one row-offset expression retained fixed width 852); production renderer had no failure. Corrected only the harness and rechecking that matrix before acceptance.
- Expanded persistent-resource matrix passed 32 zoom/resize/DPI states at 852x480, 853x481 (125% UI scale), 320x160 and 1704x480, including original and exact-x2 views. All compared pixels match CPU with zero differences. Harness row indexing was corrected before this pass.
- Table-only prototype completed five A/B pairs: median mean 8.220145 -> 8.562769 ms (+4.1681%), all pairs slower. Correctness/state/counts pass; prototype retained locally but not accepted. This regression justifies changing raster coverage and measuring the changed implementation, rather than shipping the slowdown.
- Bounds candidate native suites passed 58 runtime/45 screen tests (4/5 optional ignored) and all three explicit Radeon UI GPU tests: strict CPU equality, growing/empty visible bounds, source-sharing graphs, resize/DPI and byte accounting.
- Final bounds candidate passed all 3530 reference framebuffer/OAM hashes, final state and dispatch counts, private files unchanged. Five new-code A/B pairs improve 5.36-6.08%; median mean 8.229995 -> 7.774799 ms (-5.5309%), median p95/p99 10.003106/10.432033 -> 9.351037/9.847532 ms. UI payload remains 316299008 measured bytes; maps stay cached during this interval.
- World-map 240-frame correctness matches reference images/state/counts and two alternating diagnostic 1200-frame pairs have comparable means (candidate 3.026715/3.027036 ms, baseline 3.005388/3.059836 ms); private data unchanged.
- Native default GUI with profiling off and both FPS counters passed load, running at zoom 6, Undo load, reload/resume and window-close checks; inspected the saved battle/HUD and Undo-menu captures. Physical Radeon Vulkan/Wayland replay matched all 3530 reference image/OAM hashes, state/counts, unchanged private files and clean exit.
- Final Docker exports passed 332 Windows/Wine and 333 Linux tests (14 optional ignored each). Inspected archive integrity and privacy, bundled feature-doc identity, unchanged source hashes, PE resources/imports and Linux GLIBC 2.35 maximum requirement.
- Actual newly exported Linux portable passed cold-cache isolated GUI load, zoom-6 running, Undo load, reload/resume and WM close with profiling off and both FPS counters. Inspected battle/HUD and Undo captures. The initial preparation assertion was corrected for the archive empty cache-directory scaffold before launching; no runtime check failed.
- Updated local dist Docker Windows/Linux archives and BUILD-INFO after checks, preserving previous exports. Archive SHA256: Windows 183168d27a998ab0c8f8613d1e60cf878f297748e8d3447bdfe078cd881302cd; Linux 6062bfbd254161a47d28d7eabacb0d2e8b7b547bea53f302f9106fa2ac2eefc0. These are local artifacts, not a public release.
- project_memory.py render, check and check --base HEAD passed; git diff whitespace check passed with core.whitespace=cr-at-eol. Updated the TODO leaf and handoff as complete without closing the broader performance goal.

**Limitations**

- Timing scope is native Linux unpaced llvmpipe Vulkan with CPU profiling enabled, no realtime audio callback, GPU timestamps or per-frame logging. Five-pair correctness verification is separate from timing. No AI-turn speedup or native Windows device/timing coverage is claimed.
- Physical-controller, older-distro and AndroidPort checks remain outside this change. No guest state/ABI, save format or zoom/FOV change; the broader performance TODO remains open.

Documentation: [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md), [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md).

[Change record](docs/changes/2026-10-21-align-native-ui-sampling.json)

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

## 2026-10-20 — Validate and refresh desktop archives for the retained renderer optimizations

Type: docs. Platforms: Windows x86_64 GNU via Docker/Wine 10, Linux Bookworm export and Ubuntu Xvfb runtime.

**Changes**

- Rebuild the current profiling/window/background/empty-row renderer with the existing pinned desktop containers and cached dependencies. Export first to a fresh ignored validation directory, preserving old canonical archives.
- After successful validation, atomically replace local dist/docker-windows and dist/docker-linux archives and BUILD-INFO files. Preserve previous exports under the ignored validation directory; no remote release or publication is performed.
- Close current desktop build coverage in TASK_HANDOFF and select the distinct missing high-zoom workload characterization. Keep native Windows, physical controllers and hardware GPU timing unverified.

**Validation**

- Windows/MinGW container gates passed 317 tests: 171 core, 26 input/config, 33 menu, 49 runtime and 38 screen; nine optional cases ignored. Linux Bookworm gates passed 318: 171 core, 26 input/config, 33 menu, 48 runtime and 40 screen; nine optional cases ignored.
- Both archives passed integrity, path/member, required runtime/loader, executable-format, empty-Data-scaffold and private-data checks. Only the expected TinyCC DLL/shared library is shipped, with required source/ABI files and license notices; no ROM, save, recording, generated game library or test DLL is included.
- Windows PE resources are present and dependency sets equal the prior exports for runtime, launcher and TinyCC. Linux runtime maximum required GLIBC symbol version remains 2.35. Recorded renderer/Cargo/Docker source hashes stayed unchanged and archive hashes match BUILD-INFO.
- The exported Windows runtime under host Wine passed all 3530 per-frame framebuffer/OAM captures, complete final state and dispatch-count comparisons against both the previous Windows export and validated native Linux replay. Existing Windows translation and read-only fixtures were reused; this is correctness evidence, not native Windows performance timing.
- A fresh copy of the exported Linux package passed cold-cache launch, battle load, resumed play, Undo load, reload, resumed play and clean close under Xvfb/llvmpipe. Screenshots confirmed messages/preview/gameplay; detailed reports contain 1001 frames, 160160 scanlines and zero invalid intervals. Original ZIP hash and slot bytes stayed unchanged.

**Limitations**

- Native Windows/Desktop/PowerShell, hardware Radeon/GPU timing, physical controllers and older-distro/Steam Deck runtime compatibility are not established by these container/Wine/Xvfb checks.
- Packaged archives are local build outputs, not a published release. Correctness replay timing is not used as a new speedup claim; the originally reported slow turn remains uncharacterized.

Documentation: [TASK_HANDOFF.md](docs/development/TASK_HANDOFF.md), [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md), [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md).

[Change record](docs/changes/2026-10-20-validate-optimized-desktop-exports.json)

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

## 2026-10-20 — Summarize added desktop features in the GitHub README

Type: docs. Platforms: Desktop Windows and Linux documentation.

**Changes**

- Add seven concise bullets to the existing Features list for FPS counters, menu button combinations, nine previewed save-state slots, Undo, two startup save backups, Linux portable storage and Docker builds. Preserve all existing README lines and layout.

**Validation**

- Inspected the README diff: seven added feature lines, no removed or rewritten lines. Cross-checked FPS modes against docs/fps_overlay.md and desktop behavior against PROJECT_STATE.md and existing README sections.

**Limitations**

- Documentation-only change; no runtime behavior or compatibility claim changed.

Documentation: [fps_overlay.md](docs/fps_overlay.md), [menu_shortcuts.md](docs/menu_shortcuts.md), [save_states.md](docs/save_states.md), [save_backups.md](docs/save_backups.md), [portable.md](docs/portable.md), [docker.md](docs/docker.md).

[Change record](docs/changes/2026-10-20-summarize-desktop-features-in-readme.json)

## 2026-10-20 — Skip UI candidate work for rows with no UI sources

Type: performance. Platforms: Root desktop renderer; native Linux headless and isolated Xvfb/llvmpipe validation.

**Changes**

- After UI setup and portrait extraction, clear the destination and bypass world candidate sorting/blending only when native UI text/backing and portrait sources are empty. Preserve backing resize, anchor updates, capture-flag cleanup and hardware simulation.
- Keep the prior full pixel loop as a const-generic test reference; production compiles only the enabled shortcut. No persistent cache, guest timing or save-state layout is added.

**Validation**

- Release suites passed 171 core and 48 runtime tests, with 5/3 optional cases ignored. Empty-row/reference tests cover 48 panel/target/blend configurations with nonempty world BGs and semitransparent world objects, stale destination contents, exact transparent pixels, unaffected rows and complete state equality.
- The existing 48-configuration shared-background test now compares the shortcut with the previous full loop and seeds stale primary/extra/backing pixels. The preserved executable hash matches the accepted shared-background release.
- Native release build passed. Five alternating 3530-frame battle pairs after 120 warm-up frames passed every framebuffer/OAM, complete final-state and dispatch-count comparison. Medians 7.890765/6.194050 ms show 21.5025 percent less host work versus the accepted shared-BG release, with every pair favoring the candidate.
- UI capture means fell 4.466777/2.769615 ms. Core p95 9.480594/7.497194 ms and p99 9.905679/7.793264 ms improved in the separate profiles, with zero over-budget frames. The isolated maximum increased 10.770382/14.501617 ms; no universal rare-tail guarantee is claimed.
- Five world-map pairs passed framebuffer/OAM, final state and counts: medians 1.977584/1.942254 ms, zero over-budget frames in both 1080-frame profiles. These are scene-specific results, not native hardware GPU timing.
- An isolated copy with the new native runtime passed GUI load, 10-second resumed play, Undo load, reload, resumed play and clean close. Inspected loaded/Undo/reloaded gameplay screenshots. Detailed reports contain 987 frames, 157920 scanlines and zero invalid intervals; slot bytes and original ZIP hash were unchanged.
- Closed the empty-row work unit in TASK_HANDOFF and queued validation of current desktop/container builds, especially Windows, before further renderer optimization. Completed replay and baseline tasks remain closed.

**Limitations**

- The measured benefit is Linux headless host work, not an increase in the guest clock or proof of native Windows/Radeon/controller behavior. The original reported lag and rare maxima remain uncharacterized.
- Current portable Docker archives predate the retained renderer changes and were not regenerated in this work unit. No persistent state, translation ABI or save-state format changed.

Documentation: [TASK_HANDOFF.md](docs/development/TASK_HANDOFF.md), [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md).

[Change record](docs/changes/2026-10-20-skip-empty-ui-source-rows.json)

## 2026-10-20 — Share immutable native UI backgrounds within each scanline

Type: performance. Platforms: Root desktop renderer; native Linux headless and isolated Xvfb/llvmpipe validation.

**Changes**

- Prepare native BG2/BG3 once per captured scanline and share them between primary/extra and normal/backing passes. Keep backing text filtering local, preserve the Stats panel mask, and do not retain data across scanlines or guest writes.

**Validation**

- Release suites passed 170 core and 48 runtime tests, with 5/3 optional cases ignored. The strengthened reference fixture checks nonempty text/backing planes, BG-enable combinations, primary/extra and normal/action paths, Stats/Actions/Notice across all 160 scanlines in each of 48 configurations; complete final machine snapshots match fresh per-part preparation. The strengthened test and native release build passed.
- The pre-change binary was preserved and its hash matches the accepted row-window release, without rebuilding or recreating existing fixtures.
- Five alternating 3530-frame battle pairs after 120 warm-up frames passed all framebuffer/OAM, full final-state and dispatch-count checks. Baseline/candidate medians 8.244362/7.902728 ms show 4.14385 percent less host work versus the accepted row-window release; every pair favored the candidate.
- Detailed UI capture means fell 4.781399/4.476117 ms. Core p95 9.894629/9.486562 ms and p99 10.327550/9.908776 ms improved in these profiles, with zero over-budget frames. The isolated maximum increased 10.862195/12.277206 ms, so no universal rare-tail improvement is claimed.
- Five world-map pairs passed images, full final state and counts: medians 1.962043/1.950314 ms, a small difference rather than a separate world-map speedup claim. Both 1080-frame profiles had zero over-budget frames.
- The new native runtime passed isolated GUI load, 10-second resumed play, Undo load, reload, resumed play and normal close with detailed profiling enabled. Inspected load/Undo/gameplay screenshots; reports contain 989 frames, 158240 scanlines and zero invalid intervals. Original ZIP hash and GUI slot bytes remained unchanged.
- Updated TASK_HANDOFF to close prepared-BG reuse and select the distinct pending empty-UI-row shortcut, retaining completed baseline/fixture evidence and rejected experiments.

**Limitations**

- The measured benefit is scene-specific native Linux host work, not proof of native Windows/Radeon/controller behavior or resolution of the originally reported lag. Rare maxima still vary.
- No data persists across scanlines or guest writes; no guest clock, decision logic, save-state layout, translation ABI or GPU workload changed. Packaged release archives were not regenerated for this work unit.

Documentation: [TASK_HANDOFF.md](docs/development/TASK_HANDOFF.md), [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md).

[Change record](docs/changes/2026-10-20-share-native-ui-background-rows.json)

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

## 2026-10-20 — Use native paused previews when restoring states across different zoom strides

Type: fixed. Platforms: Desktop Linux and Windows.

**Changes**

- After load and Undo load, center the saved native framebuffer with cleared margins instead of interpreting the saved wide scratch buffer using the current zoom stride. Clear separated host UI caches until a completed guest frame rebuilds them and skip empty UI-layer layouts.
- Keep guest simulation, snapshot format, native ABI and translation-cache revision unchanged; preview generation does not advance the guest.

**Validation**

- Release runtime suite passed 49 tests (3 optional tests ignored). New test checks every preview pixel for four geometries including native and high zoom, stale scratch/margin clearing, and byte-identical machine snapshots before and after preview generation.
- Release Linux build passed. Isolated Xvfb/llvmpipe GUI load, six zoom steps, Undo load, reload, resume and clean close passed; screenshots inspected for removed fragmented rows and coherent resumed battle.
- Rebuilt Windows and Linux portable exports with Docker after the frontend change: 318 Windows/Wine and 319 Linux tests passed (9 optional ignored on each platform), including the new native-preview test. Archive integrity, private-data exclusion, bundled documentation equality, BUILD-INFO hashes, PE resources/dependency sets and Linux GLIBC 2.35 checks passed.
- A cold-cache copy of the actual new Linux export passed high-zoom load/Undo/reload/resume/clean close under Xvfb/llvmpipe, with inspected screenshots. Source/input hashes remained unchanged. Canonical local dist archives were replaced after validation; previous exports are preserved under target/desktop-perf/restore-view-exports/previous-exports.

**Limitations**

- Paused preview uses native 240x160 with black margins until the first completed frame rebuilds the extended view. Hardware GPU and native Windows GUI coverage remain unverified; Wine tests are not native Windows performance evidence. Local portable exports are not a published release.

Documentation: [save_states.md](docs/save_states.md).

[Change record](docs/changes/2026-10-20-restore-native-preview-across-zoom.json)

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

## 2026-10-20 — Record a deterministic battle replay with observed enemy activation

Type: docs. Platforms: Native Linux headless and isolated Xvfb/llvmpipe desktop.

**Changes**

- Derive an isolated in-game suspend save from the supplied battle snapshot, then resume in a new input-recording session. Save a checkpoint before player waits and rebase subsequent input events to replay the observed enemy activation and return to a player menu.
- Document the suspend-save recording procedure without weakening the existing prohibition on loading snapshots during recording. Keep all fixture bytes, recordings and results ignored under target.

**Validation**

- Original cartridge save and snapshot copies remain byte-equal to their ZIP entries; the original archive SHA-256 is unchanged. The new input recording sidecar equals the derived suspend save.
- A 3530-frame headless replay from the intermediate snapshot reaches the desktop endpoint guest frame and framebuffer hash; its final image was visually inspected. The first A/B verification matched framebuffer and OAM on every frame.
- The completed rerun passed five alternating native 3530-frame pairs with 120 warm-up frames: baseline/candidate medians 8.346306/8.290120 ms. Every verification framebuffer/OAM capture, final serialized machine state and dispatch count matched. Both executables are the same production baseline; the difference is measurement variation, not a code speedup.
- Separate 3410-frame profiles had zero frames over 16.742706 ms, p99 10.055213/10.128802 ms and maxima 12.534930/16.518197 ms. Hardware scanline means 6.072459/6.078200 ms account for about 73 percent of core time; core remainder means 2.247905/2.235117 ms, with no zoom rows or invalid captures.
- Recorded profiling overhead estimates versus uninstrumented medians were -0.2971/+0.2935 percent; these single-run estimates include noise and do not establish a negative profiling cost. Host checks during comparison showed CPU 15 busy and its SMT sibling CPU 14 idle.
- Git ignore checks confirm the derived suspend save, snapshot, recording and original ZIP remain excluded. The earlier interrupted comparison is explicitly failed with Broken pipe and retained separately; the rerun summary is passed.

**Limitations**

- The interval includes player waits, enemy movement, animations and idle frames, not isolated AI computation or proof of the originally reported slow turn. Native Windows and Radeon timings remain unverified.

Documentation: [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md), [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md).

[Change record](docs/changes/2026-10-20-record-observed-enemy-activation-replay.json)

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

## 2026-10-20 — Measure optional GPU render-pass time with bounded deferred readback

Type: added. Platforms: Desktop Linux and Windows.

**Changes**

- Add FFTA_GPU_TIMESTAMPS=1 alongside FFTA_PROFILE=1 to request supported render-pass timestamp queries. Default presentation requests no optional features and allocates no query resources. Retry device creation once without the optional feature if necessary.
- Reuse three asynchronous readback slots, nonblocking device polling and a 18000-submission cap; skip busy samples and stop diagnostic capture after map/poll failures without blocking or disabling normal rendering. Report GPU render-pass durations separately from CPU submission/readback phases and count incomplete/skipped/failed capture.

**Validation**

- Release Linux screen/runtime suites passed 41 and 50 tests (2 and 3 optional ignored); conversion/optional-feature selection and delayed-batch pending gauges passed.
- Explicit GPU test passed on physical AMD Radeon 780M, RADV Vulkan/Mesa 26.0.8: three-slot backpressure, bounded submissions, deferred completion and exact RGBA equality with/without timestamp queries.
- Windows GNU Docker build and Wine test suites passed 320 tests, with 10 optional cases ignored, including conversion and runtime report aggregation tests.
- New runtime GUI under Xvfb/llvmpipe Vulkan passed load, zoom 6, Undo load, reload, resumed play and clean close with inspected screenshots. 897 timestamp pairs submitted, 896 completed, 1 pending at nonblocking shutdown, zero failures/invalid/busy skips. Readback polling/decoding averaged 0.059444 ms (p99 0.124735 ms); render pass mean 1.250164 ms on this software adapter, not hardware GPU performance.
- Linux Docker export passed 321 tests (10 optional ignored). Both archive integrity/privacy checks, BUILD-INFO hashes, unchanged PE dependency sets/resources and Linux GLIBC 2.35 checks passed. Actual Linux portable export passed cold-cache load/Undo/zoom/reload/resume/clean close with both profiling variables absent; no diagnostic series or timestamp startup message was emitted. Source/input hashes remained unchanged and canonical local archives were refreshed, preserving prior exports.

**Limitations**

- Native Windows GUI/timing and controlled total GUI overhead remain unverified. Physical Radeon GUI under Xvfb failed surface configuration identically in the previous runtime baseline; physical Vulkan headless test passed. Software Vulkan GUI passed; those measurements are not Radeon render-pass performance.
- No per-scene/warm-up GPU labels yet; asynchronous aggregate samples include startup. Queued results at shutdown are reported as pending rather than awaited. Actual map/poll failure injection, unsupported-adapter GUI fallback and native Windows GPU paths remain unverified. Local archives are not a published release.

Documentation: [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md).

[Change record](docs/changes/2026-10-20-measure-deferred-gpu-render-pass-time.json)

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

## 2026-10-20 — Reconcile current user intent before executing a compacted task queue

Type: changed. Platforms: Project agent workflow.

**Changes**

- All three local skills and AGENTS now require reconciling the latest request before acting on a compacted summary; workflow-only requests preserve unrelated partial runtime work without executing it.
- Remove superseded live-process guidance from the handoff and explicitly preserve the unfinished restore-preview validation boundary.

**Validation**

- Read the existing terminal task evidence and current Git diff; reused the completed restore-preview runtime test log (49 passed, 3 ignored) without rerunning builds, fixtures or benchmarks.
- All three local skills passed quick_validate.py; git diff whitespace check passed.
- Ran project_memory.py check and check --base HEAD: both report the existing unfinished crates/recomp/src/play.rs change lacks its own memory record. This workflow record does not claim coverage of that runtime implementation.

**Limitations**

- Guidance is agent-maintained; no automatic compaction hook enforces task selection. Existing runtime preview changes remain unfinished and are not delivered by this workflow update.

Documentation: [MEMORY_WORKFLOW.md](docs/development/MEMORY_WORKFLOW.md).

[Change record](docs/changes/2026-10-20-honor-current-request-on-context-resume.json)

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

## 2026-10-20 — Identify and remove IDE npm discovery interference from local benchmarks

Type: docs. Platforms: Ubuntu Linux, VS Code local workspace and native replay validation.

**Changes**

- Document host CPU/SMT contention checks and distinguish deterministic correctness from reliable wall-time comparisons. Earlier battle timing samples collected under the saturated host are superseded for performance acceptance.
- Identify the stalled automatic workspace npm package scan and apply npm.autoDetect=off only in the ignored local workspace settings, preserving the existing color preferences. Cancel only that identified automatic scan after verifying its executable, arguments and workspace directory; no global editor settings or runtime binary are changed by this workaround.
- Record the development constraint for ignored build/Wine trees and link the measurement workflow from durable project state. Local editor preferences remain outside version control.

**Validation**

- A VS Code child ripgrep process had been scanning package manifests from the project root for over half an hour with --files --hidden --no-ignore --follow, consuming about 988 percent CPU. A two-second per-CPU sample showed most cores 59-95 percent busy alongside the pinned replay, with its SMT sibling about 77 percent busy. No source package.json files were found by a normal ignored-tree search.
- The installed VS Code npm extension manifest confirms autoDetect accepts off/on with resource scope. The existing workspace JSON was read and validated; only that property was added, and all prior settings were preserved. The exact identified scan then terminated; a follow-up process check found it absent and the replay SMT sibling idle.
- A subsequent baseline replay on the quieter host, using the same native binary, local snapshot, translation, audio setting and CPU affinity, completed five 1200-frame runs with a median of 10.776782 ms/frame. All framebuffer/OAM, full final-state and execution-count checks passed; this is an environment change, not a code speedup.
- The loaded-host and quiet-host baseline summaries match on all 1200-frame workload parameters, initial state and hashes of baseline/ROM/translation/input/snapshot. Their baseline medians are 18.655024 and 10.776782 ms/frame; the earlier 600-frame sample is not used for that comparison.

**Limitations**

- A quiet sample does not reserve a CPU or eliminate all background/thermal variation. Recheck load before comparisons; do not cancel unrelated user work.
- The editor setting is deliberately local and ignored; other checkouts need their own workspace preference if they encounter this issue. No claim is made that this scan caused the original user-reported lag in every scene.

Documentation: [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md), [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md).

[Change record](docs/changes/2026-10-20-exclude-ide-discovery-from-performance-baselines.json)

## 2026-10-20 — Keep standard UI layer sorting after evaluating two front-layer selectors

Type: docs. Platforms: Native Linux release replay and isolated Linux desktop fixtures.

**Changes**

- Evaluate simple and compact-key two-front-layer selection in CPU UI capture without changing blend/window rules. Restore the original production implementation after the measurements; neither prototype nor its extra reference test remains in the shipped core.
- Separate contended timing experiments from quiet-host comparisons after identifying the unrelated IDE npm scan. Preserve local experiment artifacts and retain original standard sorting until a material, reproducible overall benefit is shown.
- Prepare a second visually classified local world-map snapshot and input recording from a copy of the supplied cartridge save, keeping all fixture bytes ignored.

**Validation**

- Both prototypes passed an exhaustive reference comparison across 196608 valid priority/presence combinations, including BG ties, portrait/world OBJ exclusivity, empty layers and preserved payloads. Prototype suites passed 168 core and 47 runtime release tests serially; optional fixture cases stayed ignored.
- Quiet-host compact-key comparison passed five alternating 1200-frame pairs after 120 warm-up frames, with per-frame framebuffer/OAM, full final state and dispatch-count equality. Baseline/candidate medians 10.776782/10.953074 ms/frame show a 1.636 percent slowdown.
- Quiet-host simple-selector comparison passed the same five-pair and state/image/count checks. Baseline/candidate medians 10.822120/10.652935 ms/frame show only a 1.563 percent reduction. Separate 1080-frame profiles reported p95 11.337150/11.687671 ms and p99 12.501026/13.649431 ms, with zero/two frames over budget; these single captures do not establish better tail behavior or its causal source.
- The original PPU source was restored byte-for-byte to HEAD after inspecting that only the experimental changes were present. The source cartridge save and provided battle snapshot remain unchanged; the world-map fixture was saved through the isolated desktop menu after visual scene confirmation.
- The restored native release executable has exactly the preserved baseline SHA-256. The visually classified world-map snapshot passed five alternating same-binary 1200-frame pairs, full final-state and dispatch-count checks, and all framebuffer/OAM captures. Baseline/candidate medians 2.128969/2.139713 ms/frame calibrate ordinary variation, not a speedup; both 1080-frame profiles had zero over-budget frames and p95 2.348801/2.369600 ms.
- The existing exported Docker Linux portable bundle, using the original renderer, passed an isolated Xvfb/llvmpipe battle load, 10-second resumed play, Undo load, reload, resumed play and normal close. Screenshots confirmed both load messages and the Undo result; observed idle-battle overlays were 59.6/59.6 internal/external FPS and 59.0/59.0 shortly after reload. These are software-presentation observations, not a native Radeon or enemy-turn benchmark. The slot and original ZIP bytes were unchanged.
- Project-memory working-tree and --base HEAD checks passed, together with the whitespace check. No implementation file differs from HEAD after rejecting the experiments.

**Limitations**

- No new production PPU speedup is claimed. Initial loaded-host timing estimates were unreliable, and the modest quiet-host simple-selector mean did not establish an overall pacing improvement.
- The battle samples are idle-scene replays, not an identified enemy AI turn. Native Windows performance, hardware controller and actual GPU execution timing remain separate work.

Documentation: [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md), [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md).

[Change record](docs/changes/2026-10-20-evaluate-ui-front-layer-selection.json)

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

## 2026-10-20 — Characterize high-zoom workload before introducing desktop parallel rendering

Type: docs. Platforms: Native Linux headless.

**Changes**

- Reused the observed enemy replay at zoom 6 (570x322) for one ordinary and one detailed profile run; additional rows are not the dominant cost in this scenario. Keep parallel zoom and GPU policy unchanged.

**Validation**

- 3530-frame run with 120 warm-up frames: ordinary 7.623306 ms/frame, detailed 7.723037 ms/frame. Across 3410 profiled frames extra rows average 0.634410 ms (8.2161% of core), p99 total 9.389758 ms, max 10.063026 ms, no over-budget frames and no invalid stage samples.
- Ordinary/profile initial and final state hashes, framebuffer and execution counts match. GUI zoom and reload worked, but Undo revealed the separately tracked restored-preview stride bug; driver exit alone was not treated as visual success.

**Limitations**

- One workload characterization and overhead estimate, not a five-pair speedup result, isolated AI measurement, hardware GPU benchmark or coverage of all aspect ratios and CPUs.

Documentation: [PERFORMANCE_TODO.md](docs/development/PERFORMANCE_TODO.md).

[Change record](docs/changes/2026-10-20-characterize-high-zoom-replay.json)

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

## 2026-10-20 — Cache native UI window decisions for each extraction row

Type: performance. Platforms: Root desktop renderer; native Linux headless and isolated Xvfb/llvmpipe validation.

**Changes**

- Cache window registers and vertical membership once per native UI extraction row; preserve WIN0, WIN1, OBJ-window and outside precedence, inclusive/exclusive ranges and wrapping. Hoist native origin and modal flags without changing blend or sorting rules.

**Validation**

- The cached window helper matches the previous per-pixel reference on 2457600 coordinate/enable/mask/object-window combinations, including empty, wrapping and overlapping windows.
- Release suites passed 169 core and 48 runtime tests, with 5/3 optional cases ignored. Native release build passed. The preserved pre-change executable hash matches the previously validated stage-enabled release.
- The new-code verification passed framebuffer/OAM on all 3530 frames of the existing enemy replay, full final machine state and dispatch counts.
- Completed five alternating 3530-frame battle pairs with 120 warm-up frames: baseline/candidate medians 8.412148/8.229073 ms, 2.1763 percent lower work time; every pair favored the candidate. All framebuffer/OAM captures, final serialized state and dispatch counts matched.
- Detailed battle profiles attributed UI capture means 4.940586/4.779405 ms. Core p95 10.117031/9.889575 ms and p99 10.539597/10.346466 ms improved in these captures. Over-budget counts were 2/4, maxima 18.489253/18.634405 ms: retain the unfavorable rare-tail observation rather than claiming all spikes are fixed.
- World-map regression passed five 1200-frame pairs with 120 warm-up frames and full image/state/count equality. Medians 1.958091/1.966537 ms differ by +0.4313 percent; both 1080-frame profiles had zero over-budget frames. The candidate p95 increased from 2.002853 to 2.107436 ms in the single captures; no universal no-regression guarantee is claimed.
- An isolated copy with the new native runtime passed GUI load, 10-second resumed play, Undo load, reload, resumed play and normal close under Xvfb/llvmpipe. Inspected loaded/Undo/reloaded gameplay screenshots; detailed PPU reports contained 988 frames, 158080 scanlines and zero invalid intervals. Slot bytes and original archive hash were unchanged.
- The first GUI escalation timed out in automatic permission review before process creation; ordinary sandbox Xvfb then could not bind a socket. The single escalated retry was approved and completed successfully. Only that retry is counted as GUI validation.

**Limitations**

- The measured work-time benefit is modest and scene-specific; it does not establish native Windows/Radeon performance, physical-controller coverage or resolution of the originally reported lag.
- Rare-tail counts and the small world-map timing increase remain part of the evidence. No guest timing, decisions, save-state layout or GPU workload changed.

Documentation: [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md), [TASK_HANDOFF.md](docs/development/TASK_HANDOFF.md).

[Change record](docs/changes/2026-10-20-cache-ui-window-row-invariants.json)

## 2026-10-20 — Add bounded GUI replay workloads with warm-up exclusion and read-only cartridge behavior

Type: added. Platforms: Desktop Linux and Windows.

**Changes**

- Add explicit developer play --benchmark TOML configuration for existing snapshots and gba-input v1 recordings. Run one guest frame and GPU presentation per cycle with a bounded endpoint, named scene and warm-up gating for CPU and GPU profiling.
- Keep the normal interactive path intact. Benchmark mode ignores physical gameplay/menu input, disables the FPS HUD/audio callbacks/pacing, skips cartridge backups/autosave/exit save, and rejects incomplete/skipped presentations. Optional per-frame framebuffer/OAM verification and final state hashes are outside the cycle timer.
- Document the developer command in CLI help and bypass cartridge-path discovery/pickers during benchmark startup. Only formatting of the normal input-recording block changed after the measured binary was built; the measured GUI cycle body is unchanged.

**Validation**

- Release runtime tests passed 53 cases (3 optional ignored), covering strict config bounds/types/relative paths, warm-up and frame guards, machine immutability during measurement bookkeeping, and independent profiling feature versus measurement gates. Native release build passed.
- Existing 3530-frame enemy replay passed new GUI off/GPU verification: all native framebuffer/OAM hashes, final machine SHA-256 and dispatch counts agree. Both runs start at frame 8476, finish at 12006, retain 3410 measured cycles and leave private data/config files unchanged. GPU capture submits exactly 3410 pairs after warm-up, with 3409 completed and 1 pending, zero failures/invalid/busy skips.
- Five alternating off/CPU/GPU timing triplets passed on llvmpipe Vulkan with 3530 frames and 120 warm-up: median run means 8.316094/8.333123/8.423674 ms; observed CPU +0.204772%, CPU+GPU +1.293636% versus off. Paired GPU overhead ranges +0.468841% to +1.653155%. All 15 endpoints/full-state hashes and dispatch counts agree; binary/config/input/translation identities and no private-data writes verified. These are scoped unpaced software-Vulkan overhead estimates, not a game speedup or hardware-GPU benchmark.
- Final native source passed 53 release runtime tests and release build after startup/help polish. Physical AMD Radeon 780M/RADV Vulkan on native Wayland passed all 3530 per-frame hashes against the verified software-GUI run, identical final state/counts, private data unchanged and automatic clean close with timestamp queries active. This is functional cross-backend/teardown evidence; its verification timings are not an overhead comparison.
- Final Docker exports passed 323 Windows/Wine and 324 Linux tests (10 optional ignored each). Integrity/privacy, BUILD-INFO hashes, PE resources/unchanged imports and GLIBC 2.35 checks passed. Actual Linux archive passed cold-cache normal interactive load/Undo/zoom/reload/close with profiling off, plus the new command on 240 reference-verified frames with no private-data writes. Canonical local archives updated only after the full normal-GUI gate, with prior exports preserved; command smoke then passed as well.

**Limitations**

- Native Windows GUI/performance, realtime audio callback/pacing/compositor overhead and an older-binary disabled-path comparison remain unverified. Local portable outputs are not a published release.
- The measured binary predates startup save-discovery bypass, help text and equivalent input-block formatting polish. Measured source copies and hashes are preserved; no GUI cycle behavior changed in that polish. Older binaries lacking this mode cannot be used as equivalent disabled-path overhead baselines.

Documentation: [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md).

[Change record](docs/changes/2026-10-20-benchmark-bounded-gui-replays.json)

## 2026-10-20 — Resume from completed task evidence instead of repeating work after compaction

Type: changed. Platforms: Project agent workflow.

**Changes**

- Route session start, context-compaction recovery and automatic goal continuation through TASK_HANDOFF.md from AGENTS and all three local skills. Separate stable completed subtask IDs and rejected experiments from broad unchecked TODO parents.
- Seed the handoff with verified replay/baseline, tooling, caching and recovery substeps, rejected experiments, local artifact locators and the next unfinished PPU profiling criterion. Replace stale task-selection guidance without changing historical results.
- Require a concrete affected input, missing criterion, explicit request or observed failure before expensive repeats; inspect existing process/output state before restarting and keep missing local artifacts separate from completed outcomes. Maintain the handoff at work-unit boundaries and before known compaction.

**Validation**

- All three modified SKILL.md files passed the skill-creator quick validator. Checked 44 local Markdown links across the routing, skills and memory documents; all targets exist.
- Exercised the resumption rule by reading the existing completed benchmark summary: passed, five pairs, 3530 frames, full-state and framebuffer/OAM equality. No build, replay, fixture creation or benchmark was repeated for this workflow change.

**Limitations**

- This is agent-maintained routing and guidance, not a runtime compaction hook or automatic task-completion detector. The existing memory checker validates records and coverage, not agent decision quality or compliance.

Documentation: [TASK_HANDOFF.md](docs/development/TASK_HANDOFF.md), [MEMORY_WORKFLOW.md](docs/development/MEMORY_WORKFLOW.md), [AGENTS.md](AGENTS.md).

[Change record](docs/changes/2026-10-20-avoid-repeating-completed-work-after-compaction.json)

## 2026-10-20 — Attribute CPU scanline cost to disjoint optional profiling stages

Type: changed. Platforms: Root desktop runtime; native Linux validation.

**Changes**

- Add FFTA_PPU_STAGES opt-in stage attribution for BG, OBJ, composition, native UI capture and line-zero UI setup, with a residual that partitions hardware scanline time. Keep captures scoped, thread-local, bounded and outside serialized machine state.
- Read the detailed flag at profiler construction, preserve coarse-only profiling and avoid extra stage clock reads when disabled. Include relocated wide passes and keep nested UI preparation within its parent rather than double-counting it.
- Validate and retain optional scanline-stage rows in the existing comparison script, preserving compatibility with older baseline executables. Reuse the completed enemy replay to validate this new instrumentation.

**Validation**

- Release core/runtime suites passed: 168 core and 48 runtime tests, with 5/3 optional cases ignored. Detailed capture tests check full machine-state equality across forced blank and modes 0/1/2/3, stage containment, thread isolation and nested capture restoration.
- All 12 benchmark script tests passed, including new stage partition checks for missing/duplicate/nonfinite/inconsistent reports and rejection during uninstrumented timing. Native Linux release build passed.
- Completed five alternating 3530-frame pairs on the existing enemy replay with 120 warm-up frames. Every framebuffer/OAM capture, full final state and dispatch count matched the preserved baseline; summary passed and the process exited 0.
- Uninstrumented baseline/candidate medians were 8.289029/8.317234 ms, a 0.3403 percent increase in this comparison. Candidate detailed capture measured 8.373327 ms, an estimated 0.6744 percent overhead versus its own uninstrumented median. Both 3410-frame profiles had zero over-budget frames and no invalid intervals.
- Candidate stage means were BG 0.246104 ms, OBJ 0.287876 ms, composition 0.531136 ms, UI capture 4.911876 ms, UI setup 0.070950 ms and residual 0.078428 ms. These partition scanline time; UI capture accounts for about 80 percent of scanlines and 59 percent of core time. No rejected selector experiment was repeated.
- Marked the instrumentation subtask complete in TASK_HANDOFF, retained completed fixture/baseline decisions and selected a distinct pending UI row-invariant optimization. The comparison summary retains input/binary hashes and confirms the supplied inputs stayed unchanged.
- The later row-window candidate exercised the same detailed desktop profiler through an isolated Xvfb GUI load/Undo/reload/close check: 988 completed frames, all six stage reports and no invalid intervals. This checks desktop integration, not hardware GPU timing.

**Limitations**

- No rendering algorithm, guest timing or GPU workload is optimized by this diagnostic change. Native Windows and hardware GPU detailed reports remain unverified.
- Timing differences and profiling overhead are estimates from one native Linux comparison; no production speedup or universal overhead bound is claimed. Stage attribution covers scanlines, with zoom and actual GPU execution remaining separate.

Documentation: [PERFORMANCE_WORKFLOW.md](docs/development/PERFORMANCE_WORKFLOW.md), [TASK_HANDOFF.md](docs/development/TASK_HANDOFF.md).

[Change record](docs/changes/2026-10-20-attribute-cpu-scanline-stages.json)
