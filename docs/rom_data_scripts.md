# European script command directory

The bounded EUR source projection contains 115 six-byte command records at
`3acebc..3ad16e` (690 bytes). Each has a little-endian u16 stream advance and an
unaligned little-endian u32 callback at +2. All 115 callbacks are distinct, non-null
ROM Thumb entries. Their low bit is preserved for the original BX trampoline;
they differ from the even MOV-PC dispatch words in the
[effect/event directory](rom_data_events.md).

## Boundaries and original consumers

The original interpreter at `081265cc` reads a signed 16-bit stream cursor,
adds it to the live stream pointer, reads an unsigned byte opcode and computes
`3acebc + opcode*6`. Four byte loads at `081265e4..081265f2` assemble the callback.
The call goes through `08145c14` (BX r3). The byte loads at `08126600/08126602`
assemble the advance, which is added to the live cursor and stored as a halfword.
A callback result of `0x110` repeats the interpreter loop; other results return.
None of this control flow is replaced by the host model.

Two independent callbacks root a different byte lookup at `3ad16e` through
literals `1263c4/126468`. This bounds the 115 command records without inferring an
extent from the appearance of callback pointers. An initial 116-record candidate
ending at the existing selector root `3ad174` was rejected before the source plan
or model was created: its last record would include the distinct byte lookup.
The lookup and intervening bytes remain outside this projection; its complete
semantic domain has not been established.

Relative commands 0 and 1 separately read the same advance field, add their stream
operand and truncate the target to 16 bits before the original call. These readers
are at `08126628..0812664c` and `08126664..08126688`.

The source projection is not a guest opcode bound. The original unsigned byte
opcode arithmetic has no local 115-entry cap; values 115..255 must retain reads
from the existing owner or raw ROM. The signed cursor, u16 wrap, callbacks,
return values, live operands, waits and sequencing remain original. Observed
advance values 1..10 do not create a new allowed-value restriction.

## Typed model and current scope

`rom_data::script_commands` stores a canonical opcode, u16 advance and typed
Thumb callback per record. Strict JSON rejects unknown fields, overflowing
integers, reordered or missing records and malformed callback offsets. It
reconstructs all 690 source bytes without keeping a ROM-byte shadow. Source
authentication requires the exact European ROM, 106 original-instruction halfwords
and six root/successor words, plus exact source reconstruction and SHA-256.
The range is disjoint from all 43 published e49 resource owners; this directory
has no asset dependency because its callbacks refer to executable code.

A warning-free Windows test build and one corrected Wine source oracle pass:
117 callback-load prefixes, 117 advance prefixes (including raw opcodes 115/255),
115 BX transfers, four signed-cursor prefixes, ten relative-target prefixes and
five callback-return loop branches. All 690 owned source bytes are observed at
original load sites. Seven shape, four source-edit, six JSON and two ROM-identity
guards pass. The 2,905 instructions and 67 PCs count coverage only.

The first Wine attempt failed a test expectation: the original ADDS instruction
at `12660a` writes r0, while the test expected the sum in r1. Only those expected
register values were corrected, then the affected Windows test binary and the
same new filter were rerun. The failed evidence is retained. No production code
or typed-model correction was needed, and no old family proof was repeated.

The source/model criterion above is complete. Managed publication/direct guest
consumption are implemented below; native integration, production CLI and
positive ordinary use have separate completed checks below. USA stays at e8-d7. New Linux,
USA and native Windows checks remain pending. No performance or space
measurements are part of this work.

Private, reproducible local evidence is under `target/rom-data-script-commands/`:
the retained disassemblies, `source-contract.json`,
`static-source-verification.json`, `windows-model-fixed-result.json` and
`windows-model-fixed-europe/result.json`. Game data is not included in tracked docs.

## Managed publication and direct command fields

