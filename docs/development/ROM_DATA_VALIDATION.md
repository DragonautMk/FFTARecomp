# Decoded resource validation protocol

ROMDATA-05, selected before measurements on 21 October 2026. This first
tranche validates `graphics.obj-dimensions`; it does not close the remaining
resource matrix or mod phases. Existing implementation/results are in
[runtime consumption](../rom_data_runtime.md) and [TASK_HANDOFF](TASK_HANDOFF.md).

## Acceptance limits fixed before comparison

- Guest state at each compared frame, central/wide pixels, OAM, RNG/clock and
  deterministic audio must be exact. Final-state hashes alone do not claim a
  complete audio-stream comparison. Reuse passed boot/original-routine checks;
  add world map, observed battle/menu and relevant transition coverage.
- Five alternating release timing pairs per representative replay: candidate
  median mean frame work may regress at most 2%. Separate instrumented profiles
  use diagnostic thresholds of +5% p95 and +10% p99. Exceeding these limits
  requires investigation before normal enablement, not deletion of the result.
  A host-load/setup failure invalidates only the affected timing observation.
- After the ROM is selected/read, first preparation plus activation has a
  median budget of 250 ms and ready-dataset activation 100 ms. Measure seven
  first/cached observations, retain all values and maxima, and separate source
  I/O, preparation, activation and whole-process launch where observable.
- Owned first-family runtime resources stay below 1 KiB. Independently measure
  peak RSS: the candidate may add at most 2 MiB versus corrected legacy startup,
  allowing normal allocator/page granularity without hiding a ROM-sized copy.
  Report type/capacity accounting separately from process RSS.
- One valid generation's logical data/metadata is at most 32 KiB and allocated
  storage at most 128 KiB on the tested filesystem. Retained old/orphan generations
  are reported separately; do not delete user data to make a space result pass.

These are local regression gates, not native Windows, USB-drive, physical GPU
or universal storage guarantees. No speedup is presumed against in-memory ROM.

## Reference and inputs

The legacy reference must include only the independently verified EUR host
dimension-source correction on the prior preparation baseline, without the new
ROM routing/resource fields. Comparing only `--rom-data` on/off in the new
binary would omit always-present routing/layout overhead. Build the reference
and candidate sequentially from the same isolated source path with identical
Rust/toolchain/profile settings; protect root sources and canonical executables.
Retain an additional flag-off result if useful to attribute costs.

Use the already verified private worldmap snapshot/neutral input and observed
enemy replay/checkpoint under ignored `target/desktop-perf/`. Preserve hashes
of ROM, snapshots, recordings, translation and TinyCC loader before/after.
Do not recreate fixtures or reinterpret an introductory demo as an AI scene.
Each platform uses its corresponding native translation and loader.

Use the existing benchmark script, exact full-state/count verification and a
fixed zoom/FOV/warm-up. Verification with per-frame hashing precedes timing;
timing checks final identity without per-frame hashing. CPU affinity does not
reserve a core: inspect host load and the SMT sibling before/after each batch.
Do not accept timings collected while compilation or another heavy job runs.

Additional GUI checks cover decoded-data mode with Fast, pause/resume and the
necessary restoration/transition paths. Passed file-level Load/Undo/backup
resource-retention tests remain closed unless code/input changes or a new GUI
criterion requires another observation. Packaging must select distributable
inputs explicitly and exclude extracted data, saves, recordings and generated
ROM code. Native Windows hardware remains a distinct required gate.

## Results

The first native Linux comparison uses the same isolated source path, release
flags and TinyCC translation for corrected legacy and decoded-capable builds.
An initial candidate copy retained old source mtimes, so Cargo reused the
reference; identical binary hashes exposed this before any measurement. Source
mtimes were refreshed and the candidate rebuilt. Distinct hashes and runc help
were verified. Root source bytes and canonical binary were preserved.

Five alternating pairs pass the preselected mean/tail limits:

- World map,240frames/120warm-up: median2.153946 ->2.175660ms (+1.0081%);
  instrumented p95+1.7287%, p99+0.2292%.
- Observed battle/menu/enemy replay,3530frames/120warm-up: median5.833322
  ->5.819583ms (-0.2355%); instrumented p95-3.7654%, p99-4.1991%.

