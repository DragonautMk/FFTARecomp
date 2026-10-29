# Report locale del problema — DBG-16

Il report riunisce sintomo, caso, frame/input, identità e risultato conservato
di un [profilo di replay](debug_replays.md), con eventuale collegamento al primo
confronto divergente e una prossima verifica suggerita. Non esegue il caso,
non invia file e non duplica ROM, salvataggi, snapshot o dump.

## Creare il report

CLI Windows, con il runtime aggiornato:

```powershell
.\recomp.exe debug-report create '.\profilo\replay.json' --symptom 'Descrivere cosa si vede e cosa ci si aspetta'
```

Il JSON restituisce il percorso `Data/config/issues/<run>/index.html` **dentro
il profilo selezionato**. `issue.json` contiene il contesto completo e i link;
`summary.json` contiene soltanto metadati da controllare prima di condividere.
La descrizione accetta al più 2.048 byte. Se omessa appare «Not supplied»;
il report non inventa un sintomo.

Nella copia isolata in pausa scegliere **Debug > Create issue report**.
Usa il replay già selezionato, oppure apre il selettore `replay.json`.
Una selezione non avvia il replay. L'azione prende come descrizione l'ultimo
messaggio di replay disponibile; per una descrizione esplicita usare la CLI.
Il comando equivalente sulla stessa finestra è:

```powershell
.\recomp.exe debug-control issue-report '.\sessione' --profile '.\profilo\replay.json' --symptom 'Il sintomo osservato'
```

`--profile` può essere omesso quando esiste già una selezione. `status.issue_report`
espone l'ultimo percorso. L'azione richiede pausa, senza budget/probe/replay
pendenti; osserva lo stato guest senza modificarlo. Il report appartiene al
profilo scelto, che può essere diverso dalla copia usata per aprire il menu.

## Leggere e riprodurre

Il report mostra preflight, risultato conservato e prossima verifica. Il riepilogo
distingue `recorded_frames` dalla finestra `selected_frames` e dai frame
`completed`; il confronto aggiunge `first_difference` e `different_components`. I pulsanti
aprono la directory del report/profilo nel browser, copiano il comando esatto
del launcher locale e, se identificato, aprono il confronto conservato.
La scelta del terminale distingue PowerShell/Windows e shell/Linux; i profili
Windows richiedono `wine64` quando eseguiti su Linux.

I file `reproduce.ps1` e `reproduce.sh` risolvono il profilo rispetto alla propria
posizione. Eseguono il **runtime fissato nel profilo**, con `debug-replay run`,
il relativo `replay.json` e output JSON. Non consultano una build più recente o
ricostruiscono variabili d'ambiente di gioco. Il worker conserva la normale
separazione dalle scritture di partita. Il codice di uscita del replay resta
quello descritto in DBG-05.

Spostare **l'intera cartella del profilo**, compresi report e payload, mantiene
i collegamenti e i launcher. Spostare il solo HTML non trasferisce il caso.
Aprire di nuovo l'HTML dopo lo spostamento aggiorna il comando copiabile in base
alla posizione corrente. Non viene modificato il replay originale per generare
il report. L'esecuzione del comando resta una scelta esplicita successiva.

## Errori e valore della prova

La preflight verifica schema/limiti del profilo, pin della sessione, hash di
runtime/ROM/BIOS/dataset/seed/input/endpoint/libreria e parsing dell'input. Non
esegue istruzioni né carica la libreria native; il worker resta responsabile di
decodifica degli snapshot, compatibilità effettiva e verifica dell'endpoint.

Un input alterato o un profilo incompatibile produce comunque HTML/JSON con
errore e prossima verifica: la CLI standalone restituisce **3** e il percorso
del report. **0** significa report creato con preflight compatibile, non caso
rieseguito o bug risolto. Un percorso di destinazione invalido, lock occupato,
parametri malformati, capacità esaurita o errore di scrittura può impedire la
pubblicazione; l'esito CLI spiega il motivo.

Il risultato precedente viene accettato soltanto se schema e identità disponibili
runtime/ROM/input/libreria/dataset corrispondono. La sua presenza non autentica
la freschezza o ogni impostazione di un vecchio profilo. Il confronto, se presente,
richiede anche ROM/runtime/input/seed/libreria e run corrispondenti. Queste sono
verifiche su byte **già conservati**, senza nuova accettazione di correttezza.
Un risultato assente o incompatibile rimane sconosciuto, non «caso mai eseguito».

## Privacy e limiti

- ROM, BIOS, input, stati, dataset, traduzione native e dump restano allegati
  **privati**, referenziati con link relativi. Nessun upload o messaggio automatico.
- `issue.json` include errori, pin e percorsi locali: è privato. `summary.json`
  esclude percorsi sorgente e byte di gioco, ma sintomo/nome caso possono ancora
  contenere informazioni personali. Il nome «metadati» non equivale a sicurezza
  automatica di condivisione.
- Al più otto cartelle per profilo, compresi tentativi incompleti, e 2 MiB JSON.
  Nessuna sovrascrittura o pulizia automatica. Un lock writer separato coordina
  due creazioni simultanee. I launcher non vengono eseguiti dalla creazione.
- I limiti mailbox impongono un percorso di profilo entro 1.024 byte e una
  richiesta serializzata entro 3.500 byte; abbreviare descrizione/percorso quando
  gli escape superano il limite. La CLI standalone non usa quella mailbox.

La [homepage locale](report_home.md) riconosce `issue.json` dopo rigenerazione.
Prerequisiti, prove effettive e piattaforme non verificate sono nel
[record DBG-16](changes/2026-10-29-add-reproducible-local-issue-reports.json).


## Verifiche del 29 ottobre 2026

Build Windows feature/default bloccate e offline senza warning; 72 test Wine
(collector, controller, menu e frontend), più una riesecuzione mirata dopo la
separazione dei conteggi nel riepilogo. Sette comandi CLI standalone e un launcher
verificano creazione senza esecuzione, importazione readonly della divergenza
conservata e report incompatibile pubblicato. Il caso appena creato da una
registrazione già accettata viene spostato con spazi/apostrofo/Unicode: il launcher
SH/Wine, da cwd estranea, raggiunge l'endpoint completo esatto di due frame.

Menu e CLI nella copia in pausa conservano stato/frame/clock; 37 invocazioni
includono navigazione e rifiuti di selezione/capacità/esecuzione libera/chiusura.
La capacità conserva ogni hash/mtime dei report; il genitore chiude normalmente.
I 28 input originali effettivamente selezionati restano intatti. Browser Chrome
verifica clipboard reale uguale agli argv del launcher già eseguito, apertura
locale della cartella, quoting PowerShell, errore/confronto e layout desktop/mobile.
La revisione del riepilogo verifica 102 frame registrati contro 2 selezionati,
prima differenza 1 e componenti CPU/stato/clock senza nuovo replay. La homepage
indicizza 49 report, otto issue; screenshot ispezionati.

Le prove restano sotto `target/debug-tools-issue/`; report e payload privati sono
ignorati da Git. PowerShell e Windows nativi, Linux-native, altri browser e
controller fisici non sono stati eseguiti. Il confronto native precedente è
stato soltanto importato: la sua parità non viene riaperta o certificata qui.
