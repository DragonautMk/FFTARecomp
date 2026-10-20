# Task handoff

Updated: 2026-10-20. Read before selecting work after compaction or continuation.
This is the current queue, not a second chronological changelog. The
[memory workflow](MEMORY_WORKFLOW.md#resume-and-repeat-decisions) defines how to
reuse evidence and when a repeat is justified. Latest user instructions take
precedence over this queue.

## Next open work unit

- **PERF-01-HIGH-ZOOM-WORKLOAD — in progress.** The current
  validated scene comparisons used zoom zero. Reuse the existing enemy snapshot
  and input with a controlled high-zoom schedule, inspect existing relative-zoom
  checks first, and collect bounded scanline/extra-row stage attribution. Do not
  create another recording or same-binary five-pair calibration merely to resume.
  This closes a missing scenario criterion, not the completed ordinary battle
  baseline. If additional zoom rows dominate, evaluate the existing persistent
  parallel-PPU feature as PERF-12 with real candidate equivalence/timing; otherwise
  record why parallel/GPU work is deferred. No new rendering policy is approved
  without its measured benefit and correctness evidence.
- PERF-03-CURRENT-DESKTOP-BUILDS is complete for container/Wine/Linux coverage.
  Current archives are in `dist/docker-windows/` and `dist/docker-linux/`; prior
  exports are preserved under `current-desktop-builds/previous-exports/` locally.
  Native Windows/hardware GPU coverage is still absent, not implicitly passed.
- PERF-08-UI-EMPTY-ROWS is complete and retained. Do not repeat its synthetic
  reference cases, fixture creation or baseline comparison merely to resume.
- PERF-08-UI-PREPARED-BG-ROWS is complete and retained. Do not repeat its input
  recording, reference-test setup or baseline comparison merely to resume.
- PERF-08-UI-ROW-INVARIANTS is complete and retained. Do not redo its exhaustive
  window test, fixture creation or baseline comparison just to resume context.
- PERF-02-PPU-STAGES is complete for native Linux attribution. Reuse the stage
  report rather than repeating its five-pair validation. The original reported
  slow turn and native Windows/GPU execution timing remain uncharacterized;
  those limitations do not reopen the completed recording or instrumentation.
- Other remaining acceptance criteria stay in [PERFORMANCE_TODO.md](PERFORMANCE_TODO.md).
  Its unchecked PERF parents are broader than the completed units below.

## Completed units — reuse, do not recreate

- **PERF-01-REPLAY-TOOLS — done for current desktop scope.** Native/Wine
  comparison, snapshot start, relative input/warm-up/zoom, per-frame image/OAM
  checks and optional full final state already exist. Evidence:
  [snapshot runner record](../changes/2026-10-20-resume-headless-benchmarks-from-snapshots.json)
  and [workflow](PERFORMANCE_WORKFLOW.md). Extend only for a specific missing check.
- **PERF-01-WORLDMAP — done.** World-map fixture visually classified and five
  1200-frame baseline pairs verified. Evidence:
  [evaluation record](../changes/2026-10-20-evaluate-ui-front-layer-selection.json).
- **PERF-01-ENEMY-REPLAY — done.** Recorded player waits, an observed enemy
  activation/movement and return to the player's menu; 3530-frame replay endpoint
  checked against the GUI. Evidence:
  [replay record](../changes/2026-10-20-record-observed-enemy-activation-replay.json).
- **PERF-01-ENEMY-BASELINE — done.** Five alternating pairs and separate PPU
  profiles passed; full final state and image/OAM equality, median about 8.3 ms,
  no over-budget frames in the two profiles. Same-code calibration, not a speedup.
  Evidence: the replay record above. The earlier Broken-pipe attempt is superseded
  by this completed rerun; do not restart it.
- **PERF-02-COARSE-PROFILE — done.** Core, scanline/zoom/remainder and frontend
  phase diagnostics exist and are reused by detailed attribution.
  Evidence: [workflow](PERFORMANCE_WORKFLOW.md) and [project state](PROJECT_STATE.md).
- **PERF-02-PPU-STAGES — done for native Linux.** Optional disjoint BG/OBJ/
  composition/UI extraction/setup totals implemented and validated on the existing
  enemy replay. Five new-code A/B pairs passed state/image/count checks; disabled
  difference +0.34%, detailed diagnostic overhead estimate +0.67%. UI capture is
  dominant. Evidence: [stage record](../changes/2026-10-20-attribute-cpu-scanline-stages.json).
- **PERF-08-UI-ROW-INVARIANTS — done, retained.** Per-call window/vertical
  membership, native origin and modal flag caching preserves window precedence
  and wrapping. 2457600 reference cases, 169 core and 48 runtime tests passed.
  Battle work time fell 2.176% over five pairs; state/image/count equality held.
  World-map median +0.431%, no over-budget profile frames. New-code load/Undo/
  reload/close passed under Xvfb with detailed profiling enabled. See
  [row-window record](../changes/2026-10-20-cache-ui-window-row-invariants.json).
- **PERF-08-UI-PREPARED-BG-ROWS — done, retained.** Native BG2/BG3 are prepared
  once per scanline and shared across panels/passes; backing filtering stays
  private and Stats retains its mask. New reference cases and 170 core/48 runtime
  tests passed. Five battle pairs show 4.144% less host work versus the accepted
  row-window release, full state/image/count equality. World-map and new-code
  GUI load/Undo/reload/close checks passed. See
  [shared-row record](../changes/2026-10-20-share-native-ui-background-rows.json).
- **PERF-08-UI-EMPTY-ROWS — done, retained.** Empty-source UI rows clear stale
  destinations and bypass only world-candidate work after setup/flag cleanup.
  Prior full-loop comparisons, 171 core/48 runtime tests, five battle and five
  world-map pairs passed with state/image/count equality. Battle median work
  fell 21.503% versus the shared-BG release; GUI load/Undo/reload/close passed.
  See [empty-row record](../changes/2026-10-20-skip-empty-ui-source-rows.json).
- **PERF-03-CURRENT-DESKTOP-BUILDS — done for current code and stated scope.**
  Windows/MinGW Docker export passed 317 Wine tests; Linux Bookworm export passed
  318 tests. Archives passed integrity/privacy/runtime identity checks; PE imports
  unchanged and resources present, Linux requires GLIBC 2.35. Windows exported
  runtime matched old Windows and validated Linux on 3530-frame image/state/count
  replay. New Linux package passed cold-cache GUI load/Undo/reload/close. See
  [build record](../changes/2026-10-20-validate-optimized-desktop-exports.json).
- **PERF-04-RECOVERY-IMPLEMENTATION — done; wider validation remains open.**
  Timeout skip, bounded surface recovery and fallback already implemented.
  Evidence: [surface record](../changes/2026-10-20-recover-transient-desktop-surfaces.json).
- **PERF-05-LUT-FPS / PERF-06-FIFO-PACING — done for these substeps.** LUT/FPS
  caching, FIFO capacity reuse and startup pacing resolution already implemented.
  Remaining world/panel cache criteria are separate work. Evidence:
  [reuse record](../changes/2026-10-20-reuse-desktop-uploads-and-audio-buffers.json).
- **DOC-README-FEATURES — done.** Seven concise additions preserve the layout.
  Evidence: [README record](../changes/2026-10-20-summarize-desktop-features-in-readme.json).
- **WORKFLOW-RESUME — done.** Skills and AGENTS route resumption here and require
  a concrete invalidation reason for costly repeats. Procedure:
  [memory workflow](MEMORY_WORKFLOW.md#resume-and-repeat-decisions).

## Rejected experiments — retain the decision

- **PERF-06-ROW-BOUNDS — rejected for production.** Microbenchmark benefit did
  not survive end-to-end replays. Test-only experiment retained; do not activate
  it without new representative evidence. See [project state](PROJECT_STATE.md).
- **PERF-08-FRONT-TWO — rejected.** Both compact and simple selectors evaluated;
  no sufficient overall pacing benefit. Production sorting restored byte-for-byte.
  See [evaluation record](../changes/2026-10-20-evaluate-ui-front-layer-selection.json).
  Compaction or an open PERF-08 checkbox does not justify retrying these variants.

## Local artifact locators

These ignored paths are conveniences; tracked records above retain the result
when local artifacts are absent. Never copy fixture bytes into Git.

- Enemy replay: `target/desktop-perf/ai-turn-replay/` (`start.ffstate`, `input.input`).
- Completed baseline: `target/desktop-perf/ai-turn-baseline-complete/summary.json`.
- Preserved baseline executable: `target/desktop-perf/state-baseline/recomp`;
  translation: `target/desktop-perf/translation.so`.
- World-map results: `target/desktop-perf/worldmap-clean/summary.json`.
- The completed benchmark process exited successfully. No job is pending from
  that comparison. A future process must be recorded with its own handle and
  checked live; this historical statement is not a process inventory.

If a locator is absent, inspect records and available equivalents first. Restore
only the artifact needed for the selected new work; mark its availability
separately and retain the completed task's outcome.

## Latest validation state

PERF-03-CURRENT-DESKTOP-BUILDS completed; no pending process from this unit.
Windows build session `83015`, Linux build `2279`, Windows replay `31793` and
Linux package GUI `35307` exited successfully. Local evidence and input/artifact
hashes are in `target/desktop-perf/current-desktop-builds/`; GUI captures/logs in
`target/desktop-perf/current-export-gui/`. Both canonical dist archives now contain
all retained renderer changes. Previous canonical exports remain preserved.
Do not rerun these build gates unless inputs change or a distinct missing
criterion requires it. Native Windows/physical controllers/hardware GPU timings
remain outside this evidence; Wine correctness is not native Windows performance.

Current high-zoom characterization reuses the enemy replay with level 6 from
relative frame zero, one ordinary and one diagnostic run. This is a new missing
scenario and diagnostic-overhead check, not a five-pair same-binary calibration.
No source change, new recording or build is required.

High-zoom run is live in session `43239` (last observed running). Check it and
`target/desktop-perf/high-zoom-workload/driver.log` before restarting; results
will be written to its summary.json.