Every frame's framebuffer/OAM, loaded identity, final full state and native/
fallback counts match; candidate counters show720/16988 guest resource reads
and2640/314108 typed host reads respectively. Source/fixture/library hashes
are unchanged. CPU15 affinity, no concurrent compilation; sibling CPU14 mean
busy0.745%/0.189%, one-second maxima2.94%/7.00%. These local headless results
are neutral within small variation, not a claimed speedup, realtime audio or
native Windows result. Profiles are separate instrumented observations.

Private evidence: `target/rom-data-validation/` reference-build.json, repaired
build log/help/hashes, native-{worldmap,enemy-menu}/summary.json and individual
logs, runner logs and per-second host-load observations. Full per-frame
state/audio and startup/storage/RSS results follow below. GUI
The later GUI/archive results below close those local gates; native Windows
and other regional initializer coverage remain open;
the comparison above alone does not enable the default.

### Native Linux complete-state and startup observations

The complete-state/audio harness runs two instances of the current runtime
(legacy reads versus attached decoded data); the separate corrected-old-binary
benchmark above checks every image/OAM and final state and includes layout/
routing costs. These are complementary comparisons, not interchangeable ones.
New32:9/zoom0 checks pass every full state, central/wide image, OAM, dispatch
and mixed/PSG/both FIFO/HLE output for worldmap240frames, recorded battle/menu/
enemy3530frames and clean boot with the protected suspend save through8476frames.
HLE output is nonempty and compared by bits after identical observer setup.
The boot replay first reads the primary source at frame541/clock151880961 and
the copied alias at frame5400/clock1516754627. Its mask3 is actual alias-read
coverage during the transition, with all guest RAM/clock/RNG states matching;
no DMA/routine-PC attribution is inferred from this per-frame observer.
The8476-frame comparison produces18599928mixed/PSG samples,2236876per FIFO
and18578590HLE samples per machine. Protected input/cache/loader hashes match.

Seven first/cached observations through the production helpers pass:

- First median32.415695ms/max34.267605ms after source read: source decoder
  median6.881894ms, preparation11.125100ms, activation13.941984ms.
- Cached median21.018705ms/max21.626677ms: source decoder6.912662ms,
  preparation0.195377ms, activation13.886811ms.
- Source-file read medians1.478829/1.292860ms are separate. These local reads
  have the host page cache available, not a physical cold-storage guarantee.
- Owned resources432B. Independent seven-pair whole-process peak RSS medians
 182652/183324KiB (legacy/candidate), maxima182772/183524KiB: median delta672KiB,
  maxima delta752KiB, both below2MiB. Each wrapper has exactly one child to avoid
  cumulative child `ru_maxrss`. Whole-process median wall224.294/245.942ms is
  separate from the after-ROM-read helper budget; it includes the native loader.
- One generation:1146logical file bytes,12288allocated bytes including its
  directory. A complete one-generation managed root, including selection/lock
  and ancestor directories:1164logical bytes/32768allocated bytes, four files.
  No old/orphan generations exist in these new measurement roots; none of the
  previously retained datasets were deleted. The seven-root totals and all
  observations remain in startup-results.json, not presented as one dataset.

These costs pass the stated native Linux gates. GUI/audio callback behavior
and native Windows remain separate criteria; actual callback lifecycle is
covered below.

### Windows GNU through Wine

The same three new differential checks pass240/3530/8476frames independently
against that platform's legacy-read instance and existing TinyCC ELF object.
Every full state/image/OAM/dispatch/audio comparison passes; mixed/PSG/FIFO/HLE
sample totals match the native Linux observations, although this does not claim
cross-platform machine/dispatch identity. The copied alias is actually read at
frame5400/clock1516754627, mask3; primary first read541/clock151880965.
The8476-frame pair executes47977490native blocks and91433515fallback steps.
Existing Wine D-Bus background diagnostics also occur here; both affected
checks terminate with an actual passing result, without a production workaround.

Seven first/cached observations pass their predefined **median** limits:

- First median109.222100ms/max194.175200ms after source read; component medians
  source decoder25.476500ms, preparation28.404500ms, activation56.181100ms.
- Cached median91.891400ms/max183.130300ms; source decoder25.546000ms,
  preparation10.462300ms, activation56.038600ms. The maximum exceeds100ms;
  the preselected100ms budget is for the median, not every launch. Retain this
  variability rather than rerunning until it disappears.
