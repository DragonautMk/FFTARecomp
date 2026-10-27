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
were subsequently completed below; class1 threshold ordinary use is now verified in the acceptance prefix linked below. Classes2..5 remain targeted-only. Whole-function tails, semantic names/other consumers, unused gaps and new
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
That score pair reads no thresholds. The later short acceptance proof below
closes class1 ordinary use. No negative scene locator, new fixture, directRAMedit,
persisted save write or old source/e28/e33acceptance repeat.

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
from that image. Delta use at this earlier600frame boundary is targeted-only; the later e36
1000frame dispatch completion below proves all8delta fields of row0. No negative locator, new
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
is superseded by the true047670function entry. That inventory did not add a
model/build/runtime check; the source contract is now proved below. Managed/
runtime/ordinaryuse/newLinux/USA/nativeWindows remain pending, current
e35-d34/30files/29resources unchanged.
Evidence: `target/rom-data-progression-gates/{inventory.py,discovery.json,
progression-gates-inventory.json,progression-gates-independent.json,
consumer-0487c8.txt,result-consumer-047670.txt,next-root-0d23b0.txt,
next-0d2420.txt,static-summary.json,static-source-hashes.json}`. No performance/
space or AndroidPort work is included; e35source/native/600Shop checks stay closed.


## Progression gates source contract

Strict `ProgressionGates`/`Record` preserves64physical IDs/source offsets,
byte00, halfword01, byte03and fixed eight-byte requirements without constraining
byte00to its observed ID, minima to0..50or returned halfword to item IDs. The
finite diagnostic `Outcome` carries matched ID separately from returned value,
output byte and updated received mask. Original0487c8and its u64shift helper
remain responsible for guest state; no routine or event work is replaced.

Warning-free Windows75099and one Wine11775original full-function oracle pass:
80actual-source cases select every physical row at its exact eight minima,
fail below each dimension, exercise multiple eligible rows/physical ordering,
bits31/32/63/fullmask/levels0-255and distinguish zero-valued row63success from
miss.25isolated copies with unrelated high byte00/high u16receipts/output128-255/
unsigned128minima test above/equal/below and early failures; synthetic copies
remain unauthenticated/unpublished. All704used source bytes (eleven per row)
are read; all64unused byte00fields are preserved by the768Broundtrip and never
read by this original consumer. Strict JSON/fixed-array/count/ID/order/offset/
coherent-edit/source/hash guards and complete EWRAM (only received mask/output
changed), callee registers/SP/stack guards pass.121113instructions/132PCs are
coverage only. No downstream receipt UI/foreign-key/semantic skill claim.

Evidence: `target/rom-data-progression-gates/windows-tests-build.{jsonl,log}`,
`windows-original-result.json`, `windows-original-europe/{progression-gates,
result}.json`, source-final-summary.json and source-final-source-hashes.json.
All38protected ROM/e35dataset/cache/loader/canonical/candidate/source fixtures
remain unchanged. External HEAD4423b91committed prior e35work and this source
implementation during compilation; its diff is inspected/preserved without an
agent commit. New managed publication/direct reads/lifecycle/ordinaryuse and
Linux/USA/nativeWindows remain pending; no old accepted proof is repeated.


## Managed and direct progression gates

EUR `v1/e36-d35` adds `progression-gates.json`/`gameplay.progression-gates`:
31files/30resources, all29prior decoded files/e35generation retained; USAe8-d7
unchanged. Strict parsed immutable records/source-role/file/manifest checks bind
all768B, with fixed-page routing on6e7and6e8and typed requirements/return/output/
unused fields. Physical record-mask diagnostics cover all64bits. Original live
levels/mask/firsteligible/64bitshift/receipt caller/instructions/clock/save-native
ABI remain unchanged;12Bpreceding gap/nextroot at6e80ceremain raw. No invented
item/receipt foreign-key dependency or byte00-ID equality constraint is added.

Warning-free Windows68535and Wine97635two reduced checks pass. Publication
interruption/create/reuse/repair/move/e35retention, coherent requirement/return/
output/unused/lastrow/reorder/source/dependency/path/role tampering and failed
attach owner/state preservation pass. An unavailable-copy Load/Undo/backup/
restart retains owned resources; all768typed bytes/all64record bits are checked.
19original/interpreter/native full0487c8paths share the same return exit and
match complete states/clocks with768raw bytes altered. Compiled counters read
369requirement/18return/9output/0unused bytes;two fullmask paths read0newbytes,
two zero-valued successes still set the physical received bit. Allbytes/record
edges/8-16-32widths/mirrors/real-BIOS flag/bothpages/pagecrossing/12rawgap bytes
and oldreward2/newgate4typed coexistence pass. Cross-page CpuSet/FastSet read
32typed bytes each; DMA reads2typed+2raw.28874instructions/158native dispatches
are coverage only. Source80+25and earlier e35proof are closed, not repeated.

Evidence: `target/rom-data-progression-gates/windows-managed-results.json`,
`windows-managed-europe/{managed-result,native-result}.json` and
managed-final-summary.json/managed-final-source-hashes.json. Production and
ordinary scene05are now verified below. The original ordinary call04B60Eis gated by
04B5FC..04B606checking live0201f504&0x1feafter the eight delta calls; context
pointer0201f508and selected row pointer0201f530remain original. It must be
reached through normal gameplay, not a RAM edit or unrelated blind Shop/Pub
replay. Source/native proof does not establish ordinary receipt/UI behavior.
NewLinux/USA/nativeWindows/downstream receipt semantics/FK/other consumers and
wider TODO remain pending; no performance/space or AndroidPort work included.


## Ordinary progression gates dispatch completion

Windows65421productionbuild is warning-free and restores the canonical binary
by hash. ActualCLI70968create/reuse/no-write/unrelated-cwd and a one-frame
old-native-cache/battle-snapshot startup pass;33managed file hashes/mtimes stay
unchanged. That startup reads0newbytes and is compatibility-only. A read-only
retained-snapshot locator inspects174EURheaders/49checksum-verified representative
payloads: none has the needed newlevel flags. It includes old targeted and
rejected-candidate states and provides no ordinary acceptance. A1840prefix
rendering probe still lacks detached panels because snapshot DialogueState
restores separation after runc's environment setting; no product code is changed.

One isolated Wine/Xvfb/Vulkan GUI session follows normal gameplay. Mad Alchemist
requires10dispatch days/no items/jobs/skills; Sigmund is sent for600gil on day9.
Normal map travel through the western node/Nubswood reaches day19. An observed
encounter on the eastern route is avoided with normal Load of the new day9slot;
no ROM/RAM/snapshot payload is edited. New private UI slots2(day9accepted),
3(day18pre-return),4(day19MissionCleared) support this previously missing
criterion; the original copied slot1and all source fixtures remain unchanged.
The inspected GUI shows Mission Cleared/unit returned,3400gil/80AP/MythrilPick/
TerreRod and skill levels8,1,11,3,2,8,42,2→8,2,11,4,2,8,42,2. Only isolated
storage/config/test cartridge files may change; no personal save is written.
GUI counters read209new requirement bytes/8delta bytes/1evaluation threshold
(class1). This does not infer newly unlocked receipts: new return/output reads0.

First new1000frame CLI replay29523from day18slot3 is positive209requirements/
8deltas. One Wine57387full pair matches every complete state/clock/central-wide
image/OAM/dispatch/mixed-PSG-FIFO-HLEaudio stream with identical counts. New
record mask6155840078674362367shows observed unmarked-row tests, not all64ordinary
rows. e35delta ordinary use is now proved for row0/all8delta fields; earlier600
Shop/362+68source/27native proof is not repeated. e36return/output and all-record
success coverage remain targeted-only. The signing/fee sequence occurs before
slot3, so this pair reads0thresholds; its separate paired acceptance prefix is
now verified below. No fullpair runs on negative
coverage and no benchmark/profiling/performance/space measurement is performed.

The shared test executable was overwritten by parallel Debug-save work. Its
e36scene validator/ROM-data implementation is checked unchanged; the current
Windows test artifact is frozen as recomp-tests-scene.exe (9e5e6f0cfa4db6c9c4af8469
11e7012a6fef2fe29db859454709a4c351623e17) before this new full pair. It includes
that parallel compiled code; the normal9-slot format/loader used here is
unchanged. The e36production candidate is46c8cb78cadb9c7e2d7d41f236135afaee39ca8e6
05b89fc504eaec412daff1d. Source/native19proof remains on its original artifact;
no current binary is falsely identified as that earlier proof. External3ea9b61
commits e36source/runtime; other Debug-save source/docs/skill changes are
inspected/preserved without agent edits or commit. Normal-play host diagnostics
remain in GUI logs without performance interpretation; volume0is not an audible
quality test. NewLinux/USA/nativeWindows/whole receipt UI/FK/other consumers/
remaining TODO stay pending.

Evidence under `target/rom-data-progression-gates/`: windows-build.json,
windows-cli-{processes,no-write,summary,dispatch-original-ui,dispatch-completion}.json,
retained-state-locator.json, windows-dispatch/{commands,result,scene-inputs}.json
and inspected captures, dispatch-completion.input, scene-executable.json,
windows-scene-result.json, windows-dispatch-progression-gates-full-state-audio.log,
final-summary.json/final-protected.json/final-source-hashes.json. Private data
and states remain ignored and excluded from commits/packages.

