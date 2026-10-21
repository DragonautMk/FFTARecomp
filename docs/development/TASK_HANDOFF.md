# Task handoff

Updated: 2026-10-21. Latest user instructions take precedence. Follow
[resume/repeat decisions](MEMORY_WORKFLOW.md#resume-and-repeat-decisions).
An open parent never reopens completed children or rejected experiments.

## Current request checkpoint

- Latest request: continue resolving the desktop TODO. User clarified the audio
  report as **Fast audio at4x**, suspecting stale queued output.
- Selected **BUG-03-TRANSITION-RESET — complete / retained**. Queues already
  cleared on Fast transitions, but callback-owned DAC/filter history survived.
  The new host epoch invalidates that history without rebuilding tables; pause
  discards output while preserving the Fast latch, and device loss also discards
  Normal backlog. Load/Undo retains stream recreation. Guest state is unchanged.
- Evidence: [record](../changes/2026-10-21-reset-fast-audio-consumer-history.json),
  [behavior and limits](../fast_forward_audio.md). Before-fix synthetic residual
  peak0.19975911; after-fix20source/speed cases and28fresh-callback comparisons
  pass. Native68runtime/4ignored plus the added transition test pass; final
  Windows/Wine70runtime/4ignored pass. Native candidate builds; canonical binary
  restored by hash. Portable exports and AndroidPort unchanged.
- Actual Linux CPAL44.1kHz/Xvfb/llvmpipe idle-battle checks pass Faithful/Enhanced
  hold/release/toggle/pause/resume/WM-close, inspected captures and unchanged slot
  bytes. Volume0: callback/lifecycle evidence, not audible-quality validation.
  Old launcher helper assertion failed only after Faithful exited0; result was
  retained, helper fixed, only previously unrun Enhanced executed. Initial ALSA
  sysroot and Wine sandbox setup failures are documented, not runtime failures.
- **BUG-03-FAST-AUDIO remains partial.** Sustained nominal4x consumption exceeds
  local guest production: captures INT121.7/128.2 and EXT30.4/32.1;497/499Fast
  callbacks all hold, ~47%PSG/HLE requests empty, zero producer drops/over-period
  callbacks. Enhanced configured targets are affected too. Normal-playing
  intervals207/205callbacks have zero holds. This is separate from stale history
  and does not establish the user's hardware latency or an audio-event offset.
- Next open unit: **BUG-03-EFFECTIVE-RATE** — design and verify smoothed consumption
  against actual production/queue level when requested4x cannot be sustained,
  with explicit limits and suitable resampler cutoff; do not alter guest timing.
  Reuse this measured case and existing diagnostics. Normal backlog bound,
  native Windows audio and wider2x/8x/event coverage remain open; then BUG-02-FPS.
- No process live. Local artifacts: `target/fast-audio-reset/` (regression/build/
  native/Wine logs, candidate and restored-binary hashes, isolated GUI inputs,
  captures, `analysis.json`). Test/build handles24498/45705/6631/23587/24241/
  49418/96366/30121 are terminal. No commit, push or package update.

Previous requested first fix and history cleanup remain closed:
- **BUG-01-LAWS — implemented / verified for the observed layout.** Frame-based
  laws recognition keeps the table, HELP and indicators in the centered modal.
  Native183core/67runtime, Wine183core,72EUR reconstructions,24synthetic GPU/CPU
  cases and inspected Linux/Windows-Wine frontend checks pass. Wider coverage
  stays explicit in the [feature doc](../ffta_laws_layout.md) and TODO.
  [Record](../changes/2026-10-21-fix-battle-laws-modal.json); local `target/laws-fix/`.
- **BUG-01-IMAGE-HISTORY — complete / remote verified.** Image-bearing trees
  c890cf0->6eaecd5 and e262699->e459c65 rewritten with all other tree bytes and
  metadata preserved. Exact-lease push and remote tip verified; no branch/tag
  history contains the image. Local recovery checkpoints are not branch history.
- Pending performance work below remains unselected; no completed or rejected
  performance unit was reopened for this bug-fix request.

## Pending performance context (not selected by the current request)

- Previous performance request: continue executing the performance TODO. Prior progress:
  **PERF-07-TIMER-AUDIO-ORDER — complete/retained/delivered**; see evidence below.
- The18-ROM-seed trial remains rejected. Corrected event ordering makes its first
  211frames equal, but CPU/clock differ at212 and final IWRAM/HLE differ. No
  coverage timing/acceptance; its continuous-audio pair case did not run.
- Selected ID: **PERF-07-NATIVE-FRAME-BOUNDARY — in progress**. Previous
  interrupted goal turn made progress: confirmed delayed IRQ in existing snapshots,
  reproduced it with original synthetic code, implemented a guarded prototype.
- Done: source preimages preserved; entry cycle-budget guard and post-store/shell
  recheck, unchanged RtApi layout/save schema, cache revision7. Core180/5ignored,
  runtime66/4ignored, recomp-core3/4ignored and explicit11-case TinyCC regression
  pass. Isolated runtime18081, guarded pair32781 and pair helper66188 terminal/
  passed; checked the interrupted helper handle, no process restarted.
- Final hardened correctness is proven:181core/66runtime/3recompiler tests,
  explicit11-case TinyCC frame/IRQ/DMA regression and matching Windows/Wine
  suites. Pair22246 and CLI47161 terminal/passed: all3530states match at every
  frame, retained and continuous queues; full mixed/PSG/HLE/FIFO streams equal.
  Actual CLI matches all frame/OAM and final state; legacy images also equal.
- Prototype-v1 evidence remains closed and separate. Hardened protocol uses
  len0/budget-in-addr after the positive-length content-guard regression exposed
  an ambiguity; all final sources/libraries are under `native-frame-boundary/hardened/`.
- Direct-query correctness and timing stages are complete, unshipped. Triplets
 55899:legacy5.976894ms/guarded6.242606ms/extra6.056551ms; reference sources/binaries
 are preserved under `native-frame-boundary/hardened/`.
- Selected phase: **PERF-07-NATIVE-EVENT-ALLOWANCE — in progress**. Carry a
 conservative allowance across chained blocks, refresh when insufficient and
 invalidate/recheck after stores/shells. Query/yield-bit protocol uses cache8,
 no state or RtApi layout change. New artifacts under `allowance/`.
- Allowance correctness is complete:181core/66runtime/3recompiler tests,
  explicit11-case native/TinyCC and Windows/Wine suites. Pair7045/CLI27840 are
  terminal/passed: every3530-frame full state, complete audio (HLE compared by
  bits), images and dispatch counts match the direct-query reference. Old
  legacy images match too. Sources/binaries/protected inputs remain frozen.
- Allowance timing78865 terminal/passed: medians legacy5.850195/direct5.997167/
  allowance5.972798/extra5.797168ms. Hints win all five comparisons vs allowance
  (-2.9405% median); overall-vs-legacy gain is small (-0.9064%), not a broad
  speedup claim. Worldmap22807 passes240frame/full-state equality. Separate
  tails86913 complete: no over-budget samples, not repeatable tail-gain proof.
- Selected phase: **PERF-07-EXACT-ROM-HINTS — integration in progress**. Add the
  already-proven18 immutable entries only for the exact Europe hash, using one
  union helper in cache identity and compilation. Preserve user/database labels,
  keep USA/unknown images unchanged, and make hint growth change the digest.
- Next: run metadata/cache tests, build a new isolated runtime and prove default
  translation equals the validated explicit-hint library; then final replay,
  GUI and export gates. New integration artifacts under `integrated/`.
  Source preimages retained; earlier allowance/hardened/reference results remain
  closed evidence. Canonical native/dist still contain timer/audio correction.
- Evidence:[record](../changes/2026-10-21-preserve-native-event-boundaries.json).
  No process live before starting integration validation.

## Latest completed unit and current exports

**PERF-07-TIMER-AUDIO-ORDER — complete.** Event timestamps precede batch endpoint;
DMA steals extend CPU work, DMA clock service avoids video reentry, skip uses
same path. Five ROM-free regressions plus179native/Wine core and66runtime checks
pass (5/4optional ignored). Optional3000frame full-state/frame/OAM/audio observer
regression passes. Actual native/exported CLI preserve3530old frame/OAM/counts
and exactly match corrected final state. Old/corrected baseline final CPU/clock
match, only mixed audio/HLE/timer bookkeeping changes intentionally.
Five alternating pairs:5.756460->5.823067ms (+1.157%,0.066607ms), not a speedup.
Native and actual exported Linux GUI old-slot/zoom/Undo/reload/WM-close pass;
Vulkan/no fallback, inspected battle/Undo captures, cold cache rebuilt28859blocks.
No serialization/ABI/seed change; version1 slots remain valid. Broader native
Windows/physical hardware and the original rare AI stall remain unverified.

Docker349Windows/Wine and350Linux tests pass,15optional ignored per target.
Archive/privacy/docs/frozen-source/PE/imports/GLIBC2.35 gates pass. Local archives
published only after gates; previous versions preserved. Exact validated native
runtime/launcher installed into target/release (do not assume the former native
baseline is still there). Old native reference remains `timer-audio-order/recomp-baseline`.
No commit, push or public release.

Evidence:[record](../changes/2026-10-21-order-timer-audio-events.json).
Ignored artifacts: `target/desktop-perf/timer-audio-order/` (preimage/tests/native
builds/CLI/state components/five pairs/GUI), `timer-audio-order-exports/`
(frozen manifest,staged and inspected archives,compatibility,published hashes,
previous-exports), `timer-audio-order-export-gui/` (actual cold-cache bundle/logs/
captures). Missing local artifacts don't reopen completed work.

Current local exports:
- Windows `dist/docker-windows/FFTARecomp-windows-portable.zip`, SHA256
  `be4cc2fb96c5f379001c9944f4c6c927f2947d4de0fafa67fb3fd1bfae074f68`.
- Linux `dist/docker-linux/FFTARecomp-linux-portable.tar.gz`, SHA256
  `1d7cf99ef8b8734f00687f17e2c515ce1b9738cb3ba8e3c4d534a047b89024a3`.
- BUILD-INFO beside each archive; previous accepted archives under
  `target/desktop-perf/timer-audio-order-exports/previous-exports/`.

**PERF-10-PRESENTATION-CADENCE — previously completed, retained.** Observer,
clock/audio/VSync matrix, silent fallback, default-off/benchmark and lifecycle
checks already exist; no repeat required. It observes API presents, not scanout.
[Record](../changes/2026-10-21-observe-successful-presentation-cadence.json).
Artifacts:`presentation-cadence{,-gui,-no-audio,-exports,-export-gui}/`.

## Previous completed CPU triage and rejected compact IDs — reuse evidence

Current host sample: `target/desktop-perf/post-ppu-host-triage/` has isolated
symbols, source/input hashes, perf.data/self/children/annotate/triage and state/
count proof. Profiling build11285 and sample12711 terminal. Leaf samples don't
split missing/intentional fallbacks; no AI-stall/native Windows speed claim.

Compact-ID trial: native174core/5ignored,WindowsGNU/Wine174/5 and GUI/headless
all3530frame/OAM/state/count equality pass. Five headless pairs-1.34795%; single
instrumented core run slower. GUI five CPU-profile-on pairs-0.45386%,one slower;
five profile-off pairs-0.77360%,one slower. Max observations worse in both GUI
sets. Reject: no sufficiently robust useful GUI gain established, no causal
universal-regression claim. Exact baseline source and native runtime restored
by bytes/hashes; touched source mtime to force Cargo to invalidate stale trial
artifacts on the next build. Windows test artifact is still experimental in
ignored target/, not a canonical delivered binary. No additional tests/builds
needed merely to re-prove the unchanged baseline. No packages changed.

Evidence:[record](../changes/2026-10-21-evaluate-compact-ui-layer-ids.json) and
[workflow](PERFORMANCE_WORKFLOW.md#rejected-compact-native-ui-layer-ids).
Artifacts:`compact-ui-layer-id/` (baseline and evaluated candidate source/bin,
tests/builds/measurement-inputs/evaluation/decision.json), companion
`compact-ui-layer-id-gui-evaluation/` (verification,timing-summary and default-
timing-summary), under target/desktop-perf. Historical measurement manifests
name candidate files that have since been restored at root; decision.json and
preserved evaluated-candidate files distinguish them. Reuse outputs.
Terminal:core86946,native45178,headless54859,GUIverify47650,Windows98255/Wine30695,
GUIprofile49351/default33173. Worldmap/zoom/lifecycle/export gates intentionally
not run after rejection; do not treat these as pending work for that variant.

## Previous completed backend observation — reuse evidence

**PERF-09-BACKEND-OBSERVABILITY — completed for local PipeWire.** One continuous
observer; PID on Client, stream Node client.id, active links to physical sink,
start/end object.serial/state verified. Default piped pw-top buffered tables:
first GUI77632 passes but phase timing is invalid (paused rows same read time).
Preserved that negative result; only corrected backend collection repeated with
`stdbuf -oL`,59031 terminal/pass. Cadence <=1.001s gaps, running states/nonzero
rate/quantum. Idle10samples/~9s,paused3/~2s,resumed6/~5s: stream/sink ERR delta0.
Sink alreadyERR1 at first idle sample; creating-node zeros are not a running
baseline and no time/cause of that existing count is established. Shared graph
errors aren't dedicated hardware XRUNs or proof of game causality.

The unchanged actual Linux portable hash matches canonical export. App gate
8596:9196 complete235playing callbacks,0faithful holds/drops/over-period/device
errors. Backend phases are wall intervals, not exactly that guest-frame gate.
Idle/resumed screenshots inspected, CPAL/Vulkan/no fallback/exit0; slot unchanged.
No runtime/system audio change or rebuild. Native Windows/slow AI-scene/backend
hardware guarantees remain unverified. Do not repeat diagnostics to reconstruct
context or turn the result into a universal no-XRUN claim.

Evidence:[record](../changes/2026-10-21-characterize-pipewire-backend-errors.json).
Local/ignored `target/desktop-perf/audio-backend-observability/` is the buffered
trial; `audio-backend-observability-line/` contains accepted top.jsonl/phases,
active/end dumps/PIDs/analyze.py/result.json/source binary hashes, GUI logs and
idle/resumed captures. Read-only initial inventory also retained. pw-dump needs
approved host access here; sandbox cannot connect. All processes cleaned up.

Current local exports are unchanged from the enhanced-source unit below; no
public release or redundant rebuild was performed.

## Previous enhanced-source unit — exports superseded, reuse evidence

**PERF-09-ENHANCED-SOURCE-EVENTS — completed.** Fixed per-context PSG/HLE stereo
pair and FIFO event requests/empty counts, plus configured-target totals. Warm/
unrouted consumers remain in raw totals. Target is configuration, not audibility,
voice activity, crossfade weight or a defect. No queue/FP/DAC/crossfade policy,
guest/save/cache ABI change. No new callback lock/IO/log/allocation; counters
compile out of the default DSP loop, one dispatch per callback selects variant.

Native63runtime/4ignored pass: exact sample/queue/counter/interpolator parity,
partial stereo pairs, DAC holds, routing/HLE/backlog/mute, bounded/gated totals.
Five synthetic observer off/on pairs:40666.215/40696.723ns per512-frame callback
(+0.0750%), not hardware/game/default-before-after timing. Idle Polly battle
window8596:9196 after restored8476:236callbacks each mode, no empty source reads.
Enhanced targets PSG/HLE; FIFO stays warm, faithful has no enhanced target.
Captures inspected, CPAL/Vulkan, no fallback. Driver initially shadowed keyboard
helper with source-loop variable after faithful passed; preserved first result,
fixed only harness and measured only previously unmeasured enhanced. No runtime
failure or repeated faithful run.

Docker341Wine/342Linux pass,15optionalignored each. Source/doc/archive/privacy/
resources/DLL-set/GLIBC2.35 pass. Actual Linux archive cold-cache load/zoom/Undo/
reload/resume/WM-close passes, inspected battle/FPS/Undo, Vulkan/no fallback/exit0.
Source counts reconcile. Lifecycle playing has2741PSG/HLE empty-pair requests,
660perFIFO and2741faithful holds in one callback; enhanced target counts0 in
faithful mode. Keep separate from idle window, no audible-defect inference.
Default-off has no diagnostic rows/ignores invalid gate; callback-free benchmark
ignores gate, matches240reference frames and preserves private Data. Local dist
updated only after checks; previous archives preserved, no public release.

Evidence:[record](../changes/2026-10-21-observe-enhanced-audio-source-requests.json).
Artifacts under `target/desktop-perf/`: `audio-source-events/` (source preimages,
focused diff/tests/overhead/build/measurement hashes), `audio-source-events-gui/`
(result/captures/original harness-error and resume logs), `audio-source-events-exports/` (frozen manifest/inspection/compatibility/published-artifacts/old exports),
`audio-source-events-export-gui/` (actual archive result/captures/profile/default/
benchmark logs). Terminal sessions:67091/92019/61539 tests,47581 native build,
48191 initial GUI,14067 enhanced completion,24533/31303 Docker,57172 archiveGUI.

Previous local exports (superseded; reuse evidence):
- Windows `dist/docker-windows/FFTARecomp-windows-portable.zip`, SHA256
  `8cc8466020c332876fec5c644facf9a7d4dfe85f50c4ff1435ece354f0f47b93`.
- Linux `dist/docker-linux/FFTARecomp-linux-portable.tar.gz`, SHA256
  `d348913aa3d0b22396fe33befc6f934efac09662f3b2cf7278ae3f89170d2fbc`.
- BUILD-INFO beside archives; previous versions under
  `target/desktop-perf/audio-source-events-exports/previous-exports/`.

## Closed units — reuse evidence

- **PERF-09-INTERACTIVE-GATING — completed.** One producer-frame traversal,
  load/Undo closure, context separation, idle-pair and archive/default/benchmark
  checks done. [Record](../changes/2026-10-21-gate-realtime-audio-by-guest-frame.json).
  Artifacts:`audio-scene-gate{,-gui,-exports,-export-gui}/`. Old exports superseded.
- **PERF-08-UI-WORLD-ONLY-PIXELS — retained.** Default-write world-only pixels
  without sort/blend;174native/Wine core checks,3530frame/state/count parity,
  headless-4.20%,GUI-2.67%,worldmap/zoom6/defaultGUI and final exports passed.
  [Record](../changes/2026-10-21-evaluate-world-only-ui-pixels.json).
  Artifacts:`ui-world-only-pixels{,-gui-evaluation,-gui,-exports,-export-gui}/`.
- **PERF-08-UI-PANEL-ROW-CLASSIFICATION — retained.** Lazy per-pixel bounds and
  row rules preserve missing-bounds/banner precedence;173native/Wine core checks,
  full replay/zoom6 parity,headless-2.69%,GUI-2.26%,worldmap/defaultGUI/exports pass.
  [Record](../changes/2026-10-21-evaluate-ui-panel-row-classification.json).
  Artifacts:`ui-panel-row{,-gui-evaluation,-gui,-exports,-export-gui}/`.

- **PERF-07-FALLBACK-CANDIDATES — triage complete.** New enemy census preserves
 3530frames/state/counts;22.63M native/79.14M missing/55.91M intentional hooks
 after warmup. One pre-panel-cache optimized perf sample3928/zero lost identifies
 capture_dialogue_part31.52%self. Guest counts aren't host time, callchains don't
 fully split missing/intentional work. No lookup/translation change accepted.
 Profile binary isolated, original native restored. [Record](../changes/2026-10-21-characterize-enemy-fallback-host-cost.json).
 Artifacts:`enemy-fallback-census/`; do not rerun the completed census/sample.

- **PERF-04-DEVICE-LOSS — controlled invalidation fixed/complete.** Actual
  destroy+new UI panicked on invalid texture. Owned Device callbacks now latch
  first error in Arc/OnceLock; return fallback and suppress subsequent UI/VSync.
  Scoped errors stay local; no per-frame allocation/wait/fault API. Native58/46,
  explicit four-case native/Wine tests, normal GUI and3530frame Radeon Wayland
  verification pass. Current exports333Wine/334Linux,15ignored; actual Linux GUI
  cold-cache and archive gates pass. [Record](../changes/2026-10-21-handle-owned-gpu-device-failures.json).
  Artifacts:`device-loss/`, `device-loss-{gui,wayland,exports,export-gui}/`.
  Does not prove all asynchronous hardware resets/native Windows faults.

- **PERF-04-MONITOR-TRANSITIONS — complete for virtual X11/Wine outputs.**
  --monitors diagnostic requires two actual enumerated handles/current_monitor,
  A/B/A/B/A, resize/VSync and teardown. Native/Wine unequal1280x720/1280x960
  runs20Presented/0Skipped, actual640x360/640x480 changes and independent positions.
  Actual Linux portable battle+Undo/reload/close passes after explicit evdev US
  keyboard setup; initial intro-only flow is labeled unverified, not a battle pass.
  No physical GPU/DPI/color/Wayland/native Windows claim. [Record](../changes/2026-10-21-validate-virtual-monitor-transitions.json).
  Artifacts:`monitor-transitions/`, `monitor-transitions-{wine,game}/`.

- **PERF-04-MINIMIZE-RESTORE — complete for native Linux X11/Wine Vulkan.**
  Original no-ROM default mode:3observed WM cycles, three resize/VSync tests,
  clean teardown. Native27Presented/0Skipped; Wine18Presented/9Skipped. xprop
  independently confirms3Iconic/4Normal. Actual Linux portable battle then
  Undo/reload/WM-close passes. First Wine D3D12 device creation failed and remains
  negative evidence; successful retry only disables that prefix's D3D12 DLLs.
  [Record](../changes/2026-10-21-validate-window-minimize-restore.json).
  Artifacts:`minimize-restore/`, `minimize-restore-{game,wine}/`.

- **PERF-04-SKIPPED-UPLOADS — completed.** Track pending Queue writes across
  UI updates, flush transfer-only on skips, avoid frontend zero-size uploads/
  software surfaces, retain benchmark skip errors. No guest/audio change.
  Native58/46 tests,1200zero-size batches/492MB logical bytes with stable RSS,
  GUI and actual archive GUI pass. Current exports333Wine/334Linux,14ignored;
  privacy/doc/source/PE/GLIBC2.35 checks pass. Initial pre-guard builds superseded.
  [Record](../changes/2026-10-21-flush-skipped-presentation-uploads.json).
  Artifacts:`skipped-uploads/`, `skipped-uploads-{gui,exports,export-gui}/`.

- **PERF-03-FRACTIONAL-UI-TEXELS — retained/completed.** Cached CPU-defined
  RG32Uint sampling maps and conservative bounds/scissors preserve original
  zoom/state. Radeon32-case resize/zoom/DPI and all fractional comparisons match
  CPU; replay3530/state/counts, worldmap, GUI and Wayland pass. Table-only trial
  +4.1681% rejected; bounds candidate five pairs -5.5309% (8.229995->7.774799ms),
  lower p95/p99. Native unpaced llvmpipe/CPU-profile scope, not AI/native Windows.
  Previous Docker332Wine/333Linux,14optionalignored and actual archive GUI passed.
  [Record](../changes/2026-10-21-align-native-ui-sampling.json).
  Artifacts: `ui-coordinates-{evaluation,exports,gui,wayland,export-gui}/`.

- **PERF-05-SHARED-WORLD-UPLOADS — retained/completed.** Same-call immutable
  source extent/dimensions share upload; divergent sources split before writes.
  Five pairs -0.3929% work, -40.37% UI payload, exact independent-GPU equality.
  Previous exports329 Wine/330 Linux and GUI/Wayland pass. Fractional CPU gap
  subsequently closed above. [Record](../changes/2026-10-21-evaluate-shared-world-uploads.json).

- **PERF-02-AUDIO-CALLBACK-DIAGNOSTICS — done for faithful/current scope.**
  Bounded context-separated observer and callback parity/overhead tests exist.
  Final Linux p99 total 0.486576 ms, mutex 0.000972 ms; no over-period callbacks
  or producer drops; transition-inclusive holds are not isolated AI underruns.
  [Record](../changes/2026-10-21-profile-realtime-audio-callbacks.json).
- **PERF-05-NATIVE-UI-INPUT-CACHE — retained; PERF-06 complete.** Full input
  keys skip packing; five pairs -1.41%; GUI/style/restore/exports checked.
  [Record](../changes/2026-10-21-evaluate-ui-packing-input-cache.json).
- **PERF-05-LUT-FPS / PERF-06-FIFO-PACING — done.** Revisions/LUT content
  comparison, FIFO capacity and startup pacing retained.
  [Record](../changes/2026-10-20-reuse-desktop-uploads-and-audio-buffers.json).
- **PERF-02-GPU-EXECUTION-TIMESTAMPS — done for implementation/current coverage.**
  Three bounded asynchronous readbacks, no wait; physical Radeon/GUI checks pass.
  [Record](../changes/2026-10-20-measure-deferred-gpu-render-pass-time.json).
- **PERF-02-GUI-CONTROLLED-OVERHEAD — done for unpaced workload.** Benchmark
  command, 3530-frame verification and five off/CPU/GPU triplets exist; reuse.
  [Record](../changes/2026-10-20-benchmark-bounded-gui-replays.json).
- **PERF-02-COARSE-PROFILE / PERF-02-PPU-STAGES — done for native Linux.**
  CPU/frontend and detailed PPU attribution already implemented and calibrated.
  [Record](../changes/2026-10-20-attribute-cpu-scanline-stages.json).
- **PERF-01-REPLAY-TOOLS — done.** Native/Wine snapshot/input/state tools exist.
  [Record](../changes/2026-10-20-resume-headless-benchmarks-from-snapshots.json).
- **PERF-01-WORLDMAP — done.** Visually classified fixture and five-pair baseline.
  [Record](../changes/2026-10-20-evaluate-ui-front-layer-selection.json).
- **PERF-01-ENEMY-REPLAY / PERF-01-ENEMY-BASELINE — done.** Observed enemy action,
  3530-frame recording, five baseline pairs and stage profiles are complete.
  [Record](../changes/2026-10-20-record-observed-enemy-activation-replay.json).
- **PERF-08-UI-ROW-INVARIANTS — retained (-2.176% at that boundary).**
  [Record](../changes/2026-10-20-cache-ui-window-row-invariants.json).
- **PERF-08-UI-PREPARED-BG-ROWS — retained (-4.144% at that boundary).**
  [Record](../changes/2026-10-20-share-native-ui-background-rows.json).
- **PERF-08-UI-EMPTY-ROWS — retained (-21.503% at that boundary).**
  [Record](../changes/2026-10-20-skip-empty-ui-source-rows.json).
- **PERF-01-HIGH-ZOOM-WORKLOAD — characterized; no parallel policy justified.**
  [Record](../changes/2026-10-20-characterize-high-zoom-replay.json).
- **PERF-03-RESTORE-VIEW-STRIDE — fixed.** Native paused preview/Undo never
  interprets old scratch stride; cache clearing and current exports pass.
  [Record](../changes/2026-10-20-restore-native-preview-across-zoom.json).
- **PERF-03-CURRENT-DESKTOP-BUILDS — done for stated container/Wine scope.**
  Old archives are superseded by current validated exports above.
  [Record](../changes/2026-10-20-validate-optimized-desktop-exports.json).
- **PERF-04-RECOVERY-IMPLEMENTATION — done; wider lifecycle matrix open.**
  [Record](../changes/2026-10-20-recover-transient-desktop-surfaces.json).
- **PERF-04-XVFB-GL-INITIAL-SURFACE — fixed/current coverage done.** Balanced
  configure scopes/fallback, valid resize/VSync and native close checked.
  [Record](../changes/2026-10-21-handle-surface-configuration-errors.json).
- **DOC-README-FEATURES — done.** Seven concise feature lines, layout preserved.
  [Record](../changes/2026-10-20-summarize-desktop-features-in-readme.json).
- **WORKFLOW-RESUME / WORKFLOW-RESUME-CHECKPOINT — done.** Latest request and
  selected ID/outcome must be reconciled before acting on old summaries.
  [Record](../changes/2026-10-21-require-resume-selection-checkpoint.json).

## Rejected units — keep the decisions

- **PERF-07-IMMUTABLE-ROM-SEEDS — rejected.** Existing18-seed translation
  fails full state, initially audio and after event fix CPU/clock at212. Source
  library pair retained; no timing or acceptance. Follow the new boundary
  criterion rather than repeating this trial unchanged.
  [Original trial](../changes/2026-10-21-evaluate-immutable-rom-entry-seeds.json),
  [first diagnosis](../changes/2026-10-21-isolate-rom-coverage-audio-divergence.json),
  [corrected event ordering](../changes/2026-10-21-order-timer-audio-events.json).

- **PERF-10-POST-WAIT-COMPOSE — rejected/restored; no validation pending.**
  Five alternating realtime pairs show worse p99 in four of five; exact accepted
  source/runtime restored, portable exports unchanged. Do not rerun the trial
  because its former next action survived in an older handoff or summary.
  [Record](../changes/2026-10-21-evaluate-composition-before-deadline.json).
  Artifacts: `target/desktop-perf/pre-wait-compose{,-gui}/`; jobs terminal.

- **PERF-08-COMPACT-UI-LAYER-ID — rejected/restored.** Sub-percent GUI gain
  inconsistent across pairs, contrary maxima; exact baseline source/bin restored.
  [Record](../changes/2026-10-21-evaluate-compact-ui-layer-ids.json).

- **PERF-05-NATIVE-UI-REVISIONS — rejected and restored.** Output-byte double
  buffers reduced payload but worsened total work +0.131%; packing-input cache
  is a different accepted design. Do not reactivate output-byte comparison.
  [Record](../changes/2026-10-21-evaluate-native-ui-upload-revisions.json).
- **PERF-06-ROW-BOUNDS — rejected for production.** Synthetic benefit did not
  survive end-to-end replay; experiment remains test-only.
  [Workflow](PERFORMANCE_WORKFLOW.md#panel-bound-scan-diagnostic).
- **PERF-08-FRONT-TWO — rejected, standard sorting restored.** No sufficient
  overall benefit; an open PERF-08 parent does not reopen this experiment.
  [Record](../changes/2026-10-20-evaluate-ui-front-layer-selection.json).

## Artifact locators and remaining scope

All paths below are local/ignored; missing artifacts do not reverse closed tasks.
Read tracked results first and restore only what the next new criterion needs.

- Current completed unit artifacts are listed above. Earlier shared-world
  verification/timing/world-map/resource manifests and GUI/export evidence remain
  in `target/desktop-perf/shared-world-{baseline,evaluation,gui,wayland,export-gui,exports}/`.
  Do not rerun these finished checks simply because they are no longer at the top.
- Common fixtures: `ai-turn-replay/{start.ffstate,input.input}` (3530 frames),
  `user-fixture/worldmap.ffstate`, `translation.so` under target/desktop-perf/.
  Never commit/package fixture bytes or regenerate their recording to resume.
- Reference: `gui-controlled/{verification.json,off-verify.log}`. Its FNV frame/
  OAM transcript differs in format from headless SHA logs; compare like with like.
- Earlier evidence: `audio-callback-{gui,export-gui,exports}/`,
  `ui-input-cache-evaluation/`, `surface-config-exports/`, `gui-controlled/`.
- Use `surface-config-vsync-gui/` for actual ON/OFF. Earlier valid-gui Return-only
  captures both show OFF. Old high-zoom-gui Undo capture shows a fixed defect;
  use restore/current-export captures as passing evidence.
- Native Windows GUI/device/GPU, physical controllers, full fault/lifecycle/
  monitor matrix, interactive scene/warm-up gating, enhanced starvation/backend
  XRUNs and original reported slow-turn attribution remain separate criteria.
  Callback-free unpaced runs do not establish realtime overhead. See
  [PERFORMANCE_TODO.md](PERFORMANCE_TODO.md) for the full remaining objective.
