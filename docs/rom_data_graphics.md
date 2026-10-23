# Decoded graphics formats and source contracts

The next resource work units extend the OBJ metadata prototype toward actual
graphics. They provide bounded offline LZ77, packet and descriptor decoders,
with verified source ranges and original-instruction consumer checks.
Descriptor metadata, base palettes and alternate palettes now belong to the
managed dataset and are directly consumed. Descriptor-linked packet catalogs
are also verified below, with typed ordered programs in the managed e8file.
Their actual source/body reads now consume those files for EUR/Wine. New
Linux/USA consumer checks and other linked graphics/programs remain open.
The [inventory](development/ROM_DATA_INVENTORY.md) and
[full TODO](development/ROM_DATA_TODO.md) retain the remaining catalog,
semantic decoding, extraction and runtime integration work.

## Observed producers

An optional test follows 1200 clean-boot frames with standard demo input, without
a save or forced PC, independently on each exact supported regional image.
It records actual executed BIOS decompression instructions through `StepEvent`,
not a guessed PC before interrupt dispatch. ROMs are read-only and all output
goes into a new private directory. The producer trace and post-SWI destination
bytes match byte-for-byte between Linux and Windows GNU/Wine for each region.

Each trace observes four LZ77 WRAM calls (SWI 0x11), no other decompression SWI:

- USA frame 801: source 0x56f4a8, 125 consumed bytes -> 160 decoded bytes at
  IWRAM 0x3860. Frame 803: 0x5704cc, 133 -> 456; 0x56fefc, 1481 -> 1996;
  0x56f52c, 2507 -> 2960, each into EWRAM 0x3cb0 scratch at its respective
  call. The source extents exclude padding.
- EUR frame 1161: 0x76c64c, 125 -> 160 at the same IWRAM destination.
  Frame 1163: 0x76d670, 133 -> 456; 0x76d0a0, 1481 -> 1996;
  0x76c6d0, 2507 -> 2960 at EWRAM 0x3cb0.
- The actual wrappers are 0x08141890 USA and 0x08145254 EUR. Caller link
  registers are 0x0801a4a9/0801f519/0801f3f1/0801f295 USA and
  0x0801b619/08020689/08020561/08020405 EUR, in the order above.

The four compressed streams and decoded outputs match across the two regions:
4246 consumed source bytes and 5572 decoded bytes per region. Decoded SHA-256,
in call order: 2a360ec828fb53a6becc94e8f1c3aa9c6044821639c44ad393b97c66ba0a6b44,
8f768d8eda371a2fa1a5fbe5b72a36f61f286e40f14088730013fdb44d824235,
3e9599c722762edd19e059a5d0bbb51e1eb907f94c44e17ebe7d8445df54429f,
3d379876034eec8b762e05ad7b8d987007b5c89d3369c0c5e05b3b8e2ccabc42.
These are exact stream-consumption bounds derived by the decoder, not a claim
that padding belongs to the payload or that every graphics source was visited.

Independent original caller disassembly also identifies a shared relative
descriptor layout: 88-byte stride, root USA 0x569104 from literal 0x1a4ac and
root EUR 0x7662a8 from literal 0x1b61c. Field 0x0c in record 0 resolves the observed 160-byte
output's source. Fields 0x04/08/10 in record 0 resolve outer `11ffffff`
wrappers with the observed LZ77 streams four bytes later. Their decoded packet
formats are described below. The contiguous 162-record metadata block and its
selector contract are now verified below. Linked payload/program formats,
other graphics catalogs and language completeness remain to be established.
Do not infer catalog or language completeness from equal initial outputs.
Scratch output preserves packet headers; the typed body decoders below preserve
ordered writes rather than declaring those outputs to be flat tile images.

## Authenticated descriptor metadata

`gba_core::rom_data::bg_descriptors::Catalog::from_rom` accepts only the exact
supported images and checks each original root literal plus the descriptor
source hash. The block is 162 records of 88 bytes, 14256 bytes total, at USA
0x569104/EUR 0x7662a8. Its SHA-256 is
2766525e0889e0ec2f3646dda7a95c143fe68931ee19923b6808b5d989519ce6.
Every mandatory and optional relative link in this block is outside the
descriptor region and within the image. Each active transfer has both sources
and an aligned VRAM destination. The lowest referenced payload starts exactly
at relative 0x37b0; treating that payload as another descriptor fails the
mandatory source bounds. This establishes the supported contiguous block,
without asserting that it contains every graphics asset or catalog.

The typed model preserves all 88 bytes:

- Offsets 0x00/04/08/0c/10 are root-relative sources: the first graphics producer,
  paired-plane packets, halfword-plane packets, palette and auxiliary patches.
  The first source's full format remains unclassified.
- Offsets 0x14/18/1c form an optional control-source/data-source/absolute-VRAM
  transfer triple. Four further triples use the arrays at 0x20, 0x30 and 0x40.
  A zero triple is absent. Payload lengths and transfer-program semantics are
  separate work; checking link addresses does not prove complete transfer bounds.
- Offset 0x50 is an optional root-relative variant-program source. Original
  readers both test its presence and add it to the root; it is not a boolean.
- Bytes 0x54/55 are conditional selectors, not dimensions. The original getter
  chooses the second when the environment byte at EWRAM 2fc2 has low bits 1 or
  2 and that selector differs from ff; otherwise it chooses the first. Their
  game-level names remain unverified. Byte 0x56 indexes alternative palette
  tables in original loaders. Byte 0x57 is retained without assigning a meaning.

`Descriptors::decode/validate` bounds record count at 512, rejects partial/empty
records, inward/out-of-image source links, partial transfer triples and invalid
VRAM destinations. `source_bytes()` reconstructs the metadata exactly; it does
not reconstruct linked assets. Serde rejects unknown keys, while the explicitly
unclassified metadata bytes retain their source values.

Three synthetic tests pass on Linux and Windows GNU/Wine for typed roundtrip,
selectors/fallback/low-bit masking and malformed counts/links/transfers. Four
exact-ROM cases also pass, one per region/target. They reconstruct the source,
refuse an extended prefix and an edited base ROM, and produce byte-identical
106309-byte TOML files (SHA-256
e408f8122bc7689e02c72d6e5844b6176d217f25d921afc35a8f2980b7c60f81).
Each case executes 2612 original selector calls/134629 instructions: all 162
IDs, eight environment values covering upper-bit aliases, direct/current-ID
calls, ff sentinels and a post-authentication in-memory ff-fallback probe.
All EWRAM remains read-only. The fallback probe restores its changed ROM byte;
edited base data remain rejected. There is no boot replay or guest-code copy.

Evidence: ignored `target/rom-data-bg-descriptors/` structural-boundary.json,
core/runtime build metadata/logs, native/windows results and original-selector
logs, four private TOML/report directories and verification-summary.json.
The initial standalone files above were private test outputs. The later managed
integration below publishes the same metadata under its explicit file budget
and dataset revision. All derived data stay outside distribution.

## Managed descriptor extraction and preparation

`rom-data extract|verify|prepare` now handles the descriptor file together with
OBJ dimensions. Schema stays 1, extractor/decoder become 3/2, selecting
`v1/e3-d2` while retaining earlier generations. The three fixed names are
`manifest.toml`, `obj-dimensions.toml` and `bg-descriptors.toml`. Manifest and OBJ
keep 16 KiB caps; BG gets 256 KiB and the whole set gets 272 KiB. No manifest
path controls file access. All files are serialized within budget, synced and
read back before atomic publication or selection. Reuse verifies the complete
set and performs no writes; missing/corrupt BG data trigger a fresh generation.

