# European text and font resources

The broad offline model now covers all nine pointer-string selectors and all
29relative-archive selectors found in the declared original reader block,
across five language IDs. Current e12prepares/authenticates and directly consumes
the broad tenth file. The earlier overlapping ninth file stays authenticated;
the full catalog owns its reads and counters.

## Broad pointer catalogs and relative text archives

`text_catalogs::Catalog::from_rom` authenticates the exact EUR image and uses
the original reader literals recorded in `text_roots.rs`. The nine pointer
selectors `08016588..08016888` have767/753/107/725/92/512/52/128/84entries per
language. Every table's end meets the next independently selected payload
start; no pointer/header sweep defines the contract. Across five language IDs
there are16100distinct source strings,10735compact/5365extended and229314source
bytes. The model reuses the proven short-string byte/code representation and
preserves all45index tables and exact source pointer identity.

The29root selectors `080168e8..08017288` choose145relative archives. Original
`08013f8c` indexes little-endianu16offsets relative to the archive root, then
reads a16-bit program header. The first offset bounds the index table; the
final`ffff`is explicit sentinel metadata with no program. There are17035programs:
6138raw extended,8898custom-LZ extended and1999raw compact. Their exact source
extents total1766190bytes; alignment between programs remains outside those
extents. All33581pointer/archive/font provenance ranges are disjoint.

Program headers retain every bit. Bit0selects compact encoding; bit1selects
original custom LZ/RLE `080054a0`, shared with graphics. Program units retain
game glyph IDs and original command opcodes/operand bytes. Compact scripts use
byte glyph IDs and`ff`termination; extended scripts useBEglyph codes and zero
termination, with commands introduced by`40`. Command parsing follows original
`08014010`/`0801416c`: zero operand bytes remain inside commands, and command53
retains two embedded zero-terminated byte strings. Dynamic substitutions remain
commands; names/unit context/flags are not baked into extracted text.

Compressed programs retain their exact original encoded representation,
decoded units and any byte after the logical terminator.2012streams end at the
terminator and6886have one further decoded byte, retained without interpretation.
Validation reconstructs command/glyph bytes and checks them against bounded
custom decoding. It refuses truncated operands, invalid encoding/commands,
bad index order/sentinels, overlapping extents and encoded/typed mismatches.
The combined catalogs reference128glyph IDs1..486; complete128-byte packed
slots and variable metrics are retained without claiming Unicode or all possible
font slots. The earlier65glyphs remain independently covered.

Two new reduced Windows/Wine checks pass after warning-free release compilation:
a command/codec guard check and one exact-EUR catalog/original-reader oracle.
Strict deterministic JSON/source authentication matches independent retained
static classifiers for all pointers/indices/source extents/hashes, and rejects
coherent metric edits and malformed index metadata. Newly covered pointer
catalogs use80first/last boundary reader/copy cases, preserving output/guards;
the closed107-entry all-string oracle is not repeated. All145archive root
selectors match, and300archive cases cover every archive/language boundary,
three encoding/compression forms and all21command forms present in the source.
Original output/guards match a script reconstructed from decoded units; the
original uncompressed return adjustment above511bytes is checked separately.
All63new glyph slots/metrics match the original copier. Total968calls and
867138guest instructions are coverage counts, not performance measurements.

Dynamic command80requires a valid active unit/mission context. The retained
pub snapshot has none: index0underflows to`ffff`at original08013dd2, causing the
initial oracle to read outside its selected archive. The final fixture uses
the original nullable-context fallback with one-based index1for both original
and rebuilt programs. This is an explicit reader fixture, not an ordinary
gameplay/scene or full-state/clock-equivalence claim. The failed diagnostics
remain retained; decoder/model/bounds were not relaxed. Only the failed new
oracle was repeated, while the passed codec check remained closed.

Evidence: `target/rom-data-text-catalogs/reader-roots.json`, `pointer-catalogs.json`,
`archives.json`, original bounded disassemblies, `windows-core-archive-codec.log`,
`windows-results.json`, `windows-europe/result.json` and typed `text-catalogs.json`.
Original ROM/slot/cache/loader/canonical hashes remain preserved.
Broader ordinary scenes for this broad model,
all font slots/Unicode and other reader catalogs remain open. New Linux/USA/
native Windows checks remain pending, with no performance or space campaign.

