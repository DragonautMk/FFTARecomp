# European map and terrain sources

`ROMDATA-01-TERRAIN-MAP-SOURCE` closes the source classification of the normal
map loader for the authenticated 162-descriptor EUR block. Its static sources
already belong to `graphics.bg-descriptors` and `graphics.bg-packet-programs`.
This adds no decoder, resource, revision or runtime shortcut. Other map paths
remain unclassified where stated below. Offsets are EUR file offsets; addresses
with an `08`, `02`, `03`, `05` or `06` prefix are guest addresses.

## Original loader and static ownership

The setter at `0801B270` narrows its first argument to u16. Zero selects the
`00FF` sentinel; other arguments become `(argument - 1) & FFFF`. It stores this
descriptor index at `02007F10`. It also obtains descriptor selector field54/55
through `0801ED08`, storing that result at context+12. Neither this setter nor
the inspected loader introduces a global 162-record guest limit. The declared
source catalog is bounded independently by the existing descriptor contract.

The staged loader at `0801B688`, state6 at `0801BAB4`, uses root `087662A8`
and stride `58`. Its normal branch at `0801BAF0` constructs these arguments
for `0801AE88`, called at `0801BB36`:

- r0: descriptor field10, auxiliary patches, added to the descriptor root.
- r1: field04, paired planes, added to the same root.
- r2: field08, halfword planes, added to the same root.
- r3 and the fifth argument: 32 and 32, used by the original scene setup.

`0801AE88` calls the original plane reset at `0801C838`, then wrappers
`080215F4`, `080215E8`, `080215DC`. These call the already classified
`080205A8`, `0802045C`, `08020318`, respectively. Their sources are the existing
auxiliary, halfword and paired packet programs. There is no separate discovered
terrain archive behind this normal branch.

The source audit checks every one of the 162×3 links against the original
descriptor words, authenticated JSON program IDs, original encoded sources and
manifest ranges. It verifies exclusive ownership against all 49 e54 resources:

- 122 paired programs, 236091 source bytes, 162 links and 40 alias links.
- 117 halfword programs, 143317 source bytes, 162 links and 45 alias links.
- 113 auxiliary programs, 18819 source bytes, 162 links and 49 alias links.