The manifest pins both family IDs, formats, hashes and source ranges. Typed BG
values must reproduce the actual authenticated metadata, even if an edited
file's hash was recomputed. Activation checks both files before changing the
machine, and every prepared resource ID/file hash contributes to dataset
identity. Equal semantic BG data with a comment and a new file hash changes
dataset identity while preserving OBJ file identity and guest snapshots.
The managed unit authenticated BG but did not yet consume it. The subsequent
[direct runtime unit](#direct-descriptor-consumption) now retains both typed
families and serves BG source reads from file-derived data. Preparation alone
was not treated as guest-use evidence.

Affected release checks pass: Linux2core extraction+4core runtime+7file+10
preparation cases, Wine2+4+6+9; two Unix symlink checks explain the difference.
Two further private tests per target cover both exact regions, actual command
handlers, source-authenticated activation and existing Load/Undo/backup restore.
Rehashed edits/missing BG leave the live object and guest snapshot intact.
All resource hashes participate in identity. The version-isolation test imports
a real retained e2-d1dataset and proves it stays unchanged; no legacy fixture
is regenerated. Simulated partial BG writes, three-file process-death cases, size/total-budget,
concurrency, corruption/missing repair and relocation cases pass.

Production CLI builds are warning-free. Each actual Linux/Wine executable passes
18checks from unrelated working directories: help, regional first extraction/
reuse/verification, managed create/no-write reuse, corrupt/missing BG recovery,
resource-root move/reuse and rejection/retention of the EUR legacy dataset.
Three output files are byte-identical across targets for each region;
logical set size is107861bytes. Two synthetic tar/zip archive scenarios also
exclude new BG markers beside selected package inputs. Canonical executables
are restored by SHA-256 and source mtime refreshed for later Cargo builds.
No archive rebuild, export or AndroidPort change is inferred.

Private evidence: `target/rom-data-bg-managed/verification-summary.json`, core/
file/preparation/regional/lifecycle logs, production build logs/candidates,
native/windows results, imported old-generation copies, three-file datasets,
CLI operation logs/results and package marker log. Native Windows/physical
power loss, combined-dataset GUI and new startup/frame costs remain unverified.
The previous OBJ scene/performance/GUI/archive outcomes are retained at their
old revisions; no unchanged scene is replayed just for context.

## Direct descriptor consumption

ROMDATA-04-BG-CONSUMPTION retains the authenticated typed metadata and a
14256byte exact source projection in the immutable session. Fixed ROM-page
checks coverUSA569..56c/EUR766..769alongside the four OBJ pages. CPU reads,
DMA and RtApi loads use the projection; neighbors, mirrors, GPIO/EEPROM and
open-bus ordering retain their existing behavior. No hot I/O/allocation,
original instruction/decompression skip or new runtime lookup by manifest is
introduced. Linked payloads and programs still use ROM.

Schema/extractor/decoder remain1/3/2:the file contract is unchanged. Source,
save/native-cache identity, RtApi and state schema1 stay compatible for these
source-equal base data. Distinct BG activation/file/dataset diagnostics and
bus-operation/projected-byte counters expose actual use. Owned limits were
set before testing:shared/OBJ<=1KiB including Arc, additionalBG<=64KiB.
Both targets measure440+28672=29112bytes. Allocator/RSS/startup/frame-time costs
are separate05work, not inferred from type-size accounting.

Linux and Windows/Wine release suites pass204core tests each (7optional
ignored) and98/97frontend defaults (18optional ignored). Three new synthetic
core cases cover every byte/width/mirror/boundary/native callback, CPU loads,
16/32bit DMA, guest timing and transactional snapshots with poisoned raw BG
source bytes. The shared bus change is the reason for the full core suites.

Four real-file regional cases reuse existing e3datasets and compile only the
original selector/query functions through the production decoder/emitter/
TinyCC loader. Each executes2608calls/134424original instructions, compares
registers/CPSR/clock/EWRAM/IWRAM at native boundaries and every interpreter
step, and ends with equal complete snapshots after restoring only the test's
raw-ROM poison. Each reports3888BG reads,3867from translated byte-load
callbacks,5836native dispatches and660legitimate event-boundary interpreter
shells. These are actual native loads, not an API-only synthetic claim.
Sentinels and all162records/eight environment values/direct-current arguments
are covered; the earlier offline fallback counterfactual is not rerun.
Load/Undo/backup preserve the immutable owner/counters; corrupt/missing new
activation fails transactionally, while current reads survive removal of the
BG file from the directory used to activate that owner.

Two new EUR native1200-frame clean boots independently match every full state,
central/wide image, OAM, dispatch and complete mixed/PSG/FIFO/HLE stream, with
53BG reads and all five audio streams nonempty. HLE compares float bits.
Linux6479719/Wine4067051native blocks execute, with each platform's existing
TinyCC ELF object. No cross-target dispatch/timing equality is inferred.
Actual warning-free production executables pass updated help, interpreter
1200-frame demo pairs and native CLI1200-frame idle/explicit-demo pairs. Each
pair matches its legacy framebuffer hash; both native pairs also match final
full-state hashes. Demo interpreter/native runs each report53BG reads. With no
input,runc correctly reports0; its explicit demo script is required to leave
the initial screen. The first CLI helper incorrectly demanded positive reads
without input; its completed frames/idle runs are retained, and only the new
input pair is executed. Existing e3files are reused without writes during that
pair; canonical/ROM/library hashes match. No extraction fixture is recreated.

The frontend helper initially omitted dispatcher frame-ready clearing and
stalled at a legitimate event yield. Only that harness was corrected; the
negative output is retained. Wine's background D-Bus diagnostics coexist with
successful test exits; no production workaround is introduced.

Private evidence:target/rom-data-bg-runtime/verification-summary.json,
core/default/region/native-boot logs, result.txt reports, bounded component
objects, frozen test/build metadata, source hashes and production candidates.
No game code/data or these artifacts enter Git/packages. Broad BG scene/GUI/
cost/archive coverage, linked asset/program formats, other catalogs/languages,
normal enablement, native Windows and mods remain open. Old OBJ outcomes
retain their frozen revision scope; they were not rerun for context.

## Readable JSON datasets

The current managed contract is schema1/extractor4/decoder3 at `v1/e4-d3`.
`bg-descriptors.json` uses `bg-descriptors-json-v2` and standard two-space
pretty JSON; manifest/OBJ remain TOML. Direct typed deserialization avoids a
whole intermediate document tree. The model, exact source projection, family
ID, bus/native/state behavior and immutable session owner are unchanged.
Earlier e3 TOML generations and e1/e2 remain in separate directories, unmodified.
No format fallback silently interprets a previous dataset as the new revision.

The user explicitly removed space-minimization as a priority. The one-space
prototype was therefore not adopted. Current BG output149261bytes and complete
generation150818bytes are kept for readability; former128KiB disk performance
budgets are historical observations, not reasons to pack/compress the data.
Parser input bounds remain16KiB manifest/OBJ,256KiB BG and272KiB total as
malformed-input guards. Previous e3 RSS results remain recorded; no extra
benchmark/RSS campaign is run solely to satisfy the old space target. The
isolated parser heap result is not a claim about current whole-process RSS.

Release affected checks pass32Linux/30Windows-Wine including private cases:
2core extraction+7runtime+8/7file+10/9preparation defaults, regional command/
identity/old e2 preservation, original Load/Undo/backup, actual old e3 TOML
retention and two regional JSON-native provenance/lifecycle tests per target.
The new parser rejects unknown/duplicate/missing fields, invalid UTF-8,
truncation/trailing data, numeric overflow, bad arrays and empty records.
Rehashed semantic edits still fail source authentication. Output is deterministic
and typed source bytes match the previous catalog. Native component cases retain
2608calls/134424instructions,3867native BG callbacks/3888total reads each.

Both production builds are warning-free and pass18actual CLI checks per target:
regional extract/reuse/verify, managed create/no-write reuse, corruption/missing
repair, move from unrelated cwd and prior e2 retention/refusal. New e3 retention
uses the actual existing three-file fixture; no old dataset is regenerated.
Current output files are byte-identical across targets. Actual interpreter and
native CLI1200-frame demo pairs match legacy framebuffer hashes, and native
final full-state hashes also match; both report53BG reads. Ready-generation
file bytes/mtimes, ROM, inputs, native caches and canonical runtime hashes stay
unchanged. Two tar/zip marker scenarios exclude both old TOML and new JSON data.

Evidence:target/rom-data-bg-json/verification-summary.json, final build/test/
regional/revision/provenance logs, actual three-file datasets, CLI/runtime
results, protected hashes and package marker log. No guest ABI/state/native-key
change, compression, archive export, AndroidPort update, agent commit or push.
Broader BG GUI/archives, actual payload/catalog migration and native Windows
remain separate work; completed scene/timing results are not rerun for context.

## Base background palette catalog

ROMDATA-01-BG-PALETTES inventories every palette link in the authenticated
162-record descriptor block for each exact region. There are127distinct
sources each:125LZ77tag10and2tag0raw blocks. Record5uses one raw block;
records54/56share the other. Unique source extents total15612B excluding
alignment padding; decoded payloads total23008B/11504little-endian u16words.
All source/decoded hashes and alias membership match across regions.
Sizes64/128/160/192/224/256B are multiples of32, retaining groups of16words.
476unique-source words (480in the record-indexed catalog) have bit15set;
retain all16bits and do not flatten them to RGBA/alpha or mask flags.

Original producer entries are independently located atUSA0801a42c and
EUR0801b59c. With environment byteEWRAM2fc2low bits0/3, the producer reads
field0c's relative source and checks its first byte. Tag10calls the existing
LZ77wrapper; the raw branch skips the4Bsize header and calls CpuSet for
size/2halfwords. All known raw sources have tag0; other tags are not declared
supported by this catalog. The current-ID argumentFFFFreadsEWRAM7f10.
Environment low bits1/2instead select distinct alternate-palette program
tables via descriptor byte56. Those programs remain unclassified, not
silently treated as base palette colors.

Four original execution oracles pass:each region on Linux and Windows/Wine,
162records×4base environments×2direct/current arguments =1296calls and
72792original instructions per case. All output atIWRAM3860matches the strict
LZ77decoder or validated raw slice, with complete surrounding guards, unchanged
EWRAM and restored SP. High bits are preserved exactly. Record0also matches
the four existing original160Bboot captures without a new boot replay.
No complete clock/native/scene equivalence or runtime palette replacement is
inferred from this producer-output oracle.

Evidence:target/rom-data-bg-palettes/verification-summary.json, header-inventory,
per-region source/output hashes and private decoded buffers, exact original
producer disassembly, warning-free test builds and four oracle logs/results.
That inventory unit added only cfg(test) code, with no producer/HLE change.
The typed model and subsequent runtime integration are recorded below.
Alternate programs, other catalogs/languages and the remaining matrix stay open.

## Typed palette extraction and preparation

ROMDATA-02-BG-BASE-PALETTE-DATA adds `bg_palettes` with completeu16banks of16
words and record-indexed palettes. Both raw tag0and LZ77tag10have exact
consumed extents, excluding padding. Decoded sizes must be nonzero multiples
of32B, at most512B/16banks; truncated/unsupported sources and malformed banks
are rejected. Preserve high bits and all bank/index order, without RGBA/alpha
conversion. All162records are decoded, including127source aliases; no
compression/deduplication is needed for file size.

The completed preparation unit used schema1/extractor5/decoder4 at `v1/e5-d4`
with four fixed files:
manifest/OBJ TOML, BG descriptors JSON and `bg-base-palettes.json` using
`bg-base-palettes-v1-json`. Palette resource ID is `graphics.bg-base-palettes`,
dependent on descriptor IDs. Its162manifest provenance entries preserve every
record's source offset/extent/hash/role, including aliases. Every typed bank
matches independently decoded source and retained original output. Rehashed
palette edits remain invalid base data. All IDs/files participate in one
source-authenticated dataset identity through a borrowed complete-file view.
A semantic-equal whitespace change to the palette file changes dataset identity
while OBJ/BG identities and complete guest snapshots remain unchanged.

Input bounds64KiB manifest/16KiB OBJ/256KiB descriptors/512KiB palettes/768KiB
total protect malformed data; the manifest bound grows for palette provenance.
Readable palette JSON is270075B and complete USA/EURsets446114/446120B.
No space tuning or extra budget campaign follows the user's preference.
Four-file write/sync/validation precedes atomic publication/selection; partial
palette writes, cancellation/death and missing/corrupt repair preserve committed
or prior generations. Actual old e4three-file generations remain intact and
are refused by the new full-set verifier; new data use a separate revision.

Affected release defaults32Linux/30Wine and seven private cases per target
pass (39/37total). Three palette-core tests cover exact raw/LZ77consumption,
endianness/high bits and malformed headers/sizes/tags/banks. JSON/source tests
reject overflow, unknown fields, bad arrays/counts, missing files, forged paths/
dependencies/ranges and rehashed edits. Regional data equal retained buffers
for all162records. Four original producer oracles now consume the new typed
model as expected data and retain1296calls/72792instructions each, matching
all output/guards and unchanged EWRAM/SP. The original producer code/control
has not changed. Existing native BG metadata provenance and Load/Undo/backup
checks also pass with the expanded complete-file API.

Both actual production builds are warning-free.22CLI checks per target pass
regional extraction/reuse/verification, creation/no-write reuse, BG and palette
corruption/missing repair, unrelated-cwd relocation and old e2refusal/retention.
All files match across targets; earlier OBJ/BG file bytes are unchanged. Actual
interpreter/native1200-frame pairs match legacy framebuffer hashes and native
final full states, with53BG reads. These prove unchanged existing consumers;
palettes explicitly report `prepared ... runtime=rom`, not file use by gameplay.
Source/input/cache/canonical hashes and ready-generation mtimes are protected.
Tar/zip marker scenarios exclude the new palette data, without a public export.

Evidence:target/rom-data-bg-palette-data/verification-summary.json, source
preimages/hashes, core/default/private/regional/oracle/provenance logs, four-file
CLI datasets/results, runtime pairs, old revision imports and package marker
log. This was the preparation-only outcome; the subsequent consumption unit
below establishes runtime use and its distinct verification scope. Alternate programs, other catalogs/languages and
the remaining resource matrix remain open.

## Direct base-palette consumption

ROMDATA-04-BG-BASE-PALETTE-CONSUMPTION retains the authenticated model in the
immutable session. Palette v2JSON adds `encoded_source`, the exact consumed
original header/stream bytes beside banks. No recompression is performed:
unused flag bits and byte reads must remain source-equal, including for guest
BIOS code. Model validation decodes this metadata, checks exact consumption
and requires bank equality; source authentication separately rejects coherent
re-encodings with the same decoded output. Schema1/extractor6/decoder5 selects
`v1/e6-d5`, retaining e5/e4 and all earlier data. Four filenames and semantic
resource IDs stay unchanged. Current input guards are64KiBmanifest,16KiBOBJ,
256KiBdescriptors,1MiBpalettes and1280KiBtotal. Standard readable palette JSON
is513639B; complete sets689678USA/689684EURB. These bounds protect malformed
input; no storage tuning follows from them.

All CPU/DMA/native reads in verified palette spans come from files. Fixed asset
pages keep catalog searches away from ordinary code reads. Unique sorted source
bindings preserve aliases and crossing neighbors. Raw payload bytes derive from
decoded banks; headers/compressed bytes use the exact source representation.
The BIOS LZ77HLE returns an owned borrow of decoded output only for verified
LZ77source starts/mirrors. It calls the unchanged ordered `write_out`; raw
CpuSet, other ROM and dynamic RAM retain their loops. Source ROM reads here
have no device/clock effects and the same SWI/instruction cost/return hook runs.
No guest instruction is bypassed, no native ABI/cache or snapshot revision
changes. Exact real-BIOS source views are verified, not a BIOS-ROM execution.

Measured owner bytes are448shared/OBJ+28672BG+94713palette=123833total on both
64-bit targets. No per-read allocation/I/O or new palette memory budget is
introduced. Resident accounting is distinct from allocator overhead and RSS.

Release checks:210core each and101Linux/100Winefrontend defaults pass; three
new core cases verify widths/mirrors/edges, native APIs, DMA16/32, raw CpuSet,
SWI11/12toRAM/paletteRAM/VRAM/OAM, fallback and snapshot/ownership behavior.
Four regional original/native cases each pass1296calls/72792instructions /
4208native dispatches with entire original palette source spans poisoned after
activation. Full states match; each owner records1272LZ77calls/231936decoded
bytes and3544source bus reads/5720source bytes. Missing/corrupt activation
preserves the owner; deleted files do not prevent subsequent use or Load/Undo/
backup clean boot. Current regional extraction also matches all retained
original outputs and passes four-file repair/identity/retention checks.

New EURnative1200-frame pairs pass every full-state/image/OAM/dispatch/audio
comparison on Linux/Wine with one160Bdecoded palette and53BGreads. Actual
production CLI checks add e5refusal/retention to extract/reuse/verify/repair/
move; exact source views and banks match across targets. Canonical executables,
ROMs, native libraries and old datasets remain hash-identical. New actual
interpreter/native1200-frame pairs show positive palette use and match legacy
frame/final-state hashes with no dataset rewrite. No benchmark speedup or
broader scene/GUI/archive/native-Windows claim follows from these checks.

Evidence:target/rom-data-bg-palette-runtime/verification-summary.json, core and
frontend logs, four producer reports, regional four-file datasets, CLI/runtime
pairs and protected hashes. Alternate palette programs, other catalogs and
languages, remaining05coverage and the rest of the resource matrix stay open.

## Alternate background palette catalogs

ROMDATA-01-BG-ALTERNATE-PALETTE-PROGRAMS classifies the valid alternate streams
linked by descriptor byte56. Independent original literals select USA427a04/
42cc80and EUR489bb0/48ee2c. Environment low bits2select the first catalog;
low bits1select the second. Each has157little-endian relativeu16links keyed
0..156(314Bplus2Balignment before the first stream). The shared original
consumer is USA0800543c/EUR080054a0, with destinationr0and sourcer1.
There is no evidence for game-level names such as day/night.

All157keys occur once in the162descriptors; records29/31/144/145/146instead
containFF. The original producer does not locally guard FF and indexing it
reads beyond the verified table, yielding nonsensical size headers. Preserve
that unavailable marker; do not dereference or invent fallback content. The
callers' exclusion/precondition for those five records is still unknown.

The valid314streams per region each decode256bytes (128fullu16words/eight
16word banks), with39933consumed source bytes and80384decoded bytes total.
Their exact source/output hashes and both relative-link tables match across
regions. No source ranges overlap within a catalog; consumed lengths exclude
padding. This is a distinct custom LZ/RLE format, not BIOS LZ77. A BEu32output
length precedes tokens. Token classes are literal data, short/medium/long
back-references and short/long zero runs; an FFfill branch exists in code but
is absent from these valid streams. Retain allu16bits and source semantics.
The initial private decoder established the inventory hypothesis; the typed
production decoder and malformed-input contract below now validate that syntax.

Four original interpreter cases (EUR/USAonLinux/Wine) each pass1256calls /
2655156guest instructions:157valid records×four environment bytes1/2/FD/FE×
direct/current-ID arguments. Each call enters the independently verified
consumer with the expected source/destination, produces the complete expected
256Bwith surrounding guards, leaves EWRAM unchanged and restores SP. The five
FFrecords are deliberately not invoked. These are functional output/instruction
checks, with no performance measurement or native/runtime replacement claim.

Evidence:target/rom-data-bg-alternate-palettes/verification-summary.json,
regional producer/consumer disassembly, exact literals/calls, source and decoded
inventories/private buffers, bounded decoder hypothesis, warning-free test
builds and four original oracle reports. Only an ignored cfg(test) module and
registration are added. No active dataset revision, production decoder, native
ABI, save schema, runtime/archive or canonical executable changes in this unit.
Typed decoding/managed extraction/consumption, caller preconditions for FF,
other catalogs/languages and remaining GUI checks stay open.

## Alternate palette callers and Color Mode

ROMDATA-01-BG-ALTERNATE-CALLER-CONTRACT closes the bounded direct-caller
inventory and the EUR mode meaning; the global exclusion of unavailable FF
records remains open. A whole-image aligned Thumb BL scan finds four direct
calls per region: USA0801a8b6/0801aeea/080e0ad8/0811b616 and
EUR0801ba26/0801c05a/080e45e8/0811f126. Their original instruction slices
cover staged graphics loading, palette refresh and alternate dynamic-palette
branches. No direct ARM BL or producer-address literal candidate was found;
this is not proof that indirect callers cannot exist. None of the inspected
local paths checks descriptor byte56 against FF. The staged loader's separate
current-background ID00ff check does not establish that descriptor precondition.

The setter USA08135cfc/EUR08139974 preserves the upper six bits of
EWRAM02002fc2 and replaces the low two. Its one direct BL caller per region
is USA08136c7c/EUR0813a96c in the original options confirmation path. Candidate
selection alone does not commit the setting. In an isolated EUR native GUI
session loaded from the existing worldmap state, original Options > Color Mode
cycles LCD A, LCD B and TV; B opens Save changes, then Yes applies it. Snapshots
read without advancing guest frames establish LCD A=0, LCD B=1 and TV=2.
Thus LCD B selects the second verified catalog and TV the first. Low bits3
still take the original base branch, with no selectable label established.
These are display-hardware color settings, not an inferred day/night cycle;
the [official Nintendo manual](https://www.nintendo.com/eu/media/downloads/games_8/emanuals/game_boy_advance_8/Manual_GameBoyAdvance_FinalFantasyTacticsAdvance_EN_DE_FR_ES_IT.pdf)
describes the three intended hardware targets. Internal values come from the
local original UI and snapshots, not from the manual.

Original slot/ROM/cache/loader/canonical hashes remain unchanged; the GUI uses
private copies and closes0. Captures cover candidate cycling, confirmation,
committed LCD B/TV and Party. Its alternate counter is0: this closes mode
identification, not actual alternate-stream consumption or full GUI equivalence.
The supplied deployment dumps also use mode0. A later scene must explicitly
record positive alternate table/literal reads. No build, old producer oracle,
performance measurement or public package was repeated for this inventory.
Evidence:target/rom-data-bg-alternate-callers/verification-summary.json,
regional static candidates/disassembly, natural-mode-fields.json and isolated
GUI logs/captures. Five unavailable FFrecord preconditions, indirect reachability,
other languages and native Windows remain unverified.

## Typed alternate-palette codec

`gba_core::rom_data::bg_alternate_palettes` decodes the distinct BE-size custom
format without using the BIOS LZ77 parser. It checks every read and every
back-reference, copies overlapping history in output order, and returns exact
consumption without padding. A declared output must be nonzero, within the
caller's budget and at most256B. Complete tokens must fit; malformed final
runs are rejected rather than clipped or allowed to overwrite another asset.
A4096Bconsumption guard bounds nonproducing controls independently of output;
it is an input-safety limit, not a packing or disk-size target.

Tokens80..FFcopy3..18Bfrom an11-bit-distance representation;10..1Fcopy4..67B
with a14-bit distance;00copies5..260Bwith a16-bit distance. Distances are
stored minus1. Tokens40..7Fcarry1..64literal bytes,20..3Ffill2..33zero bytes,
01fills3..258FFbytes and02fills3..258zero bytes. The verified palette size
further limits which whole tokens can fit. Contrary to an unknown-token-error
assumption, original dispatch skips03..0Fwithout output. The production parser
preserves those controls and bounds them; a synthetic oracle verifies them.

`Palette` holds eight16-wordu16banks and exact consumed source bytes as
compatibility metadata. Validation redecodes those bytes and checks the full
banks, retaining bit15. `Palettes` holds two157-key catalogs, relativeu16links
and the162original descriptor keys. It rejects missing/duplicate/out-of-range
keys, overlapping extents and inconsistent banks. `get` masks environment low
bits, returns no alternate for0/3and never dereferencesFFor an invalid ID.
There are no invented day/night names or substitute palettes for unavailable IDs.

`Catalog::from_rom` authenticates the entire supported regional image, original
root literals and each314Brelative-link table. Every stream gets its exact
source range/hash; table padding is excluded. The bounded descriptor catalog
supplies the key map. This is a verified base catalog, not a generic search for
plausible compressed headers or support for arbitrary modified ROMs.

Validation on Linux and Windows GNU/Wine:

- Four synthetic core checks pass for all token classes, overlap, full words,
  exact consumption, every truncated prefix of the synthetic streams, malformed
  headers/budgets/history/overruns, source/model edits, selection and key bounds.
- All314typed outputs and consumed sources per region match the retained
  independently verified original-consumer buffers:39933sourceB/80384decodedB.
  EUR/USAandLinux/Wineproduce identical deterministic readable JSON; semantic
  roundtrip and source authentication pass. No old boot/oracle is rerun.
-70new synthetic streams per regional original consumer, with4source×4target
  alignments (1120calls/case), check all copy/fill/literal/control classes,
  original source cursor, return value, complete outputs, guards, unchanged
  EWRAM/source and restoredSP. Four regional/target cases pass.
- A separate bounded malformed probe shows the original writes2literal bytes
  despite a declared size1. The offline parser rejects this input. This is
  retained negative evidence, not permission to accept an overrun or change the
  original runtime consumer. Both regional cases pass on both targets.

At the codec boundary the typed files were standalone test outputs and the
active set remained `v1/e6-d5`. Managed integration is now implemented below;
actual alternate runtime consumption remains separate. No guest/HLE/native/save ABI, canonical executable, archive
or AndroidPort change, and no performance measurements. Evidence is under
ignored `target/rom-data-bg-alternate-codec/` (protected inputs, builds/tests,
source hashes, typed files and original synthetic/negative logs).

## Managed alternate-palette datasets

The managed set now publishes `bg-alternate-palettes.json` as its fifth fixed
file, with ID `graphics.bg-alternate-palettes` and format
`bg-alternate-palettes-v1-json`. Schema stays1, extractor/decoder advance to7/6
and storage selects `v1/e7-d6`, retaining existing e6and earlier namespaces.
The manifest includes both314Blink tables and all314consumed stream ranges,
plus a dependency on descriptor keys. The file contains fullu16banks, original
relative links/keyFFand exact consumed compatibility metadata, with standard
readable JSON. Input guards are256KiBmanifest/16KiBOBJ/256KiBdescriptors/
1MiBbase/4MiBalternate and5MiBtotal. These are parser safety guards, not targets
for reducing or packing the decoded data.

All five files are written/synced, parsed and source-authenticated before
publication/selection. Valid reuse writes nothing. A missing/corrupt alternate
file repairs into a fresh generation while retaining earlier generations;
cancellation, partial fifth-file writes and process death preserve a valid
selection. The reader uses fixed names, rejects symlinks, bounds every file
before parsing, and compares the complete typed model and original compatibility
representation against the selected exact image. A recomputed file hash cannot
authorize modified banks, source encoding, key maps, links or manifest paths.

Activation checks the complete set before replacing its existing owner. Every
file/ID contributes to dataset identity: whitespace-only semantic-equivalent
alternate JSON changes that identity while OBJ/BG/base file identities and
machine snapshots stay the same. Missing or forged data leave the current
machine/owner intact. The existing three gameplay consumers remain unchanged;
alternates are explicitly reported as `runtime=verified-only` in this unit.
Preparation/authentication is not evidence of alternate guest use.

Validation on Linux and Windows GNU/Wine:

-2core extraction and10file+11preparation defaults pass on Linux;2+9+10on
  Wine (Unix symlink checks explain the difference). Checks include the fifth
  file's parser/count/key/link/source guards, rehashed edits, total/per-file
  limits,4publishers/preparers, cancellation and three forced process deaths.
  Partial-write regression now covers both base and alternate palette files.
- A new regional private test on each target covers coherent bank/source edits,
  equal-bank re-encoding rejection, whitespace identity, failed activation,
  new extraction/reuse/corrupt+missing repair/move and byte-exact e6retention.
  Existing Load/Undo/backup and the USA/EURresource-owner restart tests also
  pass using the new complete set. No old producer or scene fixture is replayed.
- Both production CLIs build warning-free.23actual checks per target pass from
  unrelated working directories: regional extract/reuse/verify, e6refusal and
  preservation, first prepare/no-write reuse, alternate corruption/missing-file
  repair, invalid offline data refusal without overwrite and resource-root move.
  Regional output files match across targets; the prior three data files match
  e6exactly. Canonical runtime/source/old-dataset hashes remain protected.

Private evidence: ignored `target/rom-data-bg-alternate-managed/` test/build/CLI
logs, regional results, protected/source manifests and helpers. The first forged
probe re-encoded a short stream as a larger literal stream and correctly hit
the overlap guard before source authentication. The helper now uses a coherent
same-extent mutation; only that private criterion was repeated, retaining the
failed observation and already passed defaults. No production workaround.
Final post-build edits only format existing statements or adjust test helpers;
no functional production change follows the verified builds.

Actual alternate runtime file consumption, FFcaller preconditions, broader
catalog/language/scene/GUI/archive coverage and native Windows remain separate.
No performance measurements, space optimization, public archive, AndroidPort,
agent commit or push is part of this unit.

## Direct alternate-palette consumption

The `e7-d6` fields already contain the needed banks, links and source metadata;
no file-format, dataset revision, native ABI/key or snapshot revision changes
for this04unit. `RuntimeResources` retains an immutable alternate owner. The
shared verified codec also exposes each literal token's encoded/output offsets.
At activation these build a bounded byte map: original CPU/DMA/native reads of
literal payloads resolve fullu16bank bytes, link-table reads resolve typed
relative links, and headers/control bytes resolve exact compatibility metadata.
The literal bytes are not copied back from the original ROM or taken from the
retained encoded payload. No guest instruction, copy, write, cycle or decoder
call is bypassed; the original custom decompressor still runs normally.

The existing bus entry points route only fixed verified asset pages:
USA427..431andEUR489..493. Within those pages, only exact table/stream extents
are mapped; unrelated data and padding retain original ROM behavior. Widths,
mirrors and mixed mapped/unmapped reads use the existing bus APIs. All fields
and bindings authenticate once at activation; gameplay does no file I/O or
revalidation. A separately counted `decoded_literal_bytes` distinguishes real
bank reads from merely preparing data or reading control tokens. There is no
invented host dereference/fallback for unavailableFFkeys; original caller
preconditions remain an open contract.

Verification on Linux and Windows GNU/Wine:

- Final release suites:216core+104runtime pass on Linux,216+103onWine;
 7core/31runtime optional ignored per target, with selected private checks run
  separately. Builds are warning-free. The new core fixture poisons both raw
  ROM spans and retained encoded literal bytes after authentication: every
  output still matches typed banks for both regions, widths/mirrors/boundaries,
  native API, DMA and Load/owner/counter preservation. Thus identical metadata
  alone cannot make the provenance test pass.
- Four regional/target original-producer probes each pass1256calls covering
 157valid IDs, environments1/2/FD/FEand direct/current arguments. Descriptor
  metadata, both link tables and all314raw stream spans are poisoned. Original
  interpreter and translated execution match CPU/register/CPSR/clock/RAM at
  boundaries, complete outputs/guards, restoredSPand final full serialized
  machine states after restoring raw source bytes. Each records143980bus reads,
 2512typed link bytes and116020decoded literal bytes.
- The translated path includes11767existing interpreter shells among14262
  dispatches; it is not claimed to be entirely native. Native API callbacks
  independently count143037asset reads. A separate reused-component probe
  proves92literal bytes are read specifically by native API calls (104asset
  calls) for each region/target, with correct complete output.
- Separate regional lifecycle cases delete copied dataset files, then pass
  Slots Load/Undo, backup restore and quick Restart ROM. The same alternate
  owner and counters survive; current cartridge bytes stay correct. Loaded
  resources remain usable without reopening the absent files.
- Both actual production CLIs activate the existing e7dataset and reuse its
  unchanged files with the prior TinyCC cache. Interpreter/native first-frame
  legacy/decoded hashes agree and native dispatch is positive. This is CLI/
  cache compatibility evidence: the cold first frame uses0alternate bytes;
  positive alternate use comes from the stronger original/native probes above.

Evidence: ignored `target/rom-data-bg-alternate-runtime/` source preimages,
initial/final warning-free builds, core/full suites, four producer reports,
separate native-bank/lifecycle logs, candidate production/CLI results. Existing
codec/managed/e6artifacts and original ROM/canonical runtime hashes are protected.
Only newly added test expectations/coverage changed after initial builds; raw
bus methods use byte offsets, with CPU alignment handled by instruction helpers.
No successful producer matrix was repeated merely to add the separate bank/
lifecycle criterion; its existing compiled components were reused.

The initial04boundary leaves broader scene/GUI/FF/archive acceptance separate;
the declared EUR pub scene and GUI checks are closed below. No benchmark,
profile, startup/RSS campaign, space optimization, AndroidPort, commit or push. The full
ROM_DATA_TODO remains active.

## Decoder contract and checks

`gba_core::rom_data::lz77::decode` accepts a bounded input slice and explicit
output budget. It requires type 0x10 and a nonzero 24-bit decoded size, checks every
input read, bounds allocation before decoding and rejects references before
the output history. Overlapping matches and a final match clipped to the
declared size follow the existing BIOS HLE contract. The result contains exact
decoded bytes and consumed header/stream length, leaving trailing padding to
the caller. The BIOS HLE itself retains its existing malformed-reference
behavior; strict offline decoding refuses such data rather than treating it
as a supported asset.

Four synthetic tests pass on Linux and Wine: literal/overlapping matches with
exact consumption and guard bytes against both BIOS 0x11/0x12 variants, partial
final tokens/flag groups, all truncated prefixes/invalid history/type/size and
the maximum 4096-byte history distance. Both target test builds are warning-free.
Separate optional tests use the already captured traces and destination dumps
to check all eight regional source ranges against actual guest outputs, on
both targets, with a 2 MiB output budget. No boot replay is required for these
offline decoder checks; original ROM, traces and outputs remain unchanged.

Private evidence lives in ignored `target/rom-data-graphics/`: per-target build
logs/executable metadata, core decoder test logs, regional calls.toml and output
captures, lz77-verified.toml reports, cross-target checks and helper results.
An initial observer compile typo was corrected before tracing. A stale native
Cargo library tracked the previous isolated measurement source rather than
the edited checkout; its original fingerprint/logs are retained, and only the
affected library fingerprint was invalidated. Successful traces and earlier
measurements were reused, with no reference-source mutation.

## Typed background packets

`gba_core::rom_data::bg_packets` decodes the next stage after LZ77. The caller
supplies separate destination, cumulative written-byte and packet budgets.
Parsing checks every read, alignment, counts, dictionary references and budgets
before allocating each packet. Repeated writes retain their original order;
this is a write program, not a replacement for mutable RAM. Terminal offsets
and consumed stream lengths are distinct from padding and dictionary bytes.

The halfword format begins with a destination byte offset and a byte count,
both little-endian u16, followed by literal u16 values. A dictionary variant
uses a u8 count and u8 indices into a separately supplied halfword dictionary.
Offset zero is valid for the first packet. A zero *next* offset terminates the
stream. Odd byte counts and misaligned/out-of-budget writes are refused.
The captured 456-byte body has 14 packets/196 values, consuming 450 bytes before
six padding bytes. Original USA 0801f098/EUR 08020208 apply it at EWRAM 7cb0.
The 1996-byte body has 112 packets/771 values, consuming 1992 bytes before four
padding bytes. Original USA 0801efb0/EUR 08020120 apply it at EWRAM d1a0,
through the original staged-pointer mechanism. These destinations are proven;
the small plane's complete gameplay meaning remains unclassified.

The paired-plane body has a u32 header. Bit 31 selects a dictionary at the
low-28-bit relative offset; bit 30 selects u16 indices instead of u8. Each
indexed packet has a u16 destination offset, u8 count and its indices. A clear
bit 15 in a dictionary halfword implies the pair `(value, value+1)`; a set bit
selects an explicit second halfword with both marker bits cleared. This yields
ordered u32 pairs. The captured 2960-byte body has 107 packets/770 pairs,
consumes 1867 stream bytes and references the dictionary at 1872. Its flags
select u16 indices; it writes into the combined calculated planes at EWRAM
91a0..d1a0. Original USA 0801ed24/EUR 0801fe94 give the same writes with unlimited
processing and with a 2048-byte budget requiring two original invocations.

Literal u32 packet syntax is retained, but its destination semantics have a
separate guard: `calculated_plane_writes()` accepts only indexed packets.
The first synthetic literal application failed the predicted-plane assertion.
A subsequent recorded-write probe independently confirms that the original
isolated literal branch attempts `(address0, value)` and leaves the calculated
planes unchanged. This is retained as a negative result on both regions and
targets. It does not justify changing guest code or applying those words at an
assumed plane address. A real producer context is needed before enabling that
destination interpretation.

Three synthetic core tests pass on Linux and Windows GNU/Wine, covering literal
and indexed payload syntax, both index widths, implicit/explicit pairs, overlap
order, padding/terminators, all truncated prefixes and invalid offsets, lengths,
dictionary references and budgets. The optional original-code test also passes
for both exact regions on both targets: seven original invocations and 87160
instructions per case, using the saved post-LZ77 bodies plus original synthetic
fixtures. It compares all EWRAM at component completion, checks original chunk
continuation and narrow indices, and records the literal branch's region-0
write. The negative probe observes CPU/bus instructions without advancing the
hardware clock; it is not a timing oracle. Other cases use normal Machine steps.
No complete CPU/IRQ/audio/scene equivalence or runtime migration is inferred.

Evidence: ignored `target/rom-data-bg-packets/` build metadata/logs, core test
logs, per-target regional original-packet logs, protected fixture/canonical
hashes and verification summary. The failed pre-guard probe is retained under
`before-literal-destination-evidence/`; its giant assertion output was replaced
with a bounded first-differences report. Exact original callers, packet/setup/
writer disassembly and regional root literal references remain private under
`target/rom-data-graphics/`. Neither original instructions nor resource bytes
are embedded in tracked sources; public fixtures use original synthetic data.

## Complete descriptor packet catalogs

ROMDATA-01-BG-PACKET-CATALOG extends the earlier observed three-body contract
to every field04/08/10 link in the authenticated162-record descriptor block.
This packet family does not decode field00graphics or the transfer/variant programs. Each
exact region has352distinct, nonoverlapping source ranges:122paired-plane,
117halfword-plane and113auxiliary programs. Total consumed source bytes398227,
post-wrapper bodies523794; aliases, source/body hashes and parent references
agree between EUR and USA. Equal catalogs do not prove language completeness.

All sources in this declared set have a four-byte wrapper. Low byte01means
raw packet body,11means an LZ77body at source+4. Upper24bits select a parent
*descriptor ID in the same family*, with FFFFFFmeaning initial fill. Parents
are decoded before the child's patches; all chains are acyclic with at most
two nodes. There are9paired,6halfword and9auxiliary inherited descriptor
references, distinct from deduplicated source counts. The paired initial fill
reads the live u16at EWRAM02007f5e and repeats it across the combined0x4000B
planes; halfword0x2000B and auxiliary0x200B initialize to zero. This dynamic
fill cannot be replaced by a constant in the extracted data.

Paired programs use indexed packet dictionaries exclusively in this catalog:
35unique sources have8-bit indices,87have16-bit indices. Wrapper counts are
91compressed/31raw. Halfword programs are116compressed/1raw; auxiliary are
110compressed/3raw. Both halfword families use literal u16packets. Keep packet
order and original inheritance, full words, offsets, dictionary/control bytes
and exact source extents. A rendered or flattened image discards these semantics.
For raw indexed sources, packet termination alone does not locate the end:
`PlanePatches::referenced_extent` now also includes every actually read
halfword/explicit-pair dictionary entry. `payload.consumed` retains its prior
stream-only meaning. Trailing padding is excluded; compressed body padding
within the LZ77declared output stays part of the body.

Original loader entries are USA0801f1a8/0801f2ec/0801f438 and
EUR08020318/0802045c/080205a8, in paired/halfword/auxiliary order. Bounded
original regional disassembly independently establishes tag handling, parent
field selection, fill and body dispatch. The normal paired wrapper's setup
sets processing flagbit2, so these complete wrapper calls leave no pending
continuation. The earlier direct chunk-writer oracle used flags without that
bit; its staged contract remains valid and is not rerun here.

Three affected core tests pass on Linux and Windows GNU/Wine, including exact
referenced extents, both index widths, implicit/explicit pairs, truncation,
malformed bounds and trailing padding. Four new original-loader cases pass,
one per region/target, each486wrapper calls/8081987original steps. For every
162descriptor×3families they compare complete target buffers,16Bprefix/suffix
guards and restored SP against ordered offline programs. Paired tests use a
nonzero live fill word; inherited writes are composed before the child. This
is a component invocation with existing BIOS HLE, not a real-BIOS or full
CPU/clock/audio/native/scene comparison.

An independent private classifier matches the core decoder's every source
extent/hash and body/hash. All352body files per regional/target case are
hash-checked; source/alias/parent data agree across targets. Test builds are
warning-free, ROM/canonical hashes preserved, all processes terminal/pass.
No new boot/scene/old producer oracle, production rebuild, performance/space
campaign, managed revision, archive, AndroidPort change, agent commit or push.
Evidence:target/rom-data-bg-packet-catalog/verification-summary.json,
headers/classified/range-comparison/cross-target-comparison JSON, bounded
regional disassembly, four original-loader result/body directories and logs.
Typed source-authenticated programs and managed publication are now complete
for EUR as documented below; actual packet runtime consumption remains04work.
Source files alone will not complete that migration.

## Typed descriptor packet programs

ROMDATA-02-BG-PACKET-DATA implements the bounded offline model in
`gba_core::rom_data::bg_packet_data`. Current user steering prioritizes EUR;
new USA validation remains pending. The prior original-loader USA evidence is
retained as history and is not rerun or extended by this unit.

`Programs` has paired, halfword and auxiliary families. Each family retains
all descriptor-to-program links and a unique program list in first-descriptor
occurrence order. Program IDs are stable semantic indices; root-relative
sources remain provenance. A program carries its optional parent descriptor,
exact consumed wrapper/encoded source, exact post-wrapper body, and typed
ordered writes. Paired writes preserve header, dictionary offset, packet-stream
consumption and referenced dictionary extent. Literal u16and decoded u32pairs
retain original values/order; encoded body metadata preserves index/marker and
control bytes separately. LZ77declared body padding stays present; source
alignment padding does not. No recompression or flattened image is used.

Parentless programs still mean the original role's initialization: the paired
live blank word, zero for the other two families. `ordered_programs` returns
ancestors before the child, leaving that live initialization separate. Validation
checks count/ID bounds, canonical alias IDs, unreferenced programs, cycles,
family agreement, source overlaps/overflow/descriptor-region overlap and every
encoding/body/parent/write relationship. Per-source and post-wrapper bodies
are bounded at64KiB, ordered writes at64KiB and packets at4096. These are
malformed-input guards, not storage targets. The generic family bound is512
records; the authenticated base factory supplies the verified162record block.
Unsupported wrapper tags and the unverified literal paired destination remain
refused. Serde rejects unknown fields and write kinds.

`Catalog::from_rom` authenticates the existing descriptor profile and supplies
352unique provenance ranges. `Programs::verify_source` also compares the entire
typed model with that exact source, so structurally coherent edited data cannot
be accepted as the immutable base merely by recomputing a file hash. At this closed02boundary, schema/extractor/decoder stayed1/7/6and no sixth
managed file or runtime consumer was added. The managed e8integration below
now publishes it; actual packet reads remain04work.

Linux and Windows GNU/Wine builds are warning-free. On each target,8affected
core packet checks(5new plus3existing), one default JSON roundtrip/unknown-field/
value guard and one private EUR retained-capture case pass. Every352source
extent/hash/body/parent and486descriptor link matches the retained original
EUR catalog, without rerunning the486-call oracle or any boot/scene. The typed
set contains185272decoded values,398227source bytes and523794body bytes.
Standard readable JSON is20217427bytes, byte-identical across targets with
SHA25679e557b3e02b0f61a6684eb2cddf7d946928f6aeb4da5225fbdc24595947a5dc.
A coherent literal edit passes structural validation but fails source
verification; an edited ROM is also rejected. Original ROM/canonical hashes
remain unchanged and all processes are terminal/pass. The user subsequently
limits new checks to reduced Wine-only EUR testing; later Linux validation is
pending, without retracting the results completed above.

Evidence:target/rom-data-bg-packet-data/verification-summary.json, native/
Windows build/test logs and {native,windows}-europe/bg-packet-programs.json.
Generated resources remain private/ignored. No old oracle/scene, performance/
space campaign, production rebuild, archive, AndroidPort, agent commit or push.
Managed extract/verify/prepare, actual runtime consumption and subsequent
functional acceptance remain separate work. New USA/native Windows checks,
field00graphics, transfer/variant programs and other families remain open.

## Managed packet program datasets

ROMDATA-02-BG-PACKET-MANAGED publishes `bg-packet-programs.json` as the sixth
fixed file, resource `graphics.bg-packet-programs`, format
`bg-packet-programs-v1-json`. Schema stays1, extractor/decoder advance to8/7,
using `v1/e8-d7` and retaining e7and all earlier generations. The new manifest
entry pins352source ranges/hashes and a BG-descriptor dependency. The packet
file exactly matches the typed02JSON; the other four decoded resource files
match e7byte-for-byte. The EUR six-file set is22405770bytes, including a
137179Bmanifest and20217427Bpacket JSON. Standard readable output is retained.

The fixed reader applies64MiBpacket and complete-set malformed-input guards;
existing manifest/OBJ/BG/base/alternate caps remain256KiB/16KiB/256KiB/1MiB/4MiB.
It parses directly into the typed program model, validates graph/encoding/body/
ordered writes, then checks the whole manifest/files against the selected exact
ROM. Rehashed coherent base edits and manifest path edits remain invalid.
All five resource IDs/file hashes contribute to dataset identity. Missing or
invalid new activation leaves the current owner and guest snapshot untouched;
a whitespace-only source-equal file changes dataset identity without changing
individual prior-family identities or guest state. No mod semantics are added.

All six files are synced, read back and authenticated before publication and
selection. Valid reuse performs no writes; corrupt/missing packet data publish
a fresh generation while retaining the old broken one. Failed partial packet
writes keep the previous selected generation. Relative selections survive a
root move; old e7selection/files remain untouched. Startup retains its existing
pre-save/pre-native ordering. This unit authenticates the packet data but does
not bind its reads yet: `ROM_DATA runtime=verified-only resource=graphics.bg-packet-programs`
makes that boundary explicit. Original packet reads, writes, cycles and dynamic
initialization remain unchanged. Native/save/cache ABI stays compatible.

Validation follows the updated reduced EUR/Wine-only policy:

- Four focused Wine checks pass: base-file authentication, synthetic six-file
  publication/reuse/move, packet partial-write/missing repair, and one actual
  EUR retained-data case covering e7retention, strict source edits, file/total
  guards, complete identity and transactional activation.
- The warning-free production Windows executable passes seven actual commands
  from unrelated cwd: extract, verify, prepare, no-write reuse, corruption repair,
  moved-root reuse and one cold native activation frame with the old TinyCC cache.
  This frame proves activation/cache compatibility only, not packet source use.
- Windows test/production builds are warning-free; canonical executable and
  protected ROM/e7/typed02/cache/loader hashes are unchanged. All processes
  are terminal/pass. Existing helper copy lists/counts/revisions were updated
  for six files without rerunning their broad optional historical matrices.

Evidence:target/rom-data-bg-packet-managed/verification-summary.json,
validated-test-sources.json, windows-results.json/windows-europe/result.json,
windows-cli-summary.json and seven command logs, candidates/build metadata and
protected hashes. Linux/USA/new GUI/archive/native Windows remain pending.
Actual packet consumption is the next04unit. No Linux/USA execution, broad
suite, old codec/original/scene replay, performance/space campaign, AndroidPort,
agent commit or push was performed. Prior02was observed committed externally
as ec5f9cb; its work is preserved.

## Direct European packet consumption

ROMDATA-04-BG-PACKET-CONSUMPTION now wires the previously retained packet
source-layout tracing/model into the authenticated runtime owner. All352typed
programs and486links remain immutable for the session. For each source, the
shared bounded packet parser identifies value/dictionary positions; typed
ordered u16/u32writes reconstruct those positions, preserving original marker
bits, indices, packet ordering, wrapper/parent metadata and padding. The shared
LZ77parser maps original literal positions to this projected body. Construction
requires every projected body to equal its authenticated source representation.
No raw ROM byte supplies a migrated packet value after activation.

CPU/DMA/native API and guest-BIOS bus reads serve the exact source view: raw
values and LZ literals use typed words, controls/matches retain file metadata.
EUR fixed source pages cover76c..895; no catalog search reaches unrelated ROM
pages. Palette and packet spans interleave: a crossing read overlays packet
bytes on the prior palette result, retaining both file-backed families and
unmapped neighbor bytes. The existing LZ77HLE obtains an Arc-owned typed body
and calls its usual ordered destination writer. Dynamic paired fill at02007f5e,
descriptor inheritance, packet processing flags and continuation stay in the
original guest code. RAM/unregistered sources follow their original path.

No instruction or cycle is skipped, no filesystem access enters gameplay, and
resources are not serialized. File loss after activation does not invalidate
the owner or an outstanding body read; failed reactivation is transactional.
Dataset schema/extractor/decoder remain1/8/7, with no changes to native/cache/
profile/save identity or ordinary startup. EUR reports `runtime=decoded` and
usage separates source reads, typed bus values, HLE calls/body bytes and typed
HLE bytes. USA remains authenticated and explicitly verified-only until its
consumer validation; no regional offset extrapolation was enabled.

Validation uses reduced Wine-only tests as requested:

- One synthetic core case covers typed pairs/marker words and halfwords,
  raw/LZsources, last EUR page895, widths/mirrors/edges including adjacent
  palette/packet bytes, real-BIOS bus mode, both LZ77SWIs with EWRAM/VRAM writes,
  DMA, complete-state/clock parity and Arc-owned reads after detachment.
- Nine selected EUR original-loader calls cover all three roles, raw/LZwrappers,
  parented programs, both paired index widths and two live fill values. A bounded
  trace compiles only the actually visited original instructions. All352packet
  source spans are poisoned in the decoded interpreter/native machines; complete
  states/clock, destinations/guards/SP and completed paired cursors match legacy.
  176838original instructions and884native dispatches produce18typed bytes
  through compiled loads and10506typed HLE bytes across11calls. This is a focused
  new consumer check, not a rerun of the old486call catalog oracle.
- One temporary-copy lifecycle test makes copied files unavailable by renaming
  only its temporary directory; the original dataset is read-only. Failed
  reactivation, slots/Undo, backup clean boot and restart retain identity/owner/
  counters. An outstanding body read also survives detachment.
- ROMDATA-05-BG-PACKET-SCENE passes one397frame EUR LCD B Cyril-pub BG62transition
  using the retained slot5/input/native cache/e8dataset. Every full state, image,
  OAM, dispatch and mixed/PSG/FIFO/HLE sample matches. It consumes4720body bytes/
  2640typed bytes in3HLEcalls and12wrapper reads. Its raw typed bus count is0;
  the compiled component probe establishes that separate source path.

Two actual Windows CLI calls from unrelated cwd also match the legacy/decoded
397frame image hash and dispatch counts, using the old native cache. The
decoded run reuses the prepared e8generation without writes and reports positive
packet HLE consumption. No new extraction or old preparation matrix is run.

Windows test/production builds are warning-free. All original dataset/ROM/
slot/input/cache/loader and canonical executable hashes remain protected.
Evidence:target/rom-data-bg-packet-consumption/windows-results.json,
windows-europe/result.json, windows-scene-results.json, windows-cli-summary.json, build logs/candidates
and protected hashes. New Linux/USA and other scene/GUI/archive/native-Windows
checks remain pending. Actual BIOS execution is distinct from the bus-mode
check. There was no benchmark, profile, startup/RSS measurement, space tuning,
re-extraction, full default suite, agent commit/push or AndroidPort change.
Field00graphics, transfer/variant programs, other catalogs/languages and the
remaining text/audio/gameplay/mod phases stay open.

## Reproduction and remaining work

Build the `recomp` test executable using the existing Windows/native build
setup. With `FFTA_STATE_TEST_ROM` selecting an exact supported image,
`FFTA_WIDESCREEN=0`, `FFTA_STATE_TEST_FRAMES=1200` and an absent
`FFTA_STATE_TEST_TRACE_ROOT`, run the ignored test
`rom_data::compression_tests::observe_boot_graphics_decompression` with
`--exact --ignored --test-threads=1 --nocapture`. It requires an explicit
1..12000 frame count and refuses an existing output directory.
Then use that existing trace root and ROM with
`rom_data::compression_tests::decode_observed_lz77_matches_guest_outputs`.
The latter refuses to overwrite its completed verification report.
The four self-contained core tests use filter `rom_data::lz77::tests`.
The three packet tests use `rom_data::bg_packets::tests`. To compare the original
packet writers, select the same private ROM and existing trace root, then run
`rom_data::bg_packet_tests::decoded_packets_match_original_regional_writers`
with `--exact --ignored --test-threads=1 --nocapture`. No boot replay is needed.
Descriptor core tests use `rom_data::bg_descriptors::tests`. The optional
`rom_data::bg_descriptor_tests::descriptor_catalog_matches_source_and_original_selectors`
uses the exact private `FFTA_STATE_TEST_ROM` and an absent
`FFTA_STATE_TEST_CATALOG_ROOT` for its local TOML/report output, with the same
ignored-test flags. It also needs no boot replay.

Authenticated descriptor source reads now consume the managed file-derived
session projection. Current scene/cost validation remains separate from
establishing linked asset/program
formats, other catalog bounds, sharing and language coverage for the remaining
graphics migration.
No BIOS call, guest instruction, cycle, DMA/IRQ, audio path, save/native
identity, current dataset schema/revision or default startup behavior changes.
This work gives no decompression speedup or native Windows result. Windows
hardware is unavailable to the user; Linux/Wine remain distinct evidence.

## Actual alternate-palette scenes

ROMDATA-05-BG-ALTERNATE-SCENES and ROMDATA-05-BG-ALTERNATE-GUI close the
previously missing actual source-use requirement for the declared EUR Cyril
pub transition. Original Options-confirmed LCD B/TV worldmap state copies and
normal A-button input load BG62. Four397-frame Linux/Wine native legacy/decoded
pairs match every full state/image/OAM/dispatch/audio sample, with first actual
alternate reads at relative102. LCD B consumes165decoded literal bytes and TV
153, plus2typed link bytes each. Both interactive native GUIs then present the
pub in both modes and close0with aggregate351reads/4linkB/318decoded literalB;
pub captures are inspected. No forced mode/PC or scene0read is used as source
proof. [Protocol, exact checks and remaining scope](development/ROM_DATA_VALIDATION.md#alternate-palette-e7-scene-and-gui-acceptance).

The ignored per-frame diagnostic now accepts
`FFTA_STATE_TEST_ALTERNATE_MODE=1|2`, checks the loaded/current original mode
on every frame and rejects missing table/literal consumption. Original data,
states, caches, loader and canonical binaries remain hash-protected. No
production rebuild, old base/producer replay, performance measurement, public
archive or default-enablement claim follows. Five global FFpreconditions,
other regional/gameplay/language/catalog coverage and native Windows remain.

## European main background graphics

`bg_graphics::Catalog` now covers the162field00links in the authenticated EUR
prefix:50disjoint sources,23tag20and27tag22. IDs follow first descriptor
occurrence; the complete link map retains aliases. The catalog consumes
621923source bytes and yields835232decoded bytes, excluding alignment/next assets.
The normal0801b2cccopy path alone did not establish this contract: staged
producer0801b688phase1selects original080054a0, the same custom BE-size LZ/RLE
routine used for alternate palettes. It is not BIOS Huffman.

Tag20's upper24bits declare output bytes and are followed by the custom stream.
Tag22's upper24bits specify a destination byte offset, followed by a nested20
header and that same stream. The inner BE length must equal the20length.
Every declared output and destination offset is a multiple of32and the complete
extent fits0x4fe0bytes, the original0x13f8-word graphics target. Typed data retain
full little-endian words, destination offset and exact consumed encoded bytes.
No palette/RGBA conversion, recompression, flattened prefix or live RAM content
is substituted. Bounds on compressed input/output protect malformed parsing;
they are not extracted-data size budgets.

The original phase1writes the compact decoded buffer atEWRAM020091a0. Phases2/3
queue DMA32 transfers of its first/second halves to the selected graphics target
plus the byte offset. Each DMA has length output_bytes/8words (half the buffer).
A22resource therefore preserves the existing destination prefix; it does not
contain that prefix. Transfer/variant programs and other catalogs remain separate.

The shared `custom_lz` parser keeps the previously verified token semantics,
exact consumption and literal-span mapping. The alternate-palette wrapper still
limits outputs to256bytes and input to4096bytes. New graphics decoding rejects
truncation, mismatched sizes/tags, misaligned/oversized output/offsets, references
before history, token overruns and excessive nonproducing controls. Typed models
also reject overlap, unused/noncanonical IDs and encoded/decoded disagreement;
`verify_source` rejects coherent base edits against the exact EUR image. USA is
explicitly unsupported for this new catalog until independently verified.

Reduced validation is Windows GNU/Wine only:3graphics core checks,4affected
alternate-palette checks,1JSON guard/roundtrip and1private catalog oracle pass;
the release test build is warning-free. All50original staged outputs match the
complete offline buffers, exact live source cursors, suffix/prefix guards and
restoredSP. Two representative20/22cases also verify the phase2/3DMA queue source,
destination and control words:54calls/11142126original instructions in total.
The readable JSON roundtrip is deterministic (SHA256
`8dd0bb6f09bd293b3c5ff68ea3990d5c4b09f978ccaada5346980efd090fc00c`). Existing
ROM/e8dataset/cache/loader/canonical executables are hash-preserved.

Evidence stays ignored under `target/rom-data-bg-main-graphics/`: bounded
original disassemblies, Windows build and four logs, windows-results.json,
windows-europe/result.json, typed JSON and captured output buffers. At this
offline boundary no active manifest revision, startup publication or consumer
was added. Subsequent managed publication and consumption are described below;
packaging stays open.
New Linux/USA checks remain pending. No benchmark/profile/startup/RSS campaign.

## Managed European main graphics

The seventh EUR fixed file is now `bg-main-graphics.json`, coverage ID
`graphics.bg-main-graphics`, format `bg-main-graphics-v1-json`, schema1/
extractor9/decoder8. It is byte-identical to the completed offline graphics
model above. The manifest retains all50exact source ranges and the descriptor
dependency. USA explicitly preserves its existing six-file1/8/7contract;
its new graphics catalog and validation remain pending.

Offline extract/verify, managed prepare and opt-in startup use the same fixed
filename, strict typed JSON and complete source validation. The graphics file
has a32MiBmalformed-input guard; the existing64MiBwhole-set guard also applies.
These are parser protections, with no extracted-data size tuning. Every file
must validate before atomic publication or runtime owner replacement. Every
resource/file hash enters dataset identity; coherent edited base words fail
source authentication even with a recomputed manifest hash. File absence,
truncation, oversized input, removed coverage, forged provenance and path edits
fail before changing the live machine. At this managed boundary main graphics
were logged as `runtime=verified-only`; direct consumption is now described below.

Four reduced Windows/Wine tests pass: core base authentication, synthetic
seventh-file deterministic publication/reuse, partial graphics write/missing-file
repair preserving the prior generation, and an actual EUR full-set case. The
last covers e8retention, create/no-write reuse, missing/corrupt repair, relocation,
identity, coherent edits, fixed paths, file guards and failed activation with
unchanged state/Arc. All five prior decoded files stay byte-identical.

Seven actual Windows CLI processes from unrelated cwd pass extract, verify,
prepare, explicit no-write reuse, corruption repair, moved-root reuse and
one-frame activation with the existing TinyCC cache (290native blocks). This
frame reads0resource bytes and is only activation/cache compatibility evidence.
The CLI parser initially expected `native=` instead of the actual
`native_blocks:` field; it was corrected from the completed log without
rerunning a process. Windows test and production builds are warning-free;
canonical executable, original ROM/e8files/cache/loader hashes are preserved.
Evidence: ignored `target/rom-data-bg-graphics-managed/`, windows-results.json,
windows-europe/result.json, windows-cli-summary.json, logs/candidate and hashes.
New Linux/USA checks, direct graphics consumption/native provenance, owner
lifecycle, scenes/GUI/archives remain open. No benchmark/profile/RSS or space work.

## Direct European main graphics consumption

The managed e9-d8main-graphics model is now retained in the immutable runtime
owner and reports `runtime=decoded`. During authentication, the shared custom
codec supplies exact literal source/output positions; each literal byte is then
read from the corresponding little-endian typed word. Header/offset/token bytes
come from exact file metadata. CPU, DMA and native RtApi paths share these views
on fixed EUR769..895pages, including earlier graphics pages absent from the
packet/palette dispatch. Mapped graphics bytes overlay prior family results so
crossing reads preserve adjacent descriptor/palette/packet bytes. There is no
new per-read allocation/I/O, skipped decompression instruction, scratch write,
DMA or guest cycle. Schema1/9/8and native/save/ROM identities are unchanged.

`GraphicsStats` reports independent `bus_reads`, `source_bytes` and
`decoded_literal_bytes`; only the last proves typed word use. Retained encoded
literal payloads are not the runtime data source. Three focused Wine tests pass:

- Synthetic both-wrapper reads cover first769/76a/76bpages and895,8/16/32-bit
  widths/mirrors, descriptor and palette edges, real-BIOS bus mode, native API,
  CpuSet/DMA and owner/load preservation. Raw ROM spans and retained encoded
  literal metadata are both poisoned, so returning duplicate metadata fails.
- Four original staged calls (IDs0/19/38/161) cover both tags, maximum output
  and last source. All50old source spans are poisoned after authentication.
  Original/interpreter/native complete states and clocks, destinations/guards/SP
  match.788393original instructions/3414native dispatches consume29023typed
  bytes specifically through compiled loads (29226decoded literalB overall).
  This is a new reduced consumer proof; the closed50-source oracle is not rerun.
- Temporary copied files become unavailable, while failed reactivation,
  Slots Load/Undo, backup clean boot and Restart ROM retain the same owner and
  counters. Source reads remain file-backed after those operations.

One397frame EUR LCD B transition into Cyril pub BG62reuses the existing slot5,
input/native cache/e9dataset. Every full state/clock, central/wide image, OAM,
dispatch and mixed/PSG/FIFO/HLE sample matches legacy. Main graphics records
14815reads/16209source bytes/10334decoded literal bytes. Other resource families
remain positive where expected. This closes only the declared scene criterion;
new Linux/USA, other scenes/languages, GUI/archives and native Windows remain.
Evidence:ignored `target/rom-data-bg-graphics-consumption/`, Windows build logs,
windows-results.json, windows-europe/result.json, windows-scene-results.json and
the retained scene log. No performance/profile/startup/RSS or space campaign.

Two actual Windows CLI processes from unrelated cwd also pass the legacy/decoded
397frame transition with identical final image hash/native/fallback counts.
Decoded startup reuses e9without writes, uses the old TinyCC cache, and reports
the same10334typed graphics literal bytes. The production candidate is retained
under ignored target/; the canonical executable is restored by hash. Actual CLI
and component/scene evidence cover different criteria; no new extraction or
old managed/original catalog campaign was repeated.

## European background transfer programs

`bg_transfers::Catalog` now decodes the declared EUR descriptor block's five
transfer slots:97active links (83base/6/5/3/0extra),42controls and34shared raw tile
banks. Independent control/bank IDs follow first descriptor/slot occurrence,
retaining every alias and VRAM destination. All162optional variant pointers
are null in this block. Nonzero variant programs remain unsupported; no decoder
or fallback is invented for absent data. Other catalogs/USA remain unverified.

Each control begins with u16transfer length in bytes, u8frame count and a retained
unclassified byte3, followed by ordered little-endian u16duration/source-tile
pairs. The original updater reads only the low count byte. Tile offsets are
multiplied by32; zero duration preserves the original u16countdown wrap behavior.
Declared base durations are6/8/10/12/16ticks and counts4/8/12. Exact raw tile bank
extents are the maximum offset+transfer size over all referencing controls;
shared banks and overlapping frame windows stay shared, with complete u32words
and high bits. There is no image flattening or palette conversion.

The76control/bank extents are disjoint from each other and current e9families,
with968control bytes and199296tile bytes. Bounded typed validation checks frame
counts, tile-sized transfers, VRAM bounds/alignment, canonical IDs, exact shared
extents, missing/unreferenced resources, source overflow/overlap and unsupported
variant links. Source reconstruction and strict JSON retain every byte, order
and alias; coherent changed tile words still fail exact EUR source authentication.
The readable JSON SHA256is
`051e0eee7412a24e1b0ce51a79736e4d8488f26593254e2cee907aa53eea71e8`.

Original creator08021f10allocates an active RAM object, initializes frameFFand
a1tick countdown, retains the control/destination/bank pointers, and reads the
manager's live transfer limit. Updater08021bd8advances/counts frames; dispatcher
08021d10copies type0data through DMA32, preserving partial progress until the
configured size is complete. Mutable frame, countdown, loop/reverse flags and
copy cursors remain live guest state. The offline model does not preadvance or
replace these operations.

Five reduced Windows GNU/Wine checks pass after a warning-free test build:
3core cases,1strict JSON case and1private EUR catalog/original-contract case.
All76source reconstructions/hashes,97link locations/aliases and162absent variants
match the exact ROM and independent retained static classifier. Four original
cases (descriptor/slot19/0,4/1,58/0,63/1) cover4/8/12frames, base/extra controls,
shared banks and a2432-byte transfer split by the original1024-byte limit.
Two full loops plus the boundary match every original countdown/frame and full
VRAM after each chunk, including guards and unchanged ticks:634calls,
80766instructions,34partial chunks. An existing slot5snapshot initializes the
original IWRAM helper code; no boot, recording or earlier graphics oracle is
repeated. It is not an actual-scene or native consumer proof for this family.

Evidence stays ignored under `target/rom-data-bg-transfer-variant/`: bounded
producer/update disassemblies, static-inventory.json/candidate-layout.json,
Windows build/three logs, windows-results.json and windows-europe/result.json/
bg-transfer-programs.json. Original ROM/slot/e8/e9/native cache/loader/canonical
hashes are preserved. This offline boundary added no manifest revision or
runtime projection; managed publication is described below. New Linux/USA checks, transfer native provenance and
lifecycle/scene/GUI/archive integration remain pending. No performance/space work.

## Managed European transfer programs

The eighth fixed EUR file is now `bg-transfer-programs.json`, ID
`graphics.bg-transfer-programs`, format `bg-transfer-programs-v1-json`, under
schema1/extractor10/decoder9. Its readable JSON is byte-identical to the offline
model above; the manifest includes all76control/tile source ranges and the
descriptor dependency. All six previous decoded files and old e9generations
remain intact. USA retains the existing six-file1/8/7contract; its new validation
is pending.

Extract/verify, prepare and opt-in startup read this fixed filename through
bounded strict JSON (32MiBinput guard, existing64MiBwhole-set guard). Complete
source/file authentication precedes publication and runtime owner replacement.
The new file's hash participates in dataset identity, including semantic-neutral
formatting changes, while coherent edited tile values fail base authentication.
Removed coverage/dependencies, forged paths/provenance, malformed/missing/oversize
files fail transactionally. The family is explicitly `runtime=verified-only`;
no transfer reader is redirected yet and mutable animation/DMA state is unchanged.

Four reduced Windows GNU/Wine tests pass after warning-free test and production
builds: core base authentication, synthetic eighth-file deterministic publication/
reuse, partial-write/missing repair, and actual EUR full-set authentication/
retention/identity/repair/relocation/failed activation. The actual case preserves
e9and all previous decoded bytes and verifies equality with the retained typed02
file. Three actual Windows CLI processes from unrelated cwd pass prepare,
no-write reuse and startup reuse/activation with the existing TinyCC cache
(290native blocks). The first frame has0new transfer use and is activation/cache
evidence only. The canonical executable is restored and protected ROM/slot/e8/
e9/cache/loader hashes remain unchanged.

Evidence:ignored `target/rom-data-bg-transfer-managed/`, Windows build and four
test logs, windows-results.json, windows-europe/result.json, windows-cli-summary.json,
source/protected hashes and candidate. New Linux/USA tests, direct typed source
consumption/native provenance, lifecycle and actual transfer-use scenes remain
pending. Earlier original-animation/graphics/scene campaigns were not rerun;
there is no performance/profile/startup/RSS or space optimization work.

## Direct European transfer consumption

The authenticated e10-d9transfer model is now retained in the immutable runtime
owner. Exact source views reconstruct controls directly from typed transfer
sizes/frame counts/header byte3/duration/tile-offset fields, and raw bank bytes
from little-endian u32words. Fixed EUR778..897pages include the last bank ending
897ad4. Mapped bytes overlay prior family results, preserving reads crossing
graphics/palette/control/tile boundaries. Source lookups allocate/read no files;
there is no cached raw-control substitute, skipped original instruction/DMA or
advance of mutable frame/countdown/loop flags/copy progress. Those remain live
RAM driven by the original creator, updater and dispatcher. Schema1/10/9and
ROM/save/native/cache identity are unchanged; USA's existing e8contract remains.

`TransferStats` separates bus/source/control/tile bytes. Three reduced Windows
GNU/Wine tests pass after a warning-free test build:

- Synthetic all-width/mirror/unaligned/boundary reads cover graphics and palette
  neighbors and the last897page, with migrated raw ROM spans poisoned. Native
  API, real-BIOS bus mode, CpuSet, DMA and Load/owner preservation match legacy.
- Two original animation cases (58/0forward,63/1reverse;17ticks each) use live
 512/1024-byte limits. Every76old control/tile source span is poisoned after
  activation. Original/interpreter/native complete states and clocks match after
  every60calls, including full VRAM/guards and original IWRAM helper execution.
  Original tracing covers7567instructions/234native dispatches. Compiled source
  loads consume66typed control bytes; DMA initiated by native writes consumes
 9984typed tile bytes. These DMA bytes are distinguished from direct compiled
  loads, and this new provenance check does not repeat the closed two-loop
  catalog oracle.
- Only temporary copied files become unavailable. Failed reactivation leaves
  state/Arc untouched; Slots Load/Undo, backup clean boot and Restart ROM retain
  the same owner and counters. Typed control/tile reads still work after those
  operations with the old raw ROM ranges poisoned.

The retained EUR LCD B397frame Cyril-pub62transition now passes every full state/
clock, image/OAM/dispatch and mixed/PSG/FIFO/HLE sample with positive transfer use:
6579reads/22707source bytes,1203controls and21504tiles. One new actual decoded
Windows CLI process from unrelated cwd reuses e10without writes and the existing
native cache, matching the retained valid legacy final image/dispatch row and
the same positive counters. The legacy process was not repeated: its protected
ROM/slot/input/cache/loader and legacy guest behavior/ABI are unchanged; the new
scene pair independently proves complete frame/state/audio parity.

The production candidate is warning-free and retained under ignored target/;
canonical runtime, original ROM/e8/e9/e10files/slot/input/native cache/loader
hashes are preserved. Evidence: `target/rom-data-bg-transfer-consumption/`,
windows-results.json, windows-europe/result.json, windows-scene-results.json,
windows-cli-summary.json, build/logs/candidate and source/protected hashes.
New Linux/USA, broader scenes/GUI/archives and native Windows remain pending.
No performance/profile/startup/RSS or space optimization campaign was run.


## Other raw UI palette sources

[EUR e20raw UI palettes](rom_data_ui_palettes.md) add a separate21-bank source
catalog for both BG/OBJ UI shadows, with2656raw source bytes,23direct/derived/
Color-Mode references and exact u16storage. They do not change the162-descriptor
base/alternate palette catalogs above. Original copied memcpy, live palette
transforms and publication remain; reduced Wine original/native/lifecycle and
one positive240-frame boot full-state/audio/actual CLI check pass. Other
indirect sources and new Linux/USA verification remain pending.


## Additional custom BG/OBJ UI graphics

[EUR e21custom UI graphics](rom_data_ui_graphics.md) add nine independently
rooted raw custom banks/eightBGsources/oneOBJsource,57038encoded B/84160decoded B.
Ten original decode/upload paths include count computation from decoder r0,
which the earlier constant-count OBJ filter omitted, and a544of2944byte partial
BG upload. Typed literals/metadata preserve all original decode/scratch/count/
copy instructions and clock. Reduced Wine source/native/lifecycle and a positive
320-frame ordinary Clan/actual CLI pair pass; all15earlier decoded files/e20
stay unchanged. Other callers/indirect sources and new Linux/USA remain open.


EUR e22extends the same UI resource to16banks/two language selectors and two
explicit shared OBJ links, keeping every old record/other decoded file intact.
Seven new banks and ten asset-pointer literals add17ranges/14436source bytes.
Original jump tables, language fallbacks, partial Clan and5056OBJ/2688BG split
copies stay active. See [localized UI contract](rom_data_ui_graphics.md#localized-ui-selectors-in-e22).
