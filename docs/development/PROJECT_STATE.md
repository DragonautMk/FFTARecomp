# Current development knowledge

This file stores durable facts and entry points. Chronological changes and their
validation belong in [CHANGELOG.md](../../CHANGELOG.md) and `docs/changes/`.
Read current source and manifests before relying on older build instructions.

Task selection after compaction or goal continuation starts with
[TASK_HANDOFF.md](TASK_HANDOFF.md): completed substeps, rejected experiments,
evidence and one next open work unit. An unchecked parent TODO does not reopen
completed children. See [repeat rules](MEMORY_WORKFLOW.md#resume-and-repeat-decisions)
before rebuilding, rerecording fixtures or rerunning an unchanged benchmark.

## Architecture and current scope

- [Godot frontend assessment](GODOT_FRONTEND_TODO.md) is planning only,
  explicitly deferred until ROM extraction is finished and the user selects
  development. No prototype remains. Machine is reusable as a library, but
  full native dispatch and application lifecycle are still private binary
  modules; a shared runtime boundary is a prerequisite for native frontend
  parity. Do not infer implementation approval from this deferred TODO.

- [EUR audio source contracts](../rom_data_audio.md) identify the original
  M4A driver,3player configurations,640song-directory entries and455unique
  headers with explicit dummy-header/WaveData alias. A graph over1610track
  roots identifies390bank/program pairs,429tones,347keymap B and277PCM waves;
  a focused Wine source check confirms program/map selection and276interpolating
  endpoints plus one fixed-frequency endpoint. Their736-range/1941759Bsource
  plan now has a shared bounded producer parser and strict lossless typed
  tone/keymap/signedPCM/CGBmodel. One focused Wine decoder/independent hashes/
  alias-dependency check and2neworiginal CGBcopy checkpoints pass. Managed
  publication plus guest and MP2Kshadow-sampler consumption now pass in e42-d41:
  37files/36resources/all35prior decoded/e41retained, actual directory alias
  dependency. Two reduced Wine checks pass26guards/lifecycle/all1941759B/
  25original-interpreter-native fullstate-clock checkpoints/736edges-SWI-DMA;
  both actual MP2Khook/tick callsites read typed headers/samples for all277waves,
  nonzero bit-exactshadow audio and completehoststate-clock equality. Internal
  checkpoints remain partial. ProductionCLIcreate/reuse-no-write39managedfiles/
  oldcache and one400ordinary Winefullstate-image-audio pair pass516956guestB
  and3093208hostB(1056header/3092152PCM); CGBordinary0. DefaultCLIshadowoff is
  explicit; differential arms shadow after load. No newperformance measurement.
  No local640song or128tone guard is inferred. The strict15,236Bsource model
  and one focused Wine original-reader oracle pass640entries/nine targeted
  inputs/one raw lookup/six synthetic variants; unused bytes stay explicit.
  EUR e41-d40adds the typed audio directory:36files/35resources/457new source
  spans, all earlier decoded files/e40retained, USAe8-d7. Two focused Wine
  publication/authentication/lifecycle and44original-interpreter-native start
  paths pass with raw15236Baltered/fullstate-clock equality. ProductionCLI
  create/reuse/oldcache and one400-frame ordinary Wine fullstate-image-audio
  pair pass179newsourceB across directory/header/track fields (no player reads).
  Sequence source model now derives105253nonoverlapping instructions from
  1610roots/805ranges177047B, with canonical branch/pattern IDs and explicit/
  inherited opcode/typed operands.49clockB/36commandwords144B/CD8-9dispatch8B
  add3spans:808sources177248B. One focused Wine source/model check passes all
  independent fields/hashes/27guards and404new original-reader checkpoints;
  no fullsequencer/native/clock/ordinary claim for that source check. e43-d42now
  publishes sequences with actual directory+tonebank dependencies:38files37
  resources/all36prior/e42retained. Two reduced Wine checks pass25guards/
  lifecycle/all177248B/allinstruction-table masks and35original-interpreter-
  native fullstate-clock checkpoints,808edges-BIOS-SWI-DMA/generic-host cross-
  owner/rawfallback/liveRAM runningoverride. ProductionCLIcreate/reuse-no-write
  40managedfiles/oldcache and one400ordinary Winefullstate-image-audio pair pass
  819newB(366opcode/262operand/191clock),185source instruction records including
  lookahead;0command/extended/generic-host. No hostsequencer/wholeABI claim.
  Seven further driver tables370b20..370d04/484Bnow have a strict typed source
  model and oneWineoracle:180PCMkeys/12u32bases/12samplecounts/132CGBkeys/
  12i16bases/60noise/16volume codes.1138neworiginal probes pass unsignedmulhi/
  signedASR/truncation-clamps/sample0-13-15adjacent aliases/publiczero skip/live
  volume aliases; all484B read at observed original loadsites. Eight additional
  targeted gain returns correct the initial cap15inference: balanced liveinputs
  canyield16/23/31, while one-sided routing clamps15. No ordinaryreachability/
  global16-domain claim. e44-d43now publishes/consumes seven tables:39files/
  38resources/all37prior/e43retained, actualsequence boundarydependency.
  Two reduced Wine checks pass19guards/lifecycle/all484B/424entry bits and
  28original-interpreter-native complete-state/clock paths including mixed
  Thumb/ARM/aliases/BIOS-SWI-DMA/guest-host boundaries. No fullinit/mixerABI;
  productionCLIcreate/reuse-no-write41files/oldcache and one400ordinary Wine
  fullstate-clock-image-audio pair pass756aggregate byte reads/67entries from
  PCM/CGB key-base tables.0samplecount/noise/volume/generic-host ordinaryreads;
  their targeted scope remains distinct. Queued370eb0/b94690are nowreclassified:
  copied-IWRAM memoryhelpers (executable support), and palette-effect dispatch.
  Fivefixed constructors/state1/unmaskedu16index prove a17word68Bprojection with
  12nullslots; no globalmax16guard/fulltableextent. Static7literal/5producer/hash/
  all38priorowner checks pass. Strict17slot68Bpalette model/oneWinecheck nowpass
  independent fields/hash/JSON/12guards/31neworiginal prefixes, all68Bobserved;
  source-only scope distinct. e45-d44now publishes40files/39resources/all38prior/
  e44retained. Two reduced Wine checks pass12guards/lifecycle/all68B/17slotmask/
  10original-native complete-state/clock prefixes/livekind-null-raw aliases/
  BIOS-SWI-DMA/boundaries. ActualproductionCLIcreate/reuse-no-write42files/
  oldcache and one normal English-confirm1000Winepair pass everyfullstate-clock/
  image-OAM-dispatch-audio with5944aggregate byte reads/kind1only(mask2), other
  callbacks/nulls targeted-only. Neutralboot240closed0use/no fullpair. No whole
  callback/allocatorABI. Paletteprogram inventory nowroots185programs/1000commands
  from285literalexpressions/3countedlists/2stackcopies:16135candidateprogramB+
  88producerBdisjointfrom39e45owners.20indexedproducer projections now follow
  explicit task-ID/message/constant/count-loop evidence:97cells388B add45programs,
  total230programs/1135commands/18221programB+476producerB=18697B/165ranges, disjoint
  all39owners.185earlier records unchanged;39bcc4only2pointers, mixedtail excluded.
  These are no globalRAM guard/fullphysicaltable claim. Strict lossless typedmodel
  nowreconstructs165ranges/18697B and oneWine sourcecheck observes all18221programB/
  18681allB (16reservedlistBpreserved/unread),97cells/8rows/2stacktables,6copyboundary/
  3count-period/1direction probes and16shape-JSON-source guards; warningfreeWindows.
  Source/model childclosed; no fullcallback/native/stateclock/ordinary claim.
  e46-d45nowpublishes41files40resources/all39prior-e45retained and consumes typed
  header-command-parameter-producer fields. ReducedWine passes12guards/lifecycle/
  all18697B/230programs25tables and24original-native complete-state-clock prefixes/
  rawoutside/BIOS-SWI-DMA; testchaining endpoint corrected, emitterunchanged.
  ActualCLIcreate/reuse-no-write43files/oldcache passes. English-confirm1000reads0
  newB/closednegative/no fullpair;10protected worldmap-bank snapshots have no
  palette-program task. Actual EUR Montblanc Richiamo/Pecorelle CLI and one
  1000-frame Wine fullstate-clock-classic240x160-image-OAM-dispatch-audio pair
  nowpass170sourceB/4programs (32header/90command/48parameter),0producer. The
  generic test guard accepts typed bus or host consumers; selected palette-use
  assertions remain strict. Other226programs/tables targeted-only/new32:9 pending.
  Neighbor080d77c4is nowidentified as blendcontrol via originalDMAshadow→
  BLDCNT/BLDALPHA/BLDY mapping. All248direct constructor calls are classified;
  CFG-checked constants/conditional/indexed/inline/RAMtemplate producers yield
  160programs/345commands/2340B plus22pointercells88B=2428B/158ranges, disjoint
  all40e46owners. False code-root080e2ec8rejected; actualmutableRAMtemplate from
  39a5f0has a live duration override. Independent static reconstruction/hash/
  44consumer opcodes pass. Strictlossless160program/6table source model now
  authenticates110opcodes/all2428B158ranges. Warning-freeWindows/oneWineoracle
  passes160headers345commands22cells/5instantchains/template-RAMoverride/packed
  masks31-mode0/6duration/3delay/index65535raw;17shape3source2JSON2identity guards.
  2376Bobserved/52nonalpha timed second bytes preserved-hashchecked only for
  these initial modes. No fulltaskABI claim. EUR e47-d46nowpublishes/consumes
  42files41resources/all40prior-e46retained/USAe8-d7 unchanged;158bindings2428B
  serve typed header/command/producer fields. Reduced Wine passes12guards/
  lifecycle/all2428B/160program6tables and20original-native fullstate-clock
  prefixes/[34header70command16producer]compiledB/live2→1mode/rawcommand65535-
  index3/previouspaletteindex5/new2-old2/SWI-DMA. Source-set coherence separately
  passes onefocused Wineguard; production backend/source proofs unchanged.
  ActualCLIcreate/reuse-no-write44managedfiles/oldcache and one1000Wine ordinary
  EURPecorelle fullstate-clock-classic240x160-image-OAM-dispatch-audio pair now
  pass18aggregateB(10header8command)/1program/0producer; other159programs/tables
  targeted-only. NewLinux-USA-nativeWindows/new32:9 scene staypending. [Blend consumption](../rom_data_graphics.md#managed-blend-programs-and-typed-guest-reads).
  OtherLUTs/unselected bank/sample/sequence work
  and newLinux-USA-nativeWindows stay open.

- [UI choice list source contract](../rom_data_ui.md) now bounds thirteen EUR
  ranges/42eight-byte entries/336B: twelve lists from16direct8BBF0callers plus
  one six-entry template for two liveRAMfilters. All18literal occurrences/
  93original instruction pins/50prior owners checked statically. RawID/X/Y/
  opaque/navigation bytes retain exact provenance, high-bit coordinate reads
  and signed full-word copy count versus low8live count; no host target/count
  cap. Filtered0200F250lists remain mutable; base-copy ordering preserves down1
  with zero selected slots. Strict lossless public decoder/shared source plan
  and one Wine source-reader oracle now pass17guards/all336original-loadsiteB/
  18copy prefixes/44getters/43idle/64livefilters/2unmasked raw-coordinate
  prefixes with703hash-mtime guards; warning-free Windows build. No fullwidget/
  native/fullstate-clock claim for that source oracle. Managed e56-d55now
  publishes52files51resources/all50prior decoded unchanged/e55retained, noasset
  dependency/fullsetcoherence. Two reduced Wine checks pass17guards/lifecycle/
  all336typedB/13lists and29original-interpreter-native complete-state-clock
  paths/compiled[92ID183coordinate92opaque368nav]aggregateB/widths-mirrors-
  BIOS-SWI-DMA/mixedoldOBJ-raw-new/706hash-mtime guards. No wholewidget ABI;
  actualCLIcreate/reuse52generation54managedfiles no-write/oldcache and one
  ordinaryParty320fullstate-clock-image-OAM-dispatch-all-audio pair nowpass
  32aggregateUIB=[4ID8coordinate4opaque16nav]/1list/769hash-mtime guards.
  Pecorelle1000/Moogle600are terminalnegative/no pairs; other12lists have
  ordinary coverage pending and retain targeted checks. Existing snapshot/input reused, no new fixture/
  navigation or oldproof repeat. USAe8-d7unchanged; new Linux/USA/native Windows checks pending.

- [Task descriptor source contract](../rom_data_tasks.md) covers341primary/
  20auxiliary/51script records,3296B/3spans disjoint41e47owners. Original main and
  secondary constructors use different ID formulas. Independent graphics root
  39a490and directory kind0/ID413reference bound the final descriptor39a488;
  nullableThumb/contextsize/rawmetadata remain distinct. Strict lossless model/
  one reduced Wine original-reader oracle pass all3296Breconstructed/3156Bobserved,
  aliases/rawindices/23guards;140suffixmetadataBpreserved-unread intheseprobes.
  No global live selector cap or complete task ABI is inferred. e48-d47nowpublishes
  43files42resources/all41prior decoded/e47retained, USAe8-d7. Two reduced Wine
  checks pass13guards/lifecycle/all3296typedB/412records3catalogs and29original-
  interpreter-native fullstate-clock prefixes/[44callback22context9metadata]API B,
  raw/aliases/oldblendowner/BIOS-SWI-DMA; no wholeABI claim. ProductionCLIcreate/
  reuse-no-write45managedfiles/oldcache andone1000Wine ordinaryPecorelle fullstate-
  clock-classic240x160-image-OAM-dispatch-allaudio pair pass28aggregateB/4records/
  2catalogs (16callback8context4metadata). Other408records/thirdcatalog targeted-
  only; new32:9/Linux-USA-nativeWindows pending. No fixture/navigation/payloadedit.

- [Effect/event source model](../rom_data_events.md) covers75event records,
  51selectors and28evenThumbsame-state dispatchwords:1516B/4spans disjoint42e48
  owners. Independent successor roots andevent compare74bound projections; no
  global live index guard or source count fromtaskcatalog size.70assetlinks use
  engine.task-descriptors. Strictmodel/oneWineoracle pass1106original-loadsiteB,
  410Bpreserved-unread/27guards/defaults/rawindices/wrap;92halfword8word fingerprints.
  Evententry124e1creachability unknown; no wholeinterpreter/ordinaryclaim.
  e49-d48nowpublishes44files43resources/all42prior decoded/e48retained, actual
  taskdescriptor dependency/USAe8-d7. Two reduced Wine checks pass13guards/
  lifecycle/all1516typedB and29original-interpreter-native complete-state-clock
  prefixes/[24event6selector24dispatch]API B, raw/defaults/evenstate/previousfont11B/
  samepagepalette4B/BIOS-SWI-DMA. ActualCLIcreate/reuse-no-write46managedfiles/
  oldcache passes. Pecorelle1000/schoolcoldboot3000/suspendboot-to-battle8476
  locators readzero newbytes, closednegative/no event-requiredfullpairs. New normal
  EUR progression reaches a genuine39420bedroom/book snapshot; one1800-frame
  Wine locator and fullstate-clock-image-OAM-dispatch-all-audio pair on actual
  e52generation pass5aggregateB=1selector+4dispatch/one selector/one table and
  470hash-mtime guards. Ordinaryeventrows0/other50selectors/other table remain
  targeted-only; entry124e1creachability/wholeABI/new32:9 unclaimed. Linux-USA-
  nativeWindows pending, no product rebuild/performance measurement.

- The [EUR script command model](../rom_data_scripts.md) covers 115 six-byte
  commands/690 bytes, independently bounded by a distinct byte lookup. Strict
  source authentication and one corrected Wine original-reader oracle pass all
  690 bytes, signed cursors, u16 advances, relative targets, Thumb BX and loop
  return branches. A test-only ADDS expectation was corrected; production/model
  code needed no correction. Managed e50-d49 nowpublishes45files44resources,
  all43prior decoded/e49 retained/USAe8-d7 unchanged, noassetdependency. Two new
  reduced Wine checks pass13guards/lifecycle/all690B and26original-interpreter-
  native complete-state-clock prefixes/[14advance28callback]API B, raw115/255/
  signedcursor/wrap/relative-target/BL-BX/returnloops/adjacentoldowners/rawgap.
  ActualCLIcreate/reuse-no-write47managedfiles/oldcache andone8476-frame Wine
  EURsuspendboot-to-battle fullstate-clock-image-OAM-dispatch-audio pair pass
  42B=14advance28callback/7commands. Other108commands targeted-only; new32:9/Linux-
  USA-nativeWindows pending. No whole script interpreter ABI/new opcode cap.

- The [EUR relative script archive source contract](../rom_data_scripts.md#relative-script-archive-and-event-key-source-contract)
  identifies115464B/two independently bounded ranges:234event keys,62topbanks/
  233slots,207physical command segments/35048stored instructions, explicit
  repeated/header/fill target views and45FFfillspans.493links use49existingeffect
  selectors; command/effect assets are actual dependencies. Static exact-byte/
  address/fingerprint/disjointness checks pass. Strict wholearchive model andone
  Wine source oracle nowpass42333original-loadsiteB/73131Bpreserved-unread/40guards,
  including header/fill views andrawIDs/q0/255. Managed e51-d50 now publishes
  46files/45resources/all44prior decoded/e50 retained, USAe8-d7 unchanged. Typed
  index/opcode/parameter/key/fill reads preserve prior owners/raw boundaries.
  Two reduced Wine checks pass13guards/lifecycle/all115464B and28original/native
  complete-state-clock prefixes. Actual CLI create/reuse-no-write48managedfiles/
  oldcache passes; retained8476-frame EUR locator reads27archiveB across one
  bank/program/key, zero fill. One Wine8476-frame pair passes every full state/
  clock, image/OAM/dispatch and mixed/PSG/FIFO/bitwise-HLE sample; other records
  and fills remain targeted-only.
  Five linked localized text archives now have a [source model and typed runtime](../rom_data_text.md#managed-script-text-and-direct-typed-reads):
  1803744B/five ranges,385banks/41010programs,63top entries/26live variants per
  language and147glyphs already in the complete font resource. Strict source
  model/Wine oracle passes all fields/hashes/45guards and837source-vs-typed
  expansion cases. e52-d51now publishes47files46resources/all45prior decoded/
  e51retained/USAe8-d7, with actual script-key/font dependencies. Shared text
  literal mapping and two new Wine checks pass13guards/lifecycle/allbytes and
  35original/interpreter/compiled full-state-clock prefixes; live variants,
  exact slack, mixed previous-owner/raw reads and original ABI remain. Actual
  CLI create/reuse-no-write49files/oldcache passes. Retained8476battle locator
  reads0new text; a distinct school3000locator reads289B across one language/
  bank/two programs. One Wine3000-frame pair passes every full state/clock/
  image/OAM/dispatch and mixed/PSG/FIFO/bitwise-HLE sample; other records/variants/
  slack retain targeted coverage. New Linux/USA/native Windows/new32:9
  checks remain pending, without new guest caps or performance measurements.

- The [EUR script-coordinate source/model](../rom_data_scripts.md#european-script-coordinate-paths-source-contract)
  covers3056B/210paths/856XYpairs/42alignment spans, bounded by independent roots
  D68FD4/D69BC4.68script producer links use56paths; command44low8/ignored byte3
  differs from command45u16. Live map height and raw guest arithmetic remain.
  One warning-free Windows build/single Wine oracle pass24guards/allsource fields/
  hashes,212original copies/all210paths/two changed live map cells and26raw/
  producer prefixes.2972sourceB observed/84padding Bpreserved-unread; no native/
  fullstate-clock/wholemovement-task/ordinary claim for that source oracle.
  Managed e53-d52now publishes48files47resources/all46prior decoded/e52retained/
  USAe8-d7. Shared typed index/count/XY/alignment owner preserves tripleD68text/
  key/coordinate and rawD69BC4, original height/control/save/native ABI. Reduced
  Wine managed14guards/lifecycle/all3056B and24original-interpreter-compiled
  fullstate-clock cases/four full copies/allfour classes pass; new-test array/
  observer fixes retained, successful proofs reused. ProductionCLIcreate/reuse-
  no-write50files/oldcache and one6000-frame genuine EUR pair pass every full
  state/clock/image/OAM/dispatch/audio with28B=4index14count10XY/one path,
  0alignment/581guards. Other209paths/42alignment targeted-only; new32:9/Linux-
  USA/nativeWindows pending, no performance/space measurement or new fixture.

- The [EUR auxiliary script archive contract](../rom_data_scripts.md#european-auxiliary-script-archive-source-contract)
  identifies a separate u16-relative archive atD69BC4 and17-row command directory
  at370F94:4200B/8banks442slots/133physical programs913instructions.30slot links
  enter interior instructions; root walks alone omit69instructions. All489
  relative targets are instruction boundaries; structural908/913coverage is
  not actual reachability. The4098Barchive projection endsD6ABC6 after its last
  linked terminator; graphicsbaseD6B4C8 is only a ceiling. FinalFF2/archive and
  zero2/directory gap bytes remain unowned. Static reconstruction/hashes/723
  halfword4word/all47e53owner disjointness pass; no guest cap/host interpreter/
  primary-archive dependency inferred. Strict typed directory/archive models
  now preserve interior instruction references/all physical storage; one final
  Wine source filter passes40guards/442lookup/913dispatch+RAMopcode3/80advance/
  489relative/10wrap/15raw/11wrapper probes.2887Bobserved/1313Bpreserved-unread;
  raw LDRH expectations corrected to aligned32-bit rotation, first failure and
  narrower successful retry retained; model/core unchanged. No predicate/whole
  interpreter/native/fullstate-clock/ordinary claim for that source oracle.
  Managed e54-d53now publishes both directory/archive resources:50files49resources/
  all47prior decoded/e53retained/USAe8-d7. Actual own-directory dependency and
  one shared typed4200Bowner preserve old370audio/D69coordinate owners/rawtails,
  liveguest/LDRH/save-nativeABI. Two new exact Wine filters firstPASS26guards/
  lifecycle/all4200B and44original-interpreter-compiled fullstate-clock prefixes/
  allsix classes[6,65,18,19,15,2]B/edges-widths-mirrors-BIOS-SWI-DMA. Whole predicate/
  interpreter/callback ABI and ordinary unclaimed for that targeted scope.
  ActualproductionCLIcreate/reuse-no-write52managedfiles/oldcache/all50generation
  equality and one6000-frame genuine EUR fullstate-clock-image-OAM-dispatch-audio
  pair nowpass with96directoryB=32advance64callback/6commands and82archiveB=
  16index20opcode46parameter/2banks2programs/0alignment;640guards. Other11commands/
  6banks131programs/5alignment spans targeted-only. NewLinux-USA-nativeWindows/
  32:9pending, no performance/space measurement or new gameplay fixture/navigation.

- The [normal EUR unit initial-pose source model](../rom_data_unit_obj.md)
  follows existing metadata52/14 producers through394834 and original
  22174/22254/221D4/00B14/00800. Its121IDs/477frames/sixOAMlists31objects/
  333raw4bpp requests135168B become4224indexed tiles, preserving opaque fields
  and original live scheduling/DMA.415spans146358B are disjoint from51e56owners.
  Source model/original Wine oracle is closed. EUR e57-d56 now publishes and
  consumes typed fields/decoded indices with metadata52/14dependencies:
  53files52resources/all51prior decoded/resource records/e56retained/USAe8-d7.
  Two new Wine filters pass17guards/lifecycle/all146358B/30original-interpreter-
  compiled complete-state-clock paths, rawID/animation/liveRAMframe and original
  DMA. All415edges/mirrors/BIOSbus/SWI-DMA/rawholes/oldOBJsamepage compose;
  source/model proofs remain closed. Actual productionCLIcreate/reuse-no-write
  55managedfiles/all53accepted generationfiles/oldcache and one320-frame existing
  Party-unit fullstate-clock-image-OAM-dispatch-allaudio pair now pass. New10962
  aggregateB include3520pixelB/2sequences; other119ordinary sequences targeted-only.
  Otherposes/parts/IDs/palettes and unresolved65535job0/1values
  are not covered. No guest cap/wholeunit-allocator/physical catalog follows.
  New Linux/USA/native Windows/new32:9 pending; no performance/space campaign.
- The [EUR equipment OBJ source model](../rom_data_unit_obj.md#equipment-poses-source-model)
  follows original item-property9/action overrides and part0/1 loops, separate
  from actor-body sprites.748physical cells/86null/258sequences3439frames map
  to593spans110666B, disjoint all53e58resource sources. Only operation1 words
  become OAM/pixel references; commands0/6/8 retain opaque words. One reduced
  Wine source oracle passes1496constructor-prefix aliases/86null+258sequence
  resets/sixcommand cases/280scheduler-DMA uploads/all37440pixelB/10guards.
  EUR e59-d58 now publishes55files54resources with actual item dependency,
  all53prior decoded/resource records/e58 retained/USAe8-d7. Two new Wine filters
  pass17guards/lifecycle/all110666typedB/26original-interpreter-compiled complete
  state-clock paths, commands0/6/8/nulls/liveRAMframe, all593edges/mirrors/BIOS/
  SWI-DMA/new-raw-oldowner composition. Source/model acceptance stays closed.
  ActualCLI create/reuse-no-write55generation/57managed files passes from
  unrelatedcwd/oldcache. Sheep1000/Fight700 equipment-negative locators are
  terminal/no fullpairs. One newly confirmed Polly→Hildegard copiedEURFight500
  ordinary fullpair now passes all states/clocks/central-wide/OAM/dispatch/audio,
  equipment4477aggregateB/1408pixelB/two sequences exactlyequal actuale61CLI.
  Explicit positiveuse guard prevents vacuous scene acceptance; other256
  equipment sequences retain targeted coverage. General body/alternate/script
  pairings remain open. User explicitly authorizes local
  ordinary-pad EURcheckpoint derivation from a copy on25October, preserving
  originals/cartridge saves and private outputs; previous review block cleared. No bounds or extents from adjacency.
  New Linux/USA/nativeWindows/new32:9 pending; no performance campaign.
- The [normal-mode EUR battle pose source model](../rom_data_unit_obj.md#normal-mode-battle-poses-source-projection-and-typed-model)
  extends the121normal producer IDs plus42 through29body/8extra-part bases:
  10980logical keys/8374cells/5293nulls/2581sequences17894frames,150OBJlists and
  3216indexed4bpp requests. Exact initial/equipment shared views retain146358/
  6984B prior ownership; new6021spans1455474B are disjoint other54owners. One
  Windows build/one reduced Wine sourcefilter firstPASS12guards/16474constructor
  prefixes/5293null+2451newsequence resets/100commands/2858scheduler-DMA/all
  1063200newpixelB/wholeVRAM. Prior original-reader acceptance is not repeated.
  The unrestricted alternate cross-product is rejected: sprite247/animation104
  reaches directory394834, so a frame-count filter or guessed bank size would
  misclassify data. Source/model child closed; EUR e60-d59now publishes/consumes
  typed sources with four actual metadata52/14/initial/equipment dependencies:
  56files55resources/all54old records/e59retained/USAe8unchanged. Two reduced Wine
  filters firstPASS17guards/lifecycle/all1455474newtypedB/2451newsequences and32
  original/interpreter/compiled complete-state-clock paths, parts0-1-2/null/raw
  IDs/liveRAMframe/commands0-2-3-6-7-8/two uploads+twoDMA/all6APIcategories. All
  6021edges/mirrors/realBIOSbus/SWI-DMA pass; mixedwindow2new1036raw316initial
  248equipment8oldOBJ B preserves exact charges; separate2new+2editedraw API
  case passes. ActualCLIcreate/no-write reuse56generation/58managedfiles andone
  existingF09Wait3530fullstate-clock-image-OAM-dispatch-allaudio pair now pass
  842236aggregateB/815360pixelB/fivesequences, exactlymatchingCLIusage. Other2446
  newsequence ordinary uses retain targeted coverage. Alternate/script/live-
  stream pairing and newLinux/USA/nativeWindows/32:9 remain pending; no metrics.
- The [alternate EUR battle source contract](../rom_data_unit_obj.md#alternate-battle-selectors-and-explicit-shared-target-views)
  now retains all65ID/5850logical queries as genuine nulls/counted sequences/
  explicit shared-index-root targets.20sprite247targets have independently
  typed initial-pose directory/index witnesses; they are not thousands-frame
  sequences. One4Bdirectory/index self-overlap is fully witnessed, no cap/pruning/
  guessed bank extent.706proper sequences2771frames/30OBJlists108objects/
  1203pixel requests give3321new spans410086B after three earlier-owner shares;
  all55e60owner source/overlap checks pass. The tagged source model and one new
  original Wine oracle now pass15guards/all5850bindings/10544constructorprefixes
  including40shared-target aliases/4428null+606proper resets/five newcommand
  cases/1079scheduler-DMA/all342112newpixelB/wholeVRAM. No fakeframes or oldoriginal
  replay. EUR e61-d60now publishes/consumes the3321new spans with five actual
  dependencies;57files56resources/all55prior records and e60retained/USAe8unchanged.
  Two reduced Wine filters firstPASS20managed guards/lifecycle/all410086typedB/
  606new sequences and34original/interpreter/compiled fullstate-clock paths,
  properpart0/realnullparts1-2/shared-root prefixes/commands2-3-8/two uploads+DMA/
  all3321edges/BIOS-SWI-DMA/mixedraw-oldowner composition. StrictNull emptystruct
  rejects extra fields; valid source JSON stays identical. Source/model/managed/
  native/actualCLI closed:57generation59managed no-write reuse matches accepted
  e61and oneframeoldcache attachment passes (zero newalternate use). Read-only81
  existing EURsnapshot checks find no current newalternate-task replay lead;
  positiveordinary/reachability remain open, no newnavigation/checkpoint.
  Linux/USA/nativeWindows/32:9 pending. No performance/space measures/gameplay checkpoints.
- [Script49 actor selection](../rom_data_unit_obj.md#script-animation-selectors-require-live-actor-bindings)
  is now classified as live lookup, not an immediate sprite operand. Decimal49
  (0x31)/126CC0has167instructions:151low selectors scan live units,16high selectors
  use registry/fallback, then actor lookup matches unit pointers/current sprite
  +34. Group×4low16/facing signed+1F/body flag0and samebase extra parts are original.
  Nearby31can reuse a unit without changing its job field;32only repositions.
  Static script×all-sprite projection remains unjustified. Existing scripts/
  67instruction/nine literal/eighteen BL pins pass without guest/build work;
  one new test-only Wine6000functional census now links six49sources/selector16/
  group2to sprite46/animation8/parts0-1-2(eighteen queries). Body is already owned;
  two unowned null cells37C760/37C764add8Bbut no new sequence/pixels. Original
  missing-entry steps/native constructor are preserved; first all-native
  preflight failed before frames, corrected build/actual census pass. Stable
  null-cell source contract and other161instructions remain open, no fixedsprite
  promise for selector16. e61/USAe8unchanged; Linux-USA-nativeWindows-new32:9
  pending; no metrics.
- [Conditional normal script-special source pairs](../rom_data_unit_obj.md#conditional-normal-script-poses-for-special-unit-selectors)
  now join151low-selector49instructions to exact logical metadata14normal fields,
  under same-unit current normal assignment/override20clear/facing0..3.20producer
  pairs/14logicalIDs→18sprite-base pairs/108queries76nulls/32sequences241frames;
  no arbitrary cross-product, pruned query, role/extent/mixed-item conflict.
  All56e61owners share4780initial+30586normalB; new33spans6436B include60cells53
  nulls/sevensequences46op1frames/17pixel requests5248B. Sourcecontract/27halfword/
  one literal/three BL pins closed. Typed model/151refs20witnesses and single new
  sourceWine oracle pass18guards/120constructor aliases/53null7proper resets/
  17realDMAuploads/all5248newpixelB; overlapping initial/normal dependency views
  independently validated. Managed/runtime now EUR e62-d61/58files57resources,
  actual metadata14/scriptarchive/initial/normal dependencies/all56prior records/
  retained e61/USAe8. Two new Winefilters firstPASS29guards/lifecycle/all6436typedB
  and23original-interpreter-compiled fullstate-clock paths/33edges/mirrors/BIOS/
  SWI-DMA/mixed2new4normal and2new2raw; original tasks/commands remain guest work.
  Newdirectory/OBJ bytes are zero, sharedowners retained. ActualproductionCLI
  create/reuse58generation60managed/all58matchaccepted passes, canonical exact
  restored; oneframeoldcacheF09attachment haszero newusage/smokeonly. Newactual
  Snow6000locator observes48typednullindexB/no newpixels; RIVERstart618zero newuse.
  Bothclosed/no fullpair/repeat;81new-sequence snapshot reads find no tasklead.
  Ordinarynewsequence/pixel/equivalence and alternate/override/high16/current-state
  cases pending;
  newLinux/nativeWindows/USA/32:9 remain pending, no performance work.
- [Conditional alternate script-special poses](../rom_data_unit_obj.md#conditional-alternate-script-special-source-and-typed-model)
  now have an exact151reference/20logical-witness source projection and strict
  offline model:84keys68null/14sequences64frames,20new spans816B over three
  prior owners. One focused Wine source oracle passes19guards/108newcell
  constructor aliases/52null+2proper resets/two actualDMAuploads512pixelB with
  wholeVRAM equality. EUR e63-d62now publishes/consumes59files58resources with
  five actual dependencies/all57prior records+data/e62retained/USAe8unchanged.
  Two new Winefilters pass31guards/lifecycle/all816typedB and24original-
  interpreter-compiled complete-state-clock paths/20edges/BIOS-SWI-DMA/
  mixed2new4alternate+2new2raw; oldowners unchanged, no task/save/nativeABI
  rewrite. ActualproductionCLIcreate/reuse59generation61managed/all59accepted
  files/no-write andoneframeoldcacheattachment pass(zero newuse/smokeonly).
  Read-only81snapshot search for exactly two new sequences finds no matching
  enabled/dormant task; locator closed/no guest/fullpair/retry. Ordinarynewpixel/
  equivalence remains pending; high16/42-230/currentactor and newLinux/
  nativeWindows/32:9 remain separate. No performance/storage measurement.
- [Forced42/230script pose sources](../rom_data_unit_obj.md#conditional-forced42-and230script-pose-source-model)
  now project167exact refs/13groups through original flag20forced current
  normal42/alternate230assignments under explicit retained actor conditions.
  26mode-base/334conditional witnesses/156keys126nulls/28sequences140frames;
  new40spans1050B over oldnormal/alternate views. Strict offline model and one
  new Winefilter pass19guards/192newcell aliases/90null+6proper resets/bothnew
  operation2argument copies/two actualDMAuploads256pixelB/wholeVRAM equality.
  EUR e64-d63managed/runtime now passes60files59resources/threeactual deps/
  all58prior data+records/e63retained/USAe8unchanged. Two newWinefilters pass
  31guards/lifecycle/all1050typedB and27original-interpreter-compiled fullstate-
  clock paths/bothnewop2/newOBJ/40edges/BIOS-SWI-DMA/mixedoldowners/raw.
  ActualproductionCLIcreate/reuse60generation62managed/all60accepted/no-write/
  oneframeoldcacheattach pass(zero newuse/smokeonly); ordinarynewpixel/equivalence
  remain pending, no task/save/nativeABI rewrite. New read-only81snapshot scan
  for allsixnewoverride sequences finds no enabled/dormant task; locatorclosed,
  no guest/build/fullpair/retry.
  This does not close highregistry/liveactor identity; no metrics/newfixtures,
  newLinux/USA/nativeWindows/32:9 pending.
- [Registry-knownspecial script source projection](../rom_data_unit_obj.md#registry-selectors-and-known-special-unit-source-projection)
  authenticates16high refs/seven pairs/93knownmetadata14rows under same-unit
  retainednormal-or-alternateassignment/override20clear, before targets; no fixed
  registry identity.1302witnesses/2220keys1908cells/373842sourceB share358566B
  across six priorowners; new272spans15276B include598cells567nulls/21seq96op1
  frames/37pixelrequests10880B. All59e64owners/41halfword4literal8BLpins pass,
  no root-target/role/extent/mixed-item conflict. Strictlossless model and one
  focused Wineoracle pass26guards/1196newcell facingaliases/567null+21proper
  resets/37actualDMAuploads/all10880newpixelB/wholeVRAM; six previous typed
  views validated independently, physical aliases coalesced exactly. Corrected
  frontend build warning-free. EUR e65-d64nowpublishes61files60resources with
  actual metadata14/archive+sixpreviousview dependencies. Two focusedWinechecks
  firstPASS33attach+10dependency guards/lifecycle/all15276typedB21seq and25
  original-interpreter-compiled fullstate-clock paths/272edges/BIOS-SWI-DMA/
  mixedold-raw-new. Six previous owners retain sharedbytes. ActualproductionCLI
  create/reuse61generation63managed/all61accepted/no-write/oneframeoldcache
  attach firstPASS(allnewusage0/smokeonly); ordinarynewpixel/equivalence
  pending. New read-only81snapshot/21newsequence locator finds no enabled or
  dormant task, closed/no guest-build-fixture-fullpair or unchangedretry.
  USAe8unchanged; genericjob/outofcatalog/redirect/currentchanged
  source cases remain separate, no metrics/newfixtures or newotherplatform tests.
- [Registry generic direct-job source projection](../rom_data_unit_obj.md#registry-selectors-and-generic-direct-job-source-projection)
  authenticates high16refs/sevenpairs, nonnullunit+4=0/1/currentjob+5/alternate+7/
  retainedsameunit originalassignment/override20clear before targets. All116rows
  classified:110normal100alternate concrete/2normal12alternateFFFF/4routeFF;
  2940witnesses6720refs,83halfword6literal-dispatch11BLpins. All60e65owners checked
  before targets.107sprites535pairs3210cells2420null/777properseq5093frames and12
  explicit initialindexroot48..59views. Full796992B shares697168B withsixowners;
  new659spans99824B include1386cells1260null/126seq588op1/268pixelrequests82016B.
  No mixeditem/role/extentconflict; initialjoint110count corrected beforetargets,
  no hidden sentinel/redirect pruning or u16FFFFfallback. Strictlosslessmodel
  and firstfocusedWineoracle pass34guards/2772newcell aliases/1260null126proper
  resets/268DMA82016pixelB/wholeVRAM/all12initialroot views; sixpriorviews checked
  independently. EUR e66-d65nowpublishes62files61resources withactual metadata52/
  archive+sixpreviousview dependencies. TwofocusedWinefilters firstPASS36attach+
  10dependencyguards/lifecycle/all99824typedB126newseq and25original-interpreter-
  compiled fullstate-clock paths/659edges/BIOS-SWI-DMA/mixedold-raw-new. Six
  previous owners retain sharedbytes. ActualproductionCLIcreate/reuse62generation
  64managed/all62accepted/no-write/oneframeoldcacheattach firstPASS(allnewusage0/
  smokeonly); ordinarynewpixel/equivalence pending. New read-only81snapshot/
  126newsequence locator finds0enabled/dormant bodytasks without currentpointer
  filtering, closed/no guest-build-fixture-fullpair or unchangedretry. USAe8
  unchanged; no previous acceptance replay,
  fixture/navigation/metrics or newotherplatform tests; globalregistryidentity/
  outofcatalog/nonreturningredirect/currentchanged remain separate. The
  [known alternate-job redirect contract](../rom_data_unit_obj.md#registry-generic-jobs-redirected-to-a-known-alternate-job)
  closes currentjobs80/81/82/85routeFF to concrete known alternate rows:
  11760conditional witnesses26880refs map exactly four-to-one onto all2940direct
  witnesses. All116rows/928effective classifications retained;98halfword6literal
  11BLpins and all61current resource/source hashes pass before targets. Same
  3210cells/777sequences/full796992B/sevenowners, zero newbytes/resources/revision;
  existing metadata52ande66Wine proofs reused without build/guest replay.
  The [known FFFFreader contract](../rom_data_unit_obj.md#known-generic-job-ffff-results-and-raw-bus-reads)
  classifies196direct+784redirected witnesses/2240refs: actorproperty083B4C12=DC86,
  directory083D4830=14AE3F1E;60facingqueries/30words use unmapped14bus, with live
  bios_open/CPUrotation rather than a proved ROMsequence/null result.71readerpins/
  threecodeblocks/twoliterals pass. Both ROMreads total6Bcurrently unowned/raw;
  their migration and ordinaryreachability remainpending, no cap/pointerfix.
- [Actor property words](../rom_data_unit_obj.md#actor-property-words-from-known-sprite-assignments)
  have a separate source contract:221C4reads394C14+2*sprite, unsignedu16 through
  actor9AD0E/r8/constructor r3into allocation/task+16.398knownnormal/alternate
  job-special-override witnesses derive182words before targets;98halfword15BL/
  fourcodeblocks/oneliteral andall61currentowner hashes pass. Fourunownedranges/
  364B/sourceSHAad328c8e.. retain139words16/43words24 andrawholes, no248extent/livecap
  or flag/VRAM-unit interpretation. Strictlosslessmodel andfirstWineoracle now
  pass20guards/all182originalgetters364B/3rawfullwordcases/fournewtaskprefixes/
  tworeuseprefixes/fourisolatedsetters/wholeRAMoutside declaredscratch-destinations.
  Existingtask keeps its own+16word; newtask receives newproperty. Windowsfrontend
  firstbuildwarning-free/82build83Wineguards11compiledinputs exact. No fullallocator/
  native-clock/ordinary claim for02. EUR e67-d66nowpublishes63files62resources with
  actualmetadata52/14dependencies/all61priorfiles-records/e66retained. Twofirst
  exactWinefilters pass17attach+4fullsetdependencyguards/lifecycle/364typedB/
  182recordmask and16original-interpreter-compiled fullstate-clock paths with
  all364rawB altered. Fourrangeedges/widths-mirrors/realBIOSbus/SWI-DMA/mixed
  new-old-raw pass;84build86Wineguards20compiledinputs exact. ActualproductionCLI
  create/reuse63generation65managed/all63accepted/no-write/oneframeoldcacheattach
  firstPASS(allnewusage0/smokeonly),154activeCLIguards exact. One newordinary
  F09Wait3530CLI/fullpair nowfirstPASS10reads/20aggregateB/twowords; everyframe
  fullstate-clock-central-wide-OAM-dispatch-allaudio equal, remaining180words
  targetedonly.155activeguards/21compiledinputs exact; optionalnewpositiveguard,
  oldnormal-scenechild remainsclosed/no otherproof replay. No fullallocator or
  new32:9claim. Ordinarydeclaredsceneclosed,
  USAe8unchanged. No oldproof replay/gameplayfixture/navigation/performance measurement;
  FFFFraw6B/globalidentity/changedactor separate.
- The [twenty-slot job lookup source](../rom_data_gameplay.md#twenty-slot-job-lookup-source-contract)
  now has a bounded EUR contract:20u8jobrefs/20B at399750, twenty distinct existing
  metadata52rows, disjoint64currentowners. Original12CA98live20slot selection,
  preservedr6 across CFF70clear andsixlookupwrappers prove the conditional domain
  before data inspection;268halfwords/9literals/8BLs authenticated. No whole
  physicaltable/globalinputcap/gameplayreachability or sprite target is inferred.
  Sourcechild closed. Strictlossless20entrymodel andone firstWinefilter nowpass
  independent20B/12guards/123originalpaths (21bare/81fullproperty/21nameIDprefix),
  threehighinputaliases/all20originalLDRBsiteB/callee-fullRAMoutside20Bstack/
  VRAM-OAM-palette; firstWindowsfrontendbuild warning-free. Onlytwo moduledecls,
  The model phase left e70-d69/65files64resources/USA8-7 unchanged. Managed
  e71-d70now publishes66files65resources/all64oldrecords-files/e70retained, actual
  metadata52dependency/immutable20Bowner/page399/20bit mask. Two Winefilters after externalENOSPC/tmp recovery/
  one warning-freeWindowscore/frontend build pass18attachguards/3dependencyguards/
  unavailable lifecycle/all20rawB altered and16original-interpreter-native full
  state-clock paths, including12CA98mode3slot19→clear/nameIDprefix/rawoutside/
  boundaries-SWI-DMA. No globalguestcap introduced. ActualproductionCLI nowpasses
  create/reuse-no-write/oldcache-F09oneframe:66generation68managed65resources/
  all66accepted, canonicalhash-mtime exactrestored; privateTMPbacking avoids03
  actualENOSPC withoutnewcleanup/buildretry. DeclaredBank120ordinary nowpasses
  firstnewCLIpositive andoneWinefullpair120state-clock/capturedcentral-wide/OAM/
  dispatch/allaudio,2jobB/1slot equalCLI; other19ordinaryslots targeted-only.
  Only5lineoptionalnewguard/report, firstWindowsfrontendbuild warning-free;
  rawCLI284x160Bankbackground has no UIpanels, no composedGUI/fullABI claim.
  Source/model/runtime/CLI/declaredordinary children closed; no fixture/navigation/
  measurements or oldmetadata52/FFFF/A4proof replay.

- The [bounded native ROM API residue test](../rom_data_runtime.md#bounded-native-rom-api-residue-inventory)
  wraps a test-only BlockTable/read8-16-32 API, leaving production dispatch/API
  unchanged. First EUR Bank12 Wine filter/one warning-freeWindows frontend build
  preserve every fullstate-clock/buffer/OAM/dispatch/audio comparison and73actual
  input guards.3166context reads/3683imageB include676unownedB/79observedranges;
  these are source leads, not asset boundaries/completeness. Interpreter/HLE/DMA/
  native constants/other scenes remain outside this observer. Its five highwords
  are now staticallyclassified: two bounded4entry DMAregister directories and
  one ThumbIWRAM branch pointer viaBXr2;392halfwords/47pins/5literals/4BL exact.
  Lower656B now have static original Thumb witnesses:158literalwords/184linear
  context paths (507unique halfwords) and6words from5guarded switchtables
  (82prefix halfwords). No exact dynamic instruction/mode attribution or all-use
  claim. All676stilloutside owners; originalROM/code support dependency explicit.
  No RAMbody/ABI/asset or whole-game completeness inference.
  Samefrozenobserver nowpasses one WineF0912filter/fullstate-clock/buffers/OAM/
  dispatch/audio, no rebuild or oldscene acceptance replay.13671contextreads/
  13961imageB include604unownedB,584sharedwithBank+20newB/fivePCrelative static
  witnesses.76actualinputguards exact. No newasset/revision/completeness claim.
  No fixture or performance campaign; e71/USA8-7 unchanged, platforms pending.

- The [optional bus/host ROM audit](../rom_data_runtime.md#optional-bus-and-host-rom-read-audit)
  Cargo feature is absent by default. Scoped TLS collects successful bus and
  selected audio/sprite/table/keyboard source requests, suppressing interpreter
  instruction fetch only; no Machine/API/save schema fields. One WineF0912filter
  passes fetch/width/mirror/lifetime probes and every fullstate-clock/buffer/OAM/
  dispatch/audio comparison after a warning-free build retry for a test import.
  23061unique requests/19984imageB include1146unownedB,542beyond prior nativeAPI
  sample;90new highbytes are source leads only.76actualinput+onecomparison guards
  exact, oldacceptances not replayed. Its90newhighB nowhave sourcecontracts:
  two ordinaryindirectOBJlists64B/10objects,10parameterB and16IWRAMcontrolB.
  One newproducerargumentWineframe plus295windowhalfwords/53pins/7literals/
  reusedABIhashes identifiesactualreaders. No assumedwhole table/cap/allocator/
  commandproducer ABI. Two-list isolatedtypedmodel nowpasses one Winefilter:
  64B/10objects/14guards/sixoriginalROM-RAMreader paths, wholeoutput/matrix/callee/
  RAMguards exact. One warning-freeWindowsfrontendbuild;75selectedguards exact,
  oldCatalog74/e71unchanged during that isolated model milestone. Managed/runtime
  integration now passes EUR e72-d71:76lists/185objects1262B,66files65resources,
  126direct refs/all74oldlists/all64other records-files/dimension dependency/e71
  retained. One warning-free Windows frontend build and two first Wine filters
  pass12attachguards/publication/lifecycle, six original/interpreter/native
  fullstate-clock paths with64rawB altered, sixSWI-DMA/sixhost checks and mixed
  old-new/raw-new API reads.77selected guards/compiled code exact; required
  artifact completion retires only disposable copies. Actual productionCLI now
  firstPASS create/no-write reuse66generation68managed/all66accepted plus existing
  F09oneframe/cache/input16:9; bothnew sources guest-host positive.143CLIguards/
  implementation/canonicalhash-mtime exact, one warning-freeproductionbuild.
  Declaredordinary firstF09frame8476→8477 now passes one Winefullpair withboth
  newguest-host sources required: fullstate-clock-central/wide-OAM-dispatch-all
  audio, actualsamples[2196,2196,264,264,2120], exactCLIaggregate147B/12hostreads.
  Five optionalvalidator lines only; firstfrontendbuild warning-free,75guards
  exact. Broaderordinary remains separate. The10parameterB now have an isolated
  observed_task_parameters model:2phase1/2u8thresholds/fourindices4..7signed16
  corrections,27guards/18originalprefix cases in one firstWinefilter after a
  warning-freeWindowsfrontendbuild. Oldcounter compare/phasewrap/lowbitalias/
  signedsubtraction/RAM-callee guards exact;79selectedinputs exact. Phase0/
  correctionpriorowner/outsidequeries stayoutside model, no globalguest cap or
  units inferred. Model milestone left productione72unchanged. Managed/runtime
  nowfirstPASS e73-d72/gameplay.observed-task-parameters:67files66resources/all65
  oldfiles-records/e72/76OBJlists retained, one10B/twospan file withindependent
  fields/nodeps. Two Wine filters pass13attach/3fullset guards/lifecycle and24
  original/interpreter/native fullstate-clock prefixes with10rawB altered,
  outsidequeries/edges-mirrors-BIOS/sixSWI-DMA/mixednew-old-raw.26compiledB/
  316instructions24dispatch arecoverageonly;74guards/compiledinputs exact.
  Firstcorebuildcompiled; two frontendtest-call missingargs fixed, incremental
  buildwarning-free. Requiredartifactcompletion retiresonlydisposable copies.
  ActualparameterCLI nowpasses firstproductionbuild/canonicalrestore, prepare/
  no-write reuse67generation69managed/all67accepted andoldcacheattachment. One
  user-interruptedprepare archived/retriedsamebuild; firstframe1thresholdB but
  correction0 isnegative. New12frameCLI reaches20reads28B/12threshold16correction/
  2phases4rows;145guards/code/canonical/userAtlas exact. Ordinary12framefullstate
  nowpasses one Winefullpair8476→8488/all6rows: everyfullstate-clock/central-wide/
  OAM/dispatch/allaudio, samples[26334,26334,3168,3168,26258], exactCLI20reads28B.
  Five optionalvalidator lines only, firstWindowsfrontendbuild warning-free/
  76guards/code/originals/userAtlas exact. Allbounded parameterchildren closed;
  broaderordinary separate. The lower452B are now source-classified by an
  opt-in filtered execution-site capture:113words/114reads,101literal words404B
  from102executed Thumb LDRs and12switch words48B. Original opcode/address/width
  checks distinguish actual operands from inherited HLE/DMA scope; cached and
  uncached probes retain old address-only keys. One new Wine source-only filter
  observes all selected words in a single current F09 decoded12frame arm;
  no old equivalence replay.85guards/source/defaultcfg preimages/Atlas exact.
  These bytes remain code support, no new resource/revision/asset extent.
  Pointed394FBC now has a separate conditional constructor-animation source/model:
  four8B threshold/list-pointer pairs32B plus four14B/two-object lists56B/eight
  objects. The original u8counter increment/store/reload precedes unsigned-word
  compare;255wraps0, newphase>3 clearsactivebit. Oldphase has no prelookup cap;
  model0..3 only. One firstWindowsbuild/oneWinefilter passes13guards/15counter/
  4constructor/4new-list copied-reader cases,94inputs/source/preimages exact.
  The isolated milestone left e73 unchanged. Managed/runtime now passes e74-d73:
  68files67resources,32B table withOBJdependency and4lists56B merged into existing
  owner,80lists193objects1318B/all76oldlists/65otherrecords-files retained. First
  Wine passes10attach/2halfset/publication/lifecycle/13fullstate-clock prefixes/
  native36typedB/all4phases/4outside/708buscases, then testsetupROMidentity fails
  beforeDMA. Restore-before-load correction is testonly; remainingfilter passes
  6SWI-DMA/4hostpublications/twoobjects each without replaying completed checks.
  Production byteidentical across observations;169guards/source/Atlas exact.
  ActualCLI/ordinarypositive pending (retainedF09 zero matchingdatareads), no
  firstfullfilterPASS claim. Pointed371064phase0/371034 separate source contracts.
  [Constructor animation](../rom_data_obj.md#conditional-constructor-obj-animation-source-and-model).
  [Source proof](../rom_data_runtime.md#lower-bus-and-host-residue-source-contracts).
  USA8-7 unchanged; Linux/nativeWindows/new32:9 pending, no newfixture or
  measurements. Other threads/scenes/readers/typed HLE bypasses remain separate.

- The [following live-slot helpers](../rom_data_gameplay.md#live-state-helpers-following-the-slot-job-lookup)
  D05E8/D0618test/count nonzero firstwords in20RAMslots at02002E78, whileD0650
  increments livebyte02001F68onlywhenold<=98 (largervaluesunchanged). Exactthree
  bodies/68halfwords/30criticalpins/6literals authenticateRAMstate, no newimmutable
  asset/root/resource/revision or global20inputcap. Raw3/2/1caller candidates are
  not whole-game/liveness proof; partialD0670separate. Sourcechildclosedstatic-only,
  e71/USA8-7 unchanged, no newbuild/game/test/fixture/navigation/measurements.

- The [known FFFF numeric reader fields](../rom_data_unit_obj.md#typed-model-for-known-ffff-reader-fields)
  now have a strict lossless model: u16property/u32encoded directory/query65535,
  two spans6B and980conditionalmetadata52/archive49witnesses2240refs. Existing
  property/byte-alias/valid-sequence models remain unchanged; no asset target,
  pointer fix or index cap is inferred. FirstWindowsfrontendbuild warning-free/
  frozen83ffcd75.. and one firstWinefilter pass13guards/3numericroundtrips/
  independent980records+6B/twooriginalreaders28instructions/callee-fullRAMguards.
  156build157Wineguards35compiledinputs exact, all63current owners disjoint.
  Modelclosed. [Managed publication/typed bus](../rom_data_unit_obj.md#managed-ffff-reader-fields-and-typed-bus-consumption)
  now passes e70-d69/65files64resources/all63oldrecords-files/e69retained, actual
  metadata52/archive dependencies. Two first Wine filters/one warning-freeWindows
  core/frontend build pass20attachguards/4dependencyguards/lifecycle/unavailable
  directory/all6Bpoisoned and14original-interpreter-native fullstate-clock paths,
  liveBIOSunmapped14/rotate16/zeroearlyreturn/boundaries-SWI-DMA. Only two numeric
  fields are owned; no valid sprite target is inferred. ActualproductionCLI
  create/reuse-no-write/oldcache-oneframe now passes65generation67managed/
  64resources/all65accepted; canonicalhash-mtime and originalF09/cache exact.
  Ordinarypositive/reachability/equivalence remain pending. Newreadonly81snapshot
  locator finds159potentialFFFFcontexts/61snapshots (allkind0/job0/alt0),157null
  bodypointers/twooutsideEWRAM/no livebodytask lead. It infers neither actorvalidity
  nor globalunreachability; no guest/build/fixture replay. USAe8-d7 unchanged,
  unmapped0x14 live bus semantics retained. Closed71pin/60query/A4/oldoracles/scenes notreplayed; no
  fixture/navigation/performance-space work. Otherplatforms remain pending.
- The [additional property getter callers](../rom_data_unit_obj.md#additional-property-getter-callers-and-the-ui-byte-input-window)
  have a source contract for all five retained direct calls. Two additional u16
  paths update the task property while retaining allocation; their known concrete
  assignments alias the existing182words. Two UI callers use a byte input to
  select a descriptor. Their exact256-query width window shares182property words
  and two palette-offset words, leaving72newwords/144B in four disjoint ranges.
  All62current owners authenticate; no256catalog extent, livegettercap or universal
  ordinary reachability is inferred. The new lossless72wordmodel and firstWine
  filter pass14guards/512UIprefixes/all512readB/eightskips/fouru16comparisons/
  fourbytewriters/sixadditionalsetterprefixes/wholeRAMoutside destinations plus
  VRAM/OAM/palette guards.109build110Wineguards/13compiledinputs exact; no fullUI/
  allocator/native-clock/ordinary claim for02. EUR e68-d67 now publishes64files/
  63resources/all62oldrecords63files/e67retained withactualpropertyword+palette
  dependencies. Newimmutableowner servesonly72words144B/fourspans/ordinalbitmap;
  184sharedwords keeppriorowners. FirsttwoWine pass17attach+fourdependencyguards/
  lifecycle/all144typedB72records plus original-interpreter-compiled fullstate-
  clock getter/UIprefix/fullword/liveRAM/mixed paths. Fourrangeedges/width-mirrors/
  realBIOSbus/SWI-DMA pass; API2new2property/2new2palette/2new2raw, CpuSet2new4property/
  FastSet28new4property/DMA4new2palette. ActualproductionCLI firstprepare/reuse
  nowpasses64generation66managedfiles/63resources/twodeps/all64accepted/no-write;
  authenticF09oneframeoldcache attachesanduses1read2aggregateB/oneword, smokeonly/
  nofullsceneequivalence.182activeguards/23compiledinputs exact; canonicalrestored.
  NewordinaryF09Wait3530fullpair nowfirstPASS 476reads/952aggregateB/4words;
  everyframe fullstate-clock-central-wide-OAM-dispatch-allaudio equal, allfiveaudio
  streams nonzero. Other68words targetedreaderonly; 183guards/24compiledinputs
  exact. Optionalfive-line positiveguard only, no duplicateCLI3530 or oldscene/
  source/model/runtime replay. Declaredordinarychild closed; broadercoverage,
  fullUI/allocator andotherplatforms separate. USAe8-d7/FFFFraw6B unchanged. No fixture/navigation/performance measurement or old
  acceptance replay. Unknown u16, changed actors and computed
  indirect calls remain open.
- The [property-selected static UI OBJ lists](../rom_data_obj.md#managed-ui-property-list-extension-and-typed-consumption)
  now extend the existing catalog in EUR e69-d68:126refs/74lists175objects1198B.
  Four conditional A4/B2 roots flow through1F6Cto1500; source proof is closed.
  A4adds one14B/two-object list; B2retains its existing typed owner. Same64files/
  63resources/format/dimension dependency/runtime handlers; all73oldtypedlists,
  62otherfiles/records ande68 retained. Two first Wine filters pass model/source
  roundtrip/eight negative guards/publication/repair/move/lifecycle plus two
  original-interpreter-compiled draws/fullstate-clock/709instructions/4dispatches/
  36compiledB/twohostpublication checks. Bothboundaries/widths/mirrors/realBIOSbus/
  mixedAPI2new2B2/DMA-CpuSet14new2B2/FastSet4prior14new14B2 pass. CorrectedWindows
  build warning-free; first failure was test-only constant qualification.
  77build79Wineguards29compiledinputs exact. ActualproductionCLI firstprepare/
  no-write reuse64generation66managed/all64accepted/63resources/74sources and
  authenticF09oneframeoldcacheattach pass; 148activeCLIguards/29inputs exact.
  Declaredordinary05nowclosed: firstDispatch1840actualCLI A4guest-hostpositive/
  15evaluationB; firstexactWinepair passes all1840fullstate-clock-central-wide-
  OAM-dispatch-allaudio, all5streams actualsamples. Five-lineoptionalA4gateonly,
  firstWindowsfrontendbuild warning-free/frozen427845f2../152guards29inputs exact.
  New25halfword/sixswitchword join6375C/D2D44→1092→063230/02AF14→animation24→A4
  explainsordinaryroute. F09/Party/bank andearlyDispatch1400negative stayclosed;
  1400missed1600input, original1840positiveevaluationproof justifiedcorrectedbound.
  Sourcecheckpoint/input hashes-mtimes exact/authorizedcopyonly/no cartedit.
  No oldsource/model/CFG/scene replay, performance/storage or fullUI/globalclaim.
  USAe8-d7/Linux/nativeWindows/new32:9 and broader indirect sources remain open.
- The [EUR Color-Mode unit/UI palette model](../rom_data_color_palettes.md)
  follows five original CF49C..CF5DC helpers with70direct callers,40constant
  supplemental calls/four slice callers and51metadata14special projections.
  Its30spans5750B=2873u16colors+fourselectors are disjoint all52e57owners.
  Strict lossless model and corrected reduced Wine selector/copy-prefix oracle
  pass all5750actual-read bytes:165pointer cases/99copies/30staged readers/
  54special copies/12slice prefixes/eightguards. Live low2modes/low8arguments,
  companion mode1alias, source gaps and guest RAM remain intact. The first
  oracle failed from a destination-register seed assumption, corrected using
  original MOV/LDR decoding; no model difference. EUR e58-d57 now publishes and
  consumes typed colors/selectors with metadata14dependency:54files53resources/
  all52prior decoded/resource records/e57retained/USAe8-d7. Two new Wine filters
  pass17guards/lifecycle/all5750B/28original-interpreter-compiled complete-state-
  clock paths, low8index/mode3/liveRAMpalette/30edges/mirrors/BIOS-SWI-DMA and
  mixed2new+20raw+4oldUIB/exact2/4charges. Source/model/oldproofs stay closed;
  actualCLIcreate/no-write reuse56managed/all54generationfiles and one positive
  ordinaryLutia1700 fullstate-clock-image-OAM-dispatch-allaudio pair now pass.
  New1102aggregatecolorB=[512unit512companion78supplemental]/fourfragments;
  Party320/boot240negative locators stay closed/no fullpairs. Other26ordinary
  fragments/othermodes/shared-special-selector/wholeallocator-scheduler and
  newLinux-USA-nativeWindows/32:9 remain pending; no performance/space campaign.
- The [normal EUR terrain sources](../rom_data_maps.md) already belong to the
  162 BG descriptors and352packet programs: original1B688/state6→1AE88 routes
  all486field04/08/10links to existing typed owners. Field10initializes the
  normal16×16height/flag grid02007CB0; live context02007F10+4/+8/+9 and mutable
  flags remain guest state. Static link/source/49owner checks pass without a
  new decoder, revision or guest run. Other context layouts/ten-byte1DFB4caller
  remain unknown. Separate raster palette bank4872B0..489BB0now has a verified
  static32record/324link/33range/10496Bcontract, with160u16colors per record.
  Original161halfword copy crosses into the next header or the existing
  alternate-palette owner, then wraps the extra destination word. Preserve
  this read/timing boundary. Strict lossless model/one Wine original-reader
  oracle now pass all10496B/324links/15guards and36complete322Bcopies including
  the neighboring013Cword before wrap; unsigned/signed/raw/sentinel/deferred/
  synthetic bit15/range/queuedDMA cases pass with component RAM/SP/callee checks.
  Managed e55-d54now publishes51files50resources,
  all49prior decoded/e54retained, actualBGdependency and one immutable owner.
  Two reduced Wine filters pass17guards/lifecycle/all10496B and28original-
  interpreter-native fullstate-clock paths, compiled[15index46header24color]B,
  original copies/wrap/queuedDMA/mixedoldowner/rawfallback/BIOSbus-SWI-DMA.
  ActualCLIcreate/reuse-no-write53managedfiles/all51generation equality/oldcache
  and one1500frame genuine coldboot Winefullstate-clock-image-OAM-dispatch-audio
  pair nowpass50880aggregateB=[2index322header50556color],701guards. Programmask2
  includes neighbor-header touches, not two full palettes/distinctB. Other
  contexts/whole camera-rasterABI/newLinux-USA-nativeWindows/new32:9 remain
  unclaimed/pending, USAe8-d7unchanged.

- The root Cargo workspace and `AndroidPort/` are separate trees. A root fix
  does not automatically update AndroidPort.
- `crates/recomp/src/play.rs` connects the desktop event loop, input, menu and
  presentation. `crates/menu` owns UI state and rendering; `crates/input-config`
  owns persisted bindings and combo capture/latching. `crates/screen` presents
  the framebuffer and UI layers.
- `crates/gba-core` implements the emulated machine. `crates/recomp-core` emits
  native code; TinyCC compiles it. Runtime execution can still use interpreter
  fallbacks. Inspect cache revision/ABI requirements when changing emission or
  guest semantics; a host-only overlay does not itself change the guest ABI.
- Desktop packaging covers Windows and Linux. The release profile uses thin
  LTO and 16 codegen units; the Cargo manifest is authoritative.
- The desktop loads the full ROM into `MemMap.rom` at startup; ordinary guest
  ROM reads use that in-memory buffer. The [ROM data TODO](ROM_DATA_TODO.md)
  orders extraction/decoding, startup reuse and direct runtime consumption first,
  followed by mod overrides and broader changes. The explicit
  [rom-data CLI](../rom_data.md) now extracts/verifies typed OBJ dimensions for
  exact EUR/USA images, with a source-authenticated manifest and bounded parsing.
  `scripts/extract_rom_data.py` runs the existing offline extractor via Cargo or
  a selected executable/Wine and lists readable outputs under ignored `target/`.
  `scripts/linux/extract_rom_data_docker.sh` and
  `scripts/windows/extract_rom_data_docker.ps1` instead build/run
  Dockerfile.linux's shared `extract` target, requiring only Docker on either
  host (Docker Desktop Linux containers on Windows); ROMs are mounted read-only
  at run time and outputs remain host files, with host UID/GID on Linux.
  Docker extraction defaults to the EUR filename under root `rom/`; no ROM
  argument is required. Custom relative ROM arguments also resolve from the
  repository root, while absolute overrides remain available.
  The Linux Docker build wrapper saves every run's stdout/stderr under
  target/build-logs/ and waits for Enter on interactive failure, retaining
  the original exit status for noninteractive automation.
  [Inventory/contract](ROM_DATA_INVENTORY.md) documents other families' unknowns.
  The [overall ROM coverage atlas](ROM_COVERAGE.md) follows the latest committed
  EUR extractor generation with metadata-only HTML/JSON and separate candidate
  code/data/runtime/ordinary-use/sharing/unknown layers. Docker launchers stage
  a Git HEAD archive plus current public tools into Dockerfile.coverage, excluding
  uncommitted extraction work and private inputs. The cache key comes from the
  built extractor source, independently of historical evidence. New/changed
  resources remain pending ordinary-proof review; exact decoded AND source-map
  hashes preserve reviewed proofs. Migrated source-only spans are subtracted
  from that layer. HTML shows generation time, commit, dataset/evidence revisions;
  logs give deltas or unchanged coverage feedback. Every success rewrites outputs;
  PowerShell -OpenReport uses a versioned URL. Host requires Git/Docker and EUR ROM,
  no Python/Rust/old DLL. Runtime source counters do not establish distinct bytes
  read; speculative code/data conflicts remain explicit. Direct Python and the
  old working-tree Docker target remain developer alternatives. Native Windows/
  PowerShell5.1/Docker Desktop are unverified.
  EUR `v1/e14-d13` adds [static OBJ lists](../rom_data_obj.md):107direct literal
  roots reference66lists/160objects. Source-authenticated JSON fields now feed
  original CPU/DMA/native and host descriptor readers without changing guest
  instructions/clock or save/native ABI; all13files/12resources are verified
  before activation. Focused Wine original/native/managed/lifecycle and one
 397-frame ordinary transition/actual CLI checks pass; e13and prior decoded
  file bytes remain unchanged. Other indirect lists/animation/sprite pixels
  are not claimed migrated, and new Linux/USA verification remains pending.
  EUR e15-d14also adds ten indexed OBJ/animation bank pairs:167lists/95sequences/
 184frames with exact relative indices and duration words. Typed CPU/DMA/native/
  host views preserve original task counters/scheduler work, save/native ABI and
  e14file bytes. Reduced Wine original constructor/body, authentication/native/
  lifecycle checks and one positive160-frame ordinary menu/actual CLI pair pass;
  the three empty direct-root domains are now classified below; indirect/live
  scheduling remains unverified. The retained short battle sample is state/audio-equal
  but uses0new animation bytes, so it is not an ordinary-use acceptance.
  EUR e16-d15extends the static catalog to123references/73lists/
 173objects/1184sourceB, retaining all original66records, e15generations and
  every other decoded file. Seven new lists reuse the typed model/source paths;
  per-source guest/host flags keep new-list coverage distinct. Three focused
  Wine integration checks and one positive240-frame language-screen full-state/
  audio/actual CLI pair pass. Remaining switch/indirect roots stay open.
  EUR e17-d16adds an eleventh OBJ bank with five proved nonrepeat zero-count
  holds: explicit list selectors/duration-source aliases preserve header zero
  and reuse typed next-header/index owners without overlaps. Earlier ten-bank
  records and all other decoded files/e16generations remain unchanged; current
  totals168lists/286objects/100sequences/184frames/290ranges. Focused Wine
  original/native/host/authentication/lifecycle checks and a positive240-frame
  Party full-state/audio/actual CLI pair pass. The three earlier empty direct-root domains are classified below; other
  constructor-fed roots and new Linux/USA checks remain open.
  EUR e18-d17adds five language-selected OBJ/sequence pairs and two
  typed pointer tables, retaining all eleven old bank records/other files/e17.
  The indexed catalog now has16pairs/317lists/105sequences/328frames/456ranges;
  original selector loads, task work and duration256/u8counter wrap are preserved
  through typed guest/native/host views. Reduced Wine original, authentication/
  native/lifecycle checks and one positive400-frame worldmap“Engage!”full-state/
  audio/CLI pair pass. New localized counters distinguish this family. Linked
  compressed OBJ graphics/palettes remain open, along with new Linux/USA
  verification. The three older empty records B4BD20/B4BD34/B4FA10 have a
  bounded direct-root exclusion: twelve calls select0; eighteen widget mode
  producers select0..3 and two updates2/3. Thirty root literals/47instruction
  pins/50owners checked statically; no proved hold producer, so no new model/
  resource/revision. Six possible selector bytes stay raw; durations8/16/4
  alias existing sequence indices. Rawmode7/65539 can select6/7; no guestcap
  or global dead-code claim. Reopen only for a concrete producer/ordinary trace
  with established scheduling semantics; indirect/live mutation remains
  unverified. [Caller domains](../rom_data_obj.md#earlier-empty-sequence-caller-domains).
  EUR e19-d18adds fifteenth file obj-graphics.json/resource
  graphics.obj-custom-banks:39raw custom word banks/six five-entry selector
  tables,45disjoint ranges/62674sourceB/131872decoded B. Exact typed literals
  and pointers serve original CPU/DMA/native reads, retaining all decompression,
  scratch transforms, partial uploads and live RAM tails; PPU reads live VRAM.
  Previous13decoded files/e18generations stay unchanged. Reduced Wine original/
  managed/native/lifecycle and one positive400-frame Engage full-state/audio
  scene/actual CLI create/no-write reuse/old-cache replay pass; an audited DMA-fixture address is corrected by a narrow positive
  typed20Bcheck. Save/native ABI unchanged; new Linux/USA remain pending.
  EUR e20-d19adds sixteenth ui-palettes.json/resource
  graphics.ui-raw-palettes:21raw BG/OBJ UI shadow banks/23direct-derived-branch
  references/2656B/1328full-u16colors. Original176Bcode initializer and two live
  Color-Mode selectors, memcpy/fades/publication and palette RAM stay active.
  Typed CPU/DMA/native sources preserve save/native ABI and all14earlier decoded
  files/e19generations. Reduced Wine original/native/lifecycle and one positive
 240-frame boot full-state/image/audio/actual CLI pair pass; new Linux/USA
  remain pending.
  EUR e21-d20adds seventeenth ui-graphics.json/resource
  graphics.ui-custom-banks:nine raw custom banks(eightBG/oneOBJ)/ten original
  literal decode/upload paths,57038encoded B/84160decoded B. Typed literals
  preserve original r0returned-length/count work, live scratch and544Bpartial
  BG upload; all15earlier decoded files/e20generations and save/native ABI
  remain unchanged. Focused Wine original/native/lifecycle and a positive
 320-frame Clan full-state/image/audio/actual CLI pair pass; first Party→Unit
  locator is coverage-negative and preserved. Other indirect palette/graphics
  sources, unverified indirect/live empty selection and wider TODO/new Linux/USA stay open.
  EUR e22-d21extends UI graphics to16banks/two language selector groups
  and two explicit shared OBJ links,26ranges/71474sourceB/119168decodedB.
  Seven new banks and ten asset-pointer literals add17ranges/14436B; code jump
  tables, live language, English-vs-skip fallback and5056OBJ/2688BG split copies
  remain original. All9oldUIrecords/15other decoded files/e21generations stay
  unchanged, still17files/16resources. Focused Wine source/native/authentication/
  lifecycle checks pass every new bank and both selectors with full state/clock
  equality; save/native ABI unchanged. One positive1200-frame English intro
  full-state/image/audio/actual CLI Wine pair passes with4selectorB/2347new-sourceB/
 1124new-literalB. Other ordinary languages/new Linux/USA/native Windows stay open.
  EUR e23-d22adds [static item records](../rom_data_gameplay.md),
  eighteenth items.json/resource gameplay.item-records:376records/32bytes/12032B.
  Exact typed halfword/raw/signed/tail fields serve original CPU/DMA/native
  property readers. Live02002030bonuses, inventory, signed/clamp logic and
  secondary table remain original; save/native ABI and all16previous decoded
  files/e22generations unchanged. Reduced Wine source14371+240isolated-negative
  calls and authentication/lifecycle/native86calls with altered source/full
  states/clocks pass. One positive320-frame Unit full-state/image/audio/CLI Wine
  pair consumes40typed signed-field bytes; prefix/tail scenes not claimed. Source signed fields are all nonnegative; the synthetic
  cases are isolated and never authenticated/published. Other gameplay catalogs/
  semantics and new Linux/USA/native Windows remain unverified.
  Current EUR e24-d23adds item-links.json/resource gameplay.item-links:
 225records/20bytes/4500Bwith count/opaque/nine-pair storage and parsed item-owner
  dependency.19files/18resources authenticate together; all17old decoded files/
  e23remain unchanged. Typed CPU/DMA/native reads retain live unit query/bit7/
  FFbyte3dpublication and original work/clock/save/native ABI. Reduced Wine
  managed/lifecycle/native62paths with altered raw source match complete states/
  clocks; all bytes/BIOS/SWI/DMA and a2old+2new typed-byte boundary word pass.
  One positive560-frame headgear inventory full-state/image/audio/actual CLI Wine
  pair consumes574new bytes(287count/205selector/82value); the first slots-only
  locator is negative and retained. Source2244+8original checks stay closed. New Linux/USA/native Windows/wider
  gameplay remain pending. [Contract](../rom_data_gameplay.md#managed-and-direct-item-link-consumption).
  EUR e25-d24 adds `linked-metadata16.json` / `gameplay.linked-metadata16`:
  128 records of 16 bytes, 2048 source bytes, 20 files / 19 resources. All 18
  prior decoded files and e24 generations remain unchanged. The standalone
  source oracle passes 2581 original cases plus 19 isolated synthetic edges;
  the subsequent two Wine integration checks pass 42 sampled native paths with
  every new raw byte altered and equal complete states/clocks. Original live
  list/RNG work, typed crossing ownership and save/native ABI remain intact.
  One positive 520-frame Clan Mission Item full-state/image/audio/actual CLI
  pair passes with 6 typed display bytes. Broader field names, alias-zero caller
  preconditions and other ordinary fields remain unproved; new Linux/USA/native
  Windows checks remain pending. See the [contract](../rom_data_gameplay.md#managed-and-direct-metadata16-consumption).
  EUR e26-d25 adds `linked-metadata52.json` / `gameplay.linked-metadata52`:
  116 records / 6032 source bytes, 21 files / 20 resources, retaining all 19
  earlier decoded files and e25 generations. Actual parsed normal item-link
  selectors validate against the new owner; FF retains its separate contract.
  Two focused Wine checks pass 75 sampled native paths with new raw bytes
  altered / equal complete states and clocks, transactional authentication,
  lifecycle, all-byte bus/SWI/DMA and adjacent typed ownership. Original getter/
  alternate/redirect/live-record work and save/native ABI remain. Closed source
  6369+294 calls / bounded-loop proofs are retained. One positive 540-frame
  Change Jobs full-state/image/audio/actual CLI pair passes with 664 new bytes
  (74 header / 335 routing / 255 packed); ordinary tail/opaque is not claimed.
  New Linux/USA/native Windows checks remain pending. See the
  [contract](../rom_data_gameplay.md#managed-and-direct-metadata52-consumption).
  EUR e27-d26 adds `linked-metadata4.json` / `gameplay.linked-metadata4`:
  20 two-pair records / 80 source bytes, 22 files / 21 resources, retaining
  all 20 earlier decoded files and e26 generations. Parsed metadata52 indices
  and new pair selectors validate together. Two focused Wine managed/native/
  lifecycle checks pass 56 prefixes with raw bytes altered / equal complete
  states/clocks and adjacent typed ownership. Original count/contiguous walk/
  live qualification/signed comparison/tally and save/native ABI remain.
  One positive 540-frame Change Jobs full-state/image/audio/actual CLI pair
  reads 32 selector / 6 value bytes. Closed source320+24 proof stays valid;
  new Linux/USA/native Windows checks remain pending. See the
  [contract](../rom_data_gameplay.md#managed-and-direct-metadata4-consumption).
  EUR e28-d27 adds linked-metadata2.json/gameplay.linked-metadata2,23files/
  22resources, all21prior decoded files/e27retained. Metadata2 bounds 44 two-byte key/value records / 88 bytes at
  6e6c44..6e6c9c; last row43 is real. Original forward u8 indexing reads44,
  inverse u8 search scans43 and returns first stored key / miss0. One focused
  Wine348real+13synthetic source oracle and parsed metadata52 owners pass;
  semantic value names/global caller bounds remain unproved. Two focused Wine
  managed/lifecycle/native checks pass21paths with88rawB altered/equal complete
  states-clocks, typed boundaries/SWI/DMA. One1840Pub mission full-state/image/
  audio/actualCLI pair reads4value/0keyB; inversekey use is targeted-only.
  Source348+13proof stays closed, new Linux/USA/native Windows stay pending. [Contract](../rom_data_gameplay.md#two-byte-forward-and-inverse-source-contract).
  EUR e29-d28adds default-constructor-template.json/resource
  gameplay.default-constructor-template,24files/23resources/all22prior decoded
  files/e28retained. `default48::Catalog` owns48B at6e6c9c..6e6ccc. Original constructor
  mode1/2gates copying, IDs20..23select four separate sources, common signed-ID/
  selector/argument/ORflag patches preserve all remaining fields. One focused
  Wine271real+30synthetic prefix oracle passes after original IWRAM helper
  initialization; allocation/live-unit tails, alternate192B/ROMpointer and next
  240Bremain unowned. Two focused Wine managed/native/lifecycle checks pass16paths
  with48rawB altered/fullstates-clocks equal, real IWRAMhelper and8alternate/
  rejected paths0newreads. One397Pub full-state/image/audio/actualCLI pair reads
  96newBacross all structural categories; source271+30closed/not repeated.
  New Linux/USA/native Windows remain pending. [Contract](../rom_data_gameplay.md#default-constructor-template-source-contract).
  EUR e30-d29adds constructor-alternate-templates.json/resource
  gameplay.constructor-alternate-templates,25files/24resources/all23prior decoded
  files/e29retained. `constructor_alternates::Catalog` owns4records/192Bat6eca78..6ecb38
  and their4Bbase pointer70cd58,196Btotal. Typed IDs/offsets/structural fields and
  first5equipment halfwords perrecord validate against parsed376item owners.
  One focused Wine24real+8synthetic original-copy/live-patch prefix oracle passes,
  with allocation/live-unit tails unclaimed. Two focused Wine managed/native/
  lifecycle checks pass14paths/196rawBaltered/equal fullstates-clocks/pointer+
  all4recordsmask15/five zero-new-use paths. One3600Lutia deployment full-state/
  image/audio/actualCLI pair reads45B(pointer4/record20fields41), mask1; other
  records/full-copy behavior targeted-only. NewLinux/USA/nativeWindows pending.
  [Contract](../rom_data_gameplay.md#typed-constructor-alternates-source-proof).
  EUR e31-d30 adds linked-metadata14.json/gameplay.linked-metadata14,
  93 records / 1,302 bytes at 6e6f6c..6e7482, logical IDs2..94 and biased
  address6e6f50. All 26 files / 25 resources and parsed metadata52 byte02 owner
  authenticate together; all 24 earlier decoded files/e30 remain unchanged.
  Byte01 has no proven semantic owner. Source504+13 and two focused Wine
  managed/native/lifecycle checks pass; 37 paths with raw bytes altered match
  full states/clocks, normal IDs0/1 use no new bytes. Original live books,
  palette copies, overrides, clock and save/native ABI remain. One397Pub
  full-state/image/audio/actualCLI pair reads27B/two records/all field groups.
  Global bounds/palette payloads/allocation tails and newLinux/USA/nativeWindows
  remain pending. [Contract](../rom_data_gameplay.md#managed-and-direct-fourteen-byte-special-unit-metadata).
  EUR e32-d31 adds linked-metadata12.json/gameplay.linked-metadata12,27files/
  26resources/all25prior decoded files/e31 retained;108rows/1296B across two
  indexed banks6e7484..6e7994,48two-bit fields per row. Original full32row/
  signed-wrapping slot/u8mode/clock/save/nativeABI remain. No semantic owner
  dependency invented. Source1640real/10raw-wrap/384synthetic plus two focused
  Wine managed/native/lifecycle checks pass49paths/raw1296Baltered/fullstate-
  clock equality/bothbanks positive/three outside0newreads.
  One positive2200-Echo Grass confirmation full-state/image/audio/actualCLI pair passes.
  Global bounds/next global bank/otherfamilies and newLinux/USA/nativeWindows
  remain pending. [Contract](../rom_data_gameplay.md#managed-and-direct-twelve-byte-packed-pair-banks).
  EUR e33-d32 adds global-pair-flags.json/gameplay.global-pair-flags,
  28files/27resources/all26earlier decoded files/e32 retained. Minimum44pairs/
  11Bat6e7994..6e799f, next3Bunowned. Source102+10+91 and two focused Wine
  managed/native/lifecycle checks pass28paths/raw11Baltered/fullstate-clock
  equality, prior/new typed crossing and partial SWI/DMA tails. Original
  ignoredr0/slotwrap/u8mode/live-mask/earlyexit/clock/save/nativeABI remain.
  ActualCLIcreate/no-write-reuse/oldcache/cwd pass;2200EchoGrass is0globalB
  despite44oldmetadata12B. Positive ordinary global use/fullpair and highmode/
  newLinux/USA/nativeWindows remain pending; original descriptorhword4/live
  mask gates guide new scene selection. [Contract](../rom_data_gameplay.md#managed-and-direct-global-packed-pair-flags).
  EUR e34-d33 adds evaluation-parameters.json/gameplay.evaluation-parameters,
  29files/28resources/all27prior decoded files/e33retained.7terms/2bonuses/
  5class thresholds own38B at6e79a2..6e79b4and6e79c6..6e79da;14Bscoregap/
  4Brow0unowned. Source132+52 and two focused Wine managed/native/lifecycle
  checks pass27prefixes/raw38Baltered/fullstate-clock equality/all5thresholds/
  class0no-read/SWI-DMA/rawgap boundaries. Original live producers/unsigned-u8/
  signed32/wrap/bonuses/RNG/clock/save/nativeABI remain. One1840Pubmission
  full-state/image/audio/actualCLI pair reads15scoreB; a later400frame ordinary
  Yes/fee acceptance CLI/Winefullpair reads1threshold/class1. Classes2..5
  remain targeted-only. NewLinux/USA/nativeWindows/wholefunctiontails/
  semanticlabels pending. [Contract](../rom_data_gameplay.md#managed-and-direct-evaluation-parameters).
  A static ten-byte progression/price contract bounds100records/1000B at
  6e79da..6e7dc2 from normal index0..99and three literal/indexed/default paths.
  Eightu8deltas retain original live level/progress/flag work; byte08discount
  uses the original parsed-Items price chain. Byte09unused/12Btailunowned;
  candidate101rows rejected. Strict lossless source model and one Wine original
  oracle pass362real/68synthetic publisher/delta/live-price cases using actual
  parsed Items, exact u8wrap/signed-unsigned minimum and full EWRAM/stack/callee
  guards. Managed/runtime/ordinaryuse and newLinux/USA/nativeWindows pending;
  e34unchanged. [Contract](../rom_data_gameplay.md#next-ten-byte-progression-and-price-records).
  EUR e35-d34 adds progression-rewards.json/gameplay.progression-rewards,
  30files/29resources/all28prior decoded files/e34retained;100records/1000B
  direct typed delta/discount/opaque bytes. Two focused Wine managed/native/
  lifecycle checks and27original/interpreter/native paths with1000rawB altered
  pass fullstate-clock equality; compiled17delta/4discount/0opaque, outside/
  publisher/ungated paths0newreads, allbytes/boundaries/mirrors/BIOS/SWI/DMA
  and old-evaluation/new-progression crossing pass. Source362+68closed;
  ActualCLI prepare/reuse/no-write/oldcache and one600Shop fullstate/image/
  audio pair pass7discountB/record4; later e36dispatch below covers8deltaB/row0.
  NewLinux/USA/nativeWindows/whole progression caller remain pending.
  [Contract](../rom_data_gameplay.md#managed-and-direct-progression-rewards).
  Next static progression-gates source owns64x12/768B6e7dce..6e80cefrom
  original0487c8stride/loop, with independent next root exactly at its end.
  Byte00unused, halfword01returned/byte03output/eight live unsigned minima
  preserve physical-index u64received-bit ordering; row63returns0but still
  marks bit63. Strict source model/one Wine oracle pass80real+25synthetic
  full0487c8cases, all64rows/eightliveunsignedminima/bit31-32-63/zero-success/
  fullRAM/SP/callee guards;704used bytes and768Broundtrip. Whole downstream
  receipt/FK/managed/runtime/ordinaryuse pending, e35unchanged. [Contract](../rom_data_gameplay.md#next-progression-gates-and-returned-receipts).
  EUR e36-d35adds progression-gates.json/gameplay.progression-gates,
  31files/30resources/all29prior decodedfiles/e35retained;768typedB/two pages/
  requirements-return-output-unused/64physicalrecordmask. Two reduced Wine
  managed/native/lifecycle checks pass19original/interpreter/native paths
  with all768rawB altered/fullstate-clock equality, zero-valued success and
  fullmask0read preserved;bus/BIOS/SWIpagecross/DMA/gap/coexistence pass.
  Source80+25closed. ProductionCLIcreate/reuse-no-write/oldcache passes; a
  new normal10dayMadAlchemist dispatch yields209requirements/8deltaB, one
  1000Winefullstate-image-audio pair passes. Return/outputordinary remain
  targeted-only; the separate400frame Yes/fee prefix now closes thresholdclass1
  ordinary use. Downstreamreceipt-FK/newLinux/USA/nativeWindows pending. [Contract](../rom_data_gameplay.md#managed-and-direct-progression-gates).
  Next progression selection contract owns1024B6e80de..6e84de (64x16,
  physicalindices1..64), row0/16B and next2B unowned. Hard unsigned minima,
  ANY of8active live thresholds, original flag/reset/count helpers and physical
  order preserve conditional selector/zero-terminal/no-store-exhaustion behavior.
  Source-only strict model and one Wine fullD2390/reset-count-flag oracle pass
  138real+80synthetic cases,831original fieldB/1024Broundtrip/SP-callee-fullRAM;
  row52zero-terminal versus exhausted no-store preserved. Managed/runtime/
  ordinaryuse subsequently verified below. [Contract](../rom_data_gameplay.md#progression-selection-gates-source-contract).
  EUR e37-d36adds progression-selection-gates.json/gameplay.progression-selection-gates,
  32files/31resources/all30prior decodedfiles/e36retained/USAe8-d7. Two reduced
  Wine managed/native/lifecycle checks pass18fullD2390paths/raw1024Baltered/
  fullstate-clock equality, all6field counters/64recordmask/16Brow0+2Bgapraw/
  typedbus-BIOS/SWI/DMA/oldgate-newselectioncoexistence. Original reset/count/
  ANY/flag/terminalstore/clock/save-nativeABI remain. Source138+80closed;
  ActualCLIcreate/reuse-no-write/oldcache passes; first400Yes-fee locator0newB
  has no fullpair. Existing1000normal dispatchcompletion CLI/fullpair passes
  277newB/all6fields/first26records, every complete state-clock-image-audio equal.
  NewLinux-USA-nativeWindows/all64ordinary/semanticdisplay pending.
  [Contract](../rom_data_gameplay.md#managed-and-direct-progression-selection-gates).
  Aggregate-flag-keys source contract owns300u16slots+ffff/602B6e9a5c..6e9cb6,
  independent next root at exactend. Strict source model and one Wine full
  D22FCglobal/13FB1Ccontext oracle pass32real+26synthetic cases/all602B,
  key+2ffu16wrap/duplicates/earlysentinel/contextisolation/flag36ORlinked017f/
  read-only fullRAM-SP-callee guards. Managed/runtime/ordinary proofs follow
  below; newLinux-USA-nativeWindows pending. [Contract](../rom_data_gameplay.md#aggregate-flag-keys-source-contract).
  EUR e38-d37adds aggregate-flag-keys.json/gameplay.aggregate-flag-keys,
  33files/32resources/all31prior decodedfiles/e37retained/USAe8-d7. Two reduced
  Wine managed/native/lifecycle checks pass16fullglobal-contextcounter paths/
  raw602Baltered/fullstate-clock equality/all602B-300entrymask/terminal-SWI-DMA/
  raw-boundary-BIOS guards. Original live predicates/context/earlystop/wrap/
  count/clock/save-nativeABI remain. Source32+26closed. ActualCLIcreate/reuse-no-write/
  oldcache and one1000Wine dispatch fullstate-image-audio pair pass1204sourceB
  (1200key/4terminal)/all300slotmask; includesrereads, no bothentrypointsinference.
  NewLinux-USA-nativeWindows/wholeotherUIreaders pending. [Contract](../rom_data_gameplay.md#managed-and-direct-aggregate-flag-keys).
  Source-only trade-parameters model owns five96B records/physical IDs2..6,
  480B at6e85bc..6e879c, from explicit selector/biased price/independent next roots.
  64opaqueB/u16markup/30discountB preserve count0alias of markup highbyte41.
  Original separate70B-source D1F30property36, zero skip, live flags/count30RAMrows,
  unsigned divide/wrap arithmetic remain guest code. One Wine source oracle
  passes499real+160edited-source+15separate injected arithmetic operands,
  all160price-fieldB/320opaqueBroundtrip-only/fullRAM-SP-callee/source guards.
  EUR e38-d37 unchanged; managed/runtime/ordinary/newLinux-USA-nativeWindows
  pending. Adjacent6e9cb6halfword lookup remains partial: producer index bounds
  unproved, no106-entry model inferred. [Contract](../rom_data_gameplay.md#trade-parameters-source-contract).
  EUR e39-d38publishes trade-parameters.json/gameplay.trade-parameters,
  34files/33resources/all32prior decodedfiles/e38retained/USAe8-d7. Two reduced
  Wine checks pass28original/interpreter/native paths/raw480Baltered/fullstate-
  clock equality, all480B/mask31/field-boundaries/priorowner2-raw222-new2/SWI-DMA/
  lifecycle/source guards. Original live/getter/price/count/clock/ABI preserved;
  opaque price reads unclaimed. ActualCLIcreate/reuse-no-write/oldcache and one
  400Wine feeacceptance fullstate-image-audio pair pass14sourceB(markup-low7/
  shared417),mask1physicalID2, includesrereads. Other ordinary fields/records/
  newLinux-USA-nativeWindows pending.
  [Contract](../rom_data_gameplay.md#managed-and-direct-trade-parameters).
  Next packed item-effect parameter source is statically bounded:14×7B/98B
  6e879c..6e87fe, original itemIDs362..375/subtract362 and independent next root.
  Nine5-bit coefficients/8-bit cap/3opaque bits; CFE28low8property/index and
  CFDA8fullu16cap index differ. Strict source model/one Wine original oracle pass
  540real+4explicit raw-cap aliases+166synthetic cases/all98B/fullEWRAM-SP-callee/
  itemowner/source guards; no14371itemoracle repeat. Managed/runtime/ordinary/
  newLinux-USA-nativeWindows pending, e39unchanged.
  [Contract](../rom_data_gameplay.md#item-effect-parameter-source-model-and-original-oracle).
  EUR e40-d39publishes item-effect-parameters.json/gameplay.item-effect-parameters,
  35files/34resources/all33prior/e39retained/USAe8-d7; explicit parsed item-owner
  dependency. Two Wine checks pass50original-interpreter-native paths/raw98B
  altered/fullstate-clock equality/all98B-mask16383/edges-BIOS/SWI-DMA/two-owner
  trade crossing/publication-retention-lifecycle. Original getter/indexwidths/
  raw-cap aliases/live current/quantity/clamp/clock/ABI preserved. ActualCLIcreate/
  reuse-no-write/oldcache and one400Wine EUR GoblinPlay fullstate-image-audio
  pair pass3sourceB(coefficient1/shared051/shared061),mask1record0. Normal guest
  input from retained EUR IN13reaches actualCyrilBank/GoblinLv4/Play; replay is
  mixed native/interpreted. Other14items/20monsters/cap>99UI/newLinux-USA-native
  Windows pending.
  [Contract](../rom_data_gameplay.md#managed-and-direct-item-effect-parameters).
  `rom-data prepare` and opt-in `play --rom-data` share installed/portable
  preparation: versioned immutable generations, source-verified reuse, OS locks,
  atomic relative selection and corruption recovery before save writes. Explicit
  `FFTA_RESOURCES_DIR` overrides the root. No old generation is recursively
  deleted; Linux and Windows/Wine process-interruption/relocation/startup checks
  pass. [Direct OBJ metadata consumption](../rom_data_runtime.md) is now opt-in
  in play/frames/runc: fixed source projections serve CPU/DMA/native reads,
  typed host readers preserve changed RAM and nonserialized Arc resources
  survive Load/Undo/backup clean boot. Extractor2 adds EUR's copied alias;
  the verified EUR host draw source is ROM14ced8, not IWRAM. Base data remain
  source-equal, preserving save/native identities. New EUR worldmap/battle/menu/
  boot-transition full-state/audio checks pass independently on Linux and Wine;
  the copied alias is actually read at frame5400. Local frame/startup/memory/disk
  costs pass the predefined [validation protocol](ROM_DATA_VALIDATION.md), with
  neutral frame results and retained Wine startup variability. Actual Linux/
  Wine native/interpreter Fast/pause/Load/Undo and backup clean-boot GUI checks
  now pass with real muted callbacks and positive decoded use. Actual container
  archives pass395Linux/392Wine tests, privacy/PE/ELF checks and extracted-runtime
  GUI consumption with automatic bundled TinyCC and new portable preparation.
  Both Docker recipes/context include the packager's resource guides. An
  independent USA clean boot reaches the original copied-table initializer:
  all13instructions match full legacy states and24reads consume decoded data,
  including a poisoned-raw-source provenance probe, on Linux and Wine. Native
  Windows remains a gate before normal enablement; the user currently has only
  Linux/Wine. Other families/mods remain open.
  [Graphics groundwork](../rom_data_graphics.md) adds bounded offline LZ77
  decoding, four independently observed source ranges per region and exact
  comparisons to captured guest destinations on Linux/Wine. Original callers
  identify88byte descriptor roots atUSA569104/EUR7662a8. Typed halfword and
  indexed-pair packet decoders now match original regional writers, including
  chunk continuation, on Linux/Wine. Literal pair syntax has a separate
  destination guard after an original-code negative probe. A source-authenticated
  typed model now verifies the contiguous162-record/14256-byte descriptor block,
  exact source/TOML roundtrip and original selectors for every record on Linux/
  Wine. Links, transfer triples, optional variant pointer and opaque tail bytes
  are distinguished. Initial managed v1/e6-d5 extracted/verified/prepared OBJ, BG metadata and base
  palette banks with exact consumed source metadata:64KiB manifest/16KiB OBJ/
  256KiB descriptors/1MiB palettes/1280KiB total input guards, atomic four-file publication/reuse/repair and prior-version retention. Activation
  authenticates all files and hashes every ID/file into dataset identity;
  direct BG source reads now use its retained typed metadata/projection through
  fixed CPU/DMA/native bus paths with separate diagnostics/counters. Owned
  memory448shared/OBJ+28672BG+94713palettebytes stays within predeclared1KiB/64KiB limits.
  Linux/Wine204core tests,98/97frontend defaults, regional original/native
  poisoned-ROM provenance and Load/Undo/backup pass. New1200-frame EUR native
  boots match every full state/image/OAM/dispatch/audio with53BG reads each.
  Current BG worldmap/battle/boot-transition full-state/image/audio parity now
  passes Linux/Wine; five native timing pairs and startup/disk gates pass.
  Historical e3 TOML peak RSS exceeds the declared2MiB diagnostic bound,
  mostly preparation/parsing rather than the29KiBowner. Current e4 JSON uses
  direct typed parsing and standard readable formatting; actual regional file/
  native/lifecycle/CLI checks pass and old e3data remain intact. The user removes
  extracted-data space minimization as a priority, so no packing/indentation
  tuning or extra budget-only campaign continues. Updated user steering also
  defers performance concerns and explicitly asks to avoid measuring them;
  do not resume old benchmark/profile/startup/RSS queues. Preserve historical
  results, continue functional migration/equivalence and optimize later.
  A later explicit request on 22 October authorizes one current Linux-export EUR
  headless performance sample with ROM-data off/on; this scoped exception does
  not resume the paused implementation goal or broader profiling/RSS/optimization
  queues. See the current [validation protocol](ROM_DATA_VALIDATION.md).
  Updated regional priority is EUR; new USA verification remains pending.
  Retain completed USA evidence without reopening or extending its campaigns.
  Updated platform priority is reduced Wine-only testing; mark new Linux checks
  pending and retain the Linux evidence already completed before this steering.
  Current whole-process JSON RSS is not inferred from the parser prototype. Remaining BG GUI/archive,
  associated payload/program formats, other catalogs and language coverage.

  The next content family has a complete declared base-palette inventory:
  all162descriptor links resolve127unique sources per region (125LZ77/2raw),
  with equivalent source/output hashes. Original regional producers match all
  decoded buffers on Linux/Wine in1296calls/72792instructions each, including
  direct/current IDs and both base environment modes. Fullu16palette words
  retain bit15; alternate environment programs remain unclassified. A bounded
  typed palette model and readable four-file preparation now pass39Linux/37Wine
  affected checks and22actual CLI checks per target, including original producer
  comparison, prior e4retention, full-set identity and transactional failures.
  Palette04consumption now uses decoded banks in the existing LZ77HLE and
  exact file-derived CPU/DMA/native/guest-BIOS source views. Schema1/extractor6/
  decoder5 retains e5; all forms stay source-equal. Linux/Wine210core and
  101/100frontend defaults pass. Four original/native regional producer cases
  match1296calls/72792instructions with all source spans poisoned; full state
  and Load/Undo/backup/file-deletion behavior pass. New EURnative1200-frame
  pairs match every state/image/OAM/audio and consume one160Bpalette. Actual
  current CLI verifies retention/repair/reuse and positive interpreter/native
  use. New e6EURworldmap240/battle3530/transition8476frame checks now match
  every full state/image/OAM/dispatch/audio on Linux/Wine; stable segments use
  no new palette, transition consumes one160Bpalette. Further GUI/costs, actual
  BIOS execution, native Windows,
  remaining GUI/alternate palette migration and other families stay separate
  gates. Alternate palettes now have two independently located157-key catalogs
  per region,314custom LZ/RLEstreams producing256Beach. Source/output/link-table
  hashes agree EUR/USA; four original producer oracles match1256calls each.
  Five descriptor FFkeys are unavailable and caller preconditions remain
  unknown. Bounded typed alternate decoding now matches all retained sources/
  outputs and1120synthetic original calls per regional/target case, including
  every token/03..0Fcontrol, alignments and source cursor. A malformed original
  overrun is rejected offline. Fullu16banks, relative links, keyFFand exact
  source metadata are retained. Managed alternate extraction/preparation now adds a fifth fixed JSON file
  under e7-d6, with all316provenance ranges, complete dataset identity and
  transactional activation authentication. e6is retained; standard readable
  JSON is used without space tuning. Actual regional CLI, repair/reuse/move,
  Load/Undo/backup/restart and failed activation checks pass on Linux/Wine.
  Alternate guest consumption now retains immutable banks/links and resolves
  custom-codec literals from decoded words, controls from exact metadata. Shared
  codec maps preserve every original instruction/write/cycle through existing
  CPU/DMA/native bus APIs. Four regional/target poisoned-source producer cases
  match1256calls/final full states each, with positive decoded/native bank reads;
  Load/Undo/backup/restart retain the owner after file deletion. Native216core/
 104runtime and Wine216/103pass; separate first-frame CLI/cache checks are
  activation-only with0alternate reads. Original EUR Options now identifies LCD A=0, LCD B=1(second alternate catalog),
  TV=2(first), committed only through Save changes Yes. Four direct callers per
  region and the setter are inventoried; local descriptor-FF guards were not
  found. Worldmap/Party mode changes report0alternate reads, while the new
  Cyril-pub BG62transition proves positive link/literal use for LCD B/TV. Four
  native397-frame Linux/Wine pairs match every full state/image/OAM/dispatch/
  audio sample; actual GUIs present both modes and close0with positive use.
  Field04/08/10packet catalogs now have352disjoint sources per region,
  raw/LZ77wrappers, at most2-node descriptor inheritance and exact dictionary
  read extents. All source/body/alias/parent hashes agree EUR/USA. Four original
  component loader cases on Linux/Wine match every162×3complete target buffer,
  guards and SP. Dynamic paired fill and ordered patches remain explicit;
  Typed packet files now retain352programs/486links, ordered writes and exact
  source/body metadata with strict graph/source validation. Eight core packet
  tests, JSON guards and full retained EUR body checks pass on Linux/Wine;
  standard readable JSON is identical. New USA validation is pending by user
  instruction. Managed e8-d7now publishes the sixth packet JSON, authenticates
  all352source ranges/file identity and retains e7. Four focused Wine cases plus
  seven actual Windows CLI operations pass create/verify/reuse/repair/move,
  transactional activation and old-cache dispatch. Canonical/protected inputs
  unchanged, builds warning-free; Linux/USA new checks pending. Runtime packet
  reads now consume the immutable typed e8model for EUR: original control bytes,
  typed raw/LZliterals and decoded HLEbodies, with merged palette/packet edges
  and the verified895page. USA stays authenticated/verified-only. Three reduced
  Wine checks pass, including nine original/native calls with all352old source
  spans poisoned, full-state/clock parity and compiled typed use, plus
  Load/Undo/backup/restart with unavailable temporary file copies. One397frame
  EUR LCD B pub transition matches every state/image/dispatch/audio stream and
  consumes2640typed packet bytes. Dataset/native/save/cache ABI unchanged; no
  performance/space work. New Linux/USA/GUI/archive checks pending. EUR field00
  now has a bounded typed offline catalog:162links/50disjoint sources,
 23tag20/27tag22,621923encoded/835232decoded bytes. The staged producer0801b688
  uses the alternate palettes' custom LZ/RLE080054a0; shared parsing preserves
  independent palette guards. All50full outputs/cursors/guards/SP and two
  representative phase2/3DMA layouts match the original on Wine (54calls).
  Nine focused Wine checks pass including JSON/source-authentication guards.
  Managed EUR extraction/preparation now adds the seventh fixed graphics JSON
  under1/e9-d8, with50source ranges, full-set authentication/identity, atomic
  publication/reuse/repair/move and e8retention. Four reduced Wine checks and
  seven actual CLI operations pass; prior decoded files remain byte-identical.
  At that managed boundary first-frame old-cache activation used0resource bytes.
  EUR graphics consumption now retains typed words, exact headers/controls and
  custom-codec literal maps under fixed769..895bus pages. Three reduced Wine
  tests pass, including four original/native calls with all50raw sources
  poisoned and full state/clock parity (29023compiled typedB), and retained owner
  across Load/Undo/backup/restart with temporary copied files unavailable.
  One397frame Cyril-pub EUR transition matches every full state/image/OAM/audio
  and consumes10334decoded graphics literalB. e9-d8/save/native ABI unchanged;
  USA's six-file e8-d7contract is preserved, new Linux/USA checks remain pending.
  EUR transfer/variant inventory now has97active triple links across five slots,
 42typed controls/34shared raw tile banks (968/199296B) and all162variant pointers
  absent in this declared block. Bounded offline source-authenticated models
  retain frame durations/tile offsets/aliases/words. Five reduced Wine checks
  pass, including four original tick/animation/split-DMA cases with full VRAM
  equality after each chunk (634calls/80766instructions); no new recording.
  Managed transfer publication now adds an eighth fixed JSON under1/e10-d9with
  all76source ranges and complete-set authentication/identity. Four reduced
  Wine checks and three actual CLI prepare/reuse/old-cache activation commands
  pass, preserving e9and six prior decoded files. Transfer runtime now retains
  typed controls/tiles through fixed778..897CPU/DMA/native source views, with
  separate counters and original mutable countdowns/flags/copy progress/clock.
  Three reduced Wine checks pass: source widths/edges/last page/SWI/DMA/load,
  two poisoned-source original/native forward/reverse cases (60calls/66compiled
  controlB/9984native-DMA tileB) and unavailable-copy Load/Undo/backup/restart.
  One397frame EUR pub transition matches every state/image/OAM/dispatch/audio,
  uses1203controlB/21504tileB; one actual CLI with no-write e10reuse/old cache
  matches the valid retained legacy row. e10-d9/save/native ABI unchanged.
  USA retains six-file e8-d7;
  new Linux/USA checks pending. Broader regional/gameplay/catalog/language coverage,
  global FFpreconditions, archives and nativeWindows stay open.
  Exporting files alone is not completion; preserve guest behavior,
  native-cache/profile/save identity; cost measurements are deferred by the user.
  EUR text/font inventory now independently locates original08016648's five
 107-entry short-string tables. Bounded offline typed model retains535sources
  (478compact/57extended), exact controls and65referenced packed4bpp glyphs/
  widths; full provenance670ranges. Two reduced Wine checks match every original
  pointer/expanded string/guard, all65glyph slots and variable/fixed metrics
  (1331calls/89446instructions). See [text/font contract](../rom_data_text.md).
  Managed EUR1/e11-d10now adds ninth short-texts.json/eight resources, all670
  provenance ranges and whole-set identity; retains e10/seven decoded files and
  USAe8-d7. Three reduced Wine checks cover publication/reuse/atomic failure/
  repair/move/source/guard/transactional activation; actual CLI prepare/no-write
  reuse/one-frame old-cache activation passes at that managed-only boundary.
  Text runtime now retains typed indices/codes/glyph words/metrics through fixed
  EUR source pages, separate counters and unchanged original work/clock. Three
  reduced Wine checks pass: synthetic widths/mirrors/edges/SWI/DMA/load, five
  original/native reader chains with all670raw sources altered (20calls/full
  states/clock equal;20table/124string/636glyph/6metric compiled API bytes), and
  unavailable-copy Load/Undo/backup/restart ownership. Fixed mode skips metrics
  exactly as original. No schema/save/native ABI change. Manifest guard is1MiB.
  Other catalogs,
  full font/control/Unicode contracts and new Linux/USA verification remain open.
  The broad EUR offline text model now covers all nine pointer selectors
  (16100strings/45tables) and29relative selectors (145archives/17035programs)
  in the declared reader block; all33581source ranges disjoint,128glyphs.
  Headers, compact/extended commands/operands, exact custom-LZ representations
  and decoded trailing bytes remain explicit; dynamic context stays guest RAM.
  Two reduced Wine checks match independent full static inventories plus
 80new pointer boundaries/145roots/300archive outputs/63new font slots
  (968calls/867138instructions). Oracle explicitly supplies one-based active
  text context1via original nullable fallback, not ordinary gameplay coverage.
  Managed EUR1/e12-d11now adds tenth text-catalogs.json/nine resources with all
 33581source ranges and whole-set authentication/identity; e11/eight earlier
  decoded files retained, USAe8unchanged. Three reduced Wine checks cover
  deterministic publication/reuse, atomic-write failure/repair and actual EUR
  guards/identity/corruption/move/transactional activation. Actual CLI prepare/
  no-write reuse/old-native-cache one-frame activation passes at managed-only
  boundary. Broad runtime now serves typed pointers/strings/relative indices/
  headers/program units/font through fixed EUR pages; compressed literals come
  from decoded units/padding. Three reduced Wine checks pass: synthetic source/
  encoded-literal alteration/edges/SWI/DMA/load, fifteen original/native chains
  with all33581raw sources altered (90calls/full states/clock equal, positive
  compiled use in all8kinds), unavailable-copy Load/Undo/backup/restart.
  Explicit nullable context1fixture, no ordinary scene claim. Earlier overlapping
  subset stays authenticated/verified-only, no duplicate read owner. Manifest
  guard16MiB/catalog+dataset512MiBaccept readable data, not packing targets.
  New Linux/USA pending, no performance/space campaign.
  Full text catalog now also passes one retained EUR/Wine397-frame LCD B/pub
  transition with every state/clock/image/OAM/dispatch/audio equal and positive
 52pointer/185string/2index/4header/27decoded-program/8064glyph/93metric bytes.
  This is a raw-program scene; compressed use has separate native04proof.
  Warning-free production candidate/canonical restored; one new decoded actual
  CLI with no-write e12reuse/old cache matches the valid retained legacy row
  without repeating legacy. Broader scenes/GUI/archives/full font/readers and
  new Linux/USA/nativeWindows remain open. No new recording or performance work.

  EUR now prepares and directly consumes the complete3175-slot packed font/
  metrics and three name-keyboard programs/indices/navigation widths under
 1/e13-d12(12files/11resources, seven additional provenance ranges). Earlier e12
  files remain retained and unchanged, USA stays e8-d7. The complete font owner
  replaces overlapping subset reads; original keyboard page construction,
  rendering work and guest clock/save/native ABI remain unchanged. Reduced Wine
  source/original/managed/native altered-source/full-state/lifecycle checks and
  one397-frame ordinary pub transition/actual CLI pass. Keyboard ordinary scene,
  other catalogs/control semantics/Unicode, GUI/archives and new Linux/USA/
  nativeWindows verification remain pending. The latest request resumes EUR
  functional TODO work with reduced Wine tests and again excludes all new
  performance/space campaigns; the earlier Linux sample is closed history.

## Container builds

- `Dockerfile.linux` and `scripts/linux/build_linux_docker.sh` build Linux amd64 using
  Rust 1.94.0 / Debian Bookworm with ALSA/udev development dependencies inside
  the container. BuildKit caches Cargo/target separately from the host checkout.
  The scratch export stage writes only the portable tar.gz and BUILD-INFO.txt
  to `dist/docker-linux/`; runtime GUI/system libraries remain host dependencies.
  `.dockerignore` allows desktop build inputs and excludes game/personal data.
  Container export is gated by serial input/config, menu, core, runtime and
  screen suites; optional local-ROM/GPU cases remain separate. The exported
  runtime requires glibc 2.35 and previously passed a native Ubuntu Xvfb
  save/load/relocation check.
  Current optimized-renderer exports also pass an isolated cold-cache Xvfb
  load/Undo/reload/close check. Canonical dist exports include the retained
  renderer changes; validation records identify their checks and limits.
  Older-distro and physical-controller compatibility remain unverified.
  See [Docker workflow](../docker.md). Keep the direct Ubuntu-built archive
  distinct, and establish actual libc requirements for each build environment.

- `Dockerfile.windows` cross-compiles Windows GNU x86_64 with MinGW from the
  pinned Linux Rust base, builds TinyCC with distinct native/target compilers,
  and runs input/config, menu and runtime tests under container Wine.
  `scripts/windows/build_docker.ps1` is the Windows-host entry point: default Windows,
  `-Platform linux` for Linux. Docker Desktop must use Linux containers.
  `scripts/linux/build_windows_docker.sh` offers the Windows target from Bash.
  Export is `dist/docker-windows/`; it contains no game or personal data.
  Resource build scripts branch on Cargo's target OS rather than build-host cfg.
  The Windows builder reuses the pinned Rust image and copies its toolchain
  into Debian Trixie for MinGW 14 / Wine 10. Windows/Wine tests gate exports
  on serial core and screen suites as well as input/config, menu and runtime.
  Earlier packaged PE resource/import checks and a simulated-XInput portable
  save/load/move smoke also passed. PowerShell 7.4.6
  Linux mock-CLI validation passed; native Windows/Desktop/PowerShell 5.1 remain
  unverified. The current exported Windows runtime also matches the prior
  Windows and validated Linux replay on framebuffer/OAM, final machine state
  and counts under Wine. This is compatibility evidence, not native timing.

## Desktop behavior

- [Fullscreen](../fullscreen.md): Video > Fullscreen (F11) persists default-off
  `video.fullscreen`, sharing startup/menu/shortcut requests and asynchronous
  reported-state reconciliation. Winit/WM restore normal placement/maximization;
  missing-monitor fallback keeps a usable window. Existing resize/cursor/presenter
  paths remain; no guest/audio/aspect/zoom/history change. Menu GPU/software
  presentation uses the completed canvas dimensions across concurrent resize,
  avoiding upload overflow and a fallback stride panic. Native28config/35menu/
 81runtime and Wine28/35/82 pass; physical/native Windows coverage stays open.

- [Audio controls](../audio_controls.md): Audio > Mute persists default-off
  `audio.mute` separately from Volume and temporary Fast Silent/fallback mute.
  Startup/live edits share audio configuration; source consumption continues
  while output gain slews to silence, retaining volume and timing. Timeline
  resets preserve user mute. Audio-only edits do not reset video history.
  Native27config/34menu/79runtime and Wine27/34/80 pass; hardware listening and
  native Windows remain unverified.

- Fast audio transitions invalidate callback DSP history as well as queues:
  host epoch resets DAC/sinc/FIFO/DC/crossfade state under the existing mutex,
  retaining filter tables and ramping new output through the volume slew.
  Pause discards pending output while preserving the Fast latch; device loss
  also discards Normal's backlog. Load/Undo retains stream recreation.
  Fast consumption now follows produced PCM pairs against device sample-clock
  progress with a common smoothed clock for faithful/PSG/HLE/FIFO, a60ms reserve
  after each callback and fractional sinc cutoff tables. Normal1x retains its
  DSP; explicit waiting/rebuffer telemetry prevents silence hiding starvation.
  Native74/Wine75runtime tests and72clock cases pass. Actual local4x CPU-limited
  battle gates pass357/359Faithful/Enhanced callbacks with no source holds,
  PSG/HLE empties, waiting/rebuffers or drops; consumption tracks~2.2x production
  and max estimated source queue stays~108ms. Under changing compilation load,
  recovery can still require silence; this is reported, not continuous playback
  proof during arbitrary stalls. Backend-buffered audio cannot be retracted;
  native Windows/hardware/event latency and audible quality remain open.
  Normal delayed1x now uses a preallocated512KiB/2second queue, replacing old
  audio with the latest250ms and a10ms stereo fade on overflow; no temporary
  file or callback I/O. Its menu states the delay, legacy config remains valid,
  and Enhanced configuration still uses faithful PCM for Normal. Native78/
  Wine79runtime and33menu checks pass; real Linux4x Normal callbacks stay
  below131072pairs through10resyncs per mode with no holds/over-period calls.
  The queue budget excludes backend buffering. See [Fast audio](../fast_forward_audio.md).

- The observed battle laws table and its HELP window use the existing centered
  Notice layer, including the selector/continuation sprites, independently of
  the action-menu split and any remaining HUD. Recognition checks the frame,
  rows and divider inside the existing ROM/terrain/action gate, not translated
  words. The host subtype resets per frame and is skipped in snapshots. Native
  and Windows/Wine suites, 72 private EUR reconstruction cases, 24 synthetic
  GPU/CPU modal cases and inspected frontend checks pass. Other layouts/USA,
  native Windows and exact original tutorial coverage remain open. See
  [laws layout](../ffta_laws_layout.md).

- Native UI capture skips per-pixel sorting/blending when no admitted UI BG or
  portrait can reach the texture, retaining a default destination write to clear
  stale pixels. Real UI keeps the full blend/world-target path. No persistent
  cache or state/translation ABI change. Native/Wine174 core checks and full
  replay/zoom6 equivalence pass; five pairs reduce headless work4.20% and complete
  native unpaced GUI work2.67% relative to panel-row classification. Worldmap and
  default GUI load/Undo/reload/close pass. Current local Docker exports include
  that stage:336Windows/Wine and337Linux tests passed (15optional ignored each),
  archive/privacy/DLL/GLIBC2.35 and actual cold-cache Linux portable GUI verified.
  Previous archives are preserved. This is not original rare-stall or native
  Windows performance evidence.

- Release desktop audio/presentation resources in `PlayApp::exiting`, before
  `EventLoop::run_app` consumes/drops the display connection. Release surfaces
  and their contexts before the window, including error exits. Holding them
  until PlayApp's later field destruction caused a reproducible Wayland/Mesa
  EGL SIGSEGV even when Vulkan rendered the game. Native Ubuntu 26.04 GNOME /
  AMD RADV close-event regression checks pass; see [portable notes](../portable.md).

- [Desktop portable editions](../portable.md): `portable.txt` beside the actual
  executable opts into shared `input_config::storage::portable_root()` resolution.
  Settings/labels use `Data/config`, native translations `Data/cache`, cartridge
  progress `Data/saves/ffta.sav`; backups and slots remain beside that save.
  No AppData ROM/settings/save-pointer fallback in this mode. The wrapper copies
  a first-launch selected ROM to `ffta.gba`. Explicit environment path overrides
  still apply. Move the entire folder while closed; paths are resolved afresh.
  `scripts/windows/package_ffta_windows.py` and Windows `build-all.ps1` create a portable
  ZIP by explicit file selection. Native Linux uses
  `scripts/linux/package_ffta_linux.sh` and the shared
  `scripts/package_ffta_portable.py --platform linux` for a tar.gz with executable
  modes preserved and `tcc/libtcc.so`. The current archive targets the Ubuntu
  26.04 x86_64 host; broad older-distro/Steam Deck compatibility is unverified.
  Existing installed behavior needs no marker;
  migration is an explicit copy, never a silent move of personal data.
- Developer `play --benchmark <config.toml>` restores a bounded snapshot/input
  replay with a scene label and warm-up exclusion. It reuses guest/UI/temporal/
  GPU presentation functions while disabling physical input, FPS HUD, audio
  callbacks and pacing, and skips cartridge discovery/backup/autosave/exit
  writes. Verify images/OAM and full state separately from timing. This is an
  unpaced GUI work benchmark; realtime input/audio/compositor performance and
  older-binary default-path overhead need distinct evidence. See the
  [measurement workflow](PERFORMANCE_WORKFLOW.md).
- With `FFTA_PROFILE=1`, normal-play audio callbacks have bounded wall-time
  observation under the existing mutex and phase-separated faithful holds/
  producer drops. No new callback lock, logs or allocation; clock reads stay
  off by default. Capture survives legacy counter resets and reports after
  stream teardown. The unpaced GUI benchmark does not run the callback.
  Optional `FFTA_AUDIO_PROFILE_FRAMES=START:END` now selects one traversal of
  producer-published guest frames. Load/Undo inside capture interrupts it;
  default-off and callback-free benchmarks ignore the window. Native/Wine
  boundary/restore/audio parity and actual archive GUI checks pass. Idle battle
  window measurements show sub-microsecond mutex p99, with no faithful holds or
  drops; lifecycle transitions can still produce holds. Enhanced source demand
  telemetry is now available as raw PSG/HLE/FIFO requests/empty reads plus
  configured-target totals under the same gate. Warm/unrouted consumers are
  separate, with no per-request counters in the default const specialization.
  Output/queue/interpolator parity tests pass, including Windows/Wine. Idle
  battle captures show no empty source reads; lifecycle transitions can still
  produce holds/empty reads. Current Docker exports pass341Wine/342Linux tests
  (15optional ignored each), archive compatibility/privacy and actual Linux
  cold-cache lifecycle/default-off/benchmark checks. Configured target is not
  audibility or live crossfade weight; CPAL0.16.0 ALSA can recover XRUNs without
  app errors. Independent local PipeWire observation now matches PID/client/
  stream/link/sink and verifies stable serials/sample cadence: no ERR increments
  in measured idle/pause/resume intervals, but sink baseline already1 and graph
  errors are not dedicated hardware XRUNs. No audio-policy change is justified
  by these idle-scene results; Windows/hardware/slow-scene attribution remains
  open. See the workflow.
- Successful host presentation intervals are available in normal play under
  `FFTA_PROFILE=1`, optionally scoped by `FFTA_PRESENT_PROFILE_FRAMES=START:END`.
  They reuse the FPS timestamp; no new clock/allocation off or benchmark capture.
  Lifecycle/configuration/backend boundaries create anchors; surface skips stay
  in success-to-success gaps. The pure range helper is shared with audio without
  changing its behavior. Native/Wine tests, local clock/audio/requested-VSync
  matrix, silent fallback and actual portable lifecycle/off/benchmark checks pass.
  Clock is already Linux/Windows default; local means~16.74ms versus audio batch
  cadence~42.67ms. This is host API cadence, not scanout or native Windows/physical
  high-Hz proof. Current Docker exports pass344Wine/345Linux tests (15optional
  ignored each), archive compatibility/privacy and cold-cache Linux GUI gates.
- GPU render-pass profiling is opt-in via `FFTA_PROFILE=1` plus
  `FFTA_GPU_TIMESTAMPS=1`. Request only supported timestamp queries, retry device
  creation without optional features if needed, and use three reusable readbacks
  with nonblocking polling and a 18000-submission cap. Results are asynchronous
  gameplay-pass durations, separate from CPU queue/acquire/present and scanout.
  Paused frames do not submit queries; shutdown reports pending results without
  waiting. Mapping/poll failures stop diagnostics, not normal rendering. See
  [measurement workflow](PERFORMANCE_WORKFLOW.md). Default rendering requests
  no timestamp features or resources.
- Surface acquisition timeouts skip a host presentation while retaining the GPU.
  Lost/Outdated reconfigure and retry once per call; three failed recovery calls
  trigger software fallback. Successful acquisition resets the budget; memory
  exhaustion/unknown errors fall back immediately. Zero-sized surfaces skip
  acquisition, and skipped frames do not increment FPS EXT. Event-loop pacing
  remains responsible for retries; no tight retry loop or per-frame error log.
  Pending Queue writes are tracked across UI updates and flushed once on skips
  with transfer-only submissions. This prevents unsubmitted staging growth while
  retaining revision caches; these submits do not count as EXT or GPU pass samples.
  Zero-sized frontend windows avoid GPU/UI preparation and software surfaces;
  benchmarks still fail on skipped presentations. A no-ROM lifecycle diagnostic
  lives in `crates/recomp/examples/presenter_skip_uploads.rs`. Its actual
  llvmpipe run completed 1200 batches/492 MB logical transfers with stable RSS
  between batches 600 and 1200, then resumed normal rendering. Current exports
  pass 333 Windows/Wine and 334 Linux tests (14 optional ignored each), archive
  compatibility/privacy checks and cold-cache Linux portable GUI load/Undo/
  reload/close. Observed X11 minimize/restore is covered separately below;
  Wayland/native Windows, monitor/device-loss cases remain open.
- `crates/recomp/examples/presenter_window_lifecycle.rs` exercises real WM
  minimize/restore, resize and VSync without ROM data. It uses the event loop,
  bounded phase deadlines and observed minimized state, then releases Presenter
  before window. Native X11 Openbox/llvmpipe passes three cycles, as does the
  unchanged Linux portable in battle. The example also passes under Wine10/
  Vulkan after disabling only Wine's failing D3D12 DLLs in an isolated prefix.
  Native X11 minimized sizes stayed nonzero (27 Presented); Wine exposed zero
  sizes (18 Presented/9 Skipped). Do not equate minimized state with zero size
  or assume every hidden window skips acquire. EXT still counts successful host
  presentations, not visible scanout. Pinned winit cannot observe/restore
  minimized state on Wayland; this diagnostic fails on unsupported state rather
  than treating it as a pass. Native Windows/Wayland minimize, monitor changes
  and runtime device loss are not covered by these results.
  The separate `--monitors` mode now covers two distinct virtual CRTC outputs
  and observed A/B/A/B/A transitions, monitor-relative resize and VSync checks.
  Xorg/dummy 1280x720/1280x960 with Openbox/llvmpipe Vulkan passes20 presentations
  on native Linux and Windows/Wine; independent positions confirm the moves.
  The unchanged Linux portable passes the same moves in battle plus Undo/reload/
  close, inspected captures and canonical hash checks. Xvfb RandR monitor areas
  alone do not establish this: pinned winit enumerates CRTC outputs. The dummy
  keyboard needs explicit evdev US setup for synthetic menu input. Physical
  screens, GPU/DPI/color transitions, hotplug and Wayland remain separate work.
- Presenter owns its Device and retains the first uncaptured GPU error or
  device-loss callback in Arc/OnceLock. A confirmed destruction regression could
  panic when new UI/framebuffer textures were created; callbacks now return the
  failure to the existing software fallback. Further UI/VSync work is rejected
  on that presenter. Local configuration scopes stay recoverable and do not
  poison this latch; normal checks do no allocation, GPU poll/wait or callback
  logging. The private ignored `device_loss_tests.rs` covers actual destroy with
  plain/cached/new UI/resize and software presentation. Native and Wine/Vulkan
  pass, including scope isolation and stable first-error behavior. It does not
  prove all spontaneous/asynchronous hardware-fault cases.
  Updated Docker exports pass333 Windows/Wine and334 Linux tests (15 optional
  ignored each), with this regression also explicitly run on native/Wine Vulkan.
  Archive source/doc/privacy/DLL/GLIBC checks and actual cold-cache Linux portable
  load/Undo/reload/close pass without unexpected fallback. Normal physical Radeon/
  Wayland replay preserves3530frame hashes, state/counts and clean close.
- Surface configuration is guarded separately from acquisition recovery.
  `configure_surface` in `crates/screen/src/present.rs` captures native wgpu
  validation/internal/out-of-memory errors with balanced device error scopes.
  Startup/resize/VSync/recovery failures return to existing software fallback;
  the VSync setter returns a Result and its menu caller discards a failed GPU
  presenter. Scope inspection uses the pinned native backend's ready futures,
  without a new GPU poll/readback wait. Normal unchanged frames skip it.
- A native UI output-comparison cache was rejected after five GUI A/B pairs.
  It cut logical upload bytes about 74% but added preparation work and showed no
  useful whole-cycle gain (+0.131109% median work time). Source/native binary
  were restored exactly, and existing FPS/LUT caching is retained. Do not
  reactivate that double-buffer comparison design for context recovery. A
  follow-up must eliminate packing work or have distinct measured evidence;
  see the [evaluation workflow](PERFORMANCE_WORKFLOW.md#rejected-native-ui-output-comparison-cache).
- Native UI packing now caches complete source/backing inputs and resolved
  mode/length/anchor parameters using safe exact field words. Identical inputs
  skip pure packing and GPU texture transfer; changes conservatively repack.
  Clear/load/Undo invalidates host snapshots/revisions, and FPS/native producers
  share process-unique tokens. World/background pixels, LUT and layout remain
  dynamic. The five-pair native unpaced replay improves median mean host work
  1.41%; this does not establish an AI-turn or native Windows speedup.
- Native UI worlds share a texture/upload only for the same immutable slice
  extent and dimensions within one presentation. Every unique world uploads
  again on the next call; changed source groups split handles before writes.
  Stats/FPS sources remain independent, layout/LUT/UI state stays per layer,
  and dimension changes rebind the world without discarding valid UI/LUT data.
  Five native unpaced pairs show 40.37% lower scheduled UI payload and 0.39%
  lower median mean work; no AI/native Windows speedup is established.
- The row-endpoint panel-bound scan is a test-only experiment. It preserves
  the full scan's results and reduces synthetic panel cost, but production
  integration was withheld after slower 3000-frame introductory A/B replays.
  Keep the current full scan until representative end-to-end benefit is shown;
  see the optional diagnostic in the performance measurement workflow.
- GPU UI layers retain LUT contents and skip identical uploads; FPS pixels use
  process-unique render revisions. Native UI uses complete packing-input keys;
  unversioned UI and world data still update normally. Resource recreation clears caches, and color comparison handles
  in-place changes. FIFO audio taps drain without discarding producer capacity;
  `FFTA_FRAME_PACING` is resolved at startup. See [FPS notes](../fps_overlay.md)
  and the performance TODO for remaining optimization work.
- [FPS overlay](../fps_overlay.md): Video > FPS counter; off/internal/external/
  both. Inline row help explains game/present counts and expected Fast divergence;
  the user confirms the report occurs only with Fast, consistent with the4:1
  live captures at4x and~59.7/59.7 after disabling it. No pacing change.
  INT measures completed guest frames per wall second; EXT measures host
  presentations, not display scanout. Configuration uses `video.fps` in
  `%APPDATA%\FFTARecomp\av.cfg` in installed Windows mode.
- [Menu shortcuts](../menu_shortcuts.md): configurable single controls/chords
  under Input > Keyboard, Mouse & Keyboard, or Gamepad > Open menu. Defaults
  Escape and Start+Back/Select. Escape remains a recovery shortcut; Guide/Mode
  works where exposed. Persistence uses `key.menu` and `pad.menu` in
  `%APPDATA%\FFTARecomp\input.cfg` in installed mode.
- Save resolution lives in `crates/recomp/src/save_path.rs`. `FFTA_SAVE_PATH`
  overrides the selected location. Windows remembers the shared save location
  under `%LOCALAPPDATA%\FFTARecomp\shared-save-path.txt`. Save files are raw
  cartridge `.sav` data (64 KiB for the tested EUR game), not emulator save
  states. Portability requires matching game edition/region and a tool/device
  that accepts that save format; no new conversion feature was implemented.

- Restored paused displays use the authoritative native 240×160 framebuffer
  centered with cleared margins. The wide scratch buffer has no serialized
  stride and cannot be interpreted with the current frontend zoom. Separate
  UI caches stay hidden until the next completed guest frame rebuilds them;
  preview generation does not advance the machine or change its snapshot format.
- [Extended saves](../extended_saves.md): Debug option defaults off and expands
  snapshot slots from 9 to 99 without changing paths/formats or deleting hidden
  files. Debug > Game saves selects 99 standard cartridge files: original slot 1,
  `-slot-02.sav` through `-slot-99.sav`. Copy protects confirmed replacements;
  switching validates/prepares boot, backs up target, flushes old source and
  updates active path before frontend reset. Empty targets use erased memory.
  Portable selection persists only a number; explicit FFTA_SAVE_PATH wins.
- [Save states](../save_states.md): pause menu > Save states, nine default controller-
  operated slots with embedded previews and in-menu overwrite/error feedback.
  Storage: `SaveStates/<ROM SHA-256>/slot-1.ffstate` through `slot-9.ffstate`,
  relative to the selected cartridge save directory. A new session starts at
  slot 1; Left/Right wraps between the nine or 99 enabled slots. Legacy raw states can be copied
  into an unused slot and loaded without an image until resaved.
  Files use `.ffstate`, carry ROM/BIOS identity and a versioned machine snapshot,
  and are separate from cartridge `.sav` files. Loading also rewinds cartridge
  memory, which normal autosave/exit later persists. Native translations and host
  resources are not stored; input recording must stop before loading.
  Undo last operation is one RAM-only session entry across all slots: undo save
  restores the previous file/preview (or an empty slot), undo load restores the
  pre-load machine. Failed operations preserve the entry; successful undo
  consumes it without redo. A detected externally changed/missing slot prevents
  undo save. Load and undo load both use the shared frontend restore reset.

- [Restart ROM](../restart_rom.md) in the pause menu flushes current cartridge
  memory and transactionally rebuilds the startup machine in the same window.
  It resets frontend timelines and slot Undo while preserving slot files and
  startup backups. Failed writes and active recording keep the paused session.
  Clean boot retains the current immutable decoded-resource owner without file
  I/O, shared with backup recovery.
  Deployment laws now recognize the distinct WIN0 compact table and WIN1 HELP
  from user slot3, keeping both columns/selector intact and HUD separate.

- [Cartridge save backups](../save_backups.md): capture the selected `.sav` at
  desktop startup, retain two raw copies under `SaveBackups/<save-filename>/`,
  and restore via the controller-operated Save backups menu. Sequence/hash
  filenames identify order and integrity. Restore prepares a clean boot from
  the session's startup snapshot before atomic file replacement, resets the
  machine and frontend, and clears save-state undo. It preserves both backup
  copies and the nine state slots. Backup failure aborts before gameplay writes.
  Missing remembered/default primary files keep their recovery location when
  recognized backup copies are present, rather than selecting an unrelated save.

## Validation knowledge and remaining uncertainty

- [Debug save cases](debug%20saves.md) uses stable DS-01 through DS-26 IDs and
  distinguishes observed coverage from candidate snapshots and missing scenes.
  The ignored `saves/debug-cases/` collection follows the same slot numbers;
  its local catalog preserves source hashes and flags partial cases explicitly.
  Expanded imported sources and ordinary derived checkpoints live under ignored
  `debug_saves/ready/`, split by USA/EUR. IN source numbers belong to standard
  save selection; DS case numbers belong to snapshot selection. Its local
  catalog distinguishes wrapper region hints, verified runtime region, payload
  duplicates and untested internal game File No.2 entries. Original containers,
  converted SRAM, screenshots and snapshots remain outside Git/packages.
  Additional user snapshots retain their original bytes under descriptive
  ignored names; the local catalog records relocation, ROM/BIOS and proof scope.
  DS20 now has a verified EUR failed-dispatch return at slot20, with a nearer
  precursor at EUR65.
  Derived standards use GEN IDs:30/31 for map progress,33 for an ordinary
  Save Now suspended battle verified through Resume Battle. Retained masters
  live outside the working family in ignored `debug_saves/generated/`.
  NEW-32 is factory-empty memory for naming checkpoints: choose that standard
  slot before loading DS23, since snapshots also restore/flush cartridge memory.
  Snapshot slots51–64 are explicit variants, separate from stable DS IDs.
  Clan-threshold prizes and the originally slow enemy turn still require their
  own evidence; related mission rewards or a normal turn do not close them.
- [Desktop performance TODO](PERFORMANCE_TODO.md) tracks task completion separately
  from partial progress. Its first P0 tranche adds native Linux/Windows/Wine
  comparisons, optional final serialized-state verification, bounded headless
  frame-time capture and opt-in interactive CPU phase profiling, including
  separate CPU-side presenter setup/upload/acquire/encode/submit/present times. See the
  [measurement workflow](PERFORMANCE_WORKFLOW.md). These tools do not establish
  an AI-turn speedup; CPU submit timers alone do not measure GPU execution.
  Optional render-pass queries now provide a separate GPU series. The frontend fails before advancing
  temporal history on an exhausted frame guard; stats display separate native
  block/fallback-step counts rather than their mixed-unit percentage.
  Detailed PPU attribution and an observed enemy-activation replay are complete;
  reuse them through TASK_HANDOFF. Remaining P0 work includes the originally
  reported slow-turn scene, controlled realtime GUI overhead and broader
  hardware/native Windows coverage. Frame-window gating, raw enhanced source
  counters, local PipeWire ERR observation and presentation cadence are already
  implemented. They do not alone establish audible underruns or every hardware
  XRUN. The bounded
  unpaced replay mode now provides controlled work-overhead comparisons. Preserve completed recovery
  and allocation/upload substeps rather than restarting them. GPU LUT/temporal processing is conditional on
  measured cost; preserve per-guest-frame history and the CPU reference path.
  Guest AI/GPU or full-PPU rewrites are not the first-line approach.
- The observed enemy replay has a dispatch census distinct from introductory
  observer validation. Missing entries/guest cycles do not isolate host time.
  A pre-panel-cache native perf sample placed 31.52% of leaf samples in
  capture_dialogue_part and guided panel-membership caching below. Preserve
  direct lookup/interpreter hooks without specific evidence; do not reopen
  completed window caches or rejected selectors from this historical sample.
  Profiling binaries/logs are isolated; see the performance workflow.
- Extra immutable ROM seeds can preserve every image while changing audio state.
  The recorded18-seed coverage trial is rejected: first full-state difference
  at relative38/guest8514 affects mixed audio and HLE verification, with equal
  CPU/clock; the diagnostic reproduces both original final hashes. A ROM-free
  timer fixture also exposes mixed-output dependence on tick batch size. Event
  chronology now has a core correction with single-cycle/DMA/skip regressions:
  scheduled events are serviced before finishing a tick batch, and DMA steals
  extend CPU work without video reentry. Native/Wine179core and66native runtime
  checks pass;3530old frame/OAM/counts and native load/Undo/reload/close pass.
  Corrected extra-seed coverage still diverges in CPU/clock at212 and remains
  rejected. Timing/export gates for the core correction are tracked in the
  handoff; retain strict per-build full-state/audio checks. No schema/ABI change.
  Five pairs measure+0.066607ms (+1.157%) mean-work cost for correctness, not a
  speedup. Current Docker exports pass349Windows/Wine and350Linux tests,15optional
  ignored each; archive/compatibility and actual Linux cold-cache GUI/replay
  gates pass. Prior archives preserved; canonical native builds also updated.
- CPU PPU timing uses scoped thread-local capture outside machine/save state.
  `FFTA_PROFILE=1` includes scanline/zoom/core-remainder rows on desktop;
  `--ppu-profile` enables separate headless diagnostic runs after warm-up.
  Disabled entry points check capture state without reading a clock. Capture
  is bounded, incomplete frames are discarded and parallel zoom measures the
  caller's wall time including worker waits. No guest or translation ABI change.
- Native UI extraction caches window registers/vertical membership, modal flag
  and native origin only within one `capture_dialogue_part` call. WIN0 overrides
  WIN1, then OBJ window and outside; horizontal/vertical wrapping is unchanged.
  Values are reread on each call, so DMA/scanline changes and load/Undo need no
  persistent cache invalidation. No serialized guest state or ABI changes.
- Native panel area/banner rules are snapshotted per capture call and lazily
  evaluated once per eligible pixel across BG candidates. Missing bounds remain
  unevaluated under overriding banners or absent BGs. The uncached test policy
  and full state comparisons preserve clipping/blending/sort semantics. Native/
  Wine173core tests pass;3530frame/state parity includes zoom6. Five pairs show
  2.69% less core work and2.26% less complete unpaced GUI work on native Linux;
  worldmap/default GUI checks pass. No guest/cache ABI or Windows-speed claim.
  Final Docker335Wine/336Linux tests,15optionalignored, archive compatibility/
  privacy and cold-cache actual Linux portable GUI pass. Local dist is updated;
  prior archives preserved. Subsequent world-only pixel work is also retained
  and verified (see Desktop behavior); it is not a next candidate to repeat.
  PERF-03 correctness controls and PERF-08 CPU PPU reductions meet their TODO
  criteria. Remaining host cost or an open parent does not reopen closed trials.
- Native BG2/BG3 UI planes are prepared once per hardware scanline and borrowed
  by primary/extra and normal/backing extraction. Backing text filtering uses
  a private copy; Stats' zero mask uses transparent planes. Scratch lifetime
  ends at the scanline, so no cross-frame/write/restore invalidation is needed.
- Native UI rows with transparent text/backing and no portrait candidates clear
  their destination and bypass world-candidate sorting/blending. Allocation,
  anchor updates and capture-flag cleanup precede the check. Nonempty rows keep
  the full pixel loop; that loop is also the test-only const-generic reference.
  No hardware scanline, CPU step, guest cycle or interrupt is skipped.
- `FFTA_PPU_STAGES=1`, alongside the existing PPU profile, adds optional disjoint
  per-frame BG/OBJ/composition/UI capture/setup/residual totals. The flag is read
  at profiler creation; detailed clocks stay off otherwise. Nested UI preparation
  belongs to capture, and the totals partition scanline time. The comparison
  parser accepts older baselines without these optional rows and validates any
  supplied stage report. See the performance workflow for scope and overhead.
- Headless `runc --state` and the comparison script can resume desktop slots
  or legacy raw snapshots through the menu validator. Snapshots and cartridge
  `--sav` inputs are mutually exclusive. Replay length, input, zoom and warm-up
  use a relative origin while guest frame/clock values remain intact; existing
  no-snapshot replays retain boot origin zero. Comparisons check loaded machine
  identity and exact absolute frame ranges, and keep loading outside timing.
  Inputs and personal snapshots stay local and read-only. See the performance
  workflow for the diagnostic CLI and its remaining AI-scene coverage limits.
  To record from a supplied battle snapshot, use an isolated in-game suspend
  save, then resume during a new recording session; checkpoint saves are allowed
  while host loads remain prohibited. Rebase subsequent events to the checkpoint
  origin and visually verify the enemy action and the replay endpoint.
- `runc` guest-cycle profiling (`RECOMP_COST_FROM`) observes the shared native
  dispatch loop, preserving normal HLE/interpreter gates and strict fallback
  behavior. `COST_DISPATCH` separates native, missing, system and intentional
  hook dispatches. Its per-PC charged cycles are guest time, not host timings;
  keep the census disabled for timing comparisons. See the performance workflow.
- [Windows runtime performance](../windows_runtime_performance.md) documents
  the tested replay and matching framebuffer/OAM results. Timing improvements
  were measured under Wine, with environment lookup costs specific to that
  setup; they do not establish native Windows gains or AI-turn performance.
- Windows GNU tests and interactive FPS/menu checks ran through Wine. Menu
  gamepad smoke checks used a simulated XInput controller. Native Windows and
  physical controller coverage remain to be obtained.
- Check host load and the pinned CPU's SMT sibling before timing. VS Code npm
  discovery can follow ignored build/Wine test trees and saturate the host; the
  root Rust source tree has no npm manifest. See the performance workflow for
  the observed confounder and local workspace-only `npm.autoDetect` workaround.
  `.vscode/` remains ignored; no global editor settings are required.
- Some gba-core tests share display globals. Use serial execution for that
  suite; distinguish a baseline parallel failure from a regression.
- Optional TinyCC observer checks need its ELF relocatable object loader even
  for Windows. Record the backend and checks actually used.
- Keep test saves/config separate. Wine may replace APPDATA supplied by the
  caller; verify the effective location within the isolated prefix.
- Local `target/` and `dist/` files are ignored build artifacts, not portable
  prerequisites. Discover installed toolchains and rebuild when absent. Never
  include ROMs, saves, recordings or test controller DLLs in release archives.
  [Local artifact retention](LOCAL_ARTIFACTS.md) requires closing successful
  managed/native units with `scripts/local_artifacts.py complete`: retire only
  disposable interrupted/moved fixture copies after preservation/results, retain
  accepted offline data and proof logs, and explicitly review old test binaries.
  Refuse links/private/tracked content. Historical mutable-output maps are not
  perpetual live-input requirements; cleanup receipts describe availability,
  without reopening completed proofs. Coverage Docker launchers keep two owned
  automatic cache generations after successful publication; explicit datasets
  remain untouched and read-only direct generation defaults to no eviction.

## Native UI coordinate policy

Native UI GPU draws now use cached RG32Uint axis maps computed with the existing
CPU pixel-centre/floor/clip/clamp formulas. This avoids backend-dependent float
reassociation selecting adjacent texels at fractional world boundaries. Keys
include exact layout/clip/view bits, world dimensions and the device limit;
resource recreation invalidates maps. Conservative source bounds use the exact
transparent-pixel predicate and integer-map-derived scissors. Invisible layers
still claim shared world sources. No guest timing, snapshot or zoom/FOV change.

Game UI layers also receive the current Grid parameters and share
`crates/screen/src/grid_filter.wgsl` with the world pass. Grid phase and
small-scale fade follow the UI's own layout/Size, not world zoom; the original
coordinate maps still define centre samples and transparent coverage. FPS
overlays explicitly opt out. Software fallback and the pause canvas remain
grid-free. See [dialogue layout](../dialogue_layout.md).

The explicit Radeon tests match CPU for fractional crops and 32 persistent-resource
zoom/resize/DPI states. The original table-only prototype regressed 4.17% and
was rejected; bounded rasterization plus the maps improves the five-pair native
unpaced replay median mean by 5.53%. World-map correctness, native GUI
load/Undo/reload/close and physical Radeon Wayland replay pass. Final exports pass 332 Windows/Wine and 333 Linux tests (14 optional ignored
each), archive privacy/integrity and PE/GLIBC compatibility checks. The actual
Linux archive passes cold-cache GUI load/Undo/reload/resume/close with inspected
captures. Evidence is tracked in the [record](../changes/2026-10-21-align-native-ui-sampling.json).
This is not an AI-turn or native Windows performance claim.

## Maintaining this knowledge

Platform-specific root-workspace scripts are grouped under `scripts/linux/`,
`scripts/windows/`, `scripts/android/` and `scripts/unix/`; shared tools remain
directly under `scripts/`, with tests under `scripts/tests/`. See the
[script directory guide](../../scripts/README.md). AndroidPort keeps its separate
script tree.

Follow [the memory workflow](MEMORY_WORKFLOW.md) and the local skills routed
from [AGENTS.md](../../AGENTS.md). Update this page when facts or decisions
change, rather than adding a second chronological log here.
