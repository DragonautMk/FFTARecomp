# Graphics producer discovery and bounded LZ77 decoding

The next resource work unit extends the OBJ metadata prototype toward actual
graphics. It provides a bounded offline LZ77 decoder and verified early-boot
source ranges. It does not add graphics files to the active dataset yet.
The [inventory](development/ROM_DATA_INVENTORY.md) and
[full TODO](development/ROM_DATA_TODO.md) retain the remaining catalog,
semantic decoding, extraction and runtime integration work.

## Observed producers

An optional test follows 1200 clean-boot frames with standard demo input, without
a save or forced PC, independently on each exact supported regional image.
It records actual executed BIOS decompression instructions through `StepEvent`,
not a guessed PC before interrupt dispatch. ROMs are read-only and all output
goes into a new private directory. The producer trace and post-SWI destination
bytes match byte-for-byte between Linux and Windows GNU/Wine for each region.

Each trace observes four LZ77 WRAM calls (SWI 0x11), no other decompression SWI:

- USA frame 801: source 0x56f4a8, 125 consumed bytes -> 160 decoded bytes at
  IWRAM 0x3860. Frame 803: 0x5704cc, 133 -> 456; 0x56fefc, 1481 -> 1996;
  0x56f52c, 2507 -> 2960, each into EWRAM 0x3cb0 scratch at its respective
  call. The source extents exclude padding.
- EUR frame 1161: 0x76c64c, 125 -> 160 at the same IWRAM destination.
  Frame 1163: 0x76d670, 133 -> 456; 0x76d0a0, 1481 -> 1996;
  0x76c6d0, 2507 -> 2960 at EWRAM 0x3cb0.
- The actual wrappers are 0x08141890 USA and 0x08145254 EUR. Caller link
  registers are 0x0801a4a9/0801f519/0801f3f1/0801f295 USA and
  0x0801b619/08020689/08020561/08020405 EUR, in the order above.

The four compressed streams and decoded outputs match across the two regions:
4246 consumed source bytes and 5572 decoded bytes per region. Decoded SHA-256,
in call order: 2a360ec828fb53a6becc94e8f1c3aa9c6044821639c44ad393b97c66ba0a6b44,
8f768d8eda371a2fa1a5fbe5b72a36f61f286e40f14088730013fdb44d824235,
3e9599c722762edd19e059a5d0bbb51e1eb907f94c44e17ebe7d8445df54429f,
3d379876034eec8b762e05ad7b8d987007b5c89d3369c0c5e05b3b8e2ccabc42.
These are exact stream-consumption bounds derived by the decoder, not a claim
that padding belongs to the payload or that every graphics source was visited.

Independent original caller disassembly also identifies a shared relative
descriptor layout: 88-byte stride, root USA 0x569104 from literal 0x1a4ac and
root EUR 0x7662a8 from literal 0x1b61c. Field 0x0c in record 0 resolves the observed 160-byte
output's source. Other record fields, total record count, selectors, raw
alternatives and the downstream interpretation remain to be verified.
Do not infer catalog or language completeness from equal initial outputs.
Scratch output preserves any packet headers; it is not declared to be a flat
tile image or a decoded map without following its subsequent consumers.

## Decoder contract and checks

`gba_core::rom_data::lz77::decode` accepts a bounded input slice and explicit
output budget. It requires type 0x10 and a nonzero 24-bit decoded size, checks every
input read, bounds allocation before decoding and rejects references before
the output history. Overlapping matches and a final match clipped to the
declared size follow the existing BIOS HLE contract. The result contains exact
decoded bytes and consumed header/stream length, leaving trailing padding to
the caller. The BIOS HLE itself retains its existing malformed-reference
behavior; strict offline decoding refuses such data rather than treating it
as a supported asset.

Four synthetic tests pass on Linux and Wine: literal/overlapping matches with
exact consumption and guard bytes against both BIOS 0x11/0x12 variants, partial
final tokens/flag groups, all truncated prefixes/invalid history/type/size and
the maximum 4096-byte history distance. Both target test builds are warning-free.
Separate optional tests use the already captured traces and destination dumps
to check all eight regional source ranges against actual guest outputs, on
both targets, with a 2 MiB output budget. No boot replay is required for these
offline decoder checks; original ROM, traces and outputs remain unchanged.

Private evidence lives in ignored `target/rom-data-graphics/`: per-target build
logs/executable metadata, core decoder test logs, regional calls.toml and output
captures, lz77-verified.toml reports, cross-target checks and helper results.
An initial observer compile typo was corrected before tracing. A stale native
Cargo library tracked the previous isolated measurement source rather than
the edited checkout; its original fingerprint/logs are retained, and only the
affected library fingerprint was invalidated. Successful traces and earlier
measurements were reused, with no reference-source mutation.

## Reproduction and remaining work

Build the `recomp` test executable using the existing Windows/native build
setup. With `FFTA_STATE_TEST_ROM` selecting an exact supported image,
`FFTA_WIDESCREEN=0`, `FFTA_STATE_TEST_FRAMES=1200` and an absent
`FFTA_STATE_TEST_TRACE_ROOT`, run the ignored test
`rom_data::compression_tests::observe_boot_graphics_decompression` with
`--exact --ignored --test-threads=1 --nocapture`. It requires an explicit
1..12000 frame count and refuses an existing output directory.
Then use that existing trace root and ROM with
`rom_data::compression_tests::decode_observed_lz77_matches_guest_outputs`.
The latter refuses to overwrite its completed verification report.
The four self-contained core tests use filter `rom_data::lz77::tests`.

Next establish the descriptor catalog bounds and downstream packet formats,
including palette/tile/map sharing and regional/language selection, before
adding source-authenticated graphics resources to extraction/preparation.
No BIOS call, guest instruction, cycle, DMA/IRQ, audio path, save/native
identity, current dataset schema/revision or default startup behavior changes.
This work gives no decompression speedup or native Windows result. Windows
hardware is unavailable to the user; Linux/Wine remain distinct evidence.