- Separate source-file read medians37.396000/36.691600ms, owned resources432B.
  Generation files1146logical/12288allocated bytes; complete managed root
 1164logical/32768allocated bytes on Wine's Z drive backed by the Linux host
  filesystem. No native Windows disk, peak RSS or frame-time claim follows.

Evidence: windows-{worldmap,enemy-menu,battle-entry}-full-state-audio.log,
windows-scene-tests.json (protected input hashes), startup-windows/startup.toml
and windows-startup-results.json under the same ignored validation directory.
Native and Wine test builds are warning-free. The new tests are opt-in; closed
production suites and existing boot/matrix/lifecycle gates were not repeated.

### Package selection

The portable whitelist now includes `docs/rom_data.md` and
`docs/rom_data_runtime.md`. A synthetic tar.gz/zip test plants private markers
at both repository/runtime roots in Data/resources, other resources, native
cache, generated code and ROM/save/state/recording neighbors. Both archive
formats include the selected runtime/help bytes and exclude every marker.
This validates selection without copying private game data; it is not an
actual new release export or archive GUI check.

## Reproducing the optional private checks

`rom_data::validation_tests::decoded_base_matches_recorded_scene_at_every_frame`
uses `FFTA_STATE_TEST_ROM`, `FFTA_STATE_TEST_NATIVE`,
`FFTA_STATE_TEST_INPUT` and an explicit `FFTA_STATE_TEST_FRAMES`. Select either
`FFTA_STATE_TEST_STATE` (existing desktop/raw snapshot),
`FFTA_STATE_TEST_SAVE` (read-only cartridge save), or neither for clean boot.
The same transactional loader/input/native loop is used for both machines.
`FFTA_TCC_LIBRARY` selects the corresponding TinyCC loader; the Windows cache
is an ELF object despite its DLL filename. Set widescreen/zoom explicitly.
Run this ignored release test with `--exact --ignored --nocapture
--test-threads=1`. The existing600-frame boot test remains a separate closed
check and need not be repeated to run this one.

The recorded-scene check compares the untouched loaded states first, then arms
both audio shadows/taps identically after loading and clears accumulated host
output equally. Every subsequent state includes guest CPU/clock/RNG/memory;
central/wide images, OAM, dispatch and mixed/PSG/FIFO/HLE output are also compared
explicitly before draining. HLE samples compare floating-point bits, and every
stream must contain actual samples. Source-mask changes log the first completed
frame/clock. `FFTA_STATE_TEST_REQUIRE_SOURCE_MASK=3` additionally requires reads
from both primary and copied source ranges; a missing range is a coverage
failure, not proof of initializer execution.

`rom_data::validation_tests::measure_decoded_startup_costs` takes the private
ROM and a **new** `FFTA_STATE_TEST_COST_ROOT`. It saves all14observations in
`startup.toml`, retaining seven independent generation roots and their cached
reuse. Source-file reading is timed separately; machine construction (common
to both paths) precedes the source decoder/preparation/activation clock.
Explicit-root lookup and progress-console formatting are outside those helper
timings; independent whole-process startup/RSS observations include normal
startup behavior. Report every value/max and evaluate the preselected medians;
do not delete earlier outputs or rerun an exceeded budget without a reason.

## Actual GUI and callback lifecycle

ROMDATA-05-OBJ-GUI is complete for local EUR Linux/Wine coverage. Source-verified
production candidates and previously validated immutable datasets were reused;
no runtime rebuild, extraction, benchmark or recording was needed. New isolated
config/save/cache roots import the protected frame8476battle slot read-only.

Linux with native guest translation, and Windows/Wine with native translation
and interpreter, each pass actual Slot Load, held Fast/release, toggle, pause/
Return-resume keeping Fast, return to1x, Undo load, reload, resumed battle and
clean WM close. Captures show about4:1 INT/EXT in Fast and about59.7/59.7at1x.
Native dispatch uses each platform's existing hash-protected TinyCC ELF object.
Positive decoded usage (guest/typed host ROM/typed host RAM):14640/27061/222040
Linux;15744/29102/238784Wine native;14874/27473/225589Wine interpreter. No
regeneration or dynamic-RAM fallback occurs; original resource and slot hashes
remain unchanged. These counters are cumulative session use, not per-phase
trace or instruction timing. Native Linux's smaller virtual screen clips the
wide window right edge; inspected FPS/menu/battle establish these controls,
while complete-image equality is the already closed headless criterion.

