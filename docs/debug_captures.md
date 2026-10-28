# Cattura al raggiungimento di un caso (DBG-08)

`debug-capture` e **Debug > Capture on trigger** eseguono una registrazione
completa già disponibile in un worker isolato. Osservano una condizione e
producono un punto derivato al primo **confine di frame completato** successivo.
Il normale gioco resta in pausa. Le condizioni non modificano input, task,
registri, clock o stato della scena per farla comparire.

Usare un runtime compilato con `--features rom-data-read-audit`. Il candidato
Windows verificato localmente è `target/debug-tools-capture/recomp.exe`;
il normale eseguibile non viene sostituito. Le verifiche Windows descritte qui
sono eseguite sotto Wine, non su Windows nativo.

## Preparare un profilo

Servono una [registrazione snapshot/input completa](debug_recordings.md) e la
relativa [sessione isolata chiusa](debug_sessions.md). I parametri comuni sono
quelli di [debug-replay](debug_replays.md). Ogni profilo usa una nuova directory;
la creazione rifiuta sorgenti incompatibili o modificati.

Esempio PowerShell, cattura dopo il cinquantesimo frame del replay:

```powershell
.\recomp.exe debug-capture create `
  --recording 'target/sessione/Data/config/recordings/registrazione' `
  --session 'target/sessione' --dir 'target/cattura-caso' --case DS-20-EUR `
  --frame 50 --limit-frames 100 --timeout-ms 120000
.\recomp.exe debug-capture run 'target/cattura-caso/replay.json'
```

La registrazione deve contenere almeno 100 frame per questo esempio. Il limite
predefinito è la lunghezza registrata; non si prosegue oltre l'input disponibile.
Scegliere esattamente una delle seguenti condizioni:

- **Prima lettura di una risorsa:** `--resource ID --manifest FILE`, con
  `--preset all|bus|host` facoltativo. Il manifest deve appartenere alla stessa
  ROM e ogni intervallo della risorsa deve autenticarsi per hash. `--dataset DIR`
  può fornire sia il manifest sia un dataset runtime esplicito; senza dataset
  il trigger osserva anche il normale percorso raw. Il piano viene copiato e
  identificato nel profilo come `capture-plan.toml`. Si cattura la prima lettura
  strumentata che interseca uno degli intervalli, conservando reader, larghezza,
  servizio Typed/Raw e contesto interprete eventuale.
- **Ingresso in un'istruzione ROM verificata:** `--pc HEX --mode arm|thumb
  --opcode HEX`. PC, allineamento, larghezza, opcode atteso e decodifica vengono
  verificati contro la ROM scelta. Ricavare PC e opcode dall'analisi della propria
  ROM; un valore diverso viene rifiutato. Il trigger osserva l'ingresso effettivo
  nell'interprete **con condizione superata**, prima dell'esecuzione/HLE della
  relativa istruzione. Un PC ereditato da una lettura HLE/DMA non viene scambiato
  per un nuovo ingresso. Non copre istruzioni con condizione fallita, native,
  BIOS o codice RAM.
- **Evento di frame noto:** `--frame N`, relativo all'origine del replay,
  con `1 <= N <= --limit-frames`. Si cattura esattamente dopo N frame completati.

Risorsa e PC richiedono gli hook della feature; anche i profili frame usano lo
stesso contratto di cattura. I profili sono **interprete soltanto**: `--library`
e audit simultanei sono rifiutati. Nessuna traduzione nativa viene compilata.
Identità del runtime selezionato e della registrazione originale restano distinte.

## Eseguire dal menu o dalla CLI

Nel gioco normale aprire Debug, scegliere il `replay.json` con **Choose replay
profile**, poi **Capture on trigger**. Il menu usa lo stesso worker della CLI.
Un profilo senza condizione viene rifiutato da questa azione. **Cancel replay**
annulla anche una cattura; chiudere la finestra proprietaria termina il worker.

```powershell
.\recomp.exe debug-capture cancel 'target/cattura-caso/replay.json'
```

Il profilo conserva `capture.condition`, `capture.limit_frames` e
`stop: "capture-trigger"`. Le verifiche e i lock di DBG-05 restano attivi:
ROM/BIOS, runtime, dataset, origine e input sono identificati, due usi simultanei
sono rifiutati, e il worker non possiede writer di SRAM, slot o backup personali.
Il client può supervisionare un runtime diverso esplicitamente selezionato;
il worker stesso deve avere gli hook richiesti.

`progress.json`, `result.json` e `worker.log` restano sotto
`Data/config/replay/`. Il risultato dichiara il run e il limite effettivo:

- `captured`, uscita **0**: trigger osservato e bundle pubblicato;
- `not-reached`, uscita **7**: limite di frame raggiunto, nessuno snapshot;
- **3** incompatibile, **4** timeout, **5** interrotto, **6** errore del worker,
  secondo la supervisione del replay.

Il menu mostra **CAPTURED** oppure **NOT REACHED**. “Non raggiunto” significa
soltanto non osservato nei collector, nel thread e nella finestra selezionati;
non dimostra che una risorsa non venga mai letta. Interruzione o timeout non
producono una nuova cattura. I vecchi bundle restano indipendenti dal risultato
più recente: confrontare sempre il run.

## Riutilizzare il punto

Ogni successo pubblica una nuova directory `captures/<run>/` nel profilo:

- `capture.ffstate`: stato completo al confine sicuro, ricaricabile normalmente;
- `origin.ffstate` e `input.input`: copie dei buffer autenticati realmente usati,
  conservate anche se i file del profilo cambiano dopo l'esecuzione;
- `capture.json`: motivo, prima osservazione, confine effettivo/frame/clock,
  hash di snapshot, ROM/BIOS/runtime/dataset e input, origine e limiti;
- `debug-catalog.json`: catalogo locale già pronto, con un'entrata **variante**.

Il fingerprint `profile_settings_sha256` identifica la serializzazione compatta
delle impostazioni effettivamente caricate, non una rilettura tardiva del file.
`input_end_exclusive` indica quanti frame della registrazione sono stati applicati;
il file input originale può contenere eventi successivi. Conservare il profilo
con `capture-plan.toml` se occorre riesaminare il piano sorgente della risorsa.

Aprire il catalogo dal [Debug catalog](debug_catalog.md), oppure:

```powershell
.\recomp.exe debug-catalog list 'target/cattura-caso/captures/RUN/debug-catalog.json' --json
.\recomp.exe debug-catalog open 'target/cattura-caso/captures/RUN/debug-catalog.json' `
  --entry 'CAP/RUN' --rom 'rom/la-propria-rom.gba' --no-bios --interp `
  --dir 'target/punto-riutilizzato'
