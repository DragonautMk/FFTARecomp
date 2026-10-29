# Sessioni e artefatti di debug — DBG-18

Il gestore produce una **preview HTML/JSON**, con spazio stimato e sessioni da
conservare. Apply usa quel piano e una nuova ricevuta. Si selezionano librerie
generate nelle cache di sessioni concluse e le copie disposable dei test ROM già
riconosciute dalla [pulizia esistente](development/LOCAL_ARTIFACTS.md).

Le sessioni e i loro originali, input, salvataggi, registrazioni, dataset, report,
errori, manifest, eseguibili autenticati e preimmagini sorgente restano conservati.
`--keep` esclude completamente una sessione dalla pulizia. I file sconosciuti
restano conservati; il gestore non esegue una pulizia globale di `target/`, Docker,
Cargo o cache normali. I limiti dei collector precedenti rimangono attivi:
otto cartelle per tipo, limiti di eventi/frame/tempo e dimensioni indicati dalle
[guide](development/DEBUG_TOOLS_TODO.md) e riepilogati nella preview.

## Python o PowerShell

Servono **Python 3.11+**, la cartella pubblica `scripts/` completa e Git quando
il percorso appartiene a un checkout o si selezionano unità ROM. Non serve
compilare il gioco. La preview scrive soltanto il proprio report e lease
transitori di coordinamento; non rimuove payload.

Dalla root portable/progetto, scegliendo cartelle realmente proprie:

```powershell
python3 scripts/debug_artifacts.py preview --session 'Data/debug-sessions/caso uno' --session 'Data/debug-sessions/caso due' --keep 'Data/debug-sessions/caso uno' --out 'target/artefatti-preview-1'
python3 scripts/debug_artifacts.py apply 'target/artefatti-preview-1/plan.json' --receipt 'target/artefatti-ricevuta-1.json'
```

`--sessions-root CARTELLA` elenca i figli immediati, senza cercare in tutto il
progetto. Si possono ripetere `--session` e `--unit target/rom-data-NOME`;
`--unit` dichiara conclusi i controlli di quell'unità e riusa la selezione
`interrupted` / `moved resources` del vecchio helper. Il dataset accettato, i
risultati, gli originali e le prove restano conservati. Non vengono ritirati
eseguibili di unità dal nuovo gestore; la vecchia opzione esplicita resta distinta.

Per avere sempre il log del comando, anche avviando da un'altra directory:

```powershell
.\scripts\windows\debug_artifacts.ps1 -ToolArguments @('preview', '--session', 'Data/debug-sessions/caso uno', '--keep', 'Data/debug-sessions/caso uno', '--out', 'target/artefatti-preview-2')
```

Oppure su Linux:

```sh
bash scripts/linux/debug_artifacts.sh preview --session 'Data/debug-sessions/caso uno' --out 'target/artefatti-preview-3'
```

I launcher risolvono la root dalla propria posizione e indicano il file in
`target/build-logs/`. Exit **0** indica successo del comando, **1** errore o piano
cambiato; la preview può riuscire con contesti protetti e zero candidati: leggere
lo stato di ogni riga. Nessun comando sovrascrive una cartella di preview o una
ricevuta esistente. La [homepage](report_home.md), dopo rigenerazione, indicizza
le preview locali.

## Docker

Il launcher usa `ffta-debug-tools:local`, derivata dall'immagine coverage già
presente. La derivata aggiunge Git, assente nell'immagine coverage originaria.
La preparazione richiede rete per i pacchetti Debian; i comandi di gestione
successivi usano `--network none`, filesystem container readonly e un bind della
cartella del progetto. Non copiano dati di gioco nell'immagine né compilano Cargo.

```sh
docker build -f Dockerfile.debug-tools -t ffta-debug-tools:local .
bash scripts/linux/debug_artifacts.sh --docker preview --session 'Data/debug-sessions/caso uno' --out 'target/artefatti-docker-1'
bash scripts/linux/debug_artifacts.sh --docker apply 'target/artefatti-docker-1/plan.json' --receipt 'target/ricevuta-docker-1.json'
```

