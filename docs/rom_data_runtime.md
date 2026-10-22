# Direct decoded resource consumption

`play`, `frames` and `runc` accept developer opt-in `--rom-data`. They prepare
and load `graphics.obj-dimensions`, `graphics.bg-descriptors` and
`graphics.bg-base-palettes` and `graphics.bg-alternate-palettes` from the selected
ROM's decoded dataset. The sixth e8-d7file, `graphics.bg-packet-programs`, now
also serves EUR packet source/body reads from immutable typed values and exact
control metadata. EUR reports `runtime=decoded`; USA stays authenticated and
`runtime=verified-only` pending its consumer validation.
EUR prepares a seventh `graphics.bg-main-graphics` JSON, introduced in `v1/e9-d8`,
authenticating every source and file hash into dataset identity. This family
now reports `runtime=decoded`: custom-codec literals come from retained typed
words, with exact original headers/tokens and unchanged guest instructions.
USA keeps
its existing six-file `v1/e8-d7` contract. New Linux/USA verification is pending.
EUR `v1/e10-d9` additionally authenticated an eighth
`graphics.bg-transfer-programs` JSON and its76source ranges. This new family
reports `runtime=decoded`; original transfer readers now use retained typed
controls/tile words through the same CPU/DMA/native bus paths. Original live
frame/countdown/flags/copy cursors and every guest instruction/cycle remain.
EUR `v1/e11-d10` added a ninth `text.short-strings` JSON with670source
ranges. It participates in whole-set authentication/identity and reports
`runtime=decoded`: original readers obtain table/string/glyph/metric bytes from
retained typed indices/codes/words/widths, with separate counters. Original
expansion/copy/render work and guest clock remain intact. See the
[direct text/font contract](rom_data_text.md#direct-consumption).
Current EUR `v1/e12-d11` adds the broad `text.catalogs` JSON with33581source
ranges. It joins whole-set authentication/identity and reports `decoded`:
typed pointers, strings, archive indices/headers/program bytes and font data
serve original CPU/DMA/native reads. Compressed literals come from decoded
units/padding; original controls and guest work remain intact. The overlapping
earlier subset stays authenticated but `verified-only`, with no duplicate read
owner/counter attribution. See the [broad direct-consumer proof](rom_data_text.md#broad-direct-consumption).
One retained EUR/Wine397-frame pub transition now matches every state/clock,
image/OAM/dispatch/audio with positive27decoded program/8064glyph/93metric bytes.
One actual CLI no-write e12reuse/old cache matches the retained legacy row.
[Declared scene/CLI scope](rom_data_text.md#ordinary-transition-and-production-cli).
Normal enablement awaits the remaining ROMDATA-05 regional and native
Windows gates.
Other resource families and mod support remain open in the
[full TODO](development/ROM_DATA_TODO.md). These bounded metadata migrations do not provide ROM-free execution or
complete graphics extraction.

## Sources and typed readers

The twelve shape/size records are decoded into typed width/height entries.
Runtime construction parses the captured file bytes, checks their manifest
hash and exact regional source/provenance, and refuses edited base data even
when their file hashes were recomputed. The manifest's relative file names and
supported resource set stay fixed; unsupported data do not silently use ROM.

The independently verified sources are USA direct table 0x149510 and copied-code
alias 0xa398dc; EUR direct table 0x14ced8 and copied-code alias 0xd976e8. Each range
is 48 bytes, with the shared source digest recorded in the
[inventory](development/ROM_DATA_INVENTORY.md). Searching both exact local
images finds exactly these two occurrences per region. The fourth range was
found during the runtime provenance investigation; extractor revision 2
records it. The original USA initializer now has independent clean-boot
execution/provenance coverage, described below; the EUR replay observes alias
reads without attributing them to an initializer PC.
That alias expansion retained schema/decoder1 and selected `v1/e2-d1`,
preserving `e1-d1`, because the OBJ format was unchanged.
The managed metadata extension introduced `v1/e3-d2`; `v1/e6-d5` added exact
palette source views beside readable banks. The `v1/e8-d7` generation also
contains alternate palettes and packet programs; EUR consumes packets while
USA remains verified-only. Earlier revisions including preparation-only
palette e5and e7are retained.
EUR's `v1/e9-d8` adds authenticated main graphics while retaining e8and the
same prior decoded files. Save and native cache identities stay source-equal.
The EUR e10generation adds transfer programs while retaining e9and
the same prior decoded resource bytes.
EUR e11adds the bounded short-text/font subset, retaining e10and all seven
prior decoded files byte-for-byte. See the [text contract](rom_data_text.md).
EUR e12adds the broad text catalogs, retaining e11and all eight earlier decoded
files byte-for-byte.
Manifest/OBJ stay TOML; BG metadata and base palettes use readable JSON.
The borrowed complete-file view authenticates all resources before changing
the live object and includes every resource ID/file hash in dataset identity.
OBJ, descriptors and base palettes remain in the immutable session and report
`runtime=decoded`; alternate palettes also have direct consumption below. BG metadata and palette CPU/DMA/native reads consume
file-derived projections, with independent counters. The palette LZ77 HLE
consumes decoded bank bytes; raw CpuSet keeps its original bus-read loop.
The alternate palette and packet paths are described below. Other linked
payloads still remain ROM until their direct-consumption units.

`RuntimeResources` in `gba-core::rom_data` owns immutable typed entries and
little-endian source projections:48bytes for OBJ,14256bytes for BG. At the
historical three-family boundary, owned memory was448bytes shared/OBJ including Arc counters and identity strings,
plus28672bytes for BG and94713bytes for palette models, decoded byte views,
source bindings and file identity:123833bytes total on both tested64-bit targets. Enforced bounds were declared
before testing:shared/OBJ<=1KiB and extraBG<=64KiB. Historical OBJ-only
measurements below retain their432byte scope. The machine also holds one optional Arc handle; allocator overhead and
process RSS are separate ROMDATA-05 measurements. The resource contains no file
handle, filesystem callback, translation pointer or serialized guest state.
There is no allocation or I/O on resource reads.

The ROM arms of `MemMap::read8/read16/read32` check only fixed source
pages:four for OBJ, four BG pages per region and fixed palette asset regions
USA56f..697/EUR76c..894. A regional dispatch supplies migrated bytes, including
unaligned/crossing accesses; adjacent opaque bytes retain the original path.
GPIO/EEPROM, mirrors and bounds/open-bus behavior keep their existing order.
This is not a general manifest/resource lookup on every bus access. DMA already
uses these bus reads, and generated native loads already call the same RtApi
callbacks. No guest instruction, modeled cycle, write, IRQ or DMA is skipped. Verified
palette HLE calls use the already decoded bank bytes and the same ordered
`write_out`; their guest producer/control flow still executes. The source ROM buffer is retained for code and unmigrated data.

Host sprite draw decoding reads the typed dimensions directly. Copied battle
and visibility routines retain their RAM table at IWRAM 0x03006d28: unchanged
records may use the typed file data, while genuinely changed RAM records keep
their guest values and are counted. Shape 3 and out-of-table reads keep their
old opaque-memory behavior. Table validation also uses the decoded source
projection when active rather than rereading the old ROM bytes.

The investigation corrected an existing EUR host-reader error: the two main
draw routines use the direct ROM table, not the copied RAM table. The exact
EUR literal at ROM 0x15ac points to guest 0x0814ced8. Both legacy and decoded
host paths now use that verified source; copied battle/visibility readers
still use RAM. This can correct extended sprite placement/clipping for flips
and off-screen objects. Original guest instructions remain unchanged.

## Identity, snapshots and diagnostics

ROM, decoded file and dataset have distinct SHA-256 identities. The dataset
identity includes source, schema, extractor/decoder revisions and every
prepared resource ID/file hash in the pinned order, including BG metadata.
Only a source-equal unmodified base can enter this runtime API. Native cache
keys and verified ROM profiles remain tied to the original image: generated
loads use bus callbacks and the accepted guest representation is byte-equal.
No RtApi layout, generated C or translation revision changed. Future mods must
establish a separate compatibility/cache/save identity before being accepted.

The machine's optional Arc resource owner and counters are skipped by Serde.
Snapshot payloads and version 1 ROM/BIOS identity remain compatible. Successful
Load retains the live session resource object after transactional validation;
failed loads do not replace it. Slot Load/Undo follows that path. Backup restore
clones the selected resources into its prepared clean-boot machine before
loading the boot state or replacing the save. A running session needs no file
access after activation; a new activation with missing files fails rather than
falling back. Managed startup can regenerate those files from the source ROM.

Activation prints `ROM_DATA runtime=decoded` with resource/source/dataset/file
identities and resident bytes. Shutdown/headless completion prints
`ROM_DATA_USE` counters for OBJ bus, typed host ROM/RAM, changed RAM and
validation, plus separate BG bus operations and projected byte counts.
Palette counters distinguish source bus operations/bytes from decoded LZ77
calls/output bytes. Exact compatibility views cover compressed streams and
raw headers; they are validated against banks rather than recompressed.
`bus_source_mask` bit 0 identifies the direct/primary source, bit 1 the copied
alias. Counters describe use, not timing or speedup. An in-view boot scene can
legitimately show zero guest metadata reads because the original draw routine
fetches dimensions only for clipping/flips; host file use is recorded separately.

## Current base-palette consumption evidence

The162record-indexed models cover127unique sources per region (125LZ77/2raw).
The owner authenticates exact encoded bytes and decoded banks together. Raw
body reads come from bank bytes; compressed/header reads come from the file's
source representation. LZ77 SWI11/12only select a bank at an authenticated
source start, including cartridge wait-state mirrors. Dynamic RAM and other
ROM sources keep the existing decoder. Real BIOS guest code can read every
original stream byte through the bus; no actual BIOS-ROM execution is inferred
from the byte-equivalence checks.

The existing HLE charges no per-source-byte cycles or device effects for these
ROM spans. Both paths execute the same SWI, instruction cost/return hook and
ordered halfword writes, including paletteRAM/VRAM/OAM destination semantics.
This preserves the emulator's modeled timing, not a new hardware BIOS claim.

Linux and Windows GNU/Wine pass210serial core tests each (7optional ignored),
including all widths/mirrors/edges/native callbacks, poisoned source views,
DMA16/32, SWI11/12, raw CpuSet, all destination types and full snapshot equality.
Frontend defaults pass101Linux/100Wine (22optional ignored). Four original
producer cases (EUR/USA ×Linux/Wine) each pass1296calls/72792instructions and
4208native dispatches after poisoning every old source span, including headers.
Interpreter registers/CPSR/clock/RAM match at every step and native boundary;
complete final states match. Each file owner records1272decoded LZ77calls /
231936output bytes and3544source bus reads. Load/Undo/backup, failed binding,
corrupt/deleted files and continued use after deletion preserve the live owner.

New EUR1200-frame native pairs match every full state, image, OAM, dispatch and
mixed/PSG/FIFO/HLE audio on both targets. Each consumes one160byte decoded
palette and53BG reads. Actual CLI checks cover e5refusal/retention, current
extract/verify/create/reuse/repair/move; interpreter/native1200-frame pairs have
positive palette diagnostics and identical legacy frame/final-state hashes.
See [the graphics contract](rom_data_graphics.md#direct-base-palette-consumption).
The subsequent e6recorded-scene checks now pass240worldmap,3530battle/menu
and8476transition frames on both targets, including every full state/image/OAM/
dispatch/audio stream. Stable segments have zero palette reads; the transition
consumes one160Bdecoded palette. See the [scene results](development/ROM_DATA_VALIDATION.md#base-palette-e6-scene-acceptance).
GUI/costs, native Windows and default enablement remain05gates.

## Current BG consumption evidence

The authenticated162-record block atUSA569104/EUR7662a8 now supplies all
fixed-source bus projections. No payload decoder or original producer is
bypassed. Linux/Wine pass204core tests (7optional ignored), including every
width/mirror/edge, native API reads, CPU/DMA and unchanged snapshots. The
frontend defaults pass98Linux/97Wine (18optional ignored).

Real e3files feed2608selector invocations per region/target, with raw descriptor
ROM bytes poisoned after activation. Original interpreter, file-backed
interpreter and actual TinyCC code agree on registers/CPSR/clock/EWRAM/IWRAM;
restoring only the test poison gives identical complete snapshots. Each case
records134424original instructions,5836native dispatches,660event-boundary
interpreter shells,3867native BG byte-load callbacks and3888total BG reads.
No zero-fallback claim is made. Missing/corrupt activation is transactional;
reads survive deletion of the activated BG file, Load, Slots Undo and backup
clean boot with the same resource owner and retained counters.

New EUR native1200-frame clean-boot pairs pass every complete state, central/
wide image, OAM, dispatch and mixed/PSG/both FIFO/HLE audio stream (HLE by bits)
on Linux and Wine independently. Both reach53BG reads; all five audio streams
are nonempty. Existing target-specific TinyCC objects are reused. Actual
production CLI evidence and detailed limits are in the
[graphics runtime section](rom_data_graphics.md#direct-descriptor-consumption).
Broader BG scenes/GUI/costs and native Windows remain separate05gates.

## Retained OBJ evidence and scope

- Linux release suites pass 191 core and 95 runtime tests; Windows GNU/Wine
  passes 191 and 94. Optional local-ROM/native tests are run separately. Tests
  poison the old ROM table bytes and check every bus width, mirror and source
  boundary, native API reads, CPU registers/cycles, DMA and mutable RAM handling.
- A 48-case original-ROM instruction matrix runs both main draw routines in
  EUR/USA with all twelve shape/size records, flips and clipping positions.
  Registers, CPSR, clock, RAM writes and host replacement attributes match at
  every original step. Each region records 239 decoded guest reads and 288
  typed host reads. No original instruction bytes are included in the source.
- Private file/lifecycle tests pass on both targets/regions: file provenance,
  byte-equal pre-activation states, rejected missing-file activation, resident
  reads after file deletion, slot Load/Undo and backup clean-boot resource
  retention. State schema and cartridge save format are unchanged.
- Actual native loops compare all 600 full states, central/wide images, OAM,
  dispatch counts and mixed/PSG/FIFO/HLE streams (HLE by float bits). Linux and
  Windows/Wine each pass against their own legacy reference, using existing
  TinyCC ELF objects. The Windows cache has a `.dll` filename but is loaded by
  TinyCC, not the OS DLL loader. Each run consumes 3,519 typed host records;
  guest-read coverage comes from the original-routine matrix, not this boot.
- Actual `frames` and `runc` commands on both platforms match their corresponding
  legacy 120-frame hashes and report decoded activation/use. ROM/library hashes
  stay unchanged. Actual interpreter GUI runs create, close, move the portable
  folder and reuse the identical generation with positive host file-use counts;
  captures are inspected and exits are clean.

The initial boot gate failed on zero guest reads even while all 600 comparisons
matched. It exposed both a missing EUR copied alias and the host reader's wrong
source. The failed probes are retained; source inspection and the stronger
original-routine matrix establish the correct conditional-read coverage.
The final boot result is reported at its actual scope. A missing TinyCC path
and missing helper working directory were setup failures before useful runs;
existing fixtures/binaries were reused after correcting only those helpers.

The next ROMDATA-05 tranche now passes every full state, central/wide image,
OAM, dispatch and complete mixed/PSG/FIFO/HLE stream for worldmap240frames,
observed battle/menu/enemy3530frames and protected-save boot-to-battle8476frames
on Linux and Windows/Wine independently. Audio observation is configured equally
after loading snapshots; HLE compares bits with actual samples. The copied EUR
alias is read at frame5400, mask3, while guest RAM/clock/RNG remain equal.
The USA initializer trace is not inferred from this EUR replay.

A separate USA test boots without a save or forced PC, using demo inputs,
and reaches the original copier at0808f74c on frame791/clock222325013. Its13
original instructions return to08023f76; every complete state matches a legacy
instance starting from that real caller checkpoint. CpuSet at0814186c copies
3648bytes to IWRAM5f28, including24decoded halfword reads for the48byte table
at6d28. A separate probe poisons only the raw alias after authenticated file
activation: the entire copied blob and final state still match. This passes
independently on Linux and Wine. It proves the original USA initializer's file
provenance, not full-USA scene/audio parity or native Windows execution.

Five native Linux alternating timing pairs versus a corrected legacy binary
without resource routing measure +1.0081% worldmap/-0.2355% battle mean cost;
p95/p99 also pass predetermined limits. This is a neutral result, not a speedup.
Seven first/cached helper observations give median32.416/21.019ms on Linux and
109.222/91.891ms on Wine after source read. Wine cached maximum183.130ms is
retained; the100ms gate applies to the median. Native process peak RSS deltas
are672KiB median/752KiB maxima over seven pairs; owned resources432B. A managed
one-generation root uses1164logical/32768allocated bytes on the host filesystem.
[Protocol, exact costs, comparisons and reproduction](development/ROM_DATA_VALIDATION.md).

Portable archives include these resource help documents. Synthetic Linux tar/
Windows zip marker checks prove the selected inputs exclude neighboring private
resources, ROM/save/state/recordings, native caches and generated code; no new
release export is claimed. Actual decoded-resource GUI native/interpreter
checks now pass on Linux/Wine: held/toggled Fast, pause/resume preserving Fast,
return to1x, Slot Load/Undo/reload and a separate in-menu backup clean boot.
Real callbacks at44.1/48kHz report0device errors, over-period callbacks and
producer drops; muted output and transition-inclusive profiles do not establish
audibility or steady zero-starvation. Protected files and source hashes match.
[Detailed GUI evidence and limits](development/ROM_DATA_VALIDATION.md#actual-gui-and-callback-lifecycle).
Actual Docker archives also pass395Linux/392Windows-Wine tests (24optional
ignored each), privacy/identity checks and extracted-runtime GUI Load/Undo/
reload with automatic bundled TinyCC and fresh portable resources. The Docker
context/COPY lists now include both resource guides required by the packager.
Linux requires at most GLIBC2.35; Windows PE/resources/dependencies are checked.
Native Windows remains before normal enablement; the user currently has only
Linux/Wine. Other families/mods stay open. Canonical runtime and archive bytes are preserved;
a prior BUILD-INFO metadata hash variation is retained in the
[artifact evidence](development/ROM_DATA_VALIDATION.md#actual-portable-archives).
Game-derived test artifacts remain local/ignored.

The user's current priority is functional migration without performance
measurement or space-reduction work; optimization comes later. Prior benchmark/
RSS limits and results are historical, and no performance campaign should be
resumed from an older next-action line. Guest clock/state/audio correctness,
source provenance and save/native compatibility checks remain required.

## Alternate palette banks

The verified `e7-d6`dataset now retains both alternate catalogs in immutable
session resources. Literal payload reads for the original custom decoder come
from fullu16banks; typed relative links and authenticated control metadata
preserve all original guest instructions/writes/cycles. These bus paths serve
CPU, DMA and native API reads without runtime disk access. Native cache and
snapshot identities remain based on the original image; no ABI/schema bump.

`ROM_DATA_USE resource=graphics.bg-alternate-palettes` reports `bus_reads`,
`source_bytes`, `table_bytes` and `decoded_literal_bytes`. The last counter
proves decoded bank use independently of preparation/control-byte reads.
Positive original/native poisoned-source and separate native-API bank probes,
full-state/clock equivalence and deleted-file Load/Undo/backup/restart pass on
Linux/Wine. The translated path includes existing interpreter shells; a cold
first-frame CLI check is activation/cache evidence with0alternate reads.
See the [complete contract and results](rom_data_graphics.md#direct-alternate-palette-consumption).
Original EUR Color Mode LCD B/TV now has actual Cyril-pub source-use evidence:
four397-frame Linux/Wine legacy/decoded native pairs match every full state,
image/OAM/dispatch and audio stream. Actual GUIs present both modes and close0
with positive typed link/literal reads. See the [scene protocol and limits](development/ROM_DATA_VALIDATION.md#alternate-palette-e7-scene-and-gui-acceptance).
Global FFpreconditions, broader regional/gameplay/catalog/language coverage,
archives and native Windows remain separate. No performance measurements were selected.

## Packet dataset activation and EUR consumption

The e8-d7six-file set now includes typed ordered packet programs in preparation,
source verification and complete dataset identity. Packet activation failures
preserve the current owner/guest state; a source-equal file annotation changes
the complete identity, leaving prior family identities and guest snapshots
unchanged. [Managed packet checks](rom_data_graphics.md#managed-packet-program-datasets)
pass with four focused Wine cases and seven actual Windows CLI operations;
old cache dispatch is positive in a single cold activation frame. This is no
packet-use proof. Subsequent04now binds actual EUR reads: the owner retains
typed ordered writes, source/body control metadata and shared decoder-derived
literal maps. Raw packet values and LZ literals are reconstructed from typed
words; existing LZ77HLE writes the predecoded typed body through its normal
destination writer. CPU/DMA/native/guest-BIOS bus reads retain exact original
bytes. The fixed EUR asset pages include895, and reads crossing adjacent
palette/packet spans combine both file-derived views.

No guest packet instruction, dynamic initial fill, inheritance, continuation,
write or cycle is bypassed. Mutable RAM and unregistered sources retain their
original paths. Host resources/owned reads survive Load/Undo/backup/restart and
unavailable source files without entering snapshots. Schema/extractor/decoder
remain1/8/7; native/save/profile/cache identities remain source-equal.

Three focused Wine checks pass: a synthetic boundary/SWI/DMA/owner test, nine
EUR original/native calls with every packet source span poisoned, and a
temporary-copy lifecycle test. The nine calls match complete states/clock,
destinations, guards and stack, with18compiled typed bytes and10506typed HLE
bytes. One397frame LCD B Cyril-pub transition matches every full state, image,
OAM, dispatch and mixed/PSG/FIFO/HLE sample, consuming2640typed packet bytes
through three HLE calls. Its raw typed bus count is0; the component probe
provides that distinct evidence. See [the full contract](rom_data_graphics.md#direct-european-packet-consumption).
New Linux/USA, broader scenes, GUI/archive and native Windows checks remain
pending. No performance/space campaign or normal-enablement claim is made.
