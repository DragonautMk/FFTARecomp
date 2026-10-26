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