## Ordinary evaluation threshold acceptance

One new normal Wine GUI checkpoint captures Mad Alchemist/Sigmund immediately
before Accept conditions Yes. The isolated normal9-slot menu saves slot2;
confirmation shows the600gil fee/funds7109, then the GUI closes0. Source ROM,
original states and copied slot1 retain their hashes; no ROM/RAM/state payload
or personal save is edited. This GUI stops at the fee dialogue and reads no
threshold: the observed boundary includes confirmation and fee acknowledgement,
not the Yes press alone.

The first400frame actualCLI70012from this checkpoint reads16evaluation bytes:
7divisors/7weights/1bonus/1threshold, thresholdmask1/class1, no opaque bytes.
One Wine96264pair matches all400complete states/clocks/central-wide images/OAM/
dispatch and every mixed/PSG/FIFO/HLE audio stream, with the same positive
threshold read. The normal Yes/fee sequence closes the class1 ordinary gap;
classes2..5, whole caller and downstream outcomes retain their earlier scopes.
No gate/reward use is inferred from this short scene.

Frozen e36production46c8cb78/test9e5e6f0c, e36dataset, native cache and loader
are reused without a build. Source132+52/native27/1840score and1000dispatch
completion proofs remain closed. Evidence in `target/rom-data-progression-gates/`:
windows-accept-threshold/{commands,result,scene-inputs}.json and inspected captures,
threshold-accept.input, threshold-scene-inputs.json, windows-cli-threshold-accept.json,
windows-threshold-scene-result.json and windows-threshold-accept-full-state-audio.log.
CLI33managed hashes/mtimes and all scene input hashes remain unchanged. NewLinux/
USA/nativeWindows remain pending; no performance/space/AndroidPort work.

## Progression selection gates source contract

Original D2390 roots6e80ce and scans physical indices1..64 with stride16,
therefore the owned source is6e80de..6e84de/1024bytes. Its unused row0/16bytes
remain unowned. Independent16192→161B0roots the next source at6e84e0, leaving
another2bytes unowned. The structural name avoids assigning a rank/title meaning
to the returned selector without tracing its display consumers.

Each record preserves selector00, unsigned minimum01, little-endian unsigned
minimum02/minimum04, eight any_minima06_0d and a little-endian flag0e. Nonzero
minimum01compares live020021b4; minimum02compares02001f6c; minimum04compares the
u16result of original D22FC. Every hard minimum must pass. For the eight live
bytes020021b6+2*k, if any corresponding minimum exists, at least one active
minimum must pass. Zero minima do not count as passes. Nonzero flag0e must pass
original CCE98. No selector00==physical ID or observed-range constraint is added.

Physical order is significant. The first eligible row with at least one condition
writes selector00to020021c6. A row with no conditions explicitly writes0and stops,
even if selector00is nonzero. Exhausting64rows performs no output store, retaining
the old byte. Actual row52is the zero-condition terminal;53..64remain lossless
storage without an ordinary reachability claim. Diagnostic Outcome keeps the
matched physical index and optional output store separate.

The original function's low8input, when nonzero, first clears flags5a1/5a2/5a3/5a9
through CCECC. The original D22FCaggregate counts live flags for a separate
300-entry raw source plus flag36or a linked017fitem fallback. Those live helpers
and that separate raw source remain original; the diagnostic model takes their
result and the post-reset flag view. Six original callers pass reset0or1,
including04B464/04B4B8and063C54/063CC8. No model is published or attached to guest
reads in this step; EUR e36-d35/31files/30resources and USAe8-d7 remain unchanged.

Static opcode/root/extent/no-overlap and independent1024byte roundtrip pass.
The strict source-only Catalog requires the exact EUR ROM; fixed64records,
physical IDs1..64/order/offset, exact arrays and unknown-field guards preserve
all fields. Windows21065testbuild is warning-free. One Wine82907oracle passes
138real-source cases and80unauthenticated/unpublished mutated-source cases through
full original D2390and its original reset/count/flag helpers. It observes831distinct
original field bytes through row52; all64rows are selectable in synthetic scope.
Unsigned u8/u16/higher-bit inputs, any one of8active minima, 300live-flag counts,
flag36/linked017ffallback, low8reset0/1/0x100/0x101, selectors0/255, no-condition
zero store and exhausted no-store match. Full EWRAM changes only by original
reset flags/output; SP/callee registers/stack guards pass. 2657243instructions/
248PCs describe coverage only. Strict JSON/ID/order/source/coherent-edit guards
and all1024bytes roundtrip pass. The raw aggregate-helper source is not migrated.

Protected39ROM/e36dataset/candidate/cache/loader/canonical/inventory inputs stay
unchanged. No build/Wine failure or closed source/native/scene proof is repeated.
Managed publication, direct consumption, lifecycle and ordinary scene acceptance
remain pending. NewLinux/USA/nativeWindows/semantic display consumers are
unverified; no performance/space/AndroidPort work.

Private evidence: `target/rom-data-rank-gates/` contains independent records,
inventory, bounded consumer/helper/caller/next-source disassembly, discovery,
windows-tests-build.*, windows-tests.json, recomp-tests-original.exe,
windows-original-result.json, windows-original.log and
windows-original-europe/{progression-selection-gates.json,result.json}.

## Managed and direct progression selection gates

EUR `v1/e37-d36` adds `progression-selection-gates.json`, resource
`gameplay.progression-selection-gates`, format `progression-selection-gates-v1-json`.
All32files/31resources authenticate together; every30prior decoded file and the
e36generation remain intact. USA stays e8-d7. The same fixed physical IDs1..64,
1024byte span, source binding and source-equal JSON are verified transactionally.
No rank/title interpretation, selector00==ID or semantic foreign dependency is
inferred. Row0/16bytes and the next2byte gap retain raw ROM behavior.

Typed CPU/native/SWI/DMA reads use the model fields on source page6e8 and merge
raw/prior-family bytes at each boundary. Nonserialized counters separate
selector00/minimum01/minimum02/minimum04/any_minima06_0d/flag0e and a64bit physical
record mask. Original D2390reset/count/flag helpers, unsigned comparisons,
ANY thresholds, physical order, terminal store/no-store and every guest
instruction/clock remain unchanged. Save/native identities stay unchanged.

Two focused Wine73517checks pass publication interruption/create/reuse/repair/
move/e36retention; coherent fields/lastrow/reorder/source/dependency/path/role/
missing binding/hash and transactional attachment; unavailable-copy Load/Undo/
backup/restart; all1024bytes/64recordmask. Initial48254test compile misses the
32nd argument in an existing aggregate-guard fixture; its zero placeholder is
corrected, failed logs retained and corrected77778build is warning-free.

18full original D2390/interpreter/native paths with all1024raw bytes altered
match complete states/clocks. Compiled counts are selector15/minimum01 282/
minimum02 505/minimum04 462/ANY1635/flag162. All bytes/record edges/widths/mirrors/
real BIOS,16raw row0bytes/2raw trailing gap bytes, CpuSet/FastSet32typed bytes
each, CpuSet oldgate2/raw16/newselection2coexistence and DMA2typed2raw pass.
211068instructions/11893dispatches describe coverage only. Original setup/run
is factored unchanged to share these18new paths; the closed138+80source oracle
is not repeated. No original helper or output semantics are replaced by host code.

ProductionCLI and ordinary scene are now verified below; newLinux/USA/
nativeWindows/semantic consumers remain pending. No performance/space/AndroidPort work. Private evidence
under `target/rom-data-rank-gates/`: windows-managed-tests-build.*,
windows-initial-failed-managed-tests-build.*, windows-managed-results.json,
recomp-tests-managed.exe, windows-managed-europe/{managed-result,native-result}.json
and protected manifests. External99fb9fdcommits prior source/threshold/Debugcatalog
work; new coverage-tool changes are preserved without agent edits or commit.

## Ordinary progression selection dispatch completion

Warning-free production34174restores the canonical Windows binary by hash;
private e37candidate SHA870a964806a6d6b19e458280aa375b5a190ee8b05ccec655d31e3a192f3fd20a.
ActualCLI43381creates and reuses e37from an unrelated working directory without
changing34managed file hashes/mtimes, then replays with the existing native cache.
The first400frame Yes/fee locator reads0selection bytes; it is closed negative,
with no full pair and no change to the earlier class1threshold acceptance.

The existing normal-UI day18checkpoint and1000frame dispatch-completion input
reach original04B464/4B4B8. FirstCLI9298reads277new selection bytes: selector1,
minimum01 26/minimum02 46/minimum04 42/ANY152/flag10. Mask67108863observes the
first26physical rows, not all64ordinary selections. One Wine3138full pair matches
all1000complete states/clocks/central-wide images/OAM/dispatch and every
mixed/PSG/FIFO/HLE audio stream with the same277bytes/all6fields positive.
The inspected final image shows Sigmund/MadAlchemist success and skill levels
08,02,11,04,02,08,42,02. No new GUI/fixture, ROM/RAM/state-payload edit or personal
save write is needed. This new pair proves e37only; source138+80/native18 and
accepted e36/8delta/class1threshold proofs remain closed.

