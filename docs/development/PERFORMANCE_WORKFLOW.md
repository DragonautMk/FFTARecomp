# Desktop performance measurement workflow

Start from the [performance TODO](PERFORMANCE_TODO.md). Keep the baseline binary,
its TinyCC loader and a single ROM translation. Use a release build and separate
verification from work timing. ROMs, save copies, recordings and result logs stay
under ignored local directories. The benchmark never writes the supplied save.

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
it does not identify an AI workload. Inputs use absolute guest-frame numbers from
boot. `--sav` preloads cartridge progress, not a save state; recordings still need
to navigate from boot. Resuming arbitrary `.ffstate` files is not supported by
this command yet. Add `--zoom '0:0,1000:4'` to compare a fixed zoom schedule.

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
workloads. Separate PPU from core timing; add actual GPU timestamp queries with
asynchronous retrieval only where supported. CPU acquisition/upload/submit
intervals are already separated by the interactive profile.
Measure audio callback contention and underruns, presentation cadence and profile
overhead in controlled native GUI runs. The current profiler supplies CPU wall
times and comparison guards, not a completed diagnosis of the reported AI lag.
