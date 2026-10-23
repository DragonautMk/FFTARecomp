# Decoded ROM resources

The desktop developer CLI extracts, verifies and prepares OBJ dimensions, BG
descriptors, base/alternate palettes and `graphics.bg-packet-programs` from an
exact supported source profile. New packet integration is verified for EUR on
Wine; Linux/USA checks remain pending under the current user priority. `play --rom-data` directly consumes OBJ dimensions and
the authenticated BG descriptor metadata and base-palette banks. Palette BIOS
HLE reads decoded banks; CPU/DMA/native and guest BIOS reads use an exact
file-derived source view. Alternate palettes are source-verified during
activation and directly serve the original custom decoder: literal source
bytes come from typed banks, while headers/tokens retain authenticated metadata.
No guest decompression instruction or write is skipped. EUR packet programs now
serve source literals from typed words and existing LZ77HLE body writes from
the immutable decoded model, reporting `runtime=decoded`. USA remains
`runtime=verified-only` until its consumer checks.
EUR additionally prepares `graphics.bg-main-graphics` as the seventh fixed
file introduced under `v1/e9-d8`; its retained typed words now serve custom-codec literal
reads with `runtime=decoded`, preserving original control bytes and guest work.
EUR `v1/e10-d9` added an eighth `graphics.bg-transfer-programs` file;
its typed controls and tile words now serve CPU/DMA/native reads with
`runtime=decoded`, preserving original frame countdowns/flags and partial DMA.
EUR `v1/e11-d10` added a ninth `text.short-strings` file with535strings
and65referenced glyph slots/widths, now directly consumed as `runtime=decoded`.
EUR `v1/e12-d11` added tenth file `text-catalogs.json`/`text.catalogs`,
authenticating16100pointer strings,145relative archives/17035programs and128
referenced glyphs, now directly consumed as `runtime=decoded` through original
CPU/DMA/native readers. The earlier overlapping ninth-file subset remains
authenticated and reports `verified-only` when the full catalog owns its reads.
EUR `v1/e13-d12` additionally prepares and consumes `font-bank.json`
(`font.complete-bank`,3175packed glyph slots/metrics) and `name-keyboard.json`
(`text.name-keyboard`,three original programs, navigation widths and cells).
The complete bank owns overlapping earlier font reads; original guest work/clock remain.
EUR `v1/e14-d13` added [static OBJ lists](rom_data_obj.md): 66 directly
rooted lists/160objects, with typed coordinates, modes, transforms, tiles,
priorities and palette indices. Typed guest/native/host reads retain original producer work.
EUR `v1/e15-d14` added indexed [OBJ animation banks](rom_data_obj.md#indexed-ui-animation-banks):
ten pairs/167lists/95sequences/184frames with relative indices, OBJ fields and
frame durations. All14files/13resources are source authenticated before activation;
original constructors and live frame/tick work consume the retained typed owner.
EUR `v1/e16-d15` extends the static catalog to 73 lists/173 objects
from 123 checked references. All original 66 records and other decoded files
are retained; the fourteen-file contract is unchanged. Focused Wine checks and
a positive 240-frame language-screen/actual CLI pair pass.
EUR `v1/e17-d16` adds an eleventh OBJ pair with five proved
nonrepeat zero-count holds: explicit list selectors and duration-source aliases
preserve the original count and shared typed owners. Previous ten-bank records
and all other decoded files remain unchanged. Focused Wine source/native/host/
lifecycle checks and a positive 240-frame Party/actual CLI pair pass.
EUR `v1/e18-d17` adds five language-selected OBJ bank pairs and two
typed selector tables, bringing the indexed catalog to 16 pairs/317 lists/
105 sequences/328 frames. Every prior bank record and other decoded file is
retained. Focused Wine original/native/host/lifecycle checks and one positive
400-frame “Engage!” full-state/audio/actual CLI pair pass.
EUR `v1/e19-d18` adds fifteenth file `obj-graphics.json`, resource
`graphics.obj-custom-banks`: 39 custom decoded word banks and six ordered
language selector tables. All 15 files/14 resources are authenticated; every
prior decoded file remains byte-identical. Original decompression, live scratch
transforms, partial uploads and RAM tails keep executing with the same clock.
See [the custom OBJ graphics contract](rom_data_obj.md#custom-obj-word-banks-in-e19).
EUR `v1/e20-d19` adds sixteenth file `ui-palettes.json`, resource
`graphics.ui-raw-palettes`:21raw UI banks/1328full-u16colors for original BG/OBJ
shadow copies, including two Color-Mode selector paths. All16files/15resources
are source-authenticated; all14earlier decoded files remain unchanged. Original
copy/fade/publication work and live palette RAM remain active. Reduced Wine
source/native/lifecycle and a positive240-frame boot/CLI pair pass. See the
[raw UI palette contract](rom_data_ui_palettes.md).
EUR `v1/e21-d20` adds seventeenth file `ui-graphics.json`, resource
`graphics.ui-custom-banks`:nine custom banks/eight BG sources/one OBJ source,
57038encoded bytes/84160decoded bytes and ten original decode/upload paths.
Typed literals preserve original decoder-returned counts and partial/full
copies. All17files/16resources authenticate together and all15earlier decoded
files stay byte-identical. Reduced Wine original/native/lifecycle and one
positive320-frame Clan full-state/image/audio/actual CLI pair pass. See the
[custom UI graphics contract](rom_data_ui_graphics.md).
Current EUR `v1/e22-d21` extends UI graphics to16banks/two language selectors
and two links to existing OBJ banks,26ranges/71474source bytes/119168decoded
bytes. The seven new banks and ten pointer literals add17ranges/14436source
bytes. Both original fallback rules and split5056OBJ/2688BG uploads remain.
All15other decoded files, nine old UI records and e21generations stay unchanged,
with17files/16resources. Focused Wine source/native/lifecycle checks pass;
new-source and selector counters distinguish actual language payload use. One positive
1200-frame English intro full-state/image/audio/actual CLI Wine pair passes;
other ordinary language scenes remain pending.
USA retains its six-file `v1/e8-d7`
contract; new Linux/USA checks remain pending.
`frames` and `runc` support the same opt-in. The source ROM remains required for
code and other data; ordinary startup waits for the remaining ROMDATA-05 gates.
See [runtime consumption](rom_data_runtime.md) for sources, identity, counters,
snapshots, verified coverage and limitations.
The [TODO](development/ROM_DATA_TODO.md) tracks the remaining phases and
[inventory](development/ROM_DATA_INVENTORY.md) separates known readers from
unknown asset catalogs and formats.

```sh
recomp rom-data extract /path/to/your-game.gba target/rom-data/my-dataset
recomp rom-data verify /path/to/your-game.gba target/rom-data/my-dataset
recomp rom-data prepare /path/to/your-game.gba target/rom-data/managed-resources
recomp play /path/to/your-game.gba --rom-data
```

## Run extraction with Docker

Only Docker Engine/Desktop with Buildx is needed on the host. Rust, Python and
the native build/runtime dependencies stay inside the Linux container.
From the repository root on Linux:

```bash
bash scripts/linux/extract_rom_data_docker.sh
```

On Windows, start Docker Desktop using **Linux containers**, then run from
PowerShell (Windows PowerShell 5.1+ or PowerShell 7):

```powershell
.\scripts\windows\extract_rom_data_docker.ps1
```

Both scripts build the `extract` target in `Dockerfile.linux`, using the existing
pinned Rust/Bookworm dependencies and BuildKit Cargo caches, then run the Linux
CLI. There is no Windows cross-build or host Wine requirement for this flow.
The default source is
`<repository>/rom/1194 - Final Fantasy Tactics Advance (E)(Surplus).gba`, so no
ROM argument is needed. The dataset defaults to
`<repository>/target/rom-data/<ROM filename stem>/`. To select a different ROM
and output directory:

```bash
bash scripts/linux/extract_rom_data_docker.sh "rom/my-game.gba" "target/rom-data/my-dataset"
```

```powershell
.\scripts\windows\extract_rom_data_docker.ps1 -RomPath "C:\Games\FFTA.gba" -OutputDir "target\rom-data\extracted-europe"
```

The scripts work from another directory too: default and custom relative ROM
paths start at the repository root; absolute ROM paths are used directly.
Explicit output paths remain relative to the calling directory. The build
context and default output are relative to the script's repository. Paths with
spaces are supported; commas and newlines in ROM/output paths are rejected
before Docker invocation.
Use a local Docker Engine/Desktop whose daemon can access those host paths.

The ROM is excluded from the build context and mounted as a single read-only
file only at extraction time. Only the output parent is mounted writable so the
extractor can publish a complete dataset by atomic sibling rename. Extraction
runs without network access; the initial image build needs dependency downloads.
Linux containers run with the host user's UID/GID so exported files remain
user-owned. Both scripts print the final host path and the JSON/TOML file list.
Repeated invocations reuse the build caches and verified datasets; invalid
existing datasets remain untouched. Outputs are private game data, kept under
ignored `target/`. See [Docker extraction details](docker.md#extract-decoded-rom-data).

## Run extraction without Docker

From the repository root, with Python 3.9+ and the existing native build
dependencies installed:

```sh
python3 scripts/extract_rom_data.py "/path/to/your-game.gba"
```

On Windows, use `py` in place of `python3`. The script runs
`cargo run --locked --release --package recomp --bin recomp -- rom-data extract`;
Cargo builds the current CLI as needed and reuses its build cache. The default
output is `<repository>/target/rom-data/<ROM filename stem>/`, containing the
readable JSON/TOML files listed below. After success, the script prints the
absolute directory and the files actually present with a description of each.
These are typed data files, rather than rendered background/sprite previews.

To use a prebuilt CLI and select a directory explicitly:

```powershell
py scripts/extract_rom_data.py "C:\Games\FFTA.gba" --recomp "target\release\recomp.exe" --output "target\rom-data\extracted-europe"
```

For the Windows CLI on a Linux development host:

```sh
python3 scripts/extract_rom_data.py "/path/to/your-game.gba" --recomp target/x86_64-pc-windows-gnu/release/recomp.exe --wine
```

ROM, executable and explicit output paths are resolved relative to the calling
directory, so an absolute script path also works from elsewhere. `--recomp`
uses exactly the selected binary's extraction coverage. The script forwards CLI
progress/errors and preserves its failure status. Extraction itself validates
the complete dataset before publication or reuse; a repeated invocation reuses
valid data and refuses an invalid existing directory without overwriting it.
Choose a new `--output` directory after an incompatible format revision.
Keep extracted game data under ignored `target/` and outside release packages.

## Extracted files

The output directory contains `manifest.toml`, `obj-dimensions.toml`,
`bg-descriptors.json`, `bg-base-palettes.json`, `bg-alternate-palettes.json` and
`bg-packet-programs.json`, plus EUR's `bg-main-graphics.json` and
`bg-transfer-programs.json`, `short-texts.json`, `text-catalogs.json`,
`font-bank.json`, `name-keyboard.json`, `obj-static-lists.json` and
`obj-animation-banks.json`, `obj-graphics.json`, `ui-palettes.json` and
`ui-graphics.json`.
OBJ dimensions store twelve indexed width/height records;
BG metadata stores 162 typed 88-byte descriptors with relative source links,
transfer triples, selectors and retained unclassified bytes. Other linked graphics
remain in the ROM; alternate palette banks/links/keys now have their own
verified decoded file. Packet programs preserve352unique programs/486descriptor
links, parent initialization, ordered typed writes and exact source/body metadata.
The manifest records
schema/extractor/decoder revisions, exact source ROM identity, regional profile,
decoded coverage, file hash, dependency IDs and the original ranges/hashes.
Both regional direct tables and copied aliases are verified independently.
Extractor21/decoder20(EUR) and8/7(USA) store all162base palettes as readable JSON banks of16full
u16words, plus the exact consumed source representation for compatibility
reads. Source aliases and bit15are preserved. Earlier e7/e6/e5/e4/e3/e2/e1datasets
remain in separate namespaces. Manifest schema stays1; current formats are
OBJv1, `bg-descriptors-json-v2`, `bg-base-palettes-v2-json`,
`bg-alternate-palettes-v1-json`, `bg-packet-programs-v1-json` and EUR
`bg-main-graphics-v1-json`, `bg-transfer-programs-v1-json`, `short-texts-v1-json`,
`text-catalogs-v1-json`, `font-bank-v1-json`, `name-keyboard-v1-json` and
`obj-static-lists-v1-json`, `obj-animation-banks-v1-json` and
`obj-custom-banks-v1-json`, `ui-raw-palettes-v1-json` and
`ui-custom-banks-v1-json`.
Transfer programs preserve42frame-control programs/34shared tile banks and97links;
all162variant pointers are absent in this declared catalog.
Short texts preserve five107-entry language selector tables, exact compact/
extended codes and control bytes, and only the65referenced glyph slots/metrics.
Resource identity is independent of regional file offsets.
There is no ROM code, image, save, absolute installation path or timestamp in
the manifest. The generated decoded values remain private game data: keep
outputs under ignored `target/` for this developer command, outside packages.

Extraction writes and syncs complete files in a temporary sibling directory,
reads them back through the validator and then renames the directory into
place. Multiple publishers may reuse a complete, source-verified winner.
Repeated extraction reuses valid existing data. An invalid existing dataset
causes an error and remains untouched; choose a fresh output directory to
regenerate it. This explicit `extract` command preserves its original behavior;
managed preparation below adds automatic recovery. No mod or save directory is
selected by the offline commands.

Manifest/OBJ use strict typed TOML; descriptors, palettes and packet programs
use strict typed JSON, as do EUR main graphics. All reject unknown fields and apply the per-family bounds
listed below. The reader uses fixed
relative file names, rejects symlink files and
directories, and validates the complete manifest against the selected source.
Edited dimensions or descriptors remain invalid even if their file hashes are
updated in the manifest: base data must reconstruct the original source exactly. Mods
will need a separate loader and compatibility identity.

The commands print `ROM_DATA action=created|reused|verified|repaired`, source/profile,
schema and decoded coverage. `runtime=offline` identifies these offline commands;
OBJ, BG metadata, base and alternate palette gameplay each report `runtime=decoded`, with separate
source/file/dataset identities and usage counters. Packet programs report
`runtime=decoded` for EUR and `verified-only` for USA; EUR main graphics report
`decoded`, with all source/file data authenticated before activation.
This does not claim that sprite pixels, text, maps,
gameplay tables or audio have been exported or migrated.
The [bounded EUR short-text/font model](rom_data_text.md) now participates in
managed preparation, complete-set authentication and direct CPU/DMA/native
source reads. Complete text/font catalog and gameplay scene coverage remain open.

## Managed preparation and startup

`rom-data prepare <rom> [resources-root]` and `play --rom-data` share one
preparer. Default storage is `Data/resources` beside a portable executable, or
the local application data directory's `FFTARecomp/resources` in installed
mode (`%LOCALAPPDATA%` on Windows, `$XDG_DATA_HOME` / `~/.local/share` on Linux).
`FFTA_RESOURCES_DIR` is an explicit root override. Portable mode never falls
back to installed storage when its own directory is unwritable.

Managed layout is `<root>/<source-sha256>/v<schema>/e<extractor>-d<decoder>/`.
This revision directory contains a persistent `prepare.lock`, immutable
`generation-*` directories and a small `current` file containing only the
selected generation's relative name. Current revisions are EUR `v1/e22-d21`
and USA `v1/e8-d7`;
all previous revision directories are retained. Each generation contains the
same seventeen EUR/six USA files as offline extraction. Source identity is rehashed from
the selected ROM contents at every preparation; file name or mtime cannot
substitute another source. Region and schema/decoder changes select separate
directories and retain previous versions.

The preparer validates `current` and every selected file before reuse. Valid
reuse does not invoke the extractor or writer. Corrupt/missing data are
reported and regenerated under a process-owned filesystem lock, with a second
validation after lock acquisition so concurrent launches publish once.
Cancellation is checked between preparation stages and while waiting for the
lock; terminating a launcher/runtime releases the OS lock automatically.
The lock file is retained rather than deleted, so live writers cannot lock
different replacement files.

Files are synced and read back before `current` is atomically replaced.
Unpublished, partial or orphan generations are never selected. A failure before
commit preserves the old selection; a failure after commit leaves a complete
verified generation. Existing generations are retained rather than overwritten
or recursively deleted. They can be removed with the whole resource revision
directory while the game is closed; the next preparation regenerates it.
User mods, saves and native caches are outside this namespace and remain
untouched by regeneration. Generated root-workspace `Data/` is ignored by Git;
release packaging already selects explicit inputs and excludes personal Data.

Startup preparation runs after ROM/package verification and before save
discovery, backup capture or native translation. Errors abort startup instead
of silently selecting the old ROM resource path. Progress uses readable
`resources:` messages and the existing `STATUS building` protocol with game-data
phase labels; lock-wait messages are deduplicated. The prepared typed data are
loaded from validated files and connected to the OBJ family's guest/host
consumers. BG descriptors remain immutable typed session data; a reconstructed
source projection serves their CPU/DMA/native bus reads. Base-palette banks
feed the LZ77HLE with exact file-derived source views for other bus consumers.
All prepared files
contribute to dataset identity and are authenticated before activation. This remains an opt-in partial migration, not complete ROM extraction.

## Validation and limits

The historical OBJ-only extraction/preparation results below cover their
original revisions. The descriptor metadata contract and managed dataset work
are recorded separately; preparation alone does not close BG consumption or
its startup/runtime performance gates.

The earlier three-file integration passed the affected extraction/runtime/file/
preparation tests (25Linux/23Wine including private regional/lifecycle cases),
warning-free production builds and18actual CLI checks per target. These cover
source authentication, exact output/reuse, BG corruption/missing repair,
transactional activation, whole-set identity, old e2-d1retention and resource-root
relocation. Three files total107861logical bytes for the supported data. Synthetic
tar/zip marker scenarios exclude both old and new private resource names.
Canonical executables/old fixture bytes remain unchanged. The subsequent
[direct BG runtime unit](rom_data_graphics.md#direct-descriptor-consumption)
passes all-width/mirror/DMA, real-file regional interpreter/native provenance,
Load/Undo/backup and1200-frame native boot parity on Linux/Wine. Combined-data
GUI and native Windows remain open; the frozen e3scene/cost outcomes are
recorded in the [validation protocol](development/ROM_DATA_VALIDATION.md).

Release-mode targeted tests pass on Linux (3 core, 6 CLI) and Windows GNU/Wine
(3 core, 5 CLI; the additional symlink test is Unix-only). These include the
opt-in private-ROM checks for both exact regions, independent source/alias
roundtrips, identical decoded payloads with distinct regional manifests,
source/region mismatch rejection, deterministic output/reuse/relocation,
malformed/bounded input, forged base data, missing files and four concurrent
publishers. No game bytes or personal fixtures are checked in.

Production CLI builds also pass without warnings on both targets. Actual Linux
and Windows/Wine executables pass command help and first extraction/reuse/verify
for both regions; their generated manifests and decoded files are byte-identical.
The pre-existing canonical executables were restored and checked by SHA-256;
portable archives were not rebuilt.

The first Wine launch could not create its server lock in the sandbox's
read-only runtime directory. The same compiled tests pass when Wine can create
its normal server lock. The offline unit did not change machine/save formats,
native cache identity, guest cycles or audio processing.

The subsequent preparation unit passes 14 targeted runtime tests on Linux and
12 on Windows GNU/Wine, plus the affected private regional CLI regression on
each. Two extra Unix-only checks cover symlinks. A separate ignored test helper
is invoked by the process-interruption regression, which kills child preparers
after the first file, before commit and after commit, then proves old data
preservation, unpublished-data rejection, committed-data reuse and lock release.
Tests also cover four simultaneous preparers, cancellation while waiting,
simulated storage exhaustion, failed publication, invalid generation paths,
source/version separation, relocation and mod/save/cache preservation.

Both production CLI builds are warning-free. Actual Linux and Windows/Wine
commands verify managed creation/reuse/repair for corrupt and missing files,
regional switching, portable moves from an unrelated working directory,
explicit overrides, installed roots and startup failure before save writes.
Actual opt-in interpreter GUI runs on Linux and Windows/Wine prepare data on
first boot, reuse the identical generation after a portable move, display the
EUR language screen and exit cleanly. Captures were inspected; no image or save
fixture is committed. The first Linux display setup was blocked by sandbox
socket access; the same prepared fixture passes with an isolated Xvfb socket.
A Windows smoke helper initially failed to decode an extended-length path;
only the helper was corrected, and the existing dataset was reused.

Native Windows and physical power-loss testing remain unverified. Directory
metadata is explicitly synced on Unix; Windows uses synced files and atomic
replacement without a portable directory-flush API. Old/orphan generations are
retained for recovery rather than automatically pruned. Direct reads and
load/Undo/backup rebinding are now verified for this family in the
[runtime evidence](rom_data_runtime.md); broader scene/lifecycle and measured
performance gates remain. Canonical executables are restored by hash;
archives and AndroidPort are unchanged.

BG descriptors and base palettes use readable JSON with standard indentation.
The user explicitly prioritizes functionality and readability over reducing the
space used by extracted data. File/dataset size checks bound malformed input;
they are not compression targets. The historical e3cost observations remain
recorded, without further formatting or packing work to meet their former disk
budget. [Current JSON revision and checks](rom_data_graphics.md#readable-json-datasets).

Current input guards are16MiB manifest,16KiB OBJ,256KiB descriptors,
1MiB base palettes,4MiB alternate palettes,64MiB packet programs,32MiB each for
EUR main graphics and transfer programs,8MiB short texts,512MiB broad text catalogs
16MiB complete font,1MiB name keyboard,4MiB each for static OBJ lists and indexed animation banks, and512MiBtotal. The manifest guard accommodates the33581additional broad
text/font provenance ranges;
these are malformed-input guards, not storage optimization targets.
The manifest includes162base-palette,316alternate and352packet provenance
ranges, plus metadata/OBJ sources. These are malformed-input guards, not storage
targets. Standard readable packet JSON is20217427bytes; the verified EUR set
is22405770bytes. The existing four decoded files remain byte-identical to e7;
old generations are retained. No compression/space tuning is performed.
[Alternate codec](rom_data_graphics.md#typed-alternate-palette-codec) and
[managed checks](rom_data_graphics.md#managed-alternate-palette-datasets).

The user's current priority is functional migration without performance
measurement or space-reduction work; optimization comes later. Prior benchmark/
RSS limits and results are historical, and no performance campaign should be
resumed from an older next-action line. Guest clock/state/audio correctness,
source provenance and save/native compatibility checks remain required.

## Managed packet programs: EUR/Wine

The [typed packet model](rom_data_graphics.md#typed-descriptor-packet-programs)
is now the sixth fixed managed file in `v1/e8-d7`. Manifest coverage, all352
source ranges, BG dependency and complete file identity are authenticated before
publication and activation. Four focused Wine checks and seven actual Windows
CLI operations pass: extract/verify/prepare, no-write reuse, corruption repair,
root move and a single native activation frame with the prior TinyCC cache.
The e7generation and protected ROM/canonical/cache/loader/typed02files remain
unchanged. This cold frame proves activation/cache compatibility, not packet
consumption. At that managed boundary the runtime reported `verified-only`.

[Managed contract/results](rom_data_graphics.md#managed-packet-program-datasets).
Actual EUR packet source/body use is now connected in04with source-equal e8
files, preserving guest timing, dynamic fill and native/save compatibility.
Three focused Wine checks pass, including nine original/native calls with all
old packet spans poisoned and Load/Undo/backup/restart. One actual397frame
pub transition matches every state/image/audio stream and consumes2640typed
packet bytes. [Consumer contract/results](rom_data_graphics.md#direct-european-packet-consumption).
New Linux/USA checks are pending; no broad suite, old complete producer/scene
matrix or performance campaign was selected. Full ROM_DATA_TODO, other content
families and mod phases remain open.
