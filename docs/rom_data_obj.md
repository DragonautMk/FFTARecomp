# European OBJ lists and animation banks

EUR e63-d62adds [conditional alternate script-special poses](rom_data_unit_obj.md#conditional-script-special-alternate-managed-runtime):
59files58resources, five actual dependencies, all57prior records+decoded files/
e62retained/USAe8-d7. Typed owner serves only20newspans816B with shared initial/
normal/alternate owners and rawholes preserved. Two new Wine filters pass31guards/
lifecycle/all816typedB and24original-interpreter-compiled fullstate-clock paths/
20edges/BIOS-SWI-DMA/mixed2new4alternate+2new2raw. ActualproductionCLIcreate/reuse
59generation61managed/all59matchaccepted/no-write and oneframeoldcacheattachment
pass(zero newuse/smokeonly). Ordinarynewpixel/equivalence and newLinux/USA/native
Windows/32:9 remain pending.
Source/model/previous54cell/sourceDMA/oldcommands2-8proofs stay closed; no
performance/storage measurements.

The separate [normal-mode battle pose source model](rom_data_unit_obj.md#normal-mode-battle-poses-source-projection-and-typed-model)
now decodes2581sequences/17894frames over122normal-mode IDs, including real nulls,
parts1/2 and command-dependent fields. New6021spans1455474B exclude byte-identical
initial/equipment views. One new Wine source oracle passes constructor/reset/
command/scheduler-DMA checks, including all1063200newpixelB; prior acceptance
is not replayed. EUR e60-d59now publishes and consumes the new typed ranges:
56files55resources/all54old resources/e59retained/USAe8unchanged. Two reduced
Wine filters pass17guards/lifecycle/all1455474newtypedB and32original/interpreter/
compiled full-state-clock paths with mixed older-owner/raw reads. Actual
production CLIcreate/no-write reuse56generation/58managed files and one existing
F09Wait3530ordinary fullstate-clock-image-OAM-dispatch-allaudio pair now pass
842236newaggregateB/815360pixelB/fivesequences. Other2446new sequences retain
targeted coverage; alternate/script/live-stream pairings and new Linux/USA
checks stay open, without performance measurements.

The normal unit graphics path now has a separate [initial-pose source model](rom_data_unit_obj.md):
121 producer IDs,477 frames and135168 decoded4bpp pixel bytes. EUR e57 now
publishes and consumes this family; bounded source/model and managed/native
Wine criteria pass. Production CLI create/no-write reuse and a positive320-frame
Party-unit full state/image/audio pair now pass too. Other poses/parts/IDs remain open. The separate
[Color-Mode unit/UI palette source model](rom_data_color_palettes.md) now covers
30spans5750B with original Wine selector/copy proofs. EUR e58-d57 now publishes
and consumes typed colors/selectors with targeted lifecycle/native-state-clock
checks passing. ActualCLI create/no-write reuse and a positive ordinaryLutia1700
fullstate-clock-image-audio pair pass1102aggregate paletteB/fourfragments. This does not extend the
custom UI catalogs described below.

The separate [equipment pose model](rom_data_unit_obj.md#equipment-managed-publication-and-runtime-reads)
now publishes and consumes748cells/258sequences/593spans110666B in EUR e59-d58,
including real nulls and non-transfer commands. Two reduced Wine filters pass
lifecycle/alltyped bytes/26original-interpreter-compiled state-clock paths.
All53prior resource records/e58 remain intact. ActualCLI create/no-write reuse
55generation/57managed files passes; Sheep1000/Fight700 are equipment-negative,
no fullpairs. One authorized copiedEURPolly→Hildegard confirmedFight500fullpair
passes allstate-clock-image-OAM-dispatch-audio/4477equipmentB/1408pixelB/twosequences;
other256ordinary sequences targeted-only. No metrics.

`rom_data::obj_lists` defines the next bounded sprite resource family after
OBJ dimensions. The current catalog contains 73 static lists (173 objects, 1184 source bytes),
independently rooted by 123 direct literal references to EUR draw wrappers.
The original e14 catalog contained 66 lists; e16 adds seven lists and sixteen
references without changing any original record.
`obj_lists/roots.rs` records reader, literal, call, wrapper and source offsets;
it contains addresses, not game instructions or asset bytes. The local source
inventory and disassembly remain under ignored `target/rom-data-obj-lists/`.

This is the declared direct-literal catalog, not the complete sprite archive.
Indirect/table-selected lists, animation frame indices/timing, sprite graphics,
palettes and lists assembled in RAM remain separate migration work. No USA
offsets or catalog equivalence are inferred from EUR.

## Format and original consumers

Each uncompressed source has a little-endian 16-bit count and exactly six bytes
per object, with no alignment padding between records. Source offsets are stable
list identities; objects retain their original ordering. The readable JSON
splits the packed attributes into signed 9-bit X and signed 8-bit Y, affine and
object modes, mosaic, colour depth, shape, all five transform bits, size, tile,
priority and palette. Transform bits retain either affine-index or flip/reserved
meaning according to the object's mode. No normalization discards packed bits.
The source projection reconstructs all three original halfwords exactly.

The generic decoder checks alignment, complete count/extent, signed-header
contract and offset overflow. Catalog validation rejects overlapping/unordered
lists and invalid field widths. The declared EUR source catalog uses only
nonnegative counts and shapes 0..2; negative-count and unavailable-shape behavior
outside these sources stays with the original readers.

`Catalog::from_rom` requires the exact EUR digest. It independently rechecks each
root's PC-relative literal instruction, literal pointer and original Thumb call
target. Each list has its own source extent/hash. `verify_source` compares the
whole typed catalog to that source, so a structurally valid edit or omitted list
is rejected even with an updated JSON hash.

Original EUR draws at `08001500` and `0800170c` read counts and packed records.
They retain signed arithmetic, clipping, attribute masks and OAM capacity, and
write only the first six bytes of each eight-byte OAM entry. Matrix halfwords
remain untouched. EUR draw A loads the initial output index into r0 before its
early capacity branch; both EUR routines return that index when already full.
The older USA routine document does not establish EUR's early-return result.

## Managed preparation and runtime

EUR schema 1/extractor 14/decoder 13 adds `obj-static-lists.json` with resource ID
`graphics.obj-static-lists` and a dependency on `graphics.obj-dimensions`.
There are 13 files including the manifest, with 12 resources. All eleven earlier
decoded file bytes and previous e13 generations are retained. USA remains on its
existing e8-d7 contract pending separate verification.

The fixed filename, strict typed JSON, source provenance, dependency and complete
file set are authenticated before attachment. File hashes contribute to the
whole dataset identity. The JSON input guard is 4 MiB; this is a malformed-input
bound, not a space target. Existing atomic publication, reuse, repair, relocation
and transactional activation apply to the new file.

An immutable session owner serves original source bytes from the typed count and
attribute fields through CPU/DMA/native bus reads on declared source pages.
Crossing reads merge with the prior resource view, including the immediately
adjacent name-keyboard navigation widths. No producer, instruction, DMA or guest
cycle is skipped. The same owner supplies the host sprite draw, visibility and
battle descriptor readers; lists in RAM and other ROM lists retain their live
read path. Animation state, OAM and matrices remain guest state. Save and native
translation identities/ABI are unchanged, and the owner follows existing
Load/Undo/backup/restart handling without file I/O.

`ROM_DATA_USE resource=graphics.obj-static-lists` reports bus reads, count and
attribute bytes and host list reads. These are consumption counts; no performance
measurements are made for this work.

## Verification

The offline boundary passes two reduced EUR Wine checks: packed-bit/extent
guards and all 66 lists against both original draw producers. There are 1056
original calls and 253953 interpreted instructions, covering clipping, reflection,
signed wrap, capacity and early return, complete OAM buffers, matrix halfwords
and guards. Deterministic strict JSON and exact source hashes also pass.
Evidence: `target/rom-data-obj-lists/windows-core.log`, `windows-results.json`
and `windows-europe/result.json`. The initial test-only early-return expectation
was corrected against EUR disassembly; failed evidence is retained.

Three focused EUR Wine integration checks pass: 13-file publication with injected
write interruption, no-write reuse, repair/move/e13 retention, coherent-edit,
source/metadata/input guards and transactional activation; eight original/native
calls with all 66 raw sources altered, complete state/clock equality and 301
compiled source bytes; every edge/width/mirror/real-BIOS bus path, CpuSet and DMA,
eight host draw/publication cases, and Load/Undo/backup/restart with temporary
resource files unavailable. An initial iterator lifetime compilation error was
fixed before execution; the failed build evidence is retained.

One retained 397-frame EUR pub transition matches every complete state/clock,
central/wide image, OAM, dispatch and audio stream. It uses 5470 typed source
bytes (958 count, 4512 attribute bytes) and 958 host list reads. Three actual
Windows CLI processes from unrelated cwd prepare/reuse/replay with no writes on
reuse, the old native cache and the retained legacy final row. The original ROM,
slot, input, loader, cache, old dataset and canonical Windows executable remain
unchanged. Evidence: `target/rom-data-obj-lists/windows-managed-results.json`,
`windows-managed-europe/*-result.json`, `windows-scene-results.json` and
`windows-cli-summary.json`.

A supplemental core-only EUR Wine test passes 792 host draw/visibility/battle
cases across all 66 lists and three origins with every static raw source altered.
It also confirms live coordinate edits in RAM lists and noncatalog ROM lists;
these fallbacks do not count as static-resource reads. This uses a test-only
resource constructor; the closed production/native/scene checks remain valid.
Evidence: `windows-all-host-readers-result.json` and `windows-all-host-readers.log`
in the same ignored evidence directory. New Linux, USA and native Windows verification, broader
GUI/archive coverage and other OBJ catalogs remain pending. No performance,
startup/RSS, space optimization or AndroidPort work is included.


## Additional direct lists in e16

EUR schema 1/extractor 16/decoder 15 extends `obj-static-lists.json` with seven
independently rooted lists: `394f34`, `3950fa`, `395102`, `399628`, `3adfc4`,
`3adfd2` and `3adfda`. They add 13 objects/92 source bytes. Sixteen additional
checked literal references include one earlier list. Typed format and runtime
source pages are reused. There are still 14 files/13 resources; every earlier
decoded file except the expanded static-list file is byte-identical, including
the animation banks. All original 66 typed records and e15 generations remain.
USA stays e8-d7. Save/native identities and ABI are unchanged.

Nonserialized per-source guest/host flags supplement aggregate consumption
counts. `ROM_DATA_OBJ_SOURCE source=... guest=... host=...` reports touched
sources, and the new scene gate requires the same newly added list in both
paths. An old list cannot satisfy that requirement.

Three focused EUR Wine checks pass publication/injected failure/reuse/repair/
move/e15 retention, strict JSON/source/coherent-edit rejection, complete-set
identity and transactional attachment. Fourteen original/interpreter/native
calls cover both draw producers for all seven new lists with only those raw
ranges altered: complete states/clocks agree, with 236 compiled source bytes,
33 native dispatches and 4497 original instructions (coverage counts only).
Fourteen host draw/publication cases and all new source edges/widths/mirrors,
real-BIOS bus, CpuSet/DMA and unavailable-copy Load/Undo/backup/restart pass.
Old 66-list and ten-bank source oracles are retained without repetition.

The first 240-frame battle, 240-frame Party roster and 320-frame character-menu
locators use no new list through both paths; they are retained coverage-negative
observations. Callsite inspection instead identifies the EUR boot language
selection. One 240-frame clean-boot Wine pair matches every complete state,
clock, central/wide image, OAM, dispatch and audio stream with positive guest
and host use of `3adfc4`, `3adfd2` and `3adfda`. It runs 94105 native dispatches
with no fallback. The actual current Windows CLI prepares, reuses without file
writes and runs this scene from an unrelated working directory with the old
native cache; its final language-screen image is inspected. Source ROM,
fixtures, prior datasets, loader/cache and canonical executable are unchanged.

Windows test/production builds are warning-free. Evidence is under ignored
`target/rom-data-obj-extra/`: `windows-managed-results.json`,
`windows-managed-europe/*-result.json`, `windows-cli-{initial,unit,language}.json`,
`windows-language-scene-result.json` and `windows-build.json`.
New Linux/USA/native Windows, broader scenes/GUI/archives and remaining
switch/indirect source roots stay open. No performance, startup/RSS or space
campaign is included.


## Indexed UI animation banks

A separate EUR catalog, `rom_data::obj_animations`, locates ten pairs of
uncompressed object-list and animation banks through 30 independently checked
constructor literal pairs. There are 167 lists/284 objects, 95 sequences/184
frames and 282 disjoint consumed source ranges (4012 bytes). These high-ROM
banks are separate from the earlier 66 direct-literal lists.

Each bank starts with relative u32 offsets. The first offset bounds the declared
index table; the records are independently checked for alignment, ordering,
complete extents and overlap. Lists reuse the exact typed OBJ fields above.
Sequence records contain a u16 frame count followed by ordered u16 object-list
IDs and u16 durations. All frame IDs resolve within the paired list bank.
Zero durations retain the original immediate-advance behavior; the duration word
is not narrowed to the live task's wrapping u8 tick counter. Structural guards
reject invalid links, missing records, overflow, overlapping ranges and counts
outside the original task frame-count width. Whole-source verification also
rejects a structurally valid, coherently rehashed content edit.

Original constructors `0804e494`, `0804e4e4` and `0804e558` initialize the 28-byte
RAM task, selecting a sequence from its index and retaining the original list
bank pointer. Original scheduler body `0813f814..0813f9a6` selects the current
list, draws it and advances the live counter/index. Loop mode 1 wraps; the
other observed mode retains the last frame. Drawing mode 0 uses the 128-entry
IWRAM bank at `03000030`/count `03000020`; mode 1 uses the distinct 48-entry bank
at `03002c58`/count `03002c50`. These are original code entry points and RAM
contracts, not copied instructions or host replacements.

Three sequence records are empty. Their source headers and original constructor
results are retained. Their declared direct-root callers exclude these entries,
as classified below; indirect or changed live task selection remains unverified. The extractor
does not invent a frame or silently remove their index entries. The original
runtime still handles live tasks, including any accesses outside a declared
empty record, through its unchanged bus behavior.

Two reduced EUR Wine offline checks pass: strict source/JSON and malformed-input
checks, independently generated catalog comparison, 115 original constructor
calls and 2292 original per-object scheduler ticks. All nonempty sequences,
per-bank mode 1 and final-frame hold samples match expected task fields, full
OAM buffers, matrix halfwords and guards. The 1123717 instructions are coverage
counts only. Evidence: `target/rom-data-obj-remaining/windows-results.json` and
`windows-europe/result.json`. The whole scheduler's scene-specific tail is not
claimed by this body-level oracle.

### Managed UI banks and direct reads

EUR schema 1/extractor 15/decoder 14 adds fixed `obj-animation-banks.json`, ID
`graphics.obj-animation-banks`, with an OBJ-dimension dependency. All 14 files/
13 resources are authenticated before activation, and every file contributes to
whole-set identity. Earlier e14 generations and all twelve earlier decoded files
remain unchanged; USA retains its prior e8-d7 contract. The 4 MiB JSON guard
bounds malformed input and is not a space target.

Immutable typed index offsets, list counts/fields, sequence counts, frame IDs
and duration words serve CPU/DMA/native reads on their declared pages. Host draw,
visibility and battle descriptor readers resolve the new lists through the same
retained owner; RAM and other ROM lists preserve their live fallback. Original
constructor/scheduler instructions, task counters, OAM writes and guest clock
are unchanged. The resources remain outside save serialization and retain the
existing Load/Undo/backup/restart ownership and save/native ABI.

Three focused Wine integration checks pass publication/injected interruption,
reuse/repair/move/e14 retention, coherent-edit/source/path/metadata/input guards,
whole-set identity and transactional attachment; 49 original/interpreter/native
constructor/body calls with all 282 raw ranges altered and equal complete states/
clocks. Compiled reads consume all six source kinds: 192 index, 93 list count,
618 attribute, 4 sequence count, 92 frame-list and 94 duration bytes. All source
edges/widths/mirrors/real-BIOS bus, CpuSet/DMA, ten-bank host publication and
unavailable-copy Load/Undo/backup/restart checks also pass. Evidence:
`windows-managed-results.json` and `windows-managed-europe/*-result.json` in the
same ignored directory. Production/test Windows GNU builds are warning-free.

The new positive-use scene gate rejects the retained 240-frame battle sample:
every state/clock/image/OAM/audio comparison matches, but this family has zero
reads there. This is a coverage limitation; the closed full bank/native checks
remain valid. A bounded synthetic Start input from the retained worldmap selects
an actual Party-highlighted menu, inspected in the final frame. One 160-frame
pair matches every complete state/clock, central/wide image, OAM, dispatch and
audio stream. It consumes 36513 typed source bytes: 10244 index, 3200 list count,
14747 attribute, 1922 sequence header, 3200 frame-list and 3200 duration bytes,
plus 3200 host list reads. Three actual Windows CLI processes from unrelated
cwd prepare/reuse/run this same scene with the old native cache and no writes
on reuse. Original ROM, snapshots, inputs, loader/cache, e14 files and canonical
runtime remain unchanged. Evidence: `windows-menu-scene-result.json`,
`windows-cli-summary.json` and the inspected `worldmap-start.png` under the same
ignored evidence directory. The synthetic input is a new local fixture; no
personal recording is changed.

New Linux/USA/native Windows, broader GUI/archive coverage and the remaining
unknown source/caller contracts stay pending. No performance/startup/RSS/space
campaign or AndroidPort change.


## Zero-count hold bank in e17

A bounded switch-path discovery independently roots object bank `b4d8e8`
and sequence bank `b4f4e8` through constructor calls `080332ca`/`080332f0` to
`0804e494`. This eleventh pair adds one list/two objects and five zero-count
records. The actual callers select sequence zero with repeat mode zero.
Following independently rooted banks bound the enclosing gaps; the broad CFG
experiment remains incomplete and is not a catalog-completeness proof.

Original code retains a zero header count and still reads a selector at
`source_offset + 2`, then a duration at `source_offset + 4`. In nonrepeat mode,
the index stays zero after each duration boundary. All five selectors resolve
to list zero. The first four durations alias the next zero-count header; the
last aliases the first halfword of the next bank's relative-index table (32).
These are actual reads, not an invented animation frame or source padding rule.

`Sequence.zero_count_hold` explicitly retains `object_list` and
`duration_source_offset`; `frames` stays empty. The optional field is omitted
for the previous ten banks, keeping all earlier typed records unchanged.
New selectors add two owned bytes per sequence. Duration references must resolve
to a typed sequence header or the start of a typed sequence index; they do not
create overlapping owners. Invalid selectors, aliases, mixed nonzero headers,
missing owners and source-inconsistent edits are rejected. The three earlier
empty records have no proved hold interpretation; the bounded direct-root caller
classification below rejects extending this field to them.
Repeat mode one and other caller preconditions are unverified.

EUR schema 1/extractor 17/decoder 16 retains all fourteen files/thirteen resources
and e16 generations. Only animation-bank JSON and its manifest entry expand;
all other decoded files, including the 73 static lists, are byte-identical.
Current animation totals are 11 pairs/32 literal references, 168 lists/286 objects,
100 sequences/184 counted frames, 290 disjoint source ranges/4070 source bytes.
The new pair owns eight ranges/58 bytes. Its final duration reuses two bytes
already authenticated by the following bank. Save/native ABI and original
instructions/clock remain unchanged. New nonserialized
`zero_count_selector_bytes`/`zero_count_host_reads` diagnostics and the scene
gate distinguish this bank from old-bank activity.

One focused EUR Wine original-code check passes 14 constructors and 104 ticks
across the two actual caller parameter sets and all five sequence IDs in both
per-object scheduler bodies. Exact task/held-index/elapsed-reset values,
selector/duration read addresses, complete OAM/matrices and guards match.
The 51974 original instructions are coverage only; the whole scheduler tail
is not claimed by the body oracle.

Two reduced EUR Wine integration checks pass strict source/JSON and new-field
malformation/coherent-edit guards, transactional attachment, interrupted
publication/create/reuse/repair/move/e16 retention and unavailable-copy
Load/Undo/backup/restart. Eight constructor/body interpreter/native comparisons
with all new ranges and the shared duration halfword altered match complete
state/clock: 60 compiled source bytes include 12 held-selector bytes. Durations
are attributed to their header/index owners. All new edges/widths/mirrors,
real-BIOS bus/CpuSet/DMA and new-list host publication match. The previous
14-constructor/104-tick proof and closed ten-bank/73-list oracles are retained.

One 240-frame EUR Wine worldmap-to-Party pair matches every complete state,
clock, central/wide image, OAM, dispatch and audio stream with 288 new-selector
bytes and 288 new-bank host reads. Three current Windows CLI processes prepare,
reuse without writes and run the same retained input from unrelated cwd with
the old native cache; the final Party roster image is inspected. Windows test/
production builds are warning-free, canonical executable restored, and original
ROM/snapshots/input/e16 dataset/cache/loader unchanged.

Evidence: ignored `target/rom-data-obj-zero-count/` contains
`windows-original-result.json`, `windows-original-europe/result.json`,
`windows-managed-results.json`, `windows-managed-europe/*-result.json`,
`windows-cli-summary.json`, `windows-scene-result.json` and `windows-build.json`.
Constructor-fed roots at `38008`/`88328` are now resolved below; OBJ graphics/palettes,
GUI/archives and wider scenes remain separate work. New Linux/USA/native
Windows verification remains pending. No performance/startup/RSS/space campaign.


## Language-selected UI banks in e18

The original EUR caller `08038008` selects object and sequence roots through
five-entry tables `3955c4` and `3955b0`, indexed by the original language byte
at `02001f6e`. Three selector-to-constructor paths call `0804e494`/`0804e558`.
Each literal and load/shift/add/word-load chain is independently checked;
the five language IDs come from the established EUR selector, without clamping
other values or inferring USA equivalence. Another previously unresolved call,
`08088760`, references the already migrated `b4bc98`/`b4bd24` pair.

`Banks.selector_tables` retains table offsets, selector address and ordered
object/sequence ROM-offset arrays; runtime projection restores original ROM
pointers. Every pair must resolve to a typed bank, cardinalities/alignment/source
extents are bounded, and duplicate/overlapping tables or unmatched links are
rejected. Coherently rehashed language reordering, payload edits or omitted tables
fail source authentication. Guest CPU/DMA/native table reads and indexed-bank
reads use typed values; live language/task/tick/index and original instructions/
clock remain. Host draw readers use the same list owner. New nonserialized
`localized_selector_bytes`, `localized_source_bytes` and `localized_host_reads`
counters and a positive scene gate distinguish this family from older banks.

Five pairs add 149 lists/574 objects, five nonempty sequences/144 frames and
164 bank ranges/4944 source bytes, plus two selector ranges/40 bytes. EUR e18-d17
has 16 pairs, 33 direct literal references and three indexed selector paths,
317 lists/860 objects/105 sequences/328 frames, 456 source ranges/9054 source
bytes. All eleven previous typed bank records, other decoded files and e17
versions remain unchanged. There are still 14 files/13 resources; USA remains
e8-d7. No save/native ABI or translation-cache revision changes.

The final frame in each new sequence has u16 duration 256. The original wrapping
u8 counter never reaches that comparison, so the final image holds while its
counter wraps. Narrowing the duration to u8 would change that behavior. One
reduced EUR Wine source check passes 15 original selector chains, ten constructors
and 1589 scheduler-body ticks: strict JSON/source/shape/link guards and independent
inventory, all new list fields/frame transitions, exact task/OAM/matrices/clipping
and final hold/wrap match. Its 1657211 instructions are coverage only. Initial
Windows compilation exposed a closure-index type inference error, fixed before
execution; the failed build is retained.

Two focused EUR Wine integration checks pass publication interruption/create/
reuse/repair/move/e17 retention, all old-record/other-file equality, coherent
selector/list/missing-table rejection, transactional activation and unavailable-
copy Load/Undo/backup/restart. Thirty original-selector/constructor/body calls
across five language IDs and all three paths match complete state/clock under
interpreter and native execution with all 166 new ranges altered. Compiled reads
include 120 selector bytes/210 bank bytes. New edges/widths/mirrors/real-BIOS bus,
CpuSet/DMA and five bank-specific host decode/publication cases pass. The closed
15-selector/1589-tick and older bank/static-list oracles are retained.

The old 3530-frame battle and initial directional worldmap locators have zero
family use; they are retained coverage-negative observations, with no expensive
differential or repeat. Inspection of linked graphic data identifies “Engage!”.
A bounded synthetic input from the retained worldmap state uses the existing
semantic mouse control to point at Lutia Pass, then physical A to confirm. The
frontend queues original coordinate stores; the fixture does not write guest RAM.
One 400-frame EUR Wine pair matches every complete state/clock/central-wide image,
OAM, dispatch and audio stream with 8 table bytes/5906 bank bytes/228 host reads.
The final “Engage!” image is inspected. Current Windows CLI processes create,
reuse without writes and run from unrelated cwd with the old native cache;
original ROM/snapshots/inputs/e17 dataset/loader/cache/canonical are unchanged.
Windows test/production builds are warning-free.

Evidence is under ignored `target/rom-data-obj-indirect/`:
`localized-bank-inventory.json`, `windows-original-result.json`,
`windows-original-europe/result.json`, `windows-managed-results.json`,
`windows-managed-europe/*-result.json`, `windows-cli-summary.json`,
`windows-cli-engage-confirm.json`, `windows-scene-result.json` and
`windows-build.json`. New Linux/USA/native Windows verification, other ordinary
scenes/languages, GUI/archives and other source roots remain open. The three
earlier empty-record direct-root caller domains are classified below; indirect
selection and changed live task contexts remain unverified. The linked compressed OBJ graphics table
`395588`/3072-byte transfer is subsequently migrated under e19 below. No
performance/startup/RSS/space campaign is included.


## Custom OBJ word banks in e19

EUR schema 1/extractor 19/decoder 18 adds fixed `obj-graphics.json`, resource
`graphics.obj-custom-banks`, format `obj-custom-banks-v1-json` with an
OBJ-dimensions dependency. It contains 39 raw BE-size custom LZ/RLE banks and
six ordered five-entry language selector tables. Thirteen checked direct
source/upload references root nine banks; canonical indexed paths at
`32852/3809c/3286e/4fcee/89f02/89f48` root thirty more through tables
`395574/395588/39559c/397d1c/397d30/397d80`. Original decoder `080054a0` and
CpuSet/FastSet wrappers `08145230/0814522c` independently establish this scope.
The inventory owns 45 disjoint ranges/62674 source bytes and produces 131872
bytes. There is no overlap with earlier managed resources.

Exact little-endian u32 words retain every decoded bit and internal mixed
subregion. They are not claimed universally pure pixels or converted to RGBA.
The shared bounded codec preserves exact consumed source metadata, validates
whole-token output extents and rejects invalid history, truncation, trailing
bytes and mismatched words. Selector offsets are aligned, ordered and linked
to existing banks. Strict JSON rejects unknown fields; source authentication
also rejects coherent encoded/decoded edits or reordered/missing selectors.
The 32 KiB decoded-bank and 128 KiB source guards and 16 MiB file read guard
bound malformed input; no space optimization or acceptance budget is added.

The immutable runtime maps each literal source position to a byte in decoded
words and each selector position to its typed ROM pointer. Headers and tokens
come from authenticated encoded metadata. Exact fixed-page bindings merge with
previous resource owners, including the adjacent old selector at3955b0.
CPU/DMA/native and guest-BIOS bus reads share this view. All original decoder,
RAM writes/transforms, transfer and clock work remains. In particular source
3b0ab8 produces1280bytes but its caller copies3072bytes: the1792-byte tail stays
live RAM. Sources b7f874/b80140 produce7744bytes but upload only5056bytes;
other paths transform scratch or copy an offset/subrange. Those are not baked
into files or replaced with a full-bank upload. PPU and sprite rendering read
live VRAM after original uploads, with no fabricated host ROM consumer.

All fifteen files/fourteen resources contribute to whole-set identity and are
source-verified before transactional attachment. Every prior decoded file is
byte-identical, e18/older generations remain and USA stays e8-d7. New atomic
counters (`bus_reads/source_bytes/decoded_literal_bytes/selector_bytes`) are
session diagnostics, excluded from saved machine state and native ABI.

One reduced Wine original-source test matches39original decodes,30actual
selector chains and6sampled full/prefix/offset/live-tail CpuSet/FastSet copies
(1471897instructions, coverage only). It verifies exact original r7 source
cursor, returned size, output, scratch guards and whole VRAM. It does not
claim every surrounding caller's scratch transformation was independently
reconstructed. Two new focused Wine integration tests pass publication
interruption/create/reuse/repair/move/e18retention, exact earlier-file equality,
coherent edits/hash/path/source guards and transactional rejection, plus
unavailable-copy Load/Undo/backup/restart.39decodes/30selectors/4live uploads
through interpreter/native match full states and clocks with all45raw ranges
altered. Compiled source/literal/selector reads are positive. All source bytes,
edges/widths/mirrors, real-BIOS bus and CpuSet/FastSet pass. Audit found an
initial DMA fixture used397574instead of395574; a separate corrected Wine
check requires exactly20new typed selector bytes and full state/clock equality.
The closed39-bank native/source campaign was not repeated for that correction.

One positive400-frame ordinary EUR “Engage!” scene matches every complete
state/clock/central-wide image/OAM/dispatch/audio sample and consumes1194source
bytes,460decoded literal bytes and4selector bytes. It reuses the retained
semantic mouse/physical-A input without direct RAM edits. The new resource's
scene gate prevents old metadata use from satisfying this acceptance. Three
actual Windows CLI processes create e19, reuse without writes and replay the
same scene from unrelated cwd with the old native cache and identical positive
usage. The final “Engage!” image is visually inspected. Test/production Windows
builds are warning-free, the canonical executable is restored and protected
ROM/snapshot/input/e18 dataset/cache/loader remain unchanged.

Private evidence: `target/rom-data-obj-graphics/` contains the independent
`custom-obj-inventory.json`, `windows-original-result.json`,
`windows-original-europe/result.json`, `windows-managed-results.json`,
`windows-managed-europe/{managed,native,dma}-result.json`,
`windows-dma-results.json`, `windows-scene-result.json`, `windows-cli-summary.json`,
`windows-engage-custom-graphics-full-state-audio.log` and Windows build logs.
The native-result aggregate DMA flag is superseded by the explicit corrected
DMA result. New Linux/USA/native Windows, other ordinary scenes/languages,
remaining OBJ graphics/palettes, unproved indirect empty-record selection and
broader GUI/archive/gameplay/mod phases remain pending. No performance,
startup/RSS or space campaign, AndroidPort change, commit or push is included.


Raw palette colors used by original OBJ/BG UI shadow copies are subsequently
migrated in [EUR e20raw UI palettes](rom_data_ui_palettes.md):21banks,
23direct/derived/Color-Mode references and2656source bytes, preserving every
original copy/fade/publication step. This does not claim all game OBJ palettes.


[EUR e21custom UI graphics](rom_data_ui_graphics.md) subsequently owns a separate
nine-bank catalog spanning eight BG sources and one OBJ source (b7a9d0, two
original upload paths). This preserves the existing39-bank OBJ resource/file
unchanged and retains original decoder-returned counts and partial/full copies.


EUR e22extends that UI catalog with seven localized banks and two five-entry
selector groups. Its second group references existing OBJ banks b7f874/b80140
through explicit authenticated owner links; obj-graphics.json stays identical.
Original5056byte OBJ and2688byte BG copies cover each7744byte decoded bank
exactly; no duplicated owner or invented tail. Both language fallbacks and
original work/clock remain. See [localized UI contract](rom_data_ui_graphics.md#localized-ui-selectors-in-e22).


## Earlier empty sequence caller domains

A bounded EUR static audit closes the direct-root caller question for the three
older empty records, without adding `zero_count_hold` or changing extraction:

- Bank0/sequence1 at `B4BD20`, roots `B4BC84`/`B4BD10`, and bank1/sequence1 at
  `B4BD34`, roots `B4BC98`/`B4BD24`: all twelve declared constructor calls pass
  constant sequence0, excluding the empty sequence1.
- Bank14/sequence6 at `B4FA10`, roots `B4F028`/`B4F9B8`: initializer `0808BBF0`
  reaches constructor wrapper `0804E558`; updates `0808CA42`/`0808CF48` reach
  `0804E494`. All eighteen direct Thumb callers of the initializer supply a
  full-word mode constant0..3 at stack argument3. Their computed initial
  selectors are0..3; the update paths select2/3 for every live flag byte. These
  sequences are nonempty. Constant setup across calls relies on the original
  compiler's callee-preserved r4..r11 convention, with explicit writes and
  stack-slot replacement checked independently.

The initializer preserves the low two mode bits and one flag bit in live RAM.
Its original full-word comparisons against2/3 select the flag bit alone; other
modes select `2*(mode & 3) | (flag & 1)`. Thus raw mode7 or65539 can select6/7.
The initializer passes repeat1, and no actual rooted producer of these modes is
established here. This is not a guest mode/index cap or a proof that the empty
records are globally unreachable. Computed pointers, indirect entry and later
RAM mutation remain outside the bounded literal/direct-call audit.

The unchanged constructors retain count0 and body pointer `header+2`. The
original scheduler reads the object-list selector there and a duration at
`header+4`; nonrepeat mode can retain cursor0, while repeat1 takes the original
zero-divisor path, whose scheduling result is unverified here. The three
possible selector halfwords are zero and unowned (six bytes total). Their
possible durations8/16/4 alias existing typed sequence-index owners at
`B4BD24`/`B4BD38`/`B4FA14`. No new ownership follows from that adjacency.

An all-byte ROM pointer census accounts for all thirty literal occurrences of
the six bank roots and finds no absolute pointer to the three empty headers or
bodies. Forty-seven original instruction fingerprints, fifteen rooted calls,
eighteen mode producers and all fifty current source owners pass the static
contract audit. The proposed hold extension is rejected for the current
source evidence; reopen only with a concrete new producer or ordinary trace
selecting an empty record with established scheduling semantics. EUR e55-d54
and USA e8-d7 remain unchanged. No build or guest execution was needed; new
Linux/USA/native Windows checks remain pending. Evidence:
`target/rom-data-obj-empty-callers/source-contract.json`, `rooted-calls.json`,
`mode-producer-bindings.json`, `pointer-census.json` and original disassembly.


EUR `v1/e61-d60` adds `unit-obj-alternate-battle.json` /
`graphics.unit-obj-alternate-battle`,57files56resources/all55prior decoded
resources and e60retained/USAe8unchanged. The immutable typed owner serves only
3321new spans410086B with actual metadata52/14/initial/equipment/normal
dependencies. All20shared index-root targets stay explicit non-sequences under
older ownership. Two reduced Wine filters firstPASS20managed guards/lifecycle/
all new bytes/606sequences and34original/interpreter/compiled complete-state-clock
paths, genuine extra-part nulls/shared-root prefixes/commands2-3-8/uploads-DMA/
all3321edges/BIOS-SWI-DMA/mixed raw-oldowner reads. StrictNull parsing rejects
extra fields while canonical valid JSON remains identical. Source/model/managed/
native and actualCLI children are closed. Actual Wine CLI creates/reuses all57
accepted generation files/59managed files without rewriting; one-frame oldcache
attachment passes with zero new alternate use. Positive ordinary alternate use,
script/live reachability and new Linux/USA/nativeWindows/new32:9 remain pending. No performance/storage campaign
or new gameplay checkpoint.
[Alternate runtime contract](rom_data_unit_obj.md#alternate-managed-publication-and-typed-runtime-reads).
