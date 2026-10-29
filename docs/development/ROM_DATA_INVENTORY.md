# ROM data inventory and contract

Status: ROMDATA-01, 24 October 2026. This is an inventory of verified entry
points and explicit unknowns, not a claim to have located every asset. Exact
EUR and USA images are identified by `ffta_rom::{EUROPE_SHA256,SHA256}`.
Offsets below are file offsets unless prefixed with a guest memory region.
Do not apply a regional offset to another image or infer language coverage
from a shared rendering format.

The [overall ROM coverage map](ROM_COVERAGE.md) provides an offset atlas and
reproducible metadata-only JSON/HTML following the committed EUR extractor.
Dataset generation and historical ordinary-proof review are independent; new
resources appear with review pending, without freezing regeneration. It
separates speculative code, authenticated data, typed runtime availability,
ordinary positive resource use, shared references and unclassified regions.
Overlapping layers are not additive; ordinary distinct-read-byte coverage is
explicitly unavailable rather than inferred from aggregate read counters.

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
  0x1af08..0x1b5d0 for each region; this is code, not a map archive. New EUR
  static dataflow ties staged loader1B688/state6→1AE88 to the existing
  root7662A8/162descriptors and all486field04/08/10links into352packet sources.
  Extractor/typed decoder/runtime already use `graphics.bg-descriptors` and
  `graphics.bg-packet-programs`; no duplicate terrain resource is needed.
  Auxiliary patches initialize the normal16×16height/flag grid02007CB0,
  assigned to live context02007F10+4/width+8/height+9. Original accessors read
  heights and mutate flags, so static seeds do not replace live cells or
  calculated EWRAM/VRAM/movement planes. Forty instruction/11literal/nineBL
  fingerprints and exact descriptor/source/ownership checks pass without
  guest execution. Other context paths, ten-byte accessor1DFB4 and complete
  flag semantics remain unclassified. Separate raster palette root4872B0now
  has a bounded source contract:32records/324selector links/33ranges/10496B,
  64index192header10240colorB, no overlap with the49prior owners. Original161halfword
  copy reads a neighboring header or existing alternate-palette owner at
  489BB0, then overwrites the extra destination word; preserve all bus/timing
  behavior without duplicating that source. Strict lossless typed model and
  one Wine original-reader/copy oracle now pass all10496B/324links/15guards,
  64initializers32getters/11rawgetters7rawprefixes3disabled/36full322Bcopies,
  deferred/explicit-source/queuedDMA and synthetic bit15/range words. Whole
  RAMoutside stack/SP/callee checks pass;4753instructions202PCare coverage only.
  Managed e55-d54now publishes51files50resources/all49prior decoded/e54retained,
  actualBGdescriptor dependency and one immutable typed owner. Two new Wine
  filters firstPASS17guards/lifecycle/all10496B and28original-interpreter-native
  complete-state-clock paths, compiled[15index46header24color]B, original copies/
  wrap/queuedDMA/rawneighbors and mixed alternate owner. Widths/mirrors/BIOS
  bus-mode/SWI/DMA checks pass;646guards. ActualproductionCLIcreate/reuse-no-write
  53managedfiles/all51generation equality/oldcache and one1500frame genuine
  coldboot Winefullstate-clock-image-OAM-dispatch-audio pair nowpass. Ordinary
  50880aggregateB=[2index322header50556color],mask2source records includes
  neighbor-header touches; not two full palettes or distinctB.701guards pass.
  Other contexts/newLinux-USA-nativeWindows/new32:9 remain pending.
  [Source and ordinary contracts](../rom_data_maps.md).
