# European static gameplay records

Current EUR schema 1 / extractor 27 / decoder 26 adds fixed
`linked-metadata4.json`, resource `gameplay.linked-metadata4`, format
`linked-metadata4-v1-json`: 20 two-pair records / 80 source bytes. All 22 files /
21 resources authenticate together; all 20 earlier decoded files and e26
generations remain unchanged, USA stays e8-d7. Actual parsed metadata52 indices
and pair selectors validate against each other. Typed selector/value bytes feed
original CPU/DMA/native code, preserving counting, contiguous walking, unsigned
values/signed comparisons, tally wrapping, live ability scans, clock and
save/native ABI. Two focused Wine integration checks and a positive 540-frame Change Jobs
full-state/image/audio/actual CLI pair pass with 32 selector / 6 value bytes.

EUR schema 1 / extractor 26 / decoder 25 added fixed
`linked-metadata52.json`, resource `gameplay.linked-metadata52`, format
`linked-metadata52-v1-json`: 116 records / 6032 source bytes. All 21 files /
20 resources authenticate together; all 19 prior decoded files and e25
generations remain unchanged, USA stays e8-d7. Typed header/routing/packed/tail/
opaque fields feed original CPU/DMA/native reads while the original getter,
alternate indices, redirect behavior, live records, guest clock and save/native
ABI stay intact. Two focused Wine integration checks and one positive 540-frame Change Jobs
full-state/image/audio/actual CLI pair pass with 664 newly typed source bytes. Detailed packed-field semantics retain structural
names.

EUR schema 1 / extractor 25 / decoder 24 added fixed
`linked-metadata16.json`, resource `gameplay.linked-metadata16`, format
`linked-metadata16-v1-json`: 128 sixteen-byte records / 2048 source bytes. The
full dataset has 20 files / 19 resources; all 18 previous decoded files and e24
generations remain unchanged, USA stays e8-d7. Original CPU/DMA/native readers
consume typed fields while live-list lookup, indices, RNG state, arithmetic,
shifts, guest clock and save/native ABI remain original. Focused Wine source,
managed/native/lifecycle and one positive 520-frame Clan Mission Item
full-state/image/audio/actual CLI pair pass. That ordinary scene reads 6 typed
display bytes; broader field names remain structural.

EUR schema1/extractor24/decoder23added fixed `item-links.json`, resource
`gameplay.item-links`, format `item-links-v1-json`:225counted pair records/
4500source bytes. It depends on the authenticated item-record resource and
checks all376parsed item owners. That e24dataset has19files/18resources;
every17previous decoded file and e23/older generation remains unchanged.
Typed count/selector/value/opaque bytes feed original CPU/DMA/native logic.
Retained reduced Wine source/managed/native/lifecycle and ordinary headgear
scene/CLI acceptance remain valid. New Linux/USA/native Windows and wider
gameplay catalogs stay pending.

EUR schema1/extractor23/decoder22introduced fixed `items.json`, resource
`gameplay.item-records`, format `item-records-v1-json`. Its376static records
own12032source bytes; that e23dataset had18files/17resources and retained all16
previous decoded files/e22. USA stays e8-d7. Live unit/inventory/task state and
property modifiers remain original RAM/code. The source contract and retained
Wine acceptance for that earlier resource remain below.

## Item record source

