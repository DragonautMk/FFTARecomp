# Custom UI graphics resources

EUR schema1/extractor21/decoder20 introduced fixed `ui-graphics.json`, resource
`graphics.ui-custom-banks`, format `ui-custom-banks-v1-json`. The declared
catalog has nine raw BE-size custom LZ/RLE banks, ten independently rooted
literal/decode/upload paths, nine disjoint source ranges/57038encoded bytes
and84160decoded bytes. Eight unique sources feed BG VRAM; one source feeds
OBJ VRAM through two callers. Other indirect/custom sources remain outside
this declared catalog.

## Source contract

The earlier constant-count OBJ upload filter intentionally omitted callers
whose copy count derives from decoder r0. Original decoder `080054a0` returns
the declared decoded byte length. New callers shift/mask that result into
CpuSet/FastSet control, preserving full guest decoding and transfer work.
Bounded analysis reuses the73retained decoder callers, propagates only this
independently proved return value and checks both BG/OBJ VRAM destinations.
Its23transfer references/18streams include10references/nine sources absent
from the earlier39OBJbank catalog. New original caller disassemblies and
independent token/source/output hashes establish the selected scope.

The raw source offsets are `3bade0`, `43bc28`, `b6b434`, `b6d074`, `b6e848`,
`b6f9c0`, `b70ea8`, `b7233c` and `b7a9d0`. None overlaps any earlier resource.
The two banks atb70ea8/b7233care exactly adjacent; provenance retains separate
owners. Source43bc28produces2944bytes but original caller08ed9ccopies only544
bytes to BG VRAM. Other selected transfers copy the complete decoded extent,
including b7a9d0's25664bytes to OBJ VRAM. Decoder-derived counts, selected
scratch pointers and partial prefixes remain original instructions.

The model reuses `obj_graphics::Graphic`'s exact little-endian decoded u32words
and consumed encoded metadata. This shares the codec/storage contract, without
changing the older OBJ resource ID or file bytes. Internal mixed subregions
remain exact words; no universal pure-pixel/RGBA interpretation is invented.
Strict JSON and bounded decoding reject unknown fields, truncation/trailing
bytes, invalid history, wrong decoded words, ordering and overlap. Exact ROM
root/literal/BL validation and source equality reject coherent encoded/decoded
edits. The per-bank32KiB output/128KiB encoded guards and16MiB file-read guard
bound malformed inputs; no extracted-space acceptance budget is introduced.

## Managed and runtime behavior

All17files/16resources are authenticated before transactional attachment and
contribute to dataset identity. All15earlier decoded files remain byte-identical,
e20/older generations stay retained and USA remains e8-d7. Source bindings
project literal byte positions from typed decoded words; exact header/token
positions retain authenticated encoded metadata. CPU/DMA/native and guest-BIOS
bus reads use this source view, merging crossing reads with existing owners.
Every original decoder/count/scratch/copy instruction and clock remains active.
PPU reads live VRAM after those uploads; no added host ROM graphics consumer or
decompression shortcut is introduced. Nonserialized family and per-bank
source/literal counters prove ownership, preserving save/native identity and
ABI. Immutable ownership survives Load/Undo/backup/restart with files unavailable.

## Verification

One reduced EUR Wine original-source check passes independent source/output
hashes, strict deterministic JSON/source/bounds/coherent-edit rejection and
all ten actual decoder→upload paths. It verifies exact original r7consumed
cursor/r9produced count/r0returned length, source/destination/control registers,
full scratch output/guards and whole VRAM, including the544of2944byte partial
copy.1368457instructions are coverage only. The first new-source fixture seeded
r5as scratch at143f0cinstead of inheriting the preceding original mask literal
143f6c; that failed check is retained. The corrected fixture uses the actual
mask, and only that failed new-source oracle was repeated. No old bank oracle
was reopened.

Two focused EUR Wine integration tests pass publication interruption/create/
reuse/repair/move/e20retention, equality of all15previous decoded files,
coherent payload/missing-bank/source/hash/path guards and transactional rejection,
and unavailable-copy Load/Undo/backup/restart. All ten actual interpreter/native
paths with all nine raw spans altered match complete states/clocks;68906compiled
source bytes/39861literal bytes and every bank's positive counter prove typed
ownership. All source bytes/edges/widths/mirrors, real-BIOS bus, CpuSet/FastSet,
32typed DMA bytes and reads crossing adjacent new banks pass. Test and
production Windows builds are warning-free, canonical executable restored and
protected originals/e20data/cache/loader unchanged.

Actual CLI create/no-write reuse from unrelated cwd with old native cache
passes. The first retained320-frame Party→Unit replay reads0new bytes; it is
coverage-negative and is not used as an ordinary-use acceptance. Its final
image is an Equip/Abilities/Jobs submenu. The new bank43bc28preview instead
contains CLAN MBR/stat labels, and a bounded normal worldmapStart→Down→A input
opens Clan. Actual CLI replay320frames consumes1642source bytes/764typed
literal bytes in1579reads, all attributed to43bc28. One320-frame ordinary
Wine pair then matches every complete state/clock/central-wide image/OAM/
dispatch/audio sample with the same positive new-resource counters. Final
Clan status image is inspected; no RAM fixture edit, old source/native/lifecycle
or old-scene repeat was used to locate this new-family scene.

