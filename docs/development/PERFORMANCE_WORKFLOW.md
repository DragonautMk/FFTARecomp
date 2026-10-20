# Desktop performance measurement workflow

Start from the [performance TODO](PERFORMANCE_TODO.md). Keep the baseline binary,
its TinyCC loader and a single ROM translation. Use a release build and separate
verification from work timing. ROMs, save copies, recordings and result logs stay
under ignored local directories. The benchmark never writes the supplied save.

## Host load before timing

Check background CPU load and the pinned CPU's SMT sibling before accepting a
comparison. Affinity does not reserve a core; unrelated work can change cache,
boost and available execution resources without changing replay results. Retain
contaminated logs as correctness evidence and rerun timing after resolving the
interference. A passed deterministic comparison alone is not a performance gain.

In this Rust workspace, VS Code's automatic npm task discovery was observed
running `rg --files --hidden --no-ignore --follow` for package manifests for over
half an hour, using roughly ten CPU cores. Ignored build trees and Wine prefixes
can still be traversed by that search. No npm manifests were found in the source
tree. A local workspace `"npm.autoDetect": "off"` setting stops unnecessary npm task
discovery; it does not disable manual npm commands or change other projects.
Preserve existing editor settings. `.vscode/` stays ignored, and no global editor
configuration is required. Identify the specific stalled automatic scan before
cancelling it; leave unrelated interactive work alone. Check that it does not
restart before remeasuring. The observed background scan invalidates using the
earlier loaded-host battle timings as an uncontended code-performance baseline.

## Personal local fixtures

The optional root `saves.zip` and extracted `saves/` directory are ignored,
as are individual `.sav` and `.ffstate` files. Inspect archive paths before
copying selected entries into an isolated portable bundle under `target/`;
do not launch against the originals, since normal play persists cartridge
memory on exit. Keep the matching ROM and all fixture bytes local. The Docker
context uses an allowlist and excludes saves; portable archives also exclude
personal data.

The supplied slot has passed actual desktop loading and visual scene inspection;
the cartridge save has also reached the world map through Saved Game > Load.
After loading a slot, leave the Save states page, select Resume explicitly,
and verify that the game advances without the host pause menu. A successful
load message alone is insufficient for a gameplay measurement. Failed keyboard
automation on desktop Xwayland was discarded; isolated Xvfb input succeeded.
Xvfb/llvmpipe uses CPU rendering and shares host resources with emulation:
label those captures separately from Radeon/Wayland results. Current interactive
profiles include startup and navigation, so a scene screenshot does not make
the whole-session distribution a clean scene-only benchmark.

### Record a battle from a supplied snapshot

Input recording refuses host snapshot loads because its event timeline would
otherwise rewind. When the original cartridge save has no resumable battle,
load the supplied snapshot in an isolated, non-recording session and use the
game's own battle Save command to create a suspend save. Close cleanly and keep
a separate copy of that derived `.sav`. Start a new session with
`RECOMP_RECORD_INPUT` and resume the battle through Saved Game > Resume Battle.
This uses the existing recorder without weakening its load protection.

While recording, save a host snapshot before the target actions. The headless
runner's `--state ... --frames 0` reports its guest-frame origin. For replay
from that snapshot, retain subsequent recorded events and subtract that origin
from their frame numbers; carry the held key mask at the origin into frame zero
if needed. Keep the original recording and its `.input.sav` sidecar intact.
Save another snapshot after the observed enemy action, inspect the scene, and
check the replay's end frame and framebuffer hash against that endpoint before
timing. Do not classify the whole interval as AI computation: it also contains
player inputs, animations and idle frames. A matched image alone does not prove
complete machine or audio equality; retain the separate A/B state checks.

## Headless comparison

The historical script name now supports native Linux/macOS and Windows/Wine;
it detects the executable format and checks it against the host/runner. Existing
Windows commands remain supported. For example, on Linux from the repository root:

```bash
python3 scripts/benchmark_windows_runtime.py \
  --baseline target/perf-baseline/recomp \
  --candidate target/release/recomp \
  --rom /path/to/ffta.gba --library /path/to/translation.so \
  --input /path/to/battle.input --sav /path/to/save-copy.sav \
  --frames 6000 --warmup 120 --runs 5 \
  --scenario enemy-turn --out target/perf/enemy-turn
```