## Broad managed catalog

EUR schema1/extractor12/decoder11publishes fixed tenth file `text-catalogs.json`,
resource `text.catalogs`, format `text-catalogs-v1-json`, with33581source ranges
and dependency `text.short-strings`. The complete typed base is compared against
the authenticated ROM before activation, including indices, headers, commands,
operands, encoded streams/padding and glyph words/metrics. Whole-set identity
includes the file. Earlier e11and all eight prior decoded files are retained
byte-for-byte; USA keeps six-file e8-d7. Guest/native/save identity is unchanged.

Readable JSON is retained without packing. Its malformed-input guard and the
whole dataset guard are512MiB, manifest16MiB. The first128MiB guard refused the
actual readable file before publication; it was widened, not used as a target
for compression or space optimization. Previous evidence is retained separately.
Three reduced Wine checks cover tenth-file deterministic publication/reuse,
interrupted-write/missing repair and the exact EUR source/whole-set identity,
coherent edits/metadata/path/coverage/dependency/truncated/oversized rejection,
corruption repair, folder move and transactional activation. Only the failed
real-EUR guard case was repeated; passed synthetic checks stayed closed.

Evidence: `target/rom-data-text-catalog-managed/windows-tenth-publication.log`,
`windows-atomic-write-repair.log`, `windows-results.json` and
`windows-europe/result.json`. At that managed-only boundary it was `verified-only`.
The actual production CLI prepare/no-write reuse and one-frame old-native-cache
activation also pass from an unrelated working directory; see
`windows-cli-summary.json`. The canonical executable is restored and all
protected originals/e11files remain unchanged. Managed publication alone did not
prove gameplay consumption; the direct unit below now closes that boundary.
Broader scenes and new Linux/USA/native Windows verification remain open.

## Broad direct consumption

The immutable owner retains the complete tenth-file model. Source bytes for
pointer tables, compact/extended strings, relative indices/`ffff`sentinels,
program headers, raw program glyph/command/operand bytes and font words/metrics
are reconstructed from typed fields. Compressed custom-LZ literal spans map
back to typed decoded program units/terminator/padding; exact size/token bytes
remain original file controls. Small unit offsets/literal maps hold locations,
not a second flat decoded script. Dynamic command resolution, original copy/
decode/render work, every guest instruction and clock stay unchanged.

Fixed declared EUR pages are`495..498`,`49a..49e`,`4a4`,`4f8..6b3`,`71e..766`.
Sorted authenticated bindings preserve previous-family or unmigrated bytes on
crossing reads. Eight separate source-kind counters cover pointer tables,
strings, relative indices, program headers, encoded controls, decoded program
bytes, glyphs and metrics. Under e12the earlier ninth-file subset is still
authenticated, but no second owner is created for its overlapping ranges:
it reports`verified-only`, while`text.catalogs`reports`decoded`.

Three reduced Wine checks pass after a warning-free Windows release test build:

- Synthetic source widths/mirrors/edges/prior-byte merging, zero operands,
  extended strings, glyph486/last font page, BIOS bus mode, SWI CpuSet, DMA and
  Load. Raw migrated ROM spans and duplicate retained encoded literal bytes
  are both altered; source reads still reproduce the exact original bytes.
- Fifteen original/interpreter/native chains cover five language IDs and raw
  extended/raw compact/custom-LZ forms, pointer strings, archive roots/copies,
  glyph486and variable/fixed metrics. All33581raw source ranges are altered
  after activation;90calls match complete serialized states and clocks after
  every call. Compiled read APIs consume60pointer-table,81string,30index,
 30header,396encoded-control,434decoded-program,1892glyph and18metric bytes.
  Original30840instructions/3093native dispatches are coverage counts only.
  The explicit valid nullable-context index1fixture is retained; this is not
  ordinary gameplay scene coverage or a repeat of the300-case offline oracle.
- With only temporary copied files made unavailable, failed activation preserves
  state/owner, and Load/Undo/backup/restart retain the same full model/counters.
  Subsequent typed pointer/string/index/header/program/font reads still work
  with migrated ROM source spans altered.

