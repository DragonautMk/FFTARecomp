# Android frame-spike review

Device: RG Rotate, Android 12, T618, 720x720. All device replays start at
frame zero and use isolated benchmark.sav. The ordinary save is checked unchanged.
No ROM, recording or save is included in source. Native CPU/PPU/game patches are
unchanged by this revision.

## Measured causes and changes

The old benchmark measured gbaFrame only. New benchmark-loop.csv also records
texture publication, AudioTrack submission, benchmark bookkeeping, autosave,
periodic diagnostics and total game-thread work (before pacing).
The logging is debug-benchmark only, with no loop file in ordinary play.

The baseline 6000-frame max-zoom replay found one 16.62 ms publication/audio
outlier, autosave <=0.60 ms, periodic diagnostics <=2.92 ms, and benchmark
checkpoint bookkeeping <=5.12 ms. The first frame is excluded from steady-state
statistics. Background-render outliers and frames 5048-5050 also remain in the
native gbaFrame measurement; their exact guest-code cause is not established.

- GL texture uploads no longer hold the simulation's publication monitor.
  A latest-frame mailbox transfers exclusive ownership to GL, returning that
  buffer only after texture upload has finished. A pending obsolete presentation
  may be replaced, but guest frames still execute in order. Buffers are reused;
  at most three complete sets are needed when GL and simulation overlap.
- AudioTrack volume is sent only when its value changes, avoiding the redundant
  per-frame call. Audio samples, speed modes and autosave behavior are unchanged.
- Ordinary gameplay no longer computes full framebuffer/UI/OAM diagnostic hashes
  every 120 frames. Explicit benchmark checkpoint hashes remain available.

## Verification

Kotlin: 24 tests passed, including mailbox ownership and frame replacement.
Full tutorial replay: 1201/1201 final frames contain eight present and eight drawn
characters, with no duplicates or missing positions.
Previous native zero-extra 600-frame USA/EUR equivalence proofs remain applicable:
this revision changes no native source or game logic. They were not rerun here.

Reproduce: scripts/benchmark_dialogue.py spikes-final --frames 6000 --zoom 6
(add --adb and --serial for the device). Local data: target/dialogue-spikes-*.
Results below are CPU/work timings, not measured on-screen FPS. They include the
extra cost of debug recording and benchmark diagnostics and do not imply fixed
60 FPS or elimination of all OS/GPU/scheduling stalls.

## Results and release decision

6000-frame max-zoom replay, excluding first-frame initialization:

| Measured interval (ms) | Baseline mean / p99 / max | Final mean / p99 / max |
| --- | --- | --- |
| Publication + audio | 0.369 / 0.818 / 16.617 | see separate intervals below |
| Publication only | not isolated | 0.062 / 0.286 / 0.659 |
| Audio only | not isolated | 0.199 / 0.302 / 24.064 |
| Total measured work | 15.501 / 20.781 / 31.082 | 15.423 / 20.763 / 38.943 |

These runs do NOT establish an overall improvement in worst-case latency.
Audio-path outliers persist despite WRITE_NON_BLOCKING; the timing interval
also includes possible OS preemption, so it does not prove an AudioTrack bug.
Exact syscall/scheduler attribution remains unmeasured. Native-render outliers
also persist. No claim of eliminating all spikes or of 60 FPS at zoom 6 is made.

A direct PCM ByteBuffer experiment was built/tested on the same replay, but did
not improve the result (total p99 22.338 ms, max 47.548 ms; audio max 30.558 ms).
It was reverted, including its temporary source and tests, before delivery.
Retained changes avoid redundant work and remove GL uploads from the publication
monitor, without modifying native emulation or the established audio queue.

The 12 checkpoint hashes (UI, framebuffer, OAM at four checkpoints) and final
screenshot PNG are identical to the baseline in both the retained implementation
and the discarded direct-buffer experiment. Normal save is unchanged throughout.

Final APK SHA256: 82cc573470bea60d5d5d30a4794e6a6f9e859e29f7db818bef0229f7665158a6.