`--demo` instead of `--input` generates the existing introductory Start/A sequence;
it does not identify an AI workload. `--sav` preloads cartridge progress, so its
recording still navigates from boot. Alternatively, `--state /path/to/slot-1.ffstate`
resumes a desktop slot with embedded preview or a legacy raw snapshot through
the menu's transactional version/identity/checksum validator. Both runtimes must
support this flag. `--sav` and `--state` are mutually exclusive; a snapshot needs
an explicit `--input`, rather than the introductory `--demo`.

For snapshot runs, `--frames N` advances N additional guest frames, warm-up and
input events start at zero at the loaded point, and zoom schedules also use that
relative timeline. The guest's frame counter, clock, RNG and hardware deadlines
retain their original values. Without a snapshot the origin is boot frame zero,
which preserves existing scripts. `RECOMP_COST_FROM`, character diagnostics and
other legacy guest-frame diagnostics retain absolute frame numbers.
An idle input file is simply `gba-input v1` followed by `0 3ff` on the next line;
it exercises the loaded scene but does not prove enemy AI coverage.
Add `--zoom '0:0,1000:4'` to compare a fixed zoom schedule.

The runner never writes the supplied snapshot, and rejects `--out` pointing to
that same existing path or symlink. The comparison script protects snapshot
inputs from output collisions, records their file hashes, checks the loaded
machine identity and guest-frame origin on every run, and verifies exact frame
ranges relative to that origin. `summary.json` includes `start_frame` and
`initial_state` in addition to the requested replay length. Snapshot loading and
its identity hash precede timing; ordinary frame work remains unchanged.

For Windows under Wine add `--wine wine --wine-prefix target/perf/wine` and use
Windows executables with their TinyCC DLLs and a Windows-compatible translation.
Native Windows needs no runner. Results distinguish native Linux/Windows/macOS
from Windows via Wine. Use the same toolchain to build both binaries when isolating
small code changes; a previously packaged binary can also serve as a regression
baseline, but build-environment differences remain a confounding factor.

Each comparison:

1. Captures framebuffer/OAM hashes for every frame in two verification runs.
2. Checks the warm-up scene fingerprint, interval, dimensions and final image.
3. Alternates baseline/candidate timing order for five pairs by default.
4. Checks final results again on every timing run, without per-frame hashing.
5. Publishes `summary.json` with individual samples, median, platform, workload
   label and SHA-256 of the binaries, translation, ROM, input and supplied save.

`--verify-state` additionally requires both runtimes to support
`FFTA_BENCH_STATE=1`. It hashes the complete serialized machine snapshot after
the measured interval, including CPU, hardware and cartridge state. It is not a
per-frame proof of all memory transitions or a full-stream audio comparison.
An older executable without this diagnostic is rejected when the flag is set.

`--require-execution-counts` also compares native-block/fallback-step counts.
Leave it off when a deliberate translation change alters dispatch granularity;
counts are not a measure of CPU time or guest-state equality.

The script rejects incomplete/duplicate captures, mismatched warm-up scenes,
nonfinite timings, changed inputs and input/output path collisions. Processes
have a configurable timeout (`--timeout`, default 300 seconds per run).
Once execution starts, `summary.json` explicitly records running/failed status,
so an execution failure cannot leave an old successful result in place. Timing and verification logs remain
available for inspection; do not interpret a run as passed without checking status.

## Guest-cycle cost census

`RECOMP_COST_FROM=N` enables the existing per-PC guest-cycle histogram from
frame N in `runc`. It now observes the same dispatch loop as normal execution,
including sprite/battle mouse interpreter gates, BIOS/interrupt handling, audio
HLE hooks, strict no-interpreter mode and fallback-entry tracing. The default
runner uses a no-op observer specialization; it does not build a cost histogram.

`COST_DISPATCH` reports native blocks (`native`), missing translations (`missing`),
interrupt/sleep/IRQ-return steps (`system`) and intentionally interpreted sprite
or mouse steps (`hook`). The last two categories are separate from the existing
native/missing totals. `COST` retains its legacy PC aggregation, without adding
the Thumb mode bit, and reports guest cycles rather than host CPU time.
A hot guest PC is a triage lead, not proof of a host bottleneck. The census adds
per-dispatch accounting overhead; keep it out of timing comparisons. The A/B
script explicitly clears `RECOMP_COST_FROM` from child environments.