Su PowerShell usare `-Docker -ToolArguments @(...)`. Non servono Python o Git
sull'host per questo percorso. Docker Desktop/daemon e l'immagine coverage sono
prerequisiti. Preview e apply devono usare lo stesso launcher: i percorsi Docker
sono `/project/...`. Dopo uno spostamento della cartella o un aggiornamento dello
strumento, generare una nuova preview. I piani conservati descrivono la
posizione e le identità dei file al momento della selezione.

## Coordinamento e conservazione

Le nuove copie isolate dichiarano `artifact_coordination: directory-v1` nel
manifest. Runtime, creazione della sessione, gestore e generazione/rotazione
delle cache coverage condividono il protocollo. Una directory
`.debug-artifacts-lease` e il suo `owner.json` identificano il possessore;
la creazione atomica della directory evita assunzioni sull'interoperabilità
fra lock Windows, Wine, Python e container. Il normale lock della sessione
rimane presente. Il lease viene preso prima della verifica/copia e mantenuto
per l'intera operazione o esecuzione; un concorrente rifiuta il contesto in uso.

Un processo ucciso lascia un lease protettivo. Non esiste recupero automatico
basato sul PID, che non identifica lo stesso processo tra host/Wine/container.
Prima di rimuovere manualmente un lease residuo, verificare il processo e i log
che lo hanno creato e la conclusione del lavoro. Una sessione legacy senza il
protocollo, incompleta, fallita o contenente link/file condivisi resta protetta.
Non si sostituisce il suo eseguibile autenticato per renderla pulibile.

Apply ricostruisce il piano, prende tutti i lease richiesti e ricontrolla sotto
coordinamento le identità dei candidati e, per le sessioni, dei file conservati prima della prima
rimozione. Cambiamenti, nuove catture, un runtime avviato o un piano alterato
richiedono una nuova preview. Una nuova sessione distinta non può entrare nella
selezione già fissata. I file tracciati da Git sono rifiutati.

Le ricevute registrano piano, operazione pendente, rimozioni riuscite ed errore o
completamento; vengono scritte e sincronizzate prima e dopo ogni rimozione.
Un'interruzione può lasciare un prefisso applicato: controllare ricevuta e file,
poi creare una nuova preview. La rotazione automatica coverage conserva sempre
la generazione corrente e scrive ricevute in `.retention/` nella cache, fino a
64 ricevute; oltre il limite la pulizia si ferma conservando report e dataset.
I lettori espliciti di una generazione `CACHE/SHA256/eN-dN` prendono lo stesso
lease senza abilitare pruning. Quando Docker nasconde il percorso dietro
`/input/dataset`, entrambi i launcher montano separatamente la root di
coordinamento e passano `--cache-lease /cache-lease`: il dataset resta readonly.
L'invocazione Python diretta riconosce il layout; invocazioni personalizzate
possono indicare `--cache-lease ROOT_CACHE`. Un marker attivo annidato in una
copia ROM protegge l'intera unità, comprese le altre copie diagnostiche.

Tutti i processi che condividono una cache devono usare gli strumenti aggiornati:
lettori esterni o vecchie versioni non partecipano al protocollo.

Limiti del gestore: **32 contesti, 8.192 entry per albero esaminato, 8.192 file candidati,
16 MiB per piano/ricevuta**; non legge payload per stimare lo spazio. I byte
logici sono esatti per la selezione; quelli allocati usano `st_blocks` quando
presente, altrimenti i byte logici con `allocation_estimated: true`. Lease
interrotti, cartelle incomplete ed errori restano disponibili per la diagnosi.
Le ricevute descrivono disponibilità locale: payload ritirati non riaprono
prove concluse e non autorizzano ricostruzioni o replay per ripristinarli.

## Verifiche

[Record DBG-18](changes/2026-10-29-coordinate-debug-artifact-retention.json).
Le prove Python usano placeholder pubblici sintetici; quelle Wine verificano
il runtime in pausa, senza eseguire nuovi frame guest. Gli input e le prove
precedenti rimangono conservati. La matrice distingue Python/Linux e Docker,
Windows GNU/Wine e PowerShell/Windows nativo non eseguiti.


Nel launcher PowerShell -Docker precede il comando. I flag del tool, incluso
--out, sono inoltrati letteralmente senza intercettazione dei parametri comuni
PowerShell. La [distribuzione DBG-22](debug_tools.md) aggiunge13controlli pubblici
dei launcher su PowerShell7.4.6/Linux; Windows nativo/PowerShell5.1 resta non
verificato.