The frozen corrected77778test executable is37a243230d39db1e34a40ac3b7782b5565efe5c99
bca89ca0ca2aa23077358ed, validator84b5d4230b8386b2b50d52746ddb1b3edfe8287809e4567e26b
0fc742c461e5c. Evidence under `target/rom-data-rank-gates/`: windows-build.json,
windows-cli-{processes,no-write,summary,dispatch-completion}.json,
windows-scene-result.json, windows-dispatch-selection-gates-full-state-audio.log,
scene-executable.json and dispatch-completion.png. ROM/state/input/dataset/cache/
loader/candidates/canonical hashes are protected, CLI also checks file mtimes.
NewLinux/USA/nativeWindows/all64ordinary rows/other callers/semantic display
consumers and the rest of the TODO remain pending; no performance/space/
AndroidPort work or agent commit/push.

## Aggregate flag keys source contract

The raw ordered table at6e9a5c..6e9cb6owns602bytes:300little-endian u16slots
and terminalffffat6e9cb4. Original D2300→D2344and13FB22→13FB74independently
root the two counters. Independent29AF4→29B0Croots the next table at exactly
the exclusive end6e9cb6. All602bytes roundtrip without overlap with e37.
Other indexed lookup/publication readers50354/5165C/51E50/52110are statically
located; this does not prove their complete UI contracts or arbitrary indices.

Original D22FCcounts global flags at02001f70;13FB1Ccounts flags in the passed
context atctx+1f70. Both add0x2ffto each stored key and truncate to u16before
their original flag predicate. Count is by occurrences, preserving order and
duplicates. Affffinside a slot stops immediately; later slots remain lossless
storage. The explicit terminalffffkeeps the declared stream bounded. The exact
base contains300unique keys3..315with gaps; uniqueness/value==ID/range/foreign-key
constraints are not imposed on the model. Synthetic high/wrapping keys are
separate unauthenticated cases and are never published.

After the key loop, either live flag36or an original linked list containing
masked ten-bit ID017fadds one, with OR/short-circuit behavior. Global list head
is020028c8; context list head isctx+28c8. Original pointer chasing/flag getters/
return-u16 remain active. A diagnostic model reads a flag closure and a linked
match input; it does not replace guest code, bind an unrelated context or write
RAM. Strict fixed300slot IDs/order/offset/terminal/unknown-field/source checks
and exact602byte authentication preserve the structural contract.

Windows5092testbuild is warning-free. One Wine38100oracle passes32real-source
and26mutated-source cases through both complete counters and original flag/
linked predicates. All602source bytes are observed. Global/context flag isolation,
key+2ffu16wrap, duplicate occurrences, early slotffffat0/1/149/299, flag36ORmasked
linked first/second matches and short circuit pass. Full EWRAM stays read-only;
SP/callee/stack guards pass. 506977instructions/177PCs describe coverage only.
The previous e37source/native/scene proofs are not repeated. Protected ROM/
e37data/states/inputs/cache/loader/candidates/canonical retain hashes.

This step adds a source-only model; EUR e37-d36/32files/31resources and USAe8-d7
remain unchanged. Managed publication/direct consumption/lifecycle/ordinary
scene and newLinux/USA/nativeWindows/whole other UI readers remain pending.
Private evidence: `target/rom-data-aggregate-flag-keys/` contains independent
records/inventory/literal readers/bounded disassembly, windows-tests-build.*,
windows-tests.json, recomp-tests-original.exe, windows-original-result.json,
windows-original.log and windows-original-europe/{aggregate-flag-keys.json,result.json}.
No performance/space/AndroidPort work or agent commit/push is included.

## Managed and direct aggregate flag keys

EUR `v1/e38-d37` adds `aggregate-flag-keys.json`, resource
`gameplay.aggregate-flag-keys`, format `aggregate-flag-keys-v1-json`.
All33files/32resources authenticate together; all31prior decoded files/e37
remain unchanged. USA stays e8-d7. Strict fixed300slots/terminal/source binding
and source-equal JSON attach transactionally. Readability is retained without
an extraction-space target; malformed-file parsing guards remain bounded.

Direct typed reads project602bytes on page6e9, merging neighboring raw/prior
bytes. Nonserialized counters separate600key bytes from2terminal bytes and
track all300physical slots in five64bit words. An in-slotffffis still a keyfield;
the original counter decides its early-stop behavior. Original global/context
arguments, key+2ffu16wrap, flag36ORlinked017f/count/return-u16, every instruction/
clock and save/nativeABI remain active. No host replacement of live code occurs.

Windows76657testbuild is warning-free. Two focused Wine87184checks pass
publication interruption/create/reuse/repair/move/e37retention; coherent key/
duplicate/earlyffff/terminal/ID/order/binding/source/dependency/path/role/hash/
coverage and atomic owner guards; unavailable-copy Load/Undo/backup/restart;
all602bytes/300entry mask including the last44bits of the fifth word.

16full original/interpreter/native global and context counter paths with
all602raw bytes altered match complete states/clocks. Compiled key/terminal
counters are9540/30bytes. All bytes/entry edges/widths/mirrors/real BIOS,
preceding8/following4raw bytes, terminal-word2typed2raw, CpuSet/FastSet32typed
each plus terminal CpuSet2typed2raw, and DMA2typed2raw pass. 168888instructions/
10357dispatches describe coverage only. Original key/terminal reads and every
live predicate/context/linked pointer remain original code; source32+26is not
repeated. No build or Wine failure occurs.

ProductionCLI and ordinary scene are now verified below; newLinux/USA/
nativeWindows/whole other UI readers remain pending. No performance/space/AndroidPort work. Evidence under
`target/rom-data-aggregate-flag-keys/`: windows-managed-tests-build.*,
windows-managed-results.json, recomp-tests-managed.exe,
windows-managed-europe/{managed-result,native-result}.json and protected manifests.

## Ordinary aggregate keys dispatch completion

Warning-free production59636restores the canonical Windows executable by hash;
private candidate SHA2dfdd6096c948234e34b33cf3e3af3757771b882868e7001af436e1b63428f25.
ActualCLI44401creates and reuses e38from an unrelated working directory with
35managed file hashes/mtimes unchanged, then replays with the old native cache.
The first1000frame replay of the existing normal-UI day18dispatch checkpoint
reads1204new source bytes:1200key/4terminal, with all300physical slots in the
five-word mask (last44bits). Counts include rereads;1204is not a distinct-ROM-
byte count and does not establish both counter entrypoints in ordinary play.

One Wine11196full pair matches all1000complete states/clocks/central-wide images/
OAM/dispatch and every mixed/PSG/FIFO/HLE audio stream with the same counters/
mask. The final image shows Sigmund/MadAlchemist success and skill levels
08,02,11,04,02,08,42,02. Existing state/input suffice: no new GUI/fixture, ROM/RAM/
state-payload edit or personal save write. The new pair proves e38only;
source32+26/native16/e37same-input and earlier accepted proofs remain closed.

The frozen76657test artifact is4369f09cfb2ba859cc0d1513e50d9fd0d1916aba3ec2920b261
f4d5ce77e5af4, validatora3838598b36ba49861a5257904713ab1f270cb0e6e77c0c2af78b4d950589074.
Private evidence in `target/rom-data-aggregate-flag-keys/`: windows-build.json,
windows-cli-{processes,no-write,summary}.json, windows-scene-result.json,
windows-dispatch-aggregate-keys-full-state-audio.log, scene-executable.json and
dispatch-completion.png. Protected ROM/data/state/input/cache/loader/canonical/
candidate hashes and35CLIfile hash/mtime guards pass. NewLinux/USA/nativeWindows/
whole other UI readers/both ordinary counter entrypoints and the rest of the
TODO remain pending; no performance/space/AndroidPort work or agent commit/push.


## Adjacent halfword lookup discovery

The table rooted at6e9cb6 has16 verified literal references. The next independent
root is6e9d8a, leaving a212-byte candidate interval. Its106halfwords alone do not
prove the natural indexed domain. Generic readers dereference RAM indices;
62818mode1 uses a signed low16argument plus41, with no established0..64guard.
Fixed indices64/67 and bounded lists16..20 and21,22,4,23 are verified, but the
live producers of the other UI indices remain unresolved. An apparent6B4C0
reference is not accepted without control-flow evidence. No model or owner is
created from numeric patterns or the neighboring boundary. Reopen this partial
inventory only when a producer bound or another ownership proof is available.
Private static evidence stays under `target/rom-data-adjacent-halfword/`.

## Trade parameters source contract

Original selectorD0744 truncates its input to u8, accepts physical IDs2..6 and
returns five explicit pointers at6e85bc/6e861c/6e867c/6e86dc/6e873c. Other IDs
return null. The price readerD26AC independently uses biased root6e84fc plus
96times the u8ID. The next independent rootCFDE2→CFE14 is6e879c, establishing
five96-byte records/480owned bytes at6e85bc..6e879c. Earlier biased rows and
other indices remain unowned. A nearby counter over30live RAM rows bounds an
internal discount argument; the initial30-ROM-record hypothesis is rejected.

