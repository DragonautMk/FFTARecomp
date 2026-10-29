# Local validation artifacts

ROMs, saves, input recordings, accepted datasets and test evidence have distinct
lifetimes. Do not delete the entire `target/` tree or run a global Docker prune
to clean one unit. The root workspace and AndroidPort remain separate.

## Preserve local files when aligning branches

Aligning branches does not select a validation unit for cleanup. Preserve
ignored and untracked local files, including files newly covered by the target
branch's `.gitignore`. Inventory them before switching, check incoming tracked
paths for collisions, and use `git switch --no-overwrite-ignore` so Git refuses
to overwrite an ignored file. Do not use `git clean -fdx` or `git clean -fdX`.

When replacing a divergent local branch tip, retain a recovery reference first.
Let checkout remove only obsolete tracked paths absent from the target tree;
verify local files remain afterwards. Delete disposable local artifacts only
through a separately selected, successful unit and the preview/receipt rules
below. An ignored file is not evidence that it is no longer needed.

## Session manager and activity coordination

[DBG-18](../debug_artifacts.md) adds bounded preview/apply with keep-selection,
HTML/JSON estimates and per-operation receipts. It preserves every session's
inputs/reports/errors and selects only generated native cache libraries; explicit
completed ROM units reuse the disposable-copy rules below. New sessions and
Python tools share directory-v1 activity leases across Wine/native/container
processes. Active/interrupted/legacy/unattested/failed scopes remain protected.
Automatic coverage preparation/read/pruning holds one cache lease and retains
receipts in `.retention/`;64receipts stops further pruning until archived.
Old uncoordinated tools/readers must not run against an evictable shared cache.

The `complete` CLI below now takes the same unit lease; duplicate/concurrent
cleanup refuses before removal. No automatic stale-lease reclamation, global
prune or deletion of original/accepted data. Portable Python/PowerShell/Docker
entry points, logs, retained payload availability and exact limits are in the
linked guide. Root/staged coverage Docker contexts include every Python import.

## Complete a validation unit

Managed validation deliberately creates several large dataset copies to test
failed publication, repair and relocation. After successful managed/native tests,
record their results and run preservation checks, then retire the disposable
copies. Keep the accepted `offline dataset` for CLI/scene work and later source
investigation. Failed or running tests retain all diagnostic data.

Preview from the repository root (Python and Git required):

```sh
python3 scripts/local_artifacts.py complete target/rom-data-example-runtime
```

After reviewing the paths, apply and keep a receipt:

```sh
python3 scripts/local_artifacts.py complete target/rom-data-example-runtime \
  --apply --report target/rom-data-example-runtime/artifact-cleanup.json
```

The helper removes only `windows-managed-europe/interrupted` and
`windows-managed-europe/moved resources`. It keeps the accepted offline dataset,
source/model preimages, screenshots, logs, results, canonical executable and
shared Wine/Cargo caches. It refuses tracked content, links, private inputs and
unexpected file types. It rechecks candidate sizes, times and file identities
before removal. Symlinked validation roots, including the current `/tmp` backed
slot-job unit, are skipped.

`--retire-executables` additionally selects known old unit-local `recomp`,
`recomp-tests-*`, `recomp-rom-data` and `gba-core-tests-*` executables. Use this
only for superseded units after checking that later work uses a retained helper.
It does not discover or remove arbitrary binaries, release packages or Wine
prefixes. Every command previews unless `--apply` is present.

Preservation maps should contain inputs actually used by the new unit. Old
expected-output maps remain historical evidence; do not require retired mutable
test copies to remain on disk forever. A cleanup receipt records local artifact
availability and does not invalidate a completed acceptance. Do not recreate old
fixtures or replay closed tests just because these payloads were retired.

## Automatic coverage cache retention

Both ROM coverage Docker launchers pass `--cache-keep 2`: after successful HTML
publication, retain the current dataset plus the newest other generation for
that ROM. Version ordering is numeric. Cache deletion is restricted to the
automatic cache's ROM/revision layout and identity-matching manifest. Unknown,
linked or private content is refused; unrelated ROM directories remain intact.
Cache-cleanup errors are reported without failing an already published report.
Never share a cache with a concurrently running historical coverage analysis
when enabling eviction; use an explicit dataset/output outside that cache.

Explicit `--dataset` inputs are untouched. Direct Python invocation keeps
retention disabled by default (`--cache-keep 0`), including read-only cache
validation; developers may opt into a chosen positive count. Docker continues
to bundle Python, so the host needs no Python for coverage generation.

## 26 October cleanup

The reviewed local sweep retired 80 disposable test trees and 160 superseded
unit-local executables: 7,138 files / about 50.3 GB allocated. All 3,186 private
ROM/save/recording hashes and 53,887 retained-file sizes/times were unchanged.
The newest UI-property/FFFF/slot-job/post-slot units were preserved, as were
accepted offline datasets, actual CLI datasets, Wine prefixes, build caches,
source preimages and proof logs/results. Source files were unchanged by the
deletion. Local manifests are under `target/local-artifact-cleanup-2026-10-26/`.

The lifetime of build caches is separate from these per-test payloads:
there is no scheduled background deletion, Cargo clean or global Docker prune.
