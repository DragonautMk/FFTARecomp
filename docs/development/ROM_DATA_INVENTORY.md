# ROM data inventory and contract

Status: ROMDATA-01, 21 October 2026. This is an inventory of verified entry
points and explicit unknowns, not a claim to have located every asset. Exact
EUR and USA images are identified by `ffta_rom::{EUROPE_SHA256,SHA256}`.
Offsets below are file offsets unless prefixed with a guest memory region.
Do not apply a regional offset to another image or infer language coverage
from a shared rendering format.

## First migration unit: graphics.obj-dimensions

The first family is the static OBJ dimension metadata, not sprite pixel data
or animation. Twelve records in shape-major, size-minor order contain two
little-endian u16 values, width and height in pixels. The range is 48 bytes,
uncompressed, with no interior pointers or dependencies. Invalid shape 3 is
outside the resource; guest memory beyond this table must keep its old behavior.

The USA draw routines at 0x080014c8 and 0x080016d4 read the table at
0x149510. The USA copied RAM sprite/visibility routines use a second table at
0xa398dc, copied to IWRAM 0x03006d28. EUR draw routines at 0x08001500 and
0x0800170c use the direct ROM table at0x14ced8; their host decoder's old IWRAM
selection was corrected against the original instructions during ROMDATA-04.
EUR also contains an identical table at0xd976e8 in the copied-code source area,
with the same relative source/RAM offset as the copied code. Both ranges are
mapped. The protected EUR boot-to-battle replay now observes actual alias reads
at frame5400 (primary first read541), with every full guest/RAM state matching
legacy reads through8476frames; see the
[validation protocol/results](ROM_DATA_VALIDATION.md). This is per-frame source
coverage, not attribution to a particular DMA/PC. An independent USA clean-boot
trace now reaches the original copier at0x0808f74c, frame791. Its own literals
at0x8f764/68/6c specify source0xa38adc..0xa3991c and IWRAM0x03005f28:
3648bytes, including the48byte alias at relative0xe00. Original CpuSet SWI
at0x0814186c performs24decoded halfword reads; all13initializer steps match
legacy full states. Independently inspected EUR entry0x080924e8 and literals
at0x92500/04/08 specify source0xd968e8..0xd97728 and the same destination;
the EUR replay above does not attribute execution to that entry. These blobs
include executable code, which remains in ROM; only48metadata bytes migrate.
Existing consumers and original execution are described in
[sprite routines](../ffta_sprite_routines.md) and implemented in
`ffta_sprites::{decode,decode_battle,visibility_result,ffta_sprite_interpret}`.
Both exact local regional images have been read independently: all four
48-byte ranges have SHA-256
`6ff886b23cde2726a4ceb9cba8c5569bbb4dc6a1185769547f78dfbcb7e2d991`.
Both copied aliases are checked independently, not assumed equal to the primary.
Exact-byte occurrence searches find two copies per region. Extractor revision2
records the newly discovered EUR alias; schema/decoder remain1.
The format is language independent; this does not validate localized assets.

Decoded files retain indexed dimensions rather than a rendered picture.
Host readers must preserve any genuinely changed RAM dimensions; a static
resource cannot replace dynamic RAM without checking the consumer contract.
CPU/DMA/interpreter/native source reads and typed host consumers now use the
decoded files with --rom-data. No original instruction or decompression is
skipped. [Runtime evidence](../rom_data_runtime.md) distinguishes conditional
guest reads from ordinary in-view boot/host use; broader gates remain05.

## Other families and migration matrix

Each entry names the known decoder/runtime reader, static versus dynamic
boundary, regional evidence, remaining extraction work and residual ROM use.
BG descriptor metadata and base palettes now have managed extraction and
direct consumption as detailed below. The bounded generic LZ77/packet decoders
remain groundwork for other linked graphics families.

