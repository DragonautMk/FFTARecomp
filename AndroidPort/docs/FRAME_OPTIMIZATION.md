# Android frame and zoom optimization (2026-10-18)

The subsequent dialogue-only optimization and current APK are documented in
[DIALOGUE_ROWS.md](DIALOGUE_ROWS.md). Measurements below describe this earlier
iteration and remain useful as its baseline.

Continuation of DIALOGUE_PERFORMANCE.md. The APK now enables the measured
performance-core policy during normal gameplay, after starting the independent
audio worker. Only AndroidPort was changed; PC/Linux and installer are untouched.

## Implementation

- Detect site, Pub and Shop panels from a single EWRAM cursor scan at scanline
  zero, sharing validated candidates between the existing detectors. A bounded
  stack buffer avoids per-frame allocations; overflow falls back to the full
  queries. No results are cached across frames or RAM changes.
- When the detached UI requires a second world composition, compose only the
  original 240 columns in the first pass. The original hardware framebuffer is
  still produced, and the second pass still produces the entire expanded view.
- Reject off-row world sprites before UI classification when separate UI means
  those sprites cannot be transformed. Guest sprite producers and culling hooks
  are unchanged.
- Use a PC-specific integer hash table for native dispatch instead of SipHash.
  Thumb bits, RAM/ROM regions, equality and fallback behavior are preserved.
- Prefer performance cores for serial simulation using Android's cpu_capacity
  data within the current allowed affinity mask. Performance and prime clusters
  are both retained. Homogeneous cores, missing data or denied affinity leave
  normal scheduling intact. No CPU model, core number, frequency or thermal
  policy is hardcoded or overridden. Hardware validation here is RG Rotate only;
  selection logic for other cluster arrangements has unit coverage.
- Optional debug counters split original scanline rendering, expanded zoom
  rendering, panel setup, UI capture and export. They are disabled in normal play.

## Measurements on physical RG Rotate

Android 12, T618, 720x720 display, verified USA ROM. Each replay starts at frame
zero and uses existing benchmark.input and the separate benchmark.sav. Normal
ffta.sav was compared before/after every successful run and remained unchanged.
No original PC ROM/save files were written.

Same-APK CPU-affinity A/B, 3000 frames, maximum zoom, identical 433 panel-active
frames: complete-frame mean decreased from 31.8503 to 18.3434 ms, p95 from
35.6099 to 19.3142 ms. All 12 UI/framebuffer/OAM checkpoint hashes and the final
screenshot PNG match. Earlier frame-work A/B also matched all 12 hashes.
This comparison isolates CPU affinity after the other frame optimizations.

Final APK: three full 13100-frame replays, with maximum zoom tested first.
Viewport sizes match the RG Rotate square/fullscreen zoom formula:

| Zoom level | World viewport | Camera CPU mean | Camera CPU p95 | Panel-active CPU mean | Panel-active CPU p95 |
| --- | --- | ---: | ---: | ---: | ---: |
| 2 | 324x324 | 11.6696 ms | 13.1279 ms | 16.5468 ms | 18.1186 ms |
| 3 | 364x364 | 12.3348 ms | 13.6525 ms | 17.0700 ms | 18.5420 ms |
| 6 (maximum) | 512x512 | 14.9368 ms | 16.1298 ms | 18.8911 ms | 20.0060 ms |

Camera samples are the final 1201 frames; each run has 2455 main-panel-active
frames. Recorded scroll ranges are HOFS 8..142 and VOFS 8..72 in each run.
This validates that recorded movement, not an exhaustive traversal of every map.
Camera frames within the native 16.7427 ms simulation budget: 1195/1201 at
level 2, 1199/1201 at level 3, and 1187/1201 at level 6. Occasional outliers
remain. These are CPU simulation timings, not measured display FPS. In
particular, panel-active level 3 still exceeds the budget on average; fixed
60 FPS everywhere is NOT claimed. No frames or world content were skipped.

## Validation and reproducibility

- Core suite: 162 passed, 4 ignored; native Android suite: 13 passed, 2 ignored.
- USA and EUR: original framebuffer, OAM and zero-extra presentation are
  identical on every one of 600 frames per region.
- Full tutorial regression: 1201/1201 final samples have 8 present and 8 drawn,
  no duplicate IDs and no missing positions.
- Build, Kotlin unit tests, APK assembly and adb install -r succeeded.
- Final APK SHA256:
  f42790743c203eb599733c7a5fa6e6047917353db4b33cde5c6153953ba9b358.

Debug-only benchmark utility: scripts/benchmark_dialogue.py (standard library).
It expects the user's private benchmark.input and benchmark.sav already in the
app's files directory; neither is distributed in source. Example from repo root:

```powershell
python AndroidPort/scripts/benchmark_dialogue.py zoom6 --adb C:/platform-tools/adb.exe --serial ROTATE01523130 --zoom 6
python AndroidPort/scripts/benchmark_dialogue.py zoom2 --adb C:/platform-tools/adb.exe --serial ROTATE01523130 --zoom 2
python AndroidPort/scripts/benchmark_dialogue.py zoom3 --adb C:/platform-tools/adb.exe --serial ROTATE01523130 --zoom 3
```

Use --slow-cpu for the affinity control, --reference-frame for the frame-work
oracle, and --reference-ui for the older pixel extractor. Character counters
remain off in performance runs because they disable parallel zoom rendering.
Reports and screenshots are under target/dialogue-<tag>/ and are local test
artifacts. Final measurements: target/frame-final-results.txt. Test logs:
target/frame-final-{core,native,600-usa,600-eur,eight}.log.

APK: APK/FFTARecomp-android-arm64-preview.apk. Installed on the RG Rotate without
uninstalling, clearing data or changing the imported ROM.