Private evidence stays under ignored `target/rom-data-return-length-graphics/`:
`new-ui-bank-inventory.json`, `caller-*.txt`, `windows-original-result.json`,
`windows-original-europe/result.json`, `windows-managed-results.json`,
`windows-managed-europe/*-result.json`, `windows-cli-summary.json`,
`windows-cli-clan.json`, `windows-scene-result.json`,
`windows-clan-ui-graphics-full-state-audio.log` and build logs. New Linux/USA/native Windows, other callers/scenes and broader GUI/archive/
mod phases remain pending. No performance/startup/RSS/space campaign,
AndroidPort change, commit or push is included.

## Localized UI selectors in e22

EUR schema1/extractor22/decoder21 extends the same file/resource to16banks,
two five-entry selector groups and two links to existing OBJ banks. The original
nine bank records and all15other decoded files stay identical; e21and earlier
generations remain retained. There are still17files/16resources. The UI resource
now explicitly depends on graphics.obj-custom-banks as well as OBJ dimensions;
linked banks must exist in the authenticated parsed OBJ resource.

Original Clan switch0808ed28selects43bc28/44d22c/45ed38/46fdc8/4818b8for
English/French/German/Italian/Spanish. It reads live language byte02001f6eand
uses English for values above4. Switch08143cbaselects
b7d9e4/b7e310/b7edfc/b7f874/b80140; it instead skips localized uploads for
values above4. Its5056byte CpuFastSet OBJ copy and2688byte CpuSet BG copy
exactly cover7744decoded bytes. The last two banks stay owned by the existing
OBJ resource; they are referenced rather than copied into UI JSON. English
Clan likewise reuses the existing43bc28record and its544byte partial upload.

The seven new banks own14396encoded bytes/35008decoded bytes. Ten4byte asset
pointer literals add40bytes, making17new disjoint ranges/14436bytes. The whole
UI catalog owns26ranges/71474source bytes and119168decoded bytes. Code jump
tables remain code; selector records authenticate only their asset pointers.
Exact root instruction/jump-target checks retain both original fallback rules.
Strict JSON, ordered/nonoverlapping sources, owner presence and source equality
reject coherent pointer/fallback/owner changes. Runtime typed pointer reads and
new-source/literal counters distinguish selector use from localized payload use.
Live language selection and every original decoder/copy/count/clock remain.

One reduced Wine source oracle passes14original selector paths(two groups,
values0..4/5/255), all seven new decoder→upload paths and isolated2688byte BG
tails for the two already proved shared OBJ banks. Exact source cursor, decoded
return length, transfer controls, scratch guards and whole VRAM match independent
inventory. It preserves all nine old records and verifies strict JSON/source/
selector/fallback/shared-owner guards. No old decoder campaign was repeated.

Two focused Wine integration checks pass publication interruption/create/reuse/
repair/move/e21retention, all15other decoded-file equality, coherent payload/
selector/fallback/shared alias/source/dependency/path transactional rejection,
and unavailable-copy Load/Undo/backup/restart. Native execution covers14selector
paths/seven new producers/two isolated shared tails with17new spans altered,
matching complete states/clocks. It consumes48compiled selector bytes,
14310new-source bytes/6449new literal bytes and every new bank positively.
All new bytes/edges/widths/mirrors/real-BIOS bus/CpuSet/FastSet and4typed DMA
selector bytes pass. Initial duplicate test-module compile failure is corrected.
Initial native fixture let chained full-producer blocks overshoot selector-only
stops; a separate selector-only table excludes those three stops. Failed evidence
is retained; only the failed native criterion is repeated, without runtime or
original-source proof changes. Test/production builds are warning-free and the
canonical executable is restored.

Actual Wine CLI create/no-write reuse/old-cache replay from unrelated cwd passes.
The first320-frame Clan locator uses only the prior English bank and0new
selector/localized bytes, retained as negative evidence with no full pair.
One bounded1200-frame cold-boot replay reuses existing demo input and positively
reads4selector bytes/2347new b7d9e4source bytes/1124new typed literal bytes.
One1200-frame ordinary Wine pair matches every complete state/clock/central-wide
image/OAM/dispatch and mixed/PSG/FIFO/HLE audio stream with the same positive
counters. Final intro-transition image is inspected. This English intro scene
proves new data use; other ordinary language scenes remain pending. No direct
RAM/language edit, new recording or closed old oracle repeat was used.

Private e22evidence stays under ignored target/rom-data-ui-localized:
localized-inventory.json, selector-*.txt, windows-original-result.json,
windows-original-europe/result.json, windows-managed-results.json,
windows-managed-europe/{managed-result.json,native-result.json},
windows-cli-summary.json, windows-cli-boot-intro.json, windows-scene-result.json,
windows-boot-intro-localized-full-state-audio.log and Windows build logs.
Protected original ROM/states/inputs/e21data/cache/loader/canonical stay intact.
New Linux/USA/native Windows, other indirect sources and wider TODO gates stay
pending. No performance/startup/RSS/space campaign or AndroidPort change.