The strict source-only model stores physical ID, source offset,64opaque bytes,
a little-endian u16markup at40/41 and30discount bytes42..5f. Discount count0
reads byte41, also the high byte of markup. Counts1..30 read the tail. This
alias has one stored owner. The model imposes no percentage limit, header
meaning, equality across records or semantic foreign key. All480bytes roundtrip;
320opaque bytes have no price-field-read claim.

D26AC truncates its first argument to u16 and passes it to original D1F30 with
property36. That getter uses a separate70-byte source at713b3c; property36
returns byte3e times200. It is not the already decoded32-byte item catalog,
and its source is not owned by this model. Zero returns before reading any
trade parameters or live flags. Otherwise flags at02002cb4+12*ID, byte1, select
one of two paths. Both bits0and1 set select byte41+u8count and subtract
(base times percentage)/100 from base; other flags select u16markup and add
the same quotient. Multiplication and add/subtract wrap at32bits; original
146474 divides unsigned. This diagnostic arithmetic does not replace guest
code, RAM flags, division, source getter or clocks. Out-of-bank source and
discount counts above30 return no diagnostic value, never an invented guest
fallback; the zero-base early return remains defined even for those inputs.

Windows73640testbuild is warning-free. One Wine52757source oracle passes499
real-source cases,160unauthenticated/unpublished edited-source cases and15
separate arithmetic cases that explicitly inject the getter return operand.
The latter are arithmetic-tail evidence, not authenticated ROM behavior. All256
low8descriptor inputs/null outside2..6, price flags/truncation/zero skip,
count0alias/count1..30tail and all31possible live RAM counts pass. Full original
D0744/D26AC/D1F30/146474/D2728 execute. High synthetic markup proves unsigned
quotients; isolated injected operands prove32bit product/add/subtract wrapping.
All160price-field bytes are read, while320opaque bytes roundtrip only. Strict
JSON/ID/order/offset/arrays/coherent source rejection and full EWRAM read-only/
SP/callee/stack guards pass.72810instructions/184PCs describe coverage only.
146protected-file hashes including the frozen test binary match; prior source/
runtime/ordinary proofs are not repeated. No build or Wine failure occurs.

Managed publication/direct consumption/lifecycle/ordinary scene and newLinux/
USA/nativeWindows are separate pending criteria.
EUR e38-d37/33files/32resources and USAe8-d7 remain unchanged. No performance,
space or AndroidPort work is included. Static evidence remains under
`target/rom-data-stride96/`; independent fixture and the new original oracle
are under `target/rom-data-trade-parameters/`.


## Managed and direct trade parameters

EUR `v1/e39-d38` adds `trade-parameters.json`, resource
`gameplay.trade-parameters`, format `trade-parameters-v1-json`. All34files/
33resources authenticate before transactional attachment; all32previous decoded
files and e38generations remain unchanged. USA remains e8-d7. No dependency
on items.json or the separate70B source is invented. Fixed five physical IDs,
source-equal fields, source binding/role/path/hash/coverage and publication
failure guards preserve the declared480byte owner.

Typed projections on page6e8 serve CPU/native/guest-BIOS/SWI/DMA reads, merging
prior owners and raw gaps. Nonserialized counters distinguish64opaque bytes,
markup low byte40, shared byte41 and30discount tail bytes per record, plus a
five-bit physical record mask. Byte41 counts its address without inferring
which branch read it. Earlier biased rows and other indices remain raw.
Original descriptor selection, separate D1F30getter, zero early return, live
flags/count, arithmetic/division and every instruction/clock remain active.
Save/native ABI stays unchanged; immutable ownership survives Load/Undo/backup/
restart without dataset files. A shared original setup/run helper skips its
raw-source assertion only when typed ownership is attached, with no added
observer read or change to the closed source-only oracle.

Windows21694testbuild is warning-free. Two focused Wine20164checks pass
publication interruption/create/reuse/repair/move/e38retention;20coherent field/
ID/order/source/dependency/path/role/hash/coverage guards, missing-file failure
and transactional ownership; unavailable-copy Load/Undo/backup/restart. All480
bytes match after altering raw source, with320opaque/5markup-low/5shared41/
150discount-tail bytes and physical mask31.

28full original/interpreter/native paths match complete states/clocks with only
480new raw bytes altered. Compiled opaque/markup-low/shared41/discount-tail counts
are0/5/10/10: no original opaque-field price read is inferred. Descriptor/null/
zero skip/flags/count/getter remain original. All bytes, record/field boundaries,
widths, mirrors, real BIOS,222preceding raw gap bytes/4following raw bytes,
tail word2typed2raw, CpuSet/FastSet32typed each, tail CpuSet2typed2raw, earlier
selection2/rawgap222/newtrade2coexistence and DMA2typed2raw pass.
3377instructions/109dispatches describe coverage only. The closed source-only
499+160+15oracle is not repeated. No build or Wine failure occurs.

ProductionCLI and ordinary scene are verified below; newLinux/USA/nativeWindows
remain separate pending criteria.
Evidence is retained under `target/rom-data-trade-parameters/`.


## Ordinary trade parameter fee acceptance

Production46740is warning-free and restores the canonical Windows executable
by hash. The private candidate SHA is59892c74d251ff35c5144e5773d4110c69a12bd8a441
fbf67da167581e3defbe. ActualCLI17191creates/reuses e39from an unrelated working
directory with36managed file hashes/mtimes unchanged, then replays the existing
normal-UI pre-Yes slot2/threshold-accept input through the old native cache.
The first400frames read14new source bytes:7markup-low/7shared41, mask1 for
physical ID2. Counts include rereads; no14distinct-byte claim is made.

One Wine89539full400pair matches every complete state/clock/central-wide image/
OAM/dispatch and all mixed/PSG/FIFO/HLE audio samples, with the same14bytes/mask.
The final inspected image shows the Pub mission list and7109funds. No new GUI,
fixture, ROM/RAM/state-payload edit or personal save write is needed. This pair
proves e39only: the earlier class1threshold400/e38/source499+160+15/native28/
managed/lifecycle proofs remain closed and are not repeated. Ordinary discount
tail/opaque fields/all5records/other callers remain targeted-only or unverified.

The frozen21694test SHA is4070866db2c4fb6825f3f0df8e9d17dd2bf76157397ef4b677761531
f8a9bc0c, validator SHA ded241a5520283cb3b658244afd3e306f61c598e49feeda8485cbe0518cc7ed5.
Evidence under `target/rom-data-trade-parameters/`: windows-build.json,
windows-cli-{processes,no-write,summary}.json, windows-scene-result.json,
windows-fee-accept-trade-parameters-full-state-audio.log, scene-executable.json
and threshold-accept.png. Protected original/canonical/cache/loader/fixture/data/
executable hashes and CLI file mtimes remain unchanged. NewLinux/USA/native
Windows and the wider TODO remain pending; no performance/space/AndroidPort work.


## Item effect parameter static contract

Eleven verified literals root6e879c. Original getterCFE28 and helpers CFDA8/
5AAAC use7-byte records. Four independent readers root the next table at6e87fe,
exactly14records/98bytes later. Existing original item enumeration362..375/
14records and explicit subtraction of362 agree with physical IDs0..13. This
reuses the closed item enumeration proof; it does not assign item names or
reclassify the existing catalog. Arbitrary other indices remain raw/unclaimed.
The new source begins exactly at the e39trade parameter end, without overlap.

Each56-bit record stores nine5-bit coefficients in bits0..44, an8-bit cap in
bits45..52 and three opaque bits53..55. All98bytes roundtrip independently;
the contiguous bit representation matches each original split-byte getter.
Low8properties0..19 map to coefficient indices
0,0,1,1,1,2,2,3,3,3,4,4,5,5,6,6,7,7,8,8. Higher low8property values return0 before
source access. The getter truncates its record index to u8 without a local
14-record guard; the declared bank does not invent behavior outside its owner.

CFDA8 truncates slot to u8, item to u16 and quantity to u8. Its cap lookup uses
the full (item−362)u16index, while the coefficient getter uses that index's low8.
These are distinct address rules. With live current byte02002e7f+16*slot, it
returns coefficient×quantity when the sum fits the cap,0when current is already
at/above cap, otherwise (cap−current)u16. It writes no live value itself.
The UI wrapper5AAAC/caller5DA44 has extra cap>99 branches and live selection;
its complete write behavior is not yet verified.

The source model and original oracle are verified below. EUR e39-d38 and
USAe8-d7 remain unchanged; managed/runtime/ordinary/newLinux/USA/nativeWindows
remain pending. Private evidence: `target/rom-data-stride7/`, including
item-effect-parameters-{inventory,independent}.json, literal/caller maps and
bounded original disassemblies. No previous item oracle, fixture or benchmark
is repeated.


## Item effect parameter source model and original oracle