EUR e50-d49 adds `script-command-directory.json` / `engine.script-command-directory`
as file45/resource44, retaining all43previous decoded files and the e49generation.
The directory has no asset dependency; the resource set also requires the
preceding effect/event owner before activation. One authenticated690-byte source
binding serves the original advance and callback loads from typed commands,
without a ROM-byte shadow. Pages3ac/3ad merge the preceding event bytes, following
selector bytes and raw lookup gap at mixed access boundaries.

Original stream selection, signed cursor, unsigned opcode arithmetic, relative
targets, u16 advance wrap, callback BL/BX and return-controlled loops remain guest
instructions. Save and native ABIs do not change. Aggregate advance/callback
bytes and command-use counters remain outside machine serialization. The ordinary
scene assertion requires actual reads of both advances and callbacks.

A warning-free Windows build and two reduced Wine filters pass on the first run.
Managed validation covers13coherent/source-set guards, failure-atomic publication,
create/reuse/repair/move, all43prior decoded files/e49 retention, transactional
missing-file activation and resident ownership through unavailable files, Load,
Undo, backups and Restart. All690typed bytes and115commands agree with source
while the isolated machine's raw source is altered.

Native validation passes26original/interpreter/native complete-state/clock
prefixes, including raw115/255, signed cursors, u16 advance wrap, relative targets,
BL/BX call prefixes and callback-return loops. Compiled API reads account for
14advance/28callback aggregate bytes. Adjacent old event4B/selector4B remain typed
with their raw sources altered; previous2+new2/new2+raw2 accesses, all widths,
ROM mirrors, real BIOS policy, CpuSet/FastSet and DMA pass.314instructions/70PCs/
32native dispatches count coverage only. The prefix endpoint guard is reused;
production emitter and source model remain unchanged. Ordinary scene acceptance is recorded below, independently of the source/model
and targeted checks above. No closed source or older family proof is
repeated for context; Linux/USA/native Windows stay pending.

## Production CLI and positive ordinary scene

A warning-free Windows production build and reduced Wine actual CLI checks pass:
create/reuse without writes of47managed files, an unrelated working directory,
and loading the existing native cache. All45generation files match the accepted
e50dataset. The canonical executable is restored byte-for-byte.

One clean boot frame first establishes activation/cache compatibility with zero
command reads; it is not ordinary script acceptance. A later locator reuses the
genuine retained EUR suspend boot-to-battle input and save through8476frames at
classic240x160. It positively reads42command bytes, comprising14advance and
28callback bytes from7commands. Visual dialogue or an idle snapshot alone did
not establish this consumer; the source-use counters do.

One Wine fullpair from that actual CLI-published e50generation then passes all
8476complete states/clocks/central-wide image buffers/OAM/dispatch and every
mixed/PSG/FIFO/bitwise HLE audio sample, with the same42bytes/7commands. Audio
sample counts are18599928/18599928/2236876/2236876/18578590. The already built test
executable, native cache, input and suspend save are reused. No source/native/
lifecycle/CLI acceptance is repeated and no new fixture/navigation/payload edit
is needed. The other108commands remain targeted-only; this does not establish
a whole callback/interpreter ABI or new32:9scene acceptance. The preceding event/
selector owner reads zero bytes, so its ordinary criterion remains open.

Private results include `windows-build.json`, `windows-cli-summary.json`,
`windows-cli-no-write.json`, `windows-cli-battle-entry8476.json`,
`windows-scene-result.json` and `windows-battle-entry-script-full-state-audio.log`.
Original data, prior generations and316hash-mtime guards remain unchanged.

The producer audit confirms1265cc←125b6a in manager125ac8, dynamically assigned
through literals125ed4/125f10. Producer125e8c resolves the relative script archive
atB946EC using an event key returned by08009aa0 (sourceD68C2C). The complete archive
and event-key projections are not yet migrated; their domains/bounds are a new
inventory task. A read-only audit of63distinct retained EUR transitional boot
snapshots had queued root0 and did not establish an active context by itself.
