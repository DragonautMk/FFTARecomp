# Decoded ROM resources

The desktop developer CLI can extract and verify the first supported family,
`graphics.obj-dimensions`, from an exact supported FFTA EUR or USA ROM.
This is the initial ROMDATA-02 implementation. Startup preparation and runtime
consumption are not connected yet; gameplay still uses the original ROM.
The [TODO](development/ROM_DATA_TODO.md) tracks the remaining phases and
[inventory](development/ROM_DATA_INVENTORY.md) separates known readers from
unknown asset catalogs and formats.

```sh
recomp rom-data extract /path/to/your-game.gba target/rom-data/my-dataset
recomp rom-data verify /path/to/your-game.gba target/rom-data/my-dataset
```

The output directory contains `manifest.toml` and `obj-dimensions.toml`.
The latter stores twelve indexed width/height records. The manifest records
schema/extractor/decoder revisions, exact source ROM identity, regional profile,
decoded coverage, file hash, dependency IDs and the original ranges/hashes.
USA's direct table and copied alias are verified independently; EUR has its
own verified source address. Resource identity is independent of file offsets.
There is no ROM code, image, save, absolute installation path or timestamp in
the manifest. The generated decoded values remain private game data: keep
outputs under ignored `target/` for this developer command, outside packages.

Extraction writes and syncs complete files in a temporary sibling directory,
reads them back through the validator and then renames the directory into
place. Multiple publishers may reuse a complete, source-verified winner.
Repeated extraction reuses valid existing data. An invalid existing dataset
causes an error and remains untouched; choose a fresh output directory to
regenerate it. Automatic corruption recovery, cancellation and crash durability
are still ROMDATA-03 work. No mod or save directory is selected by this command.

Both files are limited to 16 KiB, use strict typed TOML and reject unknown
fields. The reader uses fixed relative file names, rejects symlink files and
directories, and validates the complete manifest against the selected source.
An edited dimension file remains invalid even if its file hash is updated in
the manifest: base data must reconstruct the original source exactly. Mods
will need a separate loader and compatibility identity.

The command prints `ROM_DATA action=created|reused|verified`, source/profile,
schema and decoded coverage. `runtime=not-connected` explicitly identifies the
current stage. It does not claim that graphics, sprite pixels, text, maps,
gameplay tables or audio have been exported or migrated.

## Validation and limits

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
its normal server lock. Native Windows execution, abrupt power-loss testing,
automatic startup, guest reads, save-state lifecycle and performance comparisons
remain unverified. This additive offline tool does not alter machine/save
formats, native cache identity, guest cycles or audio processing.