The strict source-only model stores14fixed physical IDs0..13, item IDs362..375,
source offsets, nine5-bit coefficients, cap45..52u8 and opaque53..55three bits.
JSON rejects unknown/missing fields, wrong arrays/IDs/item mapping/order/offsets
and values exceeding packed widths. Source equality rejects coherent field
changes; parsed376item-owner validation confirms all declared item identities.
The diagnostic getter preserves both low8arguments and invalid-property early
zero. The delta diagnostic requires the fullu16cap source index, leaving other
indices unowned; live current/quantity/cap arithmetic remain guest code/RAM.

Windows94173testbuild is warning-free. One Wine94185full originalCFE28/CFDA8
oracle passes540real cases,4explicit raw-cap alias probes and166isolated
unauthenticated/unpublished synthetic cases. All98bytes are observed in original
reads and roundtrip. Getter20property mapping/truncation/invalid earlyzero,
fullu16item-minus362cap index versus low8getter, live current byte, quantityu8,
fit/zero/remaining and u16return pass. Synthetic coefficients0/1/15/16/31,
caps0/1/99/100/255 and all opaque high-bit values preserve the original packed
rules. Four observed raw cap addresses outside the declared source are neither
owned nor a diagnostic fallback. Full EWRAM stays read-only; SP/callee/stack
guards pass.31740instructions/156PCs describe coverage only.

No build/Wine failure or old14371item/e39proof repetition occurs. Existing ROM,
owner datasets, saves/inputs/cache/loader/canonical and frozen original binary
retain258protected hashes. Private evidence under
`target/rom-data-item-effect-parameters/`: windows-tests-build.*,
windows-tests.json, recomp-tests-original.exe, windows-original-result.json,
windows-original.log and windows-original-europe/{item-effect-parameters,result}.json.
The static artifacts remain under `target/rom-data-stride7/`.

This source-only step retains e39-d38/34files/33resources/USAe8-d7. Managed
publication/direct consumption/lifecycle/ordinary/newLinux/USA/nativeWindows
and the full UI wrapper remain pending. No performance/space/AndroidPort work,
new gameplay fixture or personal save write is included.


## Managed and direct item effect parameters

EUR `v1/e40-d39` adds `item-effect-parameters.json`, resource
`gameplay.item-effect-parameters`, format `item-effect-parameters-v1-json`.
All35files/34resources authenticate together; all33previous decoded files and
e39generations remain unchanged. USA stays e8-d7. The original item IDs give an
explicit `gameplay.item-records` dependency, validated against the parsed376
item owner. Fixed98-byte source/ID/item/order/packed-width/source-binding/
path/role/hash/coverage guards attach transactionally.

Typed page6e8 reads merge the immediately adjacent e39trade owner and following
raw source. Exclusive nonserialized counters track bytes00..04 (coefficients),
byte05 (coefficient+cap), byte06 (cap+opaque), plus a14-record mask. Shared bits
retain one stored byte owner; counters do not invent semantic bit-read counts.
Original CFE28getter, fullu16cap index versus low8getter, out-of-bank raw cap,
live current/quantity/cap arithmetic, all instructions/clocks and save/nativeABI
remain active. Immutable ownership survives unavailable files and Load/Undo/
backup/restart; no guest routine or source outside98bytes is replaced.

Windows93110testbuild is warning-free. Two reduced Wine21177checks pass
publication interruption/create/reuse/repair/move/e39retention;22coherent field/
bitwidth/ID/item/order/source/dependency/path/role/hash/coverage guards, missing
file and transactional owner; unavailable-copy Load/Undo/backup/restart. All98
bytes match with raw source altered, counters70/14/14 and record mask16383.

50full original/interpreter/native paths with all98raw bytes altered match
complete states/clocks, including the explicit raw-cap alias distinction.
Compiled coefficient/shared05/shared06 counts are36/37/23. All7-byte record and
field edges/widths/mirrors/real BIOS,4following raw bytes, tail word2typed2raw,
CpuSet/FastSet32typed each, tail CpuSet2typed2raw, adjacent trade2/itemeffect2
owned crossing and DMA2typed2raw pass.2406instructions/134dispatches describe
coverage only. The closed540+4+166source oracle is not repeated; no build or
Wine failure occurs.

ProductionCLI and one EUR ordinary path are now verified below. Other items/
monsters/cap>99UI paths/newLinux/USA/nativeWindows remain pending. Existing bank
references identify EUR5AEE4/5E7B4; the earlier USA DS-22proof is not reused as
EUR evidence. A new ordinary EUR checkpoint is exported through guest input. Private evidence under
`target/rom-data-item-effect-parameters/`: windows-managed-tests-build.*,
windows-managed-results.json, recomp-tests-managed.exe,
windows-managed-europe/{managed-result,native-result}.json and protected guards.
No performance/space/AndroidPort work is included.


## Ordinary item effect parameters in the European Monster Bank

Production4620is warning-free and restores canonical Windows SHA f3e277db…;
private candidate SHA9f1093246640762e89383f566a55b2748ce6c219e91c227d14b45f0209b64927.
ActualCLI59404creates/reuses e40from an unrelated working directory with37
managed file hashes/mtimes unchanged. CLI50707then loads the old native cache
and replays400frames from an ordinary EUR Goblin Play checkpoint. It reads3
new source bytes: coefficient1/shared051/shared061, physical record mask1.

The retained EUR IN-13snapshot comes from the earlier ordinary boot/load of
its unchanged imported SRAM. A frozen existing Windows replay/export helper
uses only game buttons: travel to Cyril, MonsterBank→List, actual Goblin Lv4,
then Play/Release with Play selected. Normal guest state exports supply
`bank-eur-IN13/open-feed-1.ffstate`; four spaced A presses form bank-play.input.
No guest RAM/state payload, ROM, original cartridge file or personal save is
edited. The actual native-size captures show the owned monster and Play menu.

Initial BagubaPort(Pub/Shop) and Sprohm(Pub/Shop/Prison) probes did not reach a
bank. The guessed64,184target/constant long-hold speed was rejected. Existing
accepted Cyril pre-Yes state proves120,216/place18; short key pulses reach it
from IN13world112,168 without repeating boot/old asset checks. Those failed
navigation guesses remain terminal evidence, not an extraction/runtime failure.

One Wine5779full400pair matches every complete state/clock/central-wide image/
OAM/dispatch and all mixed/PSG/FIFO/HLE audio samples, with the same3bytes/mask.
The ordinary replay mixes1379881native and2741618fallback dispatches; these are
coverage context, not an all-native or performance claim. Source540+4+166/
native50/e39/earlier proofs are not repeated; this pair proves newly migrated
e40only. The final bank-background image is inspected, but serialized separate
UI omits its panels from CLI PPM (the existing capture limitation). No final
food count/confirmation is inferred from that image.

The frozen93110test SHA isa6e7037b4c6aa567945a303f9a659e1fa47114c1a9a74a920e0894154e0c11d3; validator SHA is7a1356a3218159919a4b0137737c5c11267c71caa89099c49cd359caa34ef93c.

Evidence under `target/rom-data-item-effect-parameters/`: windows-build.json,
windows-cli-{processes,no-write,summary}.json, windows-scene-result.json,
windows-bank-play-item-effect-parameters-full-state-audio.log,
scene-executable.json, bank-play.png, bank-locator-protected.json and
bank-eur-IN13/{known-cyril-*,enter-bank-*,open-monster-*,open-feed-*}.
All original/SPS/SRAM/snapshot/cache/loader/canonical/candidate/dataset hashes
and37managed CLI file mtimes remain unchanged. Other14item/20monster/cap>99UI
ordinary paths/newLinux/USA/nativeWindows and the rest of the TODO remain
pending; no performance/space/AndroidPort work is included.


## Twenty-slot job lookup source contract

The separate root `08399750` supplies one unsigned job byte per selected slot.
Six original EUR readers share it: `0D0538` returns the byte directly;
`0D0548/0D0568/0D0588/0D05A8/0D05C8` pass it as both job arguments to `0CBEC8`
with metadata52 properties1/4/6/9/0. The first four property wrappers keep their
low8result; the property0wrapper continues through the original name helper
`0165E8`. This classifies the lookup without inventing game-specific slot names.
The prior UI property-getter contract explicitly left this producer table out.

The source domain is chosen before table inspection from original `12CA98`.
It scans20live16-byte slots at `02002E78`, adding each nonnull slot's index to a
20-byte stack list. Mode3 selects the maximum live slot+5 value, with inclusive
ties; conditional on a single occupied slot, each index0..19can be selected.
The chosen index stays in r6 across `0CFF70`, which clears the live slot while
preserving r6. `12CB64/66` forwards that index to `0D05C8`. This is a declared
producer projection, with no universal gameplay reachability, whole physical
table extent, global20input cap, neighboring-root end or replacement of live
selection/RNG/slot state inferred. The readers' other u8inputs remain outside
this selected source domain.

268halfwords across three complete original code blocks,27critical pins,
nine literals and eight BL edges authenticate that source path. The original
six shared-root references are explicit. Narrow raw Thumb BL candidates for
other callers are retained separately; raw matches, including the one to the
property9wrapper, are not automatically treated as authenticated semantic
callers or universal input bounds.

