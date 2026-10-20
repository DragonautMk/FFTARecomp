# Desktop FPS overlay

For the desktop portable editions, settings are under `Data/config` and the
cartridge save is `Data/saves/ffta.sav`, with backups and state slots beside it.
See [portable storage and migration](portable.md). Installed-mode paths below
apply when `portable.txt` is absent.


Open the pause menu with Esc, then **Video > FPS counter**. Choose **Off**,
**Internal**, **External**, or **Both**, using Left/Right or the dropdown.
The choice persists as `video.fps` in `av.cfg`; existing configs default to Off.
On Windows this file is `%APPDATA%\FFTARecomp\av.cfg`.

The small top-left panel shows:

- **FPS INT**: completed emulated GBA frames per second of wall time. Normal
  speed is about 59.7; fast forward can produce several internal frames per
  presentation.
- **FPS EXT**: successful game-frame presentations per second. It counts host
  submissions, not the monitor's refresh rate or physical scanouts. Failed
  presentations and pause-menu redraws do not contribute.

Values use a one-second measurement window and initially show `--.-`.
Pausing/resuming or changing the selected mode resets the window. The HUD is
hidden while the pause menu is open. It stays the same size when changing
game zoom or aspect ratio and follows window DPI for readability.

The GPU uses a final native UI layer above the dialogue layers; the software
presenter samples the same panel after composing the game UI. The panel uses
its own high-contrast palette, independently of game brightness and screen
simulation. Text is rasterized once per measurement window. Off creates no
HUD textures or per-frame text rendering. Guest memory, timing and save data
are untouched.

## Validation

Windows x86_64 GNU release builds were executed via Wine:

- 27 runtime tests, including independent internal/external counts during
  fast forward, reset after pause, HUD clipping and packed-layer sampling.
- 25 menu tests, including FPS dropdown commit/cycling and persistence.
- 22 input/config tests, including all four modes, case normalization,
  legacy configs and invalid values.
- GPU/CPU native-layer equivalence on AMD Radeon 780M / Vulkan, including
  the HUD's one-pixel backdrop texture.
- An interactive Windows/Wine smoke run selected Both through the actual
  menu, resumed gameplay and verified the saved `video.fps = both` setting.
  Captures show INT/EXT at 59.7/59.7 normally and distinct rates while holding
  fast forward. The game exited cleanly using an isolated test save.

Logs and visual previews are under ignored `target/windows-fps-*.log`,
`target/fps-*-preview.*` and `target/fps-ui-smoke/`.

## Upload reuse

FPS pixels carry a process-unique revision changed when the text is rendered
or reset. A layout/DPI change still updates uniforms, without reuploading the
unchanged text texture. Unversioned game UI keeps unconditional pixel uploads;
dynamic world textures keep their existing update path. Each GPU layer compares
its LUT contents against a retained copy, so in-place color changes are detected
without relying on allocation addresses. Recreated layers begin with empty caches.

For an unchanged 480x160 UI with a one-pixel world, scheduled update payload
falls from 438,338 bytes to 66 bytes (world pixel plus uniforms); the first update
and subsequent text changes still upload their required data. This is a transfer
payload reduction, not an FPS or AI-turn speedup measurement. CPU fallback reads
the same current frame data directly. Opt-in `FFTA_PROFILE=1` diagnostics report scheduled UI payload bytes at shutdown.

The extended GPU/CPU equivalence test checks repeated frames, in-place LUT
mutation, changed UI revisions, unversioned UI, changed world data and recreated
resources at three world sizes including 1x1. On AMD Radeon 780M / RADV it passed
all pixel comparisons and exact scheduled-byte assertions. Meter tests cover
unchanged revisions between measurements, reset, DPI/layout changes and distinct
meter instances.

### Temporary surface failures

A GPU surface timeout skips that host presentation and retries on the next
normally paced frame. It does not disable GPU rendering or increment EXT.
Lost/outdated surfaces receive one reconfiguration/retry per frame; three
consecutive failed recoveries activate software fallback. Successful acquisition
resets that budget. Memory exhaustion and unknown errors fall back immediately.
Zero-sized windows skip GPU acquisition. Guest frame accounting stays separate.