Evidence: `target/rom-data-text-catalog-consumption/windows-results.json`,
`windows-europe/result.json`, synthetic/native/lifecycle logs and source/protected
hashes. All jobs are terminal/pass; original ROM/slot/e12dataset/cache/loader/
canonical files stay intact. e12-d11, snapshots and native ABI/cache identity
are unchanged. The declared ordinary scene/CLI below now passes; broader scenes,
actual BIOS execution, complete reader/font/Unicode coverage and new Linux/USA/native
Windows checks remain open. No new recording, benchmark/profile/startup/RSS/
space measurement or old archive/scene oracle repetition.

## Ordinary transition and production CLI

One EUR/Wine legacy/decoded pair reuses the retained slot5LCD B, `pub.input`
397-frame transition and old native cache: entering the Cyril pub, BG62.
Every complete serialized state/clock, central/wide image, OAM/dispatch and
mixed/PSG/FIFO/HLE audio stream matches. The new family records2328reads and
8427source bytes:52pointer-table,185string,2relative-index,4header,27decoded
program,8064glyph and93metric bytes. Encoded-control use is zero: this scene
uses a raw archive program. The separate native04cases prove compressed use;
no compressed ordinary-scene claim is inferred.

A warning-free Windows production candidate is copied aside and the canonical
executable restored. One new decoded CLI command from an unrelated cwd reuses
e12without writes and the old native cache, reporting the same positive text/
font use and the retained legacy final image/dispatch row. The legacy command
is not repeated: protected ROM/slot/input/cache/loader and legacy behavior/ABI
remain identical; the new pair independently covers every state/audio sample.

Evidence: `target/rom-data-text-catalog-consumption/windows-scene-results.json`,
`windows-lcd-b-text-catalog-full-state-audio.log`, `windows-cli-summary.json`,
production build/candidate and source/protected hashes. All jobs terminal/pass;
original ROM/e12/slot/input/cache/loader/canonicals are unchanged. This closes
the declared reduced scene/CLI scope, not all gameplay or languages. Broader
scenes/GUI/archives, actual BIOS/complete font/readers/Unicode and new Linux/USA/
native Windows remain open. No new recording, performance/startup/RSS/space
measurement, old routine/catalog acceptance or extra scene campaign.

## Declared short-string catalog

`gba_core::rom_data::text_short::Catalog::from_rom` extracts the bounded EUR
catalog selected by original Thumb routine `08016648`. It authenticates the
complete EUR ROM before reading its five language tables. Managed preparation
now publishes it as a ninth fixed file under schema1/extractor11/decoder10;
startup authenticates it, and original CPU/DMA/native readers now consume it.

Each selector ID0..4has107entries. The independently read original literals
select pointer-table offsets `725c40`, `734598`, `741b9c`, `750230`, `75f318`.
Their string payloads begin at `7257f8`, `734134`, `7417a8`, `74fdec`, `75ee94`.
The next catalog's first string begins exactly428bytes after each table root,
which independently bounds the10732-bit little-endian cartridge pointers.
All535pointers and exact source extents are retained; padding before tables is
not claimed as text. Resource IDs follow first pointer occurrence, preserving
pointer identity rather than deduplicating equal contents.

Original `08019d98` defines two source encodings:

- Leading `01` selects compact strings. Each following nonzero byte expands to
  the big-endian glyph code `8000 | (byte - 1)`; zero terminates the source.
- Other strings use extended codes. A byte with bit7set consumes a second byte
  as one big-endian glyph code, including a possible zero low byte. Lower bytes
  remain exact control/operand bytes; a zero at that boundary terminates.

The typed model retains game glyph IDs and individual control bytes, plus the
source encoding. It can reconstruct both exact source bytes and the original
copy routine's expanded output. The declared535strings comprise478compact and
57extended sources, totaling5468bytes. They include `40 73` spacing and one
`40 3e 03` sequence; retaining these bytes does not claim a complete command
interpreter or Unicode mapping. Human language labels are not inferred from
selector IDs.

## Referenced font slots and metrics

Original `080135a4` computes `4954dc + glyph_id * 128` and copies the complete
128-byte packed4bpp slot through the initialized IWRAM callback at `03005ee9`
(ROM callback pointer `370eb4`). It then reads the variable width at
`4f885c + glyph_id`, or returns16in fixed-width mode. Original `08013bb4`
independently reads the same metric without copying. The mode is mutable RAM
at `020020e3`, not extracted data; rendering colors/palette remain dynamic.

