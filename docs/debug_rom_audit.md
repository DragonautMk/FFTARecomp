# Audit delle letture ROM — DBG-06

L'audit osserva le letture di dati ROM durante un replay isolato. Non estrae
nuovi asset, non modifica i byte del gioco e non aggiorna la copertura o le prove
DS. Riusa i collector bus/host già presenti, aggiungendo filtri, finestra di
frame, limite di eventi e percorso effettivo typed/raw.

Serve un eseguibile compilato con **`--features rom-data-read-audit`**. La feature
rimane disattivata nelle build normali. Se manca, la richiesta è rifiutata:
un report vuoto non viene presentato come assenza di letture.

## Preparare e avviare

Partire da una [registrazione completa](debug_recordings.md) e dalla sua
[copia chiusa](debug_sessions.md), come per i [profili di replay](debug_replays.md):

```powershell
.\recomp.exe debug-rom-audit create `
  --recording 'C:\Debug\DS-20\Data\config\recordings\recording-…' `
  --session 'C:\Debug\DS-20' --dir 'C:\Debug\audit-DS-20' --case 'DS-20-EUR' `
  --preset all --from 0 --to 102 --limit 20000 `
  --known-manifest 'C:\Dati\FFTA\manifest.toml'

.\recomp.exe debug-rom-audit run 'C:\Debug\audit-DS-20\replay.json'
```

Create usa una destinazione nuova e conserva il profilo dell'audit insieme agli
input. Run e Cancel usano lo stesso worker, timeout, annullamento ed esiti della
CLI `debug-replay`. Non occorrono test Rust ad hoc, Python sul computer dell'utente
né automazione di coordinate per l'assistente.

I filtri sono:

- **`--preset all|bus|host`**: tutti i punti strumentati, solo bus oppure solo
  host selezionati. Il preset predefinito è all.
- **`--from N --to N`**: frame relativi allo snapshot, intervallo `[from,to)`.
  Il replay procede comunque fino all'endpoint registrato; vengono osservati
  solo i frame scelti. Senza to, la finestra termina alla fine della registrazione.
- **`--range START:END`**: intervallo di offset **fisici** `[start,end)`, in
  decimale oppure `0x` esadecimale. Ripetibile fino a 32 volte, per esempio
  `--range 0x370000:0x380000`. Non sono indirizzi guest `0x08…`. Il filtro
  include le letture che intersecano la selezione, mantenendone tutta la larghezza.
- **`--limit N`**: da 1 a 50.000 eventi distinti, predefinito 20.000. I duplicati
  mantenuti aumentano il conteggio richieste. I nuovi eventi oltre il limite
  incrementano dropped_requests e rendono esplicito truncated.

Per confrontare una generazione già disponibile di dati decodificati,
`--dataset DIR` ne copia e autentica i file come risorse del runtime scelto;
la registrazione sorgente rimane intatta. L'endpoint deve continuare a coincidere.
Senza override valgono le risorse fissate nella sessione sorgente. Non viene
pubblicato né rigenerato un dataset.

`--known-manifest FILE` fornisce un piano sorgente da confrontare. Si controllano
schema, identità/dimensione ROM e limiti degli intervalli, copiandone i metadati
nel profilo. È un riferimento di provenienza, distinto dalla prova di consumo.
Non si riestrae la ROM per verificare il piano. Senza il flag si usa il manifest
del dataset selezionato, se presente; altrimenti known_plan.compared è false
e i byte fuori piano non vengono inventati.

## Dal menu

Preselezionare il profilo con `play --debug-replay FILE` o **Choose replay profile**,
poi scegliere **Debug → Audit replay**. Il menu richiede un profilo con audit
configurato. Anche Replay case esegue l'audit se il profilo lo contiene.
Cancel replay e la chiusura della finestra fermano il worker. La partita normale
resta in pausa e conserva i suoi progressi.

L'esito JSON del replay indica il report, la troncatura e il completamento della
finestra. **Passed riguarda l'endpoint del replay**; un audit troncato o parziale
rimane tale anche quando quell'endpoint coincide.

## Leggere il report

Il profilo conserva solo l'ultimo `Data/config/replay/rom-audit.json`, schema 1,
tipo `ffta-rom-read-audit`, con caso, run e identità degli input. Un errore o
un'interruzione prima della raccolta può non produrre un nuovo audit: controllare
che il suo run coincida con quello del risultato. Un vecchio report non è una
nuova osservazione e un report mancante non significa zero letture.

Gli intervalli sono fisici, ordinati e uniti: letture sovrapposte, larghezze
diverse e mirror ROM non aumentano due volte observed_distinct_bytes. I contatori
bus e host possono sovrapporsi; bus_host_shared_distinct_bytes misura proprio
quell'intersezione. I totali per owner del piano possono sovrapporsi e non vanno
sommati per ricostruire l'unione globale.

Ogni evento distingue il lettore e il servizio effettivo **Typed**, **Raw** o
**Unclassified**, indipendente dall'esistenza di un owner nel manifest. I totali typed/raw possono sovrapporsi sugli stessi byte in percorsi diversi.
Il report mantiene conteggio richieste e primo/ultimo frame guest per chiave. Gli eventi
oltre il limite non contribuiscono agli intervalli mantenuti; non sono byte
provati come assenti o non consumati.

