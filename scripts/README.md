# Project scripts

Platform-specific entry points live in these folders:

- `linux/`: Linux Docker build/extraction entry points and the native Linux
  packager. `build_windows_docker.sh` builds the Windows package from a Linux host.
- `windows/`: PowerShell entry points, the Windows portable-packager wrapper,
  MSYS2 TinyCC builder and Windows-to-Linux cross-build helper with its C linker.
- `android/`: Android APK builder and Android-device camera helper. The current
  APK builder requires a Windows SDK/NDK toolchain as documented in its header.
- `unix/`: TinyCC builder shared by Linux and macOS.
- `tests/`: shared Python tooling tests.

Cross-platform extraction, packaging, release, analysis and project-memory
tools stay directly in `scripts/`. For example,
`benchmark_windows_runtime.py` supports native and Wine runtimes despite its
historical filename; `package-release.sh` runs on all three CI host platforms.
The root `build-all.ps1` remains the Windows all-platform build entry point.
It launches the Linux cross-build helper with the current PowerShell executable,
so PowerShell 7 does not try to find the Windows PowerShell `powershell.exe`
inside its own installation directory.
`AndroidPort/` has its own scripts and is managed separately.

The runtime's [session diagnosis](../docs/debug_diagnostics.md) is available as
`recomp diagnose` and Debug > Copy diagnosis; it needs no Python installation.
[Isolated debug sessions](../docs/debug_sessions.md) use `recomp debug-session`
and Debug > Start debug session to work on portable copies of the selected case.
[The save catalog](../docs/debug_catalog.md) uses `recomp debug-catalog list/open/import`
and Debug > Debug catalog for shared filters, integrity checks, previews and
opening a DS checkpoint in an isolated copy. These runtime commands need no Python.
[Snapshot input recordings](../docs/debug_recordings.md) use Debug > Start/Stop
input recording and `recomp debug-record start/verify`: capture relative input
from a private snapshot and compare a bounded interpreter replay's full endpoint.
[Replay profiles](../docs/debug_replays.md) use `recomp debug-replay create/run/cancel`
and Debug > Replay case. The same isolated worker reports progress and distinct
success/divergence/incompatible/timeout/interruption outcomes, without performance
measurement or game-save writes; an existing native library is optional.
[ROM read audits](../docs/debug_rom_audit.md) use `debug-rom-audit create/run/cancel`
and Debug > Audit replay, with an explicit `rom-data-read-audit` worker feature.
They add bounded events, frame/address/reader filters, actual typed/raw service
and physical-byte unions without re-extraction or automatic coverage promotion.

The [debug-tools checklist](../docs/development/DEBUG_TOOLS_TODO.md) maps existing
CLI/debug capabilities to planned user-facing and machine-readable workflows.
It distinguishes available commands from proposed tools and DS scene cases.

From the repository root, extract readable ROM data with Docker:

```bash
bash scripts/linux/extract_rom_data_docker.sh
```

```powershell
.\scripts\windows\extract_rom_data_docker.ps1
```

For portable builds:

```bash
bash scripts/linux/build_linux_docker.sh
bash scripts/linux/build_windows_docker.sh
```

```powershell
.\scripts\windows\build_docker.ps1
```

Both extraction entry points use
`rom/1194 - Final Fantasy Tactics Advance (E)(Surplus).gba` relative to the
repository root by default. Override it with a first ROM argument on Bash or
`-RomPath` on PowerShell; absolute ROM paths are accepted and relative ROM paths
always start at the repository root, even when launched from another directory.

