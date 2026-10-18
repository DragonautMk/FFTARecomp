# Desktop engine: Android USA/EUR patches

The desktop core now uses the regional sprite profiles from
AndroidPort/crates/gba-core. Exact SHA256 identity and per-function digests
select the profile; copied IWRAM functions and the size table are checked
against the corresponding ROM before their hooks run.

Ported behavior:

- Regional sprite producers, unit visibility, and vertical zoom visibility.
- Verified movement and terrain source maps for USA and Europe.
- Finite EUR background margins during zoom-out.
- EUR stats panel padding and flat panel fill instead of repeated outlines.
- Exclusion of parked guest OAM entries exposed by vertical zoom.
- Host OAM publication and rendering when vertical margins are enabled.
- Reusable per-worker zoom buffers and audio mixer work only when a sample is due.

The desktop XInput path and pause menu are unchanged. Translation cache
revision 6 regenerates native code containing the new EUR guards.
The installer recognizes the exact USA and Europe SHA1 hashes and requires
the regional sprite and map producer checks before enabling patches.

Validation performed locally:

- Exact USA/EUR regional function and map producer signatures passed.
- Core suite: 92 passed, 4 ignored, run serially because view dimensions are
  process-global. One concurrent suite run failed the parallel zoom test;
  the isolated test and complete serial suite passed.
- EUR boot, extra=0: framebuffer and OAM hashes matched for every frame in
  two 600-frame runs, with BG/SPR patches disabled versus enabled.
- Installer EUR policy: exact ROM verified; one-byte alteration unpatched.

Battle gameplay at zoom-out and the final tutorial character metric have
not been remeasured on desktop in this integration. The 600-frame check
above covers boot only. Original ROM and save files were not modified.
