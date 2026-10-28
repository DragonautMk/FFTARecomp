# Local cheat reference catalog

The root `cheats/` directory is a private reference collection for future work.
Open `cheats/README.md` locally to browse the Markdown index. The directory and
all derived Markdown/JSON are excluded from Git: the supplied third-party
compilations include an explicit copyright notice and their redistribution
permissions have not been established. This documentation contains no cheat
payloads and does not assign a license to the source material.

## Organization

The initial collection contains 341 distinct code blocks from five supplied
documents, after merging 228 duplicate occurrences. Browsable pages separate
European Codebreaker/GameShark SP/Xploder codes, USA and Japanese GameShark
sections, and Codebreaker/Xploder and Action Replay lists with unconfirmed
regions. Separate pages retain the prose guide, modifier lookup lists, source
attributions and quality findings. The original language and values are kept.

Each block has a stable `CH-` identifier, ordered code lines, source references,
aliases and links to the master code supplied by each source. Deduplication uses
the entire ordered block, declared device format and region together. The
unconfirmed-region group records identical text without asserting that its
sources target the same ROM. Repeated instructions inside a block are retained.

`cheats/catalogo.json` is a local schema-1 export for future indexing. It contains
source hashes, `entries`, format/region, canonical code lines, aliases, notes,
status and provenance. Each provenance item retains the source title/category,
original line numbers and raw code lines (or the site's code ID), plus its
`master_id`. Existing IDs should remain stable when extending the catalog.

## Review and limitations

Validation checks preservation and organization; none of these codes has been
applied to the runtime, a ROM or a save. USA/Japan labels follow the supplied
sources. English text alone does not establish a ROM region. Device versions,
encrypted-code semantics and compatibility with FFTARecomp remain unverified.

The local quality page flags two different Codebreaker master codes, apparently
swapped P3/P4 level labels, a possible Player 3 modifier typo and unresolved
placeholders. Values remain unchanged. The saved European web page contains
23 actual code blocks but 25 empty groups loaded on demand, and one missing
popup note; its collection is explicitly incomplete.

The static check verified all 796 source code-line occurrences, the 341 rendered
blocks and their source-specific master links, 1,544 local Markdown links, guide
and modifier-list preservation, and Git exclusions. It also verified all 42
original files against their SHA-256 archive manifest. The active directory now
contains ten Markdown files and one JSON catalog; 37 saved-browser support files
and the five superseded source documents were removed from that directory.

The reversible original archive, manifest, conversion/verification scripts and
cleanup receipt are retained locally under
`target/cheats-catalog-2026-10-28/`. They are ignored and unavailable in a fresh
clone. Missing local copies do not authorize downloading third-party collections
or recreating game fixtures automatically. The scripts describe this one-time
conversion and are not a product importer.

## Adding references

Record the source and attribution, ROM identity/region, exact device version,
complete ordered block and actual verification status. Keep master-code
relationships and regional variants explicit. Review suspected typos against
an independent source or an isolated validation before changing values.
Reassess redistribution permission before publishing any collection content;
the local ignore rule alone is not a license or a packaging exclusion.