For a local ROM/native-translation fixture, the optional test
`native_cost_observer_matches_game_frames_audio_and_full_state` runs the same
3000-frame demo with and without dispatch observation and scoped PPU timing.
It compares a full-stream digest
of native/wide framebuffer, OAM, mixed/PSG/FIFO/HLE audio, final serialized machine
state and dispatch totals; it also checks that attributed cycles sum to the
actual guest-clock advance. Set `FFTA_STATE_TEST_ROM`, `FFTA_STATE_TEST_NATIVE`
and, when required, `FFTA_TCC_LIBRARY`, then run:

```sh
cargo test --locked --release -p recomp native_cost_observer_matches_game_frames_audio_and_full_state -- --ignored --nocapture --test-threads=1
```

No ROM, translation or diagnostic result is packaged or committed. This fixture
covers the exercised introductory sequence; it does not establish AI coverage
or every hardware/audio-engine/hook path.

## Frame-time diagnostics and their overhead

Add `--profile` when both executables support `FFTA_BENCH_FRAME_TIMES=1`. This
adds two separate instrumented runs after ordinary timing. Reports include p50,
p95, p99, maximum and counts above the GBA frame budget (16.742706 ms) and twice
that budget. The nearest-rank percentile convention is used. Samples measure
CPU wall time inside the frame execution path, including CPU PPU work; they
exclude the window, device callback, frame verification and report generation.

The collector reserves memory before timing, retains at most 360,000 frames and
reports omitted samples. The comparison rejects truncated diagnostic runs.
The report's overhead estimate compares a single instrumented run against the
uninstrumented median; noise can make this estimate negative. It is not a new
optimization result. Per-frame logging and final-state serialization stay outside
ordinary timing; sample capture is disabled unless explicitly enabled.

## CPU PPU timing

Add `--ppu-profile` to request separate instrumented runs with CPU PPU stages;
it implies `--profile` and requires both executables to support this diagnostic.
Ordinary verification/timing runs explicitly disable it, even if the parent
shell has `FFTA_BENCH_PPU=1`. For manual `runc` runs that environment variable
enables PPU capture without requiring the benchmark script.

The report has one sample per completed guest frame after headless warm-up:

- `hardware_scanlines`: wall time inside the 160 hardware scanline calls,
  including UI extraction and forced-blank handling.
- `zoom_rows`: wall time inside extra host zoom-row rendering when available;
  zero when no such work occurred. A parallel path includes the caller's wait
  for its workers, not the sum of worker CPU times.
- `core_other`: captured core interval minus those two PPU totals, including
  CPU execution, other hardware work and instrumentation overhead.

`FFTA_PPU_FRAME` rows provide p50/p95/p99/max, total/mean and budget counts;
`FFTA_PPU_COUNTS` checks frames, scanline/zoom calls and invalid interval
totals. The script rejects missing/duplicate stages, truncated captures,
nonfinite/inconsistent totals, missing scanlines and unexpected instrumentation
in normal timing runs. It stores the stages under each profile's `ppu` field
in `summary.json`. Compare overhead against uninstrumented timing separately;
do not sum percentiles from different stages as if they describe the same frame.

Capture is scoped and thread-local in `gba_core::ppu_profile`, outside machine
state and save serialization. Timers read the host clock only while enabled;
disabled PPU entry points only check whether a capture is active. Completed-frame
capture is bounded like the existing frame profiler. Incomplete guarded frames
are discarded. Nested/error/unwinding scopes restore their prior capture;
worker threads do not inherit it. No GBA cycles/events or GPU timestamps change.

For current-source desktop builds, reuse the Docker scripts with a fresh local
export directory before replacing existing dist archives. Check test gates,
archive CRC/member paths, required executables/loaders, executable modes and
private-data exclusions. Empty Data directories are the expected portable
scaffold; personal files and translation caches are not payload. Verify PE
resources/imports and Linux symbol requirements, and retain BUILD-INFO hashes.
A Windows/Wine replay can use the existing Windows translation with unchanged
ABI to check frames/state/counts against the old export and validated Linux
results. Such a correctness run is not a native Windows performance comparison.
Test the newly exported Linux package itself, including cold-cache startup and
restore, before updating canonical local archives; preserve the old outputs.