Scripts resolve their repository from their own location. Explicit output
paths remain relative to the calling directory. See
[Docker workflows](../docs/docker.md) and [extraction options](../docs/rom_data.md).
The EUR extractor also publishes `word-task-obj-animations.json` in e77-d76:
four source-authenticated banks for u16 task counters, with readable OBJ lists,
relative indices, frame selectors and durations. See the [managed runtime
contract](../docs/rom_data_obj.md#managed-runtime-for-the-u16-task-obj-banks)
for validation, actual-CLI evidence and the source-backed ordinary Bank check. USA remains e8-d7.
EUR e78-d77 additionally publishes `world-nodes.json`: typed coordinates,
ordered path/neighbor links and losslessly preserved opaque fields. The
[world-node runtime contract](../docs/rom_data_maps.md#managed-world-nodes-and-typed-guest-reads)
records the reduced Wine integration checks and actual CLI create/reuse/normal
coordinate use. The subsequent ordinary travel360check passes coordinates,
paths and neighbors with complete state/clock/image/audio equivalence. Other
nodes/maps, opaque ordinary use and new Linux/USA checks remain pending.


The Linux Docker builder saves each run's output and errors under
`target/build-logs/linux-*.log`. On failure it waits for Enter in an interactive
terminal; noninteractive automation receives the original exit code immediately.

After recording successful managed/native validation, retire disposable test
copies with `python3 scripts/local_artifacts.py complete target/<unit> --apply`.
Preview without `--apply`; retain the accepted offline dataset and proof logs.
See [local artifact retention](../docs/development/LOCAL_ARTIFACTS.md) for safety
checks, optional superseded-binary retirement and coverage cache limits.


EUR e79-d78 extraction additionally publishes `world-roads.json`
(`gameplay.world-roads`), covering45conditional8-byte headers and2,188signed
XY samples through46source spans. Schema1 and USAe8-d7 stay unchanged. Runtime
preserves guest movement and bytewise old-owner/raw fallback; the reduced Wine
integration proof is recorded in [managed world roads](../docs/rom_data_maps.md#managed-world-roads-and-typed-guest-reads).
Production CLI and ordinary-scene proof remain next; Linux/USA tests pending.


Actual e79-d78CLI creation/reuse/startup and one Giza360ordinary road pair now
pass on Wine, with72files/71resources/74managedpaths unchanged and full states/
clock/images/OAM/dispatch/fiveaudio. All seven road counts match CLI, including
positive count/pointer/sample bytes. See [the scoped result](../docs/rom_data_maps.md#ordinary-sampled-road-travel-equivalence).
Linux/USA/nativeWindows and broader coverage remain pending; no measurements.

I launcher ROM coverage accettano audit DBG-06 ripetibili (`--audit FILE` su Bash,
`-AuditPath @('a.json','b.json')` su PowerShell) e una fotografia precedente
(`--compare FILE` / `-ComparePath FILE`). Importano soltanto metadati locali,
con mount in sola lettura; nessun replay. La mappa distingue unioni osservate,
servizio Typed/Raw e intere famiglie positive, con filtri e limiti espliciti.
Vedi [istruzioni ed esempi](../docs/development/ROM_COVERAGE.md#scene-letture-e-confronto-dbg-07).


EUR e80-d79 extraction publishes `encounter-headers.json`
(`gameplay.encounter-headers`) with a constructor-alternate dependency. Its two
fragments migrate17,676bytes while retaining existing ownership of the shared
four-byte pointer. The reduced Wine integration result is documented under
[managed encounter headers](../docs/rom_data_maps.md#managed-encounter-headers-and-typed-guest-reads).
Actual CLI/ordinary coverage and Linux/USA/nativeWindows remain pending; no
performance measurement is included.

Il comando runtime `debug-capture create/run/cancel` e Debug > Capture on trigger
producono punti derivati da replay isolati: prima lettura di una risorsa,
ingresso interprete in un PC ROM verificato, oppure N frame completati.
Richiedono `rom-data-read-audit`, input registrato e limite esplicito; il risultato
è un bundle/catalogo privato o `not-reached` senza snapshot. Vedi
[uso, provenienza e limiti](../docs/debug_captures.md).


EUR e81-d80 extraction adds `encounter-unit-templates.json`
(`gameplay.encounter-unit-templates`) with five explicit header/constructor/item/
metadata14/job dependencies. Two source fragments skip the192 bytes already
owned by constructor alternates; strict coherence and typed reads preserve
original live initialization. [Contract and checks](../docs/rom_data_gameplay.md#managed-encounter-unit-templates-and-typed-guest-reads).

`recomp debug-compare create/run/cancel` e Debug > Compare first difference
confrontano ogni frame dal medesimo snapshot/input: interprete/raw come
riferimento e interprete/native con dataset opzionale come candidato. Il primo
confine diverso produce un report HTML/JSON privato con campi CPU/Bus esatti,
immagini affiancate e stream; un prefisso uguale conserva solo metadati.
Non richiede la feature audit. Vedi [uso, ambito e verifiche](../docs/debug_comparisons.md).

`recomp debug-control status/pause/step/resume SESSION` controlla la finestra
di una copia isolata. `step SESSION N [--keys HEX]` avanza esattamente 1–600 frame
e rimane in pausa; menu e F6/F7/F8 condividono le stesse azioni.
`debug-session open SESSION --paused` evita frame prima della prima richiesta.
Vedi [DBG-10: procedura, audio e limiti](../docs/debug_control.md).


EUR e82-d81 extraction adds `encounter-coordinates.json`
(`gameplay.encounter-coordinates`) with header pointer/count/role coherence.
Its691 spans retain raw XY bytes and original signed/unsigned placement/RNG
logic. [Managed resource and checks](../docs/rom_data_maps.md#managed-encounter-coordinates-and-typed-guest-reads).

`recomp debug-control inspect SESSION ADDRESS LENGTH` legge buffer e registri
senza effettuare accessi I/O. `break SESSION PC arm|thumb` e `watch SESSION
read|write START END` cercano un punto nell'interprete con limiti di passi/frame;
watch richiede `rom-data-read-audit`. Vedi [DBG-11 e granularità](../docs/debug_probes.md).

`recomp debug-control graphics SESSION [--compare-last]` cattura dalla copia
in pausa un [report grafico DBG-12](../docs/debug_graphics.md), con PNG/JSON e
buffer PPU privati, proiezioni readonly e confronto con l'ultimo report.
Non richiede audit e non genera un nuovo snapshot.

`report_home.py` genera `index.html` nella root, con tutti gli indici HTML locali,
ricerca e filtri. `windows/report_home.ps1 -OpenReport` e
`bash scripts/linux/report_home.sh` usano l'immagine Docker coverage già esistente,
con rete disabilitata e checkout readonly; non eseguono estrazione o runtime.
Il solo catalogo generato resta ignorato. [Uso e verifiche](../docs/report_home.md).

`recomp debug-control ui-start SESSION` / `ui-report SESSION` e le voci Debug
Start UI / input trace / Save UI / input report raccolgono osservazioni limitate
di focus, font, input/rimappatura e dispatch host/guest nella copia isolata.
`status.host_menu` espone il focus reale prima di automatizzare la navigazione.
I report privati sono riconosciuti dalla homepage. [DBG-13](../docs/debug_ui.md).
