# Dialogue row reuse and clipping optimization (2026-10-18)

This follows FRAME_OPTIMIZATION.md. Changes are confined to AndroidPort; no
guest patches, sprite producers, camera ranges, PC/Linux or installer changed.

## Work removed

- Reuse this scanline's already-decoded native BG2/BG3 pixels when their map
  coordinates are in bounds and no host reconstructed plane replaces them.
  Affine/bitmap modes, wrapped edges and reconstructed planes retain decoding.
  Nothing is cached across frames or VRAM updates.
- Sort background priority once per row and merge only the two leading visible
  backgrounds with the object pixel. Preserve the original priority tie rules,
  world occlusion, window masks and GBA blend arithmetic.
- Test panel membership once per pixel instead of once for every background.
- Skip UI pixel composition on rows where the game's window masks disable the
  UI backgrounds and no portrait/cursor pixel exists. Preserve anchor updates.
- Reuse the first world composition on rows with no UI objects and empty or
  masked native UI backgrounds. Stale margin tiles are cleared exactly as the
  separate world pass would clear them. The original hardware framebuffer is
  still computed and character pixel counters are preserved.
- Keep the full world/background path for Stats panels, which require their
  separate raw background for alpha blending.

The previous path remains a debug oracle: ffta_benchmark_reference_dialogue=true
(benchmark utility --reference-dialogue). Normal gameplay uses the optimized
path; this is not a saved user preference or a new menu option.

## Physical RG Rotate measurements

Android 12, T618, USA ROM, square viewport. Each run starts at frame zero using
the existing recording and separate benchmark.sav. Normal ffta.sav was unchanged
before/after every run. Testing started with maximum zoom as requested.

After a 6000-frame cache warmup, reference and optimized runs each contained
the same 1499 panel-active frames and 18026 native blocks at every checkpoint.
Maximum zoom (512x512), mean CPU timings over those active frames:

| Component | Reference | Optimized |
| --- | ---: | ---: |
| Complete simulation frame | 19.9990 ms | 18.2338 ms |
| Original scanlines including UI | 7.4181 ms | 5.9618 ms |
| UI extraction | 2.1448 ms | 1.4067 ms |

UI extraction decreased 34.4%; complete-frame mean decreased 8.8% in this A/B.
Complete-frame p95 did NOT improve in that pair (22.9332 -> 23.2451 ms).
Timing variance remains; do not describe this as a complete stutter fix.

Final optimized runs, same 6000-frame recording and same 1499 active frames:

| Zoom | World viewport | Dialogue CPU mean | Dialogue CPU p95 |
| --- | --- | ---: | ---: |
| 2 | 324x324 | 14.9359 ms | 16.1163 ms |
| 3 | 364x364 | 15.5210 ms | 16.5794 ms |
| 6 (maximum) | 512x512 | 18.2338 ms | 23.2451 ms |

Level 2/3 p95 is below the native 16.7427 ms simulation budget in these runs.
These are CPU timings, not measured display FPS or a guarantee of fixed 60 FPS.
The earlier preliminary variant reduced extraction cost but did not improve
whole-frame timing; the final implementation includes window-masked rows.

## Validation

- All 12 reference/optimized checkpoint hashes match (UI, framebuffer, OAM),
  and final screenshot PNGs are byte-identical. The preliminary A/B also matched.
- Core: 163 passed, 4 ignored. Native Android: 13 passed, 2 ignored.
- Native-row tests cover both backgrounds, 4/8bpp, all map sizes, scroll edges
  and unaligned mobile zoom widths. Pixel equivalence tests include both panel
  sides, blending/brightness modes, window-masked rows and the wide framebuffer.
- USA/EUR each match original framebuffer, OAM and zero-extra presented pixels
  on every one of 600 frames.
- Full tutorial regression: 1201/1201 final frames have 8 present, 8 drawn,
  no duplicate IDs and no missing positions.
- APK build, Kotlin tests and adb install -r succeeded. No uninstall/data reset.

Reproduce with scripts/benchmark_dialogue.py, --frames 6000 --zoom 6 first.
Use --reference-dialogue for the A/B control, then --zoom 2 and --zoom 3.
Local artifacts: target/dialogue-dialogue2-{control,final,zoom2,zoom3}/;
summary target/dialogue2-results.txt; tests target/dialogue2-delivery-*.log.
No ROM, save or input recording is included in source.

APK: [FFTARecomp-android-arm64-preview.apk](../APK/FFTARecomp-android-arm64-preview.apk)

SHA256: 072d7c674a5bb2252c0b1d43f561036bf51358c91551c88246dfb8aa853254c7