`FFTA_PROFILE=1` also includes these rows in desktop exit reports, with the
interactive profiler's first-18000-frame limit and omitted-sample counts.
It still includes startup and has no GUI scene/warm-up selector. Actual GPU
execution timestamps and the originally reported slow enemy turn remain
separate work.

### Optional scanline stage attribution

Set `FFTA_PPU_STAGES=1` in addition to `--ppu-profile` for headless comparisons,
or alongside `FFTA_PROFILE=1` for desktop reports. It is resolved at profiler
creation; it does nothing without the enclosing PPU capture. Ordinary timing
and verification runs retain disabled capture. Coarse-only profiling still
works without this flag and does not read the additional stage clocks.

`FFTA_PPU_STAGE` reports bounded per-frame aggregates for disjoint intervals:

- `backgrounds`: BG rendering in the hardware and relocated wide passes.
- `objects`: OBJ rendering, plus wide-pass OBJ scratch reset.
- `composition`: hardware/wide composition.
- `ui_capture`: native panel/backing extraction, including its internal BG/OBJ
  preparation. Those inner operations are not counted again in other stages.
- `ui_setup`: line-zero dialogue/UI detection and preparation.
- `scanline_other`: scanline total minus these five intervals, including affine
  stepping, blank handling, scratch setup and timer/bookkeeping overhead.

These six totals partition `hardware_scanlines`; do not add them to that parent
or sum their percentiles. Extra zoom-row work remains a separate coarse total.
The comparison script validates complete, finite stage reports and their total,
then stores them as `ppu.scanline_stages`. Old baseline executables can omit
these optional rows. Check their presence in the candidate report before claiming
stage coverage. Compare its instrumented time with its own uninstrumented median
to estimate diagnostic overhead, keeping single-capture noise explicit. This
does not measure per-tile cost or GPU execution and changes no guest timing.

### Native UI background row reuse

UI extraction prepares native BG2/BG3 once within a scanline and shares those
immutable planes across primary/extra and normal/backing passes. Backing-only
text filtering uses a private copy. Stats still uses its original zero BG mask
and transparent input planes; portrait capture flags are resolved per pass.
Anchor accumulation and destination writes remain part of each extraction.
The data is not retained across scanlines or frames: VRAM/palette/register
changes and load/Undo need no new cache invalidation or save-state format.
This reuse belongs to `ui_capture`, not the separate world `backgrounds` stage.

UI rows with transparent native text/backing and no portrait pixels bypass the
world-candidate sort/blend loop. World-only candidates cannot become UI pixels.
The shortcut runs after backing allocation, anchor accumulation and portrait
extraction/flag cleanup, and fills the selected destination row with the exact
`UiPixel::default()` value so old frame/load contents cannot remain visible.
The previous complete loop remains a test-only const-generic reference;
nonempty rows retain it unchanged. This skips host UI work, not guest frames,
hardware scanlines, interrupts or decisions.

## Interactive CPU phase profile

On Linux launch the actual runtime or portable wrapper with:

```bash
FFTA_PROFILE=1 ./FFTARecomp
```

On Windows PowerShell:

```powershell
$env:FFTA_PROFILE = '1'
.\FFTARecomp.exe
Remove-Item Env:FFTA_PROFILE
```

Play through the target scene and close normally. The runtime prints
`FFTA_PLAY_PROFILE` summaries at shutdown. The Linux wrapper stores them in
`play.log`; when launching `recomp play ...` directly, redirect stdout/stderr to
an ignored local log. Windows wrapper success output is captured internally and
is not persisted, so use the runtime directly on Windows to retain these reports.

`FFTA_UI_UPLOAD` also reports layer-update calls and payload bytes scheduled by
those calls (textures plus uniforms), across the gameplay session. It is a
logical upload counter, not a measurement of PCIe traffic or VRAM use. Unchanged
FPS pixels and identical LUTs can now be reused. Game/world uploads retain their
normal update path; payload counters continue after timing samples reach their
retention limit.

The phases are:

- `core_including_ppu`: native/interpreter frame execution with PPU inside it.
- `ui_prepare`: packing the host UI layers.
- `temporal_push` and `temporal_compose`: CPU temporal history and color composition.
- `audio_handoff`: producer-ring transfer and any mutex wait in that function.
- `autosave`: periodic save check/hash/write, sampled only every 300 frames.
- `present_including_acquire_upload_submit`: CPU presentation call, including
  potential swapchain waits. This is **not GPU execution time**.
- `ui_layout_upload`: UI layout preparation and per-layer uploads.
- `gpu_cpu_setup_upload`: surface resize/resource setup, base-frame upload and
  uniform preparation. It excludes the UI uploads above.
- `gpu_cpu_acquire`: acquire the swapchain frame, including any existing
  reconfiguration/retry after a lost or outdated surface.
- `gpu_cpu_encode`, `gpu_cpu_submit`, `gpu_cpu_present`: command encoding,
  queue submission and the host present call measured separately. Queue upload
  transfers execute asynchronously; these are all CPU wall times, not GPU work.
- `pacing_wait`: sleeps explicitly used for the gameplay clock/ring pacing.
- `tick_after_input_including_work_and_wait`: inclusive gameplay tick after input
  handling; may include multiple guest frames or only a ring-full wait.

Counts differ between phases. The presentation subphases overlap the total
presentation time; tick samples overlap the other phases. Do not add inclusive
totals to their subphases. Stages not completed after an error have no sample. Paused-menu rendering, input polling, GPU execution, the audio
consumer callback and final shutdown save are not measured by these phase timers.
The first 18,000 invocations of each phase are retained, with omission counts;
record short focused sessions or inspect omitted counts before interpreting them.
Interactive capture currently includes startup/resource-creation samples; unlike
headless comparison it does not yet filter a warm-up or label a scene interval.
A single initial pipeline/texture-creation spike does not establish persistent
stutter. Warm-up/scene selection and controlled GUI overhead comparisons remain
part of PERF-02.

No phase timers or sample allocations are enabled by default.

The developer `--stats` title shows native blocks and fallback steps as separate
counts, replacing the misleading mixed-unit percentage. FPS INT/EXT remain the
existing user-facing rate counters. An exhausted emulation guard now fails before
pushing incomplete frames into temporal history or incrementing the frontend's
completed-frame bookkeeping; error exits still release desktop resources.

## Remaining work

Collect an identified slow enemy-turn replay and additional zoom/menu/animation
workloads. Refine PPU timing inside scanline work; add actual GPU timestamp queries with
asynchronous retrieval only where supported. CPU acquisition/upload/submit
intervals are already separated by the interactive profile.
Measure audio callback contention and underruns, presentation cadence and profile
overhead in controlled native GUI runs. The current profiler supplies CPU wall
times and comparison guards, not a completed diagnosis of the reported AI lag.

## Panel-bound scan diagnostic

A test-only candidate derives each nonempty row's first and last visible pixel
instead of accumulating every visible pixel. It retains the current
`pixel_bounds` inset coordinates, exclusive right/bottom bounds and support for
a partial final row. Production still uses the full scan: the integrated
candidate passed equivalence but was slower in two 3000-frame introductory
comparisons, including one with CPU affinity. No content cache, guest state,
serialization or translation ABI changes are involved.

Run the reference-equivalence test and optional release microbenchmark serially:

```sh
cargo test --locked --release -p gba-core row_endpoint_bounds -- --test-threads=1
cargo test --locked --release -p gba-core benchmark_panel_bounds -- --ignored --nocapture --test-threads=1
```

The ignored diagnostic compares the previous full scan with the test-only scan for
empty, dense, sparse and rectangular-panel 480x160 inputs. Each of five
alternating pairs measures 2000 calls using `black_box`. This measures one
function on synthetic data; it does not establish whole-game or enemy-turn gains.
Run without other CPU workloads and report both improvements and regressions.

On Linux, a follow-up A/B run can inherit a single allowed CPU using
`taskset -c CPU python3 scripts/benchmark_windows_runtime.py ...` (replace
`CPU` with an allowed CPU number). Record that constraint, keep workload/frame
count/settings identical between comparisons, and do not compare a short pinned
replay directly against a longer unrestricted replay. Affinity can reduce
migration noise but does not eliminate frequency, thermal or background-load
variation, and it does not establish performance on other machines.
