# Campagne mirate — DBG-21

Una campagna dichiara pochi casi e seleziona DS, famiglie o transizioni interessate
da una modifica. **report** legge soltanto i JSON indicati e riusa prove conservate:
non esegue il gioco, non crea snapshot e non riapre accettazioni concluse.

L'avvio esplicito **run-case** delega un solo profilo al supervisore
[replay esistente](debug_replays.md). Non esiste avvio automatico di tutti i casi.

## Manifest e selezione

Creare un JSON privato, fuori dai commit, con questo formato:

~~~json
{
  "schema": 1,
  "kind": "ffta-debug-campaign",
  "label": "Modifica al reader selezionato",
  "cases": [
    {
      "id": "task-entry-EUR",
      "ds": ["DS-20"],
      "families": ["task"],
      "transitions": ["entry"],
      "profile": {"path": "profili/task/replay.json", "sha256": "SHA256_ESATTO_64_HEX"},
      "evidence": {
        "result": {"path": "profili/task/Data/config/replay/result.json", "sha256": "SHA256_ESATTO_64_HEX"},
        "audit": {"path": "profili/task/Data/config/replay/rom-audit.json", "sha256": "SHA256_ESATTO_64_HEX"}
      }
    },
    {"id": "DS03-unprepared", "ds": ["DS-03"], "families": ["scene-transition"]}
  ]
}
~~~

Sostituire i segnaposto con gli hash reali. I percorsi sono relativi al manifest,
con separatori /; possono riferire una directory sorella tramite ../. Spostare
l'antenato comune di manifest/profili/report conserva i collegamenti. Le etichette
DS/famiglia/transizione sono dichiarate: non provano raggiungibilità o uso di
una famiglia da parte delle letture osservate. Non confondere caso DS e slot.

Omettere evidence per un profilo ancora da eseguire; omettere profile per un
caso non preparato. Non cambiare un vecchio hash per nascondere un file alterato.
Dopo una nuova esecuzione esplicita, dichiarare i nuovi pin in una nuova versione
del manifest conservando la provenienza della precedente. Il tool non inventa
un'associazione storica completa tra profilo ed esito da un semplice mtime.

~~~powershell
python3 scripts/debug_campaign.py report campagne/modifica.json --ds DS-20 --family task --out target/campaign-reports/task-entry
~~~

Serve almeno uno tra **--case ID / --ds DS / --family TAG / --transition TAG**.
I valori ripetuti dello stesso filtro sono in OR; filtri diversi sono in AND.
Un ID inesistente viene rifiutato; un filtro senza corrispondenze produce zero
selezionati. Le prove dei casi esclusi non vengono aperte.

## Interpretare gli esiti

Il rapporto mostra separatamente:

- **positive**: il risultato conservato dichiara passed, matched o captured,
  con completamento della propria finestra e identità riportate coerenti.
  Captured/matched restano distinti dal passed dell'endpoint completo.
- **negative**: diverged, error, timeout, interrupted o not-reached già registrato;
  non significa fallimento di ogni funzionalità o scena.
- **incompatible**: profilo/prova mancante o alterato, identità/finestra/endpoint
  incoerenti, schema non supportato oppure risultato registrato incompatible.
- **not-executed**: fuori selezione, profilo non preparato o nessuna prova indicata.

**reused-pinned** significa riuso di byte conservati autenticati dagli hash,
non una nuova esecuzione con il runtime corrente. Il report non apre ROM,
snapshot, SRAM o librerie e non verifica di nuovo disponibilità dei payload,
compatibilità corrente o vecchie classificazioni DS/ROM/native.

Per un audit associato, run/caso/identità/filtro/finestra devono corrispondere.
**zero-in-selected-window** richiede zero richieste, finestra completa, nessun
troncamento o scarto. **partial-zero** conserva la raccolta incompleta;
**unavailable** significa nessun audit autenticato, mai zero letture.
Anche uno zero completo riguarda soltanto i reader strumentati selezionati:
**l'uso globale della funzionalità rimane sconosciuto**.

Il report limita l'audit ai metadati utili; non copia tutti gli eventi.
Le sessioni con directory-v1 usano il coordinamento durante la lettura.
Per le vecchie prove il pin autentica i byte, senza inventare esclusione di
un vecchio processo che non conosce quel protocollo.

## Eseguire un solo caso scelto

~~~powershell
python3 scripts/debug_campaign.py run-case campagne/modifica.json task-entry-EUR --reason "Modificato il reader di ingresso" --reopen-reason "La modifica invalida questa specifica verifica" --out target/campaign-intents/task-entry
~~~