Il PC e la modalità ARM/Thumb, quando disponibili, sono uno **scope interprete**:
HLE e DMA possono ereditarlo. Non provano che quell'istruzione abbia eseguito
una LDR diretta all'indirizzo. None indica native/senza scope osservato, senza
inventare un PC. Per attribuire un operando occorre ancora verificare il lettore.

Sono coperti il bus e gli helper selezionati AudioValue, SpritePeek, SpriteList,
ObjSourceTable e KeyboardWidths. Restano espliciti i limiti:

- fetch delle istruzioni interprete esclusi; il conteggio non misura fetch native;
- altri accessi host diretti, fast path HLE typed non strumentati e altre thread;
- costanti native incorporate, autenticazione di codice/firme e letture fallite;
- GPIO/EEPROM, regioni fuori immagine, intervalli/frame filtrati ed eventi scartati.

Un accesso fuori dal piano conosciuto è un indizio da studiare, non una nuova
famiglia estratta. Questi report non sono la mappa complessiva della ROM: il
collegamento fra scene, risorse e livelli della ROM coverage rimane DBG-07.

## Verifiche del 28 ottobre 2026

Build release Windows offline su base pubblica congelata `2fbf9f7` con i soli
file degli strumenti e hook di audit selezionati. Il grande hub runtime viene
compilato dalla sua versione pubblica più quattro sostituzioni di hook; le
modifiche concorrenti di estrazione restano nel workspace e fuori dalla build.
Candidato finale `target/debug-tools-audit/recomp.exe`, SHA-256
`84f2bdc64e1fa6180d89b357204f6a23a572769436ccd4054b6cc042714a3343`.
Gli eseguibili normali Windows/Linux sono preservati.

- **61 test sotto Wine:** 6 audit, 2 collector bounded, 8 replay e 45 menu.
  Verificano stato completo e clock uguali con osservazione spenta/accesa,
  esclusione fetch, scope, typed/raw, filtri, limite, mirror, piano noto,
  durata/drop e proiezione invariata dei collector storici.
- **CLI DS-20:** replay di 102 frame con endpoint e clock identici. La finestra
  completa di due frame osserva 12.416 B sia raw sia con il dataset e76: 18.755
  eventi raw e 18.754 typed. Il percorso typed serve 11.495 B distinti; altri
  997 B sono osservati raw, con sovrapposizione di 76 B fra percorsi. Il piano
  e76 contiene 11.495 B dell'unione; 921 B osservati restano fuori da quel piano,
  senza una classificazione automatica come nuove risorse.
- **Troncatura:** 102 frame raggiungono il limite di 50.000 eventi: 2.557.121
  richieste corrispondenti, 1.251.772 scartate e 31.061 B mantenuti. Il risultato
  del replay passa, l'audit resta dichiaratamente troncato. Anche un limite di
  8 eventi è verificato senza modificare stato o clock.
- **15 casi CLI finali**, oltre ai riferimenti già accettati, coprono raw/typed,
  preset host/bus, intervalli e finestra, limite, profilo senza audit e riferimenti
  mancanti/modificati. Un controllo Python indipendente ricostruisce unioni,
  intersezioni col piano, filtri e conti richieste/scarti. Altri due controlli
  confermano il rifiuto di create senza feature e l'uso esplicito di un worker
  abilitato da un supervisore senza feature.
- **GUI Wine/Xvfb:** Audit replay, avvisi TRUNCATED/PARTIAL, annullamento e
  chiusura del worker passano. Stato/frame/clock e progressi della partita
  genitrice invariati; 238 input selezionati della raccolta/ROM/registrazione/
  dataset mantengono hash/mtime. Schermate finali ispezionate.

La prova reale ha rilevato il manifest e76 di 7.926.499 B oltre il limite locale
iniziale di 4 MiB: i lettori condivisi ora usano il limite di 16 MiB già definito
dal progetto. Il primo controllo assumeva erroneamente che 50.000 eventi bastassero
per tutti i 102 frame: il report corretto resta troncato e la prova completa usa
una finestra di due frame. Quei risultati e tentativi sono conservati. Il primo
avvio GUI nel prefisso Wine condiviso ha fallito prima della finestra; il prefisso
grafico dedicato passa senza una modifica al runtime. I test accettati non sono
stati ripetuti per ricostruire il contesto.

Evidenza in `target/debug-tools-audit/`: `test-results.json`, `cli-results.json`,
`cli-summary.json`, `default-cli-results.json`, `gui-results.json`,
`default-hook-verification.json` e tentativi conservati. Entrambe le build sono
senza avvisi; il codice dei quattro percorsi core con feature spenta è uguale
alla preimmagine dopo rimozione degli hook e normalizzazione dei fine riga.

Windows nativo, Linux/Wayland e controller fisici restano non verificati.
Non sono stati riaperti confronti di parità native, campagne Bank/F09, estrazione,
nuove prove DS, performance o Android/Godot. Il collegamento dei report alla ROM
coverage resta il successivo DBG-07.

## Collegamento alla mappa ROM

La [ROM coverage](development/ROM_COVERAGE.md#scene-letture-e-confronto-dbg-07)
importa i report conservati con `--audit` / `-AuditPath`, senza un nuovo replay.
Permette di selezionare le finestre, vedere byte distinti per risorsa/caso,
risalire alle prove dalla mappa e confrontare due fotografie. Gli intervalli
osservati restano distinti dalle famiglie positive; limiti, filtri, troncamento
e manifest diversi restano visibili. I report devono appartenere alla stessa ROM.