The selected range is `399750..399764`,20bytes in one span, SHA-256
`969b2cd56b835c15f5bec9d962d9f007a8746ea192857bf0ac25527a44019a38`.
All20job references are distinct and resolve to20rows in the authenticated
`gameplay.linked-metadata52` resource. Each full row reconstructs its original
52bytes. The new range is disjoint from all64current manifest owners, using the
accepted e70-d69 dataset and its existing source-authentication evidence.
No sprite/animation target is followed and no duplicate metadata is selected.

This source-contract child is closed. The strict lossless model and focused
original Wine reader proof are recorded below; managed publication and typed
consumption remain pending. EUR remains e70-d69/65files64resources; USA remains e8-d7.
These checks use static reads/disassembly only, with ROM/tool/contract/model/
dataset hash/mtime guards. No build, game/test execution, fixture, navigation,
private gameplay edit, performance/storage measurement, Android/package/
assistant commit/push occurs. Linux, USA, native Windows and new32:9 checks
remain pending. Evidence: `target/rom-data-ui-job-lookup-source/`.


## Typed twenty-slot job lookup and original reader proof

The [slot-job model](../crates/gba-core/src/rom_data/slot_job_lookup.rs) preserves
exactly20ordered(slot,u8job)references at the authenticated source base. Strict
JSON/source/span/byte lookup validation and direct metadata52row references keep
lossless numeric values. The canonical Catalog authenticates the exact EUR ROM,
three original producer/reader blocks, source hash and actual metadata52data.
The twenty-entry structural projection is not a guest input limit; no original
selection, slot mutation, redirect, property extraction or name reader is replaced.

This model phase connected only two module declarations and left production
e70-d69/65files64resources and USAe8-d7 unchanged. Versioned extraction/runtime
ownership are recorded below; all earlier resource contents remain unchanged.

The first affected Windows frontend build passes warning-free, frozen
`110342c94d99d30ef88b3d9263b2e7fb29440d86e2d29ae1d39d0fcc95bcb7a5`.
One new exact [Wine test](../crates/recomp/src/rom_data/slot_job_lookup_model_tests.rs)
passes independent20entry/source comparison, JSON roundtrip, twelve structural/
JSON/dependency/source rejections and disjointness from all64current owners.
It executes123original reader paths:21bare job lookups,81complete low8property
wrappers and21property0prefixes preserving the full identifier before `0165E8`.
Three high-input aliases verify the original low8argument behavior. All20source
bytes are observed at the six actual original LDRBsites. Callee registers,
full EWRAM/IWRAM outside the original20-byte stack footprint, VRAM/OAM/palette
remain equal to their guarded inputs. These are source/reader checkpoints,
without a native-clock/full allocator/name-reader/ordinary equivalence claim.

All 231 build/232 Wine hash/mtime guards and
39 compiled input preimages remain exact. Canonical model JSON
SHA-256 is `2cea5e7cd140ce7cb19e179ce4997cf627cd514ee5410d5d6b86a9b9d7adfdb6`.

The closed268-pin/source contract and old metadata52/FFFF/A4/managed/native/CLI/
scene acceptances are reused without replay. Publication/runtime/actualCLI/
ordinary and Linux/USA/native Windows/new32:9 remain pending. No fixture,
navigation, private gameplay edit or performance/storage measurement occurs.
Evidence: `target/rom-data-slot-job-lookup-model/`.


## Managed twenty-slot job lookup and typed bus consumption

EUR e71-d70 publishes `gameplay.slot-job-lookup` in `slot-job-lookup.json`,
format `slot-job-lookup-v1-json`, with the actual `gameplay.linked-metadata52`
dependency. The generation has66files/65resources. All64previous resource
records and decoded files remain byte-identical, and the retained e70generation
is unchanged. The canonical twenty-entry model remains intact; USA stays e8-d7.

The [immutable owner](../crates/gba-core/src/rom_data/runtime/slot_job_lookup.rs)
serves exactly20u8job references at399750..399764from typed fields. Page399enters
the shared source path; counters report owned bus bytes and twenty record-mask
bits. Reads crossing either edge combine the file-derived bytes with prior
owners or raw ROM. Source addresses outside the selected projection retain
their original path. Original low8argument casts, metadata52properties/name
identifier, live slot selection, clearing, RNG and guest clocks remain live.
The twenty-entry projection introduces no guest input cap or host replacement.

One affected Windows core/frontend test build passes warning-free; the core is
compile-only. Two exact [Wine filters](../crates/recomp/src/rom_data/slot_job_lookup_runtime_tests.rs)
pass after the external disk failure described below:

- Interrupted publication/create/reuse/repair/move, all64prior records/files and
  retained e70,18transactional attach rejections and three full-set dependency
  rejections. Coherent changes to a job byte and source/file hashes still fail
  canonical authentication. Every failed attach keeps the owner and full state.
- Load/Undo, backup restore and restart share the immutable owner after its
  source directory becomes unavailable. All20altered raw source bytes still read
  correctly from the file and reach all20mask bits. Prior FFFFproperty reads keep
  their own owner and do not charge the new counters.
- Sixteen original/interpreter/compiled paths agree on complete state and guest
  clock with all20new raw bytes altered. They cover getter/property/name-ID
  prefixes, high-input alias, outside-slot20raw lookup, dynamic EWRAM/IWRAM, and
  the original12CA98mode3/single-occupied-slot19selection through CFF70clear and
  D05C8before165E8. The selected index and cleared live slot are checked; the
  name helper and ordinary actor/allocator ABI are not claimed complete.
- Both edges, widths1/2/4, three ROM mirrors and real-BIOS bus mode pass. The
  old64owner map confirms the neighboring bytes are raw. Two mixed API windows
  preserve edited raw neighbors and each charge2new bytes. CpuSet reads20new+
  4raw, FastSet20new+12raw and DMA20new+4raw, with complete state/clock equality.

The native filter observes 16 compiled job-source bytes.
All 233 build/235 Wine input hash/mtime guards and
41 compiled input preimages remain exact. Frozen frontend:
`5f80cdaaa0215b0935283cf801fe83f0ec4573eac25e28280b3154cde3c2e46a`.
Manifest SHA-256: `7ca7bef37999223b2475917c71c3cdbec164c5a6c5524c3119c2fa17a57356b7`;
dataset identity: `aa4588755ddad92406d9082335f96ca125e4c0ebad9e9bdcee0978ed06bbf97c`.

The first managed Wine attempt stopped with `StorageFull` while copying the
retained e70generation; the native filter had not started. Regenerable Linux
compiler libraries were relocated reversibly to a separate temporary filesystem,
with hash/mtime preservation. The failed log/result/scratch remain archived.
The two uncompleted filters then ran with the same frozen binaries, sources and
inputs, using temporary dataset and scratch directories. No product change or
rebuild was needed. A workspace link records the local validation root; these
private artifacts remain local and can be unavailable after temporary cleanup.
This recovery addresses an actual filesystem failure, not a storage-budget or
performance measurement campaign.

The closed source268-pin/model123-reader proof and previous families' results
are reused without replay. This closes managed publication and declared typed
bus/native paths. Actual production CLI is recorded below; ordinary positive
use/equivalence remain pending. Linux, USA, native Windows and new32:9 remain to be checked.
No fixture/navigation/private gameplay edit/performance-storage measurement/
Android/package/assistant commit/push occurs. Evidence:
`target/rom-data-slot-job-lookup-runtime/`.


## Actual production CLI for the twenty-slot job lookup

The first changed Windows production build passes warning-free, frozen
`720843b54da9089975c48ddc695cecf23c7670adc5d1f5ce3a03914a25c0c765`. The canonical executable's original hash and mtime
are restored exactly; main content is unchanged, with mtime-only Cargo
invalidation. The actual CLI is used for these checks.

The first three Wine calls pass prepare/create, no-write reuse and one-frame
old-native-cache attachment from an unrelated working directory. The isolated
resource root has66generation files/68managed files/65resources with the actual
metadata52dependency. All66generation files match the accepted e71-d70runtime
dataset, including its canonical twenty-entry job model. Reuse keeps each hash
and mtime unchanged. A workspace link to a separate temporary filesystem avoids
the actual ENOSPC encountered in the runtime stage; no further cleanup, format
change, build retry or performance/storage campaign is introduced.

The authentic read-only F09 checkpoint loads with the previous native cache and
new dataset. This single-frame result proves production attachment; it does not
establish positive ordinary use or full scene equivalence.

Generation `generation-GmxCy2`, dataset
`aa4588755ddad92406d9082335f96ca125e4c0ebad9e9bdcee0978ed06bbf97c`,
manifest `7ca7bef37999223b2475917c71c3cdbec164c5a6c5524c3119c2fa17a57356b7`.
Observed counters: `bus_reads=0 source_bytes=0 slots_used=0`.
All 304 active input hash/mtime guards and 41 compiled
input preimages remain exact, including the ROM/checkpoint/recording/native
cache/TinyCC/canonical executable and accepted runtime evidence.

Closed source/model/runtime and previous-family acceptances are not replayed.
The declared Bank120ordinary result is recorded below. Linux, USA, native
Windows and new32:9 remain pending. No fixture/navigation/private gameplay edit/Android/
package/assistant commit/push occurs. Private temporary artifacts can become
unavailable after cleanup; the recorded outcome remains separate from that
availability. Evidence: `target/rom-data-slot-job-lookup-cli/`.