- **BG tiles, palettes and image graphics.** `ppu.rs` reads hardware VRAM
  (4/8-bpp indexed tiles and bitmap modes) and palette RAM (BGR555). Those are
  dynamic destinations, not ROM asset boundaries. CPU copies, DMA and BIOS
  HLE feed these regions. `hle.rs` implements LZ77, Huffman and run-length
  decoding, but support for a codec does not locate a game's compressed blocks.
  New1200frame clean boots independently observe four LZ77 sources per region:
  USA56f4a8/5704cc/56fefc/56f52c and EUR76c64c/76d670/76d0a0/76c6d0.
  Consumed lengths125/133/1481/2507decode to160/456/1996/2960bytes, matching
  actual IWRAM3860/EWRAM3cb0post-SWI outputs on Linux and Wine. Original callers
  independently reference88byte descriptors atUSA569104/EUR7662a8; record0
  field0c resolves the first source. Fields04/08/10in record0point to outer
  wrappers containing the observed LZ77 streams. Typed halfword and indexed-pair
  packets now match actual regional writer instructions on Linux/Wine, including
  staged/chunked application. Literal pair syntax retains a destination guard
  after a contradictory original-code probe. The contiguous162-record descriptor
  block is now source-authenticated/typed, with full metadata roundtrip and
  original conditional-selector checks for all records on both targets. Relative
  sources, optional transfer triples/variant source and tail bytes are retained.
  All field04/08/10packet links now resolve352verified source programs per region,
  with wrapper/raw/LZ77/inheritance/dictionary extents and original-loader output
  equivalence for every162records. EUR main field00graphics now has50verified
  custom-LZ sources and direct typed file consumption. EUR transfer programs
  have42typed controls/34shared raw banks, eight-file managed publication and
  original tick/DMA evidence and direct CPU/DMA/native source consumption. All162
  variant pointers are absent in this declared EUR block. Other catalog bounds,
  new USA checks and language completeness remain unverified;
  this is not a complete graphics catalog.
  [Decoder contract, exact evidence and remaining work](../rom_data_graphics.md).
  Managed descriptor extraction/preparation: implemented, v1/e6-d5; bounded
  offline LZ77/packet decoders also tested. Authenticated BG descriptor reads
  now consume a file-derived session projection through CPU/DMA/native APIs,
  with independent counters, real-file regional provenance and native boot
  parity. Graphics payloads still reach VRAM/palette through original producers.
  The complete base-palette links now identify127unique sources per region
  (125LZ77,2raw), matching source/decoded hashes across regions and original
  producers for all162records on Linux/Wine. Preserve allu16bits, including
  bit15. Alternate environment streams now have a separate verified catalog:
  157valid keys in each of two regional tables,314streams/256Boutputs per region,
  custom LZ/RLE with BEsize header. All source/output hashes agree and original
  producers match on Linux/Wine. Five FFdescriptor keys are unavailable;
  caller preconditions remain unknown. Typed production decoding now matches
  retained outputs and new original synthetic/negative cases on Linux/Wine.
  Full banks, original links/keys and consumed source metadata are preserved;
  malformed input/overruns are rejected. Managed extraction/preparation now publishes and authenticates the fifth
  fixed file under e7-d6, retaining e6and all original source metadata. Actual
  runtime consumption of alternates now serves literal source bytes from
  decoded banks through CPU/DMA/native reads, with exact typed links/control
  metadata. Poisoned-source original/native and lifecycle checks pass; declared
  EUR LCD B/TV pub source-use scenes and native GUIs now pass on Linux/Wine.
  Broader scenes and global FFcaller preconditions remain separate. Base words now have typed readable extraction/preparation and direct04
  consumption: decoded banks for palette HLE, exact source views for CPU/DMA/
  native/guest BIOS reads, with original writes/cycles and state parity. Needed:
  indexed pixels, palette identity/indices, transparency, tile order and all
  writes/timing, not only screenshot similarity.
- **Terrain maps and movement planes.** `ffta_stage.rs` observes DMA and
  verifies two calculated 64x64 tile planes in EWRAM, then checks their VRAM
  crop. `ffta_movement.rs` observes a separate dynamic movement plane.
  `ffta_rom::map_producers_verified` independently pins producer code
  0x1af08..0x1b5d0 for each region; this is code, not a map archive. Static map
  descriptors, terrain geometry, pointer roots, compression and cross-map
  sharing remain unknown. Extractor: absent; decoder/runtime: dynamic plane
  observers and PPU. The calculated planes cannot be exported as static maps.
- **OBJ graphics, descriptor lists and animation.** `ffta_sprites.rs` decodes
  a signed/unsigned count followed by packed six-byte OBJ records, reads tile,
  priority/palette/flip fields and observes partial OAM DMA. Lists can live in
  ROM or RAM; animation, owner state, matrices and OAM are dynamic. Profiles
  and copied routine source ranges are independently hash gated per region in
  `ffta_rom.rs`; these are not a catalog of all lists or frames. A bounded EUR
  direct-literal catalog now identifies107references and66static uncompressed
  lists/160objects/1092source bytes. Typed signed coordinates and every packed
  attribute field reproduce both original EUR draws in1056Wine calls, including
  whole OAM buffers/matrices/guards. Managed e14-d13preparation and typed guest/
  native/host consumption pass focused Wine publication/source/native/lifecycle
  checks and one397-frame ordinary scene/actual CLI pair; see [the OBJ contract](../rom_data_obj.md). Other indirect/table
  roots, frame timing, sprite graphics/palettes and cross-resource links still
  need discovery. A further independently located EUR UI family has ten indexed
  OBJ/sequence bank pairs,167lists/284objects/95sequences/184frames,282consumed
  source ranges. Relative indices and u16durations match original constructors/
  scheduler body on Wine; e15-d14managed and typed native/host/lifecycle checks
  pass with all raw sources altered. Three empty sequence scheduling contracts
  remain open; one160-frame ordinary menu/actual CLI pair now proves positive
  index/list/frame/duration and host consumption with full-state/audio parity. Conservative control-flow
  discovery also locates sixteen additional references/seven new direct lists,
  now migrated under e16-d15: current static totals are 123 references/73 lists/
  173 objects/1184 source bytes. All original 66 records and other decoded files
  remain unchanged. Three focused Wine integration checks and one positive
  240-frame boot language-screen/actual CLI pair pass with new-list guest/host
  coverage. Other switch/indirect callers remain unclassified. New Linux/USA
  verification pending. EUR e17-d16now adds an independently rooted eleventh
  pair b4d8e8/b4f4e8: one list/two objects/five zero-count holds, eight new
  source ranges/58bytes. Explicit selectors and duration-source aliases retain
  the original nonrepeat contract and reuse typed header/index owners. All
  ten prior bank records remain unchanged; focused Wine original/read/native/
  host/lifecycle checks and one positive240-frame Party/actual CLI pair pass.
  The three earlier empty-sequence caller contracts and constructor-fed
  sources at38008/88328are now resolved: e18-d17adds five language-selected
  pairs from two five-entry pointer tables and one old-pair reference. New
 149lists/574objects/144frames/166ranges/4984Bretain original selector loads
  and duration256/counter-wrap semantics; current16pair catalog has317lists/
 105sequences/328frames. All11prior bank records/other files stay unchanged.
  Focused Wine source/original/native/host/lifecycle and a positive400-frame
  “Engage!”ordinary scene/actual CLI pair pass. The three older empty-record
  contracts remain open. Current e19-d18adds39custom OBJ-related word banks
  and six five-entry selector tables (13direct/nine bank roots plus six
  canonical indexed paths/thirty roots),45disjoint ranges/62674source B and
  131872decoded B. Source-authenticated literals/pointers feed original
  decoder/copy paths with live scratch transforms, partial uploads and RAM
  tails retained. Fifteenth obj-graphics.json leaves all earlier files intact;
  Wine source/managed/native/lifecycle and400-frame Engage parity/use pass.
  Other graphics/palettes and new Linux/USA verification remain open.
