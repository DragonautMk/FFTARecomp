# Live aspect ratio and action HUD margins

The pause menu's Aspect setting now updates the active view on resume.
It recalculates the starting width and zoom limit, retains the current zoom
within those limits, and lets the existing frame-boundary sync resize buffers
and reset temporal history. Environment overrides set the initial ratio only;
an explicit menu change takes precedence for the running session.

The open EUR action HUD was inspected using a desktop screenshot and a
read-only graphics-memory snapshot. DISPCNT was 1F40, terrain BG0/BG1 were
4C03/4E02 at scroll 151/73, and static HUD layers were BG2=5C08 and BG3=4A09.
The left edge of BG2 contained opaque black pixels on rows 152 and 156.
The generic static-layer edge extension repeated these meter borders.

For this verified battle HUD signature, BG2 foreground stays inside the
original viewport while BG3 backing retains its normal margin extension.
The gate requires the verified movement-map patch and the terrain pair;
unrelated menus and unverified ROMs retain their prior behavior.

Snapshot previews: target/live-action-before.png and live-action-after.png.
These are offline renders of the captured graphics data, not captures of a
new running executable. The preview comparison changed 4592 margin pixels
and zero pixels in the original 240x160 viewport.

Tests: core 93 passed (4 ignored), menu 21 passed, recomp 14 passed.
Windows executable is built separately as target/release/recomp-live-fix.exe
so the user's open game session can keep running.

## EUR dialogue at 32:9, zoom x1

The live dialogue snapshot showed BG2=0B08 with HOFS=6 and BG3=0A08
behind the text. At x1 the scrolling text plane followed the generic
circular-map path, repeating text in both horizontal margins. The verified
battle dialogue foreground is now confined to the native viewport at every
zoom; its separate panel backing still extends normally. Tile spans split
at viewport edges so nonzero HOFS cannot remove native pixels. Native
scroll wrapping is retained, including HOFS=506.

Core tests: 94 passed, 4 ignored, serial execution. The captured-state
previews target/dialogue-x1-before.png and dialogue-x1-after.png differ by
1533 margin pixels and zero native viewport pixels. They are reconstructed
from the read-only graphics snapshot, not captures of the updated window.
The new binary is target/release/recomp-dialogue-fix.exe.

## Native-resolution pause menu and scene transitions

The menu canvas now matches the physical window dimensions instead of being
capped at 1600x1000 and enlarged. Vector icons and font outlines rasterize
directly into that canvas, with proportional layout at 1080p and 4K. The
presenter uses direct pixel sampling for the UI. Idle menu frames keep their
last presentation; input and window events invalidate the menu for redraw.

Desktop frame readback now matches Android when vertical zoom is enabled but
the current scene has no expanded terrain rows: the newly rendered 160 rows
are centered with cleared outer rows. Previously desktop displayed the full
wide buffer, including extra rows left over from an earlier scene.

A synthetic regression seeds those stale rows with old object pixels and
verifies they are excluded, while complete battle frames pass through without
padding. Core, menu and executable tests passed: 94, 22 and 15 respectively.
This proves the stale-row fix; the specific brief startup objects reported
by the user have not yet been reproduced and confirmed visually.