Actual callbacks run at44.1kHz Linux/48kHz Wine with final volume0. Playing,
paused and Fast contexts all have callbacks; Fast382/1634/1631respectively.
All sessions report0device errors, callbacks over period and producer drops.
Fast inclusive callback p99 is0.836176/0.265100/0.256200ms. The whole-session
profiles include transitions: Linux playing has17callbacks with faithful holds,
and Fast waiting device frames11290/7680/7200are retained, with0rebuffers and
maximum estimated source queues156.616/122.350/114.466ms. This is actual
callback/lifecycle observation, not steady-play zero-starvation, an audio
quality judgment or an A/B callback-performance claim.

A separate Wine/native GUI backup check observes Cancel selected by default,
explicit Restore and restart, success feedback, exact startup save bytes while
paused, Resume into clean boot and clean close. Backup bytes/order do not
change, slot/source save hashes stay protected, and decoded resource usage
survives the reset. Three stream constructions report0device errors. Closed
file-level Load/Undo/backup retention tests are reused, not rerun.

Private evidence: `target/rom-data-gui/verification-summary.json`, per-mode
result.json, measurement-inputs.json, play logs, inspected PNG captures and
helpers. Every process52685/14957/14446/26083is terminal. The observed controls
close this bounded GUI gate. Actual archive evidence follows below; USA
initializer, native Windows and all other families/mods remain open.

## Actual portable archives

ROMDATA-05-OBJ-ARCHIVES is complete for the declared container/host scope.
The packager required rom_data.md/rom_data_runtime.md, but the Docker context
allowlist and COPY lists omitted them. Both recipes now include those explicit
files. Frozen input hashes identify the actual worktree rather than assuming a
commit contains all uncommitted implementation files.

Both real builds/test/exports pass:392Windows GNU/Wine and395Linux tests,
24optional ignored per platform. No compiler warnings occur; cold Windows apt
setup reports only absent optional man-page links. Exported zip/tar files pass
integrity, safe path/member checks, executable modes and public source/document
hash checks. No Data files, private game/resource/save/recording/cache/code
members or matching known private content hashes occur, including inside the
TinyCC source zip. Windows has only the intended libtcc DLL, PE resources and
no unbundled GCC/C++/pthread runtime imports. Linux maximum required GLIBC is
2.35and host ldd resolves its libraries.

The actual extracted executables automatically find the bundled TinyCC, create
portable Data/resources with no resource override, load the protected battle
slot, Undo/reload/resume and close0. Windows guest/host ROM/host RAM resource
reads3084/5875/46774; Linux3072/5870/46592. Captures are inspected. All original
fixtures and public archive-member hashes match; private test caches/data are
added only after archive inspection, while zip/tar bytes remain untouched.
This validates these artifact bytes, not the old host candidate by substitution.

The post-build preservation guard finds one prior BUILD-INFO.txt hash changed
outside the selected export path. All prior archive bytes and both canonical
runtime hashes are unchanged; current Linux metadata matches the identical
archive. The cause is unestablished and an exact old preimage is unavailable;
retain this exception rather than inventing old ldd ASLR addresses. Both build/
export stages had already passed, so they are not rerun. The first scanner also
mistook a harmless empty TinyCC source file for the empty prepare.lock payload;
excluding that non-asset identity and byte-checking the existing extraction
completes the failed inspection without another build/unpack.

Evidence: `target/rom-data-archives/` build-results.json/source hashes,
windows/linux-build.log and test-counts.json, both docker-* archives/BUILD-INFO,
platform-inspection.json/PE dumps, extracted runtime result/measurement JSON,
logs/PNG captures and verification-summary.json. Windows archive SHA-256
88ea78fc5c03fe5dc3c073086b2f18d7b7106ed08cf2d5bb705a3c2d5f3b5b94;
Linux08726f7fcea11f0ed4bfd967b70eefdee49693fb09a44a307d0ddd968f44904c.
Handles4102/48361/6979are terminal. USA initializer/native Windows still gate
normal enablement, and the full remaining resource matrix/mods stay open.
