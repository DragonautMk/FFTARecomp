# Desktop updates and experimental Android touch selection

The dedicated AndroidPort core now includes the desktop USA/EUR semantic
selectors, world map, battle grid, action/confirmation lists, shops and town
services, together with the current sprite, terrain and UI fixes. Existing
Android compiler, audio, gamepad and touch-layout editing remain Android code.
ROM/profile checks and short function signatures remain in the core.

Input > Touchscreen adds Experimental touch selection, OFF by default, next to
Vibration feedback. The English description marks this as a work-in-progress
preview. Single taps on the game use the existing PC targets and GBA button
pulses; unsupported targets fall back to A. There is no simulated continuous
hover or multi-finger world clicking. Compose handles its controls and paused
settings separately, so these do not also generate game clicks. The engine
applies queued events on its worker, never on the Android UI thread.

Text bubbles is a top-level pause option with persistent Size and Length.
Dialogs, portraits, continuation arrows, stats and action panels use the
desktop layout rules in a separate OpenGL pass. Native 480x160 UI textures
are mapped directly to screen coordinates with uniform scale; world zoom
does not rescale these textures. Blending uses the GBA coefficients and raw
BG behind stats. An earlier CPU overlay was removed after the reported
performance regression. Empty native UI rows skip priority/blend work.

Validation logs are local target artifacts, not distribution inputs:

- touch-core-final.log: 159 passed, 4 ignored, serial execution. Parallel test
  execution failed because viewport globals are shared between tests.
- touch-engine-final.log: 11 passed, 2 ignored, including uniform screen scale
  and disabled-touch isolation. Kotlin unit tests: 22 passed.
- touch-zero-usa.log / touch-zero-eur.log: native 600-frame comparisons pass
  for framebuffer, OAM and every presented pixel with extra=0.
- touch-device-test.log: RG Rotate Android instrumentation passes one click
  per game tap, excludes touch A and excludes pause UI.
- touch-gpu-device/: complete 13100-frame replay, all 1201 final samples have
  8 present and 8 drawn characters; normal save checksum unchanged.
- touch-final-characters.csv: repeats the final 1201 character samples on the
  final core with detached UI enabled; uses private save/replay copies only.

Character counters disable parallel zoom rendering. Their timing is NOT normal
gameplay performance. The separate performance replay uses the same input with
ffta_benchmark_metrics=false and preserves parallel rendering. This debug flag
has no effect in normal gameplay. Full manual touch coverage of every menu and
every language is not claimed; this feature remains experimental.

Final validation: the final-core character replay has 1201/1201 samples with
8 present, 8 drawn, no duplicate IDs and no missing positions. Two complete
device replays match framebuffer and OAM at all four recorded checkpoints.
The normal save checksum is unchanged. In the final 1201 camera frames without
character counters, mean simulation time is 17.343 ms, p95 20.315 ms. These are
simulation timings, not measured presentation FPS; stable 60 FPS is not claimed.
The GPU implementation removes the CPU screen-sized UI composition, but full
restoration of the user's perceived performance still needs manual confirmation.

APK: ../APK/FFTARecomp-android-arm64-preview.apk.