The short catalog references65unique glyph IDs7..254. Only those65slots
(8320bytes) and65width bytes are extracted, in sorted glyph-ID order, as full
little-endian words and metrics. No pixel rearrangement, palette flattening,
new character interpretation or full-font extent is inferred. The bank limit
before the width root is an input guard, not proof that every possible slot is
used. Five index tables,535strings,65slots and65metrics produce670provenance
ranges with exact source hashes.

## Verification and remaining work

Two focused Windows/Wine checks pass after a warning-free release test build.
The synthetic codec check covers compact/extended paths, a zero low glyph byte,
control operands, truncation/length/encoding guards and pointer overflow.
The optional exact-EUR check source-authenticates a deterministic strict JSON
roundtrip, rejects unknown fields and coherent metric edits, and compares with
an independently retained static inventory.

It invokes the original selector for all535entries and the original copy
routine for every string, comparing all output bytes and destination guards.
All65font slots and both original width readers match in variable/fixed modes;
the original unknown-language fallback also matches. Total:1331calls and
89446guest instructions. These are coverage counts, not performance results.
The retained slot5only initializes the original IWRAM memcpy helper; no new
recording, boot, scene or full-state gameplay acceptance is claimed.

Local evidence: `target/rom-data-text-fonts/static-short-catalog.json`, bounded
disassemblies, `windows-results.json`, `windows-europe/result.json` and the
strict typed `windows-europe/short-texts.json`. Original ROM/slot/native cache/
loader/canonical executable hashes remain unchanged. The initial pre-guard
outputs are retained separately; final outputs include the pointer guard.

Ordinary gameplay scene proof remains open. Other short-text categories, relative-offset dialogue
archives, name keyboard producers, complete font coverage/control semantics,
Unicode and other languages/catalogs require separate contracts. New Linux,
USA and native Windows verification remain pending. No benchmark, startup/RSS
measurement, space optimization or AndroidPort change was performed.

## Managed publication and activation

EUR `v1/e11-d10` adds resource `text.short-strings`, fixed file `short-texts.json`,
format `short-texts-v1-json`, with670source ranges and no dependency on BG
descriptors. Complete-set verification compares the typed strings, indices,
glyph words and widths against extraction from the exact source ROM, refusing
coherent edits even after manifest/file hashes are updated. Dataset identity
includes the new file. The source ROM, guest snapshot and native-cache identity
remain unchanged. USA keeps its existing six-file `v1/e8-d7` contract.

Older e10generations and all seven prior decoded resource files remain intact.
The manifest input guard rises to1MiB to accommodate the new provenance, without
packing or space tuning. Invalid/missing/partial files are repaired by publishing
a fresh complete generation atomically; failed writes preserve the current
pointer and prior generation. Fixed relative names remain mandatory, and a
folder move reuses the generation without persistent absolute paths.

Three reduced Wine checks pass: deterministic nine-file publication/reuse,
partial-write failure/missing repair, and exact EUR source/whole-set identity,
coherent-edit/metadata/coverage/dependency/path/file-size guards, corruption
repair, move and transactional failed activation. The managed JSON equals the
closed offline02output; the old original string/font oracle was not repeated.
Actual Windows CLI prepare, no-write reuse and one-frame old-native-cache
activation from an unrelated working directory also pass. All protected
originals and retained e10files remain unchanged, and the canonical executable
is restored after the warning-free production build.

Evidence: `target/rom-data-text-short-managed/windows-results.json`,
`windows-europe/result.json`, `windows-cli-summary.json` and build/source/hash
manifests. Activation logs `runtime=verified-only`: source authentication and
file publication alone did not prove text/font consumption. The direct-consumer
unit below now closes that boundary. New Linux/USA/native Windows, ordinary
gameplay scenes and complete text/font coverage remain pending. No performance
measurements or old scene/oracle rerun.

## Direct consumption