```

Usare il BIOS corrispondente all'origine quando presente, invece di `--no-bios`.
L'apertura crea un'altra sessione isolata; l'importazione in slot usa il percorso
esistente con slot libero e Undo. Il catalogo mantiene **variante**, senza
promuovere la scena DS del nome. La prima osservazione e il momento dello
snapshot sono distinti: una lettura nel frame di osservazione viene conservata
prima di fermarsi al confine successivo, non interrompendo una callback DMA/HLE.

Sono conservati al massimo **otto bundle per profilo**. Il nono avvio è rifiutato
prima del worker; non si sovrascrivono o eliminano punti esistenti. Archiviare
esplicitamente i bundle oppure creare un nuovo profilo. Copie parziali vengono
preparate in una directory temporanea e rinominate soltanto alla pubblicazione.
Tutti i payload restano privati sotto `target/` o altra destinazione ignorata;
non inserirli in commit o pacchetti. Il catalogo personale non viene riscritto.

## Ambito dell'osservazione

Il collector della prima lettura è distinto dall'inventario audit: conserva un
solo match, senza lista di eventi o scarti per cap. Riconosce gli stessi sei
reader di DBG-06, esclude i fetch e normalizza i mirror fisici. Gli intervalli
sono ordinati/uniti, con ricerca per offset; limite 100.000 intervalli della
risorsa e 128 MiB complessivi da autenticare, entro il manifest di 16 MiB.
Non osserva altri thread, accessi host/HLE non strumentati, costanti native,
letture fuori immagine o percorsi EEPROM/GPIO esclusi dagli hook.

La condizione seleziona intervalli autenticati, non prova nuova proprietà dei
dati, completezza dell'estrazione o presenza semantica di una scena. Nessuno
stato o ABI della macchina è stato esteso per conservare il collector.

## Verifiche effettuate

La build Windows isolata parte dal commit pubblico `acd086e` più otto file
selezionati del lavoro DBG-08; le modifiche di estrazione concorrenti restano
escluse. Feature e default compilano senza warning. Sotto Wine passano **66 test**:
cinque cattura, undici replay, tre collector trigger, due regressioni bounded
audit e 45 menu. La correzione della pubblicazione usa i buffer già autenticati,
con prova di modifica esterna del file input dopo l'esecuzione; i test core/menu
con binari identici sono stati mantenuti senza ripeterli.

**25 controlli CLI**, un processo controllato di annullamento e quattro prove
fra client/worker con e senza feature verificano creazione, cattura, limite,
identità/opcode errati, piano modificato, timeout, esclusione native, lock,
annullamento e ritenzione. Sul replay DS-20 esistente i trigger frame, risorsa
e PC producono lo stesso stato completo dopo un frame, con clock 6.241.425.344.
La prima lettura della risorsa è `SpritePeek`: l'aspettativa iniziale del test
su `ObjSourceTable` è stata corretta senza rieseguire le catture già valide.
Il punto è riconosciuto dal catalogo come variante e apre una sessione isolata
con seed identico. Il caso non raggiunto si ferma a due frame, con stato/clock
uguali al controllo di cattura sul secondo frame e senza nuovo snapshot.

La GUI Wine/Xvfb verifica **CAPTURED**, **NOT REACHED**, annullamento e chiusura
del proprietario; stato/frame/clock e progressi della finestra principale
rimangono invariati. Le schermate finali sono state ispezionate. I 203 input
originali selezionati, inclusa la raccolta personale e i due normali eseguibili,
conservano hash e data di modifica. Non si rieseguono le vecchie accettazioni
DBG-04/05/06, native o di estrazione, e non si promuove una scena DS nuova.

Windows nativo, Linux/Wayland, controller fisici e cattura con dataset typed
esplicito non sono verificati in questa unità. Le prove reali usano un piano
sorgente e76 già conservato e il percorso raw; il modello runtime committato
successivo resta distinto. Log, punti derivati e risultati locali si trovano
in `target/debug-tools-capture/`; la loro indisponibilità futura non richiede
ricreare o ripetere prove già chiuse.

Per localizzare una differenza fra interprete/native o raw/typed, usare i
[confronti DBG-09](debug_comparisons.md). Le catture qui descritte producono
varianti riutilizzabili su trigger; il confronto conserva invece stati e stream
osservati prima del drain frontend soltanto al primo confine diverso, con nomi
dei campi e immagini affiancate. I due tipi di profilo restano distinti.
