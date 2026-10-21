# Direct decoded OBJ metadata consumption

`play`, `frames` and `runc` accept developer opt-in `--rom-data`. They prepare
and load `graphics.obj-dimensions` from the selected ROM's decoded dataset.
Normal enablement awaits the remaining ROMDATA-05 GUI, regional and native
Windows gates.
Other resource families and mod support remain open in the
[full TODO](development/ROM_DATA_TODO.md). This is a first-family migration,
not ROM-free execution or complete graphics extraction.

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
records it. Its position relative to the copied code matches the RAM table;
the actual battle initializer/read trace still belongs to ROMDATA-05.
Schema and decoder stay at 1 because the decoded format is
unchanged. Managed storage therefore selects `v1/e2-d1`, retaining `e1-d1`.

`RuntimeResources` in `gba-core::rom_data` owns immutable typed entries and a
48-byte little-endian guest projection. A source-bound session allocation is
432 bytes by type-size/string-capacity accounting on the tested 64-bit targets,
including Arc counters and identity strings, below the enforced 1 KiB resource
limit. The machine also holds one optional Arc handle; allocator overhead and
process RSS are separate ROMDATA-05 measurements. The resource contains no file
handle, filesystem callback, translation pointer or serialized guest state.
There is no allocation or I/O on resource reads.

The ROM arms of `MemMap::read8/read16/read32` check only the four fixed source
pages. A regional constant-range dispatch supplies migrated bytes, including
unaligned/crossing accesses; adjacent opaque bytes retain the original path.
GPIO/EEPROM, mirrors and bounds/open-bus behavior keep their existing order.
This is not a general manifest/resource lookup on every bus access. DMA already
uses these bus reads, and generated native loads already call the same RtApi
callbacks. No guest instruction, cycle, write, IRQ, DMA or decompression is
skipped. The source ROM buffer is retained for code and unmigrated data.

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
identity includes source, schema, extractor/decoder revisions and file hash.
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
`ROM_DATA_USE` counters for bus, typed host ROM/RAM, changed RAM and validation.
`bus_source_mask` bit 0 identifies the direct/primary source, bit 1 the copied
alias. Counters describe use, not timing or speedup. An in-view boot scene can
legitimately show zero guest metadata reads because the original draw routine
fetches dimensions only for clipping/flips; host file use is recorded separately.

## Evidence and scope

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
release export is claimed. Fast/pause GUI and realtime audio behavior, USA
initializer coverage and native Windows hardware remain ROMDATA-05 work before
normal enablement. Other families/mods remain open. Canonical binaries and input
hashes are preserved; resources and game-derived artifacts stay local/ignored.