- **Raw UI palettes.** EUR e20-d19adds21source-authenticated raw banks/
 1328full-u16words/2656B/23direct-derived-Color-Mode references for both BG/OBJ
  palette-shadow halves. Original dispatcher145c14→copiedIWRAM03005ee8is
  independently initialized from176ROM bytes; actual selectors/copy paths,
  native poisoned-source/full-state/clock/bus/DMA and immutable lifecycle pass
  Wine, followed by a positive240-frame ordinary boot/CLI pair. This distinct
  sixteenth file leaves all14earlier decoded files unchanged. Original live
  fades/palette publication remain; indirect sources and new Linux/USA checks
  stay open. [Contract](../rom_data_ui_palettes.md).
- **Additional custom UI BG/OBJ graphics.** EUR e21-d20adds nine raw custom
  banks/ten checked literal original decode/upload paths,57038source B/
 84160decoded B with no earlier-resource overlap. Return-length/count propagation
  resolves callers omitted by the old constant-count OBJ filter. Eight sources
  feed BG and oneOBJ(two paths);43bc28uploads544of2944decoded bytes. Original
  source/native/lifecycle and one positive320-frame ordinary Clan/CLI Wine
  pair pass. Seventeenth file ui-graphics.json retains all15earlier decoded
  files/e20; original work/clock/VRAM remain live. Current e22-d21extends this
  same file to16banks/two language selector groups/two shared OBJ links,
 26ranges/71474B/119168decodedB. Seven new banks and ten pointer literals add
 17ranges/14436B; original code jump tables/language/fallbacks and5056OBJ/2688BG
  split remain. Reduced Wine original/native/lifecycle pass every new bank;
  all9oldUIrecords/other15decoded files/e21retained. One1200-frame English intro
  full-state/image/audio/CLI pair positively consumes4selectorB/2347new-sourceB/
 1124literalB. Other ordinary languages/indirect sources
  and new Linux/USA checks remain open. [Contract](../rom_data_ui_graphics.md).
- **Text and fonts.** `ffta_dialogue.rs` classifies already rendered layers
  and portraits, not strings, font encodings or localized text archives.
  `ffta_mouse.rs` identifies a name-entry row-count table via independently
  checked regional literals: EUR 0x3ad2fe (three pages, six bytes each), USA
  0x3a8320 (two pages). That table describes navigation widths, not character
  codes or text resource pointers. A bounded EUR short-string catalog now has
  five independently located selector tables with107entries each:535strings,
 478compact/57extended,5468source bytes; only its65referenced packed4bpp glyph
  slots/metrics are extracted. Original language/copy/font/width readers match
  every pointer/output/guard in two reduced Wine checks (1331calls). Ninth EUR
  managed file under1/e11-d10authenticates all670ranges; publication/reuse/
  repair/move/transactional activation and actual CLI pass Wine. Runtime now
  consumes typed indices/codes/glyph words/metrics through fixed CPU/DMA/native
  pages and separate counters, preserving original rendering and guest work.
  Five altered-source original/native chains (20calls/full states/clock equal),
  source/SWI/DMA and unavailable-copy lifecycle pass reduced Wine. Other short
  A broad offline model now covers all9pointer selectors/45tables/16100strings
  and29relative selectors/145archives/17035programs from the declared reader
  block, with128referenced glyphs and33581disjoint source ranges. Two reduced
  Wine checks match independent inventories and original pointer boundaries/
  archive roots/300script outputs/63new font slots. Dynamic commands retain
  operands; the controlled oracle supplies valid one-based context1, not a
  gameplay claim. Broad managed EUR1/e12-d11now adds a tenth file with33581
  source ranges, complete-set authentication and atomic prepare/reuse/repair/
  move/transactional activation plus actual CLI verified on Wine. e11/eight
  prior decoded files and USAe8remain unchanged. Broad runtime is still
  directly consumed via typed pointer/string/index/header/program/font fields,
  including decoded custom-LZ literals and all8source counters. Fifteen
  original/native chains with33581raw sources altered match90full states/clocks;
  synthetic encoded-literal alteration and unavailable-copy lifecycle pass Wine.
  Earlier overlapping subset is authenticated without duplicate ownership.
  One retained397-frame pub transition matches full states/images/audio with
  positive raw-program/font use; actual CLI reuse/old-cache parity passes Wine.
  The auxiliary archive at4f95c4now provides three typed name-keyboard programs,
  18navigation widths and180derived cells. Its last referenced glyph3174ends
  exactly at4f885c, bounding the complete3175-slot packed font/metric bank.
  Both files join managed EUR1/e13-d12and direct CPU/DMA/native typed consumption;
  the complete bank takes over earlier overlapping font sources. Source/original,
  managed provenance/repair/move, native altered-source/full-state/clock,
  lifecycle and one397-frame ordinary font transition/actual CLI pass on Wine.
  Keyboard ordinary scenes, other catalogs/control semantics/Unicode and new
  Linux/USA checks remain open. See the
  [text/font contract](../rom_data_text.md); five selector IDs are verified for
  this declared subset, not the complete localized text catalogs.