The 352 ranges total 398227 source bytes and 523794 post-wrapper body bytes.
The 14256 descriptor bytes reconstruct exactly. Existing raw/LZ77 wrappers,
ordered writes, dictionaries, inheritance and source extents remain those of
the [packet contract](rom_data_graphics.md#complete-descriptor-packet-catalogs).
There are 9/6/5 distinct parented programs but 9/6/9 parented descriptor links;
the auxiliary difference is sharing. Maximum chain length is two nodes.
All 162 optional field50 variant pointers are zero in this declared block.
Main graphics, base/alternate palettes and transfer programs retain their
existing separate owners; this classification does not change those resources.

## Live terrain and display state

At `0801AEDA`, the normal builder stores `02007CB0` at context+4 and sets
width and height at context+8/+9 to 16. The auxiliary program initializes a
512-byte grid of 256 little-endian cells. `0801DD68` computes a cell pointer as
`live_pointer + 2 * (u8(y) * live_width + u8(x))`. `0801DD88` reads its first
byte as height. `0801DDA8` can subtract one when the second byte has bit1 set
and the caller's low flag bit is clear, returning a signed 16-bit result.
`0801DE78` reads the second byte; `0801DE98` writes that byte at `0801DEB0`.
The complete game meanings of its flag bits remain unclassified.

Consequently, decoded static patches initialize terrain; they cannot replace
the mutable terrain grid or its current pointer/width. The sentinel loader
branch clears the 512-byte buffer and stores its pointer; it does not execute
the normal dimension-setting builder, so the normal 16×16 assignment must not
be generalized to every possible context state.

Paired programs initialize the two calculated 64×64 planes at
`020091A0..0200D1A0`, with the original live fill word at `02007F5E`.
Halfword programs use the next 8192 bytes at `0200D1A0`. Original producers
retain layout calculations, fills, patches, camera transformations and DMA.
`ffta_stage.rs` observes calculated planes and their VRAM crops. Movement
overlays at `02003CB0` and `02005CB0` are also live calculated data; the first
buffer is reused as packet decompression scratch. These EWRAM/VRAM buffers
are not additional static assets to export.

## Scope and remaining sources

`0801DFB4` uses the same live pointer and width with a ten-byte stride and
reads byte4. A static scan below offset370000 finds no direct Thumb BL or
aligned Thumb pointer literal to this accessor. This does not establish dead
code, a second static archive or a global two-byte layout. Its caller and
possible alternate context remain unclassified.

The selector path also exposes a distinct root `084872B0`, through the literal
at `0801EE24` and lookup `0801EDEC`. The separate source contract below now
bounds this map raster palette family. Its typed model and targeted runtime
migration now pass below; it is separate from the already migrated terrain seeds.

## Validation and limits

The new check is static only: exact EUR ROM identity, two authenticated dataset
files, descriptor reconstruction, 486 link/source formulas, all 352 source
hashes and exclusive owners, inheritance, plus 40 instruction halfwords,
11 literals and nine original BL edges. One hand-transcribed STRB fingerprint
initially used the wrong base-register encoding; correcting that private
expectation to the inspected `704A` makes the check pass. No product source or
existing decoder/oracle changed.

Private evidence is under `target/rom-data-terrain-maps/`: disassembly,
`verify_source_contract.py`, `source-contract.json` and
`descriptor-packet-links.json`. It contains no new guest execution or build.
Earlier packet source/original-loader, typed native/managed/lifecycle and
ordinary Wine results remain closed and are referenced in the graphics
contract; they were not replayed for this semantic classification. No new
end-to-end map-loader execution or full flag/caller semantics is claimed.
This classification left EUR v1/e54-d53 and USA e8-d7 unchanged. Later raster
publication is described below. New Linux, USA and native Windows checks remain
pending. No performance or space measurement was performed.

## Map raster palette source contract

`ROMDATA-01-MAP-RASTER-PALETTE-SOURCE` identifies an uncompressed EUR bank at
`4872B0..489BB0`, 10496 bytes with SHA-256
`6570159e474457326a39cc65bab5530c9467f74470d4c4a2a0f2cb2e19e13df3`.
Original functions `0801EAF0` and `0801EDEC` independently use this root;
the only aligned literal occurrences are `01EB2C` and `01EE24`.

The first relative u16 is `0040`, placing the first record after 32 little-endian
u16 directory entries. All 32 values point to distinct consecutive records;
all IDs0..31 also occur among the 324 field54/55 links of the 162 authenticated
BG descriptors. The final record's independently decoded length ends exactly
at the known alternate-palette root `489BB0`. These checks agree on the
declared 32-record projection. Neither original reader checks a global ID<32
condition, and the extraction bound must not become a new guest limit.

Each 326-byte record contains three u16 header words followed by 160 indexed
BGR555 color words. All current headers are `[160,160,0]`. Retain the complete
u16 words, including bit15, and the physical record identity even when contents
are equal. There are 32 distinct source records but 26 distinct color contents:
pairs0/16,1/17,3/19,5/21,7/23,15/31 match in content, not source address. There
is no compression, alignment gap or additional directory in this projection.
The 33 source ranges partition 64 directory bytes, 192 header bytes and 10240
color bytes without overlap with any of the current 49 managed resources.

At `0801EAF0`, the input index is narrowed to unsigned u16. An existing live
`00FF` selector at `02007F20+2` prevents the lookup. Otherwise it reads the
relative directory entry and adds six to obtain the color pointer. It stores
header0 at live+4; header4 at live+C; header2+header4-1 at live+E; the color
pointer at live+10; and `colors + 2*header0 - 320` at live+14. These are the
observed formulas, not a complete interpretation of camera/scroll metadata.
The update routine `0801EB80` uses live header0<=160 to choose the direct
color window; larger live values have a separate camera-dependent path.
Every canonical record follows the direct window path, but live mutations
and the larger path must keep their original behavior.

`0801EDEC` has a different raw-argument contract: it first checks the current
selector at `02007F10+12` against `00FF`, returns zero when disabled, and
substitutes that live selector only for raw argument `FFFF`. It then uses
signed16(argument)*2 to address the directory and returns `root + relative + 6`.
Other out-of-projection inputs retain their original raw memory behavior.
These two readers must not be conflated into a newly capped host lookup.

### Exact copy boundary and raster setup

Calls at `0801EB38`, `0801EC3C`, `0801EC80`, `0801EE58`, `0801EEF6` reach
`08145230`, whose original instructions are `SWI 0B; BX LR`. Control `00A1`
selects 161 halfword copies, 322 bytes, not 161 words. They copy to `02008390`
or defer the source pointer in live state. The original code then overwrites
destination halfword160 with halfword0. The extra source read is still part of
the original bus accesses and timing; it cannot be dropped as an optimization.

Each canonical source window therefore reads one halfword beyond its 160
colors. In records0..30 that word is the next record's header0 (`00A0`). The
last read is at `489BB0`, value `013C`, owned by the existing 314-byte
`graphics.bg-alternate-palettes` index span. It stays with that owner. The new
source projection ends at `489BB0`; it does not duplicate or steal these two
bytes. Future managed/native checks must cover a read spanning both owners.

`0801ECAC` copies the first RAM color to the palette shadow at `03003860` and
schedules channel2 with source `02008392`, destination `05000000` and input
control/count `20400001`. Original helper `08000D80` ORs `A2000000`, queuing
`A2400001`: one halfword, incrementing source, fixed destination, repeat and
HBlank timing. This is the source-level basis for calling the bank a raster
palette. It does not establish an executed DMA trace or a game-level sky name.
Guest scheduling, enable/disable, deferred copies, RAM edits and raster timing
remain original work; a static asset cannot replace these states.

### Inventory validation and remaining migration

The private static parser reconstructs every byte and verifies all 324 links,
33 range hashes, all 49 prior owners, the extra read ownership, 67 instruction
halfwords, 13 literals and ten BL edges. It passes on its first execution.
Header0-backed extents and directory/descriptor links provide the bound;
the next resource root is an independent cross-check. No build, decompression,
guest invocation or previous test replay was needed. Evidence under
`target/rom-data-map-raster-palettes/` includes `source-plan.json`,
`source-contract.json`, disassembly and `verify_source_contract.py`.

A strict lossless model and one reduced Wine original-reader/copy oracle now
pass below, followed by managed publication and targeted bus/native/lifecycle
consumption. Actual production CLI reuse and a positive ordinary Wine pair
now pass in the final section. Preserve the raw signed/unsigned reader
differences and the mixed owner boundary. The static inventory left EUR e54-d53
and USA e8-d7 unchanged; later publication advances EUR as described below.
New Linux, USA and native Windows checks remain pending. No performance or
space measurements were made.

## Typed raster palette model and original-reader oracle

`ROMDATA-02-MAP-RASTER-PALETTE-MODEL` adds the isolated
`rom_data::map_raster_palettes` module. `MapRasterPalettes` holds 32 physical
`RasterPalette` records and 162 pairs of descriptor program links. Each record
retains its source offset/ID, typed three-word `RasterHeader` and all 160 color
words. Index bytes derive from physical source offsets. There is no raw ROM
shadow, flattened image, compression or content-based merging of records.

Validation checks the declared contiguous index/record extents, ordered IDs,
header/color counts and descriptor targets. Actual BG descriptors are a checked
dependency of the derived links. Authentication reconstructs all 33 original
source spans and compares the exact EUR source, fields and hashes. Structural
acceptance of changed color/range words does not authorize modified datasets;
`verify_source` rejects them against the canonical ROM. The neighbor halfword
at `489BB0` remains outside `byte_at` and all new source spans.

The public arithmetic helpers describe the unsigned initializer and signed
pointer-getter index formulas, including the latter's live `00FF` gate and
`FFFF` substitution. They describe source addresses; they do not impose host
bounds on guest arguments. The only existing source edits register the module
and its new private test. That model stage left startup, managed files,
revision and runtime readers unchanged.

One affected Windows GNU frontend test build passes without diagnostics. Its
frozen executable SHA-256 is
`6434366e7fad4ada6ff0bf10bdfba86d5082671606cfd8358dd92aaed67315b0`.
One new exact Wine filter passes on its first execution, verifying:

- All independent source-plan fields, 33 spans/hashes, 5120 color words and
  324 descriptor links, exact JSON roundtrip and 15 shape/dependency/JSON/source
  guards. Every new source byte matches the independent static inventory.
- All 32 original initializers with immediate and deferred paths (64 calls),
  32 canonical pointer getters, 11 raw/sentinel getters, seven unsigned raw
  initializer prefixes and three disabled initializer calls. High argument
  bits, negative signed indices and raw indices outside the projection retain
  their original behavior.
- Two deferred updates, both explicit-source setter modes and one original
  queued DMA setup. The latter checks source, destination, channel flags and
  `A2400001` in IWRAM; it is not an executed raster DMA campaign.
- All 36 original `CpuSet` observations compare the entire 322-byte destination
  immediately after the SWI and before wrap. The final source halfword `013C`
  is observed at `489BB0`; original caller code then replaces destination
  word160 with word0. Next-header `00A0` reads are also observed.
- One synthetic original initializer preserves a high color bit and changed
  range words, including wrapped range metadata. All calls check the complete
  EWRAM and IWRAM outside declared stack scratch, restored SP and callee-saved
  registers; expected live metadata/shadow changes remain explicit.

The original loadsite observations plus checked HLE copies cover all 10496
owned source bytes. The 4753 instructions and 202 PCs are coverage counts,
not performance measurements or a full state/clock equivalence proof. The
component oracle does not validate native consumers, full camera/ABI behavior,
executed raster timing or ordinary gameplay. All 643 hash/mtime guards pass.
No older source/scene tests, fixtures or builds were repeated for context.

Evidence under `target/rom-data-map-raster-palettes/` includes
`windows-model-build.json`, the frozen `recomp-tests-model.exe`,
`windows-model-result.json`, `windows-model.log` and
`windows-model-europe/{map-raster-palettes,result}.json`. That stage left
managed/runtime/CLI/ordinary work pending and EUR e54-d53/USA e8-d7 unchanged.
The next section closes managed publication and targeted consumption.
No save/native ABI, performance, space, Android or package change is implied
by the source model.

## Managed raster palette publication and typed reads

`ROMDATA-03-MAP-RASTER-PALETTE-RUNTIME` publishes
`map-raster-palettes.json` / `graphics.map-raster-palettes` under EUR
v1/e55-d54. The generation has 51 files and 50 resources. All 49 previous
decoded files remain byte-identical and the old e54 generation is retained.
USA remains e8-d7. The resource authenticates its actual
`graphics.bg-descriptors` dependency and all 33 source ranges, 10496 bytes.
The complete EUR resource set is required independently of that semantic link.

Dataset identity is
`cfc4012ce77ff78dd5e77f2b2ff9cb247a62b94e05d0a2fb80e24759a4e321b1`;
the new file SHA-256 is
`d263f1a538fa9bd8c02225c4a6c41b6931b1c85f0e13d886f66a437e22abe9e8`.
These identify the accepted targeted-test generation and identical later
production CLI result. Bounded JSON parsing and source validation precede
activation of one immutable typed owner; no raw ROM shadow or per-read file
access is added.

The owner serves index/header/color bytes through existing CPU, DMA, BIOS HLE
and native ROM APIs. Its page selection includes 487..489. Each read overlays
only its owned bytes on the result from earlier owners. A read across `489BB0`
therefore combines new color bytes with the existing alternate-palette index.
The neighbor is neither duplicated nor taken from poisoned raw ROM. Unowned
neighbors retain the original raw behavior. Usage counters report aggregate
byte reads by source class and a distinct program mask; they do not measure
distinct bytes read during ordinary gameplay.

Original signed/unsigned index arithmetic, live windows and selectors, 322-byte
copies, extra source reads, destination wrap and DMA scheduling remain guest
work. No instruction or cycle is skipped. Save/native/cache identity and state
formats are unchanged; activation remains transactional and the shared owner
survives file loss, Load, Undo, backup restore and restart.

One affected Windows GNU core/frontend test build passes without diagnostics.
The frozen frontend SHA-256 is
`bd3e69fda059e58e4c1932122220fac2304de40cc0a7f3e22f17323a02cfef2a`;
the affected core tests compile without being executed. Two new exact Wine
filters pass on their first executions:

- Managed authentication/lifecycle checks pass 17 coherent or missing-file
  guards, actual descriptor dependency and complete-set checks, failed-write
  isolation, create/reuse/repair/move, all 49 prior file comparisons and e54
  retention. After files become unavailable, Load/Undo/backup/restart retain the
  same owner. With raw sources poisoned, reads cover exactly 64 index, 192
  header and 10240 color bytes across all 32 programs. A mixed word independently
  increments new and alternate-palette counters by two bytes each.
- 28 original/interpreter/compiled cases match complete serialized states and
  guest clocks. They cover immediate/deferred initialization, getters and raw
  prefixes, signed/unsigned arguments, sentinels, live windows, explicit source
  setters, original copies/wrap and queued DMA configuration. All 10496 new raw
  source bytes and the adjacent 314-byte old index are poisoned. Compiled API
  reads serve 15 index, 46 header and 24 color bytes from typed values.
- Additional width/mirror and real-BIOS **bus-mode** reads pass, including mixed
  owners and an unowned raw neighbor. Four original CpuSet/FastSet comparisons
  and one immediate DMA comparison preserve full states/clocks. The last DMA
  reads two bytes from each owner. This does not claim a complete real-BIOS
  execution, whole camera ABI or an executed HBlank raster campaign.

All 646 hash/mtime guards pass. The 638 original instructions, 204 PCs and 55
native dispatches are coverage counts only. The previous source-model oracle
and older runtime/scene proofs were not repeated. Evidence under
`target/rom-data-map-raster-palettes/` includes `windows-managed-build.json`,
`windows-managed-results.json`, `runtime-targeted-outcome.json`, logs and
`windows-managed-europe/{managed,native}-result.json`.

The production CLI and ordinary Wine criteria now pass below. New Linux,
USA, native Windows and new 32:9 checks remain pending.
No performance/space measurement, new gameplay fixture/navigation, Android,
package, agent commit or push was performed.

## Raster palette production CLI and positive ordinary boot

`ROMDATA-04-MAP-RASTER-PALETTE-CLI` and
`ROMDATA-05-MAP-RASTER-PALETTE-PAIR` close the declared EUR/Wine production and
ordinary-use criteria. One required Windows production build passes without
diagnostics; its frozen executable SHA-256 is
`43c9992fc68038796f9c21066d536dd23c568e44963dfa295d2b81f1beebdf77`.
The canonical production executable's exact hash and mtime are restored.
The existing Cargo workflow then invalidates its output assumption by touching
`main.rs`; that file's content is unchanged.

Actual CLI commands from an unrelated working directory create and reuse e55,
with all 53 managed files retaining their hashes and mtimes on reuse. Every
one of the 51 generation files matches the accepted targeted dataset. One
1500-frame cold-boot locator loads the previous native cache and uses the
existing `target/desktop-perf/state-replay/boot-regression/demo.input`, SHA-256
`9152ab80916c04cee8234e7cb17d0cc97fcb73ddfb7e1a1b55452327afb1b322`.
No initial state, cartridge save, PC/RAM override, new fixture or navigation
is supplied. This is a new-family use check, not a repeat of earlier accepted
boot proofs.

The locator is positive for all three source classes: 25440 bus reads,
50880 aggregate bytes = 2 index + 322 header + 50556 color bytes. The program
mask reports two touched source records; reading an extra next-record header
also marks that record. This is not evidence of two complete palette
initializations or 50880 distinct source bytes.

One Wine full pair then reuses that actual CLI generation, frozen targeted
test executable, old cache, input and exact 1500 frames, with widescreen0.
Every complete serialized guest state/clock, central/wide image, OAM and
dispatch result matches. Mixed, PSG, FIFO and bitwise HLE audio match with
sample counts `[3291096,3291096,395212,395212,3269758]`. The new-family counters
match the CLI locator exactly. The run mixes native and interpreter paths;
it does not establish an all-native or new 32:9 result. All 701 hash/mtime
guards pass.

Evidence under `target/rom-data-map-raster-palettes/` includes
`windows-build.json`, `windows-cli-{processes,no-write,summary}.json`,
`windows-cli-boot-raster1500.log`, `windows-scene-result.json` and
`windows-boot-raster-full-state-audio.log`. Source/model/managed/native checks
were not rerun; no new implementation change or test build was needed for
this production/ordinary closure. Other ordinary palette/camera contexts and
whole raster/camera ABI remain outside the declared scene. New Linux, USA,
native Windows and new 32:9 checks remain pending. No performance/space
measurement, package, Android, agent commit or push was performed.