## Ordinary Bank scene use of the twenty-slot job lookup

The existing EURbank checkpoint copy and its original Down8..20input are reused
for120frames,16:9/zoom0, with the actual e71production CLI and accepted dataset
`generation-GmxCy2`. The copy still matches its protected source checkpoint;
neither is rewritten, and cartridge saves are not selected. The prior A4negative/
B2positive observation remains closed; the new criterion is file-derived job
lookup use after the new owner was introduced.

The first new Bank120CLI call is positive:2bus reads/2owned job bytes/one slot
record. It selects the exact same input/state/dataset/frame range for one new
Wine fullpair. Only a five-line optional
`FFTA_STATE_TEST_REQUIRE_SLOT_JOB_LOOKUP` guard/report is added to the generic
[scene validator](../crates/recomp/src/rom_data/validation_tests.rs). Runtime,
production defaults, all previous guards, state formats and data remain intact.

One first changed Windows frontend test build passes warning-free, frozen
`f6c4291038785e3887677132e313a2a7b6f0a6313cf42180238c8d4f57553e03`.
The first exact Wine filter passes every120complete serialized Machine states
and guest clocks from frame3660through3780, central and wide framebuffers, OAM,
per-frame native/fallback dispatch and mixed/PSG/FIFOleft/FIFOright/HLE audio.
HLE floating samples are compared by bit pattern. Allfive actual sample counts
are `[263340, 263340, 31680, 31680, 263264]`. Execution is mixed; this is not an all-native claim.
The required new-owner report is2reads/2B/one slot, exactly matching the actual
CLI lead. The other19slot records retain their targeted source/runtime proof.

The CLI capture was inspected:284×160city/Bank exterior background; the bank
UI panels are absent from that raw framebuffer capture. No fully composed GUI,
whole actor/name/allocator ABI, all20ordinary slots or new32:9claim is inferred.
The fullpair compares the declared captured buffers and complete state.

All 308 active scene input hash/mtime guards and
41 compiled input preimages remain exact. The e71dataset,
checkpoint/copy/input/native cache/TinyCC/canonical executable and accepted
source/model/runtime/CLI evidence stay unchanged.

The declared ordinary child is closed. Source268-pin/model123-reader/managed/
native/CLI/F09/A4/previous scene acceptances are not replayed for their old
criteria. No new fixture/navigation/private gameplay edit/performance-storage
campaign/Android/package/assistant commit/push occurs. Linux, USA, native Windows,
new32:9 and the broader ROM TODO remain open. Evidence:
`target/rom-data-slot-job-lookup-ordinary/`.


## Live state helpers following the slot-job lookup

The exact EURentries `0D05E8`, `0D0618` and `0D0650` have a separate live-memory
classification. Their source code is authenticated independently of the preceding
job lookup; code adjacency is a discovery locator, not a data-table boundary.

- `0D05E8` returns1when any of the20first words at
  `02002E78 + 16 * index`, index0..19, is nonzero; otherwise it returns0.
  It stops at the first nonzero word. This tests nonnull words without validating
  pointed objects or proving actor liveness.
- `0D0618` counts nonzero first words of those same20live16-byte slots and
  returns0..20. It reads twenty4-byte windows, not the entire320-byte slot region.
- `0D0650` reads the live byte at `02001F68` and increments it only if its old
  value is at most98. Existing99..255values remain unchanged. No blanket clamp99
  or meaningful scalar return value is inferred.

All three ROMentry bodies ignore incoming arguments for these addresses. Their
six literal words supply fixed RAMbase/offset constants; those are executable
context support, with no referenced immutable ROMasset root in these helpers.
The mutable slot contents and counter remain guest state; exporting them into a
base resource would freeze live behavior. No new decoder/resource/owner or
revision is needed. The20loop bound does not establish the full physical extent
of the preceding ROMjob table or a global getter input limit.

68halfwords across three complete code blocks,30critical pins and six literal
words pass against the exact EURimage. Narrow even-Thumb BLencodings below370000
produce3/2/1caller candidates, retained as raw references; they are not assumed
to be every semantic caller or proof of ordinary reachability. The partial next
entry `0D0670` is outside this classification.

The source child is closed for these three entries. e71-d70/66files65resources
and USAe8-d7 remain unchanged. Existing source/model/runtime/CLI/Bank results stay
closed without replay. No build/game/test/fixture/navigation/private payload
edit/performance-storage campaign/Android/package/assistant commit/push occurs.
Static disassembly is not Linux runtime validation; Linux, USA, native Windows
and new32:9 remain pending. Evidence:
`target/rom-data-post-slot-lookup-readers/source-contract.json`.


## Isolated model for observed task parameter fragments