- **UI choice lists.** [The bounded EUR contract](../rom_data_ui.md) identifies
  twelve immutable lists/36entries from16direct widget8BBF0callers plus a six-
  entry397A9Ctemplate used by two live filters. Thirteen ranges/42entries/336B
  are disjoint50owners; all18root literals/93instruction fingerprints/static
  byte reconstruction pass. Eightu8fields retain ID/X/Y/opaque and four raw
  navigation values. Live0200F250filter/copy/overrides and unmasked flagged
  coordinate addressing stay original; no guest count/target guard. Strict lossless
  model/shared source plan and one focused Wine original copy/getter/template
  oracle now pass17guards/all336original-loadsiteB/18copy prefixes/44getters/
  43idle calls/64filter prefixes/2raw-coordinate prefixes/703hash-mtime guards;
  warning-free Windows test build. No fullwidget/native/fullstate-clock claim.
  Managed e56-d55now publishes52files51resources/all50prior decoded/e55retained,
  noassetdependency/fullsetcoherence/USAe8-d7 unchanged. Two reduced Wine checks
  pass17guards/lifecycle/all336typedB/13lists and29original-interpreter-native
  complete-state-clock paths/compiled[92ID183coordinate92opaque368nav]aggregateB,
  widths/mirrors/BIOS-SWI-DMA/oldOBJ-raw-new/706guards. Original RAMfilters/input/
  cursors/counts remain live; no guest caps. ActualCLIcreate/reuse52generation/
  54managedfiles no-write/oldcache and one320Party fullstate-clock-image-OAM-
  dispatch-audio pair now pass32aggregateB=[4ID8coordinate4opaque16nav]/1list/
  769guards. Two negative battle contexts remain closed/no pairs; other12lists'
  ordinary use pending, no wholewidget/new32:9 claim. Existing fixtures reused. New Linux/USA/native Windows checks pending.

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
  pass with all raw sources altered. Three empty sequence direct-root domains are now classified below;
  their indirect/live scheduling remains unverified; one160-frame ordinary menu/actual CLI pair now proves positive
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
  Constructor-fed sources at38008/88328are now resolved: e18-d17adds five language-selected
  pairs from two five-entry pointer tables and one old-pair reference. New
 149lists/574objects/144frames/166ranges/4984Bretain original selector loads
  and duration256/counter-wrap semantics; current16pair catalog has317lists/
 105sequences/328frames. All11prior bank records/other files stay unchanged.
  Focused Wine source/original/native/host/lifecycle and a positive400-frame
  “Engage!”ordinary scene/actual CLI pair pass. The three older empty-record direct-root domains are classified in
  [the OBJ contract](../rom_data_obj.md#earlier-empty-sequence-caller-domains):
  twelve calls select0; eighteen initializer mode producers select0..3 and
  two updates select2/3, excluding the empty indices. The unproved hold
  extension is rejected; indirect/live mutation remains unverified. All thirty
  root literals/47instruction pins/50owners pass static checks. Six possible
  selector bytes stay raw; duration aliases reuse existing indices8/16/4. No
  resource/model/revision change or guestcap follows. Current e19-d18adds39custom OBJ-related word banks
  and six five-entry selector tables (13direct/nine bank roots plus six
  canonical indexed paths/thirty roots),45disjoint ranges/62674source B and
  131872decoded B. Source-authenticated literals/pointers feed original
  decoder/copy paths with live scratch transforms, partial uploads and RAM
  tails retained. Fifteenth obj-graphics.json leaves all earlier files intact;
  Wine source/managed/native/lifecycle and400-frame Engage parity/use pass.
  Other graphics/palettes and new Linux/USA verification remain open.
- **Normal unit OBJ initial pose.** The original
  2D394→233A8→CF144→22788 path supplies fullword animation0/part0. Existing
  metadata52 property4/metadata14 halfword04 project121IDs through394834,
  first pointer cells and121counted sequences/477operation1frames. Six
  OAMlists/31objects atA50020+relativeoffset determine333raw4bpp requests,
  135168B/4224decoded indexed tiles at898A40+relativeoffset. 00B14 is a RAM
  DMA queue constructor, not a codec;00800 executes realDMA3. The strict
  typed model/sourceplan owns415spans146358B, disjoint all51e56owners, with
  actual existing producer dependencies and opaque bytes preserved. Static
  94instruction/13BL/6literal fingerprints and one new Wine source-model
  oracle pass121originalreset paths/oneconstructorprefix/477scheduler-DMA
  uploads/allpixelB/16shape-sizeprobes/11guards.1797reserved/OAM/tailB remain
  preserved but unread in those source probes. EUR e57-d56 now publishes53files/
  52resources/all51prior decoded/resource records/e56retained/USAe8-d7 with
  actual metadata52/14dependencies. Two new Wine filters pass17guards/lifecycle/
  all146358typedB/30original-interpreter-compiled complete-state-clock paths,
  rawID/animation/liveRAMframe, original scheduler/DMA and415edges/mirrors/BIOS/
  SWI-DMA/new-raw-oldOBJmerging. Actual productionCLIcreate/reuse-no-write55files/
  all53generationfiles and one existing Party-unit320fullstate-clock-image-OAM-
  dispatch-allaudio pair pass10962aggregateB/3520pixelB/2sequences; other119ordinary
  sequences remain targeted-only. Job0/1values65535 unresolved;
  no guestcap/fullsprite/actioncatalog, otherposes/parts/IDs/palette claim.
  [Contract, typed model and limits](../rom_data_unit_obj.md).
- **Normal-mode actor battle poses (source/model).** A finite original-selector
  projection over121normal metadata52/14 IDs plus42 includes29body/8extra-part
  bases,10980logical keys/8374physical cells/5293nulls,2581sequences17894frames,
  150OBJlists676objects and3216raw4bpp requests. Byte-identical initial/equipment
  shares146358/6984B retain prior ownership; new6021spans1455474B are disjoint
  from all other54current owners. One warning-free Windows build/one new exact
  Wine sourcefilter firstPASS12guards/16474constructor prefixes/allthreeparts/
  5293null+2451sequence resets/100commands/2858scheduler-DMA/all1063200newpixelB.
  No old-owner original-reader replay or every actor/action reachability claim.
  Alternate247cross-product is rejected because animation104aliases directory
  394834; no invented extent/count cap. Dynamic/script/alternate pairing and
  actor pairing remains open. EUR e60-d59publishes/consumes typed sources with
  actual metadata52/14/initial/equipment dependencies;56files55resources preserve
  all54old records/e59/USAe8. Two reduced Wine filters firstPASS17guards/lifecycle/
  all1455474newtypedB and32original-interpreter-compiled complete-state-clock
  paths, allparts/null/rawIDs/commands0-2-3-6-7-8/6021edges/mirrors/BIOS/SWI-DMA/
  mixedoldowners/raw. ActualCLIcreate/no-write reuse56generation/58managedfiles
  andone existingF09Wait3530fullstate-clock-image-OAM-dispatch-allaudio pair pass
  842236aggregateB/815360pixelB/fivesequences, matchingCLI. Source/model/managed/
  native/CLI/ordinary declared children closed; other2446newordinary sequences
  targeted-only/alternate-script-livepairing/Linux-USA-nativeWindows pending.
  [Contract and typed model](../rom_data_unit_obj.md#normal-mode-battle-poses-source-projection-and-typed-model).
- **Alternate battle selector views (static source contract).**65alternate IDs/
  5850logical keys resolve5682physical cells:4709nulls/953counted-sequence cells
  naming706sequences2771frames, plus20explicit known-index-root views. All20
  aliases have initial-pose directory/index witnesses; the one4Bdirectory/index
  self-overlap is sprite247/animation104→sprite0. No sequence-count filter,
  dropped binding or adjacency extent repairs the rejected blanket interpretation.
  30OBJlists108objects/1203pixel requests387008B give3321new spans410086B after
  exact6232initial+52604normal-battle+22equipment sharedB. All55e60owner source
  hashes/overlap checks pass; no other conflict or mixed typed-item ownership.
  Tagged source model/one original Wine filter now pass15guards/all5850bindings/
  10544constructorprefixes including40shared-target aliases/4428null+606proper
  resets/five newcommands/1079scheduler-DMA/all342112newpixelB/wholeVRAM. No fake
  frames or oldoriginal acceptance replay. EUR e61-d60now publishes/consumes
  3321new spans with five actual dependencies;57files56resources/all55oldrecords/
  e60retained/USAe8unchanged. Two new Wine filters firstPASS20managed guards/all
  410086typedB/606newsequences and34original-interpreter-compiled fullstate-clock
  paths/all3321edges/BIOS-SWI-DMA/mixedraw-oldowner composition. StrictNull rejects
  extra fields while valid JSON stays identical. Source/model/managed/native
  and actualCLI closed:57generation59managed no-write reuse/all57acceptedfiles
  equal/oneframeoldcache attachment passes with zero newalternate use. Positive
  ordinary/live reachability pending;81existingEURsnapshots have no current
  newalternate-task lead, no new fixture/navigation/fullpair.
  [Contract and shared target views](../rom_data_unit_obj.md#alternate-battle-selectors-and-explicit-shared-target-views).
- **Script49 actor binding (live selector contract).** Decimal49(0x31), callback
  126CC0, has167five-byte instructions/18actor selectors/13bases.151selectors
  resolve live unit identity in twelve+twenty-four264Brecords;16resolve the live
  script registry/fallback, none encodes an immediate sprite. Context lookup
  matches unit pointers and uses current actor+34sprite; groupbase is low16of
  u16group×4, facing signedactor+1F, body flag0and samebase extra parts. Nearby31
  only patches byte01in new-unit branch; existing units retain it, and32only
  repositions. No static blanket sprite join or new ownership is justified.
  Exact existing scripts plus67instruction/nine literal/eighteen BL pins pass;
  no build/guest run. Selector classification closed, resolved query census/
  sprite pairing still pending. e61/USAe8unchanged.
  [Live selector contract](../rom_data_unit_obj.md#script-animation-selectors-require-live-actor-bindings).
  Newtest-onlyfunctional Wine6000census on retained snowstate/input resolves six
  command49sources with selector16/group2to sprite46/animation8/parts0-1-2,
  eighteen real constructor queries. Body is oldnormal ownership; two null
  cells37C760/37C764(8B)are unowned, no new sequence/pixels. Original49/body
  fallback steps and retained native constructor preserved. First all-native
  preflight failed before guest frames, corrected rebuild/actual census pass.
  No fixedselector→sprite claim; stable null-cell source contract and other161
  instructions remain open, no new model/runtime/generation.
  [Observed query evidence](../rom_data_unit_obj.md#source-linked-script-query-observation).
- **Normal special-unit script poses (conditional source contract).** Exact
  CBD24unit-byte4selector match/CBAD4special predicate/CF144logical metadata14
  normal field and original9B824assignment prove a conditional source join with
  override20clear/current same-unit normal sprite/facing0..3. All151low script49
  instructions give20selector/base references,18physical normal-sprite/base
  pairs/12sprites/108queries76nulls/32sequences241frames/11OBJlists64objects/
  95pixelrequests35968B. No blanket all-sprite product or failed row removal.
  All56e61owner audit finds4780initial+30586normal sharedB only, no mixed item/
  internal-role/extent conflict. New33spans6436B=240index28header920frame5248pixel;
  60newcells53nulls/sevennewsequences46frames(allop1)/17newpixel requests. Two
  observed37C760/64null cells belong to this ROM-derived plan, not private state.
  27instruction/one literal/threeBL fingerprints pass; no build/guest run.
  Source contract closed, model/originalWine/runtime/CLI/ordinary still pending;
  alternate/override/high16/current-state cases remain separate, e61/USAe8stable.
  [Conditional source projection](../rom_data_unit_obj.md#conditional-normal-script-poses-for-special-unit-selectors).
- **Equipment OBJ poses (source model).** Original9BA84→CE1EC item properties
  3/9 and9C07A→22788 select252eligible item records/61sprites124..184, normal
  bases0/4/8 andparts0/1. Explicit action12/13 overrides add185/bases0/4;
  action16/17 selects126/base16 except item45→124. Two physical orientations
  give748cells/86null/258sequences3439frames. Only1911operation1frames reference
  OAM/pixels;609op6/888op8/31op0 retain command words/tails. Twelve OBJ lists/
  25objects yield280raw4bpp requests37440B/1170decoded indexed tiles. Exact593
  spans110666B (248directory2992index69812sequence174OAM37440pixels) are disjoint
  all53e58resource sources.73instruction/4BL/4literal/twodispatch static checks
  and one new Wine source oracle pass1496constructor-prefix aliases/86null+
  258sequence resets/sixcommand cases/280scheduler-DMA/allpixelB/10guards.
  EUR e59-d58 publishes and consumes typed fields/55files54resources with actual
  item dependency/all53old decoded/resource records/e58 retained/USAe8-d7. Two
  new Wine filters pass17guards/lifecycle/all110666typedB and26original-interpreter-
  compiled complete-state-clock paths, nulls/commands/liveRAMframe/593edges/
  mirrors/BIOS/SWI-DMA/new-raw-oldowner composition. ActualCLI create/reuse-no-
  write55generation/57managed files passes; Sheep1000/Fight700 equipment-negative
  locators have no fullpairs. Authorized copiedEURPolly→Hildegard confirmedFight500
  singleordinary fullpair now passes allstate-clock-central-wide-OAM-dispatch-
  audio; equipment4477aggregateB/1408pixelB/twosequences exactlyequalactuale61CLI.
  Other256sequences ordinary remain targeted-only. No live cap/adjacency extent or actor-body/
  script pairing claim; new Linux/USA/nativeWindows pending.
  [Equipment contract and model](../rom_data_unit_obj.md#equipment-poses-source-model).
- **Color-Mode unit/UI palettes.** Five original helpers CF49C/CF4E4/CF51C/
  CF5A0/CF5DC and70direct Thumb calls prove30spans5750B=2873fullu16colors+
  fouru8slice selectors, disjoint all52e57owners. Complete512B base blocks,
  exact supplemental fragments,12Bshared colors and9special-unit selectors
  follow actual reads/copies, not adjacency; metadata14 supplies51active
  projections across93records. Preserve low2mode/low8argument/mode3default/
  companionmode1alias and all raw holes. Strict source model/sharedplan and
  corrected reduced Wine filter pass165original pointers/99copies/30staged
  readers/54special copies/12slice prefixes/all5750B/eightguards. First oracle
  had a destination-register seed failure; nearby original MOV/LDR decoding
  fixes the test, no model difference. EUR e58-d57 now publishes54files53resources
  with metadata14dependency/all52prior decoded/resource records/e57retained/
  USAe8-d7. Immutable typed colors/selectors/raw-gap/prior-owner composition
  pass two new Wine filters:17guards/lifecycle/all5750B and28original-interpreter-
  compiled complete-state-clock paths, low8/mode3/liveRAMpalette/30edges/mirrors/
  BIOS-SWI-DMA. Mixed2new+20raw+4oldUIB preserves exact2/4charges. ActualCLI
  create/no-write reuse56managed/all54generationfiles and one positive ordinary
  Lutia1700fullstate-clock-image-OAM-dispatch-allaudio pair pass1102aggregate
  colorB=[512unit512companion78supplemental]/fourfragments. Party320/boot240
  newfamily-negative locators have no fullpairs. Remaining26ordinary fragments/
  othermodes/shared-special-selector/wholeallocator-scheduler remain pending;
  source/model/managed/native proofs stay closed.
  [Contract and validation](../rom_data_color_palettes.md).
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
  pair reads15scoreB; later400frame Yes/fee CLI/Winefullpair reads1threshold
  class1/mask1, classes2..5remain targeted-only. Whole tails/
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
  row63zero return still marks bit63. Roundtrip/noe35overlap/opcodes and strict
  source model/one Wine full0487c8oracle80real+25synthetic pass all64rows/eight
  liveunsignedminima/maskorder/zero-success/704readbytes/fullRAM/SP/callee
  guards. EUR e36-d35adds31stfile/30thresource/all29prior decoded/e35retained;
  two focused Wine managed-native-lifecycle checks pass19original/typed/native
  fullstate-clock paths/raw768Baltered/two-page typed bus/zero-success/fullmask/
  SWI-DMA/gap/coexistence. ActualCLI no-write/oldcache and one1000Wine normal
  dispatchcompletion fullstate/image/audio pair pass209requirements/8deltaB;
  return/outputordinary targeted-only. Separate400frame Yes/fee prefix closes
  class1threshold ordinary use; downstreamreceipt-FK/newLinux/USA remain pending.
  Next selection gates own64x16/1024B6e80de..6e84de fromD2390physical1..64,
  notrow0. Hardunsigned minima01/02/04, ANYactive06..0d and flag0e preserve
  physical firstmatch, zero-condition store0 and exhausted64no-store. Actual
  row52sentinel/53..64storage distinguished; independentnextroot6e84e0leaves2B.
  Static contract/roundtrip/noe36overlap and strict source-only model pass; one
  Wine fullD2390/reset-count-flag oracle138real+80synthetic checks831actualfieldB/
  all1024Broundtrip/ANY/unsigned/zero-terminal/no-store/fullRAM-SP-callee.
  EUR e37-d36nowadds32ndfile/31stresource, all30prior decoded/e36retained.
  Two focused Wine managed/native/lifecycle checks pass18fullD2390paths/raw1024B
  altered/fullstate-clock, all6fields/64recordmask/bus-BIOS/16Brow0+2Bgapraw/
  SWI-DMA/oldgate2-raw16-new2. ActualCLIcreate/no-write reuse/oldcache and one
  1000Wine ordinary dispatchcompletion fullstate-image-audio pair pass277newB/
  all6fields/first26records;400Yes-fee0Bclosednegative/no fullpair. NewLinux-USA/
  all64ordinary/semanticdisplay remain pending.
  Next aggregate key table owns6e9a5c..6e9cb6/602B(300u16slots+ffff),
  D22FCglobal and13FB1Ccontext counters share exactsource, next root29AF4atend.
  Strict source-only model/one Wine32real+26synthetic oracle pass both complete
  counters/all602B/key+2ffu16wrap/duplicates/earlystop/flag36ORlinked017f/
  contextisolation/fullRAMreadonly-SP-callee guards. OtherindexedUIreader roots
  static-only. EUR e38-d37nowadds33rdfile/32ndresource/all31prior decoded/e37
  retained. Two focused Wine managed-native-lifecycle checks pass16fullglobal/
  context paths/raw602Baltered/fullstates-clocks/key-terminal-300slotmask/
  bus-BIOS/SWI-DMA/typed2raw2terminal crossing. ActualCLIcreate/reuse-no-write/
  oldcache and one1000Wine ordinary dispatch fullstate-image-audio pair pass
  1204sourceB(1200key/4terminal)/all300slotmask; countsinclude rereads, both
  ordinaryentrypoints notinferred. NewLinux-USA/wholeotherUIreaders pending.
  [Item contract/evidence](../rom_data_gameplay.md).
- **Audio samples, instruments, songs and sequencing.** `engine.rs` detects
  driver families; `mp2k.rs`, `gax.rs` and `rdrv.rs` shadow live voices while
  leaving original mixing/sequencing running. `MemView` resolves ROM/RAM
  ranges, and each driver has its own sample/loop/instrument contract. Driver
  detection is not an exhaustive instrument or song index. Exact EUR now has
  [original M4A root contracts](../rom_data_audio.md):3player configurations/
  36B,640directory entries/5120B,455unique headers/10080B including186aliases
  to a zero-track header,1610track roots and four bank roots. Independent
  lossless decoding/static readers/live retained-state pointers pass. Strict
  source model/457ranges/15236B and focused Wine original-reader649cases+one
  raw lookup+six synthetic variants pass;13477distinct original-read B,1759B
  retained-only. EUR e41-d40managed audio directory publication/typed CPU-DMA-
  native consumption/lifecycle pass two focused Wine checks/44fullstate-clock
  paths; productionCLI and one400-frame ordinary Wine pair pass179sourceB
  (36directory/79header/64track-pointer), three songs/headers and0player reads. There
  is no local640index guard or generic128tone bound. New producer inventory
  follows1610track roots/177047bytecode B to390bank/program pairs,429tones,
  347keymap B and277PCM waves/1931528nominalpayload B. One focused Wine check
  confirms390BDcopies/348mapped links/276extra signed interpolation endpoints
  plus one fixed8nominal endpoint. Source plan736ranges/1941759newBexcludes4B
  e41alias. Shared bounded parser/strict tone-keymap-signedPCM-CGBmodel now pass
  one Wine decoder/independent-hash/alias-dependency check and2original CGBcopy
  checkpoints. e42-d41nowpublishes audio-tones.json/37files/36resources with
  actual directory alias dependency; all35prior decoded/e41retained. Two reduced
  Wine checks pass26guards/lifecycle/all1941759B/25guest-native fullstate-clock
  checkpoints/736edges-SWI-DMA. Actual MP2Khook/tick reads typed header/PCMvalues
  for all277waves; bit-exact nonzero shadow samples/completehoststate-clock pass.
  ProductionCLIcreate/reuse-no-write39managedfiles/oldcache and one400ordinary
  Winefullstate-image-audio pair pass516956guestB/3093208hostB(1056header/
  3092152PCM), CGBordinary0. CLIdefaultshadowoff explicit; differential arms it
  after load. Partial probes are not fullmixerABI.
  New strict sequence source model identifies105253physical instructions with
  single ownership over805ranges177047B and1610directory roots. Canonical branch/
  pattern IDs and typed explicit/inherited operands preserve dynamic sequencing.
  49clockB/36originalcommandwords144B/CD8-9dispatch8B add3spans/201B:808sources/
  177248Bdisjoint from all36prior owners. One Wine model/source check passes
  allfields/hashes/27guards/404neworiginal-reader checkpoints. e43-d42nowpublishes
  sequences:38files37resources/all36prior/e42retained/actualdirectory+tonebank
  dependencies. Two Wine checks pass25guards/lifecycle/all177248B/allinstruction-
  table masks/35original-native fullstate-clock checkpoints/808edges-SWI-DMA/
  liveRAM runningoverride/rawfallback/tone-sequence guest+generic-host boundary.
  ProductionCLIcreate/reuse-no-write40files/oldcache and one400ordinary Wine
  fullstate-image-audio pair pass819B(366opcode/262operand/191clock),185source
  records includinglookahead;0command/extended/generic-host. Seven further driver
  tables370b20..370d04/484Bhave typed source model:180PCMpackedkeys/12u32bases/
  12u16samplecounts/132CGBpackedkeys/12i16bases/60noise/16volume. OneWinecheck
  passes1138neworiginalreader probes, all484B/source-JSON and15guards; sample
  selector0/13..15 and volume16/64/255aliases remain original/no addedbounds.
  Eight targeted gain returns confirm balancedroute levels16/23/31 and only
  single-side cap15; globalordinary16-domain remains unproved. e44-d43publishes/
  consumes39files38resources/all37prior/e43retained withactualsequence boundary
  owner. Two reduced Wine checks pass19guards/lifecycle/all484B/424entry bits/
  28original-interpreter-native fullstate-clock paths/mixedThumb-ARM/adjacent
  aliases/boundaries-BIOS-SWI-DMA/generic-host. ProductionCLIcreate/reuse-no-write
  41files/oldcache and one400ordinary Winefullstate-clock-image-audio pair pass
  756aggregate byte reads/67entries fromPCM-CGBkey-base tables;0samplecount/noise/
  volume/generic-host ordinaryreads remain explicit. Bank gaps, other
  wrapper roots/LUTs, language variants and references stay open.
  Offline catalog: implemented for this initial
  EUR directory/producedtone-keymap-PCM-CGB-sequence-driver scope; managed extractor: e44-d43; runtime:
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


### EUR adjacent lookup and five trade parameter records

ROMDATA-01-GAMEPLAY-ADJACENT-HALFWORD-LOOKUP-ROOTS is partial:16literal references
root6e9cb6, next independent6e9d8a, but the natural106halfword domain is unproved.
Fixed64/67 and bounded lists16..20/21,22,4,23 do not establish all UI producers.
No model or ownership is inferred from values or neighbor extent.

ROMDATA-01-GAMEPLAY-ADJACENT-STRIDE96-ROOTS is complete for the declared static
contract: selectorD0744 explicitly roots five96B records, IDs2..6,480B at
6e85bc..6e879c. Independent next root and biased price source agree. D2728's30
live RAM rows bound the internal count0..30, not a30-record ROM bank. Strict
source model retains64opaqueB/u16markup40-41/30discountB42-5f per record;
count0 aliases markup byte41. D26AC retains original separate70B-source D1F30
property36/zero skip/live flags/unsigned divide/wrap arithmetic. Windows73640
warning-free/Wine52757passes499real+160edited-source+15separate injected operand
cases; all160price-fieldB observed/320opaqueBroundtrip-only/fullRAM-SP-callee
and strict source/JSON guards pass. e38-d37 is unchanged; managed/runtime/
ordinary and newLinux-USA-nativeWindows remain pending. [Contract](../rom_data_gameplay.md#trade-parameters-source-contract).


ROMDATA-02-GAMEPLAY-TRADE-PARAMETERS-DATA / ROMDATA-04-GAMEPLAY-TRADE-PARAMETERS-CONSUMPTION
are complete for declared EUR/Wine: e39-d38/34files/33resources/all32prior/e38
retained/USAe8-d7. Windows21694warning-free; Wine20164two checks pass managed/
native/lifecycle/strict source bindings and28full paths/raw480Baltered/fullstate-
clock parity, all480B/mask31/fieldedges/rawgap222/SWI-DMA/priorowner coexistence.
Compiled opaque/markup-low/shared41/discount0-5-10-10, no opaque original price
read inferred. Source499+160+15closed; CLI/ordinary/newLinux-USA-nativeWindows
pending. [Contract](../rom_data_gameplay.md#managed-and-direct-trade-parameters).


ROMDATA-05-GAMEPLAY-TRADE-PARAMETERS-SCENE-CLI is complete for declared EUR/Wine:
production46740warning-free/canonicalrestored; actualCLI17191create/reuse-no-write/
oldcache/unrelatedcwd with36managedfilehash-mtime guards. First400existingpre-Yes
fee acceptance locator uses14sourceB(markup-low7/shared417),mask1physicalID2;
rereads included. One Wine89539full400pair matches every state-clock-image-OAM-
dispatch/mixed-PSG-FIFO-HLEaudio sample, same source counts. FinalPublist/7109funds
image inspected. Source499+160+15/native28/managed/lifecycle/e34threshold400/e38
proofs remain closed. NewLinux-USA-nativeWindows/ordinary discounttail-opaque-
all5records/othercallers remain pending. [Evidence](../rom_data_gameplay.md#ordinary-trade-parameter-fee-acceptance).


### EUR fourteen packed item effect parameter records

ROMDATA-01-GAMEPLAY-ADJACENT-STRIDE7-ROOTS is complete for the declared static
contract:11literal readers/14×7B/98B6e879c..6e87fe, existing itemIDs362..375
subtract362 and independent next root agree; no e39overlap. Nine5-bit
coefficients,8-bit cap and3opaque bits roundtrip and match split-byte getters.
CFE28property/indexlow8 and CFDA8fullu16item-minus362 cap index remain distinct;
live current/quantity/cap clamp stays original. UI cap>99 branches and arbitrary
indices remain unverified/raw. Source model/oracle is now complete: Windows94173
warning-free/Wine94185passes540real+4explicit raw-cap aliases+166synthetic cases,
all98B/read-onlyfullRAM/SP-callee/strictJSON-bitwidth-itemowner-source guards.
Managed/runtime/ordinary/newLinux-USA-nativeWindows pending; e39-d38 unchanged.
[Contract](../rom_data_gameplay.md#item-effect-parameter-static-contract).


ROMDATA-02-GAMEPLAY-ITEM-EFFECT-PARAMETERS-DATA / ROMDATA-04-GAMEPLAY-ITEM-EFFECT-PARAMETERS-CONSUMPTION
are complete for declared EUR/Wine: e40-d39/35files/34resources/all33prior/e39
retained/USAe8-d7, actual parsed item owner dependency. Windows93110warning-free/
Wine21177two checks pass50full original-interpreter-native paths/raw98Baltered/
complete state-clock equality, all98B/14recordmask16383/7Brecord-fieldedges/BIOS/
SWI-DMA/adjacenttrade2-itemeffect2crossing/publication-retention-lifecycle/strict
source guards. Compiled coefficient/shared05/shared06bytes36/37/23. Source540+4+
166not repeated; CLI/ordinary/newLinux-USA-nativeWindows/fullUI remain pending.
Existing DS22populatedbank proof is USA-only, not EUR evidence.
[Contract](../rom_data_gameplay.md#managed-and-direct-item-effect-parameters).


ROMDATA-05-GAMEPLAY-ITEM-EFFECT-PARAMETERS-SCENE-CLI is complete for declared
EUR/Wine: production4620warning-free/canonicalrestored; CLI59404create-reuse-no-write
37managedfilehash-mtimeguards/50707oldcache from unrelatedcwd. New ordinary guest
input from retained EUR IN13reaches CyrilBank/List/ownedGoblinLv4/Play. Existing
pre-Yes Cyril state supplies measured120,216/place18; earlier wrong-town/guessed
longhold navigation trials are closed, not rerun. One Wine5779full400pair matches
every state-clock-image-OAM-dispatch/mixed-PSG-FIFO-HLEaudio sample, positive3B
(coefficient1/shared051/shared061)/mask1record0. Ordinary mode is mixed native/
interpreted; source/native/earlier ordinary proofs stayclosed. FinalCLIseparateUI
panels absent as previously known, no food-count claim. Other14item/20monster/
cap>99UI/newLinux-USA-nativeWindows checks pending.
[Evidence](../rom_data_gameplay.md#ordinary-item-effect-parameters-in-the-european-monster-bank).

### Reclassified palette-effect sources (24 October 2026)

Roots370eb0/b94690 formerly queued as FFTA audio are not audio resources.
370eb0 is a copied-IWRAM zero-clear/copy/fill helper vector and stays executable
support metadata. b94690is palette-effect dispatch: state1/readu16kind/stride4,
fixed constructor kinds1/2/4/8/16,17word68Bprojection including12nullholes.
No liveRAM max16guard/fulltableextent is inferred. All38e44owners are disjoint;
static7literal/5producer/opcode/hash checks pass without guest execution.
[Source contract](../rom_data_graphics.md#palette-effect-dispatch-source-contract).
Model/sourceWine/publication/runtime/ordinary remain pending; audio migration
of these false leads is closed. Other unselected audio/graphics/families stay open.
Typed17slot/68Bpalette dispatch model nowretains canonicalu16kinds/guestThumb
code offsets/nulls with strict shape/source authentication. Windows71666warning-
free/Wine22360one focused check passes31original prefixes/all68B/12guards/
low16liveRAM/rawoutside/state gates/predictedproducerstores.1216protected inputs
unchanged; no fullcallback/native/stateclock/ordinary claim. Managede44unchanged;
publication/runtime/ordinary/newLinux-USA-nativeWindows remain pending.
e45-d44managed palette dispatch nowpublished/consumed:40files39resources/all38
prior/e44retained/USAe8-d7. Windows67564warning-free/Wine46138two reduced checks
pass12guards/publication-retention-lifecycle/all68B/17slotmask/10original-native
fullstate-clock prefixes/24callback-4nullcompiledB/livekind/rawaliases/BIOS-SWI-
DMA/boundaries. No fullcallback ABI; actualCLI/ordinary remains pending.
Actuale45productionCLIcreate/reuse-no-write42files/oldcache nowpassWine56076;
neutralboot240closed0use/no fullpair. New normal English-confirm1000CLI60304/
oneWine35467fullstate-clock-image-OAM-dispatch-audio pair pass5944aggregate
callback-byte reads/kind1only(mask2)/null0. Othercallbacks targeted-only; no
RAM/statepayload/save edits. Source31/native10/oldfamily proofs not repeated.
Paletteparameter/program producers and otherfamilies/newLinux-USA-nativeWindows
remain open; no performance/space measurements.

### Palette-program producers: partial inventory (24 October 2026)

Actualpalette task080d7fac/public080d8214-080d8234 has308directBLcandidates;
285exactargument expressions root179programs.3fixed-count8Blist sources add5,
2explicit3wordstack copies add1; saved-R4bridge reuses a knownroot. Combined185
programs/1000commands,16135candidateprogram B/124ranges plus88producerB, all16223
disjointfrom39e45owners. Six-byte header/countu8 and6Bcommand/u16relativeoffset
contracts distinguish direct/queued colors and minimum1countedcolor sequences.
20indexed producers now have explicit task-ID/message/constant/counted-loop
projections:97cells388B add45programs, total230programs/1135commands/18221programB
plus476producerB=18697B/165ranges, disjoint39e45owners. All185earlier records
retained; mixed39bcc4owns2pointers only. These are no globalRAM guards/fulltable
extent claim. SourceWine/model/publication/runtime/ordinary pending, e45unchanged/
no newguest/build/tests or oldrepeat.
[Contracts](../rom_data_graphics.md#palette-program-producer-inventory).


### Palette programs: source model verified (24 October 2026)

Strict typed230programs/1135commands/25producer tables reconstruct165ranges/
18697B; canonical IDs/raw fields/list delays-reserved/null slot preserved.
Windows warning-free/oneWine96989check passes all18221programB and18681distinct
allB at original Thumb readers;16reserved list B are preserved/unread.97indexed
cells/8listrows/2stacktables,6wrap-clamp copy inputs/3mincount-period/1rawdirection
probe and16shape-JSON-source guards pass. Earlier1645artifacts/newtestexe unchanged.
This closes the source oracle/model child, not fullallocator/callback/native/
stateclock/ordinary. e45-d44unchanged; managedpublication/runtime/ordinary next.
[Source model](../rom_data_graphics.md#palette-program-source-model-and-original-reader-validation).


### Palette programs: managed runtime (24 October 2026)

EUR e46-d45publishes41files/40resources, all39prior decoded/e45retained/USAe8-d7.
165bindings18697Bserve typed header/command/parameter/producer fields; no guest
sequencer/index/save/native ABI change. One Wine lifecycle check passes12guards/
publication-create-reuse-repair-move/missingfile-Load-Undo-backup-Restart/all18697B/
230programs25tables. Native-only check passes24original-interpreter-native full
state-clock prefixes, generated API[16header52command66parameter12producer]B,
rawcommand65535/coloroffset65534/index2/widths-mirrors-BIOS-SWI-DMA. Two testboundary
failures retained and fixed via prefix guard respecting native chaining/events;
production emitter unchanged/source-managed checks notrerun. ActualCLIcreate/
reuse-no-write43files/oldcache pass. English-confirm1000newprogram-use0/closed
negative/no fullpair.10protected worldmap-bank snapshots have no newpalette task.
Actual ordinary EUR Montblanc Richiamo/Pecorelle CLI and one1000-frame Wine
fullstate-clock-classic240x160-image-OAM-dispatch-audio pair pass170sourceB/
4programs (32header/90command/48parameter),0producer. Generic typed bus-or-host
consumption guard corrected; selected palette assertion unchanged. Other226
programs/producer tables retain targeted-only coverage. NewLinux-USA-nativeWindows
and new32:9 scene checks remain open.
[Consumption](../rom_data_graphics.md#managed-palette-programs-and-typed-guest-reads).


### Blend programs: bounded producer inventory (24 October 2026)

The neighboring080d77c4task/080d79ac-080d79d0constructors control BLDCNT,
BLDALPHA andBLDY through the compact PPU shadow; they are not e46palette roots.
All248retained direct calls are classified:235constant/2conditional/6indexed/
1inline/1mutableRAMtemplate/3dynamicRAM. Current CFG-block validation rejects
the false080e2ec8code root; that caller copies10B from0839a5f0then overrides its
RAMduration. Six bounded projections supply22pointer cells; no global RAM guard
or complete physical table extent is inferred. Inline0/1variants follow explicit
message-ID branches; primary and secondary descriptor constructors are distinct.

160programs/345commands own2340B(960header/1380command), plus88producerB:
2428B/158ranges, disjoint all40e46owners/no repeated logical byte. Header6B is
u16control/coefficients/count, command4B is u16duration/u8first/u8second. Static
independent reconstruction/allhashes/44consumer opcodes/DMA register mapping
pass.135alpha/20brighten/5darken;5instantffffcommands. Typed model/sourceWine/
publication/runtime/ordinary remain pending; no new guest run/build/test or
performance campaign. EUR e46-d45/USAe8-d7 remain. [Source contract](../rom_data_graphics.md#blend-program-producer-inventory).


### Blend programs: typed source model (24 October 2026)

Strict lossless `rom_data::blend_programs` reconstructs all160programs/345commands/
6tables22pointers/2428B158ranges, preserving packed reserved fields and raw
u16duration/u8targets/canonicalreferences. Exact EUR identity/110reader-producer
fingerprints, strict JSON/order/count/bitwidth/reference/overlap/source guards.
Windows12341warning-free; oneWine94935source check passes160headers/345commands/
22cells/5instantchains/templatecopy-RAMoverride/2packedheader/6duration/3delay/
index65535rawprefixes.2376sourceB observed;52nonalpha timed second fields remain
preserved/hashchecked only for these initial-mode probes.17shape/3source-edit/
2JSON/2ROMidentity guards pass;52943instructions264PCs arecoverage, no timing.
Managed/runtime/native/fullclock-ABI/CLI/ordinary/newLinux-USA-nativeWindows stay
pending; e46-d45 unchanged. [Model and scope](../rom_data_graphics.md#typed-blend-program-source-model).


### Blend programs: managed runtime (24 October 2026)

EUR e47-d46publishes42files/41resources/all40prior decoded/e46retained/USAe8-d7.
158bindings2428Bserve typed header/command/producer fields on13pages; no guest
sequencer/index/save/native ABI change. Reduced Wine checks pass12guards/
publication-retention-lifecycle/all2428B/160program6tables and20original-native
complete-state/clock prefixes. Compiled reads[34header70command16producer]B;
source modes/wait-instant-delay/templatecopy/live2→1mode/rawcommand65535/index3/
previouspaletteindex5/width-mirror-BIOS-SWI-DMA/new2raw2/new2old2 pass.
1034instructions/295PCs/93dispatches arecoverageonly. Initial compile argument/
import fixes retained; corrected source-set coherence separately passes a new
focused Wine negative guard, without closed native/lifecycle repeats. No whole
callback/allocator/scheduler ABI. ActualCLIcreate/reuse-no-write44managedfiles/
oldcache and one1000Wine ordinary EUR Pecorelle fullstate-clock-classic240x160-
image-OAM-dispatch-audio pair nowpass18aggregateB (10header/8command),1program,
0producer; other159programs/tables targeted-only. NewLinux-USA-nativeWindows/
new32:9 scene checks remain pending. [Consumption](../rom_data_graphics.md#managed-blend-programs-and-typed-guest-reads).

## Bounded European task descriptor sources

Three adjacent catalogs contain341primary/20auxiliary/51script records at3997b0/
39a258/39a2f8:412records3296B/3spans, disjoint41e47owners. Original constructors
080d7018/080d7328 use different ID formulas; nullableThumb callbacks/contextsize/
rawmetadata are distinct. Independent graphics source39a490and directory kind0/
ID413reference at3aca2c establish the suffix endpoint39a488; no global live ID cap
or whole callback ABI.53BLcandidates retained; unresolved CFGs are not proof of
absence. Static independent roundtrip/hash/59halfword12word contract checks pass.
Strict typed model now passes one reduced Wine source oracle: all3296Breconstructed/
3156Bobserved,140suffix metadataBpreserved-unread,415main/59secondary/346halfword/
346byte/18low16/9branchprefixes and23shape-source-JSON-identityguards. Warning-free
Windows build;7191instructions39PCs coverageonly. e48-d47managedruntime nowpasses
13guards/lifecycle/all3296typedB/412records/3catalogs and29original/native fullstate-
clock prefixes, generatedAPI44callback22context9metadataB, raw/aliases/oldblend
owner/boundaries-BIOS-SWI-DMA.43files42resources/all41prior decoded/e47retained;
USAe8-d7unchanged. ActualproductionCLIcreate/reuse-no-write45files/oldcache andone
1000Winefullstate-clock-classic-image-OAM-dispatch-all-audio pair nowpass28aggregate
sourceB/4records2catalogs (16callback8context4metadata). Other408records/thirdcatalog
targeted-only/new32:9pending. Source/model/runtime/CLI/ordinary children closed;
no wholeallocator/callback/schedulerABI or newLinux/USA claim. [Contract and scope](../rom_data_tasks.md). Linux/USA remain pending; no new
performance/space measurements.

## European effect/event producer directories

The bounded source contract covers75event16Brows/1200B,51selector4Brows/204B and
22+6evenThumbsame-state dispatchwords/112B:1516B/four spans disjoint42e48owners.
Original increment/compare74andindependent6Bnextcatalog3acebc boundevents; separate
u3lookup3ad240bounds selectors. No countfromshape or51taskdescriptors.19event+
51selector logical links resolve toengine.task-descriptors. Originalrelative
jumps/u16index/q0underflow/secondaryIDwrap/defaultbranches staylive. Source-model
strictJSON/92halfword8word guards/independent reconstruction andoneWineoraclepass:
77header77delay53selector43param32dispatch19frontdoor-low16-increment prefixes,
1106Bobserved/300eventraw110selectorparamBpreserved-unread,27guards.2582instructions/
70PC coverageonly; no fullinterpreter/native/stateclock/ordinary/reachabilityclaim.
Managed e49-d48integration nowpasses13guards/lifecycle/all1516typedB and29original-
interpreter-native fullstate-clock prefixes/[24event6selector24dispatch]API B,
raw/defaults/evenstate/previousfont11B/samepagepalette4B/BIOS-SWI-DMA.44files43resources
retainall42prior decoded/e48; realtaskdescriptor dependency. ActualCLIcreate/
reuse-no-write46managedfiles/oldcache passes. Pecorelle1000/schoolcoldboot3000/suspendboot-to-battle8476
locators readzero newbytes: closednegative/no event-requiredfullpairs. New normal
EUR boot/name/snowball/street/bedroom progression yields19genuine snapshots;
one1800-frame book locator/fullpair from39420 on actual e52generation passes
allfullstate-clock-image-OAM-dispatch-audio with5aggregateB=1selector+4dispatch,
one selector/one table,470hash-mtime guards. Ordinaryeventrows0/other50selectors/
other dispatch table targeted-only, entry124e1creachability unknown.
USAe8-d7unchanged/Linux-USA-nativeWindows/new32:9 pending/no performance.
[Contract and limits](../rom_data_events.md#positive-ordinary-book-transition).

## European script command directory

The next independent EUR source projection contains 115 six-byte commands,
690 bytes at3acebc..3ad16e. Three original literal roots and byte-assembled
callback/advance readers are verified. Two independent byte-lookup roots at
1263c4/126468 bound the end; an initial 116-row candidate ending at3ad174 was
rejected before the source plan/model because it included that distinct lookup.
All 115 callbacks are unique non-null Thumb BX entries; no asset dependency.
The projection is disjoint from all 43 e49 owners. Original u8 opcode arithmetic
has no new cap, and signed stream cursors/u16 advance wrap/relative targets/
return0x110 loop remain live. Strict model/hash roundtrip/106 halfword and six
word fingerprints and one corrected Wine oracle pass all 690 observed bytes,
117 callback/117 advance/115 BX/four signed-cursor/ten relative-target/five loop
prefixes and 19 guards. First Wine failure was an ADDS expected-register mistake
in the test, corrected without a production/model change. 2905 instructions/
67 PCs are coverage only. Managed e50-d49 nowpublishes45files44resources/all43
prior decoded/e49retained/USAe8-d7unchanged. Two reduced Wine checks pass13guards/
lifecycle/all690B and26original-interpreter-native fullstate-clock prefixes with
14advance/28callback API B. Raw115/255/signedcursor/wrap/relative-targets/BL-BX/
returnloops/adjacent previousowners4+4B/mixed previous2new2/new2raw2/BIOS-SWI-DMA
agree. ActualCLIcreate/reuse-no-write47managedfiles/oldcache andone8476-frame Wine
EURsuspendboot-to-battle fullstate-clock-image-OAM-dispatch-audio pair pass
42B=14advance28callback/7commands. Other108commands targeted-only; new32:9/Linux-
USA-nativeWindows pending.
[Contract, evidence and limits](../rom_data_scripts.md).

## European relative script archive and event keys

Bounded static inventory identifies115464B/two disjoint ranges:114528Barchive at
B946EC..BB064C and936B/234keys atD68C2C..D68FD4. Independent localized-text and
coordinate-index roots bound both ends. Keydomains/first relative offsets agree
with62topbanks/233nestedslots (248+932indexB).213unique targets/20repeated links
include5headeralias links and2FFfill links.207nonempty physical command segments
contain35048storage instructions/110opcodes/78212parameterB;45FFfillspans88B remain
readable.493command5alinks target49existing effectselectors; actualdependencies
areengine.script-command-directory andengine.effect-event-directories. Keybyte2
selects one of five unmigrated localized text archives; byte3raw is preserved.
Independent reconstruction/234keys/233nonnulltargetformulas/128halfword5wordfingerprints/
all115464B/disjoint44owners pass without guestexecution. First builder's empty-
program assertion exposedtwofilltargets; correctedbeforeplanwritten. Notlinear
CFG/wholeargument semantics/reachability/newguestcaps. Later model/runtime
results below cover separate criteria; inventory left e50-d49/USAe8-d7 unchanged. [Contract](../rom_data_scripts.md#relative-script-archive-and-event-key-source-contract).

The whole115464Barchive/key model now has strict lossless Program/Header/Fill
references and complete physical reconstruction, noROMshadow/aliascopies. Actual
command/effect dependencies validate lengths and493selector links. Windows91283
warningfree/oneWine85436 first-pass verifies238keyaddress/234two-level/234textbank/
35054opcode/2397relative-target/493selectorparam prefixes,42333ownedBobserved,
73131Bpreserved-unread and40guards.224382instructions62PCcoverageonly; notwhole
interpreter/native/fullstate-clock/ordinary. Model leaves e50-d49/USAe8-d7 unchanged,
source-model scope only; managed/runtime results follow. [Model evidence](../rom_data_scripts.md#typed-relative-archive-model-and-reduced-wine-source-oracle).


Managed e51-d50 adds script-archive.json/engine.script-archive,46files/45resources,
all44prior decoded files/e50 retained, USAe8-d7 unchanged. Real command/effect
asset dependencies and two typed source bindings/115464bytes authenticate before
activation. Two reduced Wine checks pass13guards/lifecycle/allbytes and28original/
interpreter/native complete-state/clock prefixes. Compiled API categories read
37index/4opcode/7parameter/12key/1fill bytes, preserving raw IDs/unaligned rotation/
header-fill views/previous palette owner/boundaries/BIOS-SWI-DMA. Production CLI
create/reuse-no-write48managedfiles/oldcache passes; retained8476-frame EUR
locator reads27B=8index7opcode10parameter2key, one bank/program/key, no fill.
One Wine8476-frame pair from the actual CLI generation passes all full states/
clocks/central-wide images/OAM/dispatch/mixed-PSG-FIFO-bitwise-HLE audio, same
positive archive use and366hash-mtime guards. Other61banks/206programs/233keys/all45fills
retain targeted coverage. No new32:9/wholeABI claim; newLinux-USA-nativeWindows
pending. [Runtime evidence](../rom_data_scripts.md#managed-relative-archive-and-typed-guest-reads).


## European script-selected localized text

Five language roots BB064C/C076F0/C5FBF0/CB7C48/D10644, selected by08009ab0,
independently bound1803744B through the next root/D68C2Cscript-key root.63top
entries per language agree with key-bank maximum62/minimum offsetFC; first word
is a shared late bank0, not a length. Eleven repeated normal links per language
and26root-relative bank61variants resolve to77physical banks per language.
Live byte division by10selects the variant; a distinct remainder helper adds
24times the remainder to the u16message. Full-u16 and low-byte special-message
branches, raw key IDs, fallback language/key and live state remain original.

385banks/41395u16index entries including385sentinels name41010programs. Existing
Program/customLZ grammar gives9813compressed extended/5709raw extended/25488raw
compact programs,212011commands/11forms,147glyph IDs1..3112 already in the font
asset; actual dependencies script-archive/font.complete-bank. The whole source
partition is1260top+520variant+82790bank-index+1684407program+34767alignment B.
34314gaps are1..3B; four mixed gaps reject a uniform-fill assumption before any
model/runtime/Wine. Exact bytes remain explicit, without inferred semantics.

Independent static whole-source reconstruction/hash/15195key-variant formulas/
349halfword13word guards/actual dependencies/glyphs/disjoint45e51owners pass.
No guest execution/build/test or prior proof repeated. Typed model/Wine/runtime/
CLI/ordinary pending; EUR e51-d50/USAe8-d7 unchanged. [Contract and evidence](../rom_data_text.md#script-selected-localized-text-source-contract).


The strict whole-source script_text model now shares physical banks through
typed top/variant references and reconstructs all relative/u16index words,
Program codec bytes, sentinels and exact slack without alias payload copies.
Actual key/font dependencies validate. First Wine strict-JSON failure exposed
unit-variant extra-field acceptance; corrected to empty struct, kept assertion
and all first-attempt evidence. Corrected Windows55123warning-free/single new
Wine34745passes all1803744B/independent fields/hashes and22shape/6source-edit/
3dependency/12JSON-shape/2identity guards. Original7language/1170key-bank/
1280live quotient/256live remainder/90raw-special/1170u16index/837header/
837source-vs-typed output cases pass, including249compressed bodies/tails.
2116900instructions/834PCcoverage only; not native fullstate-clock/ordinary/whole
ABI. Source model closed, publication/runtime/CLI/ordinary remain pending;
e51-d50/USAe8-d7 unchanged, new Linux/USA/native Windows pending. [Model evidence](../rom_data_text.md#typed-script-selected-text-model-and-reduced-wine-source-oracle).


Managed e52-d51 now publishes script-text.json/text.script-archives,47files/
46resources/all45prior decoded/e51retained/USAe8-d7, with actual key/font deps.
Five typed bindings1803744B reconstruct all index/header/control/decoded program/
slack fields and merge previous owners/raw bytes on BB0/D68. Shared literal map
keeps old categories; one affected core Wine check/new managed-native filters
pass13guards/lifecycle/allbytes/all masks and35original/interpreter/compiled
full-state-clock prefixes, including raw selectors/wrap/sentinels/width-mirror-
realBIOS-CpuSet-FastSet-DMA. Direct tail literals0is exact, generated tails remain
preserved. Real old0/1/2source163B also pass.379instructions/65PCs/71dispatches
coverage only. Production CLI create/reuse-no-write49managedfiles/oldcache passes;
8476battle locator is negative0text/no fullpair, distinct school3000positive
reads289B/one language/bank/two programs. One Wine3000pair from CLI generation passes every full state/clock/image/OAM/
dispatch/mixed-PSG-FIFO-bitwise-HLE sample, same source use/468guards;
other records/variants/slack targeted-only, newLinux-USA-nativeWindows pending.
[Runtime/CLI evidence](../rom_data_text.md#script-text-production-cli-and-ordinary-school-dialog).

## European script coordinate paths

Independent reader128914/literal128994and successor readerA1F8/literalA234 bound
D68FD4..D69BC4:3056B, a self-delimiting840B/210-slot relative directory,210u16
counts420B,856XYpairs1712B and42FFalignment spans84B. All physical storage is
covered once; these are no global live index/count/coordinate caps. Original
low16index/low8destination/buffer+80count-low8return and live map-height query
1DD88remain distinct.68primary-script links reach56paths:66opcode44low8/ignored
byte3 and2opcode45u16. Actual archive/command dependencies are checked; source
is disjoint from all46e52owners and shares only theD68page with earlier owners.

The strict typed model reconstructs/hashes all3056B. One warning-free Windows
build and one Wine source oracle pass24guards/allindependent fields/194halfword/
5word fingerprints,212original path copies (all210lists/two changed live map
cells),13raw-index/destination and13producer prefixes.2972sourceB observed;
84alignment Bpreserved-unread.40116instructions/134PCs coverage only, no native/
fullstate-clock/wholemovement-task/allocator/ordinary claim. Model source stage
closed; managed publication/direct runtime/CLI/ordinary remain pending, e52-d51/
USAe8-d7 unchanged. No new fixture/navigation/Linux-USA-nativeWindows/performance
check. [Source/model contract](../rom_data_scripts.md#european-script-coordinate-paths-source-contract).

Managed e53-d52now publishes48files47resources/all46prior decoded/e52retained,
USAe8-d7. Actual archive/command links authenticate before one shared typed
index/count/XY/alignment owner activates; D68merges text/key/coordinate fields,
rawD69BC4outside retained, guest height/control/ABI unchanged. Reduced Wine
managed/native checks pass14guards/lifecycle/all3056B/210paths856points42alignment
and24original-interpreter-compiled fullstate-clock cases/four full copies/live
height/allfour classes[28,85,76,2]B/mixed owners/edges-BIOS-SWI-DMA. New-test array/
observer errors corrected with evidence retained; successful checks reused.
ProductionCLIcreate/reuse-no-write50files/oldcache/all48generation equality and
one6000-frame genuine EUR ordinary pair pass every fullstate-clock-image-OAM-
dispatch-audio with28aggregateB=4index14count10XY/one path,0alignment;581guards.
Other209paths/42alignment targeted-only; wholemovement/allocatorABI/new32:9/
Linux-USA-nativeWindows unverified, no performance measurement. [Runtime evidence](../rom_data_scripts.md#coordinate-path-production-cli-and-ordinary-scene).


## European auxiliary script archive source projection

Independent readerA1F8/rootD69BC4 uses u8action/slot and two levels of relative
u16 offsets, separate from the primary u32 archive. The declared4098B projection
throughD6ABC6 has8banks/442slots/126distinct targets/30interior entries,
133physical programs/913instructions/2275parameterB/900indexB and5internalFFgaps
10B. Root walks alone miss69physical instructions. All489relative branch links
resolve to instruction boundaries; structural908/913coverage is not ordinary
reachability, and the five remaining instructions stay preserved.

Separate17x6Bcommand directory370F94..370FFA supplies advanceu16/Thumbcallbacks;
its successor370FFC is independently consumed. Its two zero-gapB and the last
archiveFF2B remain unowned. GraphicsbaseD6B4C8 is a ceiling, not archive end;
2306B afterD6ABC6 are excluded. Reassembly/hash/all4200B/723halfword4word/static
link formulas/all47e53owner disjointness pass. No build/guest test was run for
source inventory. Strictmodel/oneWineoracle are verified below; managed/runtime/
CLI/positiveordinary remain pending; no guest action-slot-opcode caps, primary-archive dependency or
wholeABI claim. e53-d52/USAe8-d7 unchanged, newLinux-USA-nativeWindows pending.
[Contract](../rom_data_scripts.md#european-auxiliary-script-archive-source-contract).


The strict auxiliary directory/archive model now reconstructs all4200B without
a ROMshadow; typed program+instruction entries preserve30interior links and133
physical programs. All nested JSON/source identities/own-callback+advance
 dependencies are checked. Warning-free Windows builds and the final new Wine
source filter pass40guards/442lookups/913dispatch+oneRAMopcode3/80advance/
489relative suffixes/10wrap/15raw/11wrapper probes.2887distinct canonicalB are
observed;1297other params+10gaps+6zero-rowB remain preserved/hash-checked unread.
First raw expectation fails on odd LDRH; corrected test-only observer, then a
specific32-bit high-rotation case added. First failure/earlier narrower passed
retry retained, unchanged model/core reused; only frontend/filter affected.
31839instructions177PCs coverageonly; no wholepredicates/interpreter/native/
fullstate-clock/ordinary claim. e53-d52/USAe8-d7 unchanged. [Model evidence](../rom_data_scripts.md#typed-auxiliary-archive-and-reduced-wine-source-oracle).


Managed EUR e54-d53now publishes both auxiliary directory/archive resources:
50files49resources/all47prior decoded/e53retained/USAe8-d7. One shared immutable
owner serves4200B after actualarchive→owndirectory authentication, with typed
index/callback/advance/opcode/parameter/gaps and no ROMshadow/hostinterpreter/
guestcap/save-nativeABI change. Shared370audio/D69coordinates and raw370FFA+/
D6ABC6+ remain correct. One warning-free affected Windows build/two exact new
Wine filters firstPASS26guards/lifecycle/all4200B and44original-interpreter-
compiled complete-state-clock prefixes/all6classes[6,65,18,19,15,2]B/unaligned
u32LDRH/interiorentries/signedcursor/rawopcodes/widths-mirrors-BIOS-SWI-DMA.
624instructions92PC52dispatchescoverageonly; whole predicates/interpreter/
callback ABI/ordinary unclaimed for that targeted scope. ActualproductionCLI
create/reuse-no-write52managedfiles/oldcache/all50file equality and one6000frame
positive ordinary Wine fullstate-clock-image-OAM-dispatch-audio pair nowpass:
96directoryB=32advance64callback/6commands,82archiveB=16index20opcode46parameter/
2banks2programs,0alignment/640guards. Other11commands6banks131programs/5alignment
spans targeted-only, newLinux-USA-nativeWindows/32:9pending without performance/
space work. No newfixture/navigation or oldproof repeat.
[Runtime and ordinary](../rom_data_scripts.md#auxiliary-script-production-cli-and-positive-ordinary-scene).


## European conditional normal script-special pose model

The151low-selector script references/20logical metadata14producer witnesses
retain108physical keys/12roots/32sequences241frames/11OBJ64objects/95pixel requests.
Strict lossless model reconstructs41802B and independently validates overlapping
initial/normal shared views before subtracting their union; only33spans6436B
have new ownership. One warning-free affected Windows frontend test build and
single reduced new Wine source oracle pass18guards/120constructor prefixes/
53null7proper resets/17scheduler-realDMA3 uploads/all5248newpixelB/wholeVRAM.
No oldcommands2/6/8or source/scene/census oracle repeated. Conditional source/
model closed; managed/runtime/CLI/ordinary and alternative/override/highregistry/
current-state coverage pending. e61/USAe8unchanged; newLinux-USA-nativeWindows-
32:9pending, no performance/storage measurements.
[Model and scope](../rom_data_unit_obj.md#typed-normal-script-special-pose-model).


The conditional normal script-special owner now publishes/consumes EUR e62-d61:
58files57resources/fouractual metadata14-scriptarchive-initial-normal dependencies,
all56prior decoded files/resources/coverage and e61 retained, USAe8 unchanged.
One warning-free Windows core/frontend build(core compiledonly) and two new Wine
filters firstPASS29guards/lifecycle/all6436newtypedB/seven sequences and23original-
interpreter-compiled fullstate-clock paths/33edges-mirrors-BIOS-SWI-DMA. Shared
initial/normal owners stay intact under41802graphB poisoning; new-role bytes
0dir240index28header920frame0OBJ5248pixel, nativeAPI0/36/6/43/0/1. Mixed2new4normal
transfers and2new2rawAPI preserve exact charges. No oldcommands/source/census/
scene repeat. ActualproductionCLI58generation60managed/all58matchaccepted/no-write
reuse and oneframeoldcacheattachment pass; zero newuse/smokeonly. Ordinarysource/
newpixel and newLinux-USA-nativeWindows-32:9pending, without measurements. [Managed runtime](../rom_data_unit_obj.md#conditional-script-special-normal-managed-runtime).


New actuale62ordinary locators distinguish typednull-source use from pixels:
Snow6000reads48newindexB/zero sequences-pixels; retainedRIVERstart618reads0newB.
No fullpair/guardbuild/oldcensus repeat. Newseven-sequence static81snapshot scan
finds no matching task; program159contains a producer but actualtrigger remains
unproven. Ordinarynewsequence/pixel/equivalence staysopen, original/cart inputs
preserved and no performance/otherplatform work. [Evidence limits](../rom_data_unit_obj.md#script-special-ordinary-source-locators-and-remaining-pixel-gate).


Il ramo script-special alternate ha ora una proiezione fonte separata sui
medesimi151riferimenti/20witness lowselector, half06metadata14e assegnazione
originale alternate corrente con override20clear.84celle/68null/14sequenze64frame,
9048Btotali; nuovi20span816B=216index8header80frame512pixel, resto8142alternate+
38initial+52normal. Modello offline/buildWindows warning-free/unico Winefilter passano19guardie,
108constructor prefixes/52null+2proper resets/two realDMAuploads e512pixelB/
interaVRAM esatti; avvioWine sandbox iniziale prima del test conservato, retry
stessabinaryPASS. e62-d61runtime invariato. Ordinarynewpixel/high16/override/currentstate
restano espliciti. [Contratto](../rom_data_unit_obj.md#conditional-alternate-script-special-source-and-typed-model).


EUR e63-d62pubblica/consuma il modello script-special alternate:59file58risorse,
metadata14/scriptarchive/initial/normal/alternate effettive,57precedenti dati/
record/coverage invariati/e62retained/USAe8. Nuovoowner serve solo20span816B;
31guardie/lifecycle/all816typedB e24percorsi Wine original-interpreter-compiled
stato-completo/clock/all20boundaries/BIOS-SWI-DMA/mixedowners/raw passano. CLI
reale/ordinary/high16/override/currentactor e nuoveLinux/USA/nativeWindows/32:9
restano da verificare, senza performance/spazio/nuovefixture.
[Contratto e prove](../rom_data_unit_obj.md#conditional-script-special-alternate-managed-runtime).


CLIproductionEUR e63-d62ora verificata su Wine: create/reuse59generation61managed,
all59matchaccepted/no-write, canonicalexactrestore/oldcacheoneframePASS. Zero
newalternate-script use nel singolo frame, nessuna prova ordinary/pixel/fullscene.
Fonte/modello/managed/native/CLI chiusi; prossimi ordinary/high16/override/current-
actor e nuoveLinux/USA/nativeWindows/32:9restano aperti. Nessuna misura performance.
[CLI e limiti](../rom_data_unit_obj.md#conditional-script-special-alternate-actual-production-cli).


Lettura mirata81snapshotEURper le due nuove sequenze alternate-script188/32-33:
nessun taskenabled/dormant corrispondente, inputinvariati. Leadchiuso negativo,
nessun retry/guest/fullpair; ordinarynewpixel/equivalence ancora aperto mentre
proseguono contratti fonte override/highregistry indipendenti.


Script override42/230ha un contratto fonte separato:167scriptrefs/13gruppi con
assegnazione originale forcedcurrent/flag20set/facing0..3,26mode-base witnesses/
156celle/11722B. Shared6252normal+4420alternate; nuovi40span1050B=384index24header
360frame26OBJ256pixel, sei sequenze18frame incluse due nuove copieargomentiop2.
Modellooffline/buildWindows senza avvisi/unico nuovo Winefilter primoPASS19guardie/
192constructor aliases/90null+6proper resets/two op2argumentcopies/two DMAuploads
256pixelB/interaVRAM. e63/USAe8/runtime invariati; nessuna identità statica
highregistry inventata.
[Contratto e limiti](../rom_data_unit_obj.md#conditional-forced42-and230script-pose-source-model).


EUR e64-d63ora pubblica/consuma script-override42/230:60file59risorse, tredeps
scriptarchive/normalbattle/alternatebattle/tutti58precedenti/e63/USAe8invariati.
Owner40span1050B;31guardie/lifecycle/alltypedB e27percorsiWine original-interpreter-
compiled stato-completo/clock/bothnewop2/newOBJ/40edges/BIOS-SWI-DMA/mixedowners/raw
passano. CLIproduzione/ordinary/highregistry-currentactor/newLinux-USA-nativeWindows/
32:9restano pending, nessuna misura performance/spazio o nuova fixture.
[Contratto e prove](../rom_data_unit_obj.md#forced-script-override-managed-runtime).


CLIproductionEUR e64-d63ora verificata su Wine: create/reuse60generation62managed,
all60matchaccepted/no-write/canonicalexactrestore/oneframeoldcachePASS. Zero new
usooverride nello smoke, nessunordinarypixel/fullsceneclaim; precedente e63eUSAe8
preservati. Proseguono ordinary/highregistry-currentactor, senza misure performance.
[CLI e limiti](../rom_data_unit_obj.md#forced-script-override-actual-production-cli).


Exactsix-target override ordinarylead:81snapshotEURautenticati, nessun taskenabled/
dormant corrispondente a42/32-33-40-41 o230/32-33, inputinvariati. Leadchiuso negativo,
no guest/build/retry/fullpair; ordinarynewpixel/equivalence ancora aperto, fonti
highregistry/currentactor restano indipendenti.


Highregistry-knownspecial script49ha un nuovo contratto fonte condizionato sui
16riferimenti/sevenpairs effettivi e93logicalunitrowsmetadata14con assegnazione
currentoriginale/override20clear.1302witnesses/2220logicalkeys1908cells/373842B;
shared358566Bda6owner, nuovi272span15276B (2392index84header1920frame10880pixel),
21seq96op1frames/37pixelrequests. Tutti59e64owner e41halfword4literal8BLpins passano,
nessun targetroot/mixeditem/conflict/extentvariabile. Modello lossless e primo
Wine mirato passano26guards/1196aliascostruttore/567null+21reset/37DMA10880B/
interaVRAM, sei visteprecedenti validate indipendentemente/aliasfisici coerenti.
Buildfrontend corretto senza warning; EUR e65-d64pubblica61file60risorse con
metadata14/archive e6visteprecedenti reali. DueWine mirati firstPASS33attach+
10dependencyguards/lifecycle/tutti15276typedB21seq e25percorsi original-interp-
compiled fullstate-clock/272bordi/BIOS-SWI-DMA/mixedold-raw-new. CLIreale Wine
prepare/reuse61generation63managed/all61accepted/no-write/oneframeoldcacheattach
firstPASS(tutti nuovi contatori0/smokeonly); ordinarynewpixel/equivalence pending,
USAe8invariato. Nuovo locatorreadonly81snapshot/21nuovesequenze trova0task
attivi o dormienti; leadchiuso/no guest-build-fixture-fullpair o retryinvariato.
Registryidentity/genericjob/outofcatalog/
redirect/currentchanged rimangono separati. Nessuna misura o nuova fixture.
[Contratto](../rom_data_unit_obj.md#registry-selectors-and-known-special-unit-source-projection).


Highregistry genericunit0/1ha un contratto separato per16highrefs/sevenpairs e
metadata52route0con campo concreto per modalità. CCC48legge job+5/alternate+7,
property4-5saltano overrideRAMproperty0; CFgetterszeroextendu16.116righe tutte
classificate:100bothconcrete/10normalonly/2bothFFFF/4routeFF,110normal100alternate
→2940witnesses6720refs; inizialeipotesi110joint corretta prima dei target.
83halfword6literal-dispatch11BLpins e tutti60e65owner autenticati prima dei target.
107sprite535pairs3210keys/cells2420null/777properseq5093frames/45OBJlists231objects/
1895pixelrequests677280B;12targetindexrootinitial48..59con witnesses exactfields.
Full796992B/shared697168B, nuovi659span99824B (5544index504header11760frame82016pixel),
1386newcells1260null/126newseq588op1/268pixelrequests. No nuoviOBJ/mixeditem/role/
extentconflict. Modello lossless e primoWine mirato passano34guards/2772newcell
alias/1260null126proper resets/268DMA82016B/interaVRAM/12initialrootviews; sei
precedenti viste validate indipendentemente. Primo buildfrontendWindows senza
warning; EUR e66-d65pubblica62file61risorse/metadata52-archive+6visteprecedenti.
Primi dueWine mirati passano36attach+10dependencyguards/lifecycle/tutti99824typedB/
126newseq e25percorsi original-interpreter-compiled fullstate-clock/659bordi/
BIOS-SWI-DMA/mixedold-raw-new. CLIrealeWinecreate/reuse62generation64managed/
all62accepted/no-write/oneframeoldcacheattach firstPASS(tutti nuovi contatori0/
smokeonly); ordinarynewpixel/equivalence pending/USAe8invariato. Nuovo locator
readonly81snapshot/126nuovesequenze trova0taskbody attivi/dormienti senza filtro
currentpointer/fineframe; leadchiuso/no guest-build-fixture-fullpair o retryinvariato. FFFF/redirect/
outofcatalog/changedactor/globalidentity e altre piattaforme separati; nessun
oldproof replay/fixture/navigazione/misure.
[Contratto](../rom_data_unit_obj.md#registry-selectors-and-generic-direct-job-source-projection).

Il contratto highregistry copre anche currentjob80/81/82/85routeFF con alternatejob
conosciuto route0/campo concreto:110normal100alternate →11760witnesses26880refs.
La fonte primaria è il routingFF, il campo sprite proviene dalla riga alternativa;
98halfword6literal11BLpins verificano CC35A/r4immutato/ritorno stessa property.
Tutte116righe e928classificazioni effettive conservate (840concrete56FFFF32loop).
Tutte61risorse/fonti correnti autenticate prima dei target: ogni2940witnessdiretto
ha quattro preimmagini, stesse3210celle/777sequenze/12rootviews/full796992B/sette
owner. Zero byte o risorse nuove; e66-d65invariato, proveWine metadata52/e66riusate
senza build/guest/fixture/navigazione. Ordinary/globalidentity/FFFF/nonreturning/
outofcatalog/changedactor e Linux/USA/nativeWindows/new32:9 rimangono aperti.
[Reindirizzamento e limiti](../rom_data_unit_obj.md#registry-generic-jobs-redirected-to-a-known-alternate-job).

I risultati FFFFconosciuti hanno ora classificazione del lettore separata:
196witnessdiretti+784reindirizzati/2240refs,71readerpins/trecodeblocks/twoliterals.
La lettura proprietà083B4C12restituisceDC86, directory083D4830restituisce14AE3F1E;
60query/30word restano nella regione14unmapped. CPUallinea/ruota16, MemMap usa
livebios_open: nessuna sequenzaROM/nullcostante o nonritornouniversale provato.
Le due lettureROM totalizzano6Bsenza owner tipizzato; rawfallback preservato,
migrazione6B/ordinaryreachability ancora aperti. Nessun IDcap/pointerfix/finto
countedasset o build/guest/fixture/navigazione/misure, e66/USAe8invariati.
[Letture e limiti](../rom_data_unit_obj.md#known-generic-job-ffff-results-and-raw-bus-reads).

Il lettore proprietà attore221C4ha ora un contratto fonte distinto:394C14+2*sprite,
u16→9AD0E/r8→costruttorer3→richiesta allocazione/task+16.398witnessnormal/alternate
job-special-override derivano182queryprima dei target;98halfword15BL/fourcodeblocks/
oneliteral e61owner/file/source passano. Quattro intervalli364Bnuovi/disgiunti,
SHAad328c8ebccfc43091dc878c06dab33b22521d7b617d4b11232e4a532ffc2208;
139word16/43word24, tutti bit u16conservati, buchi raw. Nessun catalogo248/IDcap,
flagbitfield o unitàVRAM dedotti. Sourcechild e modello lossless/oracoloWine ora
chiusi:20guards/tutti182getters364B/3rawfullword/quattronewtaskprefixes/duereuse/
quattrosetteru16/wholeRAMguards passano. I task esistenti mantengono il proprio+16.
Primo buildWindowsfrontend senza warning,82build83Wineguards11compiledinputs
invariati; nessun fullallocator/native-clock/ordinary claim per02. EUR e67-d66ora
pubblica63file62risorse/metadata52-14dipendenze/all61precedenti-e66preservati.
Primi dueWine passano17attach+4fullsetguards/lifecycle/364typedB/182maskrecord e
16percorsi original-interpreter-compiled fullstate-clock con364rawBalterati.
Quattrobordi/width-mirrors/realBIOSbus/SWI-DMA/mixednew-old-raw passano;84build/
86Wineguards20compiledinputs invariati. CLIproduzionecreate/reuse63generation/
65managed/all63accepted/no-write/oneframeoldcacheattach firstPASS(tutti nuovi
contatori0/smokeonly),154activeguards invariati. Nuovo ordinaryF09Wait3530CLI/
fullpair firstPASS10letture20Baggregati/dueword: ogniframestate-clock-central-wide-
OAM-dispatch-allaudio uguale;155activeguards21compiledinputs invariati, altre180
word ordinarytargetedonly. Guardpositivoopzionale, vecchioscenechild normal resta
chiuso/nessun altroproof ripetuto. No fullallocator/new32:9claim, USAe8invariato.
FFFFraw6B/altri input/changedactor/
globalidentity separati, nessuna ripetizione oldproof/fixture/navigazione/misure.
[Fonte e limiti](../rom_data_unit_obj.md#actor-property-words-from-known-sprite-assignments).


Il contratto dei chiamanti aggiuntivi del getter proprietà OBJ copre ora anche
l'intera finestra di input byte dei due LDRB UI:256 query scelte prima dei target,
non un'estensione di catalogo o un limite del getter fullword. Builder e quattro
percorsi mancanti autenticati con680halfword/25BL/21literal-controlflow pin.
I due percorsi u16 aggiornano la proprietà conservando l'allocazione; per le
assegnazioni concrete note riusano le182word. Tutti62 owner/file/fonti verificati:
184 parole condivise (182 proprietà + due parole dai quattro offset palette),
72 parole nuove/144B in394CF4/2,394CFE/2,394D0C/128,394E08/12,
SHA9d1239cc29ec40185c1605aed84b4cf74cedbc21cc217c001efb8348407253f9.
Nuovo complemento con source-contract e modello lossless/oracolo Wine chiusi:
14guard,512prefissi UI/tutti512B,otto skip,quattro confronti high-u16,quattro writer
byte e sei setter-prefix passano al primo tentativo. RAM fuori destinazioni e
VRAM/OAM/palette verificate;109build110Wineguard/13compiledinput invariati.
Le184word condivise provengono dalle due dipendenze senza copie duplicate.
Managed-runtime/CLI/ordinary pendenti, e67-d66 e USAe8-d7 invariati. FFFFraw6B/
unknown/changedactor/globalidentity/computedindirect restano separati; nessun
oldproof replay/fullUI-native-clock/fixture/navigazione/misura.
[Dettagli](../rom_data_unit_obj.md#additional-property-getter-callers-and-the-ui-byte-input-window).


Il complemento byte-input delle proprietà OBJ è ora gestito e consumato in
EUR e68-d67:64file/63risorse, tutti62record/63file precedenti ed e67 conservati.
Solo72word/144B nuovi;184word condivise restano ai due owner propertywords e
colorpalettes, dipendenze reali. Due nuovi Wine passano17attach+quattro dependency
check/lifecycle/tutti144B e72record, più equivalenza original/interpreter/compiled
fullstate-clock su getter/UI-prefix/mixed/fullword/liveRAM. Quattro bordi/
width-mirrors/realBIOSbus/SWI-DMA passano: API2new2property/2new2palette/2new2raw,
CpuSet2new4property/FastSet28new4property/DMA4new2palette. Nessun source/model/
oldscene replay o misura; CLIproduzione/ordinary restano pendenti, USAe8 invariato.
[Consumo e limiti](../rom_data_unit_obj.md#managed-ui-byte-property-words-and-typed-runtime-reads).


La CLIproduzione del complemento byte-property e68-d67 passa ora il primo
prepare/reuse Wine:64generation66managedfile/63risorse/due dipendenze/tutti64uguali
al dataset03, nessuna scrittura al riuso da cwdiversa. Primo buildWindows senza
warning/frozen0c0f5082.. e canonicale hash+mtime ripristinati; main contenuto invariato.
Un frame F09 autentico carica cache nativa precedente e usa1lettura/2B/1word nuova,
positivo nel controllo di allegato; ordinaryfullscene ancora aperta.182activeguard/
23compiledinput invariati; source/modello/runtime/vecchie scene chiusi senza replay.
Nessuna fixture/navigazione/privatepayload/misura, altre piattaforme da verificare.
[CLI](../rom_data_unit_obj.md#ui-byte-property-words-in-the-actual-production-cli).


La scena dichiarata per il nuovo complemento byte-property è ora chiusa: un
fullpairWine F09Wait3530 passa ogni frame fullstate-clock/central-wide/OAM/
dispatch/tutti audio, 476letture/952Baggregati/4word nuove positive.
Altre68word conservano prova lettore mirata. Un solo nuovo guard/report
opzionale, primo buildWindows senza warning, 183activeguard/24compiledinput
invariati. CLIoneframe già positiva, quindi nessun duplicatoCLI3530; vecchi
normal/e67property scenechild e source/model/runtime/CLI chiusi senza replay.
No fullUI/allocator/allwordordinary/new32:9 o altra piattaforma dichiarati;
nessuna fixture/navigazione/privatepayload/misura performance-spazio.
[Scena](../rom_data_unit_obj.md#ordinary-ui-byte-property-use-and-scene-equivalence).


La lista UI selezionata dalla proprietà==16, `3950A4`, ha ora contratto fonte:
48halfword/4literal/4BL e vecchie prove UI/lettore verificano quattro rami A4/B2,
1F6C→1500 e stride6. Tutti63owner/fonti autenticati prima del payload. Un header2
produce14B nuovi/twoobjects, SHA1f9cfc55a363c69dcf8422d47641705427d4291a3801ff1c7646bfb8bc5daa64;
B2/14B già tipizzati sono alias esatto. Tre riferimenti aggiuntivi proiettano
126refs/74liste175oggetti1198B nello stesso owner `graphics.obj-static-lists`,
senza nuova famiglia/formato. Fonte chiusa, modello/pubblicazione/runtime/CLI/
ordinary A4 pendenti; e68-d67/USAe8-d7 invariati. Nessuna vecchia campagna CFG/
oracolo/scene ripetuta, build/guest/fixture/navigazione o misura.
[Dettagli](../rom_data_obj.md#property-selected-ui-list-source-extension).


**ROMDATA-02-03-UI-PROPERTY-OBJ-LIST-EXTENSION completato EUR (2026-10-26).**
e69-d68 integra tre riferimenti nello stesso Catalog/List/Object/owner/formato,
126refs/74liste175oggetti1198B; nuova A4 solo14B/twoobjects, B2 condivisa.
Stessi64file/63risorse e dipendenza dimensioni; tutti73record precedenti,
62altri file/record ed e68 invariati. Build Windows corretto senza warning
(primo errore solo costante test non qualificata). Due primi filtri Wine passano:
roundtrip/otto guard/gestione pubblicazione-riparazione-spostamento/lifecycle;
due draw original-interpreter-compiled fullstate-clock/709istruzioni/4dispatch/
36compiledB, due host publication, bordi-width-mirror-BIOS e mixedAPI2new2B2/
DMA-CpuSet14new2B2/FastSet4prior14new14B2. Runtime handler invariati.
77build79Wineguard/29compiledinput esatti. CLI reale04 e ordinaryA4guest-host05
restano da fare; USAe8-d7/Linux/nativeWindows/new32:9 pendenti. Nessun replay
vecchie prove/scene, fixture/navigazione o misura performance-spazio.
[Integrazione e limiti](../rom_data_obj.md#managed-ui-property-list-extension-and-typed-consumption).


**ROMDATA-04-UI-PROPERTY-OBJ-LIST-CLI completato EUR/Wine (2026-10-26).**
Primo buildproduzione Windows senza warning/frozen17488009..; canonicale ripristinato
hash+mtime, main contenuto invariato. Primi prepare/reuse/oldcacheoneframeWine da
/tmp passano64generation66managed/63risorse/74span1198B/tutti64uguali al dataset03,
riuso senza scritture. 148activeCLIguard/29compiledinput esatti.
OrdinaryA4sceneequivalence ancora aperta; nessun vecchio oracolo/scena ripetuto,
fixture/navigazione/privatepayload o misura performance-spazio. Altre piattaforme
restano da fare.
[CLI e limiti](../rom_data_obj.md#ui-property-lists-in-the-actual-production-cli).


**ROMDATA-05-UI-PROPERTY-OBJ-LIST-LOCATORS: osservazioni negative chiuse, ordinary aperto (2026-10-26).**
F09oneframe/Partyunit320 non usano A4. Un solo locator banca120 su copiaEUR
esatta autorizzata più normaleDown usa B2guest-host, A4assente; endpoint ispezionato,
151guard/29compiledinput esatti. Nessun fullpair o ripetizione allungata/altri slot.
Quattro snapshot letti conchecksum e prime7righe visibili→proprietà24; nessuna
portata globale inferita. Nuovi setter02AE70(input/animation/tag),02AF14(animation),
02AF5C(solo stato) distinti. Prima censusBL per due nuovi target:4updater/8setter;
prefisso0578EE assegnaanimation24 al preview umano. Percorso completo e witness
ordinaryA4guest-host/fullscene ancora aperti; leggere controllo prima di altroinput.
Vecchi oracoli/CFG/scene chiusi, nessun nuovo build/fixturegiocata/misura; Linux/
USA/nativeWindows/new32:9 da fare.
[Locator e prossimo criterio](../rom_data_obj.md#ordinary-ui-property-list-locators).


**ROMDATA-05-UI-PROPERTY-OBJ-LIST-HUMAN-ANIMATION completato per la scena dichiarata EUR (2026-10-26).**
Join originale25halfword/6switchword: valutatoreD2D44→6375C→1092→063230/02AF14
imposta24 per risultati1/2, tag1+proprietà16 seleziona A4. Primo CLI WineDispatch1840
positivo A4guest-host/15evaluationB;1400negativo conservato perché fermava prima
dell'evento1600, confine corretto giustificato da prova originale1840 già positiva.
Primo buildWindows test senza warning/frozen427845f2..; solo5righe guard/report
opzionale A4 nel validator, runtimeprodotto invariato. Primo fullpairWine1840 passa
tutti stati-clock/central-wide/OAM/dispatch/tutti audio e nuovo gateA4guest-host;
5stream con campioni reali.152guard/29compiledinput esatti; originali/input/copia
privata autentici, nessun cartedit o checkpoint derivato nuovo. Due preflightprivati
corretti prima di Wine, nessun guestretry. 01/02/03/04/05 dichiarati chiusi; vecchi
oracoli/scene e locatornegativi non ripetuti. Linux/USA/nativeWindows/new32:9,
fullUI/indiretti globali/altre famiglie restano aperti; nessuna misura performance-
spazio. [Scena e limiti](../rom_data_obj.md#positive-dispatch-use-and-scene-equivalence).


**ROMDATA-02-UNIT-OBJ-FFFF-READER-FIELDS completato modello EUR (2026-10-26).**
Nuovo modello lossless: query65535, wordu16proprietà+wordu32directory encoded,
due range6B,980witnesscondizionali/2240riferimenti (196diretti784redirect) con
dipendenze metadata52/archive49. Modelli validi precedenti e aliasbyte invariati;
nessun root/sequenza/puntatore corretto/cap interpretato. Primo buildWindows senza
warning/frozen83ffcd75.. e primo singolo filtroWine passano13guard/3numericroundtrip/
980record/6B e due lettori originali completi28istruzioni, registri/RAMguard.
156build157Wineguard/35compiledinput esatti. Classificazione71pin/60query e vecchie
prove A4/modello/runtime/scene chiuse senza replay. Pubblicazione/ownerruntime03,
nativeclock/ordinary pendenti; produzione e69-d68/64file63risorse e USAe8-d7 invariati.
Linux/USA/nativeWindows/new32:9 da fare, nessuna fixture/navigazione/misura performance-
spazio. [Modello e limiti](../rom_data_unit_obj.md#typed-model-for-known-ffff-reader-fields).


**ROMDATA-03-UNIT-OBJ-FFFF-READER-FIELDS completato managed/runtime EUR (2026-10-26).**
e70-d69:65file64risorse, tutti63record/file decodificati precedenti ed e69
conservati; USAe8-d7 invariato. Nuova risorsa numerica con dipendenze reali
metadata52/scriptarchive, modello980witness2240refs invariato. Owner immutabile
serve solo2Bproperty+4Bdirectory da campi tipizzati; gate3B4/3D4, contatori e due
bitcampo. Puntatore completo, BIOSaperto vivo/unalignedLDRrotate16 e controllo
guest preservati, nessuna sequenza/target/null/cap inventata. Primo buildWindows
core/frontend senza warning e primi due nuovi filtri Wine PASS:20attachguard,
quattro dependencyguard, publication/lifecycle directory-indisponibile, tutti6B
raw alterati/duecampi e vecchioowner distinto. Original/interpreter/native14path
stato completo/clock uguali: getter/directory/zeroearlyreturn/treBIOSprefix,
liveRAM/mixed reads. Due bordi/width/mirrors/realBIOSbus; API2new2raw, CpuSet
2new6raw/FastSet4new28raw/DMA4new4raw preservati. Source/modello/A4/oldproof chiusi
senza replay. ProduzioneCLI04 e ordinary05 restano da fare, nessuna fullsprite/
allocatorABI dichiarata. Linux/USA/nativeWindows/new32:9 pendenti. Nessuna fixture/
navigazione/privatepayload/performance-spazio/Android/package/commit/push.
[Consumo e limiti](../rom_data_unit_obj.md#managed-ffff-reader-fields-and-typed-bus-consumption).


**ROMDATA-04-UNIT-OBJ-FFFF-READER-FIELDS-ACTUAL-CLI completato EUR/Wine (2026-10-26).**
Primo buildproduzioneWindows senza warning/candidato congelato; canonicale hash+
mtime esatti ripristinati/main contenuto invariato. Prime tre chiamate Wine
prepare/reuse/oldcache-oneframe da cwd estranea passano65generation67managed/
64risorse/tutti65uguali al dataset03/due dipendenze reali/riuso senza scritture.
Checkpoint F09 originale e cache precedente integri: solo prova attachment,
ordinarypositive/equivalenza non dichiarate. Source/modello/runtime/oldscene
chiusi senza replay; USAe8-d7 e Linux/nativeWindows/new32:9 da verificare.
Nessuna fixture/navigazione/privatepayload/performance-spazio/Android/package/
commit/push. [CLI e limiti](../rom_data_unit_obj.md#actual-production-cli-for-ffff-numeric-fields).


**ROMDATA-05-UNIT-OBJ-FFFF-READER-FIELDS-EXISTING-CONTEXT-LEAD chiuso, ordinary positivo aperto (2026-10-26).**
Nuovo locator readonly sugli81checkpoint EUR esistenti: predicato actor+34FFFF/
unitkind0-1/currentjob-alternatejob da980witness, entrambe modalità/override/
null-dormant-outside conservati, senza richiedere targetsequenza inesistente.
159candidati potenziali in61snapshot, tutti kind0/job0/alt0/override20clear:
157bodypointer null e due01480168fuoriEWRAM. Nessun taskcorpo disponibile inEWRAM,
nessuna identità registry/sameassignment/actorvalidity dedotta. Non è prova di
irraggiungibilità globale e non fornisce scena positiva: ordinaryreader/fullpair
restano aperti. Hash/mtime/header/payload/ROM/input/modello/closedcontract esatti.
Source/modello/runtime/CLI/126sequence/oldscene chiusi senza replay; nessun nuovo
build/Wine/gameplay/fixture/navigazione/privatepayload/performance-spazio.
Linux/USA/nativeWindows/new32:9 pendenti. [Locator e limiti](../rom_data_unit_obj.md#existing-checkpoint-locator-for-ffff-numeric-readers).


**ROMDATA-01-UI-JOB-LOOKUP-SOURCE completato per proiezione EUR20slot (2026-10-26).**
Root399750 condiviso da sei lettori D0538/D0548/D0568/D0588/D05A8/D05C8: bytejob
oppureCBEC8property1/4/6/9/0, troncamenti e namehelper165E8 originali. Dominio
selezionato prima dei dati dal12CA98: lista indici dei20slotRAM02002E78/stride16,
mode3massimo+5/tieinclusivo; un singolo slot occupato consente ogni0..19. R6
preservato attraverso CFF70clear prima di D05C8.268halfword/27criticalpin/9literal/
8BL autenticati. Nessun capglobale/extentfisicocompleto/adjacency/identitàdi gioco
inventato; altricallermatch raw non automaticamente codicesemantico. Venti job
u8/20B/unarange SHA969b2cd5.. distinti,20righemetadata52 ricostruite, disgiunti
tutti64owner attuali. Nessun target sprite seguito. Sourcechild chiuso; modello/
Wineoracle/managed/runtime/CLI/ordinary aperti. e70-d69/65file64risorse e USA8-7
invariati; nessun build/guest/test/fixture/navigazione/privatepayload/performance-
spazio/Android/package/commit/push. Linux/USA/nativeWindows/new32:9 pendenti.
[Contratto e limiti](../rom_data_gameplay.md#twenty-slot-job-lookup-source-contract).


**ROMDATA-02-SLOT-JOB-LOOKUP-MODEL completato modello EUR (2026-10-26).**
Modello strict20(slot,u8job)/20B/sourcebase/hash/byte_at/metadata52diretto; Catalog
verifica EUR/treblockproducer-reader. Due solemoduledecls, nessuna registrazione
Extraction/runtime: e70-d69/65file64risorse/USA8-7 invariati. Primo buildWindows
frontend senza warning e primo unico filtro Wine PASS:20entry indipendenti/20B/
12guard JSON-shape-source-dependency/roundtrip/disgiuntitutti64owner.123percorsi
originali:21barejob/81propertywrappercompleti/21property0prefix prima di165E8,
trealiasinputhigh. Tutti20B ai sei LDRB originali, callee/fullRAMfuori20Bstack/
VRAM-OAM-palette preservati. Nessun capguest20/null/hostfallback, codeparser268pin/
metadata52/FFFF/A4/oldproof non ripetuti. Managed/runtime03/CLI04/ordinary05 aperti;
nessuna nuova fixture/navigazione/privatepayload/performance-spazio/Android/
package/commit/push. Linux/USA/nativeWindows/new32:9 pendenti.
[Modello e limiti](../rom_data_gameplay.md#typed-twenty-slot-job-lookup-and-original-reader-proof).


**ROMDATA-03-SLOT-JOB-LOOKUP-RUNTIME completato managed/runtime EUR (2026-10-26).**
e71-d70:66file65risorse/gameplay.slot-job-lookup/slot-job-lookup.json, dipendenza
reale metadata52; tutti64record/file precedenti ed e70 conservati, USA8-7 invariato.
Owner immutabile20u8/20B/page399/maschera20record, originali inputlow8/slotselection/
clear/properties/name/rawoutside preservati, nessun cap20guest. Primo buildWindows
core/frontend senza warning e due nuovi filtri Wine PASS dopo stop ENOSPC e retry su filesystem temporaneo:18attachguard/3
dependencyguard/publication/lifecycle directory-indisponibile/tutti20Brawalterati/
tutti20bit/ownerFFFdistinto. Sedici percorsi original/interpreter/native fullstate-
clock uguali: getter/properties/nameIDprefix/highinput/slot20raw/liveRAM e12CA98
mode3slot19→CFF70clear→D05C8prima165E8. Bordi/width/mirrors/realBIOSbus; API2new2raw,
CpuSet20new4raw/FastSet20new12raw/DMA20new4raw preservati. Vecchi64owner confermano
rawneighbors. Source268pin/model123reader/oldproof chiusi senza replay. CLI04/
ordinary05 aperti; Linux/USA/nativeWindows/new32:9 pendenti. Nessuna fixture/
navigazione/privatepayload/performance-spazio/Android/package/commit/push.
[Consumo e limiti](../rom_data_gameplay.md#managed-twenty-slot-job-lookup-and-typed-bus-consumption).


**ROMDATA-04-SLOT-JOB-LOOKUP-ACTUAL-CLI completato EUR/Wine (2026-10-26).**
Primo buildWindowsproduzione senza warning/candidato congelato; canonicale hash+
mtime esatti ripristinati/main contenuto invariato. Prime tre chiamate Wine
prepare/reuse/oldcache-oneframe da cwd estranea passano66generation68managed/
65risorse/tutti66uguali al dataset03/dipendenzametadata52/riuso senza scritture.
Rootprivato su filesystemtemporaneo mediante linkworkspace dopoENOSPC03, nessun
ulteriorecleanup/formatobudget/buildretry. F09/cacheoriginali integri: sola
provaattachment, ordinarypositivo/equivalenza aperti. Source/modello/runtime/
oldproof chiusi senza replay; Linux/USA/nativeWindows/new32:9 da verificare.
Nessuna fixture/navigazione/privatepayload/performance-spazio/Android/package/
commit/push. [CLI e limiti](../rom_data_gameplay.md#actual-production-cli-for-the-twenty-slot-job-lookup).


**ROMDATA-05-SLOT-JOB-LOOKUP-BANK-ORDINARY completato per scena EUR dichiarata (2026-10-26).**
Primo nuovo CLIbank120positivo2read/2B/1slot, con checkpointcopiato/inputDown8..20
esistenti e datasetproductione71generation-GmxCy2. Unicoscopo nuovojoblookup,
vecchioA4negativo/B2positivo chiuso. Solo5lineoptionalguard/report nelvalidatore
scene, runtime/prodotto/vecchiguard/stati invariati. Primo buildWindowsfrontend
senza warning e primo filtro Wine esatto PASS:120fullstate-clock da3660a3780/
central-wide/OAM/dispatchnative-fallback e mixed-PSG-FIFOleft-right-HLEbitexact.
Tutti5samplecount[263340,263340,31680,31680,263264]positivi; nuovouso2B1slot uguale
CLI. Altri19slot ordinari targeted-only. CatturaCLI284x160ispezionata:sfondoBank,
pannelliUI assenti dalrawframebuffer; nessuna GUIcompositafinale/fullname-actor-
allocatorABI/new32:9 dichiarata. Input/copia/originale/cache/dataset integri;
nessuna fixture/navigazione/privatepayload/performance-spazio/oldproof replay.
Source/model/03/04/05dichiarati chiusi, fullTODOattivo/Linux-USA-nativeWindows-
new32:9pendenti. [Scena e limiti](../rom_data_gameplay.md#ordinary-bank-scene-use-of-the-twenty-slot-job-lookup).


**ROMDATA-01-POST-SLOT-LOOKUP-READERS completato per tre API RAM EUR (2026-10-26).**
D05E8anynonnull eD0618count leggono solo le primeword di20slotRAM02002E78/stride16;
non validano puntatori/liveness, D05E8ritorna presto. D0650incrementa byte02001F68
solo seold<=98, lascia99..255invariati (nessun clampglobale). Tutti ignorano arg
entranti perquestiindirizzi; sei literal sono supporto eseguibile RAMbase/offset,
nessuna nuova radice ROMasset.68halfword/30criticalpin/6literal/treblock completi
verificati;3/2/1BLmatch raw restano candidati/non completezzasemantica. Nessun
export diRAM viva/nuovadecoder-risorsa-owner/revisione/cap20tabellaROM. e71-d70/
66file65risorse/USA8-7 invariati. ProssimoD0670parziale escluso. Sourcechildchiuso,
vecchieprove/Bank non ripetute; nessun build/guest/test/fixture/navigazione/
privatepayload/performance-spazio. Linux/USA/nativeWindows/new32:9 pendenti.
[Stato vivo e limiti](../rom_data_gameplay.md#live-state-helpers-following-the-slot-job-lookup).


**ROMDATA-01-NATIVE-ROM-DATA-RESIDUE completato per il campione EUR Bank12
(2026-10-26).** Un osservatore solo di test avvolge le API native read8/16/32 e
sottrae i20756intervalli uniti dei65piani sorgente e71. Primo build Windows senza
avvisi e un filtro Wine:3166letture di contesto/3683byte immagine;198letture
includono676byte fuori piano,79intervalli osservati. Stato completo/clock,
central-wide/OAM/dispatch e cinque flussi audio coincidono a ogni frame3660–3672.
676byte sono indizi da autenticare, inclusi cinque word370E70/74/80/84/EB0;
nessuna nuova risorsa/revisione o completezza globale. PC esportato può essere
stale; interprete/HLE/DMA/costanti native e altre scene fuori campione.73input
hash-mtime esatti; main/CAPI/vecchi validatori invariati, solo nuovo modulo test
con dichiarazione cfg. Nessuna nuova fixture/navigazione/misura prestazioni.
Le vecchie accettazioni Bank120/source/model/runtime/CLI restano chiuse.
Linux/USA/nativeWindows/new32:9 da fare.
[Contratto, trasparenza e limiti](../rom_data_runtime.md#bounded-native-rom-api-residue-inventory).


**ROMDATA-01-NATIVE-RESIDUE-HIGH-WORD-READERS completato staticamente EUR
(2026-10-26).** I cinque word osservati370E70/74/80/84/EB0 sono supporto di
esecuzione: due directory di indirizzi registri DMA e un ingresso ThumbIWRAM
passato aBXr2.392halfword/47pin/5literal/4BL in sei blocchi interi autenticati;
loadreali0846/09B2/0D56/3A7E/11DA/11EE distinti dalPCesportato. I chiamanti
originali provano due proiezioni4x4B370E68/78 e un word370EB0,36Bdisgiunti da65owner;
20Bosservati classificati,656Blower ancora da classificare. Nessun capglobale,
RAMbody/ABI/scheduler o nuovoasset/owner/revisione inferito; dipendenza ROM/codice
esplicita e676Bancora fuori piani gestiti. Nessun build/guest/test/fixture/
navigazione/misura/vecchia accettazione ripetuti. e71-d70/USA8-7 invariati;
Linux/USA/nativeWindows/new32:9 pendenti.
[Directory hardware e controllo di salto](../rom_data_runtime.md#high-offset-native-residue-is-hardware-and-branch-support).


**ROMDATA-01-NATIVE-RESIDUE-LOWER-READERS completato per testimoni statici EUR
(2026-10-26).**656Bosservati:158wordliteral/632B con184percorsi Thumblineari entro96B
dai contestisalvati (507halfworddistinti), e6word/24B in5tabelle salto con82halfword
di prefisso bounds/indice/load/MOVpc autenticati. Proiezioni8/35/69/48/5entry;
solo0/0/27/4-6/1osservate. Non è una traccia dinamica istruzione/modo né prova di
tutti gli usi. Insieme ai20Bhardware/branchhigh, tutti676Bfuori piano hanno riscontri
statici di supporto codice; restano fuori dai65owner/dipendenzaROMoriginale.
Nessuna nuova famiglia/radiceasset/owner/revisione/completa migrazione inferita.
Nessun build/guest/test/fixture/navigazione/misura/vecchia accettazione ripetuti.
e71-d70/USA8-7 invariati; interprete/HLE/DMA/costanti native/altre scene e
Linux/USA/nativeWindows/new32:9 separati.
[Literal e tabelle di salto](../rom_data_runtime.md#lower-native-residue-has-literal-and-switch-table-witnesses).


**ROMDATA-01-NATIVE-ROM-DATA-RESIDUE-F09 completato per secondo campione EUR
(2026-10-26).** Stesso eseguibile osservatore congelato, nessun build: un filtro
Wine12frame8476→8488 da checkpoint/inputF09esistenti preserva stato/clock/buffer/
OAM/dispatch e cinque audio.13671letture contesto/13961Bimmagine;604Bfuori piano,
584condivisi conBank e20nuoviB/5word. Tutti5hanno testimoni ThumbPC-relative
lineari (20halfword); fonte statica, non traccia dinamica/modo/tutti gli usi.
Unione due campioni696Bancora fuori65owner, nessun asset/revisione inferiti.
76inputhash-mtime e codice compilato esatti, helperretention0percorsi. Nessuna
fixture/navigazione/misura o vecchia accettazione F09/Bank ripetuta. Altri percorsi
interprete/HLE/DMA/costanti e Linux/USA/nativeWindows/new32:9 restano separati.
[Secondo campione](../rom_data_runtime.md#second-native-api-sample-from-the-existing-f09-checkpoint).


**ROMDATA-01-HOST-ROM-READ-RESIDUE completato per collector e campione EURF09
(2026-10-26).** FeatureCargo opzionale non-default rom-data-read-audit: collector
thread-local indirizzo/lettore/lunghezza su bus risolto e hostaudio/sprite/tabelle/
keyboard selezionati; fetchistruzioni interprete escluso, esecuzione/clock/API/
savestate invariati. Primo build corecompilato/frontendfallito per qualifica
c_void mancante nel test; correzione solo test e buildincrementale senzaavvisi.
Un filtro Wine: probeARM/Thumb/width/mirror/audio/lifecycle e12frame8476→8488
stato-clock-buffer-OAM-dispatch-cinqueaudio identici.23061richieste/19984Bimmagine,
1146Bfuori65owner; include tutti604Bfuoripiano del vecchioAPIreport e aggiunge542B.
90nuoviBalti/6intervalli restano sorgenti da autenticare,64Banche daSpritePeek.
Nessun formato/asset/extent/owner/revisione inferito.76inputesecuzione+1report
confrontohash-mtime esatti, nessuna vecchia prova/fixture/navigazione/misura.
e71-d70/USA8-7 invariati; KeyboardWidthsnonattivo/altrethread-scene/accessihost/
fastpathHLEtipizzati fuori scope; Linux/USA/nativeWindows/new32:9 pendenti.
[Collector, prova e limiti](../rom_data_runtime.md#optional-bus-and-host-rom-read-audit).


**ROMDATA-01-BUS-HOST-HIGH-RESIDUE completato per contratti sorgente EUR
(2026-10-26).**90Bnuovi classificati:64Bdue listeOBJ indirette394F4C/26B/4oggetti e
40D89E/38B/6oggetti;10Bparametri (2thresholdphase,4wordverticali signed);16Bpuntatori
ThumbIWRAM di controllo.295halfwordfinestra/53pin/7literal+hashABIriusati. Copia
originale0924E8 provaD968E8..D97728→03005F28..03006D68, costruttoreD9752C scrive
lista+158; rawROMdis aRAMtargetvuoto rifiutato. Un solo frame Wineproducerargs
esistenteF09 conferma2lettori/10oggetti, nessuna equivalenzavecchia ripetuta.
40D89Eda task+24 è osservazioneindiretta, nonliteralinventato; creazionecomando
nonricostruita. TimingconfrontaOLDcounter, fasepostupdate0..2 nonglobalcap;
correzionequerybyte>>1 ha128possibiliindici, solo4..7nuovi qui.64Blist+10Bparametri
ancora da modellare/pubblicare;16Bcontrollo restano dipendenzaROM/codice;452B
lower separati. e71-d70/65owner/USA8-7 invariati, nessun build/fixture/navigazione/
misura. Linux/USA/nativeWindows/new32:9 pendenti.
[Liste indirette e prove](../rom_data_obj.md#observed-indirect-obj-list-sources-beyond-the-direct-catalog),
[parametri e controllo](../rom_data_runtime.md#high-bus-and-host-residue-source-contracts).


**ROMDATA-02-OBSERVED-OBJ-LISTS-MODEL completato per due liste EUR
(2026-10-26).** API isolataobj_lists::observed riusaList/Object:2liste/10oggetti/
64B, identità/count/shape/fieldwidth/JSONstrict/sourcecoerente autenticati, root
constructor-bound distinto da ordinary-task-argument. Primo buildfrontendWindows
senzaavvisi e un filtro Wine:64B/2span/14guardie/6percorsi originali ROM1500/170C/
copiedRAM030065A8, output/matrici/stack/callee/RAMesterna/OAM-VRAM-palette esatti.
JSONcanonico4530552b..;75guardieinput-artefatti e codicecompilato invariati.
Solo2dichiarazioni nei fileesistenti; vecchioCatalog74/175oggetti1198B/e71-d70/
66file65risorse/USA8-7 invariati. Nessuna pubblicazione/owner/revisione o prova
ordinary da dedurre; modellochiuso, managedruntime/CLI/nuovo uso dafile ancora
aperti. Nessun vecchio74/ABI/copier/scenecheck ripetuto, nuova fixture/navigazione/
misura.10Bparametri e452Blower separati; Linux/USA/nativeWindows/new32:9 pendenti.
[Modello e verifiche](../rom_data_obj.md#isolated-typed-model-for-the-two-observed-indirect-lists).


**ROMDATA-03-OBSERVED-OBJ-LISTS-RUNTIME completato per due liste EUR
(2026-10-26).** e72-d71 integra le due identità indirette nella risorsa OBJ
esistente: 76 liste, 185 oggetti e 1262 byte sorgente. Restano 126 riferimenti
diretti, 66 file/65 risorse, stessa dipendenza dimensioni, tutte le 74 liste
precedenti, gli altri 64 file/record ed e71. USA8-7 invariata. Primo build
Windows senza avvisi e due filtri Wine passano pubblicazione/riparazione/move,
12 guardie di attivazione e Load/Undo/backup/restart; sei percorsi originali/
interprete/nativo hanno stato completo e clock uguali con tutti i 64 nuovi byte
ROM alterati. Bordi/larghezze/mirror/BIOS, sei trasferimenti DMA-CpuSet-FastSet,
letture API 2B vecchi+2B nuovi e 2B raw modificati+2B nuovi, sei pubblicazioni
host passano. 5571 istruzioni/2418 dispatch sono solo copertura; 265 byte typed
serviti da API native. File esattamente uguale al modello accettato, 77 guardie
input/eseguibile e codice compilato invariati; completamento artefatti elimina
solo copie interrupted/moved. CLI di produzione e nuova equivalenza ordinary
restano aperte; dieci byte parametri e 452 byte lower separati. Nessun vecchio
modello/ABI/scena ripetuto, nuova fixture/navigazione/misura. Linux, USA, Windows
nativo e nuovo32:9 da verificare.
[Integrazione e prove](../rom_data_obj.md#managed-observed-indirect-lists-and-typed-consumption).


**ROMDATA-04-OBSERVED-OBJ-LISTS-ACTUAL-CLI completato EUR (2026-10-26).**
Primo build Windows di produzione senza avvisi; CLI separata c6d674d7..,
eseguibile canonico hash/mtime ripristinati esattamente, contenuto main invariato
con sola invalidazione Cargo via mtime. Tre primi comandi Wine da cwd estranea
passano create/reuse/oldcache-oneframe: 66 file generazione/68 managed, tutti i
66 uguali al dataset runtime, 65 risorse/76 span OBJ/dipendenza dimensioni.
Riuso senza modifiche hash/mtime. F09/input/cache originali a16:9: entrambe le
nuove liste00394f4c/0040d89e guest=true host=true; totali OBJ147letture/147B/
14B count/133B attributi/12letture host. Uso positivo, equivalenza ordinary
completa ancora aperta. Tutte143guardie CLI e implementazione/canonico/originali
esatti; helper artefatti zero percorsi. Nessun vecchio source/model/runtime/
producer logging/scenario ripetuto, nuova fixture/navigazione o misure.
Linux/USA/Windows nativo/nuovo32:9 da verificare; dieci byte parametri e452lower
separati. [CLI e prove](../rom_data_obj.md#actual-cli-for-the-observed-indirect-lists).


**ROMDATA-05-OBSERVED-OBJ-LISTS-ORDINARY completato per il primo frame F09 EUR
(2026-10-26).** Solo cinque righe opzionali al validatore esistente richiedono
entrambe le nuove liste guest+host. Primo build Windows senza avvisi e un solo
filtro Wine: frame8476→8477, stato completo/clock/immagini centrale-larga/OAM/
dispatch e tutti gli audio uguali. Cinque flussi con campioni reali
[2196,2196,264,264,2120]; contatori OBJ147letture/147B/14count/133attributi/
12host esattamente uguali alla CLI. 6803native/22849fallback solo copertura;
75guardie input/eseguibile, originali/cache/dataset/canonico esatti. Delta di
cinque righe verificato sul preimage, altre implementazioni invariate; helper
artefatti zero percorsi. Source/model/managed/runtime/CLI e ordinary dichiarato
chiusi; altre scene/indiretti/composedGUI/ABI/nuovo32:9 aperti. Nessun vecchio
12/3530/scenario/modello/runtime ripetuto, nuova fixture/navigazione o misure.
Dieci byte parametri/452lower e Linux/USA/Windows nativo separati o da verificare.
[Equivalenza e limiti](../rom_data_obj.md#ordinary-first-frame-equivalence-for-the-observed-lists).


**ROMDATA-02-OBSERVED-TASK-PARAMETERS-MODEL completato EUR (2026-10-26).**
API isolata observed_task_parameters per due frammenti indipendenti: soglie u8
fasi1/2 a371065..67 e quattro correzioni signed16 indici4..7 a394E14..1C.
Identità/indici/JSONstrict/ricostruzione10B/due span/code-literal-source hash;
nessuna durata/cap globale/estensione fisica dedotta. Fase0/altri byte fuori
modello; correzioni0..3 conservano l'owner precedente,8..127 restano esterne.
Primo build Windows senza avvisi e un filtro Wine:27guardie e18prefissi originali
(10old-counter/phase-wrap,8alias-bitbasso/signed-sub), RAM/callee/OAM-VRAM-palette
esatti. Modello canonico4b52c5f1..;79guardie input/eseguibile e implementazione
esatte. Solo due dichiarazioni nei vecchi hub; pipeline/e72-d71/66file65risorse/
76listeOBJ/USA8-7 invariati. Nuovi10B disgiunti da tutti gli owner e non ancora
pubblicati; managed/runtime/CLI/ordinary dei parametri aperti. Helper artefatti
zero percorsi. Nessun source/295pin/vecchio modello-ABI-runtime-CLI-scena
ripetuto, nuova fixture/navigazione/misura.452lower separati;
Linux/USA/Windows nativo/nuovo32:9 da verificare.
[Modello e limiti](../rom_data_gameplay.md#isolated-model-for-observed-task-parameter-fragments).


**ROMDATA-03-OBSERVED-TASK-PARAMETERS-RUNTIME completato EUR (2026-10-26).**
e73-d72 pubblica gameplay.observed-task-parameters/observed-task-parameters.json:
67file/66risorse, tutti65file/record precedenti/e72/76listeOBJ conservati. Due
campi indipendenti, nessuna dipendenza inventata, solo10B/due span371065..67 e
394E14..1C; pagina371/394 compone owner precedenti e ROM, senza cap/ABI/clock/
stato nuovi. Due primi filtri Wine passano13rifiuti attach/3guardie fullset,
pubblicazione/riparazione/move/lifecycle e24prefissi originali-interprete-nativo
con stato completo/clock uguali e tutti10B raw alterati. Inclusi fasi0/3/255,
correzioni0/16/255, bordi/mirror/BIOS, sei DMA-CpuSet-FastSet, letture API
2nuovi+2vecchi con raw vecchi alterati e2nuovi+2raw modificati.26B nativi;
316istruzioni/24dispatch solo copertura. Modello4b52c5f1.. esatto,74guardie e
codice compilato esatti. Primo build core compilato, frontend due argomenti di
test omessi: corretti solo due zeri, retry senza avvisi; nessun filtro Wine
fallito. Helper artefatti elimina solo interrupted/moved. CLI di produzione e
ordinary parametri restano aperti; Linux/USA/Windows nativo/nuovo32:9 pendenti,
452lower separati. Nessun vecchio modello/source/ABI/scenario ripetuto, nuova
fixture/navigazione/misura.
[Runtime e prove](../rom_data_gameplay.md#managed-observed-task-parameters-and-typed-bus-consumption).


**ROMDATA-04-OBSERVED-TASK-PARAMETERS-ACTUAL-CLI completato EUR (2026-10-27).**
Build Windows produzione senza avvisi/c7f47bd4..; canonico hash/mtime ripristinati
esattamente, contenuto main invariato. Prima prepare interrotta dall'utente:
runner assente, manifest senza current/risultato; copia/log archiviati e stesso
binario riusato. Tre comandi accettati exit0: create/reuse/oldcache-oneframe,
67file generazione/69managed/66risorse/due span10B/nodeps/all67esatti, riuso senza
scritture. Primo frame1B soglia/1fase ma0correzioni: gate presenza esplicitamente
negativo, conservato. Una nuova runc12frame, motivata da quel negativo e dal
lead sorgente esistente, exit0 con20letture/28B/12soglie16correzioni/2fasi4righe.
145guardie e implementazione/originali/cache/canonico/Atlas utente invariati;
helper artefatti zero percorsi, copia interrotta conservata. CLI chiusa; ordinary
fullstate12frame da verificare. Nessun prepare/reuse/build/vecchio source-model-
runtime-scenario ripetuto, nuova fixture/navigazione o misure. Linux/USA/Windows
nativo/nuovo32:9 e452lower separati o pendenti.
[CLI e limiti](../rom_data_gameplay.md#actual-cli-for-observed-task-parameters).


**ROMDATA-05-OBSERVED-TASK-PARAMETERS-ORDINARY completato EUR (2026-10-27).**
Solo cinque righe opzionali al validatore esistente richiedono entrambi i gruppi
e tutte le sei righe. Primo build Windows senza avvisi e un solo filtro Wine:
12frame F09 guest8476→8488, ogni stato completo/clock/immagine centrale-larga/
OAM/dispatch/audio uguale. Cinque flussi con campioni reali
[26334,26334,3168,3168,26258];20letture/28B/12soglie16correzioni/2fasi4righe
esattamente uguali alla CLI.88123native/279744fallback solo copertura.76guardie,
originali/cache/dataset67file/canonico/codice/Atlas utente esatti. Delta cinque
righe verificato sul preimage; helper artefatti zero percorsi. Source/model/
managed/runtime/CLI/ordinary dichiarato chiusi. Primo frame correction-negative
e prepare interrotta restano chiusi; nessun vecchio source-modello-runtime-CLI-
collector-scenario ripetuto, nuova fixture/navigazione/misura. Altre scene,
indiretti/ABI/composedGUI/nuovo32:9 aperti;452lower e Linux/USA/Windows nativo
separati o pendenti.
[Equivalenza e limiti](../rom_data_gameplay.md#ordinary-twelve-frame-equivalence-for-observed-task-parameters).


**ROMDATA-01-BUS-HOST-LOWER-RESIDUE completato EUR (2026-10-27).**
452byte/113word/62intervalli classificati con114letture contestuali:404B da
102LDR Thumb PC-relative su101literal,48B da12voci switch osservate7–18.
Caricamento080A1146/MOVPC080A1148 conserva Thumb; prefisso originale normalizza
u16zero/>24a1 prima dell'indice, senza nuovo limite globale. Il nuovo SiteCapture
opzionale conserva schema/chiavi precedenti; scopePC/mode richiede verifica
opcode/indirizzo/larghezza, non attribuzione automatica per HLE/DMA annidati.
Un build Windows incrementale senza avvisi dopo errore helper solo nel test;
un filtro Wine con nuove sonde e un solo arm F09 corrente12frame osserva tutti
113word.85guardie/preimage cfg/codice/Atlas esatti; helper artefatti zero percorsi.
Nessuna vecchia accettazione ripetuta, fixture o misura. I452B sono supporto
codice, e73-d72/67file66risorse invariati. Nuove sorgenti394FBC/371064fase0/
371034 richiedono contratti propri; nessun formato/estensione dedotto dai literal.
Linux/USA/Windows nativo/nuovo32:9 da verificare, TODO globale aperto.
[Contratto e limiti](../rom_data_runtime.md#lower-bus-and-host-residue-source-contracts).


**ROMDATA-01-ADDITIONAL-CONSTRUCTOR-OBJ source/modello completato EUR (2026-10-27).**
394FBC è una tabella di quattro coppie condizionali soglia32bit/puntatorelista,
32B; lettori originali9DB56/9DBCA e costruttore già autenticato provano quattro
liste394F82/90/9E/AC,56B/otto oggetti. Totale88B, nessuna sovrapposizione owner.
Incremento/store/reloadu8 prima del confronto;255→0, fase nuova>3 disattiva.
Vecchia fase non limitata prima del lookup: modello0–3, nessun nuovo cap globale.
Modello isolato con JSON rigoroso/proiezione sorgente/verifica dipendenza liste.
Primo build Windows senza avvisi/unico filtro Wine passa13guardie,15prefissi
contatore, quattro prefissi puntatore-costruttore e quattro nuovi disegni con
oracolo copiato esistente.94guardie/codice esatto, hub solo dichiarazioni/helper
solo visibilità; helper artefatti zero percorsi. F09 conservata non legge questi
dati: nessun uso ordinario dichiarato. Produzione e73-d72/67file66risorse invariata;
estrazione/owner runtime/uso positivo da fare. Linux/USA/Windows nativo/nuovo32:9
pendenti, nessuna vecchia accettazione ripetuta/fixture/navigazione/misura.
[Contratto e modello](../rom_data_obj.md#conditional-constructor-obj-animation-source-and-model).


**ROMDATA-03-CONSTRUCTOR-ANIMATION-RUNTIME completato EUR (2026-10-27).**
e74-d73/68file67risorse pubblica tabella32B con dipendenza OBJ; catalogo80liste/
193oggetti1318B, tutte76liste precedenti/126ref dirette/65altri file-record esatti.
Owner tabella esatta con composizione precedente/raw, liste servite da owner OBJ
esistente. Primo Wine supera10guardie/2halfset/lifecycle/pubblicazione e13prefissi
originale-interprete-native stato-clock con88B grezzi alterati:36B typed native/
20soglie16puntatori/4fasi,4query esterne senza addebito,708casi bus e composizione.
Si ferma poi su errore setup test: load_state prima del ripristino ROM. Fallimento
conservato, non dichiarato PASS completo. Corretto solo l'ordine nel test; filtro
restante su dataset già validato passa6DMA/SWI e4pubblicazioni host/due oggetti
reali ciascuna, senza ripetere le parti superate. Produzione identica fra i filtri.
Build finali senza avvisi,169guardie/codice/Atlas esatti; helper artefatti0percorsi,
diagnostica fallita e offline68 conservati, solo exe mai eseguito sostituito ritirato.
CLI reale/uso ordinario positivo da fare; F09 conservata inattiva non basta.
Linux/USA/Windows nativo/nuovo32:9 pendenti; nessuna nuova fixture/misura/replay.
[Consumo e prove](../rom_data_obj.md#managed-constructor-obj-animation-and-typed-consumption).


**ROMDATA-04-CONSTRUCTOR-ANIMATION-CLI completato EUR (2026-10-27).**
Primo build produzione Windows senza avvisi/1e98de03..; canonico hash/mtime
ripristinati e contenuto main invariato. Tre CLI Wine exit0: prepare/riuso senza
scritture/runc0frame.68file generazione/70managed/67risorse, tutti68hash identici
al runtime accettato e tutti67ID/hash runtime esatti; vecchia cache caricata.
Hash/mtime riuso e startup invariati,145guardie/439sorgenti esatte; helper0percorsi.
Zero frame/dispatch/fallback/contatori nuovi: prova registrazione/attach reale,
nessun uso ordinario positivo dedotto. Nessun F09/source-model-runtime-scenario
ripetuto, nuova fixture/navigazione/misura. Uso ordinario positivo e Linux/USA/
Windows nativo/nuovo32:9 restano da verificare.
[CLI e limiti](../rom_data_obj.md#actual-cli-for-constructor-obj-animation).


**ROMDATA-05-CONSTRUCTOR-ANIMATION-ORDINARY-POSITIVE parziale EUR (2026-10-27).**
Osservatore passivo Wine-only implementato e guardia positiva opzionale nel
validatore (sette righe; loop precedente esatto senza guardia). Primo build
Windows senza avvisi; quattro osservazioni normali exit0 ma NEGATIVE: due
spostamenti160/200frame dal checkpoint EUR12006 copiato, selezione ritardata160,
conferma200 dal nuovo stato di selezione. Tabella0/fasi0/liste inutilizzate,
nessun fullpair o fixture positiva dichiarati.84guardie/codice/originali esatti;
provenienza normale precedente conservata senza replay3530frame. Nessuna
mutazione manuale RAM/task/fase/ROM. Setter flag9D054/call6 verificati come lead,
azione normale/semantica ancora da provare; niente altra rotta cieca o flag forzato.
Unità aperta, fixture/diagnostica conservate; sorgente indipendente371064fase0
separata. Linux/USA/Windows nativo/nuovo32:9 pendenti, nessuna misura.
[Prova ancora aperta](../rom_data_obj.md#ordinary-constructor-animation-discovery-remains-partial).


**ROMDATA-01..05-CONDITIONAL-PHASE-ZERO completato EUR (2026-10-27).**
371064fase0 ora tipizzata: soglie condizionali0/1/2=16/8/8, span3B più8correzioni,
11B nello stesso file/68file67risorse/e75-d74. Vecchie2righe+4correzioni/all66altri
record-file esatti. Nessun cap globale prima lookup o cambio ABI. Partizione
copertura3fasi/4correzioni disgiunta. Primo build Windows senza avvisi/unico Wine
passa4prefissi originali nuovi/7attach/6stati-clock nativi/3DMA e bordi raw.
Primo build produzione/canonico ripristinato, una CLI auto-prepare28frame passa
all68file/3fasi4correzioni/36letture44B/28soglie16correzioni. Corretto solo matcher
log offline dopo CLIexit0; comando non ripetuto. Stesso exe frontend: un nuovo
fullpair28frame8476→8504 passa ogni stato-clock/immagine/OAM/dispatch/audio con
campioni[61446,61446,7392,7392,61370], conteggi esatti CLI.215guardie/codice/loop
senza guardia5righe esatti; helper0percorsi. Nessun vecchio12frame/source/modello/
runtime/constructor ripetuto o nuova fixture/misura. Ordinaryconstructor rimane
parziale;371034e piattaforme Linux/USA/Windows nativo/nuovo32:9 ancora da verificare.
[Contratto e prove](../rom_data_gameplay.md#conditional-phase-zero-threshold-extension).


**ROMDATA-01-TASK-CYCLE-DELTAS / ROMDATA-02-TASK-CYCLE-MODEL completati EUR
(2026-10-27).** Proiezione condizionale371034–371064/48delta signed-i8,
disgiunta da tutte67risorse e75. Lettori18E10/18E2A usano OLD-u8 prima della
normalizzazione; incremento memorizzato come u8 prima helper1464EC/resto48,255→0. Somma/
sottrazione signed su halfwordtask+8 con storelow16; asse/unità/lifetime non
inferiti. Nessun cap introdotto: query48..255None,48/49/50 sono vecchio owner.
Primo build Windows senza avvisi/unico filtro Wine passa23guardie/58prefissi
originali/8bordi helper/tutte48righe e guardie RAM-callee,77input-frozen esatti.
Modello lossless/JSON rigoroso; revisione e75-d74/68file67risorse invariata.
Gestione/consumo runtime/CLI/ordinary48 completati nelle unità sotto; il solo
test sorgente non dichiarava consumo o scena. Constructorordinary precedente ancora parziale; Linux/USA/
Windows nativo/nuovo32:9 pendenti, nessuna misura o nuova fixture ordinaria.
[Contratto e prove](../rom_data_gameplay.md#conditional-european-task-cycle-delta-source-and-model).


**ROMDATA-02/04-TASK-CYCLE-RUNTIME completato EUR/Wine (2026-10-27).**
Nuova risorsa gameplay.task-cycle-deltas/filetask-cycle-deltas.json/e76-d75:
69file68risorse,48B signed-i8/uno span, tutte67precedenti risorse/file esatte.
Owner tipizzato compone nuovi byte/prior/raw, niente cap o cambio ABI; USA8-7.
Primo build Windows senza avvisi/primo filtro Wine passa9rifiuti attach
transazionali/2insieme completo/interruzione nuovo file,8prefissi originali-
interprete-nativi con stato completo/clock uguali e48rawB alterati,5letture
esplicitamente compilate. Tutte48righe, confine read32con2nuovi+2vecchi byte,
bordi/mirror,3trasferimenti DMA/SWI tra owner, file indisponibili/Load/Undo con
owner conservato.154guardie/codice esatti; helper ritira solo copia interrotta.
Nessuna ripetizione filtro sorgente o vecchie accettazioni. CLI produzione completata sotto;
ordinary positiva48 dei delta completata sotto; constructorordinary parziale.
Linux/USA/Windows nativo/nuovo32:9 pendenti, nessuna misura.
[Consumo e prove](../rom_data_gameplay.md#managed-task-cycle-deltas-and-typed-consumption).


**ROMDATA-05-TASK-CYCLE-CLI completato EUR/Wine (2026-10-27).**
Primo build produzione Windows senza avvisi/canonico ripristinato conhashmtime
esatti; tre comandi Wine prepare/riuso/runc0exit0. Tutti69file uguali al dataset
Rust accettato, tutti68ID/hash runtime uguali al manifest,48righe delta registrate,
cache nativa esistente caricata. Tutto pathset71file/hashmtime invariato su riuso
ed avvio;226guardie/codice esatti, helper0percorsi. Zero frame/dispatch/fallback/
letture delta: solo attacco; positività ordinaria e fullpair48 completati sotto. Niente
rotta cieca/manualflag. Linux/USA/Windows nativo/nuovo32:9 pendenti, nessuna misura.
[CLI e limiti](../rom_data_gameplay.md#actual-cli-for-task-cycle-deltas).


**ROMDATA-05-TASK-CYCLE-ORDINARY completato EUR/Wine (2026-10-27).**
Scena Bank48neutra3660→3708 scelta da sorgente e snapshotnormale: initializer
18D6C/default03, generic015E48/03-07 e candidato Bankroot+D40flags03phase21,
indirizzo coerente col caller5C1F8. Setter18E5C conserva vecchi04/08 con nuovo01:
argomento03non basta a provare il modo.11/3/89BL grezzi sono candidati, niente
callgraph/liveness dedotti dalla sola RAM. Una CLI esistente senza rebuild usa
48letture48B/all48fasi, dataset71path/hashmtime invariato. Primo build frontend
senza avvisi per5righe di guardia positiva; vecchio loop byteesatto senza guardia.
Un solo fullpair Wine passa ogni stato completo/clock/immagine/OAM/dispatch/
cinque flussi audio, campioni[105336,105336,12672,12672,105260], conteggi48/48/48
esatti CLI.97guardie/codice/preimage esatti, helper0percorsi. Nessuna navigazione/
manomissione RAM-task-ROM/cartridge o vecchia accettazione Bank12/F09 ripetuta.
Child dichiarati01/02/04/CLI05/ordinary05 ciclo chiusi, migrazione totale ancora
aperta; constructorordinary distinto parziale. Linux/USA/Windows nativo/nuovo32:9
pendenti, niente misure.
[Scena e prove](../rom_data_gameplay.md#ordinary-european-task-cycle-scene).


**ROMDATA-01-BANK-BUS-HOST-RESIDUE: censimento EUR/Wine completato, sorgenti90B
aperte (2026-10-27).** Modalità opzionale SiteCapture::start_all conserva filtri
vuoti/singoli e vecchie chiavi. Primo build feature Windows senza avvisi/unico
Wine: probe nuovi e un braccio Bank12decodificato3660–3672 con input normale
esistente, niente vecchio fullpair ripetuto.7430letture/7532site,497con contesto
interprete/7035None;7083B immagine,1434B fuori68owner. Tutti3683B del vecchio
NativeAPI inclusi;992B già classificati,80literal/320B e8celle dispatch/32B
verificati ora:1344B supporto codice, niente cap127globale da branchconhelper.
Restano90B dati:18programma/12directory/60listeOBJ, root4165A0+4114FC via5E6F4/8,
lettori2C5E0/2C60C con tick-fase-selettoreu16, distinti dai banchi u8già tipizzati.
Contratto e modello dei quattro prefissi autenticati completati sotto; gestione/
consumo restano aperti. Nessuna nuova risorsa/revisione dal solo censimento. Dodici delta tipizzati21–32 letti da
LDRB18E2A interprete spiegano il limite del vecchio NativeAPI.95guardie/codice/
preimage default esatti, helper0percorsi. e76-d75/USA8-7 invariati, Atlasutente
preservato; Linux/USA/Windows nativo/nuovo32:9 pendenti, niente misure/fixture.
[Inventario e nuove sorgenti](../rom_data_runtime.md#bank-bus-and-selected-host-rom-inventory).


**ROMDATA-01-U16-TASK-OBJ-BANK / ROMDATA-02-U16-TASK-OBJ-MODEL completati nel
prefisso EUR dichiarato (2026-10-27).** Quattro produttori2C5E0 autenticano root
41634C/411478,41647C/4114AC,4165A0/4114FC,4166C4/41154C:50liste140OBJ/7seq49frame/
65span1378B disgiunti da68owner. Formato relative-index/payload verificato,
padding escluso; sette liste non referenziate e tre seqvuote restano dati,
non prova di raggiungibilità/hold inventati. Contatore-fase-selettoreu16,
repeatlowu8, ritorno listavecchia prima incremento,255→256/65535→0, clamp/repeat/
done latch; nessun cap prima lookup o confine globale archivio. Modello lossless
JSON rigoroso riusa campi puri, non tempistica u8. Primo build core senza avvisi:
Wine passa23guardie/proiezione1378B poi fallisce aspettativa test sul r0di ritorno
ctor. Correzione solo test; secondo build core senza avvisi/filtro solo-rimanente
passa23ctor/93update/6producer, RAM-callee/alias/bordi. Primo filtro non dichiarato
PASS e23guardie non ripetute.88guardie/199sorgenti core esatte; helper0percorsi.
Coredevserde_json preserva modifiche frontend parallele senza compilarle. Revisione
e76-d75/USA8-7 invariata: gestione/consumo/CLI/ordinary di questi banchi ancora
aperti; Linux/USA/Windows nativo/nuovo32:9 pendenti, niente misure/fixture.
[Contratto e modello](../rom_data_obj.md#european-obj-banks-with-u16-task-counters).


**ROMDATA-02/04-U16-TASK-OBJ-RUNTIME completato in fasi EUR/Wine (2026-10-27).**
e77-d76 aggiunge graphics.word-task-obj-animations/fileword-task-obj-animations:
70file69risorse/all68record-file precedenti identici,65span1378B/4banchi/50liste/
140OBJ/7seq49frame. Binding rigoroso/aggancio transazionale; proiezione pura
condivisa mantiene corpo e gate del vecchio owner, nuovo gate411/416 e lookuphost.
Indicizzazione/contatori/ritorno listavecchia guest e ABI/stati/cache invariati.
Primo build Windows senzaavvisi; Wine passa pubblicazione/interruzione-file/
9guardattach/2fullset,27prefissi originali7ctor20update/202B compilati/fullstate-
clock,1378B/6classi/217mixedread/66B rawgap/vecchioowner/Load-slotUndo senzafile/
DMA3. Poi processo senza risultato terminale: non dichiarato fullPASS, ricevute
conservate, nessuna fase passata ripetuta. Build rimanente compilato pulito ma
rifiutato dal guardiasorgenti per2modifiche DBG parallele; snapshot stabile.
Drafttest host isolato perframe; errorecompilazione test-only campo privato e
origine OAM/register corretti, prodotto invariato. Buildfinale senzaavvisi/
unico Wine solo-host passa50liste/140OBJ/95letture host con65spanROM alterati,
OAM accettato perlista e descrittori raw/tipizzati uguali.165guardie/496sorgenti
snapshot/input ROM correnti esatti; helperritira sola copia interrupted prevista,
conserva dataset/log/preimage/eseguibili. CLIreale successivamente verificata
sotto; ordinary successivamente verificata nella finestra Bank sotto,
nessuna vecchia accettazione modello/censimento/scena ripetuta. USAe8-d7/Linux/
Windows nativo/nuovo32:9 pendenti; niente misure o nuove fixture/navigazione.
[Contratto runtime e prove](../rom_data_obj.md#managed-runtime-for-the-u16-task-obj-banks).


**ROMDATA-05-U16-TASK-OBJ-CLI completato EUR/Wine (2026-10-27).**
Un buildproduction Windows senzaavvisi da copia snapshot runtime accettata;
candidatoCLI b3bfe6f425923ce461793771fe6c84b86eb5da7dda69b3b9c7e0f5208eab51ea,
eseguibilecanonico hash/mtime ripristinato. Un unico scriptWine/3comandi reali
preparecreated/reused/runc0 passa:70filegenerazione identici al runtime/69risorse
registrate conhash esatti/72path managed. Riuso e avvio senza scritture hashmtime;
cacheoriginale caricata,0frameguest/dispatch/fallback e9campi nuovoowner tutti0.
154guardie/496sorgenti copiate/inputROMcorrenti esatti/helper0percorsi. Dataset
27463ce4f7ffccc3d0b9c8ab5ee78160a3e1c844ed80f68768f9adea87cb7757.
ChildCLI chiuso, ordinary positiva separata verificata sotto; nessuna accettazione
vecchia o fixture/navigazione ripetuta. Linux/USA/Windows nativo/nuovo32:9 pendenti,
niente misure/Android/package/commit/push. Modifiche DBG parallele preservate.
[CLI reale e prove](../rom_data_obj.md#actual-cli-for-the-u16-task-obj-banks).


**ROMDATA-05-U16-TASK-OBJ-ORDINARY completato EUR/Wine (2026-10-27).**
FinestraBank3660→3672 scelta dal censimento già accettato/root4165A0+4114FC,
copie byteidentiche di snapshot/inputnormali, nessuna navigazione/nuovocheckpoint/
manomissione RAM-task-ROM/cartridge. Una CLIe77 esistente passa uso positivo:
292letture/346B=[44index20count240attr2seqheader20frame-list20duration]/20host,
72pathmanaged/hashmtime invariati. Guardiaopzionale5righe sulnuovoowner nel test
fullpair; vecchio validator/loop byteesatto rimuovendola. Primo buildfrontend
Windows snapshot senzaavvisi/unicoWinefullpair primaPASS: tutti12stati completi/
clock/immagini central-wide/OAM/dispatch e5flussi audio identici,9contatori esatti
CLI. Dispatch[19962,85770], campioni[26334,26334,3168,3168,26258].94guardie/
496sorgenti snapshot/inputROMcorrenti esatti/helper0percorsi. Nessuna precedente
accettazione modello/runtime/scena/censimento ripetuta o buildproduction nuovo.
Child dichiarati sorgente/modello/gestione-consumo/CLI/ordinary diquestafamiglia
chiusi; nessunaprova all4banks/50lists/7liste nonreferenziate/seqvuote update/
indiceindiretto/fullarchivio/fullgioco. Constructorordinary distinto parziale;
wholeTODO e Linux/USA/Windows nativo/nuovo32:9 aperti, niente misure/Android/
package/commit/push, DBG parallelo preservato.
[Scena e prove](../rom_data_obj.md#ordinary-european-u16-task-obj-bank-scene).


**ROMDATA-01/05-CONSTRUCTOR-FLAG-REACHABILITY chiuso per il flag EUR/Wine
(2026-10-28); animazione ordinary ancora parziale.** Ventuno finestre sorgente
legano95EA0/wrapper9A594/global0200F4A8/statoDC/root960F0: Move27/A dopo Bcancel/
validazione9D6C8/destinazione diversa→count1A4/28-29, count signed>1→enable99F18.
Altri3A→99300 eA5544case6→A57BE/case1→A570A sono percorsi distinti, nonliveness
globale. Due snapshotnormali autenticati letti soltanto per scegliere A-only,
senza vecchioRight: unWineobserver160 riusato chiude quinta prova negativa,
table0/phases0/listeinutilizzate; nessunfullpair/fixturepositiva o buildnuovo.
Endpointstato25/count3 non implica destinazioneinvalidata. Nuovocriterio mancante
flag/slot duranteilpassaggio: unaCLIsolalettura64frame giàesistente cattura448
fileRAM-immagine, normaleMove e flag20attivo12186–12230/45postframe; entrambi
slot394/3A4 sempreinattivi/counter-phase0/table0. Flagordinaryraggiungibile provato,
creazioneslot/callback ancora da autenticare; niente attribuzionePC/intraframe.
587guardie/496sorgenti frozen/21hashfinestre esatti, originali/e77dataset72path
invariati/helper0percorsi. Nessuna vecchia accettazione ripetuta o RAM/ROM/cart
manomessa; staticodisassembler non è testgameLinux. Nuovafullpair solo dopo
usopositivo4fasi/4listeguest-host; nientealtri tentativiMoveallacieca. Linux/USA/
Windowsnativo/nuovo32:9/fullTODO aperti, niente misure/Android/package/commit/push.
[Flag e slot](../rom_data_obj.md#ordinary-constructor-flag-activation-and-inactive-slots).


**ROMDATA-01/05-CONSTRUCTOR-SLOT-ACTIVATION chiuso per gate sorgente ed esclusione
mappa (2026-10-28); ordinary ancora parziale.** Otto finestre nuove autenticano
9DB0C→owner+10/pair[owner,9B11D]→9E300/145C10→9B11C(actor,owner); stessi slot394/
3A4del counter e9DBA4.9DAD4 è helper precedente distinto. Oltre movimento eflag20,
ramonormale9B460 richiedeunitFD2, XYsigned0..15, cellavalida/flags02/altezzaS8>
altezzaattoreS16>>4, anim36!=40; bit8000entrata-uscita, mask10000skip, slotlibero.
Rootcellelive02007F10,width+8/base+4,stride2altezza-flags, non nuovoassetROM.
Solo lettura dei64RAMdump esistenti: FD2/flagsattore0/anim0, trecellepercorse
height2/flags0; tutte256celle dichiarate senza02,512Bhash stabile. Questa mappa
immutata nonoffre destinazioni per ilgate; nientealtroMoveallacieca o vecchie
cinquenegative/flagcapture ripetute.87guardie/8hashfinestre esatti/helper0percorsi,
nessun nuovo gameplay/build/testWine-Linux/capture/schema/runtime/ABI/cache/save.
Prossimocriterio: inventariosolalettura snapshotEURnormali esistenti, mappaeattore
idonei eprovenienza; niente stati sintetici o ricreazione copie ritirate. Fullpair
solo dopo uso positivo4fasi/4listeguest-host; ordinary/Linux/USA/Windowsnativo/
nuovo32:9/fullTODO aperti, niente misure/Android/package/commit/push.
[Gate cella e slot](../rom_data_obj.md#constructor-slot-creation-depends-on-live-cell-flags-and-height).


**ROMDATA-05-CONSTRUCTOR-ELIGIBLE-MAP-INVENTORY chiuso negativo (2026-10-28).**
Nuovocriterio solalettura81snapshotEURnormali/provenienza giàverificata: tutti
hash/payload/ROM/domìni16x16validi, soltanto2proiezionimappa512B(1F09/80river),
zero celle02/candidati. Vecchioscan606sequenzealternatenonripetuto.88guardie
snapshot-report-parser-contract-output esatte; nessunbuild/gameplay/testWine-
Linux/capture/checkpoint/manomissione. Boundnegativoterminale su2mappeconservate,
nonallmap/futurescript; nonripetere81statiuguali oMovequi. Parentordinaryancora
parziale; prossimaleadsorgente produttoreflagcelle02007F10/terrainROM prima di
scegliereunamappaidonea. Fullpairsolo dopousopositivo4fasi/4listeguest-host.
Linux/USA/Windowsnativo/nuovo32:9/fullTODO aperti, niente misure/deployment.
[Mappe conservate](../rom_data_obj.md#retained-normal-maps-have-no-eligible-constructor-animation-cells).


**ROMDATA-01-CELL-FLAG-WRITERS chiuso per census/seed candidati EUR (2026-10-28).**
VecchioTERRAIN-MAP-SOURCEdel24giàchiuso/352pacchettiBGriusato, nonnuovoarchivio.
Nuovocensus diretto1DE98nessunBLThumb/puntatoreallineato (nondeadcode/indiretto),
unicoBL1DD68a1D798legge flags02. Nuovaquerysemantica datasetBGautenticato e77:
113auxprogrammi/162link/zerofill-parent-child/orderedu16writes,25programmi e44ID
inizializzano02; flags0/1/2/3/4/8/9. Nessun decoder/risorsa/revisione o accettazione
codec/loader vecchia ripetuta.24guardie/4finestresorgente esatte/helper0percorsi,
nessunbuild/gameplay/testWine-Linux/capture/checkpoint. Quote typo privato corretto
prima esecuzione; cleanupanticipato rifiutaunitasenzarisultato, poipassa dopo
summary, prodottoimmutato. Candidatiasset nonprovano battaglianormale/luogonome;
prossimoactualmapselectionscript/caller→IDidoneo/normalreachablebattle.81snapshot/
5Movebound/flagcapture restanochiusi. Ordinary4fasi/4listeguest-host/fullpair e
Linux/USA/Windowsnativo/nuovo32:9/fullTODO aperti, niente misure/deployment.
[Seed e sorgenti](../rom_data_maps.md#constructor-animation-cell-flag-seeds).


**ROMDATA-01-CONSTRUCTOR-ELIGIBLE-MAP-SELECTION: collegamento sorgente chiuso,
battaglia raggiungibile ancora parziale (2026-10-28).**253C8state10/2549Clegge
CCF00(7)→02002037lowu8→1B270/1B689loader/state11;2540Cètabelladispatch. Opcode12
advance2/callback126831: byte1nonzero→CCF18variable7; zero→70C8CC+40*U16(02002170)
+2→variable7, poimette0200F390=10.51candidatiCCF18, duearmiliteral7autenticati,
nessunallwriterclaim. Queryscriptstyped204istruzioni56default/148literal:
19literalidonee/10argmappa[31,33,34,73,75,89,91,109,115,129]; riferimenti esatti
privati, nonnomiluogo/ordinaryreachability oeventkey=activeindex. Dueesempinormali
retainedactive35/197 maparg99/85→descriptor98/84/defaultcoerenti/inidonei, non81
scanripetuto.37guardie9finestre esatte, nessunbuild/gameplay/testWine-Linux/
capture/checkpoint/runtime/schema/ABI/cache/save. Prossimoactiveindexproducer/
40Brecord70C8CC/mapbyte2→normalquest/event, nienteIDforzato/navigazioneguess.
Positive4fasi/4listeguest-host/fullpair ewholeTODO/Linux/USA/Windowsnativo/32:9
aperti, niente misure/deployment.
[Selezione script e limiti](../rom_data_maps.md#script-map-selection-and-remaining-battle-reachability).


**ROMDATA-01-ACTIVE-ENCOUNTER-RECORDS: produttore e join condizionale chiusi EUR
(2026-10-28), catalogo completo e battaglia ordinary ancora parziali.** Setter9CF0
scrive low16r0/r1 a02002170/172; due BL diretti9D2C/1273A8, nessun puntatore Thumb
allineato, senza pretesa di tutti gli scrittori.9D04 sceglie primary/alternate da
row+2 oppure71D1EC+12*low8(arg1), secondo row+4.63944 collega selettore live219E,
record70B713B3C+45 e row12B71C760;127388 cerca event key su righe255..1, prima
corrispondenza decrescente, duplicati distinti. Query nuova sul dominio u8 del
chiamante:208 righe direct-primary,54 con seed02,44 anche con script default-map.
Non prova disponibilità missioni, nomi o extent del catalogo40B; maxreferenza442
e costante separata300 non sono limiti. Pub normale conservato selettore0/row0/
maparg11 inidoneo, sola lettura.42guardie/23finestre esatte; import Python privato
corretto dopo stop prima di output; originali invariati. Nessun build/gameplay/
testWine-Linux/capture/checkpoint/nuova risorsa. Prossimo criterio: missione/evento
idoneo realmente disponibile da checkpoint normale e UI originale. Niente ID/RAM
forzato o altri Move ciechi;4fasi/4liste guest-host e fullpair ordinary restano
aperti. Vecchie accettazioni non ripetute; Linux/USA/Windows nativo/nuovo32:9/full
TODO pendenti, niente misure o deployment.
[Produttore e prove](../rom_data_maps.md#active-index-producer-and-event-record-links).


**ROMDATA-05-ELIGIBLE-QUEST-LOCATOR chiuso EUR/Wine (2026-10-28): missione idonea
presente nel menu normale.** Osservatore309ba31c/e77 riusato, nessun build: un
percorso noto620frame conserva endpoint menu mancante14969; poi300frame4Down+A
aprono dettaglio White Flowers15269. Non ripetuta come accettazione la vecchia
scena620; nuovo criterio checkpoint/selettori. Originali e copie esatte, nessun
RAM/task/ROM/cart edit. Live F44C02024D5C/count+1B14/puntatori+20+4*ordinal, low10
missionID da62126..5E. Query nuova14entry trova2candidati direct-primary/default-
script/cell02:58/72. Quinta riga e dettaglio visibile confermano58; F4480202389C
+1450→02002268/ID58 con60208..28. Evento58/primary315/maparg69/descriptor68 idoneo;
Giza Plains nella descrizione, ma accettazione e battaglia non ancora raggiunte.
130guardie/496sorgenti frozen/23finestre esatte/helper0percorsi. Layout candidato
031C/03D03001028 scartato perché non legato al menu live. Entrambi workerWine
passano, contatori constructor0 nel pub attesi, non prova positiva/fullpair.
Prossimo conferma normale e verifica accettazione/ingresso battaglia, poi attore/
celle prima di Move guidato.4fasi/4liste guest-host/fullpair, catalogo40Bcompleto,
Linux/USA/Windows nativo/nuovo32:9/fullTODO pendenti; vecchie accettazioni chiuse,
niente misure/deployment.
[Missione normale e prove](../rom_data_maps.md#normal-pub-menu-identifies-an-eligible-quest).


**ROMDATA-05-WHITE-FLOWERS-BATTLE-NAVIGATION parziale EUR/Wine (2026-10-28);
accettazione normale chiusa.** Copia esatta dettaglio ID58→loadoutOK→costo/
obiettivo→condizioniYes; pagamento300gil/fondi7709→7409 e successivo Fee:Paid
provano missione attiva. Voce inizialmente letta QuitPub è QuitMission: aperto
solo dettaglio A:Cancel, nessuna conferma annullamento; B/B/Down/A su Leave porta
alla mappa.27capture normali/4840frame, osservatore309ba31c/e77 già esistenti,
nessun build/manualRAM-task-ROM/cart write. Tutti workerWine passano, table0,
non27accettazioni equivalenza/nessun fullpair. Label mondo Cyril/LutiaPass/
Nubswood/Sprohm, patch centrale esclusa; camera scroll/overshoot documentati.
Non provati Giza Plains raggiungibile/assente/non piazzata.337guardie/496sorgenti/
17hash finestre esplorative esatti.92524init/staletask/031C03001028 non provano
worlddispatch; catalogo165E8indice725Giza e base718/lista3EE60 sono solo lead,
non coordinate. Unità parziale conserva tutti checkpoint diagnostici, niente
ritiro fixture. Prossimo source selector/località piazzate da stati conservati,
nessun altro cursore cieco o27bound/5Move/81map/vecchie accettazioni ripetute.
Battaglia/attore/4fasi4liste guest-host/fullpair e fullTODO/Linux/USA/Windows
nativo/nuovo32:9 restano aperti, niente misure o deployment.
[Accettazione e limiti](../rom_data_maps.md#normal-white-flowers-acceptance-and-open-battle-entry).


**ROMDATA-01-WORLD-LOCATION-SELECTOR chiuso EUR/Wine (2026-10-28).** Corrette
ipotesi ispezione:254B2→6B40, count taskbyte+2 nonword+4; letterale2810/0300
è03002810, non03001028. Test opzionale passivo: un buildWindows senzaavvisi da
snapshot497sorgenti, un frameWine neutro18829→18830/2232read2276site323scoped,
RAM effettiva ugualeparser. Parent[03002810]=0200F3A4; nodes[parent+67C]=08395CA8.
375F8loop30/stride20, byte0/1*8+8centro, hitbox e snap1pixel condizionali;
37D6A..70 cursorview+6/+8,37A44displayY=worldY−scrollC.385CC..38668 savebase+B0+
12*i enabledbyte1bit0/byte3selectoronebased→node(selector−1). Giza index7/catalog
725/node20/source395E24 è piazzata a112,296. Solo lettura dueendpointmeridionali:
21cursor128,287,22cursor128,330; vecchi27bound nonripetuti.152guardie/497sorgenti/
32finestreesatte/helper0percorsi. NuovafamigliaROM30node20B: solo2coordinateB
classificati,18B/schema/modello/estrazione/runtime aperti. Prossimo Leftbreve da
21, verificarelabelGizaprimaA; nientecoordinate/RAM/mapIDforzati. Battaglia e
4fasi4liste guest-host/fullpair/fullTODO/Linux/USA/Windows nativo/nuovo32:9 aperti,
niente misure/deployment/vecchie accettazioni ripetute.
[Selettore e prove](../rom_data_maps.md#world-selector-and-placed-region-coordinates).


**ROMDATA-05-GIZA-BATTLE-ENTRY chiuso EUR/Wine (2026-10-28).** ShortLeft guidato
porta alla label Giza Plains; A viaggio/menu White Flowers/A schieramento.
Primary315/map69/descriptor68, griglia live512B e8427b5e.. uguale seed packet56/
20celle02. Marsh schierato con posizione/direzione confermate; START To Battle/
Yes/dialogo porta a Move. AttoreFD2(6,5), Left/Down/Down cursore effettivo(5,7),
A normale attiva4fasi/4liste guest-host:75frame/22read88B=44threshold44pointer.
Checkpoint22713+suffix11 conservati, non ancora fullpair.16captureWine/3875frame,
309ba31c/e77 esistenti/no build;221guardie496sorgenti esatti/helper0percorsi.
Solo tasti su copie, nessuna modifica manuale guestRAM/task/ROM/cart save.
Primi START prima conferma facing inerti/B torna a placement: corretti e chiusi,
non ripetere. Prossimo singolo CLI positivo11 e confronto completo stato-clock-
immagini-OAM-dispatch-audio dal suffix, senza ripetere scoperta/navigazione.
Cataloghi40B/worldnodes18B/fullTODO/Linux/USA/Windows nativo/nuovo32:9 pendenti;
niente misure/deployment/vecchie accettazioni ripetute.
[Ingresso e prova positiva](../rom_data_maps.md#normal-giza-battle-entry-and-positive-constructor-animation).


**ROMDATA-05-CONSTRUCTOR-ANIMATION-ORDINARY-POSITIVE completato EUR/Wine
(2026-10-28).** Copia esatta nuovo checkpoint normale Giza22713/suffix11→22724.
Una CLI produzione esistente b3bfe6/e77 riusa dataset/cache e passa22read88B/
44threshold44pointer/tutte4fasi. Un fullpair Wine309ba31c passa ogni11stati-clock/
immagini classiche-centrali-estese/OAM/dispatch/cinqueaudio e tutte4liste guest-
host; contatori identiciCLI, dispatch57833/223470,audio24138/24138/2904/2904/23966.
91guardie496sorgenti/72pathmanaged esatti/helper0; niente build/preparazione/
navigazione/discovery/vecchie accettazioni ripetute, guestedit/cartwrite/misure.
Caso ordinario constructor chiuso, non copertura tutti attori/mappe/fullgame.
Linux/USA/Windows nativo/nuovo32:9/fullTODO restano pendenti; cataloghi40B e nuovi
worldnodes18B da completare. Preservato lavoro u16/DBG, niente deployment.
[Prova ordinaria completa](../rom_data_obj.md#ordinary-constructor-animation-positive-giza-scene).


**ROMDATA-01-WORLD-NODE-RECORDS proiezione sorgente EUR chiusa (2026-10-28).**
395CA8..395F00/30record20B/600B, count dal loop375F8già provato; nessun cap
runtime30 dedotto (getterlowu8/workarray32). Producer31488/32BE6scrivono root
[parent03002810+67C]. Byte0/1 coordinate; quattro slot2/6/10/14 pathID/neighbor/
opaque2B più tailopaque2B:300semanticB/300opaqueB conservati.120slot/90linkattivi/
30path0/45path ciascuno in2direzioni. Nodo0valido distinto da path0;4DAD0match
neighbor prima delpath (22target0→0),46EB0stessonodo oppurepath!=0/neighbor.
4DB80graphcopia4slot/salta path0/gate8-27; riferimento397018+(path−1)*8 esterno,
roadcatalog/payload aperti.25finestre(21nuove4riusate)/44pin/42guardie passano,
600B disgiunti da49746span69owner. Nuovo criterio sulla traccia esistente:
35read42site solo coordinate, niente nuova copertura edge o audit ripetuto.
Nessun build/game/test/fixture/guestedit/misura. Prossimo modello tipizzato
lossless/sourceplan e un oracoloWine mirato; estrazione/runtime/CLI/ordinary,
Linux/USA/Windows nativo/nuovo32:9/fullTODO pendenti.
[Schema e lettori](../rom_data_maps.md#world-node-record-source-projection).


**ROMDATA-02-WORLD-NODE-MODEL completato EUR/Wine (2026-10-28).** Catalogo
world_nodes autentica immagine esatta/6finestre/hash600B. Modello30nodi ordinati,
4Link path/neighbor/opaque2B +tailopaque2B, sourceplan1span600B lossless; JSON
stretto/shape distinto da autenticità, niente cap45/29/opaquezero dedotto.
Un buildWindows core-only senzaavvisi/6c2ef15a.., un filtroWine passa23guardie/
566chiamate originali con ritorni/indirizzi letture indipendenti,300semanticB
osservati/300opaqueB preservati nonletti/16aliasbyte/24fuoriproiezione. Callee/
SP/RAM fuori stack/VRAM/OAM/palette verificati, non native/fullstate-clock/BFS/
ordinary.24guardie215sorgenti congelate/helper0 esatti. Nessun gameplay/frontend
build/vecchi oracoli ripetuti; e77-d76 e schema1 invariati. Prossimo pubblicazione
managed/consumo guest tipizzato/CLI reale/scena mondo conservata. Linux/USA/
Windows nativo/nuovo32:9/fullTODO pendenti, niente misure/deployment.
[Modello e prova](../rom_data_maps.md#typed-world-nodes-and-original-reader-proof).


**ROMDATA-03/04-WORLD-NODE-RUNTIME completato EUR/Wine (2026-10-28).** e78-d77
pubblica world-nodes.json/gameplay.world-nodes/index69:71file70risorse, tutti69
record/file precedenti esatti, USAe8-d7 invariato. Binding600B/7campi statistica,
coordinate/path/neighbor/opaque letti dal modello, composizione prior/raw/bordi
senza cap su input guest o sostituzione di piazzamento/pathfinder/ABI/save/cache.
Un filtroWine passa12attach/2fullset/pubblicazione interrotta,15prefissi nativi
fullstate-clock/32Bcompilati124istruzioni/600Braw alterati dopoLoad, alias/mirror/
fuori30-31/equal255/rootRAM. Bus tutti600B/4classi30nodi/width/bordi/priorowner,
Load/Undo/file indisponibili e DMA/CpuSet/CpuFastSet fullstate-clock3 passano.
Primi2build rifiutati da cachecorevecchia: dep-info punta a vecchia copia, touch
nuova copia inefficace; spostato solo fingerprint specifico. Terzo build senza
avvisi/fresh=false/2cf1d882.. da502sorgenti stabili.200guardie/502sorgenti+215
preimmagini modello esatti; helper ritira solo copia interrupted, dataset/eseguibile
accettati conservati. CLI reale/scena ordinaria ancora da fare. Nessun nuovo
gameplay/vecchio modello o proof ripetuto/misura; Linux/USA/Windows nativo/
nuovo32:9/road397018/fullTODO pendenti. Lavoro parallelo DBG/u16 preservato.
[Consumo e prove](../rom_data_maps.md#managed-world-nodes-and-typed-guest-reads).


**ROMDATA-04-WORLD-NODE-CLI completato EUR/Wine (2026-10-28); ordinary parziale.**
Buildproduzione Windows senzaavvisi/e0491cdc.. da502sorgenti stabili; canonico
hash/mtime ripristinati. TreCLIWine exit0 preparecreated/reused/runc normale16,
71file identici runtime/70IDhash esatti/73path managed invariati riuso-avvio,
cache28859block caricata. Copie normali Giza01/02input,19049→19065:402read402B
coordinate/22nodi; path/neighbor/opaque0, nessun fullpair. Matcher privato voleva
frame relativo16, CLI mostra assoluto19065: corretto offline dopo tutti3exit0/
guardie, nessuna ripetizione. Bound16 senza path/neighbor chiuso, prossimo
sourcecallers/stato normale motivato prima di nuova osservazione.169guardie/
502sorgenti esatti/helper0. Nessun nuovo fixture/navigazione/proof precedente/
misura. Ordinary3classi/fullpair/road397018/fullTODO/Linux/USA/Windows nativo/
nuovo32:9 pendenti; lavoro DBG/u16 preservato.
[CLI e limiti ordinari](../rom_data_maps.md#actual-world-node-cli-and-ordinary-coordinate-use).


**ROMDATA-01-WORLD-NODE-ORDINARY-CALLERS e ROMDATA-05-WORLD-NODE-TRAVEL-ORDINARY
completati EUR/Wine (2026-10-28).** Nuovoquery directBL/10finestre27guardie lega
3B012→4DB1C→4DB80 a view1A(dest−1) diverso da parent7BB; output7C1.47034progress!=FF
→4DAD0; entity parent7B8+52*i. Duecheckpoint normali esistenti17/selected20→19/
progressFF e entity1node0→1 motivano bound360, non deduzione PC/frame o tutti
caller. Una CLI360 positiva e un fullpair360 passano6841B=4820coordinate864path
1157neighbor/29nodi, tutti stati-clock/immagini classiche-centrali-estese/OAM/
dispatch/cinqueaudio e contatoriCLI identici. Opaque0 distinto da prove mirate.
Dispatch2006894/3620371,audio790020/790020/95040/95040/789942. Una buildWindows
senzaavvisi/dabb1ac6.. da502sorgenti con guardia opzionale5righe; vecchio validatore
esatto senza guardia, runtime/core invariati.103guardie/502sorgenti/27caller/
169CLI esatti/helper0. Primo16negativo path/neighbor resta chiuso; niente nuovi
fixture/navigazione/guestedit/vecchi proof ripetuti/misure. Casoordinary chiuso,
non tutti nodi/mappe/opaqueordinary/fullpathfinder. Linux/USA/Windows nativo/
nuovo32:9/road397018/fullTODO pendenti, lavoro DBG/u16 preservato.
[Prova ordinaria e gate](../rom_data_maps.md#ordinary-world-node-travel-and-caller-gates).


**ROMDATA-01-WORLD-ROAD-PATHS proiezione sorgente EUR completata (2026-10-28).**
45IDpath dai90link attivi/30nodi selezionano45header8B397018..397180:countu8,
opaque3B,puntatoreROMu32. Payload45distinti contigui395F00..397018/2188coppieXY
signedi8/4376B,46span4736B=4601semantic135opaque. Rootstoreparent680 riusato;
4DCcost byte0,4DD0Cflag/index/steplowu8/nocap45, fuori stepcount outputzero32
senza pointer/sampleload, reverseordine/nonnegazione, duei16packed/returnptr.
4DD70flagcurrent>target/4DD84minnode; calleraggiungeXYnode*8 agli offset,
46B64differenze max5sample/facing nonfullABI.3rootliteral/3pointcalls/1dir/1min
candidatequery, non assenza globaleindiretti.5endpoint1pixelshort4/6/8/10/12
preservati; zero pointassenti/count32..85 nonvincoli.6finestre60pin17guardie/
70owner49747span disgiunti/helper0. Nessun build/game/test/fixture/vecchi proof
ripetuti/misura. Prossimo modello lossless/sourceplan e oracoloWine mirato;
estrazione/runtime/CLI/ordinary/catalogoglobale/opaque meaning/Linux/USA/
Windows nativo/nuovo32:9/fullTODO pendenti. Lavoro DBG/u16 preservato.
[Contratto dei percorsi](../rom_data_maps.md#world-road-path-source-contract).


**ROMDATA-02-WORLD-ROAD-MODEL completato EUR/Wine (2026-10-28).** Modello
world_roads standalone: 45 header ordinati con countu8/opaque3/puntatore codificato,
2.188 coppie XY con segno e sourceplan condiviso 46 span/4.736 B. JSON stretto,
partizione coerente e autenticità distinta; preservati cinque endpoint corti,
zero campione possibile e count0/255 strutturali senza cap32..85 dedotto.
Un build Windows core-only senza avvisi/de9d1e11.. e un filtro Wine passano
29 guardie/2.537 chiamate originali: 2.188 punti forward, 135 reverse, 180 uscite,
6 aliasu8, 6 header esterni e 22 helper ordine. Output/ritorni/sequenze letture
esatti; tutti4.601 B semantici letti, 135 B opachi ricostruiti/non letti.
32 guardie conservazione/504 file build(218 core) esatti/helper0. Nessuna scena
ordinaria/fixture/build frontend/misura o prova nodo già chiusa ripetuta.
Schema1/e78-d77 invariati. Prossimo estrazione managed/letture guest tipizzate,
CLI reale e una scena ordinaria positiva. Linux/USA/Windows nativo/nuovo32:9,
estensione globale catalogo e fullTODO pendenti.
[Modello e prova](../rom_data_maps.md#typed-sampled-world-roads-and-original-reader-proof).


**ROMDATA-03-04-WORLD-ROAD-RUNTIME completato EUR/Wine (2026-10-28).**
e79-d78/index70/gameplay.world-roads/world-roads.json:72file71risorse, tutti70
record/file precedenti esatti, USA8-7/schema1 invariati. Owner immutabile count/
puntatore/sample/opaque da46span4736B; compone nodo mondo al bordo sinistro e
rawdestro/header45..255, nessun algoritmo movimento/facing/pathfinding sostituito.
Due build Windows senza avvisi/accettato52be2981.. da506file stabili; primo filtro
chiude publication/attach, poi fallisce nativecase3 per pagina396 omessa. Fix
pagegate e filtro rimanente native/bus/lifecycle senza ripubblicare:
12guardie attach/2fullset, 12prefix original/interpreter/native fullstate-clock,
67istruzioni compilate/44B espliciti, sorgenti4736B poison dopoLoad;
tutti4736B/4classi45percorsi/widths/mirrors/bordi/priorowner/LoadUndo e DMA3 PASS.
221guardie506sourcehash+504preimage modello esatti; helper ritira solo copia
interrotta. Nessuna scena/modello/nodi/fixture ripetuti, misura o ABI modificata.
CLI reale e una scena ordinaria positiva sono prossimi; Linux/USA/Windows nativo/
nuovo32:9/globalextent/fullTODO pendenti.
[Consumo e prove](../rom_data_maps.md#managed-world-roads-and-typed-guest-reads).


**ROMDATA-04-WORLD-ROAD-CLI completato EUR/Wine (2026-10-28).** Build produzione
Windows senzaavvisi/1a86d341.. da506sorgenti accettati, canonicalhashmtime ripristinati.
Tre comandi Wine preparecreated/reused/runc360:72file esatti alruntime/71IDhash/
74path invariati hashmtime, vecchi28859blocchi cache caricati.
Copie normali Giza01state/02input19049→19409: 2976letture/5067B=
885count/2788puntatore/1394sample/0opaque/40percorsi,
3classi semantiche positive. 180guardie506hash/helper0; nessuna nuova scena,
navigazione, misura o prova giàchiusa ripetuta. Prossimo unico fullpair ordinario
con conteggiCLI identici; non tutti45payload/mappe/opaque ordinari/globalextent.
Linux/USA/Windows nativo/nuovo32:9/fullTODO restano pendenti.
[CLI e consumo](../rom_data_maps.md#actual-sampled-road-cli-and-ordinary-consumption).


**ROMDATA-05-WORLD-ROAD-TRAVEL-ORDINARY completato EUR/Wine (2026-10-28).**
Un fullpair Wine360 normale Giza01/02 con datasetCLI e79-d78,19049→19409:
tutti360stati-clock/classic-central-wide/OAM/dispatch/cinqueaudio esatti.
Sette conteggi uguali CLI:2976reads5067B=885count2788pointer1394sample0opaque,
40percorsi(anyfield, non40payload). Guard opzionale5righe conserva vecchio
validator byteexact; un buildWindows senzaavvisi/4c453f07.. da506file stabili.
100guardie506hash +180guardieCLI esatte/helper0; nessuna nuova scena/fixture/
navigazione/misura o prova source/model/runtime/nodi giàchiusa ripetuta.
Caso ordinario road chiuso; tutti45payload/mappe/opaqueordinari/fullABI/globalextent,
Linux/USA/Windows nativo/nuovo32:9 e fullTODO ancora pendenti.
[Equivalenza ordinaria](../rom_data_maps.md#ordinary-sampled-road-travel-equivalence).


**ROMDATA-01-ENCOUNTER-CATALOG-READERS contratto EUR completato (2026-10-28).**
Catalogo fisico dichiarato442header40B1..442: origin70C8CC, first70C8F4 da due
root indipendenti, next710E04 famiglia28B indipendente.17680B, nonmax442assunto
come count/né costante300/né capguest.442linktemplate contigui2850record48B/
136800B corroborano partizione.17campi/33B semantici e7opachi perheader; indici
originali low16senza guardia442. Index0è prefisso coordinate: ultimi16B payload
header442, niente header valido inventato. Puntatoreheader29+4a70CD58giàowned
constructor-alternates: dependency/coerenza/skip4B,2nuovispan17676B,14582semantic/
3094opaque disgiunti71owner49793span. Template192Bgiàowned preservati; payload
unità/coordinate separati, nonmigrati.27loader da rootcached/40finestre(39new1reuse)/
54pin68guardie/helper0 PASS. Nessun build/game/test/fixture/proofprecedente ripetuto.
Prossimo modello lossless442header/sharedpointer+unoracoloWine; estrazione/runtime/
CLI/ordinary, allindirect/globalextent/Linux/USA/Windows nativo/nuovo32:9/fullTODO
pendenti, nessuna misura.
[Header e ownership](../rom_data_maps.md#european-encounter-headers-and-shared-ownership).


**ROMDATA-02-ENCOUNTER-HEADERS-MODEL completato EUR/Wine (2026-10-28).**
Modelloencounter_headers lossless442header1..442:17campi/opaque7,17680Bfull;
2span17676B escludono70CD58len4delownerconstructor-alternates. JSONstretto/shape
separato daautenticità/coerenzadependency; sharedptrmodificato lascia span
immutati maè rifiutato. lowu16alias/None0-443..65535/priorraw, nessun capcount/
ptrnull/enum/opaquezero dedotto. Un buildWindows core-only senzaavvisi5855af92..
e un filtroWine passano29guardie/7523probes:7072prefixsingolocaricamento e451
C0034completi(442+6alias+3fuori), tutti14586semanticB incl4shared osservati,
3094opaqueB ricostruiti/nonletti. Reg/flags/PCprefix, callee/SP/LRgate, RAM fuori
stack4/VRAM/OAM/palette verificati; nonfullABI/importer/nativeclock/ordinary.
33guardie508file(221core) esatti/helper0, e79-d78invariato, niente vecchioracoli/
scena/fixture/misure. Prossimo managed/typedbus/sourcepage70C..710/dependency,
poiCLI/ordinary; nestedpayload/allcaller/global/Linux/USA/Windows nativo/nuovo32:9/
fullTODO ancora pendenti.
[Modello e prova](../rom_data_maps.md#typed-encounter-headers-and-original-reader-proof).


**ROMDATA-03-04-ENCOUNTER-HEADERS-RUNTIME completato EUR/Wine (2026-10-28).**
e80-d79/index71/encounter-headers.json:73file72risorse, tutti71precedenti esatti,
dependencyconstructor-alternates e coerenzasharedpointer. Owner2span17676B
esclude4Bvecchi, sourcepage70C..710,8classi/442headermask/11stats; guestalgoritmi/
indici/ABI/cache/save invariati, USA8-7/schema1. Un buildWindows senzaavvisi
1c4135f5.. da510file stabili, un filtroWine:14guardieattach/3fullset/failedowner-
state,26prefix original/interpreter/native fullstate-clock con17680Brawpoison,
31istruzionicompilate/37Bnuovi+4Bvecchi. Tutti17676nuovi+4sharedB/8classi/442header/
widths/mirrors/bordi/priorowner/LoadUndo/DMA3 PASS. 206guardie510hash+508preimage
modello esatti/helperritira solo interrupted. Nessuna scena/fixture/misura/prova
modello7523/nodi-road-constructor giàchiusa ripetuta. Prossimo CLIreale/ordinary;
nestedpayload/allcaller/global/Linux/USA/Windowsnativo/nuovo32:9/fullTODO pendenti.
[Consumo e prove](../rom_data_maps.md#managed-encounter-headers-and-typed-guest-reads).


**ROMDATA-04-ENCOUNTER-HEADERS-CLI completato EUR/Wine (2026-10-28).** Un build
produzione Windows senzaavvisi/9d0a772d.. da510sorgenti accettati; canonicalhashmtime
ripristinati. Tre comandi Wine preparecreated/reused/runc700 passano:73file esatti
alruntime/72IDhash/75path no-write, 28859blocchi cache caricati. Copie normali
Giza02menu/03input19409→20109: 17letture/41B, sei classi semantiche
positive/1header/0opaque. 184guardie510hash/helper0; nessuna nuova navigazione,
checkpoint, misura o prova chiusa ripetuta. Prossimo unico fullpair con conteggiCLI
identici; non tutti442header/payload/opaqueordinari/fullABI/globalcatalogo.
Linux/USA/Windows nativo/nuovo32:9/fullTODO pendenti.
[CLI e consumo](../rom_data_maps.md#actual-encounter-header-cli-and-ordinary-consumption).


**ROMDATA-05-ENCOUNTER-HEADERS-BATTLE-ORDINARY completato EUR/Wine (2026-10-28).**
Un solo fullpair700 normale Giza02menu/03input19409→20109 con datasetCLI e80-d79:
ogni stato completo/clock/immagine classic-central-wide/OAM/dispatch/5audio passa.
Tutti11conteggi esatti allaCLI:17letture41B=12count/1map/4templateptr/0parameter/
1placementcount/20positionptr/3control/0opaque, unheader; sei classi positive.
Parameterordinary e opaque non osservati, prove mirate precedenti distinte.
Una guardia facoltativa5righe/vecchio validator byteexact, un buildWindows senza
avvisi/43fcc614.. da510sorgenti e un filtroWine. 103guardieordinarie+184CLI/510hash
ciascuno esatti/helper0. Nessuna nuova fixture/navigazione/prova chiusa ripetuta
oppure misura. Caso ordinario chiuso; tutti442header/payload/fullABI/globalcatalogo,
Linux/USA/Windows nativo/nuovo32:9/fullTODO restano pendenti.
[Equivalenza](../rom_data_maps.md#ordinary-encounter-header-battle-entry-equivalence).


**ROMDATA-01-ENCOUNTER-UNIT-TEMPLATES completato per il contratto sorgente EUR
(2026-10-28).** I442link header già autenticati partizionano2.850record48B,
6EA048..70B6A8/136.800B; proiezione completa indipendente. Nuovi lettori separano
selettori/argomenti, cinque equipmentu16, halfword12, bitsetfisico14..27 con
indice dinamico senza cap20B/160bit, controlli28/29/flags2A/2C/2D e opaque2E..2F.
Tutti14.250riferimenti equipment entro376item, selettori coerenti con proprietari
metadata14/metadata52; valori osservati non sono cap. Vecchi192B constructor
6ECA78..6ECB38/record225..228/header29 esclusi dai nuovi136.608B/two-span;
full48coherence e dipendenze header/constructor richieste, nessuna duplicazione
tra49.795span/72owner. Ottofinestre5nuove3riusate/39pin/4BL/41guardie passano,
nessun build/game/testWine-Linux/fixture/vecchia prova o censimento ripetuto/misura.
Prossimo modello rigoroso+un oracoloWine mirato; managed/runtime/CLI/ordinary/
coordinatepool/global/fullABI/Linux-USA/Windowsnativo/nuovo32:9/fullTODO pendenti.
[Contratto](../rom_data_gameplay.md#european-encounter-unit-templates-and-shared-ownership).


**ROMDATA-02-ENCOUNTER-UNIT-TEMPLATE-MODEL completato EUR/Wine (2026-10-28).**
Modello rigoroso lossless2.850record48B, full136.800B/two-span136.608nuovi,
192B constructorprecedenti esclusi da byte_at ma confrontati integralmente.
Shape/JSON/partitionheader/sharedcoherence/riferimenti item-metadata14-job e
sourceauth separati; fullu8/u16/opaque2, nessun capvalori o160bitguest inventato.
Un buildWindows core-only senzaavvisi/e0ae0d5c.. da512sorgenti/224coreinputs,
un filtroWine passa38guardie/2.100prefissi originali selezionati:1.020LDR,
1.080bit incl120indici oltre20B.1531byte nuovi/102old/4raw distinti osservati,
registri/indirizzi/valori e tuttaRAM/VRAM/OAM/palette/ROM esatti; proiezioneintera
indipendente, nonfullABI/coverage semantica ditutti2.850record. 34guardie512hash+
41sourceguard esatti/helper0; niente nuova fixture, misura o prova chiusa ripetuta.
Prossima pubblicazione e81-d80/index72+consumo tipizzato, poiCLI/ordinary;
Linux-USA/Windowsnativo/nuovo32:9/coordinatepool/global/fullABI/fullTODO pendenti.
[Modello e prove](../rom_data_gameplay.md#typed-encounter-unit-templates-and-original-reader-proof).


**ROMDATA-03-04-ENCOUNTER-UNIT-TEMPLATE-RUNTIME completato EUR/Wine (2026-10-28).**
e81-d80/index72/encounter-unit-templates.json,74file73risorse/tutti72precedenti
esatti/cinque dipendenze;192B constructorrestano al vecchioproprietario. Owner
immutabile136.608B/two-span, noveclassi e record/headerstats, routing6EA..70B
locale/globale. Un buildWindows senzaavvisi/3d4b2af2.. da514sorgenti e un filtroWine
passano14attach/7fullset/fallimentitransazionali,27prefissi original-interpreter-
native con stato completo/clock e136800Brawalterati dopoLoad, 25Bnuovi/8Bprior
compilati. Ogni136608Bnuovo+192old/9classi/2846record441header, larghezze/mirror/
confini/LoadUndo e DMAword34202/CpuSet68404halfword/FastSet34208word passano.
215guardie514hash512modelpreimages esatti/helper solo interrupted. ABI/algoritmi
invariati; USA8-7/schema1. ProssimiCLI reale+ordinary; coordinatepool/allcaller/
fullABI/global/Linux-USA/Windowsnativo/nuovo32:9/fullTODO pendenti, niente misure.
[Consumo e prove](../rom_data_gameplay.md#managed-encounter-unit-templates-and-typed-guest-reads).


**ROMDATA-04-ENCOUNTER-UNIT-TEMPLATE-CLI completato EUR/Wine (2026-10-28).**
Un buildWindows produzione senzaavvisi/afeaef25.. da514sorgenti runtime accettati,
canonicalhashmtime ripristinati. TrecomandiWine preparecreated/reused/runc700
passano:74file esatti alruntime/73IDhash/76path no-write, 28859blocchi cache.
Copie normali Giza02menu/03A700input19409→20109: 186letture/211B,
5record/1header, 8classi positive e1nonosservate. Criterio positivo
selector/argument/equipment/flag soddisfatto; tutti13conteggi conservati perun
fullpair successivo. 184guardie514hash/helper0, niente nuovafixture/navigazione/
misura o proof chiuso ripetuto. Ordinaryfullpair/tutti2850record442header/fullABI/
coordinatepool/global/Linux-USA/Windowsnativo/nuovo32:9/fullTODO pendenti.
[CLI e consumo](../rom_data_gameplay.md#actual-encounter-unit-template-cli-and-ordinary-consumption).


**ROMDATA-05-ENCOUNTER-UNIT-TEMPLATE-BATTLE-ORDINARY completato EUR/Wine
(2026-10-28).** Un solo fullpair700 normale Giza02menu/03input19409→20109 con
il datasetCLI e81-d80:ogni stato completo/clock/immagine classic-central-wide/
OAM/dispatch/5audio passa. Tutti13conteggi esatti allaCLI:186letture211B,
35selector/15argument/80equipment/10auxiliary/41bit/10optional/10flag/10control/
0opaque, cinque record e unheader; otto classi semantiche positive. Opaqueordinary
nonosservato, prova bus distinta. Una guardia facoltativa5righe/vecchiovalidator
byteexact, un buildWindows senzaavvisi/33c61082.. da514sorgenti e un filtroWine.
104guardieordinarie+184CLI/514hash ciascuno esatti/helper0; nessuna nuova fixture/
navigazione/misura/proof chiuso ripetuto. Caso chiuso; tutti2850record442header/
fullABI/coordinatepool/global/Linux-USA/Windowsnativo/nuovo32:9/fullTODO pendenti.
[Equivalenza](../rom_data_gameplay.md#ordinary-encounter-unit-template-battle-entry-equivalence).


**ROMDATA-01-ENCOUNTER-COORDINATES completato per il contratto sorgente EUR
(2026-10-28).**691liste/2342coppieXY/4684B70B6A8..70C8F4 partizionati esattamente
dai linkpointer-count header:nessun buco/alias/overlap fra49797span73owner.
323placement0liste3272B/34random680B/12placement2=88B/322default644B,323headercon
payload119tuttinulli. Rawu8XY preservano letture signedsearch e unsigned; RNG02868
nonnegative15bit/remainder146314/masku16 restano originali, nessuncap10/16 o
fallbacksafe sui null inventato. Novefinestre riusate+tre nuove helper/RNG,41pin/
38guardie passano; niente build/testWine-Linux/game/fixture/navigazione/prove
chiuse ripetute/misura. Prossimo modello rigoroso691liste+coerenzaheader e un
oracoloWine mirato, poi estrazione/runtime/CLI/ordinary. Schema1/e81-d80/USA8-7
invariati; Linux-USA/Windowsnativo/nuovo32:9/allcaller/fullABI/global/fullTODO aperti.
[Contratto](../rom_data_maps.md#european-encounter-coordinate-lists-and-original-readers).


**ROMDATA-02-ENCOUNTER-COORDINATE-MODEL completato EUR/Wine (2026-10-28).**
Modello rigoroso691liste2342coppie rawu8XY/4684B/691span; shape/coerenzaheader/
autenticità separati, vista signed diagnostica, nessunfallbacksafe null/capvalori.
Primo buildcoreWindows fallisceperimportBus neltest; solaimport corretta, seconda
copia516sorgenti accettata/2fe3e236.. senzaavvisi/227coreinputs, preimmagini conservate.
Un filtroWine passa30guardie/8044prefissi originali signed-unsigned su tutti4684B,
12highbit isolati e102prefissi randomoriginali(34liste*3seed). RNG/index/indirizzi/
masku16/scritture/callee-SP/outerRAM-VRAM-OAM-palette-ROM esatti; nonfullABI,
repeatedrandom/exhaustion/nativeclock/ordinary.40guardie516accepted+516failedhash/
38sourceguard esatti/helper0; nessuna nuova fixture/navigazione/prova chiusa
ripetuta/misura. Prossimo managed e82-d81/index73+typedbus, poiCLIordinary;
Linux-USA/Windowsnativo/nuovo32:9/allcaller/global/fullTODO pendenti.
[Modello e prova](../rom_data_maps.md#typed-encounter-coordinates-and-original-reader-proof).


**ROMDATA-03-04-ENCOUNTER-COORDINATE-RUNTIME completato EUR/Wine (2026-10-28).**
e82-d81/index73/encounter-coordinates.json,75file74risorse/tutti73precedenti
esatti/dipendenza header. Owner immutabile691span4684B/quattro ruoli+XY/statistiche
liste-header, routing locale eglobale70B..70C/composizione vecchiowner ai bordi.
Tre build Windows senza avvisi, accettato0f85fbf7.. da518sorgenti. Il primo filtro
Wine passa pubblicazione/14attach/3fullset e si ferma nel harness nativo; il
secondo isola la concatenazione oltre il punto di arresto. Corretto solo il test
con BLpair e due famiglie che escludono i rispettivi stop. Il terzo filtro riusa
il dataset, mantiene tutti22casi e completa native/lifecycle senza ripetere
pubblicazione o attach. I22casi original-interpreter-native coincidono nello
stato e clock completi con4684Braw alterati dopoLoad (18load/indice0gate/3randomprefix),
21Bnuovi/2template/13header compilati. Tutti4684B/691liste323header/
ruoliXY/691confini/width-mirror/LoadUndo/DMA-CpuSet-FastSet passano. 253guardie/
518hash516modelpreimages esatti/helper solo interrupted; algoritmi/ABI invariati.
ProssimiCLI reale+ordinary; Linux-USA/Windowsnativo/nuovo32:9/fullABI/allcaller/
global/fullTODO pendenti. Nessuna fixture/navigazione/misura/vecchio proof ripetuto.
[Consumo e prove](../rom_data_maps.md#managed-encounter-coordinates-and-typed-guest-reads).


**ROMDATA-04-ENCOUNTER-COORDINATE-CLI completato EUR/Wine (2026-10-28).** Un build
Windows produzione senza avvisi/b467d4ad.. da518sorgenti accettati, canonicalhashmtime
ripristinati. TrecomandiWine preparecreated/reused/runc700 passano:75file esatti
alruntime/74IDhash/77path no-write/28859blocchi cache caricati. Copie normali
Giza02menu/03input19409→20109: 20letture20B/1liste1header,
placement0-X-Y positivi; tutti10conteggi conservati. 186guardie518hash/helper0,
nessuna nuova fixture/navigazione/misura/prova chiusa ripetuta. Fullpair prossimo,
non tutte691liste/ruoli/header/ABI. Linux-USA/Windowsnativo/nuovo32:9/global/
fullTODO pendenti.
[CLI e consumo](../rom_data_maps.md#actual-encounter-coordinate-cli-and-ordinary-consumption).


**ROMDATA-05-ENCOUNTER-COORDINATE-BATTLE-ORDINARY completato EUR/Wine
(2026-10-28).** Un fullpair700 normale Giza02menu/03input19409→20109 con datasetCLI
e82-d81: ogni stato completo/clock/immagine classic-central-wide/OAM/5audio passa;
dispatch nativo e fallback eseguiti. Tutti10conteggi esatti allaCLI:20letture20B,
placement0=20, X10/Y10, una lista e unheader. Random/placement2/default nonosservati
qui, restano distinte le prove mirate già concluse. Guardia facoltativa5righe/
vecchiovalidator byteexact, un buildWindows senzaavvisi/de67e015.. da518sorgenti e
un filtroWine. 104guardieordinarie+186CLI/518hash ciascuno esatti/helper0.
Caso chiuso; tutte691liste323headerconpayload/placement-RNG/fullABI/global/
Linux-USA/Windowsnativo/nuovo32:9/fullTODO pendenti. Nessuna nuova fixture,
navigazione o misura di performance.
[Equivalenza](../rom_data_maps.md#ordinary-encounter-coordinate-battle-entry-equivalence).


**ROMDATA-01-LINKED-METADATA28 completato per il contratto sorgente EUR
(2026-10-28).**347record28B/9716B710E04..7133F8,17root e aritmetica28B,
famiglia successiva4B con due lettori indipendenti. Confine fisico, nessun cap
347 guest. GetterD07A4 low16/low8: scalari, vista interna+12, wordflag/21bit,
selettori34..255→0 senza letture dati. Lettori direttiu16+22/byte24..26;
byte3/15/27 opachi conservati. Tutti347record ricostruiti, nessuna sovrapposizione
con74owner/50488span,20finestre14nuove6riusate/69pin/122guardie/helper0.
Solo contratto statico; modello/Wine/runtime/CLI/ordinario prossimi. Schema1/
e82-d81 invariati. Linux-USA/Windowsnativo/nuovo32:9/global/fullTODO pendenti;
nessun build/gioco/checkpoint/navigazione/misura o prova chiusa ripetuta.
[Contratto](../rom_data_gameplay.md#european-indexed-28-byte-metadata-source-contract).


**ROMDATA-02-LINKED-METADATA28 completato EUR/Wine (2026-10-28).** Modello
lossless347record/9716B, campi unsigned/flag32/byteopachi, shape/sourceauth distinti;
lookup low16/low8 conserva scalari/vista indirizzo+12/21bit/default senza lettura,
nessun cap guest o fallback nullo inventato. Un buildWindows core senzaavvisi/
358dba2c.. da520sorgenti stabili e un filtroWine:27guardie/14517prefissi originali
(12088getter completi1388tail1041inline),8675byte interpretati osservati/1041opachi
ricostruiti,9varianti isolate342prefissi; RAM/callee/SP-LR/VRAM/OAM/palette/ROM
esatti. 36guardiemodello520hash+122sorgente esatti/helper0. Schema1/e82-d81
invariati; prossimi managed/runtime poiCLI/ordinario. FullABI/nextfamiglia4B/
global/Linux-USA/Windowsnativo/nuovo32:9/fullTODO pendenti, nessuna misura o
nuova fixture/navigazione/vecchia prova chiusa ripetuta.
[Modello e prova](../rom_data_gameplay.md#typed-indexed-28-byte-metadata-and-original-reader-proof).


**ROMDATA-03-04-LINKED-METADATA28-RUNTIME completato EUR/Wine (2026-10-28).**
e83-d82/index74/linked-metadata28.json:76file75risorse/all74precedenti esatti,
uno span9716B/nessuna dipendenza inventata. Owner tipizzato immutabile5classi/
347record/8contatori, pagine locali+globali710..713, bordi headerprecedente/
successor grezzo. Un buildWindows senzaavvisi/040133f5.. da522sorgenti, un filtroWine
pubblicazione/interruzione14attach2fullset/23casi nativi stato-clock completi con
9716byte grezzi alterati dopoLoad:17load+6getterD07A4,31byte nuovi+1header compilati.
Tutti9716byte/5classi347record/tutti347bordi/LoadUndo/DMA-CpuSet-FastSet3stati-clock
passano. 216guardie522hash520preimmagini modello esatti/helper ritira solo copia
interrotta. ActualCLI/ordinario prossimi; ABI/USA8-7/schema1 invariati, nessun
nuovo gioco/checkpoint/navigazione/misura o vecchia prova chiusa ripetuta.
FullABI/global/nextfamiglia4B/Linux-USA/Windowsnativo/nuovo32:9/fullTODO pendenti.
[Consumo diretto](../rom_data_gameplay.md#managed-indexed-28-byte-metadata-and-typed-guest-reads).


**ROMDATA-04-LINKED-METADATA28-CLI registrazione completata EUR/Wine;
uso ordinario positivo ancora aperto (2026-10-28).** Un buildWindows produzione
senzaavvisi/0acb352e.. da522sorgenti: sola correzione di3messaggi metadata28, inverse
byteexact/altri521file esatti/canonicalhashmtime ripristinati. Creazione/riuso/
avvii normali:76file esatti alruntime/75IDhash/78path no-write/28859blocchi cache.
Quattro locatori normali Giza700/Jobs540/Abilityset700/BattleTechRinfo800 hanno
TUTTI8contatori0, conservati come negativi da non ripetere; nessun fullpair
eseguito. Menu osservati, nuovi input da normali comandi su copia autorizzata;
nessun checkpoint nuovo o edit manuale guest. 220guardie522hash/helper0.
Prossimo: percorso di combattimento con azione non nulla prima di positivo/fullpair.
Linux-USA/Windowsnativo/nuovo32:9/fullABI/global/fullTODO pendenti; nessuna misura.
[CLI e limiti](../rom_data_gameplay.md#actual-indexed-metadata28-cli-registration-and-ordinary-locators).


**ROMDATA-04-05-LINKED-METADATA28-COMBAT completato EUR/Wine (2026-10-28).**
Apertura normale del menu Steal su terreno asciutto a Giza:42frame22669→22711,
15letture20byte serviti/cinque record,10halfword+10byte; inline/flag/opachi0.
Cattura, CLI esistente0acb352e.. e un fullpairWine coincidono in tutti8conteggi;
ogni stato completo/clock/immagine classic-central-wide/OAM/5audio passa. Nessun
effetto abilità eseguito o tutti347record rivendicati. Un buildWindows test senza
avvisi/5a6c81be.. da523sorgenti, guardia facoltativa5righe/vecchiovalidator byteexact
ed osservatore di soli input normali; sette catture1182frame su copie autorizzate,
nessun edit manuale guest/salvataggio originale. 177guardie523hash+220CLI/522hash
esatti/78path no-write/helper0. Quattro vecchi locatori negativi restano chiusi;
non si ripetono source/model/runtime/registrazione o prove di altre famiglie.
Caso ordinario chiuso; effetti/tutti347/fullABI/global/nextfamiglia4B/Linux-USA/
Windowsnativo/nuovo32:9/fullTODO pendenti, nessuna misura di performance.
[Equivalenza](../rom_data_gameplay.md#ordinary-metadata28-steal-menu-equivalence).


**ROMDATA-01-LINKED-RECORDS4 completato per il contratto sorgente EUR
(2026-10-28).** Intervallo7133F8..713744,844 byte in211 posizioni fisiche da4 byte;
sei root e lettori originali con indice u8 moltiplicato per4, confine distinto
con la famiglia successiva a passo8. Tutti i quattro campi sono letti unsigned;
indirizzi originali conservati nel contesto live. 1041 riferimenti dalle prime
tre posizioni inline dei347 record metadata28 raggiungono194 posizioni distinte,
tre riferimenti a indice0 e nessuno esterno. Nessun cap211/max208, limite globale
ai tre indici, enum o fallback nullo inventato. Gli zeri0/1/209/210 restano record
fisici distinti; significato riserva/padding dei due finali ancora da chiarire.
Ricostruzione completa844 byte, nessun overlap con75 risorse/50489 span e83-d82;
13 finestre autenticate (6 nuove,7 riusate),45 pin e112 guardie esatti/helper0.
Solo analisi statica, senza build/test/gioco/fixture o ripetizione di prove chiuse.
Prossimo: modello e prova mirata dei lettori originali su Wine; gestione/runtime/
CLI/uso ordinario di QUESTA famiglia ancora da implementare. Linux/USA/Windows
nativo/nuovo32:9/fullABI/cataloghi indiretti/fullTODO pendenti; nessuna misura.
[Contratto](../rom_data_gameplay.md#european-linked-four-byte-record-source-contract).


**ROMDATA-02-LINKED-RECORDS4 completato EUR/Wine (2026-10-28).** Modello core
lossless:211 posizioni fisiche con quattro campi u8/844 byte/uno span; shape,
ordine/indici/parser stretti separati dall'autenticazione EUR e13 finestre.
Indirizzo guest originale calcolato per tutti256 indici, anche211..255 esterni;
None diagnostico non diventa fallback guest. Zeri distinti e significato dei due
finali ancora aperto, nessun enum/cap/limite globale ai selettori inventato.
Un buildWindows core senza avvisi/a7270580.. da525 sorgenti stabili (233 input
core), un solo filtro Wine:20 guardie/1567 chiamate o prefissi originali,
1055 lettori coprono tutti844 byte e512 setter completi coprono entrambi i
contesti/tutti256 indici/alias low8/indirizzi esterni. Due varianti isolate con
bit alto passano10 prefissi unsigned e sono rifiutate dall'autenticazione base.
RAM completa salvo i5 byte attesi dei setter/callee/SP-LR/IWRAM/VRAM/OAM/palette/
ROM finale esatti;9808 istruzioni come sola copertura. 125 guardie525 hash e112
sorgente esatti/helper0. Schema1/e83-d82/save/native/CAPI invariati. Prossimo:
managed/runtime poi CLI/uso ordinario di questa famiglia; nativeclock/fullABI/
Linux/USA/Windows nativo/nuovo32:9/cataloghi indiretti/fullTODO pendenti. Nessun
gioco/checkpoint/misura/ripetizione di prove precedenti chiuse.
[Modello e lettori](../rom_data_gameplay.md#typed-four-byte-linked-records-and-original-reader-proof).


**ROMDATA-03-04-LINKED-RECORDS4-RUNTIME completato EUR/Wine (2026-10-29).**
e84-d83/index75/linked-records4.json:77 file76 risorse, tutti75 record/payload
precedenti esatti, uno span844 byte senza dipendenze inventate. Owner tipizzato
immutabile/quattro campi/211 posizioni/sette contatori; pagina713 locale+globale,
composizione per byte con metadata28 precedente e successore grezzo. Nessun
nuovo cap/null/limite ai selettori, ABI save/native/CAPI e USA8-7 invariati.
Tre buildWindows: primo fallito per riferimento del test e cache menu obsoleta,
secondo senza avvisi mai eseguito, terzo senza avvisi33144a8c.. da551 sorgenti
correnti; sole attese di allineamento LDR corrette prima di Wine. Un filtro Wine
passa pubblicazione/interruzione14attach2fullset/owner-stato intatti/21casi nativi
stato-clock completi con tutti844 byte grezzi alterati dopoLoad:13load4getter,
un prefisso doppio e3setter/quattro gruppi;41 istruzioni/21byte nuovi6precedenti.
Tutti844byte/quattro campi211posizioni/tutti211bordi/LoadUndo stessoArc e DMA3/
CpuSet424halfword/FastSet216word passano. 237 guardie551hash +due551diagnostici/
525preimmagini modello/sette hash correnti esatti; helper ritira solo copia
interrotta. Prossimo CLI reale/uso ordinario; Linux/USA/Windows nativo/nuovo32:9/
fullABI/cataloghi esterni/fullTODO pendenti. Nessuna misura/gioco/fixture nuova/
ripetizione di vecchie prove chiuse.
[Consumo diretto](../rom_data_gameplay.md#managed-four-byte-linked-records-and-typed-runtime-reads).


**ROMDATA-04-LINKED-RECORDS4-CLI registrazione completata EUR/Wine;
uso ordinario positivo pendente (2026-10-29).** Un buildWindows produzione senza
avvisi/bcf632fa.. da551 sorgenti identiche al runtime, senza overlay; soli
fingerprint corelib/prod isolati e canonical hashmtime ripristinati. Tre figli
CLI Wine escono0 (creazione/riuso/nuovo avvio120):77 file esatti al runtime,
76 ID/hash caricati,79 percorsi gestiti hashmtime no-write,28859 blocchi cache.
Copia del menu Steal22711 con Armor selezionato, A8hold12→22831 mostra puntamento
ma tutti7 contatori nuovi sono0. Caso negativo chiuso, nessun fullpair o uso
positivo dedotto; checkpoint finale non salvato dal comando CLI. 187 guardie/
551 hash CLI+551 runtime esatti/helper0, originali invariati; nessuna nuova
fixture o ripetizione di vecchie prove chiuse. Prossimo: risoluzione normale di
combattimento/bersaglio con letture positive e poi equivalenza completa. Linux/
USA/Windows nativo/nuovo32:9/fullABI/cataloghi esterni/fullTODO pendenti, nessuna
misura o modifica Android/package/commit/push dell'assistente.
[CLI e limiti](../rom_data_gameplay.md#actual-linked-four-byte-record-cli-registration).


**ROMDATA-04-05-LINKED-RECORDS4-COMBAT completato per un caso ordinario EUR/Wine
(2026-10-29).** Normali comandi Fight/Doit/facing portano al turno seguente;
suffix40 frame24755→24795,223 letture/223 byte serviti, campi64/132/11/16 e due
posizioni fisiche. Tutti7 contatori identici tra cattura, CLI BCF esistente e
un fullpairWine; ogni40 stato completo/clock/pixel classic-wide/OAM/5audio passa.
Un buildWindows senza avvisi/a5b2fc84.. da552 sorgenti: solo osservatore normale,
modulo test e guardia facoltativa5righe; inverse validator esatto/core invariato.
Dieci catture2084 frame su copie autorizzate, nessun edit manuale guest/ROM/cart;
un frameCLI classico soloUI, poi proiezioni framebuffer readonly autenticate senza
altra esecuzione. All76IDhash/79path no-write/28859cache, 210 guardie552hash+
187CLIguardie551hash esatti/helper0. Zero-case precedenti restano chiusi; nessuna
ripetizione di source/model/native/lifecycle/registrazione. Tutti4 campi osservati,
non tutte211 posizioni o successo attacco/fullABI. Nuova famiglia8B/cataloghi
indiretti/Linux-USA/Windows nativo/nuovo32:9/fullTODO pendenti, nessuna misura.
[Equivalenza](../rom_data_gameplay.md#ordinary-linked-four-byte-turn-equivalence).


**ROMDATA-01-CONDITIONAL-RECORDS8 contratto sorgente EUR completato
(2026-10-29).** Scansione D56A0 a root713744, passo8 e ordinali1..127:
127 posizioni/1016 byte fino al distinto root713B3C, i cui due lettori usano
passo70. Byte00 zero salta la posizione, byte01 zero wildcard; chiavi LE16 a
+2/+5 e operandi u8 a+4/+7. Flag live fino05FF e contatori u8 per key-0600:
primo flag richiede uguaglianza per operandi0/1 e counter>=operand; secondo flag
richiede disuguaglianza e counter<operand, solo seconda key0 bypassa. Nessuna
coercizione booleana/cap/indice header inventato.76righe abilitate/51zero,
105riferimenti live104flag1counter/63chiavi/24gruppi alias; operand2 preservato,
47secondkey0. Output puntatori ROM originali in ordine, count; capacità/filtro
u32 senza entryguard capacità0, entrambi chiamanti osservati passano64/filtro u8.
Helper370EB0/145C10 resta supporto eseguibile già classificato, non nuovo asset.
SHA256d52e829c2ae2e3f8a519fc1fe28797a1005fcdc0e2afc81201e872024608b18b,
ricostruzione completa/disgiunto76owner50490span;8finestre6nuove2riusate/93pin/
96guardie esatti/helper0. Query parole puntatore allineate:1root/38successore,
nessun interno/origin-8 in quella rappresentazione;2callsite verificati.
Solo analisi statica, nessuna build/Wine/gioco/fixture/vecchia prova ripetuta;
schema1/e84-d83/77file76risorse invariati. Prossimo modello e lettori Wine della
famiglia; managed/runtime/CLI/uso ordinario/Linux-USA/Windows nativo/nuovo32:9/
fullABI/caller indiretti/estensione successore/fullTODO pendenti. Nessuna misura.
[Contratto](../rom_data_gameplay.md#european-conditional-eight-byte-record-source-contract).


**ROMDATA-02-CONDITIONAL-RECORDS8 modello/oracolo EUR/Wine completato
(2026-10-29).** Modello core lossless127posizioni/1016byte/uno span; parser,
shape/ordine stretti separati dall'autenticazione baseROM+8finestre. Gate/filter/
chiavi LE16/operandi u8 conservati senza enum/cap/null inventati; select conserva
puntatori originali/ordine/capacità u32 zero-wrap/filtro fullu32 e molteplicità
chiamate live, polarità primo/secondo diverse. Un buildWindows core senza avvisi
c6f5406d.. da554sorgenti/239inputcore; sola nuova foglia+test+modulo, inversehub
byteexact, isolato fingerprintcoretest. Un filtroWine passa24guardie e109cicli
originali D56CE..D57A4 con veri lettori CCE98/CCF00:16scenari base,93varianti
sintetiche isolate con chiavi0/05FF/0600/0682/FFFF, operandi0..255, header255,
filtro128/255 vs contesto completo, capacità0/1/64/FFFFFFFF. OraclePython e
modello concordano con output/count/capacità/posizioni/chiamate effettive;
612byte base e134posizioni sintetiche osservate, non tutti1016byte letti.
EWRAM esatta salvo output atteso/IWRAM salvo scratch4byte returnlink/stack-r11r12/
VRAM-OAM-palette/ROMfinale esatti;198372istruzioni sola copertura.120guardie554hash/
3hashcorrenti+96guardiesorgente esatti/helper0; nessun gioco/checkpoint/navigazione/
vecchia prova o helperIWRAM ricostruito. Helperentry/epilogo/fullABI esclusi;
schema1/e84-d83/76risorse/save/native/CAPI invariati. Prossimo managed/runtime
poi CLI/uso ordinario; Linux-USA/Windows nativo/nuovo32:9/indiretti/successore70B/
fullTODO pendenti. Nessuna misura di performance o spazio.
[Modello e ciclo originale](../rom_data_gameplay.md#typed-conditional-records-and-original-selection-loop-proof).


**ROMDATA-03-04-CONDITIONAL-RECORDS8-RUNTIME completato EUR/Wine con prove
combinate (2026-10-29).** e85-d84/index76/conditional-records8.json:78file77risorse,
76precedenti esatte/uno span1016B/nodeps. Owner immutabile6classi/127slot/9contatori,
pagina713 locale+globale e composizione records4 precedente/successore grezzo;
logica guest/live/indirizzi e ABI save/native/CAPI invariati. Quattro buildWindows
senza avvisi da568sorgenti correnti coerenti; primo mai eseguito dopo correzione
pre-Wine del solo limite di iterazioni test. Primo filtro Wine passa pubblicazione/
interruzione14attach2fullset+16casi nativi stato-clock completi, poi fallisce nel
17esimo per osservatore607vs609. Diagnosi solo caso residuo prova stato-clock
raw/typed/native e609byte reali identici, completa bus/lifecycle; finale solo
osservatore passa609callback=607diretti+2fallback interp_one, nessun byte non
attribuito. Solo test corretto; non si ripetono pubblicazione/attach/16casi/bus.
17casi distinti:14load2prefissi LE1scansione; tutti1016byte grezzi alterati DOPO
Load/setup. Totale631byte serviti dal percorso nativo=629diretti2fallback e5byte
records4, non unione distinta o tutto compilato. Tutti1016busB/6classi127slot/
127bordi/failedowner-fullstate/LoadUndo stessoArc e DMA3/CpuSet510halfword/
FastSet256word fullstate-clock passano.261guardie568hash accettati+tre568diagnostici/
554preimmagini modello/7hashcorrenti/77vecchi78nuovi file esatti; helper ritira
solo copia interrotta. Exe accettato f60055e6.., primo fallimento conservato;
nessun gioco/checkpoint/navigazione/prova precedente o misura di performance.
Prossimi CLI reale e uso ordinario; Linux-USA/Windows nativo/nuovo32:9/fullABI/
indiretti/successore70B/fullTODO pendenti.
[Consumo e prove combinate](../rom_data_gameplay.md#managed-conditional-records-and-typed-runtime-reads).


**ROMDATA-04-CONDITIONAL-RECORDS8-CLI registrazione/avvio EUR/Wine completati
(2026-10-29).** Un buildWindows produzione senzaavvisi72d44985.. da568sorgenti
byteidentiche alruntime/nooverlay, fingerprintcorelib+prod isolati e canonical
hashmtime ripristinati. Tre figli CLI Wine escono0: creazione e85-d84/riuso/avvio
neutro1;78file esatti alruntime/77IDhash caricati/80path gestiti nowrite/28859blocchi
vecchi. Primo prepare0 ha successo; helperlocale controllavaindice75 anziché76:
corretto e ripresi solo due figli rimanenti, senza rifare prepare o build.
Copia normale before-activation24755→24756 coninputneutro, tutti9contatori nuovi0;
nessuna azione/navigazione/checkpoint nuovo/endpoint salvato, nessun positivo
ordinario/fullpair dedotto.189guardie568CLI+568runtimehash esatti/helper0, originali
invariati. Source/model/native/lifecycle/vecchie scene restano chiusi. Prossimo
uso ordinario positivo e confronto completo; Linux-USA/Windows nativo/nuovo32:9/
fullABI/indiretti/successore70B/fullTODO pendenti, nessuna misura o modifica
Android/package/commit/push dell'assistente.
[CLI e limiti](../rom_data_gameplay.md#actual-conditional-record-cli-registration).


**ROMDATA-04-05-CONDITIONAL-RECORDS8-PUB completato per un caso ordinario
EUR/Wine (2026-10-29).** Ingresso normale Rumors nel pub; suffix42f15181→15223,
468letture/468byte SERVITI, sei classi127/76/152/76/34/3 e127posizioni fisiche
incluse51intestazioni zero, non127record attivi/tutti campi/tutti rami/unione byte.
Tutti9contatori esatti tra cattura/CLI72d44985 efullpair63584945; ogni42stato/clock/
pixel classic-wide/OAM/5audio uguale. Un buildWindows senzaavvisi da569sorgenti:
CLI568+osservatore passivo+modulo+guardia opzionale5righe; inversehub-validator
byteexact/core-runtime invariati. Due catture normali254frame: B180negativo poi
Up3/A12positivo74, primo readrel42+32continuazione, rollingrel32→suffix42.
Seed MissionsLIST14969 giàconservato, nessun vecchio620/397route replay; vecchio
PPMclassic240x160 scalacolore*255//31 autenticato controFFSTATE, nonformato wide
recente. Proiezioni readonly eimmagini verificate, nessun WineUI/guestedit/cart
write; promptfinaleHeardlatestRumors, lista/selezione/esito non dedotti.
143guardie569hash+568CLI/3roothashes/80managedpaths nowrite/28859cache esatti,
helper0. CLIe85generationhw22uM riusata senza prepare/build/navigazione; primo
B0case resta chiuso. Declaredsource/model/runtime/CLI/ordinario8B chiusi;
prossimo nuovo root713B3C/passo70 richiedecontratto proprio. Linux-USA/Windows
nativo/nuovo32:9/fullABI/indiretti/fullTODO pendenti; nessuna misura o modifica
Android/package/commit/push dell'assistente.
[Equivalenza](../rom_data_gameplay.md#ordinary-conditional-record-rumors-entry-equivalence).


**ROMDATA-01-MISSION-RECORDS70 contratto sorgente EUR completato
(2026-10-29).** Catalogo713B3C..71C73C/512recordx70/35840B, due scansioni
indipendenti1..511 più getterindice0/confine successore con lettori propri;
costante300 separata, nessun nuovo cap guest512. Getter65selettori indice low16/
selettore low8: LE unsigned da byte, bit impacchettati, flag, mapping zero oppure
+0177 e scala200,7selettori indirizzo/6offset inline, alias19/20 e viste24/32 con
scalari condivisi. Byte residui/valori grezzi/identità ordine preservati. Mission
link giàchiuso riusato: campo+45HEX69decimale eventrow*12+71C760;512riferimenti/
173valori/quattrogruppi alias, zero dipende dal chiamante; catalogo eventi distinto
ancora da classificare. Tutti35840B ricostruiti, SHA256
bbdbc955b9b3f7ed678123aa6bea8b6098f25f6a13a4ab94de669d1e103253dc, disgiunti
77owner50491span/e85-d84/78file invariati.19finestre9nuove10cache/157pin110guardie
esatti; vecchio testo autenticato opcode-per-opcode contro ROM e hash corrente,
nessuna falsa guardia testo storica.38literalbase/2origin+70/2candidati interni
in dati non classificati lettori, query allineata non esaustiva indiretti. Solo
statica, nessuna build/Wine/gioco/fixture/navigazione/prova precedente ripetuta;
helper ritira zero path. Prossimo modello rigoroso lossless/oracolo getter Wine,
poi managed/runtime/CLI/ordinario. FullABI/semantiche campi ignoti/conteggi viste/
successori/Linux-USA/Windows nativo/nuovo32:9/fullTODO pendenti. Nessuna misura di
performance o spazio; lavoro parallelo menu/content/DBG/package preservato.
[Contratto](../rom_data_gameplay.md#european-mission-record-source-contract).


**ROMDATA-02-MISSION-RECORDS70 modello/oracolo EUR/Wine completato
(2026-10-29).** Nuovo core mission_records70 lossless512record/35840B/unospan,
shape/ordine/JSON rigorosi separati da autenticazione ROM esatta+19finestre.
Byte residui/array inline/operandi grezzi e tutti65selettori preservati; query
indice low16/selettore low8, Some0 invalido senza lookup, indirizzi calcolati
anche fuoriowner, scalare nonpossedutoNone diagnostico senza nuovo cap guest.
Un buildWindows core senzaavvisi663e8b87.. da571sorgenti/242inputcore,569preimmagini
esatte e inversehub byteexact; sole duefoglie/pubmodule, nessun frontend parallelo.
Un filtroWine passa21guardie e698chiamate getter originale D1F30 finoalreturn con
vero helperCEEEC:438base/all65agliindici0/1/58/299/300/511/normalizzazione/default/
viste fuoriowner/scalari512-65535,260varianti sintetiche isolate suquattro70Bpattern
alrecord511. Pythonindipendente/modello/risultati eordine lettureLDRB effettivo
esatti;324byte base distinti/54sintetici contro35840B ricostruiti separatamente,
viste non dereferenziano16B inline. EWRAM/VRAM/OAM/palette invariati, IWRAM esatta
inclusi linkreturn previsti, SP/R4..R12/Thumb/ROMfinale esatti;17213istruzioni
solacopertura.139guardie571hash/3correnti569preimmagini+110guardiesorgente/helper0.
Nessun gioco/checkpoint/cartwrite/navigazione/prova giàchiusa o misura. Schema1/
e85-d84/77risorse78file/save/native/CAPI invariati. Prossimo managed/runtime/
nativeclock/lifecycle poiCLI/ordinario; fullABI/campi ignoti/indiretti/successori/
Linux-USA/Windows nativo/nuovo32:9/fullTODO pendenti. Lavoro parallelo preservato.
[Modello e getter](../rom_data_gameplay.md#typed-mission-records-and-original-getter-proof).


**ROMDATA-03-04-MISSION-RECORDS70-RUNTIME completato EUR/Wine con prove
combinate (2026-10-29).** e86-d85/index77/mission-records70.json/79file78owner,
77precedenti esatti/uno span35840B/nodeps; ownerimmutabile10classi13contatori/
8maskword512posizioni/pagine713..71c, composizione8Bprecedente e successori grezzi.
Due buildWindows senzaavvisi573sorgenti/243inputcore:73326ad9 prima pubblicazione/
prior77/interruzione PASS poi primoattach FAIL cap storicoOBJ1024, altri test
nonpartiti. Con richiesta no-limiti-spazio rimossi capattachOBJ1024/BG64KiB e
treasserzioni di allocazione, parsebounds/sourceauth/binding invariati.455bdb1c
secondo build/filtroWine SOLOresidui sulla STESSAgenerazione79file:14attach2fullset/
failedownerfullstate/41nativi stato-clock(16load25getter) con35kgrezzi alterati
DOPOload/setup/all35kbus10classi512slot/512bordi/LoadUndo stessoArc/DMA3passano.
Pubblicazione nonripetuta, primo fallimento conservato; nessun capfix interpretato
come ottimizzazione o misuraRSS. Tutti callback inclusointerp_one attribuiti:
52byte nuoviserviti5precedenti/fallbackdelta0, nonunione distinta/tuttocatalogo
nativo;3famiglie stop-safe/168entry69dispatch sola copertura. DMA/CpuSet17922
halfword/FastSet8968word statocompleto-clock identici.860guardie573accettati+573
diagnostici/571modello/8hashcorrenti/78vecchi79nuovifile esatti;helper ritira solo
copiainterrotta. close-final.pyPASS-combined-scopes, vecchio close-runtime.py
non eseguito. Source/modello/vecchie scene nonripetuti, nessun gioco/checkpoint/
navigazione/cartwrite/ABI Machine-save-native-CAPI cambiato. ProssimoCLI reale
poiordinario; Linux-USA/Windows nativo/nuovo32:9/fullABI/indiretti/successori/
fullTODO pendenti. Nessuna misura performance/spazio, lavoro parallelo escluso
dabuild e preservato.
[Consumo e prove](../rom_data_gameplay.md#managed-mission-records-and-typed-runtime-reads).


**ROMDATA-04-MISSION-RECORDS70-CLI registrazione/avvio EUR/Wine completati
(2026-10-29).** Un buildWindows produzione senzaavvisi738b7f20.. da573sorgenti
byteidentiche alruntime/nooverlay, fingerprintcorelib-prod-menu coerenti e
canonicalhashmtime ripristinati. Tre figliWine tutti0: preparazionecreata/riuso/
avvio neutro1,79file esatti alruntime/78IDhash/81path gestiti nowrite/28859vecchi
blocchi nativi. Generatoreprivato fermato su assertiontokenindice prima diogni
Wine: correttasola generazionehelper conindice77 verificato, nessuna copia
sorgenti/build/preparazioneCLI ripetuta. Copianormale24755→24756 all13contatori0;
nessuna azione/navigazione/checkpoint nuovo/endpoint salvato, positivoordinario
fullpair o nuova fase negativa del locator dedotti.195guardie573CLI+573runtime/79file/
originali/nativecache/loader/canonical esatti;helper0. Source/model/native/
lifecycle/vecchieprove restanochiusi. Prossimo positivoordinario e confronto
completo; Linux-USA/Windows nativo/nuovo32:9/fullABI/indiretti/successori/fullTODO
pendenti, nessunamisura/Android/package/assistantcommit/push.
[CLI e limiti](../rom_data_gameplay.md#actual-mission-record-cli-registration).


**ROMDATA-04-05-MISSION-RECORDS70-MISSIONS completato per un caso ordinario
EUR/Wine (2026-10-29).** Apertura normale informazioniGizaPlains, suffix33frame
15017→15050:5letture/5byte SERVITI solo classepacked/una posizione fisica, altre
9classi0. Tutti13contatori esatti cattura/CLI738b7f20/fullpair3d21e89c; ogni33stato
completo-clock/pixelclassic-wide/OAM/5audio uguale, entrambelebraccia stessa
vecchiacachenativa/nonparitàglobaleinterprete-nativo. UnbuildWindows senzaavvisi
574sorgenti=CLI573+osservatorepassivo/pubmodule/guardiaopzionale5righe, inversehub
byteexact/core-runtime invariati. UNA cattura81f daMissionsLIST14969 giàconservato:
Down3at8/A12at40, primalettura rel49/guest15019, rolling48→suffix33 A4+neutral29;
ricercaSTOP. TestWinechild0 PASS; helperpostcatturaKeyErrorsteps dopooutputcompleti
riparatosoloregistrazione, nessuna cattura/input/Wine/build ripetuta. Seedclassico
240x160/scale giàautenticato riusato; nuovoPPMwide284x160/bitrep verificatocontro
FFSTATE completo, classicPNG mostraGizaMISSIONinfo page1/3Rank1. Nessuna missione
accettata/battaglia/esito/tutticampi/all512/unione5bytedistinti dedotti.
133guardie574hash+573CLI/3correnti/78IDhash81managedpaths nowrite28859cache esatti,
helper0/originali invariati. Vecchie620/397route/source-model-runtime/registrazione
non ripetute, nessunaforzaturaID/RAM/cartwrite. Declared70Bchain chiusa; nuovi
arrayC73C/C740 eeventC760 richiedonocontratti propri. Linux-USA/Windows nativo/
nuovo32:9/fullABI/indiretti/altreclassiordinarie/fullTODO pendenti; nessunamisura
performance/spazio/Android/package/assistantcommit/push.
[Equivalenza](../rom_data_gameplay.md#ordinary-mission-information-equivalence).


**ROMDATA-01-SELECTION-BYTE-TABLES contratto sorgente EUR completato
(2026-10-29).** C73C..C740 quattro soglie di conteggio eC740..C75E trenta pesi
ordinali:34byte/two span, SHA256d758af5f1a07613f1c2047257d42ef9cbfac83b63783ee6fd0e7b3879370b8dd.
Cinque header live02002E34+6*i contati solo low3bit nonzero, tutti esaminati;
guard<=3 prima dellaloadu8 soglia,4/5escono. Secondoroot indipendente, duecicli
stride1 ordinali1..30/posizioni0..29. Verohelper377BC leggeu8 grezzo live
02002CC3+12*(low8ordinal-1), nessun guardglobale0/30 o normalizzazionebool;
callertesta low8nonzero. Somma0esceprima secondo stadio; rangohelper full32
meno peso unsigned conwrap e confrontosigned, primonegativosceglie,31esaurito
esce; nessunguardnegativo/zero-weight/cachedeligibility inventato. Numerichelper
2868/146314/RNG/distribuzione/fullABI nonmodellati. Tutti34B ricostruiti/posizioni
conservate/valori ripetuti nondeduplicati/nopuntatori dati; disgiunti78owner
50492span/e86-d85/79file invariati.6finestre2nuove4cache/95pin96guardie/helper0,
sole2nuovecontestocount ehelper live; nessuna build/Wine/gioco/fixture/navigazione/
vecchiaprova ripetuta o misura. GapC75E..C760 dueB opachi nonposseduti, next12Broot
C760indipendente/estensione pendente. IDneutrogameplay.selection-byte-tables.
Prossimo modello/oracoloWine poi managed/runtime/CLI/ordinario; Linux-USA/
Windows nativo/nuovo32:9/indiretti/fullTODO pendenti.
[Contratto](../rom_data_gameplay.md#european-selection-byte-table-source-contract).


**ROMDATA-02-SELECTION-BYTE-TABLES completato modello/stadi EUR/Wine
(2026-10-29).** Modello strict4+30u8/34B/due span, shape distinta da ROMesatta
+seiwindow; gap2B escluso/valori ripetuti distinti. Un buildcore Windows senza
warning b40503b8..576file245input/574preimmagini; un filtro Wine18guard/45base+
24sintetici esegue D323Ccount/D327Esomma-zero/D32B4scelta con vero377BC.
Python/modello/risultati/ordine querylive e LDRB esatti; wrap32/rango firmato
anche estremi, nessuno shortcutpeso0 o nuovocap. Tutti34Bbase/32sintetici osservati,
ricostruzione34 separata, tutteRAM/SP-R8..R12/ROMfinale esatti;30480istruzioni
solo copertura.123guard576hash/3correnti/574preimmagini/96sourceguard PASS,
helper0. Schema1/e86-d85/79file invariati; runtime gestito/bus/nativeclock/
LoadUndo/DMA/CLI/ordinario prossimi, helpernumerici/RNG/fullABI/Linux/USA/
Windowsnativo/new32:9/TODOglobale pendenti. Nessuna fixtureordinaria o prova
chiusa ripetuta/metriche.
[Modello e prova](../rom_data_gameplay.md#typed-selection-byte-tables-and-original-stage-proof).


**ROMDATA-03-04-SELECTION-BYTE-TABLES-RUNTIME completato EUR/Wine
(2026-10-29).** e87-d86/index78/selection-byte-tables.json/80file79risorse,
due span4/30B/nodeps, tutti78record/file precedenti identici/schema1-USAe8
invariati. Ownerimmutabile page71C compone prior70B e rawgap/eventroot;
6contatori/4+30posizioni distinte, nessun capallocazione o cambioABI. Un build
Windows senza warning e5b45668..578file246core/576model, un filtro Wine
pubblicazione/interruzione/14attach2fullset/20native fullstate-clock/all34bus/
LoadUndo/DMA3 PASS.12load+8stadi/vero377BC, raw34+prior4 alterati solo dopoLoad;
76nuoviBserviti+5prior tutti attribuiti a callback, fallbacksource0,91entry119
dispatch solo copertura. Width/mirror/confini/prior/raw, tutte4+30posizioni e
DMA-CpuSet20halfword/FastSet16word exactstate-clock.243guard578hash576model/
7correnti/79prior exact; helper ritira solo interrupted riuscito. CLIreale/
ordinario prossimi; numerichelpers/RNG/fullABI/gap-eventextent/Linux/USA/Windows
nativo/new32:9/TODOglobale pendenti, nessuna vecchiaprova o fixture ripetuta/
metriche. [Consumo](../rom_data_gameplay.md#managed-selection-byte-tables-and-typed-runtime-reads).


**ROMDATA-04-SELECTION-BYTE-TABLES-CLI completato EUR/Wine registrazione-avvio
(2026-10-29).** Un buildproduzione senza warning22af4132.. stesse578sorgenti
runtime/nooverlay/canonico hashmtime ripristinato; una orchestrazioneWine tre
comandi create-reuse-neutral1 exit0.80file datasetidentici/79IDhash/82percorsi
hashmtime invariati/cache28859immutata. e87-d86/index78/two spans34B/no deps
verificati;24755→24756 tutte6usage0, nessunaaccettazioneordinaria.192guard
578CLI+578runtime/ROM-nativecache-TinyCC-seedoriginale-canonico exact/helper0.
Nessunanavigazione/nuovocheckpoint/vecchiaprova/metriche. Positivoordinario e
fullpair prossimi; Linux-USA-nativeWindows/new32:9/fullABI/RNG/indirect/gap-event/
TODOglobale pendenti. [CLI](../rom_data_gameplay.md#actual-selection-byte-cli-registration).


**ROMDATA-04-SELECTION-BYTE-TABLES-ORDINARY completato un caso EUR/Wine
(2026-10-29).** Otto nuove window/354opcodepin direttiD3724→D3228/guardarglow8
nonzero; wrapperD6970passes1, altri2callers0skip, indirectignoti. Quattroseed
readonly guard6/count1-2; sceltonormaleGiza19049/cachedUI. Un579source8f3bee97
build observer6stats+5lineguard/no core-runtimechanges/inversehub exact.
UNA captureA12/129f19049→19178/first97-19147/rolling96→suffix33-19145→19178.
SAME22afCLI/e87ZloxMx positivo33 eSAME8f3bee97 fullpair33 tuttecomplete states-
clock/classic-wide/OAM/5audio PASS; counts6exact1bus1SERVEDthresholdB1position/
weights0, non tutte4/30posizioni oevent/missionoutcome. DiscoverySTOPafterone.
Fullwide284x160bitrep authenticated/classicmapcalendarPNG inspected.140guards
579hash578CLI/3correnti/82managed/nativecache-ROM-seed-canonical exact/helper0.
Dispatch203258/519441 eaudio[72418,72418,8712,8712,72342] solo copertura; stessa
cache inentrambi arms/non globalinterpreter-nativeparity. Nessuna prova chiusa
o fixture ricreata/metriche. Catena source-model-runtime-CLI/unordinario chiusa;
event12BrootC760/alias71D1EC/next71D360 richiede contrattoproprio. Linux-USA-
Windowsnativo/new32:9/fullABI/RNG/indirect/gap-event/TODOglobale pendenti.
[Ordinario](../rom_data_gameplay.md#ordinary-selection-threshold-travel-equivalence).


**ROMDATA-01-EVENT-RECORDS12 completato contratto sorgente EUR (2026-10-29).**
71C760..71D360/256x12/3072B/onespan SHAe7454120.. disgiunto79owner50494spans;
backscan255..1/stride12+liveu8zero+nextrootindipendente27stride provanoestensione,
nonfitonly/globalcap. Layoutu8header00-key01/u16primary02-alternate04/u8param06-
byte07/u16flags08/opaque0A2B, tutti3072roundtrip/noenums-FKcaps-zeropadding.
71D1EC=base+225x12/31physicalpositions vistaINTERNA/sameowner, helper9D04low8arg
noncappato31/puòuscireowner; sceltaordinal1..30→righe226..255. Callback127388
keyu16completo/vsbyte01/highestmatch255..1/zeroexcluded/8keygroups1recordgroup.
Alt04nonzero→aliasprimary02+originalalt04, zero→directprimary/alt0; key4Alive
special/param06next27root/liveeffects originali.15window10new5cached/67pin/
112guard/80acceptedfiles PASS/helper0; census36base2alias25next e nonindirectall.
Solo statico/no build-Wine-game-fixture-navigation-oldproofreplay;
schema1/e87-d86 invariati. Modello/originalprefixWine prossimi, poi managed/
runtime/CLI/ordinario; fullABI/liveeffects/indirect/gap/next27extent/Linux-USA-
Windowsnativo/new32:9/TODOglobale pendenti/no metrics.
[Contratto](../rom_data_gameplay.md#european-event-record-source-contract).