- **Gameplay tables.** Unit structures at EWRAM 0x020223a4, stride 0x90,
  are live state, not a static unit database; see
  [character metrics](../ffta_character_metric.md). Battle/shop mouse readers
  validate transient tasks/inventory. A bounded original shop/property chain now
  roots EUR item table6e0bd0..6e3ad0:376records/32bytes/12032B,19property field
  selectors, independent1..361/362..375enumeration bounds and following table
  root. Raw halfword/byte/signed fields and live02002030modifier branches are
  distinguished; all32record bytes roundtrip and no e22resource overlaps.
  Current e23-d22publishes eighteenth items.json/resource gameplay.item-records
  and directly consumes typed fields via original CPU/DMA/native logic, retaining
  all16previous decoded files/e22. Reduced Wine original14371+240isolated-negative
  and managed/native/lifecycle checks pass;86native paths with altered source
  match complete states/clocks. One positive320-frame Unit full-state/image/audio/
  CLI Wine pair uses40typed signed-field bytes. New Linux/USA checks remain pending. Following
 20byte table/item semantics/job/ability/law/AI catalogs still need contracts;
  live inventory/modifiers and secondary reads remain guest state/code/ROM.
  The following item-linked table now has a standalone typed source contract:
 225records/20bytes/4500B/count0..9/opaque/nine pairs,472active entries/eightFF,
  all376item owners checked,220referenced rows and independent next root6e4c64.
  Wine2244original cases/4synthetic queries plus8actualFFpredecessor gates pass.
  All unused bytes remain exact. EUR e24-d23now adds item-links.json with parsed
  item-owner dependency and typed CPU/DMA/native count/selector/value/opaque
  views. Every17old decoded file/e23retained; reduced Wine managed/lifecycle/
 62native paths with4500raw bytes altered match full states/clocks, including
  a typed crossing read of both adjacent owners. One positive560-frame ordinary
  headgear inventory full-state/image/audio/CLI Wine pair consumes574new bytes
 (287count/205selector/82value); first slots-only locator is negative, not rerun.
  New Linux/USA remain pending.
  The next table, 6e4c64..6e5464, has a lossless typed contract: 128 records
  of 16 bytes / 2048 bytes, a u16 alias plus 14 raw bytes, 23 checked LDR roots
  and an independently rooted following 52-byte table. One Wine source oracle
  passes 2581 original display/nonzero-alias/live-pair sum/choice/flag cases and
  19 synthetic edges, including original LCG state. EUR e25-d24 adds its fixed
  linked-metadata16.json resource: 20 files / 19 resources, retaining all 18
  previous decoded files and e24 generations. Two focused Wine integration
  checks pass 42 native paths with all new raw bytes altered and equal complete
  states/clocks, including adjacent typed ownership. One positive 520-frame
  Clan Mission Item full-state/image/audio/actual CLI pair reads 6 new display
  bytes. Other ordinary fields, broader field names and alias-zero caller
  preconditions remain unproved; new Linux/USA/native Windows checks are pending.
  A further source-only metadata52 catalog bounds 6e5464..6e6bf4: 116 records
  of 52 bytes / 6032 bytes, eight checked LDR roots, original 0..115 enumeration
  and independently rooted next table. Its 48 unsigned packed properties retain
  caller-alternate FF routing and nonreturning invalid/cyclic behavior. A new
  Wine source oracle passes 6369 real returning calls / 4 bounded loops and
  294 synthetic high-bit/redirect calls / 2 bounded loops, plus raw live-index
  tail readers and strict storage/source guards. Subsequent e26-d25 now adds
  linked-metadata52.json / gameplay.linked-metadata52, 21 files / 20 resources,
  retaining all 19 previous decoded files and e25 generations. Parsed normal
  link-owner guards and two Wine managed/lifecycle/native checks pass 75 paths
  with all 6032 new raw bytes altered and equal complete states/clocks, all-byte
  bus/SWI/DMA and adjacent metadata16/metadata52 typed crossing. One positive
  540-frame Change Jobs full-state/image/audio/actual CLI pair passes with
  664 new typed bytes; ordinary tail/opaque and new Linux/USA/native Windows
  checks remain unverified.
  The following four-byte source table now has a separate lossless contract:
  20 rows / 80 bytes at 6e6bf4..6e6c44, two selector/value pairs each, 26 active
  pairs and parsed metadata52 cross-owner indices. One focused Wine oracle
  passes 320 real / 24 isolated synthetic cases across both original count,
  contiguous gate, caller selection and live-count threshold prefixes. Counted
  nonzero slots and leading pairs stay distinct; unsigned values, signed
  comparison, tally wrapping and deeper live scans are preserved. Subsequent
  e27-d26 adds linked-metadata4.json / gameplay.linked-metadata4, 22 files /
  21 resources, all 20 previous decoded files/e26 retained. Parsed owner bounds,
  two Wine managed/lifecycle/native checks with 56 prefixes/all 80 raw bytes
  altered/equal full states-clocks and adjacent typed crossing pass. One positive
  540-frame Change Jobs full-state/image/audio/actual CLI pair reads 32 selector /
  6 value bytes. New Linux/USA/native Windows remain pending.
  Following metadata2 owns 44 key/value records / 88 bytes at
  6e6c44..6e6c9c, including real row43. Original direct u8 indexing reads all44;
  inverse u8 lookup scans only43 and returns first stored key / miss0. Parsed
  metadata52 key owners, independent next48Bsource, strict lossless source/JSON
  guards and one Wine348real+13synthetic oracle pass. Live byte7 wrapper prefix
  retains its frame; global caller bounds/semantic value names remain unproved.
  EUR e28-d27adds linked-metadata2.json/gameplay.linked-metadata2,23files/
  22resources; all21prior decoded files/e27retained. Two focused Wine managed/
  lifecycle/native checks pass21paths with88rawB altered/equal full states-clock,
  typed boundary/DMA/SWI. One positive1840Pub mission full-state/image/audio/
  actualCLI pair reads4value/0keyB; inverse key use stays targeted-only.
  Negative ChangeJobs/Rumors/Missions locator evidence remains closed.
  Default48owns6e6c9c..6e6ccc/48Bfrom exact constructor copy length,
  with structural byte/halfword fields and original signed-ID/selector/argument/
  ORflags patches. Mode1/2and four alternate IDs20..23stay distinct. One Wine
  oracle passes271real+30synthetic constructor prefixes after original IWRAM
  copy initialization; allocation/live-unit tails and four alternate192B/ROM
  pointer/following240B remain unowned. EUR e29-d28adds fixed default-constructor-
  template.json/resource gameplay.default-constructor-template,24files/23resources,
  all22previous decoded files/e28retained. Two focused Wine managed/native/lifecycle
  checks pass16paths/all48rawB altered/fullstates-clocks equal/IWRAMhelper and
  eight alternate/rejected paths0newreads. One397Pub full-state/image/audio/
  actualCLI pair reads96newBacross every structural field category.
  Static next contract separately roots four alternate48Brecords/192B at
  6eca78..6ecb38and pointer70cd58..70cd5c/4B,196Btotal/noe29overlap. Original
  gatedID20..23copy and five equipment-slot reads perrecord/parsed376item owners
  checked; byte01meaning/neighboring headers unclaimed. Strict typed4record/
  pointer catalog and one Wine24real+8synthetic original-copy/live-patch oracle
  pass source/JSON/fixedarray/order/ID/item-owner/stack/outerRAM guards. EUR e30-d29
  adds constructor-alternate-templates.json/gameplay.constructor-alternate-templates,
  25files/24resources/all23previous decoded files/e29retained. Two focused Wine
  managed/lifecycle/native checks pass14paths/both raw source spans (196Btotal) altered/equal
  fullstates-clocks, native pointer/all4recordsmask15 and5zero-new-use paths.
  One3600Lutia deployment full-state/image/audio/actualCLI pair reads45B(4pointer,
  41record20B/mask1); remaining records/full-copy behavior targeted-only. Closed
  source24+8/default271+30/native16/397proofs not repeated.
  Next static14Bcontract:93records/1302Bphysical6e6f6c..6e7482/logicalIDs2..94,
  original addressbias6e6f50/predicate>1, independent next12Bsource6e7484and
  unowned2Bgap. Exact reconstruction/noe30overlap/parsed field-range checks pass;
  byte02live-job link supported, byte01semantic owner unproved. Strict lossless
  model and one Wine504real+13synthetic getter/live-palette/initializer-prefix
  oracle pass exact source/JSON/bias/owner/stack/unit/palette/outerRAM guards.
  EUR e31-d30 adds linked-metadata14.json/gameplay.linked-metadata14,
  26 files/25 resources/all24prior decoded files/e30 retained. Two focused Wine
  managed/native/lifecycle checks pass37paths/raw1302Baltered/fullstate-clock
  equality; normal0/1no new source. One397Pub full-state/image/audio/actualCLI
  pair reads27B/two records/all four field categories. Separate6e6dbc4Broot/
  12Broot contracts, global bounds and newLinux/USA/nativeWindows remain open.
  Two distinct12Bindexed banks6e7484..6e78d4/92rows and6e78d4..6e7994/16rows
  now have a strict lossless source model:108rows/1296B/5184two-bit pairs.
  Shared135a1c reader uses full32row/signed-wrapping slot/u8mode; caller loops
  slots0..43, all48stored pairs retained. Next6e7994global root excluded.
  One reduced Wine source oracle passes1640real/10raw-outside-wrapping/384
  synthetic cases plus exact LDRB/address/source/JSON/callee/stack/RAM guards.
  EUR e32-d31 now adds linked-metadata12.json/gameplay.linked-metadata12,
  27files/26resources/all25earlier decoded files/e31 retained. Two focused Wine
  managed/native/lifecycle checks pass49paths/raw1296Baltered/fullstate-clock
  equality/bothbanks positive/three outside0newB and bank-boundary/SWI/DMA.
  One2200-Echo Grass confirmation full-state/image/audio/actualCLI pair is positive.
  Global bounds/flag semantics/global bank and newLinux/USA remain pending.
  The separately rooted global pair source6e7994has a minimum44pairs/11B
  source contract from original1377b4RAM-mask loop0..43/mode0. Threebytes
  before independently rooted6e79a2remain unowned; no12Browstride inferred.
  Strict fixed11×4groups and one Wine102real/10raw-wrap/91synthetic oracle
  pass wrapper/bothbits/live-mask/earlyexit/address/callee/stack/RAM/JSON/source
  guards. EUR e33-d32 now adds global-pair-flags.json/gameplay.global-pair-flags,
  28files/27resources/all26prior decoded files/e32retained. Two focused Wine
  managed/native/lifecycle checks pass28paths/raw11Baltered/fullstate-clock
  equality/newtypeduse/prior-typed and raw-tail SWI/DMA crossings. ActualCLI
  creation/no-write reuse/oldcache/cwd pass;2200EchoGrass0globalB is negative,
  no fullpair. Positiveordinary globaluse/fullpair/highmode/newLinux/USA
  remain pending, bounded descriptor/live-mask gates guide future selection.
  Evaluation parameters have a separate38Bsource contract:7divisor-weight
  terms/2conditional pairs at6e79a2..6e79b4and5class1..5threshold records at
  6e79c6..6e79da. First unsignedu8quotient/remaining6signed32/wrap/low8bonus
  predicates/originalLCG/signedlow16%256thresholds retain original code.
  Strict model and one Wine132actual-source/52mutated-source oracle pass,
  including12live-score prefixes and originalRNG/class0skip.14Bscoregap/4Brow0
  unowned; no32/24Bgap inference/foreignowner. EUR e34-d33 adds evaluation-
  parameters.json/gameplay.evaluation-parameters,29files/28resources/all27prior
  decoded files/e33 retained. Two focused Wine managed/native/lifecycle checks
  pass27prefixes/raw38Baltered/fullstate-clock equality/classmask31/class0zero/
  SWI-DMA/rawgap checks. One1840Pubmission full-state/image/audio/actualCLI
  pair reads15scoreB; thresholdordinary remains targeted-only. Whole tails/
  semanticlabels/newLinux/USA remain pending.
  Next static10Bcontract roots6e79da/*10/0default and normal index0..99from
  04B3E0..04B416,100records/1000Bthrough6e7dc2.12zeroBbeforeindependent6e7dce
  remain unowned; candidate101rows rejected. Eightu8deltas feed original live
  level/progress/flags through04870c; byte08feeds live itemprice discount,
  byte09structuralunused. Unsigned/signed/u8wrap/min-price/zero behavior and
  actual parsed Items base are distinguished; no storedforeignkey inferred.
  Exactroundtrip/noe34overlap/roots/stride/fields pass. Strict source model and
  one Windows/Wine oracle pass362real/68synthetic publisher/delta/full-price
  cases with actual parsed Items and complete EWRAM/SP/callee guards. Source
  work closed. EUR e35-d34adds30thfile/29thresource; all28old decoded files/
  e34retained. Two focused Wine managed/native/lifecycle checks pass27original/
  interpreter/native paths, raw1000Baltered/fullstate-clock equality, typed
  boundaries/mirrors/BIOS/SWI/DMA/coexistence. ActualCLI no-write reuse and
  one600Shop fullstate/image/audio pair read7discountB/record4; ordinary
  deltas targeted-only, newLinux/USA remain pending.
  Next static progression-gates roots6e7dce/stride12/index0..63,768Bto6e80ce;
  independent next root D23E6→D24A8. Byte00unused/halfword01return/byte03output/
  eight live unsigned minima and original u64mask order preserved structurally;
  row63zero return still marks bit63. Roundtrip/noe35overlap/opcodes pass,
  no new model/build/test, downstream receipt/FK/source/runtime/ordinaryuse
  and newLinux/USA remain pending.
  [Item contract/evidence](../rom_data_gameplay.md).
- **Audio samples, instruments, songs and sequencing.** `engine.rs` detects
  driver families; `mp2k.rs`, `gax.rs` and `rdrv.rs` shadow live voices while
  leaving original mixing/sequencing running. `MemView` resolves ROM/RAM
  ranges, and each driver has its own sample/loop/instrument contract. Driver
  detection is not an exhaustive instrument or song index. FFTA-specific bank
  roots, compressed/sample extents, sequencer commands, language variants and
  references remain to be verified per image. Extractor: absent; runtime:
  guest mixer/DMA and shadow samplers. A PCM export alone loses sequencing,
  pitch, envelopes, loops and original guest timing.
- **Executable code and opaque data.** Native labels and copied IWRAM
  instruction ranges are code, explicitly excluded from decoded assets.
  Unknown regions stay in the original ROM. Do not label a sweep of plausible
  compression headers as a verified asset catalog. Code/data interleaving,
  embedded pointers/constants and opaque regions prevent ROM-free startup.

## Version 1 base dataset contract

The managed set contains `graphics.obj-dimensions`, `graphics.bg-descriptors`
`graphics.bg-base-palettes`, `graphics.bg-alternate-palettes` and
`graphics.bg-packet-programs`. IDs remain semantic: OBJ index `shape * 4 + size`,
BG record index within the authenticated162-record block; source offsets are
provenance, not resource IDs. All five families have direct EUR consumption;
USA packets stay verified-only pending consumer validation. Whole-set activation
authenticates every file. Other linked payloads/programs remain ROM.

- `manifest.toml`: schema revision, extractor revision, decoder revision,
  exact source ROM SHA-256/size and regional profile; a coverage list naming
  only the decoded families; resource ID, relative file name, typed format,
  decoded-file SHA-256, dependency IDs and all original source ranges/hashes.
- `obj-dimensions.toml`: twelve typed width/height records in index order.
  The file hash covers its exact bytes. The decoder verifies dimensions,
  length, source ranges and that canonical little-endian encoding reproduces
  the source table. No absolute paths, timestamps, ROM bytes or host pointers
  appear in the manifest. Generated assets remain private local files.
- `bg-descriptors.json`: all162typed metadata records with exact source
  roundtrip, validated relative links/transfer triples and retained tail bytes.
  It does not contain linked graphics/program payloads. Its source range is
  independently authenticated for each profile; file hashes cannot authorize
  edited base semantics. All five resource files contribute to complete dataset identity.
- `bg-base-palettes.json`: all162record-indexed base palettes as fullu16banks,
  with source aliases preserved in162manifest ranges and a dependency on BG
  descriptor IDs. Known source codecs raw0/LZ7710are bounded and exact extents
  exclude padding. Preserve bit15and bank/index order; alternate environment
  programs use the separate verified alternate family. Exact consumed source bytes are
  retained beside banks for compatibility reads; both forms must agree.
  Decoded banks feed verified LZ77 HLE while raw CpuSet uses file-derived reads.
- `bg-alternate-palettes.json`: two157-key catalogs of eight16-wordu16banks,
 162descriptor keys including five unavailableFF, original relativeu16links
  and exact consumed custom-codec metadata. Two314Blink tables plus314streams
  retain their own manifest ranges/hashes and a dependency on descriptors.
  All fields and file hashes are authenticated; current activation verifies this
  family and its guest source reads now consume decoded literal bank bytes.
- `bg-packet-programs.json`:352unique typed programs/486descriptor links,
  parent graph, exact encoded and post-wrapper body metadata plus ordered writes.
 352source ranges depend on BG metadata. Every field/file hash is authenticated,
  with source-equal e8publication/repair/retention verified for EUR/Wine. Current
  EUR runtime consumption now retains typed values for source literals and
  predecoded HLE body writes, preserving original dynamic fill and guest timing.
  USA stays verified-only. [Contract](../rom_data_graphics.md#direct-european-packet-consumption).
- EUR `bg-main-graphics.json`:50typed graphics resources/162links, exact encoded
  metadata, destination offsets and full little-endian words. Fixed source views
  serve original CPU/DMA/native readers; literal bytes come from decoded words.
- EUR `bg-transfer-programs.json`:42controls/34shared raw tile banks/97active
  links and162absent variants,76authenticated source ranges. Current e10-d9
  publication validates the whole set and includes every file hash in identity;
  this transfer family now serves typed control/tile bytes through existing
  CPU/DMA/native readers without replacing live animation state or guest timing.
- Schema and decoder identity are separate from source identity and compiled
  code identity. For a base dataset, matching its own declared hashes is
  insufficient: verify the expected source ranges and reconstructed source
  digest against the selected exact image. User edits belong to a future mod
  layer; do not silently accept them as base data.
- Keep fixed names and bounded input:256KiB manifest (including162base and316alternate
  provenance entries),16KiB OBJ,256KiB BG descriptors,1MiB base palettes,
 4MiBalternate palettes,64MiBpacket programs and64MiBtotal. These guard malformed input, not extracted-data space targets.
  Reject unsupported schema/decoder revisions, duplicate/missing resources,
  bad hashes, unknown fields, truncated/out-of-range data and path escapes.
  Publication must validate all files before making a directory visible.
- Automatic path: portable `Data/resources/<rom-sha256>/v1/e8-d7/`;
  installed uses local application data `FFTARecomp/resources`. Immutable
  `generation-*` directories contain six files; an atomic relative `current`
  file selects one verified generation. Revisions have separate paths and an OS
  lock coordinates writers. `rom-data prepare` and opt-in `play --rom-data`
  implement reuse/recovery, retaining e1/e2/e3/e4/e5/e6/e7 generations; direct OBJ, BG
  metadata and base-palette consumption is opt-in. Other linked payloads/programs
  remain ROM.
  Normal enablement awaits05.
  Native caches and saves remain separate.
- Decoded base data are immutable host resources, shared for a session and
  rebuilt/rebound on load/Undo/clean-boot restore. Do not serialize their
  pointers or duplicate files in `.ffstate`. Existing ROM/BIOS identity remains
  valid only after strict base equivalence is proven. Modified data require
  a separate content identity and compatibility policy before enabling mods.

## Gates before normal runtime use

ROMDATA-02 must first validate synthetic malformed inputs and both local
regional images. ROMDATA-03 needs preparation/reuse/recovery/move tests.
ROMDATA-04 must trace every consumer, demonstrate decoded-file provenance
and make fallback visible. A host-only migration does not close guest reads.
ROMDATA-05 needs deterministic state/image/audio equality, lifecycle checks
and measured startup/memory/frame cost against the existing in-memory ROM.
No speedup, removal of the ROM dependency, complete graphics migration or mod
support follows from this first family.

### Alternate palette mode naming and caller scope

The original EUR Options UI and unchanged-frame snapshot reads establish
LCD A=0, LCD B=1(second alternate catalog), TV=2(first). Only Save changes Yes
commits the candidate. Four direct Thumb callers per region and the original
setter/confirmation are inventoried; no local descriptor-FF guard was found.
Global FFreachability is unresolved. See the [bounded caller contract](../rom_data_graphics.md#alternate-palette-callers-and-color-mode).
Worldmap/Party mode changes alone produced0alternate reads and cannot count as
actual alternate-palette scene coverage.
The subsequent Cyril-pub BG62transition does consume both catalogs: declared
Linux/Wine LCD B/TV full-state/audio and GUI checks pass. See the [scene results](ROM_DATA_VALIDATION.md#alternate-palette-e7-scene-and-gui-acceptance).

### Descriptor-linked packet programs

The declared field04/08/10catalog has122paired/117halfword/113auxiliary unique
sources per exact region:352disjoint ranges,398227encoded bytes,523794decoded
body bytes. Four-byte wrapper01/11selects raw/LZ77body; upper24bits are a parent
descriptor ID or FFFFFFinitial fill. All aliases/content/parent graphs match
EUR/USA, no cycles, maximum2nodes. Paired fill is live EWRAM02007f5e; other two
families fill zero. Preserve this initialization and ordered writes.
Four Linux/Wine original-loader cases each pass486wrapper invocations with
complete destination/guards/SP checks. The core's new referenced_extent bounds
raw dictionary reads separately from stream termination. See the [complete
catalog contract](../rom_data_graphics.md#complete-descriptor-packet-catalogs).
The sixth managed e8file authenticates these programs and now serves their
actual runtime source/body reads for EUR/Wine. Field00and transfer programs
have separate verified EUR contracts below; broader catalogs remain open.

### Typed packet resource model: EUR priority

The bounded offline packet model now retains352unique programs and486descriptor
aliases, ordered u16/u32writes, original parent graph and exact encoded/body
metadata. Source authentication rejects coherent base edits. Linux/Wine EUR
checks reuse all retained original bodies; readable JSON is byte-identical.
The new model is verified for EUR only; USA remains pending by user instruction.
See the [typed contract](../rom_data_graphics.md#typed-descriptor-packet-programs).
The file is published/verified in the e8-d7managed dataset and consumed by the
EUR CPU/DMA/native/HLE paths. Three focused Wine checks and one397frame actual
native transition pass with positive typed consumption and complete-state parity.
New Linux/USA checks remain pending. See the [consumer contract](../rom_data_graphics.md#direct-european-packet-consumption).

### Main graphics field00: verified EUR offline catalog

The162descriptor links name50disjoint sources:23tag20,27tag22; exact consumed
sizes total621923bytes and decoded outputs835232bytes. Staged producer0801b688
phase1selects custom BE-size LZ/RLE080054a0, the same original decoder used by
alternate palettes. These are not BIOS Huffman streams. Tag20's upper24bits
declare output bytes; tag22's upper24bits are a byte destination offset followed
by a nested20header and the same custom stream. Inner/outer sizes agree.
All outputs/offsets are whole32-byte tiles and fit the original0x4fe0-byte
graphics destination. Phase1writes the compact output to EWRAM020091a0;
phases2/3queue DMA32 transfers of each half to target+offset(+half for phase3).
Tag22does not supply or replace the live destination prefix.
The normal producer at0801b2cc reads field00at0801b310..324 but branches for
12/02/10and a copy case; it alone does not prove how20/22are prepared.
The separate palette-copy indirect pointer atROM370eb4 selects IWRAM03005ee9;
08145c14is only the bxr3trampoline. It is not an independently located main
graphics decoder.

ROMDATA-01-BG-MAIN-GRAPHICS / ROMDATA-02-BG-GRAPHICS-DATA are complete for
this declared EUR catalog. The bounded typed model retains little-endian words,
offsets, exact compressed metadata and all alias links; canonical IDs follow
first descriptor occurrence. Validation rejects truncations, invalid references,
overruns, overlap, wrong IDs, size mismatch and coherent source edits.
Nine focused Wine checks pass, including50original staged decoded outputs with
exact live source cursors, full destination/guards/SP, and phase2/3DMA layout for
one20and one22source:54calls/11142126instructions. Readable JSON roundtrip and
base authentication pass. Evidence:target/rom-data-bg-main-graphics/
windows-europe/result.json, windows-results.json and bounded disassemblies.
Managed EUR e9publication and direct file-backed runtime consumption are now
complete for the declared Wine scope: retained words/metadata/literal maps,
fixed769..895source pages, original guest instructions/clock and separate
positive-use counters. Four poisoned-source original/native cases and owner
lifecycle pass, followed by one397frame full-state/image/audio pub transition
and actual CLI parity/no-write reuse. New Linux/USA verification remains pending.
[Current consumer contract](../rom_data_graphics.md#direct-european-main-graphics-consumption).


### EUR descriptor transfer and variant programs

The five transfer slots have83/6/5/3/0active links across162descriptors:
97links,42control sources and34shared raw tile banks. All162variant links are
zero in this verified block; no absent pointer is dereferenced or treated as
an invented fallback.76disjoint source ranges total968control/199296tile bytes
and do not overlap existing e9resource ranges. Controls store u16transfer byte
length, u8frame count plus retained byte3, and ordered u16duration/source-tile
pairs. Source offsets are tiles of32bytes. Frame/tick/DMA progress is live RAM,
not extracted state. Tile banks retain full little-endian u32words and exact
maximum referenced extents across shared aliases. Independent stable IDs follow
first descriptor/slot occurrence; typed links preserve VRAM destinations.

Original creator08021f10, updater08021bd8and dispatcher08021d10confirm the
contract in four reduced EUR/Wine animation cases, using retained slot5only for
original IWRAM helper initialization. Two loops plus boundary match frame
countdowns/order and complete VRAM after every DMA chunk, including34partial
chunks with the original1024-byte limit:634calls/80766instructions. Five focused
Wine checks pass (3core/1JSON/1private), source/JSON authentication and all97links
match the independent static classifier. New Linux/USA checks remain pending.
Managed transfer publication is now complete for EUR/Wine under1/e10-d9,
with the eighth fixed JSON file and all76ranges. Four focused Wine checks and
three actual CLI commands pass authentication/publication/reuse/repair/move/
retention and transactional activation. Direct control/tile source consumption
now passes reduced EUR/Wine provenance/lifecycle and declared397frame scene/CLI
proof: all76raw spans poisoned in two original/native cases, full states/clocks
equal after60calls,66compiled controlB/9984native-DMA tileB. The actual pub scene
uses1203controlB/21504tileB with every state/image/audio sample equal. New Linux/
USA and broader scene/GUI/archive checks remain pending.
[Contract and evidence](../rom_data_graphics.md#european-background-transfer-programs).