This e73 milestone describes the earlier two-phase model. The current e75
[phase-zero extension](#conditional-phase-zero-threshold-extension) below adds
phase0 and supersedes its current query/source-span counts; its old acceptance
remains closed.

`rom_data::observed_task_parameters` models two independent EUR fragments in
`ObservedTaskParameters`: two `PhaseThresholds` rows for phases1/2 at371065..67,
and four `VerticalCorrections` rows for indices4..7 at394E14..1C. Entries retain
their explicit phase/index, unsigned threshold or signed16 correction, plus
fixed source identity. Strict structural validation, JSON and source-byte
projection preserve all ten bytes over two disjoint spans. Exact EUR identity,
the two existing authenticated reader windows/literals and source hashes bind
the model; coherent edited fields fail source verification.

The original018DCA..18DF2 prefix increments the RAM counter first, compares its
OLD u16 value against the selected u8 threshold and advances/wraps the phase
only after that comparison. No duration units or global prelookup phase cap are
inferred. Phase0 and every other unmodeled byte remain outside this fragment.
Original0264EA..F8 shifts the task byte right once and subtracts a signed16
correction from projected r2. Both low-bit aliases are preserved. Earlier
indices0..3 retain the existing byte-property owner; indices8..127 remain outside
the model. The four-row fragment does not establish a physical-table extent or
new guest cap.

One first Windows frontend build passes without diagnostics; frozen hash
is8ce94211f4b4469de4e5a0471eb09969ede481ddf6de998d6cb157985aba8122. One reduced
Wine filter passes independent fields, all ten reconstructed bytes/two spans/
boundaries, strict JSON/negative-signed roundtrip and27structural, coherent,
unknown-field, range, source and truncation guards. Eighteen original prefixes
pass: ten phase1/2 cases with old counters0/7/8/9/65535, and eight task bytes8..15
for correction aliases and subtraction across signed/unsigned32 boundaries.
The expected RAM counter/phase changes, all other RAM, OAM/VRAM/palette and
callee/control registers agree with the model contract.

The private canonical model hash is
4b52c5f12f7633de0b1f790c5182ec5a727150a9460eb358eac3f41374788618.
All79selected input/frozen-executable guards and compiled implementations stay
exact. Both existing hubs differ only by module declarations. The production
extraction/owner pipeline, EUR e72-d71/66files65resources/current76OBJlists and
USAe8-d7 remain unchanged; these ten bytes are disjoint all current owners and
are not yet published. Completion helper retires zero paths.

The isolated model milestone is closed. Its later managed/runtime integration
is recorded below; actual CLI and ordinary new-owner checks remain pending. The closed source contract and two-
list/source/model/runtime/CLI/scene proofs are reused without replay. The452
lower audit bytes remain separate. Linux/USA/native Windows/new32:9 remain
unverified; there is no new fixture/navigation or performance/storage campaign.
Evidence: `target/rom-data-observed-task-parameters-model/`.


## Managed observed task parameters and typed bus consumption

EUR e73-d72 publishes `observed-task-parameters.json` as the bounded resource
`gameplay.observed-task-parameters`, format `observed-task-parameters-v1-json`.
The two independent field groups keep their established model and source
identities: two unsigned phase thresholds and four signed16 vertical corrections,
over exactly ten bytes/two spans. There are no fabricated foreign dependencies.
The generation now has67files/66resources; every65earlier file/record, the current
76OBJlists and the e72generation are retained. USA stays e8-d7.

The new immutable owner reconstructs only371065..67 and394E14..1C on candidate
pages371/394. It combines each byte with the preceding owner's result or raw
ROM, preserving phase0/other phase queries, old correction owners and unmodeled
indices. Original increment/OLD-counter compare/phase-wrap and signed-load/
subtraction instructions remain live. Guest timing, CAPI/native ABI, mutable
machine fields and save format do not change. Coverage counters distinguish
threshold/correction bytes and the two/four used rows; they are outside snapshots.

The first Windows build compiles the core but finds two omitted arguments in
historical frontend budget tests. Only those two zero arguments are corrected;
the archived failure and source preimage are retained. The incremental frontend
retry passes without diagnostics, frozen hash
1823c1ab2f14e02fc9e016a073c268e5ef71454a8c1b969fd75f46f04276d694.
Two first exact Wine filters pass. Managed checks cover interruption, create/
no-write reuse, repair of damaged/missing data, relocation,13transactional attach
guards and three full-set guards. Coherent edits to either field kind, malformed
or omitted rows, source plans/hashes/dependencies/revisions preserve the active
owner and complete guest state. Directory-unavailable Load/Undo/backup/restart
retain the owner; all ten bytes and six rows remain readable with raw altered.

Twenty-four original/interpreter/native prefixes have identical complete state
and guest clock with all ten raw source bytes altered. Cases include threshold
boundaries/old-u16 overflow/phase wrap, all eight low-bit correction aliases,
phase0/3/255 and correction inputs0/16/255. Both edges, all widths/three mirrors/
real-BIOS bus and six DMA/CpuSet/FastSet transfers pass. Mixed native API reads
combine2new+2old typed bytes even when both raw sources are altered, and2new+
2edited raw bytes around the phase fragment. Native API serves26typed bytes;
316original instructions/24native dispatches describe coverage only.

All67file/source checks pass. Dataset identity:
fe0839b7f50a40333d72318a573a05a85c9bb319a1f38c2060949022710ff207;
manifest SHA256ced88a054f0f999c4476605d886e3a2ba9b32b21aac96fc3bb198f87290c6bdd.
The published JSON exactly matches the model's4b52c5f12f7633de0b1f790c5182ec5a727150a9460eb358eac3f41374788618.
All74selected input/frozen-executable guards and compiled implementations stay
exact. Required completion retires only interrupted/moved fixture copies;
accepted offline data, current executable and logs/results/preimages remain.

Managed/runtime, actual production CLI and the declared ordinary12frame
comparison below are closed. Broader ordinary coverage remains separate. No completed model/source/ABI/scene oracle,
fixture/navigation or performance/storage campaign is repeated. Broader inputs,
full task/allocator ABI,452lower audit bytes and Linux/USA/native Windows/new32:9
remain separate or unverified. Evidence:
`target/rom-data-observed-task-parameters-runtime/`.


## Actual CLI for observed task parameters

CLI acceptance finishes on2026-10-27 using one warning-free Windows production
build, frozen hashc7f47bd4fdf6263845fdf0cfc3440b3e30fd79fc19a15079231d9417d4112f94.
The canonical executable's original hash/mtime are restored exactly; main source
content stays unchanged with mtime-only Cargo invalidation.

A user interruption leaves the first prepare without a current pointer or final
result. Read-only process/artifact checks confirm incompleteness; its generation
and log are retained in `aborted-cli-prepare`. The same frozen CLI then passes
create/reuse/oldcache-oneframe, all child exits0, from an unrelated directory.
All67generation files match the accepted runtime dataset, with69managed files,
66resources/two new spans10B/no dependencies. Reuse preserves every managed hash
and mtime. No accepted command or build is repeated to reconstruct context.

The first F09 frame reads one threshold byte/one phase and no corrections. Its
both-kind presence gate is explicitly negative and preserved. A new call at the
12frame bound from the retained source audit uses20reads/28typed bytes:
12threshold bytes/16correction bytes, both phases and all four correction rows.
It uses the same original checkpoint/input/cache and16:9. This establishes the
ordinary bound; it is not a complete-state equivalence result.

All145selected input/output/frozen-executable guards and implementation content
stay exact. The two userAtlas edits remain unchanged and unattributed. Artifact
completion retires zero paths and retains the interrupted preparation. Actual
CLI and the later12frame ordinary full-state pair are closed. Linux/USA/
native Windows/new32:9 and broader scenes stay open; no new fixture/navigation
or performance/storage campaign. Evidence:
`target/rom-data-observed-task-parameters-cli/`.


## Ordinary twelve-frame equivalence for observed task parameters

The declared ordinary child uses F09guest frames8476→8488 because the actual
CLI positively reaches both parameter kinds and all six rows at that12frame
bound. The first-frame correction-negative and interrupted preparation remain
closed evidence. Only five optional guard/report lines are added to the existing
scene validator; its frame loop is unchanged. The first affected Windows frontend
build passes without diagnostics, frozen hash
isde380af6bf577eafd28c2b7e3c0a87dacb22bf3dae53b922f8b29a95e3c68ae3.

One exact Wine filter with the current e73 CLI dataset, original checkpoint/
input/cache and16:9 passes every complete serialized Machine state and guest
clock, central/wide captured buffer, OAM, native/fallback dispatch and all mixed/
PSG/FIFO/bitwise-HLE audio. All five streams have actual sample counts
`[26334, 26334, 3168, 3168, 26258]`. The new guard requires both kinds/all six
rows. Its20reads/28source bytes/12threshold bytes/16correction bytes/2phases/4rows
exactly match CLI.88123native/279744fallback dispatches describe coverage only.

All76selected input/frozen-executable guards and original state/input/cache/
current67file dataset/canonical executable remain exact. The source preimage
confirms the exact five-line addition and no other implementation change; the
two userAtlas edits remain unchanged and unattributed. Required artifact
completion retires zero paths.

Source/model/managed/runtime/actual CLI and this declared ordinary child are
closed. The proof is bounded to twelve frames; broader scenes/indirect sources,
full task/allocator ABI, final composed GUI and new32:9 remain open. No prior
source/model/runtime/CLI/collector/scene acceptance, fixture/navigation or
performance/storage campaign is repeated.452lower audit bytes and Linux/USA/
native Windows remain separate or pending. Evidence:
`target/rom-data-observed-task-parameters-ordinary/`.


## Conditional phase-zero threshold extension

EUR e75-d74 extends the same `gameplay.observed-task-parameters` file with the
conditional phase0 byte at371064. The already authenticated original18DCA–18DF8
reader and18DF4 literal establish base371064; the three conditional phases0/1/2
have unsigned thresholds16/8/8. The source span grows from2B at371065 to3B at
371064. The earlier two rows and four signed correction rows are exact, giving
11source bytes in the same two independent spans3B/8B. Source SHA256 for the
three thresholds isee8357f0303dac80be96e10b1b49913c294c163771824fca6152057a6e1da957.
This neither establishes a prelookup global phase cap nor changes the original
OLD-u16-counter comparison, increment/store, wrap or outside-query behavior.

`PhaseThresholds::threshold` now supports0–2 and returnsNone for3–255. Runtime
coverage reserves three phase bits before the four correction bits, so phase0
and correction4 remain independent. The owner composes raw edges with allthree
file-derived bytes; the correction source/earlier owners and guest logic remain
unchanged. File format and count remain68files/67resources. All66other e74
resource records/files,80OBJlists and the earlier10parameter bytes are retained.
USA stays e8-d7; no Machine/CAPI/save/native-cache ABI changes.

One first Windows frontend build is warning-free. One Wine filter passes four
new original phase0 prefixes (old counters0/15/16/65535), seven new attach guards,
six original/interpreter/native complete-state-and-clock prefixes (four phase0,
two outside3/255), four explicit compiled phase0 bytes with raw371064 altered,
disjoint3phase/4correction coverage, all11typed bytes/raw edges and three DMA/
CpuSet/CpuFastSet full-state comparisons. Existing prefix helpers changed only
visibility; old1/2/vertical/constructor acceptance was not replayed.

One first Windows production build preserves the canonical executable's hash/
mtime and main content exactly. A single actual Wine CLI auto-prepare/runc call
uses the existing F09 snapshot/input/cache for a NEW28frame bound:8476–8504,
68generation/70managed files, all68equal the accepted dataset. The CLI child exits0
and uses36reads44B/28threshold16correction bytes/all3phases4rows. Its private log
matcher initially expected the explicit prepare-subcommand format; read-only
finalization recognizes `resources: created` for auto-prepare without rerunning
the command. These counts prove new phase0 use after the old1/2 wrap.

The same frontend executable then passes one Wine28frame fullpair: every
serialized state/clock, central/wide framebuffer, OAM, dispatch and all mixed/
PSG/FIFO-left/FIFO-right/HLE-bit audio comparisons. Actual samples are
[61446,61446,7392,7392,61370];207600native/650135fallback are coverage only. All
seven parameter rows are required; counts match the actual CLI exactly. The
optional five-line `FFTA_STATE_TEST_REQUIRE_PHASE_ZERO_THRESHOLD` guard leaves
the existing comparison loop byte-identical when removed. This is the newly
positive28frame criterion, not a repeat of the closed12frame milestone.

All215selected input/output/frozen hash-mtime guards and compiled source content
pass. Required artifact completion finds zero disposable paths. Dataset identity
eefc7e329ae9bb6104c63867e5ba6331f2466a41752e1237fce92757694aac86,
manifestb43670c8212648ee249874f261b10e4ce5bbaa01bcfeb102e0aa6d8b065dd351,
parameterJSON9c9de34b938d53b4a2fba244d615226b52e3f03d71e03ec0928b01db824a8bee.
No new gameplay fixture/navigation or performance/storage campaign belongs to
this phase-zero unit. Constructor-animation ordinary discovery remains a
separate partial child;371034signed-byte source, broader scenes/all-use/physical
archive extents and Linux/USA/native Windows/new32:9 remain open. Evidence:
`target/rom-data-phase-zero-threshold/` (`source-contract.json`,
`windows-phase-result.json`, `windows-cli-positive28-result.json`,
`windows-ordinary28-result.json`, `preservation-result.json`, `artifact-cleanup.json`).
