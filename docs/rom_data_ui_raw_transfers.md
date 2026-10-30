# European localized raw UI transfers

## Declared source contract

`ROMDATA-01-LOCALIZED-RAW-TABLE-SOURCES` authenticates seven independently
rooted language pointer groups: `71e5f4`, `71e608`, `71e66c`, `71e694`,
`71e6a8`, `71e6e4` and `71e6f8`. Each contains five declared little-endian
pointers selected by live unsigned byte `02001f6e` multiplied by four. All
13 aligned root literals in the retained ROM census are covered. Physical
slots0..4 describe ownership; the original uncapped address calculation for
language5/255 remains separate. No fallback or skipped guest read is inferred.

These are raw transfer views. Their lengths come from original helper controls,
not headers, pointer spacing, tile names or visual interpretation. In root order:

- `71e5f4`:224 bytes from each bank start. Three original callers copy through
  `145c14`, which branches throughr3 to the copied IWRAM memcpy callback.
- `71e608`:160 bytes from each bank start, copied to scratch+`e0` through the
  same callback. The subsequent merged scratch upload adds no ROM ownership.
- `71e66c`:768 bytes from each bank start. The fused call at`12ee24` targets
  `145230`, SWI0B/CpuSet with`0180` halfwords.
- `71e694`:1600 bytes from each bank start, the union of two conditional views:
  first832 bytes, or768 bytes at+`340`. The original helper result's low byte
  chooses the branch; no meaning for that mode is invented.
- `71e6a8`:2048 bytes from each bank start. A live task queues two1024-byte
  copies through`000b14`, control`84000100`; the zero-task branch calls
  SWI0C/CpuFastSet with`0200` words for the same full extent.
- `71e6e4`:2432 bytes from each bank start, SWI0C with`0260` words.
- `71e6f8`:only384 bytes at bank+`0200..0380`. Three separate state handlers
  queue128-byte copies at+`200`,+`280` and+`300` through`000ad0`, each with
  control`80000040` (halfwords).