**--reopen-reason** è obbligatorio se il manifest contiene una prova precedente
oppure se esiste già result.json nel profilo, anche quando non è dichiarato.
Un caso nuovo richiede comunque --reason. Non usare perdita di contesto o parent
TODO aperto come motivo di ripetizione; seguire le
[regole di memoria](development/MEMORY_WORKFLOW.md#resume-and-repeat-decisions).

Il tool verifica pin di profilo/runtime, salva **intent.json** e sostituisce
il proprio processo con **recomp debug-replay run PROFILO --json**. Quello stesso
runner gestisce preflight dei payload, lock, timeout, progressi e cancellazione.
Un intent non è un risultato completato. Il worker conserva log/risultato nel
profilo; la successiva campagna può referenziare esplicitamente quei nuovi pin.

Su POSIX un profilo Windows richiede **--wine /percorso/reale/wine64**. Windows
nativo e runtime Linux non richiedono il launcher. Il dispatch non compila ROM
o runtime. Se l'exec fallisce, il receipt rimane dispatch-error. I codici finali
sono quelli del replay. Non viene scritto alcun supervisore alternativo.

## Launcher, limiti e privacy

**scripts/windows/debug_campaign.ps1** e **scripts/linux/debug_campaign.sh**
risolvono la root dalla propria posizione e mostrano sempre il log sotto
target/build-logs. Con --docker (Bash) o -Docker (PowerShell) usano l'immagine
già presente ffta-debug-tools:local, rete disabilitata, filesystem container
readonly e mount del progetto per output/coordinamento. Docker ammette soltanto
report/help; il dispatch usa il Python host. Nessuna build automatica.

~~~bash
bash scripts/linux/debug_campaign.sh --docker report campagne/modifica.json --case task-entry-EUR --out target/campaign-reports/docker-entry
~~~

Python 3.11 e libreria standard, più artifact_coordinator.py e il template HTML.
Al più32casi, manifest256KiB, profilo/risultato128KiB, audit16MiB in lettura,
riepilogo audit64KiB e report2MiB. JSON duplicati/non finiti rifiutati.
Al più8directory per store di report/intent, tentativi falliti inclusi:
scegliere una directory padre dedicata. Nessuna sovrascrittura/pulizia automatica.
Link/symlink, reparse point e file condivisi vengono rifiutati.

Output e percorsi sono privati: una cartella nuova contiene .gitignore.
Il manifest creato a mano va tenuto in un percorso già ignorato. Nessun invio,
promozione di accettazioni o modifica della raccolta originale.
La [homepage](report_home.md) trova i campaign.json dopo rigenerazione;
il [gestore artefatti](debug_artifacts.md) mostra i limiti e conserva questi report.

## Verifica del 29 ottobre 2026

**23 test Python distinti passati**:12campagna (11nel test raffinato più una
verifica aggiunta per JSON rigoroso/tipi/prova precedente non dichiarata) e11del
gestore artefatti interessati dai limiti. Coprono selezione esplicita/OR/AND,
esclusione senza aprire prove, quattro esiti, pin/identità/endpoint/finestra,
zero/parziale/non disponibile, limiti, lease attivo, link/file condivisi,
spostamento, motivo di riapertura e receipt. Il dispatch reale di test usa
soltanto un eseguibile pubblico sintetico, senza dati di gioco.

Un rapporto privato riusa due profili/prove DS20 già conservati (audit positivo
e divergenza native storica), più metadati sintetici zero/incompatibile e un
caso non preparato. **Zero nuovi frame guest**; nessuna vecchia accettazione
rieseguita. Undici guardie di metadati/input/runtime normali conservano hash/mtime.

Launcher Bash da directory estranea e Docker offline su immagine esistente
producono selezioni1e2; nessuna nuova immagine/build/runtime. Quattro viste Chrome
desktop/stretto/selezione vuota/homepage verificano filtri/ricerca/reset, esiti,
riuso, zero distinto dall'uso globale sconosciuto e link relativi senza overflow.
Screenshot ispezionati; homepage72indici/4campagne al momento della prova.

Verificatore finale autentica8sorgenti pubblici e quattro coppie HTML/JSON,
pin/link e guardie. Ritirati soltanto4profili browser terminati; tutti i metadati,
manifest, report, sorgenti/preimages, log e prove originali conservati sotto
**target/debug-tools-campaign/**. Nessuna fixture managed/native disposable:
il completion helper ROM non si applica. Windows/PowerShell, dispatch reale Wine
o Linux e controller fisici non verificati in questa unità; si riusa il supervisore
già validato senza chiamarlo sui dati di gioco.
[Record DBG-21](changes/2026-10-29-add-targeted-debug-campaigns.json).


Nel launcher PowerShell -Docker precede il comando. I flag del tool, incluso
--out, sono inoltrati letteralmente senza intercettazione dei parametri comuni
PowerShell. La [distribuzione DBG-22](debug_tools.md) aggiunge13controlli pubblici
dei launcher su PowerShell7.4.6/Linux; Windows nativo/PowerShell5.1 resta non
verificato.
