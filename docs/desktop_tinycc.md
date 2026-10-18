# Desktop in-memory compiler, 2026-10-16

Windows and Linux now use the same TinyCC source revision as Android:
`43c7708b85681a2fd4451c8a541af4494a8919b2`.

`recomp-core::tcc_memory` compiles C strings directly in RAM, writes one
relocatable ELF object to an atomic cache file, and relocates it in memory
on load. The owner keeps libtcc and the executable allocation alive. No C
files or external compiler processes are needed for this backend. ROM
analysis/profiling still happens before compilation, as on desktop before.
This does not change the sprite, background, audio or input implementation.

The default compiler is `tcc/libtcc.dll` or `tcc/libtcc.so` beside recomp.
`FFTA_TCC_LIBRARY` selects a compatible replacement explicitly. Its adapter
ABI is checked, not its hash. `GBA_RECOMP_CC` selects an external compiler
instead. GCC shared libraries still load normally; ELF relocatable caches
use the TinyCC loader. Desktop caches use a separate `-tccmem1` family,
preserving the old compiler's cache. No ROM/save is distributed.

## Measured Windows results

The commands used the user's ROM read-only and wrote generated artifacts
only under ignored `target/tinycc-proof`. Timings are from this PC, and are
not a promise for every device. Other builds were running during some tests.

| ROM | Blocks | TinyCC emission/compile/cache | Whole build including profiling/analysis |
| --- | ---: | ---: | ---: |
| Europe | 28859 | 0.864 s | 3.611 s |
| USA | 26091 | 0.990 s | 4.706 s |

USA GCC `-O1`, same `build --ram` entry point: 116.634 s whole build.
EUR TinyCC cache load: 0.079 s. Both ROMs produced identical framebuffer
and OAM hashes against the GCC reference for every one of 600 boot frames.
USA extra=0 also matched for 600 frames with BG/SPR patches off versus on.

Existing test suite: 141 passed, no failures. Two additional ignored tests
were explicitly run against the bundled DLL and passed: native callback
ABI plus a 16 KB stack frame after object cache round trip, and compile
error cleanup without retaining the compiler mutex.

The available `movement-pan.input` was replayed from frame 0 through 12540
in 32:9, with `ffta-respaldo.sav` copied to a test-only save. TinyCC and GCC
matched all 1242 recorded metric/hash rows and final native/fallback counts.
IMPORTANT: the old character metric reports 4 in BOTH runs for all 1201
frames 11340..12540. This comparison proves no compiler-dependent difference
in that recording; it does NOT validate the eight-character acceptance
criterion. The header-only original batalla-3 input cannot reproduce it.
No sprite code was changed to conceal this result.

## Build and distribution

`TinyCC/build.py` verifies the exact source archive SHA256, supplies platform
settings, and builds the replaceable library. Windows links TinyCC's own
stack helper; the system/MSVC helper is not ABI-compatible. Windows library:
419937 bytes. Complete upstream source archive: 1225983 bytes.

`build-all.ps1` no longer downloads w64devkit. Windows/Linux payloads carry
`tcc/` plus `TinyCC/` (license, exact source, adapter and rebuild script).
An old payload/toolchain directory, if present, is warned about, not deleted;
the installer skips it when a new tcc directory is present, and Linux TARs
exclude it. The source folder must accompany distributed binaries or be
offered alongside the same release as a corresponding-source download.

Linux x86_64 was cross-compiled for glibc 2.17 and packaged successfully.
Runtime speed and audio on Linux: NOT MEASURED on this Windows host; WSL is
not installed. Test the TAR on the user's Arch laptop.

Useful commands (from repository root):

```powershell
python TinyCC/build.py --cc E:/msys64/mingw64/bin/gcc.exe
cargo build --release -p recomp -p installer -p ffta-play
$env:FFTA_TCC_LIBRARY = (Resolve-Path target/release/tcc/libtcc.dll).Path
cargo test -p recomp-core tcc_memory::tests -- --ignored --nocapture --test-threads=1
python scripts/compare_native_backends.py target/gcc-600.log target/tinycc-proof/eur-600-tcc.log
```

MIR was removed from the active desktop path; only earlier ignored diagnostic
build artifacts under target remain. No commits were made.
