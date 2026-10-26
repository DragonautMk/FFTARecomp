# European unit OBJ source models

This bounded family covers the normal unit graphics path, separate from the
[custom OBJ word banks and UI lists](rom_data_obj.md). EUR schema1/extractor57/
decoder56 publishes `unit-obj-initial-pose.json`, resource
`graphics.unit-obj-initial-pose`, with actual metadata52/14 dependencies.
USA remains e8-d7. The later EUR e58-d57 generation retains this complete
family and adds [Color-Mode unit/UI palettes](rom_data_color_palettes.md). Other
poses, parts, sprite IDs and further palette producers remain open.
The separate [equipment pose model](#equipment-poses-source-model) follows the
item/action producer. EUR e59-d58 now publishes and consumes its typed fields;
[targeted Wine lifecycle/native checks](#equipment-managed-publication-and-runtime-reads)
pass. Actual production CLI create/no-write reuse and the authorized copiedEUR
Polly→Hildegard Fight500positive ordinary fullpair now pass below.

## Original source contract

The exact EUR image is authenticated before parsing. Original `0802D394`
obtains its sprite ID through `080233A8` → `080CF144`, stores that ID in a live
task and calls `08022788` at `0802D3F8` with full-word animation0 and part0.
`080CF144` reads property4 of the existing metadata52 job records, or halfword04
of the existing metadata14 special-unit records. These are actual producer
dependencies, not new copies of their source bytes.

The 110 declared route0 job records with a concrete ID and93 special-unit
records project121 distinct sprite IDs:0..123 except42/112/117. Four routeFF
job records follow the live alternate job. Job0/1 contain65535; their live use
and any sentinel contract are unproved, so this projection exports no graphics
from those values. Neither the producer extents nor these IDs are guest bounds.
Other IDs and later RAM changes must retain the original lookup behavior.

Original `08022174` reads a root from `08394834 + 4*sprite_id`. Animation0/part0
selects precisely its first pointer. Only that four-byte cell is owned;
adjacent actions/parts are not inferred from spacing. The result has a u16
frame count, two preserved opaque header bytes and counted20-byte frames.
This projection contains121 sequences and477 frames, all operation1.

Each frame retains a u32 pixel offset, a u32 object-list offset, a duration,
an operation and ten exact trailing bytes. `08022254` resolves the object list
at `08A50020 + object_offset`. Six lists contain31 objects in the established
counted six-byte OAM format; the model reuses its lossless attribute decoder.
Original `080221D4` sums transfer tile counts from shape and size: square
counts1/4/16/64; every nonzero shape, including reserved shape3, uses2/4/8/32.
These are transfer branches, not a new PPU interpretation. The cached count
is a u16; LSLS19/LSRS16 then computes the DMA word count without a new limit.

The source is `08898A40 + pixel_offset`, copied directly to
`06010000 + 32*live_tile_position`. `08000B14` constructs a live DMA command;
`08000800` executes the queue through the real registers. Neither function is
a decompressor. The333 distinct requests contain135168 raw4bpp bytes, decoded
into4224 tiles of8×8 palette indices. Every nibble remains intact; no RGBA
image replaces indexed data or the live palette.

The exact source plan owns415 nonoverlapping ranges/146358 bytes:484 directory,
484 first-index,10024 sequence,198 object-list and135168 pixel bytes. Only
adjacent requests with the same role merge. No gap, padding or following table
is exported. Concatenating ranges in offset order gives SHA-256
`397b721c18a26cd080f0835e056f9af411b2c3ab34cfef8bc47291ca17dd490b`.
All ranges are disjoint from the51 authenticated e56 resource owners. The
static contract passes94 instruction,13 BL and6 literal fingerprints.

## Typed model and reduced Wine validation

The [public source model](../crates/gba-core/src/rom_data/unit_obj_initial_pose.rs)
provides producer projection, bindings, sequences, typed OBJ lists, decoded
indexed tiles and shared source plan. It reconstructs every source byte.
Strict JSON rejects unknown fields; shape, ordering, links, bit widths,
alignment, overflow and physical ownership are checked. Authentication also
rejects coherent edits that satisfy the structural model. Opaque frame/header
bytes remain explicit. The source-model step was accepted under e56; the
subsequent managed/runtime integration below advances EUR to e57.

One Windows frontend test build is warning-free. One new exact Wine filter in
the [source oracle](../crates/recomp/src/rom_data/unit_obj_initial_pose_model_tests.rs)
passes on its first execution:

- All415 source spans, independent field/hash expectations, JSON roundtrip and
  eleven shape/unknown-field/coherent-edit guards.
- All121 complete original `08022E20` reset paths, plus one actual `08022788`
  constructor prefix before allocation. Predicted task bytes, untouched RAM,
  stack and callee-preserved registers match their declared endpoints.
- All477 original scheduler → operation1 → queue → DMA3 uploads. All VRAM
  matches the decoded tiles;135168 distinct source bytes are copied. RAM
  outside declared task/pool/queue fields remains unchanged.
- Sixteen original shape/size probes, including every reserved-shape3 branch.

Observed original load sites cover9393 distinct nonpixel bytes; DMA covers all
135168 pixel bytes. The1797 preserved but unread bytes in these probes are242
opaque header bytes,1431 frame-tail bytes and124 OAM bytes unused by the tile
counter. Reconstruction covers them; ordinary consumption is not established.
The245830 instructions/499 PCs are coverage only.

The frozen test executable has SHA-256
`1d759b0dd687a94364142f67422d8c4423122771b7ae741057559e08d32f37de`.
Private evidence under `target/rom-data-obj-remaining-sources/` includes
`source-contract.json`, `audit-initial-pose.py`, original disassembly,
`windows-model-build.json`, `windows-model-result.json`, `windows-model.log`
and `windows-model-europe/result.json`. These are local evidence, not portable
game fixtures. Generated game data remains outside Git/packages.

The source oracle's scope is separate from the runtime checks below. Other
selectors/palettes, new Linux/USA/native Windows and broader scenes remain
pending. There was no new gameplay fixture, navigation, performance or storage
campaign.

## Managed publication and direct runtime reads

EUR e57-d56 contains53 generation files/52 resources. All51 earlier decoded
files, resource records and coverage entries are identical; e56 generations
remain available. The new file is identical to the accepted source model,
SHA-256 `f26afe300dd906b06ffeb048f5c1d88d0ed02744f0d91be645e263acd2474bd7`.
Its dependencies are `gameplay.linked-metadata52` and
`gameplay.linked-metadata14`; producer links are checked before attachment.

The [immutable runtime owner](../crates/gba-core/src/rom_data/runtime/unit_obj_initial_pose.rs)
retains typed bindings, frames, OAM fields and8×8 indexed tiles. Directory/index
words and header/frame/object bytes come from those fields; pixel bytes are
formed from two decoded palette indices. Temporary source-reconstruction
buffers are discarded after authentication. Exact per-byte segments establish
ownership inside candidate pages, composing with earlier owners and raw holes.
Live frame copies, counters, command pools, guest instructions, DMA and VRAM
remain original. The new counters are transient diagnostics outside saved
machine state and the native ABI.

One affected Windows core/frontend test build is warning-free; core tests were
compiled only. Two new exact Wine filters in the
[integration oracle](../crates/recomp/src/rom_data/unit_obj_initial_pose_runtime_tests.rs)
pass on their first executions:

- Seventeen publication/source/hash/path/JSON/dependency guards reject edits
  transactionally, retaining the previous owner and complete state. Interrupted
  publication, create/reuse/repair/move and e56 retention pass. Load/Undo,
  backup recovery and Restart retain the shared owner with its files unavailable.
- All146358 owned bytes are read with raw source bytes altered:484 directory,
  484 index,484 header,9540 frame,198 OAM and135168 pixel bytes; all121 sequences
  are covered. The earlier OBJ owner in the same page serves its own four bytes
  with no charge to this family.
- Thirty original/interpreter/compiled paths match complete states and guest
  clocks: three resets, raw ID/animation lookup, constructor prefix, six sampled
  scheduler uploads, six original DMA executors, a modified live RAM frame and
  typed/raw/previous-owner reader probes. All146358 new raw bytes and20 old OBJ
  bytes are altered during the relevant checks. The compiled read APIs charge
  aggregate bytes `[28,20,8,127,164,1]` for directory/index/header/frame/OAM/pixels.
  The single compiled pixel-reader probe is distinct from actual compiled DMA
  execution, whose full VRAM/state/clock equivalence is checked separately.
- All415 source edges, byte/halfword/unaligned-word APIs, three ROM mirrors,
  real-BIOS bus mode, CpuSet/FastSet and DMA preserve source routing. A mixed
  transfer crosses2 new bytes,1276 raw bytes and8 previous OBJ bytes; SWI and
  DMA charge exactly2/8 to the respective owners. An unaligned API word retains
  two deliberately edited raw bytes beside two decoded bytes.

The frozen core/frontend tests have SHA-256
`7a95a9115937bab2ccb9b73cf60dadf6a1db5b3d2634dd785d50f9f34487516c` /
`d0995de8ccfc90d5ddf7b197b4c0b02ea0b251038a524dfec35955983d8470cd`.
The706 hash/mtime guards and eight compiled inputs pass. The3542 instructions,
496 PCs and177 native dispatches are coverage only. Source/model/old-family
oracles were not repeated. Evidence under ignored
`target/rom-data-unit-obj-initial-pose-runtime/` includes
`windows-managed-build.json`, `windows-managed-results.json`,
`windows-managed-europe/{managed,native}-result.json` and
`prior-manifest-audit.json`. The managed manifest file SHA-256 is
`5168faf8b834d6bfc2b2f2854cb4475517113629bb55dfb4cc5118748b6da038`;
the runtime dataset identity reported by the loader is
`ab9286b25a85d46c8b80a967399b948072bdc8d514c5b5902b48cca89996c1ae`.

The targeted scope is distinct from ordinary use below. Whole unit/allocator
ABI, other animation/part/sprite sources, new32:9 scenes and new Linux/USA/native
Windows coverage remain open. No performance or storage campaign was run.

## Production CLI and positive ordinary Party scene

One changed-code Windows production build is warning-free. Its frozen CLI has
SHA-256 `d4c782c6d84f155a34406c780c5e9e973935d90c540ac7aca337cf79835b1fc3`.
The canonical executable's previous hash and nanosecond mtime are restored;
the existing Cargo-output invalidation touches only main.rs's mtime, with its
content unchanged. No source/model/managed/native checks were repeated.

Three actual CLI processes under Wine create e57, reuse it without writes and
run from an unrelated working directory with the retained native cache. All53
generation files match the accepted managed files. All55 managed file hashes
and mtimes remain unchanged on reuse and replay. The actual generation is
`generation-ZV0TqS`, with the same runtime identity and manifest file hash above.

The existing authentic EUR worldmap snapshot has SHA-256
`7701a360c2836d8ce1bac59288786cd34922046163048aa32dfa1f09fadcac9a`; its unchanged
Party-unit input has SHA-256
`bf757e6957e596ba823c9063284c51b6e8766408dbb29247d625869aabae1dfc`.
The320-frame transition opens the Party unit screen and its animated preview;
the240×160 endpoint was inspected. This newly tests unit-pixel consumption in
e57 while preserving the earlier UI-family acceptance for the same fixture.
There was no new gameplay snapshot, navigation, input or payload edit.

The CLI locator reads10962 aggregate new bytes:8 directory,16 index,4 header,
252 frame,7162 OAM and3520 pixel bytes, with8170 bus reads/two sequences. Its
positive pixel gate is independent of earlier metadata/UI resource use.

One Wine full pair on exactly the same320 frames, actual CLI dataset and frozen
`d0995de8..` test executable passes. Every initial and frame4519→4839 complete
machine state, guest clock, central/wide image, OAM and dispatch result matches.
Mixed/PSG/FIFO/bitwise-HLE audio matches with sample counts
`[702240,702240,84480,84480,702164]`. The same pixel/source counters are positive;
764 hash/mtime guards pass. The retained cache uses mixed native/fallback work.
A private runner precondition was corrected from a root exit field to the three
CLI process exits before any Wine invocation; the actual full pair ran once.

Private evidence is under `target/rom-data-unit-obj-initial-pose-cli/`:
`windows-build.json`, `windows-cli-no-write.json`, `windows-cli-summary.json`,
`windows-scene-result.json` and `windows-party-unit-full-state-audio.log`.
Source/model/managed/native/actual CLI/positive ordinary criteria are closed for
this declared projection. The other119 ordinary sequences remain targeted-only;
other poses/parts/IDs/palettes, whole unit/allocator ABI, new32:9/Linux/USA/native
Windows remain open. There was no performance, startup/RSS or storage campaign.

## Equipment poses: source model

This source projection is separate from the normal unit initial pose. Original
`0809BA84` reads item properties3,9,8 through `080CE1EC`. Property3 is byte08;
the guarded values1..19 select a nonzero equipment style. Property9 is the u16
at record0E, and supplies the sprite argument to `08022788` at `0809C07A`.
The live loop requests part0 and, unless actor flag2 suppresses it, part1.
Item0 bypasses property reads. The252 eligible records in the authenticated
376-record item owner project61 sprite IDs124..184. These are a static
projection, not live item/sprite limits.

The normal equipment branches select animation bases0,4,8. Actions12/13
explicitly replace the item sprite with185 and bases0/4; actions16/17 select
sprite126/base16, except item45 selects124. These overrides are included.
Facing stays a live signed byte. Original `08022174` uses full-word sprite
and animation arguments: it reads the root at394834+4*sprite, adds
`6*(animation & !3)`, then12 for low bits1/2 or0 for0/3. The caller adds
4*part. This projection includes both physical orientations and the original
four directional aliases; it imposes no cap or normalization on other inputs.
No directory or animation extent is inferred from adjacent pointers.

There are748 distinct selected cells, including86 real nulls. The constructor
returns -1 for a null before reading a sequence. The662 nonnull cells share258
counted sequences/3439 frames. Original `08022254` dispatches1911 operation1,
609 operation6,888 operation8 and31 operation0 frames. Only operation1 reads
words00/04 as relative pixel/OAM sources. Operation6 clears the task graphics
pointer, operation8 sets the task event byte from frame0A, and operation0 takes
the original default return. Other words, duration, opaque header and tail
bytes remain explicit; control commands do not create graphics references.

Operation1 reaches12 OBJ lists/25 objects and280 distinct raw4bpp requests,
37440 bytes/1170 decoded indexed tiles. The complete source plan has593
nonoverlapping spans/110666 bytes:248 directory,2992 index,69812 sequence,
174 OAM and37440 pixel bytes. Every span is disjoint from all53 authenticated
e58 resources. Ordered source concatenation has SHA-256
`feb6a277c8696b3c5e189c928cbb2cb6936cb2dbdb1a82b3300b1407557c54ae`.
The static audit checks73 instructions,4 BL targets,4 root literals and two
dispatch tables. Private evidence is under `target/rom-data-unit-poses/`,
including `audit-equipment.py` and `equipment-source-contract.json`.

The [public model](../crates/gba-core/src/rom_data/unit_obj_equipment.rs)
retains roots, canonical physical-cell keys/nulls, typed sequence fields,
original OBJ attributes and decoded palette indices. It reuses the accepted
tile/shape arithmetic without reopening its tests. Producer validation uses
the actual item owner; strict JSON, ordering, links, extents and source
authentication protect reconstruction. One affected Windows frontend test build
is warning-free; the frozen executable SHA-256 is
`16670f9da4a6617e937cb3ad918e77acab8cbc28885af8748aad3f942c802289`.
One new exact Wine filter in the
[source oracle](../crates/recomp/src/rom_data/unit_obj_equipment_model_tests.rs)
passes on its first execution:

- All593 spans reconstruct110666 bytes and match independent source hashes;
  strict JSON roundtrip and ten shape/link/unknown/coherent-edit/producer guards.
- 1496 actual constructor prefixes before allocation cover both aliases of all
  748 physical cells.86 null reset paths leave the task unchanged;258 complete
  original resets match independently predicted task bytes and preserved RAM.
- Six original command cases cover operation0/default, operation6 and every
  distinct operation8 event argument in this projection, without treating
  their opaque words as graphics references.
- 280 original scheduler → queue → real DMA3 uploads cover all280 distinct
  pixel requests/37440 bytes. Every VRAM byte and the declared RAM guards match.

The source probes observe8600 distinct nonpixel load bytes. Remaining opaque
or unread data is reconstructed, without an actual-read claim for every byte.
The187459 instructions/508 PCs are coverage only. Seven compiled source
preimages and828 previous hash/mtime guards are retained; a later action-number
comment correction changes no implementation. Evidence includes
`windows-model-build.json`, `windows-model-result.json`, `windows-model.log`
and `windows-model-europe/result.json`. Accepted initial-pose/e58 oracles were
not repeated. At this source/model boundary EUR remained e58-d57/54 files/
53 resources, USAe8-d7. Managed/runtime acceptance was selected separately below.

The broader static caller census also identifies normal/alternate actor
graphics via233A8/CF144 and233D8/CF180 (metadata52 properties4/5 and metadata14
halfwords04/06), plus explicit actor sprites42/230. The alternate producer
has64 concrete IDs and unresolved65535 job fields. Script opcode49 has167
instructions with13 concrete base-animation values, but their actor pairing
is not yet authenticated. General body/parts/alternate/script sources remain
open; script groups are not crossed with every sprite. Original22E20 has no
direct BL or exact absolute Thumb-pointer reference in the retained census;
this does not establish dead code. New Linux/USA/native Windows/new32:9,
whole allocator/scheduler ABI and remaining palettes stay pending. No new
gameplay fixture, navigation, private payload edit or performance measurement.

## Equipment managed publication and runtime reads

EUR schema1/extractor59/decoder58 adds `unit-obj-equipment.json`, resource
`graphics.unit-obj-equipment`, with actual `gameplay.item-records` dependency.
The55 generation files/54 resources retain all53 earlier decoded files,
resource records and coverage entries; e58 remains available and USA stays
schema1/e8-d7. The file is byte-identical to the accepted source model, SHA-256
`23391d28a69a1bb728353d267a6080e47f6a741846c0c72ad7b631d887fb701c`.
The runtime dataset identity is
`2c200038c7c0eecc79931bfee44a8f9ef89bc9c1fea87fba07d6dac8d9cd9ae3`;
the manifest file itself has SHA-256
`aa8bba05cf6c151e4484565898be16183e3f39fd9c9887317c4f9b3d4649b911`.

The [immutable owner](../crates/gba-core/src/rom_data/runtime/unit_obj_equipment.rs)
serves directory roots, index words including real nulls, counted headers,
command fields, OBJ attributes and packed palette indices directly from typed
fields. Temporary reconstruction buffers are discarded after authentication.
Candidate pages only select a possible reader; exact per-byte segments decide
ownership. Previous owners and mutable raw holes compose inside the same read.
Guest facing, commands, task pools, frame copies, DMA, VRAM and palettes remain
original. Diagnostic counters are transient, outside save state and native ABI.

One affected Windows core/frontend test build passes without warnings; core
tests are compiled only. Frozen core/frontend executable SHA-256 values are
`0bc529876cd9280b93edb984b54344e6687d66e34ecbdf486d270ca01d0da834` /
`cd03cda904e1926ca8723aff189b1afa8bc298e6e1740904e3148c62439adb60`.
Two new exact Wine filters in the
[runtime oracle](../crates/recomp/src/rom_data/unit_obj_equipment_runtime_tests.rs)
pass on their first executions:

- Seventeen path/JSON/hash/source/dependency guards reject changes
  transactionally, preserving the attached owner and complete state.
  Interrupted publication, create/reuse/repair/move and e58 retention pass.
  Load/Undo, backup recovery and Restart retain the shared owner while its
  files are unavailable.
- All110666 owned bytes are read with raw sources altered:248 directory,
  2992 index,1032 header,68780 frame,174 OAM and37440 pixel bytes, covering all258
  sequences. Previous OBJ and initial-pose owners each serve four altered raw
  bytes in the same page, with no charge to this family.
- Twenty-six original/interpreter/compiled paths match complete states and
  guest clocks: both parts, real nulls, full-word raw ID/animation inputs,
  commands0/6/8, sampled scheduler/queue/DMA paths, a changed live RAM frame and
  typed/raw/previous-owner reads. Compiled read APIs charge the six categories
  `[32,24,6,47,32,1]` aggregate bytes. The pixel API probe is separate from real
  compiled DMA execution, whose complete state and VRAM are compared too.
- All593 source edges, byte/halfword/unaligned-word APIs, three ROM mirrors,
  real-BIOS bus mode, CpuSet/FastSet and DMA preserve routing. A mixed transfer
  includes2 new bytes,1028 raw bytes and8 previous OBJ bytes; SWI and DMA charge
  exactly2/8 to the respective owners. An unaligned API word retains two
  deliberately changed raw bytes beside two decoded bytes.

The1511 instructions/502 PCs/97 native dispatches are coverage only. The831
build and833 runtime hash/mtime guards pass; eleven source inputs/preimages
are retained. Source/model and earlier family/CLI/scene acceptance are not
repeated. Private evidence under `target/rom-data-equipment-runtime/` includes
`windows-managed-build.json`, `windows-managed-results.json`,
`windows-managed-europe/{managed,native}-result.json`, both Wine logs and
`prior-manifest-audit.json`.

This closes targeted managed/runtime acceptance for the declared equipment
projection. Actual production CLI preparation/reuse and the positive ordinary
EURFight500fullpair are closed below. No whole allocator/scheduler or ordinary
ABI claim follows. General actor-body/alternate/script pairings and other palette
sources, new Linux/USA/native Windows/new32:9 remain open. No performance/space
campaign, old accepted test replay, new gameplay fixture/navigation, private
payload edit, Android change or package is introduced.


## Equipment production CLI and ordinary coverage limits

One changed-code Windows production build is warning-free; frozen candidate
SHA-256 `0795d057470ac67658559cd674ff3141232fbeea1516c8600ad8edf6b646118e`.
The canonical executable retains its original hash and nanosecond mtime;
Cargo invalidation touches main.rs's mtime only, with its contents unchanged.
Source/model/managed/native and earlier scene acceptance are not repeated.

Actual Wine CLI create/reuse from an unrelated working directory with the old
native cache verifies all55 generation files against the accepted managed set.
All57 managed file hashes and mtimes remain unchanged on reuse and replay.
Actual e59 generation `generation-nXJuGC` retains dataset identity
`2c200038c7c0eecc79931bfee44a8f9ef89bc9c1fea87fba07d6dac8d9cd9ae3` and
manifest-file SHA-256
`aa8bba05cf6c151e4484565898be16183e3f39fd9c9887317c4f9b3d4649b911`.

Two new-family locators are terminal coverage negatives: the retained genuine
Montblanc Richiamo/Pecorelle cast1000 and F09 Fight-input700 each read zero
new equipment bytes/sequences. Their earlier palette-program observations do
not establish equipment use. Neither receives a full pair, family guard build
or unchanged repeat. The raw spell snapshot's supported FFTA-ST/version1/EUR
identity and payload hash are checked; its first private precondition assumed
a desktop preview wrapper and was corrected before any build or guest run.
The F09 desktop snapshot's preview/core/EUR hashes pass too.

The F09 source snapshot SHA-256 is
`eda24ca6a846878d58497ecea016dbde31c434b602db770c3ebe8be5ddd7e90f`.
Its preview shows Polly's Move/Action/Wait/Status menu. CLI PPM output contains
the guest framebuffer and can omit the separate native UI; a field-only
endpoint image therefore does not prove menu/action execution. A read-only
inspection of the retained211 candidate9BA84 call sites identifies potential
item arguments and dynamic action wrappers, without claiming unique reaching
definitions or live execution. Generic body calls with item0 alone cannot
establish this family; explicit action12/13/16/17 overrides remain part of the
accepted model.

Actual CLI acceptance is closed. Positive ordinary equipment use and one
matching complete state/clock/image/audio pair now pass in the authorized
copiedEURPolly-to-Hildegard Fight500case described below.
A proposed ordinary-pad checkpoint derivation was not executed after automatic
approval review rejected its new gameplay/checkpoint writes. No new sequence,
checkpoint or cartridge save was created by that command. On25October the user
explicitly authorized local checkpoints through ordinary game commands from a
copy of an EURcheckpoint, preserving originals/cartridge saves and keeping all
outputs outsideGit. That authorization clears the previous review block; raw
RAM/HP/status/save/PC edits remain outside scope. Positive-use evidence is still
required before selecting a full pair; that gate and the new pair now pass below.

Private evidence is under `target/rom-data-equipment-cli/`: `windows-build.json`,
`windows-cli-summary.json`, `windows-cli-no-write.json`,
`windows-fight-locator.json`, `fixture-precondition.json`, both endpoint images
and the static caller selection aid. All7623 prior artifact hashes,
833runtime/891CLI/893Fight hash-mtime guards and eleven earlier source inputs
remain intact. New Linux/USA/native Windows/new32:9 remain pending. No
performance/space campaign, old proof replay, private payload edit, Android
change, package, commit or push is introduced.


## Normal-mode battle poses: source projection and typed model

The offline [normal battle model](../crates/gba-core/src/rom_data/unit_obj_normal_battle.rs)
extends the initial0/0 projection through the original actor-body/part selectors.
It uses121normal IDs from metadata52property4/metadata14halfword04, plus the
explicit sprite42 override. The unresolved FFFF job fields and alternate-mode
IDs are excluded. This finite source-query projection does not assert that
every actor can perform every action, and imposes no live sprite/animation cap.

Original0809BA84's generic control flow yields19body bases, with five of them
also selecting the same actor's parts1/2 when the mapped item style is zero.
Direct calls add body4/8/64; status substitution in0809ACF8 adds12/20/128/140
only when flag!=0 and the incoming base is zero. Ten direct0809D9B0callers add
24/60 (other constants already occur). Explicit080E1914and080E2D08paths add
body/part12/36/60. Stored actor+36 reloads do not introduce another immediate
selector. Dynamic RAM animation streams and script49's13concrete bases have
unresolved actor pairing and are not crossed with these sprites.

The resulting normal-mode projection contains29body bases and8extra-part
bases, two physical facing orientations, and10980logical keys. Original08022174
maps facing0/3 and1/2 to the same cells; the logical keys resolve8374physical
cells,5293of them genuinely null. No sequence fallback is invented. The graph
contains2581counted sequences/17894frames,150OBJlists/676objects and3216raw4bpp
transfer requests/1202272pixel bytes, decoded to palette indices. Original
shape/size tile arithmetic and DMA word-count narrowing determine pixel lengths.
Only operation1's words00/04 are pixel/list offsets. Operations0/2/3/6/7/8 retain
all words and tail bytes:2copies the two halfwords at0C/0E to live task18/1A,
3toggles task flag8,6clears task28,8sets the event byte, and0/7take the default
return. Scheduling, task allocation, palette selection, RAM and VRAM remain
original guest work.

All6022full-graph spans/1608816B reconstruct exactly. The model verifies shared
views against the existing initial-pose and equipment models before excluding
146358B and6984B respectively from new ownership. The new plan is6021spans/
1455474B:4directory,32948index,355304sequence,4018OBJ-list and1063200pixel bytes.
Its concatenated-source SHA-256 is
`32b6b2408694e026fcc7858ab618d835aff43562273e46f360a039d6ec1eb8dc`.
An independent static audit of all54current owners finds no other overlap;
internal physical requests have no conflicting roles or pixel extents. Strict
JSON, ordered identities, exact links, extents and byte-identical dependency
views remain separate from authentication against the exact EUR ROM.

The unrestricted normal/alternate cross product was rejected. For sprite247,
produced only by job68's alternate property5 at6E623D, animation104's computed
cell is the independently rooted directory394834. Reading that directory word
as a sequence pointer interprets an index root as a sequence header. Other high
selectors repeat that mistake. No large-frame-count filter, bank size inferred
from adjacency, or global animation limit repairs the missing producer pairing.
Alternate-mode reachability/status constraints therefore remain open.

Private source evidence is under `target/rom-data-battle-poses/`:
`selector-flow.json`, `header-projection.json`, `header-review.json`,
`normal-source-audit.json`, `normal-source-requests.json` and annotated original
traces. Static disassembler use does not execute a Linux game validation.
One affected Windows frontend test build passes without warnings. One exact
new Wine source filter passes on its first execution:12guards,16474constructor
prefixes over8237new physical cells/all three parts,5293null resets and2451new
sequence resets.100command probes cover every distinct operation2halfword pair,
every event argument and the other command kinds.2858original scheduler/queue/
realDMA3uploads match the whole VRAM and all1063200new pixel-source bytes.
89016observed nonpixel load bytes/514PCs/2389335instructions describe coverage;
not every preserved nonpixel byte was read by these reduced probes. Prior-owner
original-reader acceptance is not replayed. The executable is frozen as
ce3fe0a0091e167541d4c049b0c3b526f22ead57d686bae265ca11c0d8479827;
ten compiled source inputs/preimages and893inherited hash-mtime guards pass.

Source/model acceptance is closed for this finite normal-mode projection.
The subsequent e60managed/runtime and positive ordinary-use proofs are covered
below. New Linux/USA/native Windows/new32:9 coverage remains pending. No full allocator/whole actor ABI, alternative
producer pairing, new gameplay checkpoint, performance or storage measurement
is part of this unit.


## Normal battle managed publication and runtime reads

EUR schema1/extractor60/decoder59 adds `unit-obj-normal-battle.json`, resource
`graphics.unit-obj-normal-battle`, with actual metadata52/metadata14/initial-pose/
equipment dependencies. The full semantic model keeps shared views inspectable;
only the6021new spans1455474B enter its ownership plan. The previous54decoded
resources and e59generation are retained; USA stays e8-d7. The source/model
acceptance above remains closed and is not rerun for this integration.

The [immutable typed owner](../crates/gba-core/src/rom_data/runtime/unit_obj_normal_battle.rs)
intersects each typed directory/index/sequence/list/tile item with those owned
spans, retaining the original item offset inside every intersection. Logical
aliases of one physical index cell are deduplicated only after their exact bytes
agree. Shared initial/equipment intervals have no new segment; their existing
owners answer even when the normal-battle model contains the same semantic view.
No raw ROM shadow is retained. Candidate page ranges only select byte lookup;
raw holes and earlier owners compose inside a single read of any supported width.

Counters separately report new directory/index/header/frame/OBJ/pixel bytes.
The41word sequence bitset indexes all2581semantic sequences, but only the2451
with new owned bytes can charge this resource. Counters are transient and remain
outside save states; guest tasks, animation commands, RAM copies, DMA, VRAM,
palette work, native translation/cache and save/load ABI stay original.

The [reduced integration oracle](../crates/recomp/src/rom_data/unit_obj_normal_battle_runtime_tests.rs)
passes both new Wine filters on their first execution. One affected Windows
core/frontend test build is warning-free; the core executable is compiled only,
not run. The frontend is frozen as94ce453c76e6081df58406d4da3062eb86ec0f149cf5f8e4beed3ec692946525;
13compiled source inputs/preimages,896build and898runtime hash-mtime guards pass.

Managed checks pass17guards, publication failure/create/reuse/repair/move,
unavailable-file attachment, Load/Undo/backup/restart and all1455474new typed
bytes. Their directory/index/header/frame/OBJ/pixel categories are
[4,32948,9804,345500,4018,1063200], with2451new sequences charged. Poisoned old
OBJ/initial/equipment probes each return four typed bytes with no new charge.
An independent manifest audit verifies56files55resources, all54old decoded
files/resource records/coverage unchanged, accepted model file33d0c76b.. and
all6021owned-source records identical to the source plan. Runtime dataset
identity is4e153572d98b4f4ebcb66503e629d5c8eaae3fda71767591dee162a5e13103c9;
manifest-file SHA-256 is87c4117e03c60572450ada3242aecf00fc871ab935de81ae020c7542da35d9f7.

The native filter passes32original/interpreter/compiled complete-state-clock
paths: all parts0/1/2, nulls, full-word raw ID/animation, live RAM frames,
commands0/2/3/6/7/8, two sampled scheduler uploads and two DMA flushes. All six
compiled API categories are positive [8,36,8,54,20,1]; the pixel API probe is
separate from actual complete-state DMA comparison. All6021span edges, widths,
mirrors, real-BIOS bus behavior, two BIOS copy services and one DMA case pass.
One mixed1610Bwindow contains2new,1036raw,316initial,248equipment and8olderOBJ
bytes; exact per-owner charges and complete states agree. A separate unaligned
API case combines2new with2deliberately edited unowned raw bytes. All new
source bytes and relevant older-owner probes are poisoned in these checks.
1945instructions/512PCs/118native dispatches describe coverage only.

Private managed/native evidence is under `target/rom-data-normal-battle-runtime/`:
`windows-managed-build.json`, `windows-managed-results.json`, both logs,
`windows-managed-europe/{managed,native}-result.json`,
`prior-manifest-audit.json` and source preimages. All7767prior artifact hashes
remain unchanged; no source/model or earlier-family original proof is repeated.
Declared managed/native acceptance is closed. Production CLI preparation/reuse
and positive ordinary consumption are now verified below. Alternate/script/live-stream pairings and new Linux/USA/native
Windows/new32:9 checks remain open; no performance/storage campaign is added.


## Normal battle actual CLI and ordinary equivalence

One changed-code Windows production build is warning-free, frozen as
`e54a7970b9a803908fc75649abdf24de191852b9b8b172f8a9f37af00f10b3be`.
The canonical production executable retains SHA-256f3e277db.. and exact
nanosecond mtime1792570130397888519; `main.rs` changes only its mtime to invalidate
Cargo's restored-output assumption, not its contents. Wine actual CLI creates
and reuses e60-d59from an unrelated working directory with the existing native
cache. All56generation files equal the accepted managed output;58managed files
retain hashes/mtimes through reuse and the locator. The CLI dataset is
`generation-TSOcSZ`, identity4e153572.. /manifest-file hash87c4117e.. . Earlier
source/model/managed/native and old-family acceptance remain closed.

The retained F09recording is an authentic EUR battle: Wait with Polly, Wait with
Sigmund, enemy monster activation/movement, next player Marche. Its unchanged
snapshot SHA-256 iseda24ca6a846878d58497ecea016dbde31c434b602db770c3ebe8be5ddd7e90f;
input SHA-256 iscd5e4d3e968b310ac4f17652700c07829f7ac737ad54c3430d1bcf6a78760397.
Only the existing3530relative frames are replayed, from guest8476through12006;
there is no new gameplay sequence, checkpoint, private payload edit or cartridge
save write. The normal body movement producer gives a positive new-family
locator:842236aggregate source bytes =40index+10sequence-header+26826frame+
815360pixel bytes across five sequences. This scene charges no new directory
or OBJ-list bytes; their targeted proofs remain separate.

A [test-only family guard](../crates/recomp/src/rom_data/validation_tests.rs)
requires positive decoded pixels in the existing complete-scene oracle. One
changed-guard Windows frontend test build is warning-free, frozen as
656347466d48969e99fc999f948b587bcbdd81c7af5f58b4feb906a716a0f9e3. One Wine3530frame
full pair using the exact actual CLI generation passes on its first execution:
every complete state/guest clock, central/wide framebuffer, OAM, native dispatch
and mixed/PSG/FIFO/bitwise HLE audio sample agrees. The eight reported family
usage values exactly match the CLI locator. All958scene guards/14compiled
source inputs/preimages and8083prior artifact hashes remain intact; thirteen
runtime implementation inputs are unchanged. No second scene or old proof is
replayed, and no timing or storage campaign is run.

Actual CLI and ordinary-use acceptance are closed for this declared projection.
The other2446new sequences retain targeted source/runtime coverage, with no
exhaustive ordinary playthrough claim. Alternate/script/live-stream pairing,
and new Linux/USA/native Windows/new32:9 remain open. The earlier equipment
checkpoint rejection was superseded by explicit25October user authorization;
ordinary equipment Fight500now passes its own positive fullpair below.

Private evidence is under `target/rom-data-normal-battle-cli/`:
`windows-build.json`, `windows-cli-summary.json`, `windows-cli-no-write.json`,
`windows-scene-build.json`, `windows-scene-result.json`, corresponding logs,
fixture selection, source preimages and the final movement image. No private
ROM, save, snapshot or recording becomes a tracked artifact or package input.


## Alternate battle selectors and explicit shared target views

The alternate-mode source projection now has a static contract, separate from
the closed normal-mode pipeline. Original0809B734selects normal/alternate actor
sprite fields using live actor/unit/map state: actor flag0x10000forces normal;
the alternate branch requires unit+0xFD==2, bounded grid coordinates, nonzero
0801DDEC, matching0801DE78flags and signed0801DD88height above the actor-derived
height. 080CF180reads metadata52property5 or metadata14halfword06; actor flag0x20
forces230. These are original branch/field facts, without an inferred terrain
name or proof that every actor/action pair is reachable. Job redirectFFand
unresolved FFFF remain separate live cases.

The65known alternate IDs, including explicit230, use the same finite29body/
8extra-part bases already derived from original selectors. All5850logical keys
remain in the projection. They resolve5682physical cells:4709real nulls,
953cells naming706counted sequences/2771frames, and20cells naming independently
rooted normal sprite indexes. The latter are explicit **shared index-root
views**, not sequence headers or missing/null bindings. Every such cell equals
an authenticated initial-pose directory field; its target's first word is the
same sprite's authenticated initial-pose index field. Their witness sprite IDs
are0,3,6,9,12,15,18,21,24,27,30,33,36,39,54,57,66,69,72,75.

The one internal4Bdirectory/index overlap is exactly sprite247/animation104:
its computed cell394834is also sprite0's directory entry, and the target371494
is sprite0's index root. The prior blanket sequence interpretation remains
rejected. A tagged target model must preserve this pointer and its dependency
witness without interpreting the pointer's low halfword as thousands of frames.
No frame-count threshold, removed binding, global animation cap, guessed bank
extent or guest fallback is introduced. Reachability/status constraints for
these alias-producing queries remain unproved; original guest control remains.

Only operation1of the counted sequences supplies OBJ/pixel offsets. The graph
contains30OBJlists/108objects and1203raw4bpp requests/387008B; commands2/3/6/8
keep all words/tails under their existing original format. Direct physical
requests cover468944unique bytes. Exact shared views comprise6232Binitial-pose,
52604Bnormal-battle and22Bequipment sources. An independent audit of all55e60
owners finds no other overlap. No pixel request has varying extents, no typed
request mixes old/new ownership, and the sole role conflict has the explicit
sprite0witness above. External index-root prefix witnesses remain fields of
the existing initial-pose dependency.

The new source plan is3321spans/410086B:240directory,21008index,46624sequence,
102OBJ-list and342112pixel bytes. It contains606new sequences/2210frames,
5252new cells/4428new nulls and1079new pixel requests. Its concatenated-source
SHA-256 is`bdf15f7d79e3af013fae656e16341d5a01d216be364ed881b877850f116bd002`.
Bounds follow exact query addresses, counts and original shape/size DMA
arithmetic. No directory/bank size is inferred from neighboring pointers.

Private static evidence is under `target/rom-data-alternate-battle-poses/`:
`alternate-source-audit.json`, `alternate-source-requests.json`,
`target-view-contract.json`, `audit-alternate.py` and its executed preimage.
The static source contract is closed for this finite tagged-view projection;
the offline tagged model/original-reader Wine proof and managed/runtime checks
below are now closed. EUR e61-d60adds the typed owner while preserving e60and
USAe8-d7. Actual CLI/ordinary use, script49/live streams and new Linux/USA/native
Windows/new32:9 remain open. No gameplay checkpoint,
private payload edit, performance or storage measurement is used.


## Typed alternate target model

The [offline model](../crates/gba-core/src/rom_data/unit_obj_alternate_battle.rs)
uses an explicit `Target` tag for null, counted-sequence and shared-index-root
references. Each of the5850bindings keeps its exact sprite/animation/part key,
index source and original pointer. The twenty dependency views remain
non-sequences, and their witness IDs/offsets are checked against initial-pose
fields. They do not create a frame array, null replacement, count cap or guest
fallback. The single4Bdirectory/index overlap coalesces only when those typed
views agree exactly; all other conflicting physical interpretations fail.

Existing command/frame, OBJ-attribute and indexed4bpp tile types are reused.
The model reconstructs all3635canonical direct-source spans/468944B, then
validates byte-identical initial/equipment/normal-battle shares before excluding
their existing ownership. Source IDs, ordering, extents, graph links, producer
keys and strict JSON remain separate from authentication against the exact EUR
ROM. Only the3321new spans410086B form its new source fingerprint.

The [source oracle](../crates/recomp/src/rom_data/unit_obj_alternate_battle_model_tests.rs)
is implemented for one reduced new Wine filter. It targets new physical cells,
proper sequence resets, command arguments and new scheduler/DMA uploads;
shared-root aliases receive original constructor-prefix checks only. It does
not schedule fake frames from index words or repeat previous-owner original
acceptance. One affected Windows frontend build is warning-free, frozen as
66abf7cee364967fd3489e87423466ce3535e38a5cf952f444ad65d65e0fabba. Eleven selected
compiled inputs/preimages and958build/960Wine hash-mtime guards pass. One exact
new Wine source filter passes on its first execution:15JSON/source/producer/
shared-view guards, all5850bindings,10544original constructor prefixes over5252
new cells and40shared-target facing aliases,4428null resets and606proper sequence
resets. Shared roots stay non-sequences and no fake frame is parsed or scheduled.
Five original command cases cover the new operation2/3/8arguments; shared
operation6retains earlier coverage.1079original scheduler/queue/realDMA3uploads
match entire VRAM and all342112newpixel-source bytes.40997observed nonpixel load
bytes/506PCs/1141155instructions describe coverage only, not every preserved byte
or whole actor/allocator ABI. Earlier original-reader acceptance is not replayed.

Private source/model evidence is under `target/rom-data-alternate-battle-model/`:
`windows-model-build.json`, `windows-model-result.json`, `windows-model.log`,
`windows-model-europe/result.json`, the tagged JSON output and source preimages.
Declared source/model acceptance is closed; managed/runtime publication is
covered below. Actual CLI and ordinary-use checks remain open; USAe8is unchanged.
New Linux/USA/
native Windows/new32:9 remain pending, without performance/storage measurements
or new gameplay checkpoints.


## Alternate managed publication and typed runtime reads

The implementation adds EUR e61-d60 `unit-obj-alternate-battle.json` /
`graphics.unit-obj-alternate-battle`, with actual metadata52/metadata14/initial/
equipment/normal-battle dependencies. Previous55decoded files/resource records
and e60are retained; USA remains e8-d7. Source/model physical-data acceptance
above remains closed and is not replayed.

The [immutable reader](../crates/gba-core/src/rom_data/runtime/unit_obj_alternate_battle.rs)
projects only3321new spans410086B from typed directory, target, frame, OBJ and
indexed-pixel fields. Every item retains its original offset after intersection
with owned sources. Initial/equipment/normal views and all twenty shared index
roots keep their older owners; raw holes compose inside reads. The12word bitset
indexes706semantic proper sequences, charging only the606new ones. Counters stay
outside save state; original task allocation, commands, RAM, DMA, VRAM, palettes,
native translation/cache and save/load ABI remain unchanged.

Inspection of the locked serde implementation found that internally tagged unit
variants consume extra fields despite `deny_unknown_fields`. `Target::Null` now
uses an empty struct variant so the struct-style decoder enforces rejection.
The intended JSON remains `{"kind":"null"}`; comparison with the accepted source
file establishes byte-identical valid output (SHA-256
`cad7771dbddf3d92b93c6a9df6a62fa430b775b9e7e2932253207d71dff35580`). This narrow parser
fix changes no original pointer/frame semantics; only the source oracle's Rust
pattern syntax changes, with its old compiled preimage preserved.

One affected Windows core/frontend test build passes without warnings; core
is compiled only. The frozen frontend SHA-256 is
`8f9a71e42a953360bde04b76d88ab268813a83b7aa22d2b1e4a99a886952c7e8`.
Both exact new [Wine integration filters](../crates/recomp/src/rom_data/unit_obj_alternate_battle_runtime_tests.rs)
pass on their first execution. Managed20guards cover create/reuse/repair/move,
interrupted publication, unavailable files, Load/Undo/backup/restart and failed
attach isolation. Coherent alias offsets, unknown tags and null extra fields
are rejected. All410086typed bytes are read as240directory/21008index/2424header/
44200frame/102OBJ/342112pixel bytes, charging606new sequences. All20shared target
views remain; probes of oldOBJ/initial-directory/initial-index/equipment/normal
fields each charge their previous owner for4B and never charge this new family.

The34original/interpreter/compiled complete-state-clock paths match: proper
part0sequences, real nulls for alternate parts1/2, shared-target constructor/read
prefixes, fullword IDs/animations, liveRAMframes, commands2/3/8, two sampled
scheduler uploads/two DMA flushes and all six compiled API categories. Shared
roots are never scheduled as fake sequences. All3321span edges/widths/mirrors/
real BIOS bus/two SWI/one DMA paths pass. The mixed2new780raw8oldOBJ-byte window
preserves exact charges; an unaligned2new+2editedraw read also passes.
1955instructions/508PCs/124native dispatches describe functional coverage only.

Independent audit confirms57generation files/56resources, all55prior decoded
records/coverage unchanged, e60retained, exact new-source plan and actual five
dependencies. Runtime dataset identity is
`5cbe42d14d8b9b7402c50ca7fd14d76b4fba55fe445eca84cd15186da2193105`;
manifest-file SHA-256 is
`6bfbbed6fdb12a81438537de07004c6338a814b683c68eadb32d772f4bb95714`.
All8216prior artifacts, fifteen selected compiled inputs/preimages and961build/
963Wine hash-mtime guards remain intact. Source/model/original acceptance stays
closed; it was not replayed for the narrow parser fix.

Private evidence is under `target/rom-data-alternate-battle-runtime/`:
`windows-managed-build.json`, `windows-managed-results.json`, managed/native
logs and result JSONs, `prior-manifest-audit.json`, `prior-preservation.json`
and compiled-source/pre-fix preimages. Declared managed/native acceptance is
closed. Production CLI/ordinary acceptance and new Linux/USA/native Windows/
new32:9 checks remain open; no performance/storage campaign or new gameplay
checkpoint is added.


## Alternate actual CLI and remaining ordinary-use gate

One new e61Windows production build is warning-free, frozen as
`d3db6585bb082a95d890b328940e7701e35dfa407b18f2eb72757db59ef6ce1c`.
The canonical executable returns to its exact original hash and mtime; main
source content is unchanged (Cargo invalidation changes its mtime only).
The actual Wine CLI creates and reuses57generation/59managed files from an
unrelated cwd, preserving every file's hash/mtime on reuse. All57files match
the accepted managed e61generation, including manifest-file SHA-256
`6bfbbed6fdb12a81438537de07004c6338a814b683c68eadb32d772f4bb95714`.
One frame from the immutable authentic F09snapshot loads the retained native
cache and new e61dataset successfully. This is an attachment smoke: it consumes
zero new alternate bytes and supplies no ordinary positive-use proof. The
accepted normal3530frame pair is not repeated.

Read-only inspection authenticates81existing EUR snapshots and checks for a
consistent new alternate sequence task plus matching original sprite/animation
index lookup. No such current task supplies a concrete positive replay lead.
This does not exclude future alternate use in those recordings. The first
Python inspection helper mixed guest pointers and file offsets; its invalid
preimages/results remain separate, and only the cheap static inspection was
corrected. No game test/build/recording is repeated for that helper correction.

All8543prior artifacts,1023CLI hash/mtime guards and fifteen runtime source
inputs/compiled preimages remain intact. There is no new implementation change,
scene guard/build/fullpair, pad navigation, checkpoint or private-payload edit.
Actual productionCLI acceptance is closed. Positive ordinary alternate use
remains open until a concrete producer/input lead exists; pending equipment
navigation approval was later explicitly granted for local ordinary-pad EUR
checkpoints on25October. New Linux/USA/native Windows/new32:9 and
script/live reachability remain open, without performance/storage measurements.

Private evidence is under `target/rom-data-alternate-battle-cli/`:
`windows-build.json`, `windows-cli-summary.json`, `windows-cli-no-write.json`,
actual CLI logs, corrected `existing-scene-leads.json`/`river-scene-leads.json`,
invalid-address helper/result preimages and `preservation-result.json`.


## Script animation selectors require live actor bindings

The167retained script animation instructions use **decimal opcode49(0x31)**,
whose five-byte command directory entry resolves to original08126CC0. Opcode
hex49(decimal73) is a different command at081270F4. The authenticated existing
script archive retains all instructions and exactly matches the earlier
projection:18distinct actor selectors and thirteen bases0/4/8/16/20/24/32/40/
48/88/136/140/180. No new script bytes or duplicate resource are needed.

The first operand is a logical actor selector. Original081290EC truncates it
to u8.151instructions use selectors below128: original080CBD24searches twelve
records at02002FC4then twenty-four at02000080, stride264, comparing live byte4;
IDs0/1return null. The other16instructions use128..254and resolve through the
live script registry at the pointer held in020159D4, with its original fallback
when no registry is installed. The fallback uses live02003CA6and02002EBC; no
immediate sprite is encoded there. Selector255would select the first of24live
records with flag0x400, but no retained49instruction uses255. These are original
lookup branches, not new input restrictions.

Original0809CC78then searches actor contexts by matching their unit pointer.
The selected context supplies its current sprite field+34; command49does not
refresh that field from metadata. A low selector value2therefore cannot be
exported as sprite2, nor can high selector128be exported as sprite128. The
lookup, saved unit/job state, registry aliases and prior actor initialization
must remain live.

The group operand is little-endian u16; original shifts derive
`(group << 2) & 0xffff`. Facing comes from signed actor+1F. The handler removes
the two extra tasks through0809D7B4, then calls0809D9C4with body flag0and the
same base for body and extra parts. This suppresses the flag-driven base-zero
status substitutions used by other callers. Existing property40facing behavior
still applies; no guessed facing cap, animation cap or blanket script×sprite
projection is introduced.

Nearby creation commands do not establish an unconditional binding. Command31
can patch template byte01only in its new-unit branch; its existing-unit branch
preserves the live value. Command32repositions an existing resolved unit and
supplies no job/sprite immediate. A textual earlier31or32in the same program
is therefore not proof of a dominating creation or an unchanged current sprite.
All constructor sources remain with their already accepted owners.

The static check authenticates the exact EUR image plus the existing script
archive/command-directory decoded files and their source hashes, all167five-byte
instructions and67halfword/nine literal/eighteen BL fingerprints. It identifies
zero context-independent new sprite queries and changes no source ownership,
model, manifest revision, runtime or save/native ABI. An initial decimal/hex
notation correction selected the proper existing handler trace; one private BL
fingerprint was corrected from its second halfword to its first. No guest test,
build, old acceptance replay or new gameplay fixture was run.

This selector classification is closed; complete sprite/animation pairing
remains open. The next useful evidence is a functional observation of resolved
actor sprite/animation/part queries while an existing unmodified EUR recording
executes command49, with explicit source/PC/actor linkage. Current static
selectors alone cannot prove those keys. EUR e61-d60/USAe8-d7are unchanged;
Linux/USA/native Windows/new32:9 remain pending, with no performance measurement.
Private evidence is under `target/rom-data-script-obj-poses/`:
`script-selector-contract.json`, `script-selector-projections.json`, the static
audit helper and original disassembly. Existing source/model/native/CLI/scene
acceptance and the rejected equipment navigation remain closed/unexecuted.


## Source-linked script query observation

The new [test-only observer](../crates/recomp/src/rom_data/script_obj_query_tests.rs)
wraps the retained native entries for49/0809ACF8/08022788. Each probe reads state
immutably, releases the read borrow and calls the exact retained native function
when present. Missing entries execute the same single original instruction via
`Machine::step` as the normal dispatch fallback. The local instrumented entry
accounting is diagnostic only and does not establish dispatch equivalence.
Original handler stack/return linkage identifies the body invocation; exact
constructor stack/call-site checks bind actual sprite/fullword-animation/part
arguments to the current actor context and authenticated script instruction.
The original functions/instructions and ordinary replay input still perform all
guest work; no guest PC/RAM value is forced.
No product native ABI/cache, source owner, save layout or guest address cap is
changed. Only the observer module and its cfg(test) registration are new code.

The selected existing EUR snowball snapshot at frame21420and unchanged6000frame
input already have closed script/coordinate/auxiliary acceptance. This new
source census addresses missing resolved actor-query evidence. The first
warning-free Windows frontend build produced an observer that required every
probe to have a native cache entry. Its Wine preflight failed at that assumption
before snapshot load or any scene frame; evidence/preimages are retained.
The corrected observer preserves normal missing-entry interpretation. One
changed-input frontend rebuild passes without warnings, frozen as
`4342f2daa457c3882591b4c035cb3645bce9349257e1f5466bca6a8cb57c1e7c`.

The single actual Wine6000frame census passes from21420to27420: six49handlers,
six SP/LR-linked body contexts and18real constructor queries. All use selector16,
group2, resolved sprite46/animation8and parts0/1/2. Their six script sources are
B94DC1/B94DE6/B94DED/B94E21/B94E28/B95080. The retained cache lacks49and body
entries, so those probes execute original missing-entry steps; the constructor
entry forwards its retained native function. No native cache is rebuilt or
changed. Instrumented entry accounting supplies no dispatch-equivalence or
performance claim.

Independent read-only authentication checks every script operand and recorded
index/target word against the original ROM and all56e61owners. Body46/8/0uses
`graphics.unit-obj-normal-battle`. The extra-part queries select two real null
cells at37C760/37C764, currently outside existing ownership. These are eight
source bytes, SHA-256
`af5570f5a1810b7af78caf4bc70a660f0df51e42baf91d4de5b2328de0e83dfc`.
They introduce no new counted sequence or pixel payload. No duplicate body
owner, blanket script×sprite product or state-dependent dataset publication is
introduced. A stable source contract/model/ownership decision for those null
cells remains separate; other161instructions and actor/group/state pairings
remain open. The six observed bindings do not establish a fixed sprite for all
uses of logical selector16.

All1025Wine hash/mtime guards and six corrected source inputs/preimages remain
intact. No old acceptance, build, full equivalence pair, unchanged81snapshot scan,
navigation, recording or checkpoint is repeated. Query/summary/source-audit JSON,
first failed preflight and both build preimages remain private under
`target/rom-data-script-obj-queries/`. Source/manifest/runtime/save/native ABI
stays e61-d60; USAe8-d7and new Linux/USA/native Windows/new32:9 remain pending.
No performance measurement or ordinary-equivalence claim is added.

A cheap independent dependency check also authenticates metadata14**logical
ID16** (not vector index16): its normal/alternate sprite fields are46/188.
The accepted080CBD24lookup matches unit byte4to selector16and the accepted
080CBAD4predicate routes nonnull unit+4>1to metadata14. This provides a concrete
next conditional producer lead for low script selectors, preserving normal/
alternate selection, overrides and current actor-field state. It does not turn
selector16into an unconditional sprite46binding or justify every script-group
by every sprite. This existing dependency is not re-extracted or replayed.

Final preservation passes8649prior hashes, both six-input compiled preimage sets,
1023/1024build and1025Wine guards. Only the two-line test registration/new
observer plus coherent memory documentation change;330local feature links,
new-module rustfmt, project-memory default/base7597b92 and whitespace pass.


## Conditional normal script poses for special-unit selectors

The next bounded source projection follows a proven join for the151low-selector
instructions, with explicit conditions: the resolved nonnull unit byte4equals
its logical selector2..94; the actor's current sprite retains that same unit's
normal CF144assignment; actor override flag20is clear; facing is0..3. This is a
conditional original source path, not a guarantee about the current sprite in
all script states. Alternate assignments, explicit42/230overrides, changed/stale
actor fields and the16high-registry instructions remain separate. These scope
conditions are selected before target inspection; no failed row is pruned.

Original CBD24compares live byte4to the low selector; CBAD4routes nonnull byte4>1
to metadata14. CF144uses that exact logical ID's halfword04. Original9B820calls
233A8→CF144, stores the result at actor+34and only overwrites it with42when flag20
is set. The new check pins27halfwords/one literal/three BLs for this join; prior
script49/constructor/facing/pixel-format acceptance is retained. No normal job
catalog or arbitrary sprite set is crossed with the script groups.

All151source instructions remain referenced by20selector/base pairs over14
logical metadata14IDs. Physical aliases reduce these to18normal-sprite/base
pairs over12sprite IDs, yielding108distinct two-orientation/three-part queries.
The groups use eleven bases0/4/8/16/20/24/32/40/88/136/180. They resolve108physical
cells/76real nulls/32proper sequences241frames,11OBJlists64objects and95indexed
4bpp requests35968B. There are no index-root target views, conflicting internal
roles, varying pixel extents or partially shared typed items in this projection.
No directory extent or animation limit is inferred from adjacent data.

Full direct sources cover41802B, concatenated SHA-256
`f64755ef3d0cfc13cb8286965b42e87fa2f7b8d450ecd86a4a97674241c0f4fa`.
Authenticated comparison with all56e61resource owners finds4780shared initial-pose
and30586shared normal-battle bytes, with no other owner overlap. The new source
plan is33spans6436B:240index/28sequence-header/920frame/5248pixel bytes. It contains
60new cells/53new nulls, seven new sequences46frames and17new pixel requests.
Every new frame is operation1; commands2/6/8occur only in existing shared data
and require no new original-command replay. Existing directory/OBJ sources keep
their owners. New source SHA-256 is
`6c3ef9c987b02ad1c06492ac245484b1574a45666cba90bff097c7e1e5cf5027`.
The two previously observed null cells37C760/37C764are contained in this stable
ROM-derived plan; no private replay state becomes extraction input.

The conditional source contract and lossless model with all151script references/
20producer witnesses and108deduplicated keys are closed. One new reduced
original-reader Wine oracle passes for the60new cells/seven proper resets/
17new pixel uploads. Earlier source/model/native/CLI/scene oracles stay closed.
Managed/runtime/actual CLI/ordinary use for this new plan are not implemented;
current EUR e61-d60/USAe8-d7are unchanged. No build, guest execution, new fixture,
navigation/checkpoint/private payload edit or performance measurement runs in
this static unit. Linux/USA/native Windows/new32:9 remain pending.
Private evidence is under `target/rom-data-script-special-poses/`:
`script-special-source-audit.json`, `script-special-source-requests.json`,
`source-plan-review.json`, `source-contract.json` and executed static helpers.


## Typed normal script-special pose model

The new [source model](../crates/gba-core/src/rom_data/unit_obj_script_special_normal.rs)
retains20producer records with exact logical selector/base, metadata14normal
sprite field and all151script-source references. Binding keys are deduplicated
only after preserving those witnesses;108keys/12roots/32sequences241frames/
11OBJlists64objects/95indexed4bpp requests remain inspectable. Existing shared
frame/command, OBJ and pixel formats are reused. No current actor, guest frame
or private recording is an extraction input.

Structural checks reject malformed identity/order/extents, unknown fields,
duplicate/missing instruction references and broken graph links. Dependency
validation separately rebuilds exact producer witnesses from the authenticated
script archive and logical metadata14IDs. Physical reconstruction remains
separate from source authentication. Shared initial/normal views are checked
independently even when their intervals overlap; subtracting their union leaves
33new spans6436B, without transferring older ownership or retaining raw ROM bytes.

The [new reduced Wine oracle](../crates/recomp/src/rom_data/script_special_normal_model_tests.rs)
passes first attempt:18strict input/source/dependency/shared-owner guards,
120new-cell constructor prefixes,53null/seven proper resets and17new pixel
scheduler/queue/real-DMA3 uploads, all5248new pixel bytes and wholeVRAM equality.
Full41802B and33newspans6436B match the independent source plan and both hashes.
Commands2/6/8are old shared data and their original acceptance is not replayed.
One affected Windows frontend test build passes without warnings; frozen
executable SHA-256 is `2eaa565389788c0b6ac7b47b777df51884f25620553a39039aaae70f01871c5d`.
15461instructions/494PCs/613nonDMAloadB are coverage only. No full allocator/native/ordinary equivalence claim,
managed/runtime generation or live input cap is introduced. e61/USAe8remain;
Linux/USA/native Windows/new32:9 and alternate/override/high-registry/current-state
cases stay open, with no performance/storage measurement or new gameplay fixture.


## Authorized ordinary equipment attack

On25October explicit user authorization permits local checkpoints derived only
through normal game keys from a copy of an EURcheckpoint, preserving originals
and cartridge saves with all outputs outsideGit. The retained interpreter helper
loads the copied state, advances ordinary keys/frames, and saves/roundtrips new
states; no rawRAM/HP/status/PC/save edits or new native cache are involved.

Copied F09frame8476 reaches Polly's Action/Fight menu and a genuine occupied
Hildegard target/forecast at8668via192normal frames. Read-only original cursor/
range and actor-unit-bodytask formulas distinguish real occupied(1,6) from two
empty probes. The first forecast-A500locator opens the separate Do it/Cancel
prompt and is equipment-negative; it is closed without a fullpair/repeat. A
changed input presses Aat0and48(releases4and52) to confirm that normal prompt.
This friendly attack exists only in the private copy and is an ordinary command.

Single actuale61production WineCLI500confirmed Fight passes4477aggregate typed
equipment bytes:8directory/16index/4header/276frame/2765OBJ/1408pixel, two sequences.
All original snapshots/cartridge files/cache/TCC/canonical executable/dataset
hash-mtime guards remain intact. Historical automatic review rejection is
superseded by explicit user authorization. The first navigation wrapper counted
its script in an output glob; actualWinehelper exited0/produced five valid
checkpoints and images. A cheap artifact review corrects that wrapper-only
classification without replaying navigation.

The every-frame scene filter now has an explicit equipmentpixel+sequence guard
(`FFTA_STATE_TEST_REQUIRE_UNIT_OBJ_EQUIPMENT=1`). One affectedWindowsfrontend
scene-guard build passes without warnings, frozen SHA-256
`a13b3aa63ee37beb2c71fe1fea126de1a4003d3e82b797ed0e1725e1d3388764`.
The one exact positive500frame Winefullpair passes first attempt: all500complete
states/clocks, central/wide images, OAM, native/fallback dispatch counts and all
fiveaudio streams match from8668to9168. Its3269equipment reads/4477aggregateB/
1408pixelB/twosequences exactly match the actualCLI locator. Declared equipment
source/model/managed/native/actualCLI/ordinary children are closed; the other256
sequences remain targeted-only, without wholeunit/allocator/actorABI or attack
damage guarantees. Existing negative locators,
source/model/managed/native/actualCLI and earlier families' scenes stay closed.
Private evidence is under `target/rom-data-equipment-ordinary/`. e61/USAe8remain;
newLinux/USA/nativeWindows/new32:9 and other ordinary sequences remain pending,
without performance/storage measurements.


## Conditional script-special normal managed runtime

The new integration wires `unit-obj-script-special-normal.json` as EUR
`graphics.unit-obj-script-special-normal` in schema1/e62-d61. Its direct declared
dependencies are metadata14, scriptarchive, initial-pose and normal-battle;
producer witnesses are authenticated against the logical metadata IDs and exact
script references, and shared bytes remain with their prior owners. USA stays
e8-d7. The new immutable owner serves only33newspans6436B, reconstructing typed
index/header/frame/4bpp pixel bytes without retaining a raw ROM shadow. The
original live actor selection, animation commands, scheduling, queue and DMA
remain guest work; no IDcap/save/native ABI change is introduced.

One affected Windows core/frontend test build passes without warnings; core is
compiled only. Two reduced new Wine filters pass first attempt. The source/model
oracle remains closed.
The managed filter covers prior-file/resource retention, dependencies, coherent
edits, transactional failed attach, publication lifecycle and unavailable-file/
load/Undo/backup/restart ownership. The native filter compares original raw-ROM,
typed interpreter and real compiled guest reads with complete states/clocks,
new null/sequence paths, two representative uploads+DMA, live RAM frames, all33
edges/widths/mirrors/BIOS and mixed new/prior/raw reads. All41802direct graph bytes
are poisoned to exercise shared owners, while only6436belong to the new owner.
Old commands2/6/8and prior source/census/native/ordinary proofs are not replayed.
Actual productionCLI now passes below. Ordinary use for this family and Linux/
USA/nativeWindows/new32:9 remain pending, without performance/storage measurements.


Managed EUR e62-d61verification publishes58files57resources, preserving all56
prior file/resource records and coverage plus retained e61/USAe8-d7.23attachguards
and6dependencyguards pass, including coherent edits and failed-attach owner/state
isolation. Interrupted publication/create/reuse/repair/move and unavailable-file
Load/Undo/backup/restart preserve the typed owner. Reading all6436newbytes under
complete41802graph poisoning gives exactly240index/28header/920frame/5248pixelB,
seven new sequences and no new directory/OBJ charges. Shared initial/normal
reads remain independently charged to those owners.

The new native filter passes23original-interpreter-compiled fullstate-clock
paths: part0proper sequences, part1/2realnulls, both orientations, fullword rawID,
live RAM frame, two uploads+realDMA, all33source edges/widths/mirrors/realBIOSbus,
CpuSet/FastSet/DMA and mixed reads. Native API new-role bytes are
[0directory,36index,6header,43frame,0OBJ,1pixel]; DMA is compared separately.
The mixed transfer has2new+4normal bytes with exact owner charges; unaligned
API source has2new+2deliberately modified raw bytes. New frames alloperation1;
old command acceptance is not repeated.1841instructions/496PCs/104dispatches
are functional coverage only. No fullallocator/actor/ordinary claim follows.

Dataset SHA-256 `f131b439ee62dc55ab0383c985f9c1c936a85088106bc4e16c3fab80ed4e69a2`,
manifest-file SHA-256 `2b84c40c03b0c4d81aa80b414f1d2271fd825093bcf94e68275883d89a464499`,
model-file SHA-256 `02e7362e2bb2bea7dad84bf769d5cb0c88b3e6efd3cc59fe77eb911da97e37c5`.
Frozen Windows frontend tests are `b47e885085e927959c388b065301e594f08a1b849c3ce42b291c7852defdaaac`;
core `b47c085aa25b4f588eb38052905997042dfcce27c78fba9356236b5613ca4e00` is compiled
only. Private evidence: `target/rom-data-script-special-runtime/`, managed/native
result JSONs, Windows build and two Wine logs. ActualproductionCLI passes below;
ordinary use remains pending, prior source/model/census/scene proofs not replayed.
Linux/USA/nativeWindows/new32:9 remain pending, with no performance/storage work.


## Conditional script-special normal actual production CLI

One changed-code Windows production build is warning-free, frozen SHA-256
`6455958ed9616b269d225a7f9da28f726cbb6624021c461baf567571f1c1c5d0`.
Canonical executable hash and nanosecondmtime1792570130397888519 are restored
exactly. Cargo invalidation touches only main.rs's mtime, preserving its contents.

Actual WineCLIcreate/reuse from an unrelated working directory verifies58files
against the accepted managed dataset; all60managed file hashes/mtimes remain
unchanged on reuse and attachment. Actual generation `generation-SHz1vX` keeps
datasetf131b439.. /manifest-file2b84c40c.. /model-file02e7362e.. identities listed
above, with all four dependencies. The unchanged authentic EURF09snapshot loads
oneframe with the retained native cache and new owner attached. That frame
reads zero new script-special bytes: compatibility smoke only, not an ordinary
source/pixel use or complete-scene equivalence claim. Earlier3530/500/census6000/
source-model/native/CLI/scene acceptances are not replayed.

Actual productionCLI criterion is closed. The retained Snow6000census observed
two new realnull cells37C760/37C764, providing an ordinary source-use lead. It
does not prove ordinary use of the seven new sequences/pixels; choose that
criterion explicitly before a new scene/fullpair. No new fixture, navigation,
privatepayload edit, performance/space campaign or otherplatform test occurs.
Private CLI evidence is under `target/rom-data-script-special-cli/`: build,
processes/no-write/summary JSONs, threeWine logs and one-frame endpoint image.
Linux/USA/nativeWindows/new32:9 and alternate/override/highregistry/current-actor
source cases remain open. Source/model/managed/native/actualCLI children stay
closed, with no unchanged repeats.


## Script-special ordinary source locators and remaining pixel gate

A single new actuale62WineSnow6000locator on the retained authentic snapshot/
input reads48newindexB and zero newheaders/frames/OBJ/pixels/sequences. This is
ordinary typednull-source use, consistent with the source-linked two new null
cells; it is not an every-frame differential or ordinary newpixel acceptance.
No actor-query census/source/native proof is repeated and no guardbuild/fullpair
is selected for this null-only observation. The seven new sequences remain
covered by their existing source/targeted native tests.

A new read-only search of81existing authenticated EURsnapshots for exactly those
seven sequences finds no enabled or dormant matching bodytask. Exact context/
manager/signedcursor reads in retained RIVERdeployment snapshots identify
program158/key168, which has no future newproducer within that program. The
retained Start/Atransition618with e62also reads zero newbytes; its archive use
is only5B/twoopcodes/oneprogram. Both locators are terminal in their exact scope.
Do not replay them unchanged or infer that every later scene is negative.

Program159/key169contains selector2/group34/face0 atBAD829, which is a concrete
new-source producer. Its invocation is still unresolved; provisional battle-end/
battle-start labels are not evidence of reachability. Original12547Areads the
selected key at2192, while09BE8queues a request at2194. Frozen RIVERfirst-turn
states keep selectedkey168with no active script manager. A copiedEURDS20cancelled-
flee B44normal menu probe also supplies no newsequence proof; two new snapshots
remain private, originals and cartridge saves unchanged. No further battle
completion or DS20failed-return acceptance is claimed or repeated.

Ordinary newsequence/pixel use and complete-scene equivalence remain open. Next
work can select an independent remaining producer contract, retaining this
missing gate rather than weakening it. Private new evidence and guarded inputs
are under `target/rom-data-script-special-ordinary/`. No performance measurements,
newLinux/USA/nativeWindows/32:9 tests or production code changes occur here;
concurrent Godot/frontend and completed userDS20work remain separate.


## Conditional alternate script-special source and typed model

The exact same151low-selector decimal49instructions now have a separate
alternate projection through metadata14halfword06and original233D8/CF180.
The conditions are selected before inspecting targets: a resolved nonnull unit
matches selector byte4, the original special-unit predicate selects metadata14,
its actor retains that same unit's actual alternate assignment under9B734,
override20is clear and facing is0..3. These witnesses do
not assert every actor's current sprite, global reachability or a terrain name.
The16high-registry instructions, explicit42/230override and stale/changed actor
assignments stay separate; no row is discarded or all-sprite product invented.

Twenty logical producer pairs over14selectors collapse to14sprite/base pairs
across188/231/232/234. Their84physical cells contain68nulls and14proper counted
sequences64frames. Four OBJlists20objects and26pixel requests7232B follow only
operation1; shared commands2/8keep their raw fields and prior reader acceptance.
All57e62resource file/source hashes are checked. No shared-index-root target,
role conflict, mixed old/new typed item or varying pixel extent occurs here.
The9048Bfull graph has SHA-256
`52ba9bc267436719dda66f7cc1628bf72a4f98310a276845b6bc1ecd994cab7a`.

Existing ownership is8142Balternate-battle,38Binitial-pose and52Bnormal-battle.
The new20spans816B comprise216index/8header/80frame/512pixel bytes:54cells52nulls,
two sequences/four operation1frames/two pixel requests. New proper bindings are
188/(32,33)/part0, with sequencesAD4950andAD4980. Concatenated new-source SHA-256
is`029ff8a0282346c25be4b436f7b5ea3e219e167d5639d497f7d495b0127a5e2c`.
Counts and original shape/size DMA arithmetic establish extents, with no guessed
bank end or frame-count threshold used to classify a target.

The [offline model](../crates/gba-core/src/rom_data/unit_obj_script_special_alternate.rs)
keeps strict producer/graph/JSON identities, indexed tiles and all original
command fields. It authenticates the exact EUR ROM and producer code, validates
each overlapping initial/normal/alternate typed view independently, then
subtracts their union. An unchanged second dependency cannot hide a mismatch
in the first. The
[source oracle](../crates/recomp/src/rom_data/script_special_alternate_model_tests.rs)
passes one ignored focused Winefilter for new physical cell constructor
aliases/null/proper resets and real scheduler/DMA uploads. One affected Windows
frontend test build is warning-free, frozen SHA-256
`3af29161ee386cf5a5e18ec3260032a14dbbca048151684af38c8db772fe38d2`.
An initial Wine startup failure occurs before the test because sandbox access
to the local server lock is read-only; its log is preserved. The same binary
then passes its first actual filter execution with the required server access.

Nineteen strict JSON/source/producer/shared-view guards pass, including altered
initial/normal views whose bytes also occur in alternate. All84keys and all9048
source bytes reconstruct exactly; the new20spans816B match the independent
source plan. Only the54new physical cells execute original constructor paths,
with both facing aliases:108prefixes,52null resets and two proper resets. Two
new original scheduler/queue/DMA3uploads match entire VRAM and all512newpixelB.
Existing shared commands2/8are not replayed. The284observed nonpixel load bytes,
492PCs and7422instructions describe coverage, without whole allocator/task ABI,
complete-state/native-clock or ordinary reachability claims. Private evidence is under
`target/rom-data-script-special-alternate-poses/` and
`target/rom-data-script-special-alternate-model/`.

At this source/model boundary managed/runtime/actualCLI and ordinary newpixel/
fullscene gates were pending, with production e62-d61 and USAe8-d7. Managed/
runtime and actualCLI are now closed below; ordinary and newLinux,
nativeWindows/32:9 tests remain pending. No game fixture, navigation, payload
edit, performance or storage measurement is selected for this source unit.


## Conditional script-special alternate managed runtime

EUR schema1/extractor63/decoder62 adds `unit-obj-script-special-alternate.json`,
resource `graphics.unit-obj-script-special-alternate`. Its direct dependencies
are metadata14, scriptarchive, initialpose, normalbattle and alternatebattle;
there is no artificial dependency on conditionalnormal. The new59generation
files/58resources retain all57earlier resource/coverage records and57earlier
decoded files, while e62remains available and USA stays schema1/e8-d7. The
accepted model is unchanged, file SHA-256
`96ce4c28cb19366fbb8804e3784782a1a4682551926b12315ae88fafed5ad035`.

The [immutable runtime owner](../crates/gba-core/src/rom_data/runtime/unit_obj_script_special_alternate.rs)
serves only20new spans816B, reconstructing typed index words, sequence counts/
opaque headers, command fields and indexed4bpp tiles. All initial/normal/
alternate shares retain their existing owners. Candidate pages386/391/89D/AD4
only select a possible reader; exact per-byte segments establish ownership,
including mixed reads through older owners and mutable raw holes. Temporary
source reconstruction buffers are dropped after authentication. One64bit
sequence-use bitset covers the14full model sequences; counters remain transient.
No raw ROM shadow, liveIDcap, task/animation rewrite, save-state or native ABI
change is introduced. Original selection, RAM task pools, scheduling, commands,
DMA, VRAM and palettes stay active.

The [new runtime oracle](../crates/recomp/src/rom_data/script_special_alternate_runtime_tests.rs)
implements two focused ignored Winefilters: publication/authentication/lifecycle
and original-interpreter/compiled complete-state-clock paths. One affected
Windows core/frontend test build passes without warnings, core compiled only
SHA-256`42a6a5cb05b68eb1e2dcf72c6051c8020d8041689be181aecf4073b48d13bb18`,
frozenfrontend`38d207c59262a94b883cc42b6f5bc909a262c02fd8af7ec80df985ff5f85598f`.
Both new exactWinefilters pass on their first executions with required access
to the local Wine server.24attachment+7dependency guards, interrupted
publication/create-reuse-repair-move, failed-attach owner/fullstate isolation
and unavailable Load-Undo-backup-Restart/sharedowner lifecycle pass. Every816
newtypedB reconstructs directly:0directory/216index/8header/80frame/0OBJ/512pixel
bytes, two sequences. Earlier initial/normal/alternate probes each charge4Bto
their existing owner, without charging the new owner.

Twenty-four original-interpreter-compiled paths match complete machine states
and guestclocks with all9048graph bytes altered, including shared older owners.
Part0proper sequences, parts1/2realnulls, two orientations, fullwordrawID, live
RAMframe, two uploads+DMA and all20boundaries/widths/mirrors/realBIOSbus/SWI/DMA
pass. Mixed copies charge2new+4alternateB; the unaligned API preserves2edited
rawBbeside2newB. Compiled aggregate newrole bytes are`[0,36,6,43,0,1]`; actual
DMA is checked separately.1842instructions/496PCs/105dispatches are coverage
counts, with no whole allocator/scheduler/actor ABI claim. Source/model/old54cell/
source-DMA and shared command2/8acceptance are not replayed.

Dataset SHA-256 is
`4ff51b24688cf1badef741c6b48619a7e20a0b781e8f62f2781453f2375cd3ec`;
manifest-file SHA-256 is
`cab6ea47dcf77ed81275f693bc921825458afc9fe0c772f98d3e61d55bf6401c`. Actual productionCLI and ordinary newpixel/fullscene
coverage stay separate, as do high16/42-230/currentactor contexts and newLinux/
USA/nativeWindows/new32:9. No performance/storage measurements or gameplay
checkpoint/navigation/payload changes are selected. Private evidence is under
`target/rom-data-script-special-alternate-runtime/`.


## Conditional script-special alternate actual production CLI

One new changed-implementation Windows production build is warning-free, frozen
SHA-256`36cba67d8e4094f2620a99aa97a055d3b004927f5167ee6e9f66653c35060ee9`.
The canonical executable is restored in finally with exact hash/mtime; main.rs
content stays unchanged and only its mtime invalidates Cargo's restored-output
assumption. The accepted source model and runtime implementation are unchanged
from their successful tests; no source/native/scene test is replayed.

ActualWine productionCLIprepare/reuse and one-frame oldcacheattachment all exit0
from unrelatedcwd. The new e63generation has59files/58resources and61managed
files including current/lock; every generation file is byte-identical to the
accepted managed dataset. Reuse and attachment preserve all protected hashes
and timestamps, including the previously retained e62generation, ROM, cartridge
saves, authentic states/input, nativecache/TCC and canonical executable.

The dataset remains`4ff51b24688cf1badef741c6b48619a7e20a0b781e8f62f2781453f2375cd3ec`,
manifestfile`cab6ea47dcf77ed81275f693bc921825458afc9fe0c772f98d3e61d55bf6401c`,
model`96ce4c28cb19366fbb8804e3784782a1a4682551926b12315ae88fafed5ad035`.
One immutableF09frame attaches the new owner but reads zero new source bytes;
this is compatibility smoke, without ordinary source/sequence/pixel or fullscene
equivalence evidence. Positive ordinary newpixel use remains pending and must
be observed before selecting a complete-scene comparison. Source/model/managed/
native/actualCLI children stay closed without unchanged retries. Private evidence
is under `target/rom-data-script-special-alternate-cli/`, including build/
process/no-write/summary JSONs, logs and the one-frame endpoint image.
Linux/USA/nativeWindows/new32:9, high16/42-230/changed-currentactor cases and other
ordinary gates remain pending. No performance or storage measurements occur.


A new read-only search of81existing authenticated EURsnapshots for exactly the
two new proper sequences AD4950/AD4980(binding188/32-33/part0) finds no enabled
or dormant matching bodytask. This checks different targets from the closed
606alternate/sevennormal scans; neither is rerun. Every snapshot envelope/core/
payload/ROM and before/after hash+mtime is checked, without guest execution,
navigation, new checkpoint or payload edit. The source producer remains exact
selector3/group8/four archive references; current snapshots supply no replay
lead. The locator is terminalnegative in its scope, with no unchanged retry or
guardbuild/fullpair. Ordinary newsequence/pixel/equivalence remains open while
independent remaining override/highregistry source contracts can advance.
Private evidence is `target/rom-data-script-special-alternate-ordinary/`.


## Conditional forced42 and230script pose source model

Original9B734stores a normal/alternate sprite into actor+34, then flag20forces
normal42or alternate230at9B830/9B818. The source projection selects its conditions
before targets: a resolved nonnull actor retains that same unit's original
forced currentassignment under the actual mode branch, flag20is set and facing
is0..3. It does not assign a static unit identity to low/high script selectors,
assert every actor/action reachable, invent a terrain name or change a live
selector cap. Flag20clear/current changed/stale/highregistry source cases remain
separate.

The167authentic decimal49references supply13exact group bases. Pairing those
with only the two original forced constants produces26conditional mode/base
witnesses and334reference witnesses over156physical body/extra-part cells.
There are126realnulls,28proper counted sequences140frames, eight OBJlists27objects
and42pixel requests8000B. Every key remains; known indexroot targets from all six
current unitOBJmodels are classified before parsing sequences. No shared-root
target, role conflict, mixed old/new typed item or varying pixel extent occurs.
All58e63owner file/source hashes,19halfword/twoBL/five producer-code fingerprints
pass; prior producer evidence is read, without its old guest campaign replay.

The full11722Bgraph has SHA-256
`6d0850006ab9c7225eb5930af6cd5f0d9f8737c84024239ff251c04ca6d8e323`.
Previous ownership is6252normal-battle+4420alternate-battleBonly. New40spans1050B
comprise384index/24header/360frame/26OBJ/256pixel bytes:96cells90nulls/six sequences/
18frames(16operation1/twooperation2), one list4objects/two pixel requests.
New-source SHA-256 is
`e47c73e8b278c7894b5f6fa2fb875850ef91474fa0e1a9779bf72ad7a2d5488a`.
Extents follow counts and original shape/size DMA arithmetic, without neighboring
pointer guesses, count-based target classification or removed queries.

The [offline model](../crates/gba-core/src/rom_data/unit_obj_script_override.rs)
uses a strict Normal/Alternate mode tag and exact original assignment provenance.
Both modes mirror the same actual group/reference sets; each mode covers all167
script references. It preserves typed frame words/tails/OBJattributes/indexed
tiles, validates shared normal/alternate views independently and subtracts their
union. Dependencies are the script archive and two actual prior pose views;
metadata14/unit identity is not an invented producer dependency. Production
revision stays e63-d62/USAe8-d7at this source/model boundary.

One [focused source oracle](../crates/recomp/src/rom_data/script_override_model_tests.rs)
is implemented for192new physical-cell constructor aliases/90null+6proper reset/
two newoperation2argument copies/two newpixel realDMAuploads256B and strict
mode/reference/JSON/source/sharedview guards. One affected Windows frontend
build passes without warnings, frozen SHA-256
`5c3a9ae9dde8d944141d272960be3effb1f2b864b071ba56c246fbe046a79ef4`.
The single new exactWinefilter passes on first execution:19guards/all156keys/
all11722Bfullgraph/40newspans1050B,192original constructor aliases over96new
cells/90null+6proper resets/bothnewoperation2argument copies/two actualDMA3uploads/
all256newpixelB/entireVRAM. The458observed nonpixelB/482PCs/12422instructions are
coverage only, without whole allocator/task/native-clock or ordinary proof.
Accepted model-file SHA-256 is
`27d6d0e0683f9cceb17155fbbac8c35fce9db1aceffde8c144ec66f00d1f8698`. No prior command/source/native/scene acceptance, gameplay fixture,
navigation, private payload edit or performance/storage measurement is repeated.
Private evidence is under `target/rom-data-script-override-poses/` and
`target/rom-data-script-override-model/`. Managed/runtime/actualCLI/ordinary and
newLinux/USA/nativeWindows/new32:9 remain pending, without wholeallocator/task ABI
or universal currentactor reachability claims.