The `000ad0`/`000b14` helpers store low16 count bits plus their respective
DMA controls in the linked transfer queue. The original executor preserves
the ROM source, chooses DMA3, and writes source/destination/control registers.
The queue, guest timing, live destinations and later uploads remain original.
The callback mapping through ROM`370eb4` to IWRAM`03005ee9` reuses the closed
[raw palette proof](rom_data_ui_palettes.md#source-and-consumer-contract).
Copied executable code is not a data resource.

The declared35 views and seven selector tables form42 disjoint spans,
38220 source bytes (38080 payload+140 selector). Exact little-endian word
reconstruction passes; concatenation by source offset has SHA256
`e11ef4d5f16feea8ef90e7fce58056e19d689004b7ca973270285e3cd31b8f80`.
No span overlaps the83 accepted e92-d91 owners/50602 runtime bindings.
All84 accepted dataset files and guarded inputs remain byte/mtime exact.

## Dynamic views remain open

Four additional calls through`000ad0` use root`71e6f8` with64-byte transfers
at offsets from two little-endian u16-pair tables, rooted at`3996f4` and
`39970c`. Live byte task+2 minus4 selects a pair; each value is shifted left5.
Bit3 at`030028a0` chooses the table. The raw calculation has no observed upper
bound, and the caller dispatcher does not establish one. A six-row count from
24-byte spacing or convenient values is therefore rejected. Those metadata,
any lower bank prefix and offsets outside the declared fixed union remain
unowned. This source contract does not close the entire seventh bank catalog,
its ordinary liveness, all producers or the full helper ABI.

## Verification and remaining work

Static authentication covers32 windows (23 new/nine reused),193 instruction/
literal pins,13 root literals,13 fixed transfer calls and the four dynamic
calls. One classifier attempt stopped before producing outputs when the fused
`12ee24` target contradicted an initial word-copy assumption. The corrected
classifier also authenticates `000ad0` halfword controls rather than borrowing
`000b14` word units. No incorrect extent was published or tested. An unrelated
`04fc70` search window is retained as diagnostic evidence, not consumer proof.
The successful result preserves130 hash/mtime guards. Evidence stays private
under ignored `target/rom-data-localized-raw-table-sources/`.

This is source analysis only: schema1/e92-d91, runtime code and accepted data
are unchanged. Typed modeling, a reduced original-source Wine check, managed
consumption/native state-clock comparison, actual CLI and positive ordinary use
remain pending. Dynamic source domains, the separate known`71e5e0` alias,
indirect/global extent and gaps/tails remain open. New Linux/USA/native Windows/
32:9 and the broader ROM TODO remain pending. No build, game, new checkpoint,
navigation, older acceptance replay or performance/storage measurement ran.

## Typed model

`rom_data::ui_raw_transfers` now provides a strict `RawTransfers` model with
seven tables and35 ordered `BankView` values. A view preserves its bank pointer,
relative source offset and little-endian words. It reuses the existing
physical five-slot table representation while preserving the original raw
language address calculation. The seventh group's lower prefix and tails
remain outside its view. Shape validation and exact EUR provenance are
independent: a raw word edit can retain valid shape and fail source
authentication. The catalog authenticates the32 retained reader windows and
all38220 declared source bytes. The model unit left e92-d91 and its84-file generation unchanged; the following
runtime unit now registers the resource separately.

The focused EUR Wine check passes all38220 source bytes/42 spans, strict JSON,
21 shape/source guards and91 original selector prefixes (13 consumers, each
with language0..4/5/255). Isolated unit inputs execute60 original transfer
helper calls, including25 real queue-executor/DMA copies. Callback byte copies,
CpuSet halfword copies, CpuFastSet word copies and both queued DMA widths agree
with independent source bytes. Complete EWRAM, VRAM, OAM and palette guards,
IWRAM outside the actual stack scratch, SP and callee registers, and final ROM
all match. These are unit inputs, not ordinary game fixtures or full caller ABI
proofs. The original IWRAM callback code is installed from its authenticated ROM
copy in the isolated unit machine; the closed initializer proof is not rerun.

The accepted603-source/262-core-input Windows build is warning-free, SHA256
`35bdc5fd15d2c458df63aadecfabfeea766c44ef2924b92d5dafc57c9105f69c`.
Three Windows builds and two Wine attempts were needed. Before Wine, a test-only
stack guard was corrected from20 to36 bytes for the original DMA wait helper.
The first Wine attempt then stopped at an incorrect expectation that every span
edge was raw: three adjacent table boundaries correctly select another owned
table. The corrected oracle uses independent spans. The model and source
contract did not change for either test correction; diagnostic executables,
logs and exact source preimages remain retained. The accepted result preserves
44 unit guards,130 source guards,601 baseline source hashes and three current
implementation paths; the hub inverse matches exactly. All84 accepted e92 files
remain unchanged. The6635 executed instructions count coverage only.

Private evidence is under`target/rom-data-localized-raw-transfers-model/`.
Managed registration and selected native state-clock checks are completed
below. Actual CLI/ordinary use, dynamic index domains and previously listed
platform/catalog limitations remain pending.
No gameplay/checkpoint/navigation or older acceptance proof was replayed; no
performance or storage measurement ran.

## Managed raw transfer resource

The implementation now registers `graphics.ui-raw-transfers`, fixed file
`ui-raw-transfers.json`, format `ui-raw-transfers-v1-json`, at EUR schema1/
extractor93/decoder92. It adds index83 to the previous83 resource IDs, making
85 files including the manifest. The prior83 entries and payloads are retained.
Strict source/model/binding validation precedes transactional attachment.

The immutable owner stores typed bank words and pointer tables, resolving
each source byte independently and composing crossing reads with earlier
owners or raw ROM. The seventh group's unowned prefix and tails remain raw.
Separate nonserialized counters expose bus reads, served source bytes, selector
bytes, payload bytes and distinct banks/groups used. Original language loads,
raw index arithmetic, CPU copies, transfer queues, DMA destinations and guest
clock work remain active. No direct host rendering or transfer shortcut is
introduced. The fixed parser guard follows the existing bounded-input policy;
no storage optimization or allocation cap for these views is added.

The reduced EUR Wine integration check passes publication of85 files/84 IDs,
retaining all83 previous records and payloads exactly. New-file interruption
leaves no published manifest. Fourteen source-coherent shape/hash/path/binding/
coverage rejection cases and two full-set dependency checks preserve the prior
attached owner and complete machine state after failed attachment.

Twenty-three original/interpreter/native paths match complete states and guest
clocks with all38220 new source bytes and four preceding indexed-selector bytes
altered after Load/setup. These cover12 loads, one original CpuSet call, one
original transfer-queue executor and nine raw-language prefixes, split into
four native families that exclude their stop boundaries. Read8/16/32,
interpreter fallback and the DMA-causing write32 callback are all accounted.
939 served new bytes and three prior indexed bytes are observed; fallback
accounts for768 of the new bytes, so this is not an all-direct-native or distinct
catalog-coverage claim. The115 visited instructions and25 dispatches count
coverage only.

Every declared byte,35 views/seven groups,140 selector bytes and38080 payload
bytes match the ROM through the bus. All42 span edges/widths/mirrors, adjacent
new tables, prior indexed data and sparse raw prefix holes pass. The archive
at`441608` starts exactly at the new sparse view's end; it remains served by
its previous indexed owner. Load/slot Undo retain the immutable owner while
files are unavailable. Four64-byte transfers, mixing prior indexed, prior UI
and new selectors through DMA16, DMA32, CpuSet and CpuFastSet, match complete
states and guest clocks.

The accepted605-source/263-core-input Windows build is warning-free, SHA256
`4b219adc54111f10b01657afe08092fb88f366dd4fecda609055b3413dbd966f`.
Two builds and one Wine filter were used. Before Wine, only the new test leaf
was corrected to distinguish the unowned lower prefix from the prior-owned
successor archive; product runtime code was identical between builds. The first
warning-free executable/log/source preimage remain diagnostic evidence.
309 preservation guards cover605 frozen sources,603 model inputs, seven
current implementation paths and all84 old accepted files. The current-only
parameter27 observer is preserved, excluded only from the frozen overlay.

Accepted dataset SHA256 is
`309b6eaf5af3c3ab26dc7b87050723b79f205fcb83dee0054625f38a7cdd1a60`;
manifest SHA256 is
`a022df159d73ef3b7f1e2f1ec5b62d3024507898386138d95d87cbd888e821b0`;
new JSON SHA256 is
`2a89044c7724107e32360279ff44d801e2f73f9802309dcf853394c29f73ef65`.
Private evidence is under`target/rom-data-localized-raw-transfers-runtime/`.
The runner's inherited console e92/DMA3 labels are stale metadata; accepted
JSON asserts e93-d92 and four mixed transfer cases. No build or test is repeated
for those labels.

Actual CLI/ordinary use, full caller ABI, dynamic index domains and the broader
catalog/platform limitations remain pending. The closed source/model proofs
are not replayed, no ordinary game fixture is created, and no performance or
storage campaign runs.

## Actual CLI registration

The actual Windows CLI now creates and reuses EUR e93-d92 under Wine, then
loads a retained authorized normal EUR checkpoint copy at24755 and advances
one neutral frame to24756. All three child processes exit0. All85 generation
files are byte-identical to the accepted runtime dataset; all84 logged resource
ID/hash pairs match. Reuse and startup preserve hash/mtime for87 managed paths
and the old native cache with28859 loaded blocks. The new six raw-transfer
counters are zero, establishing registration/startup compatibility only. No
positive ordinary consumption or full scene equivalence is claimed.

One warning-free production build uses exactly the accepted605 runtime source
files, without overlays. Its SHA256 is
`ffda6c595cfd38eae46e8f9ac2ee709ba077b4198a053f25b58ba09be214c483`.
The canonical executable's exact hash/mtime is restored in the build's finally
block.203 preservation guards cover the605 CLI and605 runtime source hashes,
all85 accepted files, original ROM/checkpoint/cache/TinyCC and canonical
executable. The private adopted generation is e93-d92/generation-5FQy4l under
`target/rom-data-localized-raw-transfers-cli/windows-cli/resources/`; receipts
and logs are retained in that ignored unit. No new checkpoint endpoint, game
action/navigation, earlier acceptance replay or performance/space campaign
is involved. Positive ordinary use, dynamic index domain/full helper ABI and
all previously listed platform/catalog limitations remain pending.