The immutable runtime owner now retains the typed short-text/font model from
the authenticated ninth file. Exact table bytes come from stable string IDs
and source offsets; compact/extended string bytes come from the typed glyph
IDs and control bytes; packed glyph words and metric bytes come from their
decoded records. Small unit-position maps contain offsets, not duplicated raw
text. Original terminators, markers and big-endian codes remain exact.

Only fixed declared EUR source pages enter this projection: `495..49d`, `4f8`,
`725`, `734`, `741`, `74f..750`, `75e..75f`. Sorted authenticated bindings handle
mapped bytes; reads crossing a boundary preserve previous-family projections
or unmigrated adjacent ROM bytes. CPU, DMA, original BIOS bus mode and native
APIs share this source view. Separate table/string/glyph/width byte counters
identify use; no instruction, guest copy, render work or cycle is skipped.
e11-d10file formats, guest snapshots and native ABI/cache identity stay unchanged.

Three reduced Wine checks pass after warning-free Windows release test builds:

- A synthetic check covers compact/extended codes, zero low glyph bytes, all
  byte/halfword/word widths, ROM mirrors, boundary/prior-byte merging, last
  font/source pages, BIOS bus mode, SWI CpuSet, DMA and load-owner retention.
  Migrated raw ROM ranges are altered after attachment. BIOS bus mode is not
  a claim of actual BIOS-code execution.
- Five original/native reader chains, one per selector, alternate compact/
  extended sources and variable/fixed widths. With all670raw source spans
  altered after activation,20calls match complete serialized states and clocks.
  TinyCC covers original ROM and initialized IWRAM instructions; wrapped native
  read APIs observe20table,124string,636glyph and6metric bytes from typed data.
  Fixed-width cases correctly read no metric bytes. Total1984original traced
  instructions/920native dispatches are coverage counts, not performance data.
- Load/Undo, backup recovery and Restart retain the same owner and counters
  after only temporary copied dataset files become unavailable. Failed
  attachment preserves machine state/owner; subsequent reads of all four kinds
  still return typed values with raw ROM spans altered.

The first native probe had an overstrict test expectation requiring metric
reads even in fixed-width mode. Its states already matched; correcting that
expectation required no production-reader change. The failed log/object and
initial successful synthetic check remain retained; only the affected native
test was repeated, followed by the previously unrun lifecycle check. The
closed535-string offline and managed publication oracles were not repeated.

Evidence: `target/rom-data-text-short-consumption/windows-core-text.log`,
`windows-results.json`, `windows-europe/result.json`, native/lifecycle logs and
source/protected hashes. All jobs are terminal. Original ROM/slot/e11files/
cache/loader/canonical executable remain intact. Ordinary gameplay text scenes,
actual BIOS execution, broader catalogs/font/control/Unicode and new Linux/USA/
native Windows checks remain open. No new recording, production CLI/scene rerun,
performance measurement, packing or AndroidPort change.

## Complete font and name keyboard: partial offline implementation

The auxiliary caller inventory identifies a separate EUR name-entry archive at
`4f95c4`, used by original `0812e900` to construct three 120-byte pages of
big-endian selectable codes. Each page has six rows and ten storage columns;
the eighteen navigation row widths are at `3ad2fe`. The new `name_keyboard`
model retains relative indices, three raw extended programs, row widths and
derived cells, including the original space command and zero unused cells.

The archive references glyph3174, whose 128-byte source slot ends exactly at
the width root `4f885c`. The new `fonts` model therefore bounds the packed bank
at3175slots from `4954dc`, retaining all32little-endian words and the width byte
per slot. It does not infer Unicode or classify the bytes after those metrics.
Both models require the exact EUR source and retain source-range hashes.

This implementation is partial: the Windows release test binaries compile,
but the prepared reduced Wine checks have not run because the goal is paused.
The pending original-reader check covers only newly added font slots, keyboard
cell production and all selectors, reusing the retained initialized snapshot.
Source/original acceptance is unclaimed. Managed publication, direct runtime
consumption and ordinary name-entry scenes remain open; current e12 data and
runtime consumers are unchanged. New Linux/USA/native Windows checks remain
pending. No performance or space measurements were taken.

Evidence locators: `target/rom-data-text-auxiliary/static-entrypoints.json`,
`keyboard-archive.json`, and `target/rom-data-font-keyboard/windows-tests.json`.
