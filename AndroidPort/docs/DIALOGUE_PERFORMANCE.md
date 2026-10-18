# Android dialogue rendering optimization

Later frame/CPU optimizations and the current APK measurements are documented
in [FRAME_OPTIMIZATION.md](FRAME_OPTIMIZATION.md). Numbers below are the earlier
iteration, retained as historical measurements.

Measured on the connected RG Rotate (Android 12, T618), using the existing
deterministic input from frame zero and the isolated benchmark save. Normal
save data was checked before and after each run and remained unchanged.
Only this AndroidPort workspace was changed.

## Change

The detached UI extractor keeps only the two highest-priority contributors
instead of sorting every contributor for every pixel. Window registers and
row predicates are resolved outside the pixel loop; transparent margins can
be skipped. Panel attachment now decodes the two edge columns it actually
examines instead of decoding 240 columns and discarding 238 of them.
No guest instructions, camera ranges, character hooks or terrain data changed.

The previous extraction path remains available only through debug benchmark
intent extras: ffta_benchmark_profile_ui=true enables counters and
ffta_benchmark_reference_ui=true selects the reference path. Neither profiling
nor the reference path is enabled in normal play.

## Measured result

Final same-APK reference/optimized replay: 3000 frames, 512x512 world viewport
(extra_x=136, extra_y=176), no clock limiter, character counters disabled.
Both runs contain exactly 433 frames with the main detached panel active.
Mean CPU times over those same active frames:

| Component | Reference | Optimized |
| --- | ---: | ---: |
| Complete simulation frame | 40.3142 ms | 36.9844 ms |
| UI pixel extraction | 5.8123 ms | 3.5848 ms |
| Panel detection/setup | 3.4081 ms | 2.8325 ms |
| UI export | 0.2173 ms | 0.2156 ms |

This is an 8.3% reduction in complete frame CPU time and a 38.3% reduction in
UI extraction cost. These are CPU timings, not presentation FPS. Stable 60 FPS
at maximum zoom is NOT established; the measured frame time still exceeds its
budget. Device timing varied between runs, so comparisons across separate
benchmark batches should not be treated as equivalent conditions.

An earlier 6000-frame comparison, before the two-column optimization, included
1499 panel-active frames: extraction decreased from 3.3526 to 2.1143 ms.

## Correctness

- Final replay: all 12 checkpoint hashes (UI pixels, framebuffer, OAM) match
  between reference and optimized paths. Final screenshot PNGs are identical.
- Earlier 6000-frame replay: all 12 checkpoint hashes also match.
- USA and EUR: framebuffer, OAM and presented native pixels match the original
  for all 600 zero-extra frames in each test.
- Full tutorial replay: 1201/1201 final samples have 8 present and 8 drawn,
  with no duplicates or missing positions.
- Core tests: 161 passed, 4 ignored. Android native tests: 11 passed, 2 ignored.
- APK build, including the existing Gradle checks, succeeded.

Local reports: target/dialogue-final-{reference,optimized}/,
target/dialogue-{core-final,native-tests,600-usa,600-eur,eight}.log.
These reports contain no redistributable ROM and are not source assets.

APK: ../APK/FFTARecomp-android-arm64-preview.apk
SHA256: ae79d64b716d7a99968533d0fbeaaf842b8ef439eafa9e6f12d5208f049aa827