The next sixteen-byte metadata table has a separate
[source-only contract](#sixteen-byte-linked-metadata-source-contract). Its
semantic family remains unproved; managed/runtime acceptance follows below.

Following the original shop list071030through category classifier0cf794roots
property reader080ce1ec. Its checked literal080ce20ccontains086e0bd0and
original instructions compute base + (u16item_id <<5). Independent list builder
080cf820uses the same source via literal080cf8dc; it enumerates1..361or
362..375for the separate mission category. Original inventory helper080cec58
independently uses boundary IDs361/375and mission count14, but returns live
EWRAM inventory02001940, not static item records.

The next independently rooted table is086e3ad0. Reader08072134loads the item
base through08072150, fetches item byte1dand uses it as a20byte index into that
next table through08072154. Together the independent roots and enumerated IDs
bound376records ×32bytes =12032bytes at ROM6e0bd0..6e3ad0(exclusive), including
record0without inventing a meaning for it. This span overlaps no e22resource.
A raw pointer-word search finds70matches to the item root; these are discovery
candidates, not70verified code readers.

## Original field access contract

Property reader080ce1ectruncates item ID to u16and property ID to u8. It has
no local item-ID bounds check. Properties0..18dispatch through a code jump
table at080ce214; other property values return0. That table remains code.

Properties0/1/2read unsigned halfwords at offsets00/02/04;3..8read unsigned
bytes08..0d;9reads halfword0e;10..17read signed bytes10..17;18reads unsigned
byte1d. Every32byte record must be preserved, including fields not exposed by
this getter. A lossless candidate representation separates four halfwords00..06,
eight bytes08..0f, eight signed bytes10..17and eight bytes18..1f. Semantic names
for most fields remain unproved; do not infer item names, prices, abilities or
job rules solely from plausible numeric values.

Properties10..13are dynamic. Original getter080ccf00reads live EWRAM bytes at
02002030+u16index. With byte13nonzero, the property reader returns the raw
signed field. With that byte zero, item IDs0033/012e/0094/011eadd live bytes
1/3/2/4to properties10/11/12/13respectively, then apply the original unsigned
comparison/clamp to255. Negative raw values and that unsigned clamp must not
be normalized into a new rule. Live modifiers remain guest code/state, never
frozen into extracted ROM records.

## Evidence and next criterion

Private read-only evidence stays under ignored target/rom-data-gameplay-items:
shop-inventory-071030.txt, callee-080cf794.txt, callee-080ce1ec.txt,
callee-080cec3c.txt, callee-080cec58.txt, item-reader-*.txt,
item-live-modifier-reader.txt, inventory-items.py, item-inventory.json and
item-records-independent.json. Independent static checks verify the exact EUR
hash, selected LDR literals/stride instructions/code targets, both enumeration
bounds, nonoverlap and complete12032byte typed-field roundtrip.

The main-record static inventory is closed. The item-linked20byte table has
the source and managed/runtime acceptance below; jobs/abilities/laws/AI, other
item semantics/readers and regional catalogs remain separate work.

## Typed file and direct consumption

Strict JSON stores four u16halfwords00..06, eight u8bytes08..0f, eight i8fields
10..17and eight u8tail bytes18..1fper record. Fixed array types, exact source
extent/count and original root/stride/enumeration/code-target checks preserve
all32bytes and record0. Source equality rejects coherent payload edits,
reordering, missing/extra records and foreign ROM images. A2MiB file-read guard
bounds malformed inputs; this is not an extracted-space optimization budget.

All18files authenticate before transactional attachment and contribute to whole
identity. Managed preparation retains e22and earlier generations through create/
reuse/repair/move. CPU/DMA/native and guest-BIOS reads project the exact source
byte from typed fields, including reads crossing record/family/source edges.
Source/typed-prefix-halfword/signed/remaining-byte counters are nonserialized.
Original property dispatch, live02002030bonuses, signed loads, unsigned255clamp,
secondary table reads and inventory updates keep executing with their clocks.
No property routine or host ROM consumer is replaced. Save/native ABI and base
ROM identity remain; ownership survives Load/Undo/backup/restart with dataset
files unavailable. PPU/audio continue consuming original live outputs.

## Reduced Wine verification

One original-source check passes14371calls: all376records/19properties under
both live-modifier branches, special-item bonuses, u16/u8truncation and invalid
property handling; independent12032byte roundtrip/source/hash/strict JSON guards
and stack/callee/live-RAM guards pass. The shipped signed fields all happen to
be nonnegative. The first fixture wrongly assumed a negative exists and failed
before executing original code; corrected evidence is retained.240isolated
in-memory negative-byte cases then prove sign extension, bonuses and unsigned
clamp without changing/authenticating/publishing real source data.354477original
instructions/139distinct PCs are coverage only. An initial permission-review
timeout created no process; one permitted retry started the test.

Two focused integration checks pass publication interruption/create/reuse/
repair/move/e22retention, all16old decoded files unchanged, coherent prefix/
signed/tail/reorder/source/dependency/path/role transactional rejection, and
unavailable-copy Load/Undo/backup/restart.86sampled original/interpreter/native
calls with only12032new raw bytes altered match complete states/clocks. Native
reads consume100typed source bytes(24prefix-halfword/40signed/36remaining),
with live bonuses/clamp/truncation active. All bytes/edges/widths/mirrors/real-BIOS
bus/CpuSet/FastSet and4typed DMA bytes pass. The full source-only14371+240oracle
is not repeated. Windows test/production builds are warning-free; canonical
executable restored. Private evidence additionally includes windows-original-
result.json, windows-original-europe/{items.json,result.json},
windows-managed-results.json, windows-managed-europe/{managed-result.json,
native-result.json} and Windows build logs in the same ignored directory.

Actual CLI through Wine creates/reuses e23without dataset writes from unrelated
cwd using the existing native cache. The retained320-frame worldmap→Party→Unit
input positively consumes40new source bytes, all signed fields. One320-frame
ordinary Wine pair matches every complete state/clock/central-wide image/OAM/
dispatch and mixed/PSG/FIFO/HLE audio sample with the same positive consumption.
Prefix/tail ordinary use is not claimed; targeted original/native checks prove
those fields separately. Final Unit menu image is inspected. No new input,
direct RAM/language fixture edit or extra locator was needed. Protected ROM/
states/inputs/e22dataset/cache/loader/canonical remain unchanged; new Linux/USA/
native Windows, wider GUI/archive/gameplay/mod gates remain pending. No
performance/startup/RSS/space campaign or AndroidPort change. Additional evidence:
windows-cli-summary.json, windows-scene-result.json,
windows-unit-items-full-state-audio.log and party-unit-items.png.

## Item-linked pair source contract

A standalone typed source catalog now owns ROM6e3ad0..6e4c64(exclusive):
225records ×20bytes =4500bytes. Twelve checked LDR literals root the table;
next table6e4c64is independently loaded at080d5e80through080d5e9cwith16byte
index arithmetic. All376item records select by byte1d, referencing220distinct
rows0..224; unreferenced rows214..217/219are retained. This source directly
adjoins the existing item table and overlaps no e23resource.

Each record contains a u8count0..9, opaque u8byte1and nine fixed pairs of
u8selector/u8value. All20bytes remain exact, including every slot beyond count.
The current header byte1and all unused pairs are zero; the model preserves them
rather than treating them as absent. There are472active pairs, eight with
selectorff. Semantic names beyond the original byte contracts remain unproved.
Links::validate_items requires every parsed item to have a table owner; raw
for_item uses the original u16item-ID truncation. The source-only phase kept the managed dataset at e23; e24integration is
described below.

Original item-chain080d0cb2uses property18and20byte arithmetic. Bounded query
080d0cc2reads count, ignores selector0, compares a nonzero pair selector with
live unit byte7and pair value withr5, and stops on the first match. It does not
read slots beyond count. Original normal publication prefix080d09ccORs80into
live unit byte40+value before its downstream call. A context mismatch with
selectorffinstead follows080d09f0and stores the value at live unit byte3d.
These are original bounded prefixes, not a reconstructed full job/ability
system; later calls, the52byte table at6e5464and live unit state remain separate.

One focused Wine source check passes378original item selectors(includingu16
truncation),1394queries covering all225rows,464normal and8isolatedffpublication
prefixes.2244cases/61906instructions/66PCs, independent full4500byte roundtrip,
source/JSON/count/extent/owner/coherent opaque/unused/active/reorder guards and
live-unit/outer-RAM/stack guards pass. Four isolated in-memory queries prove
ignored zero selector, duplicate first match and nonzero unused slots without
publishing/authenticating synthetic data. A separate eight-case Wine check
covers the original selector load/context mismatch/ffbranch/unit store from
080d09c4with live unit selector2;80instructions, without repeating the main
source oracle. Both Windows test builds are warning-free. No performance
measurement, new Linux/USA/native Windows or ordinary pair-table use is claimed.

Private evidence under ignored target/rom-data-item-links: links-inventory.json,
link-records-independent.json, inventory-links.py, candidate-literal-readers.json,
item-detail-072158.txt, candidate-reader-*.txt, secondary-reader-*.txt,
windows-original-result.json, windows-original-europe/{item-links.json,result.json,
sentinel-gate-result.json}, windows-sentinel-gate-result.json and Windows logs.
Source-only and ordinary earlier-item proofs remain closed; the subsequent
e24integration below does not repeat those oracles. Wider gameplay/maps/audio/
GUI/archive/mod work and new Linux/USA verification remain pending.

## Managed and direct item-link consumption

EUR e24-d23appends item-links.json to the existing source-authenticated set,
using fixed filenames and an explicit gameplay.item-records dependency. Every
17previous decoded file remains byte-identical; e23/older generations remain
retained. Strict file parsing and source/whole-manifest equality precede
transactional attachment; owner validation uses the actual parsed item resource.
A2MiB file-read guard bounds malformed inputs, not extracted-space acceptance.

Typed count/selector/value/opaque fields project every source byte for original
CPU/DMA/native/guest-BIOS reads, including slots beyond count. Live unit byte7,
normal bit7flags, FFbyte3dpublication, later job/ability calls and original
branch/copy/clock work remain active. Immutable session ownership survives
Load/Undo/backup/restart with dataset files unavailable. Nonserialized counters
distinguish count/selector/value/opaque consumption; save/native ABI remains.

Two focused EUR Wine checks pass publication interruption/create/reuse/repair/
move/e23retention, all17old file equality, coherent count/opaque/unused/reorder/
source/dependency/path/role transactional guards and unavailable-copy lifecycle.
62sampled actual selector/query/publication paths with all4500new raw bytes
altered match complete interpreter/native states/clocks. Native compiled reads
consume36count/114selector/56value bytes, covering all eight observed count
shapes and actual FFmismatch gates. Original bounded paths do not read opaque
byte1; all source bytes including that field are separately bus-proved.
Selector fixtures use a separate native table excluding theirD0CC2stop, which
is also a query entry; no chained overshoot or source-oracle repeat is needed.

All4500bytes/edges/widths/mirrors/real-BIOS bus/CpuSet/FastSet and4typed DMA
bytes pass. A narrow word crossing the adjacent old/new source boundary with
both raw spans altered proves2typed old item bytes+2typed new link bytes.
Windows test/production builds are warning-free, canonical executable restored,
protected e23data/originals/cache/loader intact. New integration evidence in
ignored target/rom-data-item-links includes windows-managed-results.json,
windows-managed-europe/{managed-result.json,native-result.json} and Windows
build logs. Source-only final evidence remains source-final-summary.json and
source-final-source-hashes.json.

Actual Wine CLI creates/reuses e24without dataset writes from unrelated cwd,
using the existing native cache. The first400-frame ordinary Equipment slot
entry reads0new link bytes; it is retained as negative evidence without a full
pair/repeat. Its image shows five equipped slots. A justified normal Down/A
extension opens an equipment inventory, and the560-frame final image explicitly
shows headgear(Wizard Hat/Cross Helm/Diamond Helm/Green Beret/Feather Cap).
Private artifact names retain weapon-list as a locator; no weapon-list scene
is claimed from that filename.

The560-frame CLI reads574new source bytes(287count/205selector/82value,0opaque).
One560-frame ordinary Wine pair matches every complete state/clock/central-wide
image/OAM/dispatch and mixed/PSG/FIFO/HLE audio sample with identical positive
count/selector/value use. No source/native/lifecycle proof is repeated, and no
RAM fixture edit is used. Protected original ROM/states/inputs/e23data/cache/
loader/canonical remain intact. Additional evidence: windows-cli-summary.json,
windows-cli-weapon-list.json, windows-scene-result.json,
windows-weapon-list-item-links-full-state-audio.log and weapon-list.png.
New Linux/USA/native Windows/wider gameplay/GUI/archive/mod gates stay pending;
no performance/startup/RSS/space campaign or AndroidPort change is included.


## Sixteen-byte linked metadata source contract

Standalone `metadata16::Catalog` preserves128records/2048bytes at EUR
6e4c64..6e5464: a raw u16alias and fourteen u8bytes02..0f. Twenty-three
checked original LDRreferences and sixteen-byte indexing identify the root;
CBEDA/CBEF0independently roots the following52byte table and fixes the end.
Independent field reconstruction roundtrips every byte without e24overlap.
Strict JSON rejects unknown fields, overflow, wrong array/count/extent and
coherent source edits. No law/mission/ability name is inferred from numeric
patterns. At source-only closure this standalone model added no managed file or resource;
EUR remained e24-d23/19files/18resources and USA e8-d7. Subsequent e25direct
consumption is documented below.

Rows1..127have alias375+row. Original39070loads that halfword and uses
(base-375*16)+alias*16to read byte2; alias0would point before this table, so
its caller precondition is unproved and no zero fallback is invented.
Original161e4reads byte2directly; both bounded paths produce display index0
for zero and394+value otherwise. This is a display helper index, not a proved
text/string ID. The opaque bytes and record0remain intact.

Four original readers use live linked records: head020028c8points to nodes
with record pointer+0/next+8; identity is byte0|((byte1&3)<<8). Record bytes4/5
select two metadata rows, with zero meaning skip. D5E20truncates input IDu16,
resolves the list and sums unsigned metadata byte0d, returning0on missing/null.
D5EA0does the same lookup for byte0e: zero yields the other value, then1yields
the other value; when both exceed1it calls the original02868LCG, updates live
IWRAM030034b0and selects using the low bit of the returned value. The source
proof checks the actual RNGstate transition, not a frozen choice.

D6704/D67D4instead read activeu16ID02003c28and return0when inactive. For a
nonzero IDthey require an existing linked record; unlike the first two readers,
no missing-record fallback is assumed. D6704truncates the selectoru8and sums
bytes09..0cfor selectors10..13, wrappingu8after each addition. D67D4tests raw
byte07using original register-ASR(value,selector-1)&1and ORs the two results;
zero/large selector shift behavior stays exact. All live records/indices/list
links and RNGremain guest state/code. The preceding D5DC0function has an
unproved r7base and is excluded from this table's contract.

One reduced Wine original-code check passes2581real-source cases and19isolated
synthetic cases:128direct display paths,127nonzero alias paths, every row in
pair sums/choice/flags, first/second list matches, first-two-reader missing/null,
inactive IDs, input truncation and selector edges. Shipped byte0ehas no value1;
isolated machine-owned fixtures prove that priority branch plus unsigned sum
wrap and flag shifts. Synthetic records fail authentication and are never
published. Complete live-list/callee/outer-RAM/stack guards and strict source/
JSON/extent/coherent-edit checks pass.156038instructions/319distinct PCs are
coverage counts only. Initial Windows test compilation omitted the Bus trait
import; retained diagnostics all report that fixture issue. Corrected build16512
is warning-free; Wine16242passes the new test only. All27protected ROM/e24file/
cache/loader/canonical/candidate/fixture hashes remain unchanged.

Evidence under ignored target/rom-data-metadata16: metadata16-inventory.json,
metadata16-records-independent.json, inventory-metadata16.py, reader-*.txt,
windows-{initial-failed-tests-build,tests-build}.{jsonl,log}, windows-original.log,
windows-original-result.json and windows-original-europe/{metadata16.json,
result.json}. Managed preparation, direct CPU/DMA/native projection and lifecycle were
subsequently completed below; ordinary scene/CLI remains a separate criterion.
New Linux/USA/native Windows checks remain pending; no performance or space
campaign, no AndroidPort change and no old source/scene oracle repeated.


## Managed and direct metadata16 consumption

EUR e25-d24adds fixed linked-metadata16.json/resource gameplay.linked-metadata16,
format linked-metadata16-v1-json with the base-dimensions dependency. All20files
are authenticated before transactional activation;18prior decoded files and
e24generations remain unchanged. A2MiB read guard bounds malformed input,
not a space optimization budget. Typed u16alias/raw bytes serve CPU/DMA/native
reads and merge prior owners on crossing accesses. Nonserialized counters
separate alias/display/flag/value/opaque source bytes. Original routines keep
resolving live list nodes/IDs/indices and executing sums, shifts, branches and
LCGupdates; no reader is replaced by a host shortcut. Save/native ABI is intact.

Warning-free Windows test build23604and two focused Wine80528checks pass
publication interruption/create/reuse/repair/move/e24retention, coherent alias/
opaque/value/reorder/source/dependency/path/role transactional guards, and
unavailable-copy Load/Undo/backup/restart.42sampled original/interpreter/native
paths run with all2048new raw ROMbytes altered and match complete states and
clocks:2133instructions/76dispatches as coverage,8compiled alias/17display/
22flag/24value bytes. Opaque fields are not read by these sampled original paths
and are separately bus-proved. All source bytes, record/source edges, widths,
ROMmirrors, real-BIOS bus reads, CpuSet/CpuFastSet and4typed DMAbytes pass.
A narrow crossing word with old-link and new-metadata raw spans both altered
proves2typed bytes from each owner. Source2581+19oracle stays closed.

Evidence:target/rom-data-metadata16/{windows-managed-results.json,
windows-managed-europe/{managed-result.json,native-result.json},
windows-managed-tests-build.{jsonl,log}}. Ordinary scene/actual CLI acceptance
was subsequently completed below. No old acceptance/performance/space/Linux/
USA/native Windows/Android campaign was run.


## Ordinary metadata16 use and CLI

Warning-free Windows production build77128restores the original canonical
executable. Actual Wine CLI15733creates/reuses e25from an unrelated directory
with the old native cache. Its first replay fails before execution because the
new private input lacked the required gba-input v1header; the failed input/log
remain retained. Corrected replay15780selects Area List and reads0new bytes.
Start-only99771and Party→Start11378inspect actual menu contexts and also read0;
Party inventory-tab cycle24270covers ordinary equipment/consumables and reads0.
These negative locators have no full pair and are not repeated.

The already retained Clan image visibly identifies Mission Item at row3.
Extending that input with three Down/Aselects the distinct new scene: CLI42237
passes520frames with6new display-field bytes and an inspected final Chocobo Egg
entry(1/64). One520-frame Wine pair72249matches every complete state/clock,
central/wide framebuffer, OAM, dispatch and mixed/PSG/FIFO/HLE audio stream,
with the same6positive display bytes. Ordinary alias/flag/value/opaque use is
not claimed; source/native/bus proofs above cover those fields. The observed
screen identifies Mission Item usage; the catalog keeps structural field names.

Initial CLIharness aborted before its final no-write assertion, so a narrowly
justified prepare-reuse-only invocation59534completes that uncovered criterion:
all22managed file hashes/timestamps and the file set remain unchanged. Functional
prepare/reuse and the positive scene were not repeated. No direct RAM edit or
new recording was used. Source/managed/native/lifecycle acceptance remains closed.
Evidence:target/rom-data-metadata16/{windows-build.json,windows-cli-initial-summary.json,
windows-cli-{menu-down2,start-menu,party-start,inventory-tabs,clan-mission-item}.json,
windows-cli-no-write.json,windows-scene-result.json,
windows-clan-mission-item-metadata16-full-state-audio.log,clan-mission-item.png}.
New Linux/USA/native Windows, broader gameplay/maps/audio/GUI/archive/mod work
remain pending. No performance/space campaign or AndroidPort change was made.


## Fifty-two-byte linked metadata source contract

The standalone `metadata52::Catalog` owns EUR 6e5464..6e6bf4: 116 records of
52 bytes, 6032 source bytes. Eight checked LDR literals root the table, the
original CCE28 enumerator visits 0..115 (CCE82 compares 0x73), and two independent
readers root the following four-byte table at 6e6bf4. All bytes roundtrip from
five prefix bytes, an explicit routing byte, 26 middle bytes and 20 tail bytes;
there is no overlap with e25 resources. The structural name avoids assigning
unproved meanings to packed fields. At source-only closure this catalog added no managed file/resource: EUR
remained e25-d24 with 20 files / 19 resources, USA e8-d7. The subsequent
e26 managed/direct integration follows below.

Getter CBEC8 truncates the row, alternate row and property inputs to u8 and
dispatches 48 properties through checked code targets at CBEF8. Property 2
returns raw byte05 immediately. Other properties read their field when byte05
is zero, redirect to the caller's alternate row when it is FF, and redirect to
the named row for any other nonzero value. The shipped table has 112 zero-route
records and four FF records (80, 81, 82, 85), with no other nonzero routes.
Alternate indices remain caller inputs. Properties at least 48 busy-loop at
CBEE2; FF and ordinary redirect cycles can also remain in the getter. The
standalone comparison helper returns None for a nonreturning/out-of-contract
path; it is never an emulated fallback and is not called by the runtime.

The property map preserves unsigned assembled 16-bit fields, bytes, nibbles,
three-bit fields crossing byte boundaries and four 12-bit fields. Property order
is not source order. Unknown and unexposed bytes stay exact. Original 1344B4
uses live record byte05 as an index and reads raw offset33; 137DB4 with its
second argument truncated to u16 zero reads raw offset32 instead. Its nonzero
argument path addresses another table and is outside this bounded proof. The
other surrounding live record/ability/unit logic is not frozen into ROM data.

The literal 6e6850 at 36976 is a discovery false lead for ownership: its original
path uses a 14-byte stride and an index at least 128, giving minimum actual
address 6e6f50. It is not an internal metadata52 owner or an earlier boundary.
The retained counted-pair source independently has 464 normal selectors in
2..43, all within the 116-record owner, plus eight FF selectors with the separate
original publication contract. This does not prove all surrounding FF caller
preconditions. Parsed-owner validation is implemented in the subsequent integration below.

One focused Wine source oracle passes 6369 returning real-source cases: all
116 records and 48 properties with valid alternate inputs, multiple alternates
for each FF row, u8 truncation and the two raw live-index tail readers. Four
bounded original probes retain invalid-property and FF-cycle nonreturning
behavior. Because ordinary nonzero routes are absent, 294 isolated synthetic
calls prove zero/high/mixed packed bits, a two-step ordinary redirect chain,
an FF-to-chain path and raw tail bytes; two further bounded probes retain
ordinary/FF-to-ordinary cycles. Synthetic records fail authentication and are
never published. Full source/JSON/fixed-array/count/extent/coherent-edit and
callee/stack/live-record/outer-RAM guards pass. The 223421 instructions and
524 distinct PCs are coverage counts only. Windows build21334 is warning-free;
Wine20105 runs only this new source test. All 28 protected ROM/e25 files/cache/
loader/canonical/candidate/fixture hashes remain unchanged.

Evidence remains ignored under target/rom-data-metadata52: candidate-roots.json,
metadata52-inventory.json, metadata52-records-independent.json, reader-*.txt,
item-link-cross-reference.json, windows-tests-build.{jsonl,log},
windows-original-result.json, windows-original-europe/{metadata52.json,result.json}.
Managed preparation, direct CPU/DMA/native ownership and lifecycle were
subsequently completed below; ordinary scene/CLI is an independent criterion. New Linux/USA/native Windows
checks remain pending; no old source/integration/scene proof or performance/
space campaign was repeated and AndroidPort is unchanged.


## Managed and direct metadata52 consumption

EUR e26-d25 adds linked-metadata52.json / gameplay.linked-metadata52, format
linked-metadata52-v1-json, with an explicit item-link dependency. Attachment
validates the actual parsed counted-pair owner: active non-FF selectors must
name one of 116 rows, while FF retains its separate original contract. All 21
files authenticate before activation; 19 previous decoded files and e25
immutable generations remain unchanged. A 2 MiB read guard bounds malformed
inputs rather than setting a space optimization target. Exact typed byte views
merge prior metadata16 ownership across source boundaries. Nonserialized
counters separate header/routing/packed/tail/opaque bytes. No host property
shortcut replaces original field extraction, caller-alternate selection,
redirect loops, raw live-index tail readers or guest work/clock. Save/native
ABI and original ROM identity remain intact.

Warning-free Windows build26387 and Wine96091 pass two focused checks:
publication interruption/create/reuse/repair/move/e25 retention, coherent
routing/opaque/packed/reorder/source/dependency/path/role transactional guards,
normal-selector116 rejection with separate FF acceptance, and unavailable-copy
Load/Undo/backup/restart. Seventy-five sampled original/interpreter/native paths
cover all 48 properties, four FF rows with alternate inputs, truncation and raw
tails. With all 6032 new raw ROM bytes altered, complete states and clocks match;
2414 instructions / 169 dispatches are coverage only. Compiled reads consume
18 header / 83 routing / 68 packed / 8 tail bytes. Opaque bytes are not read by
these sampled original paths and are separately bus-proved. Every source byte,
record/source edges, widths, mirrors, real-BIOS bus, CpuSet/FastSet and 4 typed
DMA bytes pass. A narrow crossing word with both adjacent raw owners altered
proves 2 typed metadata16 bytes plus 2 typed metadata52 bytes. Closed source
6369+294 returning calls and bounded nonreturning probes were not repeated.

Evidence:target/rom-data-metadata52/{windows-managed-results.json,
windows-managed-europe/{managed-result.json,native-result.json},
windows-managed-tests-build.{jsonl,log}}. Positive ordinary scene/actual CLI
acceptance was subsequently completed below. No new Linux/USA/native Windows, performance/space
campaign or AndroidPort change was made.


## Ordinary metadata52 use and CLI

Warning-free Windows production build71119 restores the original canonical
executable. Actual Wine CLI96132 prepares e26, reuses it without changing file
hashes/timestamps, and runs from an unrelated cwd using the old native cache.
The retained worldmap→Party→Unit context gains two ordinary Down presses and
A to open Change Jobs. Its 540-frame replay consumes 664 new bytes: 74 header,
335 routing and 255 packed bytes. The final Marsh/Thief job-selection image is
inspected; ordinary tail/opaque use is not claimed.

One 540-frame Wine pair6205 matches every complete state/clock, central/wide
framebuffer, OAM, dispatch and mixed/PSG/FIFO/HLE audio stream with the same
664 typed bytes. This is a distinct job-change scene; older Unit/Equip and
Mission Item acceptance was not repeated. No source/native/lifecycle oracle,
negative locator, failed harness or test was repeated in this integration.
No direct RAM edit or new recording was used. Read-only fixture/ROM/cache/
loader/canonical/candidate hashes remain protected.

Evidence:target/rom-data-metadata52/{windows-build.json,
windows-cli-{processes,no-write,summary}.json,windows-scene-result.json,
windows-change-jobs-metadata52-full-state-audio.log,change-jobs.png}.
New Linux/USA/native Windows and broader gameplay/maps/audio/GUI/archive/mod
work remain pending. No performance/space campaign or AndroidPort change.


## Four-byte two-pair source contract

Standalone `metadata4::Catalog` owns EUR 6e6bf4..6e6c44: 20 records of four
bytes, 80 source bytes. Two checked LDR literals root this table, and two
independent readers root the following two-byte map at 6e6c44. Each row preserves
two selector/value byte pairs. All 116 metadata52 records' raw offset30 indices
cover the complete range 0..19; all 40 new selectors name metadata52 rows.
There are 26 nonzero selectors, in 2..42, with values 1..5. Zero selectors and
unused payload bytes remain exact. There is no overlap with e26 resources.
At source-only closure this catalog added no managed file/resource: EUR
remained e26-d25 with 21 files / 20 resources, USA e8-d7. Subsequent e27direct
consumption follows below.

Both original consumers count nonzero selectors across both slots: CC424→CC44E
stores the count on the shared stack; 642FE→64330 keeps it in r10. Walking is a
different contract: CC474 and 64350 start only when the first selector is nonzero,
then stop at the first zero or after two pairs. An isolated hole can therefore
have count1 but no leading pair; it must not be normalized into a compact list.
The new model exposes both `active_count` and `leading_count` for comparison.
Neither is a replacement for original runtime instructions.

Selector prefixes CC41A and 642F4 call original metadata52 property39 and then
index the four-byte table. Their actual outer loops enumerate job IDs 2..43
with the same alternate index; FF rows 80/81/82/85 are not included in that
caller contract. The original threshold prefixes CC4EC and 643C2 compare a
live qualified-count register as signed against an unsigned value byte. Passing
increments the success tally with u8 wrapping, advances by two bytes and either
enters the next pair or exits. Deeper scans of live unit ability/progress fields
remain original code; the bounded oracle does not freeze those scans or claim
whole surrounding functions/global caller preconditions.

One focused Wine source check passes 320 real cases across both versions of
Count/Select/Gate/Threshold: all 20 rows, actual caller job IDs 2..43 and counts
below/equal/above each active value. Twenty-four isolated synthetic cases prove
holes, nonzero unused payload, unsigned value255, the signed comparison edge
and success-tally u8 wrapping. Synthetic records fail authentication and are
never published. Shared-stack, preserved prefix-register and outer-RAM guards
pass, as do strict JSON/arrays/count/extent/coherent-source and parsed metadata52
owner checks. These are bounded mid-function prefixes, not whole-function ABI
proofs. The 9560 instructions / 136 distinct PCs are coverage only. Windows
build21229 is warning-free and Wine74773 runs only this new test; no old source,
native/lifecycle or ordinary-scene acceptance is repeated.

Evidence remains ignored under target/rom-data-metadata4: candidate-roots.json,
reader-*.txt, inventory-metadata4.py, metadata4-inventory.json,
metadata4-records-independent.json, windows-tests-build.{jsonl,log},
windows-original-result.json and windows-original-europe/{metadata4.json,result.json}.
Managed preparation, direct CPU/DMA/native ownership and lifecycle were
subsequently completed below; ordinary scene/CLI is an independent criterion. New Linux/USA/native Windows checks stay
pending; no performance/space campaign or AndroidPort change was made.


## Managed and direct metadata4 consumption

EUR e27-d26 adds linked-metadata4.json / gameplay.linked-metadata4, format
linked-metadata4-v1-json, with an explicit metadata52 dependency. Actual parsed
metadata52 raw offset30 indices must name the new rows; all new selector bytes
must name metadata52 rows. All 22 files authenticate before transactional
activation; 20 previous decoded files and e26 immutable generations remain
unchanged. A 2 MiB read guard bounds malformed input rather than a space target.
Typed selector/value bytes preserve both slots and unused values and merge
prior metadata52 ownership on crossings. Counters are not serialized. The
runtime does not replace original count/leading-walk or qualification logic,
signed comparisons, unsigned loads, tally wrapping or guest work/clock. Deeper
ability scans keep running with live inputs; save/native ABI remains intact.

Warning-free Windows build42571 and Wine33626 pass two focused checks:
publication interruption/create/reuse/repair/move/e26 retention, coherent
selector/unused value/reorder/source/dependency/path/role transactional guards,
parsed cross-owner bounds, and unavailable-copy Load/Undo/backup/restart.
Fifty-six sampled original/interpreter/native count/selection/gate/threshold
prefixes cover count shapes0/1/2, values1/2/3/5, second-slot checks, signed
comparison and tally wrapping. With all 80 new raw ROM bytes altered, complete
states and clocks match; 1307 instructions / 81 dispatches are coverage only.
Compiled reads consume 54 selector / 34 value bytes. Every byte, edges, widths,
mirrors, real-BIOS bus, CpuSet/FastSet and 4 typed DMA bytes pass. A narrow
crossing word with both raw owners altered proves 2 typed metadata52 bytes
plus 2 metadata4 bytes. These remain bounded prefix proofs, not whole-function
ABI proofs; source320+24 acceptance was not repeated.

Evidence:target/rom-data-metadata4/{windows-managed-results.json,
windows-managed-europe/{managed-result.json,native-result.json},
windows-managed-tests-build.{jsonl,log}}. Positive ordinary scene/actual CLI
acceptance was subsequently completed below. New Linux/USA/native Windows, performance/space
campaign and AndroidPort work remain outside this integration.


## Ordinary metadata4 use and CLI

Warning-free Windows production build59395 restores the original canonical
executable. Actual Wine CLI36972 prepares e27, reuses it without modifying file
hashes/timestamps, then replays from an unrelated cwd using the old native cache.
The retained ordinary 540-frame Change Jobs input is used for the newly migrated
80-byte family, with its own positive-use gate; prior e26 metadata52 acceptance
is not reopened. The replay reads 38 new bytes: 32 selector and 6 value bytes.
The final Marsh/Thief job-selection image is inspected.

One 540-frame Wine pair48436 matches every complete state/clock, central/wide
framebuffer, OAM, dispatch and mixed/PSG/FIFO/HLE audio stream with the same
38 typed bytes. Deeper original live ability scans keep executing in that scene;
targeted tests remain bounded prefix proofs. No direct RAM edit, new recording,
old source/native/lifecycle oracle or failed locator was repeated. The private
generator's multiline-template mismatch was corrected before any build; all
integration builds and runtime checks pass.

Evidence:target/rom-data-metadata4/{windows-build.json,
windows-cli-{processes,no-write,summary}.json,windows-scene-result.json,
windows-change-jobs-metadata4-full-state-audio.log,change-jobs.png}.
New Linux/USA/native Windows and broader gameplay/maps/audio/GUI/archive/mod
work remain pending. No performance/space campaign or AndroidPort change.


## Two-byte forward and inverse source contract

Standalone `metadata2::Catalog` owns EUR 6e6c44..6e6c9c: 44 records of two
bytes, 88 source bytes. Checked LDRs CCD4C/CCD5E root it; independent literal
127E46 roots the following 48-byte copy source at 6e6c9c. The last pair
[43,19] is a real record, rather than padding. All stored keys currently equal
their indices and fit the 116 parsed metadata52 owners; both bytes remain
explicit fields. Semantic names for the values remain unproved.

Original CCD48 truncates the index to u8 and reads byte1 without a local bound.
The owned-source proof covers indices 0..43, including row43. `forward` returning
None outside this extent is a diagnostic, never an emulated fallback. Original
CCD58 truncates the query to u8, scans only rows 0..42, returns the first
matching stored key byte and returns zero on a miss. Duplicate values and this
unequal extent must not be normalized. The live wrapper CBD74 loads byte7 from
a live record and calls the direct getter; its bounded proof stops at CBD7C
with the caller frame retained. Four direct callers and one inverse caller are
checked statically; broader caller bounds and whole functions remain unclaimed.

Warning-free Windows build99948 and one Wine56417 source oracle pass 348 real
cases: all44 direct/live indices, all256 inverse values and truncation cases.
Thirteen isolated synthetic cases prove high bytes, duplicate first-match
stored keys and a last-row-only value readable forward but excluded inversely.
Synthetic records fail authentication and parsed-owner validation and are never
published. Exact results, inverse cursor, callee registers, stack/live-record/
outer-RAM guards, strict JSON, extent and coherent-source rejection pass.
75352 instructions / 27 PCs are coverage only. All31 protected files remain
unchanged; no old source/native/lifecycle/scene proof was repeated and no
build/test failure occurred.

Evidence:target/rom-data-metadata2/{metadata2-inventory.json,
metadata2-records-independent.json,reader-*.txt,caller-*.txt,
windows-tests-build.{jsonl,log},windows-original-result.json,
windows-original-europe/{metadata2.json,result.json}}. Source-only closure
leaves EUR e27-d26/22files/21resources and USA e8-d7 unchanged. Managed
publication, typed runtime ownership, lifecycle and ordinary scene/CLI were
subsequently completed below. New Linux/USA/native Windows and broader TODO remain pending; no
performance/space campaign or AndroidPort change.


## Managed and direct metadata2 consumption

EUR e28-d27 adds linked-metadata2.json / gameplay.linked-metadata2, format
linked-metadata2-v1-json, with metadata52 dependency and actual parsed stored-key
owner validation. All23files/22resources authenticate before transactional
activation; all21previous decoded files and e27generations remain unchanged.
A2MiB read guard bounds malformed input rather than a space target. Typed key/
value bytes merge earlier ownership at crossings, while original forward44/
inverse43 instructions, duplicate/first stored-key/miss0/u8truncation and live
byte7caller work keep executing. No new guest bound/fallback, instruction skip,
clock or save/native ABI change is introduced; counters are not serialized.

Initial Windows95465compile caught three existing budget-test calls missing the
new zero argument; corrected without changing their behavior, failed logs
retained. Corrected Windows91568build is warning-free; Wine80052two focused
checks pass publication interruption/create/reuse/repair/move/e27retention,
coherent key/value/lastrow/reorder/source/dependency/path/role transactional
rejection, parsed metadata52key bounds and unavailable-copy Load/Undo/backup/
restart.21sampled original/interpreter/native direct/inverse/live-byte paths
with all88raw ROM bytes altered match complete states/clocks;1705instructions/
28native dispatches are coverage only. Compiled reads use7key/235value bytes.
Allbytes/edges/widths/mirrors/real-BIOS bus/CpuSet/FastSet/DMA4B and a narrow
crossing with2oldmetadata4/2newmetadata2typed bytes pass. The source348+13oracle
stays closed; whole surrounding functions/global caller bounds remain unproved.

Evidence:target/rom-data-metadata2/{windows-managed-results.json,
windows-managed-europe/{managed-result.json,native-result.json},
windows-managed-tests-build.{jsonl,log},windows-initial-failed-managed-tests-build.*}.
New Linux/USA/native Windows and broader TODO remain pending; no performance/
space campaign or AndroidPort change.

## Ordinary metadata2 mission use and CLI

Warning-free Windows production98775build restores the canonical executable.
Actual CLI16539prepares/reuses e28without writes from an unrelated cwd with the
old native cache. Its retained540frame Change Jobs input reads0new bytes and
is coverage-negative. Pub560navigation instead enters Rumors; bounded230/290/
365prefixes and B-return480observations identify the branch, with the native
separated Pub root absent from PPM. The existing measured Pub contract identifies
Rumors/Missions/Quit Mission/Leave order. Corrected Down210/A300reaches the
Missions LIST at620; A380is too early for that list. These locator results are
preserved, with no full differential or unchanged locator rerun.

After the stable list, three Down presses and A860/A1060enter the selected
mission path; its1260frame probe still reads0bytes. Additional ordinary
A1300/A1600confirmations yield one1840frame Pub mission-requirement replay
with4new typed value bytes and0key bytes. Actual CLI24738and one full Wine
pair2490match every complete state/clock, central/wide framebuffer, OAM,
dispatch and mixed/PSG/FIFO/HLE audio stream, with the same4new bytes. Inverse
key reads are proved by focused native checks, not claimed in this scene.
The final Pub background image is inspected; separated panels, selected unit
and complete mission outcome are not inferred from that image. No direct RAM
edit, new recording or persisted save write: runc never writes its preload.
Retained Pub snapshot/input reuse is justified by the new88Bfamily and separate
positive-use gate; old source/native/lifecycle/scene acceptance is not reopened.

Evidence:target/rom-data-metadata2/{windows-build.json,windows-cli-{processes,
no-write,summary,pub-mission-details,pub-menu-prefix,pub-options-prefix,
pub-selection-prefix,pub-return-options,pub-missions,pub-dispatch-mission,
pub-dispatch-confirm}.json,windows-scene-result.json,
windows-pub-mission-metadata2-full-state-audio.log,pub-dispatch-confirm.png}.
The CLI managed25file set/hashes/timestamps and protected original inputs,
e27dataset/candidates/cache/loader/canonical remain unchanged. Source and new
native/lifecycle checks are not repeated for the ordinary scene. New Linux/USA/
native Windows and broader gameplay/maps/audio/GUI/archive/mod remain pending;
no performance/space campaign or AndroidPort change.


## Default constructor template source contract

Standalone `default48::Catalog` owns EUR6e6c9c..6e6ccc, exactly48bytes from
checked127E46→127EDCliteral and original copy length48. It stores byte00/01,
bytes02..04, argument bytes05..07,17full halfwords08..28, flags2a and two full
halfwords2c..2e; semantic names outside the observed patches remain unproved.
The following240bytes resemble five48byte shapes, but have no independently
found root/consumer and are not owned or normalized into extra records.

Constructor127DC4truncates input bytes and copies only when mode is1or2;
other modes return0. IDs20..23dereference ROM70cd58→6eca78 and select four
separate48byte sources. Every other u8ID uses the default template. The common
patch writes byte00only when the signed ID is nonnegative, byte01only when the
selector exceeds1, and bytes05..07from call arguments. Flag1/2/3ORs8000/2000/
1000into flags2a, preserving preexisting bits; other flags leave it intact.
The new `patched` helper is diagnostic; it does not replace runtime guest work.
The proof stops at127EB0before allocation/live-unit lookup, retaining the caller
frame. Invalid-mode checks additionally prove actual return0/callee/SP restore.

Existing original initializer027EE..027FEexecutes10instructions and installs
ROMd96838..d968e8atIWRAM03005e78..03005f28. Its copy entry03005ee9is selected
through ROM370eb4and original BXr3trampoline145C14. No snapshot, new fixture or
old acceptance rerun is needed. Windows54305is warning-free and one new Wine
source oracle passes271real cases: all256IDs/default-vs-four-alternate sources,
mode1/2/rejection/truncation, signed-ID edge, selector0/1/2/255and flags0/1/2/3/
255.30isolated nonzero/high-bit synthetic cases prove untouched fields and OR
preservation; they fail authentication and are never published. All48source
bytes reconstruct independently; strict JSON/fixed arrays/source/coherent-edit,
scratch/frame/incoming-argument/invalid-callee/outer-RAM guards pass.48812guest
instructions/155PCs are coverage only. All32protected original/e28/candidate/
cache/loader/canonical/source fixtures remain unchanged; no build/test failure.

Evidence:target/rom-data-default48/{constructor-127d60.txt,
constructor-tail-127f00.txt,nearby-literal-candidates.json,
raw-pointer-candidates.json,inventory-default48.py,default48-inventory.json,
default48-fields-independent.json,windows-tests-build.{jsonl,log},
windows-original-result.json,windows-original-europe/{default48.json,result.json}}.
Source-only closure leaves EUR e28-d27/23files/22resources and USAe8-d7unchanged.
Managed publication/runtime ownership/lifecycle/ordinary scene/actualCLI for
this template were subsequently completed below. Four alternate192bytes and their ROM pointer remain
unowned. New Linux/USA/native Windows/broader TODO remain pending; no performance/
space campaign or AndroidPort change.


## Managed and direct default constructor template

EUR e29-d28 adds default-constructor-template.json /
gameplay.default-constructor-template, format default-constructor-template-v1-json,
with no invented semantic owner dependency. All24files/23resources authenticate
before transactional activation; all22prior decoded files/e28generations remain
unchanged. A2MiB read guard bounds malformed input rather than a space target.
Typed header/argument/full-halfword/flag/tail bytes merge previous metadata2
ownership at crossings. Original ROM/IWRAM copy, signed-ID/selector/live argument/
ORflag patches, mode and alternate selection, allocation work, clock/save/native
ABI remain; counters are not serialized. The diagnostic patched helper is not
used to shortcut guest execution.

Initial Windows21352compile caught a stale two-element thread-local macro
counter alongside the new five-field counters; corrected, failed logs retained.
Corrected55456Windows build is warning-free. Wine5227two focused checks pass
publication interruption/create/reuse/repair/move/e28retention, coherent header/
opaque/flag/tail/source/spurious dependency/path/role transactional rejection and
unavailable-copy Load/Undo/backup/restart.16sampled original/interpreter/native
constructor prefixes include real copied IWRAM helper blocks and all48rawbytes
altered; complete states/clocks match. Eight alternate/rejected paths read0new
source bytes. Compiled header/argument/halfword/flag/tail use is39/21/272/16/32B;
2319instructions/1061dispatches are coverage only. Allbytes/edges/widths/mirrors/
real-BIOS bus/CpuSet/FastSet/DMA4B and a narrow crossing with2oldmetadata2/
2newdefault48typed bytes pass. Source271+30proof stays closed. Allocation/live
unit tail remains original, not a whole-function ABI proof; four alternate192B/
ROMpointer/following240B remain unowned.

Warning-free production83840build restores the canonical executable. Actual
WindowsCLI33995prepares/reuses e29without dataset writes from unrelated cwd and
replays the retained397frame Pub entry with old native cache. New-family use
is96B:10header/6argument/68halfword/4flag/8tail. One397frame Wine63615pair
matches every complete state/clock, central/wide framebuffer, OAM, dispatch and
mixed/PSG/FIFO/HLEaudio stream with those same96typed bytes. The final Pub image
is inspected. This retained scene/input is justified by the newly migrated48B
family and independent positive gate; older graphics/text/metadata2acceptance
is not reopened. No negative locator, RAM edit, new recording or persisted
save write was needed; no source/native/lifecycle oracle was repeated.

Evidence:target/rom-data-default48/{windows-managed-results.json,
windows-managed-europe/{managed-result.json,native-result.json},
windows-managed-tests-build.{jsonl,log},windows-initial-failed-managed-tests-build.*,
windows-build.json,windows-cli-{processes,no-write,summary}.json,
windows-scene-result.json,windows-pub-default48-full-state-audio.log,pub.png}.
Original inputs/e28data/candidates/cache/loader/canonical remain protected; CLI
26file-set/hash/mtime guards pass. New Linux/USA/native Windows and broader
maps/gameplay/audio/GUI/archive/mod remain pending; no performance/space campaign
or AndroidPort change.


## Next rooted constructor sources

Static inventory now separates four alternate constructor records at
6eca78..6ecb38/192B and their4Bpointer70cd58..70cd5c,196Btotal. Checked
127E1A→127E38loads header70cd54; LDR+4at127E28selects the base. Gatedu8IDs
20..23and original48byte indexing/copy bound the four records without owning
the neighboring header. All byte/halfword fields reconstruct independently,
with no e29overlap. Original CCF9C..CD046reads five equipment halfwords08..10
through item properties6/3; all20references fit actual parsed376item owners.
Byte01values104..107remain structural, with no new semantic-owner assertion.

Evidence:target/rom-data-constructor-alternates/{inventory-constructor-alternates.py,
constructor-alternates-inventory.json,constructor-alternates-records-independent.json,
template-consumer-0ccf9c.txt}. The private inventory's misplaced CD034LDRHassertion
was corrected toCD032before output/build; static checks pass. Source model/oracle were subsequently completed below; no managed resource or
runtime projection yet at this boundary. Current EUR e29-d28/
24files/23resources and USAe8-d7stay unchanged. Next is the reduced source-only
proof for these four records; closed default271+30/native16/397scene proofs
are not reopened. New Linux/USA/native Windows remain pending; no metrics.


## Typed constructor alternates source proof

Strict `constructor_alternates::{Record,Alternates,Catalog}` preserves all four
IDs20..23, their fixed source offsets and full byte/halfword fields, plus parsed
base pointer70cd58. Two disjoint ranges own192record bytes and4pointer bytes.
The first five halfwords of each record validate against actual parsed376item
owners. Source-offset/count/order/ID/fixed-array/JSON/coherent-edit guards retain
all196bytes. Byte01meaning and neighboring header ownership remain unclaimed.

Warning-free Windows39935and one Wine61563source oracle pass24real four-ID
copy/live-patch cases coveringu8truncation, selector0/1/2/255and flag/mode edges.
Eight isolated synthetic records retain high bits, unused fields, valid high
item IDs and preexisting ORflags; they fail source authentication and are never
published. Existing original initialization executes10instructions to install
the real IWRAMcopy helper; original constructor127DC4→127EB0keeps its scratch/
caller frame and live incoming arguments. Outer RAM guards pass.5560instructions/
137PCs are coverage only; allocation/live-unit tails remain unclaimed. Closed
default271+30/native16/397scene acceptance was not repeated. All32protected
original/e29/item-owner/candidate/cache/loader/canonical/source files stay intact;
no build/test failure. HEAD advanced externally toe180772during this unit,
committing prior e29content without changing this source scope; no agent commit.

Evidence:target/rom-data-constructor-alternates/{windows-tests-build.{jsonl,log},
windows-original-result.json,windows-original-europe/{constructor-alternates.json,
result.json}}. Managed preparation, direct typed pointer/record ownership,
lifecycle and positive ordinary scene/actualCLI were subsequently completed below; EUR e29-d28/
24files/23resources and USAe8-d7unchanged. New Linux/USA/native Windows and the
rest of the TODO remain pending; no performance/space campaign/AndroidPort change.


## Managed and direct constructor alternates

EUR e30-d29 adds constructor-alternate-templates.json /
gameplay.constructor-alternate-templates, format constructor-alternate-templates-v1-json,
with actual parsed Items dependency for all20equipment references. All25files/
24resources authenticate before transactional activation; all23previous decoded
files/e29generations remain unchanged. A2MiB read guard bounds malformed input,
not an extracted-space target. Typed4Bbase pointer and192Brecord bytes project
through pages6ec/70c, preserving ID/order/offsets and all header/argument/full-
halfword/flag/tail fields. Per-record masks/counters are nonserialized. Original
ROM/IWRAMcopy, pointer/index/mode selection, live patches, allocation work,
clock/save/native ABI remain; no guest routine or fallback is replaced.

Initial Windows89684build is warning-free; pre-Wine review corrects an inherited
zero-use guard direction for the new family and adds an old-default positive
coexistence check. Corrected Windows61725and production92748builds are warning-
free; no compiler/runtime failure. Wine39423two focused checks pass publication
interruption/create/reuse/repair/move/e29retention, coherent header/opaque/flags/
tail/byte01swap/pointer/source/dependency/path/both-source-role transactional
rejection, parsed item bounds and unavailable-copy Load/Undo/backup/restart.
14sampled original/interpreter/native paths include real IWRAMcopy blocks and
all196rawbytes altered during guest execution; complete states/clocks match.
Assertions that read raw records run only after execution stops and spans are
restored. All four records are positively typed(mask15); compiled pointer/
header/argument/halfword/flag/tail use is36/45/27/306/18/36B. Five default or
rejected paths read0newB; old default ownership remains positive.2151instructions/
979dispatches are coverage only. All196bytes/edges/record boundaries/widths/
mirrors/real-BIOS bus, both source spans in CpuSet/FastSet and DMA4B each pass.
Raw-header/pointer and final-record/following-source words each preserve2typed
and2rawbytes. Source24+8/default271+30/native16/397proofs stay closed.

## Ordinary alternate-record use and CLI

Production92748restores the canonical executable. Actual WindowsCLI52141creates/
reuses e30without dataset writes from an unrelated cwd and uses the old native
cache. The retained3000frame clean-boot input reaches the school courtyard but
reads0newbytes; it is a coverage-negative locator, with no full pair or timing.
Local text lookup for metadata52rows104/105yields Judge and106Judgemaster; this
is scene-discovery evidence without renaming the structural file fields.
Retained worldmap hover/A200plus A500/A900/A1300reaches a Lutia battle-map laws
NOTICE at1700, still0newbytes; this negative locator also remains closed.

Ordinary A1800/A2100/A2400/A2700and Start3000/A3300advance the NOTICE/deployment.
Actual CLI26234and one3600frame Wine68173pair pass with45newbytes:4pointer/
6header/3argument/28halfword/2flag/2tail, record mask1(ID20only). Every complete
state/clock, central/wide framebuffer, OAM, dispatch and mixed/PSG/FIFO/HLEaudio
sample matches. The final Lutia battle image is inspected. This ordinary proof
covers the observed direct pointer/field reads; it does not claim every48B or
the whole constructor runs in that scene. All four records/full-copy behavior
remain targeted-only. Reused input/snapshot are justified by the new196Bfamily
and separate positive gate; no direct RAM edit, new recording, persisted save
write or old source/native/lifecycle acceptance repeat. No full pair is run on
negative locators. Existing shared scene tooling supports both cleanboot and
snapshots, so no additional fixture or oracle feature is added.

Evidence:target/rom-data-constructor-alternates/{windows-managed-results.json,
windows-managed-europe/{managed-result.json,native-result.json},
windows-managed-tests-build.{jsonl,log},windows-managed-tests-pre-guard-review-build.*,
windows-build.json,windows-cli-{processes,no-write,summary,engage-battle,deploy-start}.json,
scene-class-label-locator.json,windows-scene-result.json,
windows-deploy-constructor-alternates-full-state-audio.log,deploy-start.png}.
Protected originals/e29/item-owner/candidates/cache/loader/canonical and CLI
27file-set/hash/mtime guards pass. HEAD advanced externally toe180772during the
source unit; unrelated user todo2.md additions remain untouched. New Linux/USA/
native Windows and broader maps/gameplay/audio/GUI/archive/mod remain pending;
no performance/space campaign or AndroidPort change.


## Next bounded fourteen-byte records

Static discovery separates physical6e6f6c..6e7482/1302Binto93fourteen-byte
records with logical IDs2..94. Original getters use biased base6e6f50, rooted
by CD05C→CD07Cand ten other selected literals. CBAD4routes nonnull liveunit+4>1
to these records; IDs0/1keep their original normal job/property branch. A separate
12Bindexed source at6e7484is independently rooted by13762E→137640; only whole
14Brecords before it are owned, leaving two zero bytes unowned. Global caller
upper bounds and constructorID0remain unproved; no fallback is invented.

Fields preserve bytes00..03, full halfwords04/06, packed08..0a, byte0b and bytes
0c/0d. CF144/180return halfwords4/6, CF1BCbyte11and CF1F0byte12foru8argument1or
byte13otherwise. CF08Cretains original packed palette bank/selector lookup/copy.
All186byte01/02values fit parsed116metadata52rows, but that range alone is not
semantic owner proof forbyte01. Original initializer copies byte02to liveunit+7
when descriptor override01is zero; this supports the byte02job link. All1302B
reconstruct independently without overlap with e30resources.

The earlier6e6dbcroot hypothesis was rejected: its reader uses4Bindexing,
and6e6ee4is also separate. Supposed14Baliases6e7484/6e78d4actually have12Bstride.
Evidence:target/rom-data-metadata14/{reader-*.txt,readers-*.txt,
predicate-0cbab0.txt,inventory-metadata14.py,metadata14-inventory.json,
metadata14-records-independent.json}. Model/source oracle were subsequently completed below; managed/runtime remain
next at this boundary, EUR e30-d29/25files/24resources and USAe8-d7unchanged. No new
build/test or closed acceptance repeat; newLinux/USA/nativeWindows and other
families remain pending, no metrics/space work.


## Typed fourteen-byte records source proof

Strict `metadata14::{Metadata14,Record,Catalog}` separates biased address6e6f50
from physical6e6f6c..6e7482, retaining93records/1302B/logicalIDs2..94and every
byte/full-halfword/packed field. Byte02validates against actual parsed116metadata52
job owners. Byte01has no proven semantic owner: a synthetic value250or255is
structurally accepted but fails source authentication. IDs0/1/95plus and the
2Btrailing gap are not owned; diagnostic recordNone never becomes a guest fallback.

Warning-free Windows18151and one Wine66692oracle pass504real cases:471getter
calls over all93IDs and u8argument truncation,22packed-palette cases across live
books0/1/2/default3and11initializer prefixes. Getter results preserve unsigned
u16/u8values. Palette tests observe the original source returned byCF5DC, verify
original32Bcopy/destination bank/512Bguards and keep the live book byte02002fc2.
No palette payload ownership is inferred. Initialization stops atCD0B6after the
original CCF9Cprefix, checking byte02fallback/descriptor overrides/live auxiliary
byte/flags/264Bunit contents and retained mid-function frame, not allocation tails.

13isolated synthetic cases preserve highu16/u8fields, bank15, unused bytes,
unprovedbyte01and a valid highjob selector. StrictJSON/fixedarray/bias/ID/order/
count/source/coherent-edit/byte02owner/callee/scratch/descriptor/unit/palette/
outer-RAM guards pass; synthetic records unauthenticated/unpublished.23627guest
instructions/380PCs and10original helper-init instructions are coverage only.
All33protected original/e30/metadata52owner/candidate/cache/loader/canonical/source
fixtures remain unchanged; no build/runtimefailure or old acceptance repeat.
HEAD2d66955advanced externally during this unit, committing prior e30/cleanup/
user-doc changes; inspected/preserved without agent commit.

Evidence:target/rom-data-metadata14/{windows-tests-build.{jsonl,log},
windows-original-result.json,windows-original-europe/{metadata14.json,result.json}}.
Managed publication/direct CPU-DMA-native ownership/lifecycle/positive ordinary
scene/actualCLI were subsequently completed below; this source-only boundary
retained EUR e30-d29/25files/24resources and USAe8-d7.
Normal live job/property branches, global caller bounds, neighboring families,
newLinux/USA/nativeWindows and the broader TODO remain pending. No performance/
space campaign or AndroidPort change.

## Managed and direct fourteen-byte special-unit metadata

EUR `v1/e31-d30` adds `linked-metadata14.json`, resource
`gameplay.linked-metadata14`, format `linked-metadata14-v1-json`. All 26 files /
25 resources authenticate together; all 24 earlier decoded files and e30
generations are retained. The declared dependency is the actual parsed
`gameplay.linked-metadata52` owner for byte02. Byte01 remains unconstrained by
that dependency. USA retains e8-d7. The 2 MiB file limit guards malformed input;
this change does not enforce a performance or extraction-space target.

Typed fields serve exactly 1,302 bytes at 6e6f6c..6e7482 through CPU, compiled
native, DMA and SWI reads on pages 6e6/6e7. Reads crossing a source boundary
merge typed and original bytes; IDs0/1, the following two-byte gap and higher
indices retain their existing sources. Nonserialized field counters and two
64-bit record masks distinguish positive use of these 93 records. Original
getter routing, live palette books and 32-byte copies, descriptor overrides,
initialization/allocation, guest clock and save/native ABI remain active.

The warning-free Windows test build and two Wine checks (75387) pass managed
publication interruption/create/reuse/repair/move, e30 retention, coherent
field/source/dependency/path/role edits, parsed byte02 owner validation and
transactional attachment. With the dataset copy made unavailable, Load/Undo,
backup restoration and restart retain the in-memory resources. Byte01=255 is
structurally valid but cannot authenticate as original data.

37 original/interpreter/compiled paths with all 1,302 raw source bytes altered
match complete states and clocks. IDs2/20/28/94 consume compiled byte/halfword/
packed/tail fields (13/12/35/9 bytes); two normal-ID0/1 paths consume no new
source bytes. All bytes, record-mask boundary, edges, widths, mirrors, real-BIOS
bus, CpuSet/FastSet, cross-record DMA and typed/raw padding crossings pass.
4,793 instructions and 2,502 dispatches describe coverage only. The source
504+13 oracle and earlier runtime acceptance remain closed and were not repeated.

Production build7465 is warning-free and restores the canonical executable.
ActualCLI9489 creates and reuses the dataset without file/hash/mtime changes,
then replays the retained 397-frame Pub transition with the old native cache
from an unrelated working directory. It reads 27 new bytes from two records:
4 byte fields, 8 halfword bytes, 12 packed bytes and 3 tail bytes. The final Pub
image is inspected. One Wine24398 pair matches every complete state/clock,
central and wide image, OAM, dispatch and mixed/PSG/FIFO/HLE audio stream with
the same positive source counters. No additional scene locator is needed.

Evidence: `target/rom-data-metadata14/` contains windows-managed-results.json,
windows-managed-europe/{managed-result.json,native-result.json}, windows-build.json,
windows-cli-{processes,no-write,summary}.json, windows-scene-result.json,
windows-pub-metadata14-full-state-audio.log and pub.png. Protected originals,
e30 data, source proof, snapshots, input, cache, loader and canonical executable
are retained. Global caller bounds, byte01 semantic meaning, palette payloads,
allocation tails, adjacent families and broader TODO coverage remain open.
New Linux/USA/native Windows checks are pending; no performance, space or
AndroidPort work is included.

## Indexed twelve-byte packed pair source contract

Two distinct indexed banks occupy 6e7484..6e78d4 (92 rows / 1,104 bytes) and
6e78d4..6e7994 (16 rows / 192 bytes). Original literals13762e→137640 and
137652→137664, both original multiply-by-12 paths and shared reader135a1c
establish the contract. The following independent root1377a2→1377b0 points to
6e7994 and is excluded. All 1,296 bytes reconstruct without e31 overlap.
These are separate from the earlier fourteen-byte special-unit source.

Each row stores 48 two-bit values. The strict `metadata12` model keeps twelve
fixed groups of four values0..3, IDs, offsets, bank order and exact92/16counts.
No semantic label or foreign owner is inferred. Existing caller loops visit
slots0..43; stored pairs44..47 retain every bit. Original code chooses the low
bit when the mode's low byte is zero and the high bit for any other low byte.
Rows use the full32-bit input and slots retain signed/wrapping shift arithmetic;
there is no local row/slot bound. A diagnostic None never supplies a guest
return value or changes original access outside the catalog.

One reduced Wine88437 source oracle passes 1,640 real calls over every row,
boundary slots and all48pairs in first/last rows, plus ten raw outside/wrapping
calls: next-bank/global-source reads, row256 distinct from row0, large row wrap,
negative slot and slot48. 384 isolated synthetic calls cover all pair values,
high bits and mode0/0x101/0xff/0x100. Every call observes the exact original
byte address/one LDRB and checks return value, callee, stack and outer RAM.
Strict JSON/fixed shapes/pair range/order/count/ID/source/coherent edits pass.
Synthetic sources fail authentication and remain unpublished. 75,515 guest
instructions / 54 PCs describe coverage only.

Initial Windows99149 compile exposed test setup field/type errors; these are
corrected and the22815build is warning-free. Wine has no failure. All34protected
ROM/e31/candidate/cache/loader/canonical/sourcefixture hashes remain unchanged;
no earlier source/runtime/scene proof was repeated. Evidence lives in
`target/rom-data-metadata12/`: metadata12-inventory.json, independent records,
bounded reader/caller disassembly, windows-initial-failed-tests-build.*,
windows-tests-build.*, windows-original-result.json and
windows-original-europe/{metadata12.json,result.json}.

At this source-only boundary managed EUR e31-d30 / 26 files / 25 resources and
USAe8-d7 remain unchanged. Managed publication, direct CPU/DMA/native ownership,
lifecycle and positive ordinary use were subsequently completed below. Global caller row bounds,
meaning of pair values, the separately rooted global bit source, newLinux/USA/
nativeWindows and wider gameplay/maps/audio/mod phases remain open. No
performance, space or AndroidPort work is included.

## Managed and direct twelve-byte packed pair banks

EUR `v1/e32-d31` publishes `linked-metadata12.json`, resource
`gameplay.linked-metadata12`, format `linked-metadata12-v1-json`. All27files /
26resources authenticate together. Both source roles retain exact1104/192-byte
bank extents; every earlier25decoded file and e31 generation stays unchanged.
The original semantic owner of these flags is unproved, so no foreign-key
dependency is invented. USA retains e8-d7. Parsing limits guard malformed input;
no extraction-space or performance target is measured.

Typed bytes from the 48two-bit fields in each row serve original CPU/native,
SWI and DMA reads on page6e7. Cross-bank words combine both banks; outside
bytes retain existing owners/ROM. Nonserialized bank counters and two64-bit
record masks gate the108records. Full32row, signed/wrapping slot, low8mode,
original routines, clock and save/native ABI remain active. Diagnostic None
never replaces a guest read or return value.

Warning-free Windows36670 and two Wine22271 checks pass publication failure,
create/reuse/repair/move/e31 retention, coherent pair and unused-tail edits,
source/dependency/path/both-role guards, transactional attachment and unavailable
copy Load/Undo/backup/restart.49original/interpreter/compiled paths with all
1296raw bytes altered match complete states/clocks. Four records are positive;
compiled primary/secondary reads are23/22bytes. Three outside paths use0new
bytes; full32, wrapping and next-bank accesses retain their original behavior.
All1296bytes/edges/mask boundaries/widths/mirrors/real-BIOS, both-bank CpuSet/
FastSet and cross-bank4typed/trailing-global2typed2raw DMA checks pass.
1828instructions/153dispatches describe coverage only. Only a comment and
injected-error wording were cleaned after compiling the test; assertions and
production behavior were unchanged. The source1640+10+384 and e31 acceptance
were not repeated.

Windows94316 production build is warning-free and restores the canonical
executable. ActualCLI prepares/reuses e32 without managed-file/hash/mtime changes,
then uses the existing native cache from an unrelated working directory.
The positive ordinary sample is 2200-frame Echo Grass confirmation: ROM_DATA_METADATA12_SCENE frames=2200 bus_reads=44 source_bytes=44 records_used=1 primary_bytes=44 secondary_bytes=0.
One Wine pair matches every complete state/clock, central-wide image, OAM,
dispatch and mixed/PSG/FIFO/HLE audio stream with the same counters; the final
image is inspected. Pub397, Lutia deployment3600 and the earlier3530enemy-movement replay are
coverage-negative and retained without full pairs. Bounded original action
callers guide menu navigation from a retained frame12006baseline snapshot,
avoiding3530prefix repeats. Its serialized separate-UI setting overrides the
pre-load locator environment; existing BG-layer diagnostics expose Steal and
Item menus without changing runtime code. EchoGrass1200reaches Do it/Cancel,
so it is a pre-effect locator. One extra physical A1300commits the action and
makes the2200sequence positive. No RAM/snapshot edit or new fixture is needed;
the old rejected ROM-seed experiment is not reopened when reusing its baseline.
Ordinary coverage is44primary-byte reads/one record/0secondary; the secondary
bank remains targeted-only.

Evidence: `target/rom-data-metadata12/` contains windows-managed-results.json,
windows-managed-europe/{managed-result.json,native-result.json}, windows-build.json,
windows-cli-*.json, windows-scene-result.json and the full-state/audio log.
Protected ROM/e31/sourceproof/fixtures/input/cache/loader/canonical/candidates
remain unchanged. Global caller bounds, flag semantics and the separately rooted
global bank remain unclaimed. NewLinux/USA/nativeWindows, other gameplay/maps/
audio/GUI/archive/mod coverage remain pending; no performance, space or
AndroidPort work is included.

## Global packed pair flags source proof

The original13779c wrapper roots6e7994 through1377a2→1377b0 and discards its
incoming r0: there is no row stride. Its only direct caller1377e0 belongs to
the1377b4 live-RAM mask loop over slots0..43, mode0. This proves the minimum
44two-bit fields / 11bytes at6e7994..6e799f. The independent next root6e79a2
(D2DB0→D2F20, original byte-pair reads) leaves three zero bytes unowned.
No12-byte shape, extra slots or padding ownership is inferred from adjacency.

`global_pair_flags::{GlobalPairFlags,Catalog}` keeps eleven fixed groups of
four values0..3, exact source offset and every stored bit. Diagnostic flags
accept slots0..43 and retain u8mode selection; original signed/wrapping slot
arithmetic and raw adjacent accesses remain guest code. High-mode direct
wrapper behavior is proved; ordinary callers' high-mode use is unclaimed.
The live-mask filter scans only set RAM bits, exits at the first false low
flag and accepts an empty mask. High live-mask tail bits beyond43 are ignored.

Warning-free Windows91155 and one Wine35647 oracle pass102real cases,10raw
outside/wrapping cases and91isolated synthetic cases. These cover all44fields/
bothbits, u8mode, ignoredr0, live empty/all/sparse/tail-only/single masks, source
padding/next-root/negative slot/wrap, high bits, full44scan and early rejection.
Exact source-load address sequences/return values, callee/stack/liveRAM guards,
strictJSON/fixedgroups/pairrange/offset/source/coherent edits pass. Synthetic
sources fail authentication and remain unpublished.13601instructions/70PCs
are coverage only; there is no build or Wine failure or old oracle repeat.

Evidence: `target/rom-data-global-pair-flags/` contains bounded reader/next-root
disassembly, global-pair-flags-inventory.json, independent fields,
windows-tests-build.*, windows-original-result.json and
windows-original-europe/{global-pair-flags.json,result.json}. Protected ROM/e32/
candidate/cache/loader/canonical/sourcefixtures remain unchanged. At this
source-only boundary EUR e32-d31 /27files/26resources and USAe8-d7 are unchanged.
Managed publication/direct CPU-DMA-native/lifecycle were subsequently completed
below; positive ordinary global use remains pending. Three adjacent bytes, semantic labels, other callers/conditions and new
Linux/USA/nativeWindows checks remain open; no performance/space/AndroidPort work.

## Managed and direct global packed pair flags

EUR `v1/e33-d32` adds `global-pair-flags.json`, resource
`gameplay.global-pair-flags`, format `global-pair-flags-v1-json`. All28files /
27resources authenticate together; all26earlier decoded files/e32 generations
remain unchanged. The exact11Bsource and44two-bit fields retain their single
root and original live-mask filter; no semantic dependency, row stride or
ownership of the following three bytes is invented. USA retains e8-d7.

Typed source bytes serve CPU/native/SWI/DMA reads on page6e7, merging prior
metadata12ownership and raw tail bytes. Nonserialized source counters/11-bit
byte-position mask distinguish new global use. Original ignoredr0, signed-
wrapping slot, low8mode, live RAM-mask/earlyexit, clock and save/nativeABI remain.
A byte mask observes source-byte positions, not which bit-pair a caller selected.

Warning-free Windows61276 and two Wine84782 checks pass publication interruption,
create/reuse/repair/move/e32retention, coherent pair/tail/sourceoffset/hash/
unexpecteddependency/path/role edits, transactional attachment and unavailable
copy Load/Undo/backup/restart.28original/interpreter/compiled paths with all11raw
bytes altered match complete states/clocks;17compiled byte reads/mask1059prove
new use. Eight outside paths use0new bytes. All11B/edges/widths/mirrors/real-BIOS,
prior2metadata12+2global typed crossing with both raw sides altered, CpuSet and
FastSet exact11typed bytes and DMA3typed1raw/1typed3raw pass.3076instructions /
231dispatches describe coverage only. Source102+10+91/e32proofs were not repeated.

Windows73677 production is warning-free and restores the canonical executable.
ActualCLI13165 creates/reuses e33 without managed-file/hash/mtime changes, using
the old native cache from an unrelated working directory. Its only new-global
ordinary locator reuses2200EchoGrass:0global bytes/mask0, while metadata12still
reads44bytes, confirming the action executes. This is coverage-negative; no
full pair is run on it. Bounded original131db8..131de8gates the global filter
behind nonzero descriptor halfword4 and liveunit+e8mask. A positive ordinary
scene and corresponding full-state/image/audio pair remain pending; further
selection needs concrete evidence of those conditions. High-mode ordinary
use also remains unclaimed.

Evidence: `target/rom-data-global-pair-flags/` contains windows-managed-results.json,
windows-managed-europe/{managed-result.json,native-result.json}, windows-build.json,
windows-cli-{processes,no-write,summary}.json and caller-131d8c.txt. Protected ROM,
e32/sourceproof/fixtures/input/cache/loader/canonical/candidates are retained.
Three adjacent bytes, semantics/other callers/conditions, newLinux/USA/nativeWindows
and wider graphics/text/maps/gameplay/audio/GUI/archive/mod coverage remain open.
No performance, space or AndroidPort work is included.

## Evaluation parameters source proof

The original D2DB0→D2F20 literal roots 6e79a2. Its score prefix D2DB0→D2E6A
uses seven divisor/weight pairs and two conditional bonus pairs, exactly18bytes
through6e79b4. The first quotient uses unsigned division and u8 truncation;
the remaining six use signed32 division by positive u8 divisors. Products and
sum retain32-bit wrap. The two bonuses inspect separate calls to the original
live predicate, comparing its low byte with1 and0 respectively; the first
byte of each bonus pair remains structural and unused by these branches.
Live unit attributes and original equipment/getter work are not cached here.

The full D2D44 function returns0..5; D483A calls it before the D4840 zero gate
and D4844 multiply-by-four threshold path. Literal D4846→D4888 supplies address
bias6e79c2. Active classes1..5 own five4-byte records at6e79c6..6e79da: one
threshold byte and three opaque bytes each. Class0 reads no threshold and calls
no RNG. The14-byte score gap and4-byte row0 remain unowned. The next independent
root6e79da (04B45C→04B49C, original*10pointer publication) corroborates separation.
Earlier whole32/24-byte gap assumptions are rejected;38owned bytes reconstruct
without overlap with e33. No actor/action name or foreign dependency is inferred.

The strict `evaluation_parameters` model keeps fixed7/2/5arrays, IDs, order,
offsets, all coefficient/bonus bytes and opaque tails. Its finite diagnostics
retain original arithmetic. A zero divisor yields diagnosticNone; the original
guest helper remains unchanged and its zero-divisor path is outside this proof.
Threshold comparison uses signed low16 draw modulo256, then a signed comparison
with the u8 threshold. Actual02868LCGupdates030034b0 and returns15bits, so negative
draws are tested only in isolated original-code suffixes with injected arguments.

Warning-free Windows60074 and one Wine42263 oracle pass132actual-source cases:
12live score prefixes,49arithmetic/12bonus fragments,39RNG/class0 prefixes and
20raw-draw suffixes.52mutated-source cases cover high divisors/weights, signed
values, u8 truncation/wrap, conditional arguments, thresholds0/255 and opaque
high bits. Exact source addresses, values, branches, original RNG state, register/
stack/liveunit/context guards and strictJSON/fixedshape/ID/order/offset/source/
coherent edits pass. Synthetic records fail authentication and are unpublished.
24147instructions/442PCs describe coverage only; no build/Winefailure or earlier
source/native/scene oracle repeat. All protected originals/e33/candidates/cache/
loader/canonical/source fixtures retain hashes.

Evidence: `target/rom-data-numeric-pairs/` contains the independent inventory,
records, bounded readers/calls/division/RNG/predicate/next-root disassembly,
windows-tests-build.*, windows-original-result.json and
windows-original-europe/{evaluation-parameters.json,result.json}. At this
source-only boundary EUR e33-d32/28files/27resources and USAe8-d7 are unchanged.
Managed publication/direct CPU-DMA-native/lifecycle/positive ordinary score use
were subsequently completed below; threshold ordinary use remains targeted-only. Whole-function tails, semantic names/other consumers, unused gaps and new
Linux/USA/nativeWindows remain open; no performance/space/AndroidPort work.

## Managed and direct evaluation parameters

EUR `v1/e34-d33` adds `evaluation-parameters.json`, resource
`gameplay.evaluation-parameters`, format `evaluation-parameters-v1-json`.
All29files/28resources authenticate together; every27earlier decoded file and
e33generation remains unchanged. Both source roles preserve18score and20threshold
bytes with exact IDs/order/offsets. No semantic foreign dependency is inferred.
The14-byte score gap/4-byte row0 stay unowned. USA retains e8-d7. Parsing limits
bound malformed input; no extraction-space or performance target is measured.

Typed bytes serve original CPU/native/SWI/DMA reads on page6e7, merging raw
bytes at all four boundaries. Nonserialized divisor/weight/bonus/opaque/threshold
counters and a five-class mask distinguish new use. Original live producers,
unsigned-u8 first quotient, six signed32 divisions, wrapping score, conditional
low8predicates, RNG/signed-low16 threshold comparison, clock and save/nativeABI
remain active. Diagnostic scoreNone for a zero divisor supplies no guest fallback;
that helper edge remains unclaimed, with the guest routine unchanged.

Initial Windows51951compilation succeeds with an unused-model import warning,
which the strict wrapper rejects. Pre-Wine review also replaces isolated native
fragments whose stop addresses overlap the complete score trace with27complete
score/shared-threshold prefixes. Corrected68404build is warning-free; the source
132+52oracle already proves isolated arithmetic/bonus fragments and is not rerun.
Wine32104two checks pass publication interruption/create/reuse/repair/move/e33
retention, coherent divisor/weight/unused-bonus/opaque-threshold/source/dependency/
path/both-role guards, transactional attachment and unavailable-copy Load/Undo/
backup/restart. No Wine failure occurs.

27original/interpreter/compiled prefixes with all38raw bytes altered match
complete states/clocks. Compiled reads consume82divisor/84weight/12bonus/
14threshold bytes and0opaque bytes; thresholdmask31covers all five classes.
Class0 consumes no new bytes. All38bytes/edges/widths/mirrors/real-BIOS, four
typed/raw word crossings, both-source CpuSet/FastSet (18/20typed bytes) and
2typed2raw DMA checks pass. All18gap/row0bytes remain raw without new counters.
13068instructions/342dispatches describe coverage only. Unused fields retain
exact bytes through typed all-byte tests, not invented original field reads.

Warning-free Windows87827production restores the canonical executable.
ActualCLI7961creates and reuses e34without managed-file/hash/mtime changes,
then uses the old native cache from an unrelated working directory. Bounded
063768→D2D44candidate evaluation justifies the retained1840Pubmission input.
The first sample is positive:15score reads,7divisors/7weights/1bonus,0opaque/
threshold bytes and thresholdmask0. One Wine71943pair matches every complete
state/clock, central-wide image, OAM, dispatch and mixed/PSG/FIFO/HLE audio
stream with those counters. The final Pub background is inspected; separated
panels do not independently establish selected-unit or completed-mission state.
Threshold ordinary use remains targeted-only. No negative scene locator, new
fixture, directRAMedit, persisted save write or old source/e28/e33acceptance repeat.

Evidence: `target/rom-data-numeric-pairs/` contains windows-initial-warning-managed-
tests-build.*, windows-managed-tests-build.*, windows-managed-results.json,
windows-managed-europe/{managed-result.json,native-result.json}, windows-build.json,
windows-cli-{processes,no-write,summary}.json, windows-scene-result.json,
windows-pub-evaluation-parameters-full-state-audio.log, pub-dispatch.png,
class-callers.json and caller-063724.txt. Protected originals/e33/sourceproof/
fixtures/input/cache/loader/canonical/candidates remain unchanged. Whole-function
tails, semantic labels/other consumers/unused gaps, the separate11Bglobal ordinary
condition gate, newLinux/USA/nativeWindows and broader gameplay/maps/audio/
GUI/archive/mod phases remain open. No performance, space or AndroidPort work.

## Next ten-byte progression and price records

Static discovery roots6e79da through CF68E→CF6D8,04B45C→04B49C and
04B4B2→04B4DC. Original multiply-by-ten publishes a selected ROM row through
0201F530; the default branch publishes row0. Normal index producer04B3E0..
04B416reads020021b4, increments only an old value at most98 and writes a value
at most99. This establishes100normal records/1000bytes through6e7dc2. The
candidate101st row is rejected as ownership evidence; twelve zero bytes before
the independent6e7dce root (0487F6→048868) remain unowned.01A5A6displays an
adjacent progress byte and does not establish the index bound. Invalid imported
states and out-of-range indices retain original raw access; no fallback is added.

Fields00..07are eight u8 deltas. Checked original callers04B56E/580/592/5A4/
5B6/5C8/5DA/5F0pass them through original04870c to live level/progress slots1..8
with flag bits2..256. That routine retains live growth, u8addition/wrap, carry,
level99cap and behavior for an already invalid old level. Specific skill names
and global initialization preconditions remain unproved. Field08is the discount
byte read by CF674 from the live index; byte09is structural and unused here.

The price reader uses the same6e0bd0base as actual parsed Items, truncates mode
tou8/itemtou16and gates discount on itembyte0c&8. Its live predicate can add10
to the discount with u8wrap. Original signed32 base*(100-discount)/100,
unsigned comparison with minimum price and final zero→1remain distinct. These
records store no item IDs or other proven foreign keys, so no owner dependency
is inferred merely from the consumer. Strict `ProgressionRewards`/`Record` now
preserves fixed IDs0..99/source offsets/eight delta bytes/discount08/opaque09.
Validation does not constrain deltas/discount to the values observed in this ROM.
`record` returns diagnostic None for unowned IDs without changing guest access.

Windows13783build is warning-free; one reduced Wine62460original-code oracle
passes362real cases:7default/indexed publisher prefixes,232original caller→
full04870cdelta helpers and123completeCF674prices. Every row participates;
all eight delta fields, original live growth/progress/carry/u8wrap/cap99/flags
and old99/255 behavior are checked. Prices use actual source-authenticated
parsed Items, original mode2..6mask lookup/item getter and live predicate, plus
u8mode/u16item truncation, ungated reads and raw outside indices100/255.
68isolated high-delta/opaque/discount/u8+10wrap/signed quotient/unsigned minimum/
zero cases pass. Synthetic copies remain unauthenticated/unpublished. Strict
JSON/fixed arrays/IDs/order/offset/coherent edits/all1000Broundtrip/callee/SP/
complete EWRAM and stack guards pass;64148instructions/404PCs are coverage only.
The arithmetic suffix stops before its epilogue and does not claim whole ABI.
Whole progression caller, semantic skill names, managed preparation/direct
consumption/positive ordinary use and newLinux/USA/nativeWindows remain pending.

All1000bytes reconstruct without overlap with e34. Evidence is under
`target/rom-data-metadata10/`: progression-rewards-inventory.json, independent
records, discovery.json and bounded selector/field/price/live-consumer/next-root
disassembly. Current EUR e34-d33/29files/28resources and USAe8-d7remain unchanged.
New source evidence: `windows-tests-build.{jsonl,log}`,
`windows-original-result.json`, `windows-original-europe/{progression-rewards,
result}.json`, source-final-summary.json and source-final-source-hashes.json.
All37protected ROM/e34dataset/cache/loader/canonical/candidate/source fixtures
remain unchanged; no old source/native/scene proof is repeated. External
HEADe634dd8committed earlier e34/static work; preserved without agent commit.
No performance/space campaign or AndroidPort change is included.


## Managed and direct progression rewards

EUR `v1/e35-d34` adds `progression-rewards.json`, resource
`gameplay.progression-rewards`:30files/29resources with all28prior decoded
files and the retained e34generation unchanged; USA remains e8-d7. Preparation,
bounded parsing, manifest/file/source-role authentication and immutable runtime
ownership bind the exact1000Brow fields. No item foreign-key dependency is added:
these records store no item IDs. Original index publication, live growth/level/
progress/flags, mode masks/item getter/predicate/prices, instructions, clock,
save/native ABI remain unchanged. Reads outside0..99and twelve trailing bytes
retain raw access. Typed views compose with earlier resources at shared edges.

Warning-free Windows84154build and Wine80590two focused checks pass. Managed
publication interruption/create/reuse/repair/move/e34retention, coherent edits
to delta/lastrow/discount/opaque/reordered records/source/dependency/path/role,
transactional owner/state preservation and unavailable-file Load/Undo/backup/
restart pass. All1000typed bytes and all100record-mask bits are verified.
27selected original/interpreter/native paths match complete states/clocks with
all1000raw source bytes altered: compiled17delta/4discount/0opaque bytes;
6publisher/ungated/outside paths use0newbytes. Original whole-price paths and
nonoverlapping delta fragments share safe exits; isolated arithmetic suffixes
from the closed source oracle are not added to that native object.
Allbytes/record boundaries/8-16-32bit widths/three mirrors/real-BIOS flag,
12unowned trailing bytes, crossing2old evaluation+2new progression bytes,
CpuSet/FastSet32typed bytes each and DMA2typed+2raw bytes pass.3792instructions/
96native dispatches are coverage only. Source362+68proof is closed, not repeated.

Evidence: `target/rom-data-metadata10/windows-managed-results.json` and
`windows-managed-europe/{managed-result,native-result}.json`. ActualCLI25924prepare/reuse/no-write/old-cache/unrelated-cwd checks pass;
all32managed files retain hashes/mtimes. First600frame town→Shop→Buy locator
reads7discount bytes/record4(mask16), with no delta/opaque reads. One Wine52469
full pair matches every complete state/clock/central-wide image/OAM/dispatch
and mixed/PSG/FIFO/HLE audio stream, with the same7typed discount bytes.
Final shop background is visually inspected; the separated inventory panel
is absent from the raw PPM, so item selection/transactions are not inferred
from that image. Delta use remains targeted-only. No negative locator, new
recording/RAM edit/persisted save write or old accepted proof is repeated.
Production33726build is warning-free and canonical binary restored by hash.
Evidence: windows-build.json, windows-cli-{processes,no-write,summary}.json,
windows-scene-result.json, windows-shop-progression-rewards-full-state-audio.log,
shop-buy.png and final-summary/final-source-hashes/final-protected.json in the
same ignored directory. All protected fixtures remain unchanged.
NewLinux/USA/nativeWindows,
whole progression caller/semantic names and the broader TODO remain pending.
No performance/space measurements or AndroidPort changes are included.


## Next progression gates and returned receipts

Static inventory bounds64records×12bytes at6e7dce..6e80ce (768B). Original
0487F6→048868roots the table; offset+=12/index0..63in0487c8establish its finite
extent. Independent D23E6→D24A8roots exactly6e80cefor another stride16consumer,
corroborating the exclusive end without owning any next-table bytes. The
12zero bytes between e35progression rewards and this source remain unowned;
none of the768bytes overlap existing e35resources.

Each record preserves byte00, little-endian halfword01assembled from bytes1/2,
byte03and eight u8requirements04..0b. Byte00currently equals the physical row
index but is not read by this consumer; do not use that observed equality to
normalize/drop it or select the received-bit index. Original0487c8reads a live
u64mask at02002c08, scans unmarked physical indices in ascending order and
compares each of eight live bytes020021b6+2*k(k0..7)unsigned against the minima.
The first comparison is separate from the following seven-step loop. First
eligible record sets its physical bit in the original mask through the original
145bd0u64shift helper, writes byte03to the caller output and returns halfword01.
A miss writes output0and returns0without marking. These results are distinct:
row63has zero requirements/return/output but still marks bit63 on success.
A scalar0is therefore insufficient to diagnose a miss.

Caller04B60Epasses its stack output byte, truncates/tests the returned halfword
and only enters047670when nonzero. That receiver retains its <=375and larger
encoded-value branches. All actual64returned values fit the source-authenticated
376Items (0..375), but their complete downstream receipt semantics/foreign-key
contract is not yet proved; keep the two-byte field structural. Specific skill/
reward names and whole receipt UI behavior remain unclaimed.

Exact EUR root/stride/count/field/load/store/mask/return/caller/next-root opcodes,
all768Broundtrip and no-e35overlap pass statically. Private inventory initially
compared a decoded BL ROM offset with a guest address; the address base was
corrected before producing its successful result. The earlier047678tail locator
is superseded by the true047670function entry. No new model/build/Wine test
ran for this family; source oracle/managed/runtime/ordinaryuse/newLinux/USA/
nativeWindows remain pending, current e35-d34/30files/29resources unchanged.
Evidence: `target/rom-data-progression-gates/{inventory.py,discovery.json,
progression-gates-inventory.json,progression-gates-independent.json,
consumer-0487c8.txt,result-consumer-047670.txt,next-root-0d23b0.txt,
next-0d2420.txt,static-summary.json,static-source-hashes.json}`. No performance/
space or AndroidPort work is included; e35source/native/600Shop checks stay closed.
