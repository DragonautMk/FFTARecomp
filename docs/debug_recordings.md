# Registrare gli input da uno snapshot — DBG-04

La registrazione nuova associa uno snapshot iniziale, una sequenza relativa di
input e uno stato finale. Si usa dentro una [copia di debug](debug_sessions.md),
anche aperta dal [catalogo DS](debug_catalog.md). La partita personale non è
una destinazione di registrazione.

## Dal menu

Mettere in pausa la copia e scegliere **Debug → Start input recording**.
Il punto attuale viene conservato senza avanzare la macchina. Scegliere Resume,
riprodurre il problema, poi tornare in **Debug → Stop input recording**.
Il titolo della finestra mostra `[REC]` durante la registrazione. La chiusura
normale della copia conclude la registrazione; un errore runtime la lascia
incompleta.

La cartella è `Data/config/recordings/recording-…` nella copia portable. Il log
riporta il percorso completo e il menu conferma la cartella salvata. Vengono
conservati al massimo otto tentativi prima di chiedere di spostarli: nessuno
viene cancellato automaticamente. Ogni tentativo ha limiti di 360.000 frame,
200.000 eventi e 8 MiB di input. Errori di scrittura, cambio di timeline o di
dimensioni del rendering interrompono il tentativo e lo segnalano nel menu.

Load e Undo di un caricamento concludono la registrazione e **rifiutano quel
primo comando**: ripetere la scelta dopo aver letto il messaggio. Il rewind non
viene inserito in una sequenza lineare. Salvare uno snapshot e annullare una
scrittura di file restano possibili. Riavvio, cambio salvataggio e nuova copia
richiedono di terminare prima la registrazione.

## Dalla CLI

Preparare una copia con uno snapshot, per esempio con `debug-catalog open` o
`debug-session create --state`. Quindi, con la copia chiusa:

```powershell
.\recomp.exe debug-record start 'C:\Debug\DS-20'
```

Il comando apre la copia con gli stessi input fissati e comincia a registrare
prima del primo frame di gioco. La registrazione parte dal punto effettivo dopo
l'inizializzazione del frontend, conservato in `start.ffstate`. Il menu permette
di terminarla; il processo supervisore attende la chiusura della copia.
Non sono accettati override del gioco o una seconda apertura concorrente.

Per il controllo funzionale di una registrazione fatta con l'interprete:

```powershell
.\recomp.exe debug-record verify 'C:\Debug\DS-20\Data\config\recordings\recording-…' `
  --session 'C:\Debug\DS-20'
```

Il controllo usa ROM/BIOS e dataset fissati nella copia chiusa. Verifica hash,
origine e ordine degli eventi, poi riproduce su una macchina temporanea senza
scrivere salvataggi. Il JSON `ffta-recording-verification`, schema 1, confronta
l'intero stato serializzato e il clock finali. Divergenza o errore terminano con
codice 1; il tempo massimo del replay è 120 secondi. Non è un benchmark.

Il binario deve coincidere con quello della registrazione. Questo controllo
circoscritto richiede una registrazione interpreter. I
[profili DBG-05](debug_replays.md) permettono invece di scegliere un runtime,
avviare dal menu, annullare e distinguere gli esiti della riproduzione. I vecchi comandi
`frames/runc/verify --input` continuano a leggere il file `gba-input v1`.

## Contenuto e semantica

- `start.ffstate`: stato completo all'inizio, legato a ROM e BIOS.
- `input.input`: `gba-input v1`, con frame relativi da zero, maschera iniziale
  esplicita dei tasti già premuti e cambi successivi. Le maschere sono active-low:
  `3ff` rilascia tutti i tasti. Gli eventi dello stesso frame mantengono l'ordine
  nel file; mouse e tasti seguono le semantiche di replay esistenti.
- `end.ffstate`: stato finale di una registrazione conclusa normalmente.
- `recording.json`: schema 1, tipo `ffta-input-recording`, stato recording/
  complete/incomplete, hash del binario/ROM/BIOS/dataset e dei payload, frame e
  clock assoluti di origine/fine, dimensioni del rendering e motivo di arresto.
- `av.cfg`, `input.cfg`: configurazione iniziale, utile per riprodurre il contesto.

I nomi dei payload sono relativi. La cartella può essere spostata insieme alla
copia o da sola, conservando una sessione compatibile per il controllo. Un
processo interrotto non pubblica un risultato complete. Un checkpoint integro
non dimostra da solo che gli input raggiungano il risultato desiderato: serve
il confronto finale, e una divergenza rimane visibile.

Il percorso storico `RECOMP_RECORD_INPUT` resta disponibile e separato: registra
frame assoluti e una copia iniziale della SRAM, senza il nuovo manifest. Il
comando Stop può chiuderlo, ma non lo trasforma in una registrazione da snapshot.
I suoi blocchi su Load/Undo/riavvio rimangono validi. Non si possono sovrapporre
registrazione storica e nuova.

Snapshot, input, configurazione e report contengono dati privati. Restano nella
copia ignorata da Git e non vengono inseriti nei pacchetti.

## Verifiche del 28 ottobre 2026

Build release Windows offline da base pubblica congelata `2fbf9f7` e tredici
file degli strumenti sovrapposti, con cache separata. Candidato finale
`target/debug-tools-recording/recomp.exe`, SHA-256
`fcd271732ca1a898392ef3a589133a1a45268a8fc9fd98048f109d060c641db4`.
Gli eseguibili normali e il lavoro concorrente di estrazione sono preservati.

- **57 test Wine:** sette del registratore, 44 del menu e sei delle sessioni.
  Comprendono origine non zero e tasto A inizialmente premuto, replay esatto,
  ordine degli eventi, limiti, rewind, scritture fallite, checksum/ROM/build,
  divergenza dichiarata e spostamento della cartella.
- **Cinque catture reali EUR DS-20** riproducono stato serializzato completo e
  clock identici: avvio CLI, avvio/stop menu, arresto per Undo load e Load,
  chiusura con registrazione attiva. Un replay aggiuntivo dopo spostamento passa.
- **15 comandi CLI di prova**, oltre ai processi GUI: creazione delle copie,
  confronto degli endpoint, rifiuti per file alterati, cattura incompleta,
  sessione già aperta e sessione senza snapshot.
- Un errore reale simulato nella pubblicazione di `end.ffstate` della sola copia
  restituisce codice 1 e risultato sessione fallito; non pubblica una registrazione
  complete. Gli hash/mtime dei 140 input originali e degli input fissati nelle
  copie rimangono invariati. Schermate di Start/Stop e rifiuto Load/Undo ispezionate.

La prima build dei test ha richiesto una conversione del target mouse da u8 a
u16. Dopo i test e le prime tre catture sono stati rifiniti soltanto i messaggi
e la propagazione dell'errore di chiusura; la build finale e le altre due catture
verificano quei percorsi. Evidenza locale in `target/debug-tools-recording/`:
`test-results.json`, `gui-results.json`, `final-gui-results.json`, `cli-results.json`
e `final-cli-results.json`.

Windows nativo, Linux/Wayland, controller fisici, nuovi percorsi mouse nel gioco
e registrazioni native non sono verificati da questa unità. Nessuna nuova
accettazione DS, estrazione ROM, prova di performance o modifica ad Android/Godot.
