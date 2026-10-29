# TODO — Strumenti di debug condivisi

Aggiornato il **29 ottobre 2026**. La checklist segue progettazione e sviluppo:
le spunte identificano gli strumenti completati. Gli strumenti devono servire sia
all'utente, attraverso menu e procedure brevi, sia all'assistente, attraverso
comandi riproducibili e risultati strutturati.

**DBG identifica uno strumento; DS identifica una scena di prova.** I casi
DS-01…DS-26 restano nel [TODO debug saves](debug%20saves.md), con i rispettivi
stati e limiti. Non si crea una seconda lista di scene validate.

Priorità attuale: aiutare l'[estrazione ROM](ROM_DATA_TODO.md), concentrandosi
su EUR. Windows è il riferimento desktop; le nuove verifiche funzionali
seguono l'attuale percorso Wine, indicando separatamente quelle native ancora
mancanti. Nessun benchmark è richiesto da questo piano. Il
[frontend Godot](GODOT_FRONTEND_TODO.md) resta rinviato e non è un prerequisito.

Il successore [Debug integrato — DBG2](DEBUG_TOOLS_V2_TODO.md) pianifica
selettori interni, parametri/script da menu, console eventi/comandi e cheat
per il debug. Le sue voci sono nuove e non riaprono DBG-01…DBG-22.

## Basi già disponibili da riutilizzare

Le spunte qui indicano capacità esistenti, con l'ambito documentato nei link;
non significano che siano già raccolte in un unico debugger.

- [x] **Slot di debug:** fino a 99 snapshot e 99 salvataggi standard, copie,
  anteprime e Undo; [Extended saves](../extended_saves.md). Il catalogo locale
  mantiene provenienza, regione e stato dei casi, ma il menu usa nomi generici.
- [x] **Registrazione e replay:** `RECOMP_RECORD_INPUT`, `frames`/`runc`/`verify`
  con input registrati; `runc --state` riprende snapshot in sola lettura.
  Il [workflow di replay](PERFORMANCE_WORKFLOW.md#record-a-battle-from-a-supplied-snapshot)
  documenta il limite attuale sui caricamenti durante la registrazione.
- [x] **Confronti e catture:** verifica CLI degli hash dei frame, test mirati
  di stato completo/clock/audio e dump di framebuffer/PPU/audio. Queste capacità
  hanno ambiti diversi: `verify` da solo non equivale a tutte le prove dei test
  mirati. Riferimenti: [CLI](../../crates/recomp/src/main.rs),
  [confronti](../../scripts/compare_native_backends.py),
  [analisi log](../../scripts/ffta_log_analysis.py).
- [x] **Codice e fallback:** disassemblatore `dis`, trace dell'interprete,
  raccolta e import/export di [label](../labels.md), contatori native/fallback.
- [x] **Dataset e mappa:** estrazione/verifica/preparazione e
  [ROM coverage](ROM_COVERAGE.md) offline via Docker, con identità, revisione
  e data di generazione separate dalle prove ordinarie.
- [x] **Audit ROM opzionale:** collector bus/host e catture filtrate con contesto
  di esecuzione, dietro la feature `rom-data-read-audit`;
  [capacità e limiti](../rom_data_runtime.md#optional-bus-and-host-rom-read-audit).
  Ora esposto dal menu e dalla CLI in DBG-06; le estensioni Bank mantengono
  una prova circoscritta separata, descritta nello stesso documento.
- [x] **Gestione artefatti:** helper e regole di
  [conservazione locale](LOCAL_ARTIFACTS.md) già presenti. La chiusura delle
  verifiche della pulizia interrotta rimane distinta dalla loro esistenza;
  i miglioramenti sono raccolti in DBG-18.

## Regole comuni per i nuovi strumenti

- Stessa operazione e stesso significato dal menu e dal comando equivalente;
  la CLI deve funzionare senza automatizzare clic e coordinate dello schermo.
- Ogni esecuzione identifica binario, ROM/BIOS, dataset, snapshot, input e
  configurazione effettivi. Commit e working tree modificata non bastano a
  identificare un vecchio binario: servono hash e provenienza della build.
- Output breve leggibile, JSON versionato per l'assistente e codice di uscita
  distinto per successo, divergenza, timeout o input incompatibile.
- Diagnostica disattivata nel percorso normale; attivazione esplicita, durata,
  filtri e quantità massima di output. Log troncati e hook non coperti devono
  essere riconoscibili, mai interpretati come assenza di problemi.
- ROM, snapshot, SRAM, registrazioni, dump e asset restano locali e ignorati
  da Git. Le sessioni lavorano su copie e non salvano sulla partita personale.
- L'osservazione deve preservare stato e clock guest; le letture diagnostiche
  non devono alterare registri I/O, conteggi del test o flusso del gioco.
- Una spunta richiede implementazione, istruzioni d'uso, prova del risultato
  atteso e del caso d'errore, evidenza e limiti. La sola presenza di un flag
  o di un'immagine corretta non chiude la verifica.

## P0 — Rendere i problemi riproducibili e aiutare l'estrazione

- [x] **DBG-01 — Diagnosi della sessione e controllo iniziale.**
  Menu «Copia diagnosi» e controllo CLI senza avviare la scena: mostrare profilo,
  build, modalità native/interprete, dataset attivo, ROM/BIOS, slot, input,
  percorso dati effettivo e cartella del report. Segnalare file mancanti,
  generazioni incompatibili e strumenti non compilati nella build scelta.
  **Chiusura:** lo stesso report si ottiene da menu/CLI; un input errato viene
  individuato prima dell'esecuzione e la causa indica come correggerlo.
  **Completato:** [diagnosi della sessione](../debug_diagnostics.md), menu
  «Copy diagnosis» e CLI `diagnose`, JSON comune e report singolo atomico.
  Verificati 47 test Windows sotto Wine, otto casi CLI su dati EUR e menu/appunti
  in una sessione Wine/Xvfb isolata; 139 file protetti invariati. Windows nativo
  e Linux restano da verificare. Native prima dell'avvio è una richiesta esplicita,
  non una compilazione già eseguita. [Record](../changes/2026-10-27-add-shared-session-diagnosis.json).

- [x] **DBG-02 — Sessione di debug isolata.**
  Avviare una copia di lavoro del caso scelto, indicando chiaramente la partita
  attiva. Snapshot load, Undo, autosave, backup e uscita devono restare nella
  sessione; tornare alla partita normale deve essere esplicito.
  **Chiusura:** hash degli originali invariati dopo Load/Undo/riavvio/errore e
  percorso portable trasferibile. **Dipende da:** DBG-01; riusa gli slot esistenti.
  **Completato:** [sessioni isolate](../debug_sessions.md), menu Start/Close
  debug session e CLI `debug-session create/open/inspect`. Copie portable con
  input autenticati, scritture locali e override esterni esclusi. Verificati
  64 test Wine, i 41 del menu finale, sette casi CLI e le prove GUI di operazioni,
  spostamento e ritorno esplicito alla partita normale; 73 input protetti invariati.
  Native Windows/Linux e gameplay native nella copia restano da verificare.
  [Record](../changes/2026-10-27-add-isolated-debug-sessions.json).

- [x] **DBG-03 — Catalogo DS navigabile e interrogabile.**
  Mostrare nome scena, regione, slot, variante, fonte, stato e prova disponibile;
  filtrare dal menu e ottenere gli stessi dati in JSON. Importare in uno slot
  libero con anteprima, senza confondere DS, IN, GEN e numeri delle varianti.
  **Chiusura:** selezionare DS-20 apre il punto previsto nella copia EUR;
  DS-03/DS-15 restano non coperti finché manca una nuova prova. Rilevare cataloghi
  con file mancanti o checksum diversi. **Dipende da:** DBG-01/02.
  **Completato:** [catalogo condiviso](../debug_catalog.md), menu Debug catalog
  e CLI `debug-catalog list/open/import`, filtri, provenienza, checksum e anteprime.
  Verificati 61 test Wine, 15 casi CLI e apertura DS-20/importazione nello slot 2/
  caricamento/ritorno in GUI Wine/Xvfb; 140 input protetti invariati. Le 92 voci
  mantengono distinti DS, varianti e fonti; DS-03/15 restano non coperti.
  Windows nativo, Linux e controller fisici restano da verificare.
  [Record](../changes/2026-10-28-add-shared-debug-save-catalog.json).

- [x] **DBG-04 — Registrazione semplice a partire da uno snapshot.**
  Comandi «Registra/Termina» e corrispondente avvio CLI: associare snapshot
  iniziale, origine dei frame, tasti già premuti e sequenza ordinata degli input.
  In caso di Load/Undo durante la registrazione, arrestare con un messaggio o
  iniziare un segmento esplicito; non indebolire il blocco attuale producendo
  una registrazione apparentemente valida.
  **Chiusura:** il replay raggiunge l'endpoint atteso con stato/clock coerenti,
  anche iniziando con un tasto tenuto premuto. **Dipende da:** DBG-02/03.
  **Completato:** [registrazione da snapshot](../debug_recordings.md), menu
  Start/Stop input recording e CLI `debug-record start/verify`. Snapshot iniziale,
  frame relativi, tasti iniziali espliciti, eventi ordinati ed endpoint autenticato;
  limiti senza eliminazione automatica. Load/Undo load salvano la cattura e
  rifiutano il primo rewind. Verificati 57 test Wine e cinque catture reali DS-20
  con replay identico per stato completo/clock, più spostamento, file alterati,
  sessione attiva, chiusura e fallimento di scrittura. 140 input originali invariati.
  Verifica native, Windows nativo e controller fisici restano aperti.
  [Record](../changes/2026-10-28-add-snapshot-input-recording.json).

- [x] **DBG-05 — Riproduzione con un comando o un pulsante.**
  Un profilo locale contiene caso, binario e dataset selezionati, input, numero
  di frame, condizione d'arresto e timeout. L'utente sceglie «Riproduci caso»;
  l'assistente esegue lo stesso profilo e riceve progressione ed esito JSON.
  Non attivare misure di performance per un controllo funzionale.
  **Chiusura:** ripetibilità dell'endpoint; errore chiaro su input incompatibile,
  timeout e interruzione, senza scritture personali. **Dipende da:** DBG-01…04.
  **Completato:** [profili di replay](../debug_replays.md), menu Replay case/
  Choose replay profile/Cancel replay e CLI `debug-replay create/run/cancel`.
  Worker isolato senza writer di gioco, endpoint esatto, JSON di avanzamento/
  risultato e codici distinti. Verificati 60 test Wine, ripetibilità/spostamento,
  errori/timeout/annullamento/doppio avvio e menu con genitore invariato;
  171 input selezionati invariati. La vecchia libreria native locale completa
  il caso con clock identico e stato diverso: divergenza segnalata, parità
  non accettata. Windows nativo/Linux/controller restano da verificare.
  [Record](../changes/2026-10-28-add-supervised-debug-replay-profiles.json).

- [x] **DBG-06 — Audit delle letture ROM utilizzabile senza test ad hoc.**
  Esporre i collector esistenti con preset, intervallo di frame, filtro indirizzi
  e limite di eventi. Distinguere bus/host, letture typed/raw, fetch esclusi e
  accessi non osservati. Quando disponibile riportare PC/modalità, mantenendo
  distinto il contesto ereditato da DMA/HLE dall'istruzione che legge direttamente.
  **Chiusura:** report di intervalli fisici e byte distinti, confronto con i dati
  noti e scarto dei mirror duplicati; controllo di stato/clock con audit spento
  e acceso. Una lettura fuori owner è una pista, non una nuova famiglia provata.
  **Dipende da:** DBG-01/05 e chiusura delle estensioni audit già in lavorazione.
  **Completato:** [audit ROM](../debug_rom_audit.md), CLI
  `debug-rom-audit create/run/cancel` e menu Audit replay su profili isolati.
  Filtri, finestra, eventi limitati, servizio typed/raw effettivo, scope PC/modo,
  unioni fisiche e confronto con un piano identificato. Troncatura e finestre
  parziali esplicite; feature assente rifiutata. Verificati 61 test Wine, casi CLI
  raw/typed con stato/clock identici, conti indipendenti, menu/cancel e genitore
  invariato; 238 input protetti invariati. Due frame DS20: 12.416 B distinti,
  11.495 B serviti typed con il dataset e76. Nessuna nuova accettazione ROM/DS.
  [Record](../changes/2026-10-28-add-bounded-rom-read-audit-tools.json).

- [x] **DBG-07 — Collegare scene, letture e ROM coverage.**
  Importare gli audit per caso DS ed esporre i livelli «identificato», «servito
  dal runtime», «letto nelle scene selezionate», «condiviso» e «sconosciuto».
  Clic su una regione → risorsa, scene e prove; confronto tra due generazioni
  con data/identità visibili. Conservare separatamente candidati codice e dati.
  **Chiusura:** contare le unioni senza sommare riletture, separare intervalli
  delle famiglie positive da byte osservati, rifiutare report con ROM diversa
  e dichiarare la copertura dei collector. **Dipende da:** DBG-06.
  **Completato:** [mappa e istruzioni](ROM_COVERAGE.md#scene-letture-e-confronto-dbg-07),
  importazione esplicita via Docker, unioni ricostruite dagli eventi, selezione
  dei report e dettaglio risorsa/caso/prove. Schema 3 con confronto datato,
  identità e intervalli aggiunti/rimossi; filtri, gap, TRONCATO/PARZIALE visibili.
  26 test generatore, otto Bash e 12 controlli PowerShell passano; Docker reale
  e browser 1400/620 px verificati. Tre audit DS20 esistenti: 31.061 B come
  minimo troncato; brevi completi 12.416 B, typed 11.495 B. Nessun nuovo replay,
  dato estratto o caso DS promosso; snapshot e76 esplicitamente storico.
  [Record](../changes/2026-10-28-link-rom-coverage-to-scene-audits.json).

- [x] **DBG-08 — Arresto e cattura quando si raggiunge un caso.**
  Permettere condizioni come prima lettura di una risorsa, ingresso in un PC
  verificato o evento noto. Fermarsi a un confine sicuro e creare uno snapshot
  derivato con origine, input e motivo della cattura, pronto per il catalogo.
  **Chiusura:** un trigger noto produce un punto riutilizzabile; un trigger mai
  raggiunto termina al limite dichiarato senza promuovere il caso. Le scene
  ordinarie si raggiungono tramite input reali. **Dipende da:** DBG-03/05/06.
  **Completato:** [catture su trigger](../debug_captures.md), CLI
  `debug-capture create/run/cancel` e menu «Capture on trigger». Prima lettura
  di risorsa autenticata, ingresso interprete in un PC/opcode verificato o
  evento di frame; snapshot al confine sicuro con origine/input/motivo e catalogo
  variante. Limite senza match → `not-reached` (7), nessuno snapshot; otto bundle
  per profilo, senza sovrascrittura. 66 test Wine, 25 casi CLI più annullamento
  e quattro controlli feature, GUI reached/not-reached/cancel/owner-close passano.
  DS20: tre trigger concordano su stato completo/clock dopo un frame; catalogo
  riutilizzabile. 203 originali invariati, nessuna nuova accettazione DS.
  [Record](../changes/2026-10-28-add-bounded-debug-trigger-captures.json).

## P1 — Capire dove e perché compare una differenza

- [x] **DBG-09 — Confronto al primo punto di divergenza.**
  Confrontare interprete/native oppure dati raw/typed riutilizzando i controlli
  esistenti: stato completo, clock, framebuffer centrale/wide, OAM e stream audio.
  Evidenziare il primo frame e i campi diversi, con immagini affiancate e JSON.
  **Chiusura:** localizzare una differenza nota senza limitarsi all'hash finale;
  indicare esattamente quali componenti sono state confrontate.
  **Dipende da:** DBG-05; output pesanti solo attorno alla divergenza.
  **Implementato (2026-10-28):** [confronto per frame](../debug_comparisons.md),
  `debug-compare create/run/cancel` e menu «Compare first difference».
  Riferimento interprete/raw e candidato interprete/native con dataset opzionale;
  campi CPU/Bus serializzati esatti, registri, immagini affiancate e stream audio.
  Finestra massima 3.600 frame, otto report per profilo; snapshot/PNG/audio soltanto
  al primo confine diverso, metadati per prefisso uguale o interrotto.
  Passano 67 test Wine, 20 casi CLI più annullamento, tre controlli default,
  GUI e browser desktop/mobile. DS20: primo frame diverso, clock +4 cicli e
  solo r15; immagini/OAM/audio coincidono. È una localizzazione del confine,
  senza verdetto sulla causa o accettazione della parità native. Tutti i 276
  originali controllati e i normali eseguibili restano invariati.
  [Record](../changes/2026-10-28-add-first-frame-debug-comparisons.json).

- [x] **DBG-10 — Pausa, avanzamento di un frame e ripresa controllata.**
  Fornire scorciatoie e comandi equivalenti per fermarsi e avanzare di N frame.
  Rendere visibili frame guest, clock e motivo dell'arresto; evitare che il tasto
  del debugger diventi anche un input del gioco.
  **Chiusura:** avanzamento esatto, ripresa senza salti di stato o accumuli audio
  causati dalla pausa host. **Dipende da:** DBG-02/05.
  **Implementato (2026-10-28):** [controllo della copia isolata](../debug_control.md),
  menu Step 1/10 e Resume, F6/F7/F8 consumati prima dell'input guest e CLI
  `debug-control status/pause/step/resume`; avvio `debug-session open DIR --paused`.
  Budget di 1–600 frame, input neutro o maschera CLI, frame/clock/motivo visibili,
  code audio svuotate e pacing ripristinato alla ripresa. 69 test Wine, 24 casi
  CLI/GUI e nove default passano; 22 frame menu/tastiera coincidono nello stato
  completo con CLI e le registrazioni di 22/80 frame coincidono con l'interprete
  continuo. Interruzioni, client concorrente e replay durante un budget verificati.
  202 originali invariati; piattaforme native/controller/passi native non verificati.
  [Record](../changes/2026-10-28-add-controlled-debug-frame-stepping.json).

- [x] **DBG-11 — Breakpoint, watchpoint e memoria in sola lettura.**
  Selezionare PC/ARM/Thumb, intervallo letto/scritto o valore osservato; mostrare
  registri e memoria con label, offset ROM e owner. Distinguere l'arresto a
  istruzione da quello a blocco nativo: non dichiarare granularità non supportata.
  **Chiusura:** due trigger mirati verificati e letture diagnostiche senza effetti
  I/O; nessuna modifica arbitraria di RAM necessaria. **Dipende da:** DBG-10/06.
  **Implementato (2026-10-28):** [ricerca e ispezione readonly](../debug_probes.md),
  `debug-control break/watch/inspect` e menu Inspect PC / registers nella copia.
  PC/modo prima del passo, accessi bus/valore dopo il passo con HLE/DMA espliciti;
  ricerca interprete limitata, native rifiutato, watch con feature audit. Registri,
  finestre 1–256 byte, mirror/offset/label e owner da source-plan autenticato.
  Passano 88 test Wine distinti, 21 casi CLI/GUI e nove default più interruzioni:
  PC Thumb e due accessi IRQ reali, letture senza mutazioni, guardie e chiusura.
  Risolta anche la perdita di conferma su sostituzione temporaneamente negata
  da Windows: ripubblicazione della stessa risposta senza rieseguire il comando.
  202 originali invariati; nessuna nuova accettazione ROM/DS/native.
  [Record](../changes/2026-10-28-add-bounded-debug-breakpoints-and-memory-inspection.json).

- [x] **DBG-12 — Ispettore grafico e confronto visivo.**
  Riunire i dump esistenti: BG/OBJ, OAM, palette, tile, finestre, framebuffer
  originale, estensione widescreen e composizione finale. Collegare il pixel
  selezionato alla sorgente quando la provenienza è dimostrata.
  **Chiusura:** spiegare un caso noto di palette/ritaglio/layout con vista e
  report riproducibili, senza alterare registri o rendering guest.
  **Dipende da:** DBG-05/09; riusa i dati estratti e i confronti grafici correnti.
  **Esito:** [ispettore offline](../debug_graphics.md) da copia ferma, menu/CLI,
  massimo otto report privati con buffer/PNG/JSON e confronto precedente.
  BG/OBJ/OAM/palette/tile/finestre, centrale/wide e composizione CPU con/senza UI;
  pixel backing esplicito, provenienza ROM e vincitore finale sconosciuti.
  DS-20 frame22220 spiega padding/ritaglio UI: finestra852×480, canvas con x=-294,
  contenuto effettivo89.856pixel in [114,48,738,192), dentro il clip.58test Wine,
  21casi CLI/GUI, browser desktop/mobile e5report verificati;202originali e
  runtime normali invariati. Effetti GPU, piattaforme native e altre scene non
  certificati. [Record](../changes/2026-10-28-add-readonly-graphics-inspector.json).

- [x] **DBG-13 — Diagnosi di menu, testo e input.**
  Mostrare menu riconosciuto, focus, azione host/guest, bbox e metriche del testo;
  tracciare pressione/rilascio, rimappatura e conferma consumata dal menu.
  Conservare un controllo da tastiera/gamepad e uno da CLI con input registrati.
  **Chiusura:** una conferma doppia o un testo tagliato è attribuibile a un
  passaggio preciso, senza affidarsi al solo screenshot. **Dipende da:** DBG-04/12.
  **Esito:** [trace UI/input](../debug_ui.md), Start/Save dal menu e CLI,
  focus in `status` incluse opzioni/geometry del dropdown, metriche dello stesso
  font, bbox e PNG diagnostico senza modificare rendering o input. Limiti
  30s/1.024eventi/600frame, otto report privati. Autorepeat tastiera:14ripetizioni,
  un solo dispatch; XInput simulato:una rimappatura A, nessuna conferma doppia né
  frame guest. Registrazione CLI A/rilascio verificata su2frame dopo chiusura.
  Testo tagliato attribuito a545,89px richiesti/472disponibili,64/75caratteri,
  clip canvas falso.69test Wine distinti, browser desktop/mobile e6report;
  202originali/runtime normali invariati. Errori di navigazione del test conservati
  separati dai prefissi accettati; controller fisici/testo guest non certificati.
  [Record](../changes/2026-10-28-add-bounded-menu-text-input-diagnostics.json).

- [x] **DBG-14 — Diagnostica audio circoscritta.**
  Rendere selezionabili le catture mixed/PSG/FIFO/HLE, i relativi eventi e un
  intervallo breve attorno al difetto; confronto e riproduzione locale dei dump.
  Esporre mute, Fast e reset rilevanti senza avviare benchmark.
  **Chiusura:** individuare una divergenza nota e distinguere audio generato da
  audio effettivamente consegnato al dispositivo; I/O fuori dal callback audio.
  **Dipende da:** DBG-05/09; limiti di byte/durata e listener hardware dichiarato.
  **Esito:** [catture e confronto audio](../debug_audio.md), menu Start/Save e
  CLI con selezione stream/finestra, WAV/raw/HTML/JSON privati. Buffer guest,
  decisione producer e buffer callback effettivo restano distinti; mute/Fast/
  gain/code/reset espliciti. Nessun I/O/clock/allocazione/lock nuovo nel callback.
  Divergenza conservata al frame8514: mixed81918/81919 −704 contro −576, senza
  riesecuzione.66test Wine distinti e2riesecuzioni mirate, registrazione4frame
  con endpoint esatto, menu/default e CLI/feature, browser desktop/mobile con
  WAV/seek e homepage37indici/8audio passano;197originali e runtime normali
  invariati. Report finale identifica anche seed/BIOS/dataset/build/input.
  Windows/Linux nativi, hardware/listener e altre scene restano non verificati.
  [Record](../changes/2026-10-29-add-bounded-audio-diagnostics.json).

- [x] **DBG-15 — Codice, label e fallback nello stesso rapporto.**
  Collegare PC osservato, disassemblato, label, blocco compilato e motivo del
  fallback, con filtri e catture limitate. Distinguere codice ROM, codice in RAM,
  candidati statici ed esecuzione osservata.
  **Chiusura:** una transizione interprete/native è ricostruibile e l'import di
  label per ROM diversa è segnalato; nessuna percentuale di codice estratto viene
  dedotta dal solo numero di ingressi. **Dipende da:** DBG-01/06/11.
  **Esito:** [trace codice/label/fallback](../debug_code.md), Start/Save menu/CLI,
  PC/modo/backing/disassemblato e scelta effettiva, filtri e prefisso limitato.
  Label ROM diversa rifiutata; ROM/RAM e candidati statici restano distinti.
  Nel prefisso native reale gli eventi17/18 passano da08145c10Thumb compilato a
  0801bde8Thumb mancante:41native/470missing/1IRQ. Stessa libreria con observer
  spento/acceso conserva stato completo e6547blocchi/14991passi nell'intero frame.
  74test Wine, menu/CLI con endpoint registrato2frame esatto, errori/capacità,
  browser desktop/mobile e4report passano; sei input/runtime originali invariati.
  Nessuna nuova parità interprete/native o percentuale di codice estratto.
  [Record](../changes/2026-10-29-add-bounded-code-dispatch-diagnostics.json).

- [x] **DBG-16 — Report del problema pronto da leggere insieme.**
  Generare indice HTML locale, riepilogo e JSON con sintomo, caso, frame, input,
  identità, confronto e prossima verifica suggerita. Pulsanti per aprire cartella
  e copiare il comando esatto; distinguere allegati privati e metadati condivisibili.
  **Chiusura:** l'utente può indicare un report e l'assistente riprodurlo senza
  ricostruire a mano le variabili d'ambiente; nessun invio automatico di file.
  **Dipende da:** DBG-01/05/09; report presente anche in caso di errore.
  **Esito:** [report locale del problema](../debug_issues.md), menu/CLI e indice
  HTML/JSON con sintomo, identità, frame registrati/finestra/completati, prima
  differenza, allegati privati e prossima verifica. Preflight incompatibile salva
  comunque; otto report/2MiB con lock, nessun replay/invio automatico. Launcher
  relativo dopo spostamento con spazi/apostrofo/Unicode raggiunge endpoint2frame
  esatto; clipboard/cartella reali e browser desktop/mobile passano.72test Wine
  distinti+1riesecuzione mirata, menu/CLI con stato guest invariato,28input
  originali intatti; homepage49indici/8issue. PowerShell/Windows e Linux nativi
  restano da verificare. [Record](../changes/2026-10-29-add-reproducible-local-issue-reports.json).

## P2 — Estendere la diagnosi senza accumulare lavoro e dati

- [x] **DBG-17 — Ispettore di task, script e unità.**
  Esporre strutture già verificate, cursori, attori e collegamenti alle risorse,
  con provenienza per campo. Lasciare sconosciuti i campi non decodificati e
  distinguere dati ROM da stato RAM e valori sentinella.
  **Chiusura:** una sequenza nota è ricostruibile da eventi osservati senza
  inventare limiti globali o identità fisse delle unità. **Dipende da:** DBG-06/11
  e dai contratti ROM specifici già completati.
  **Esito:** menu/CLI condivisi, filtri famiglia/contesto e report offline con
  provenienza per campo, risorse dei modelli EUR autenticati e campi sconosciuti.
  Limiti30s/60frame/512eventi/1MiB, otto cartelle2MiBJSON, nessun nuovo snapshot.
  79test Wine, GUI con traccia codice simultanea e stato completo identico:
  70eventi ricostruiscono countdown10→9→8 dello stesso indirizzo task in due
  frame, senza identità persistente. Report menu/filtro vuoto/contesto selezionato
  e browser passano; nessuno script osservato in questa finestra, senza inferire
  non-uso globale. Native Windows/Linux e cattura gameplay native restano aperti.
  [Uso](../debug_gameplay.md), [record](../changes/2026-10-29-add-bounded-gameplay-inspection.json).

- [x] **DBG-18 — Gestore delle sessioni e degli artefatti di debug.**
  Riutilizzare la pulizia esistente e completarne verifica/distribuzione:
  contesti Docker con tutti i moduli richiesti, ricevuta di rimozione, scelta
  delle sessioni da conservare, spazio stimato e limiti per trace/catture.
  Proteggere sessioni attive e cache in uso mediante coordinamento esplicito;
  preservare errori, originali, input e prove, eliminando copie ridondanti concluse.
  **Chiusura:** preview/apply coerenti, nuova sessione e pulizia concorrente non
  si danneggiano, test di errore/interruzione e percorsi con spazi superati.
  **Dipende da:** DBG-01; applicabile fin dai primi strumenti P0.
  **Esito:** preview/apply/--keep con spazio stimato, limiti e ricevute;
  lease comuni per creazione/runtime/pulizia/cache e letture storiche readonly.
  Contesti legacy, falliti, attivi/interrotti e unità con marker annidati protetti;
  nessuna rimozione automatica dei lease o degli originali. Moduli Docker completi,
  derivata con Git e launcher con log. Passano33test Python e34Wine, contesa
  Windows/Python reale in pausa, creazione distinta concorrente, errore/interruzione,
  Docker keep/apply/piano obsoleto/lettore readonly e browser1400/620.
  Tutte le rimozioni nuove sono fixture sintetiche; nessun vecchio sweep o
  replay ROM rieseguito. PowerShell/Windows nativo restano da verificare.
  [Uso](../debug_artifacts.md), [record](../changes/2026-10-29-coordinate-debug-artifact-retention.json).

- [x] **DBG-19 — Registro circolare degli eventi recenti.**
  Conservare una quantità limitata di input, cambi scena, letture selezionate,
  fallback ed errori; congelarla su richiesta o trigger, senza dump continui
  dell'intera macchina. Indicare eventi scartati e finestra effettiva.
  **Chiusura:** limite rispettato anche su un'esecuzione lunga e contesto utile
  salvato su un errore controllato. **Dipende da:** DBG-08/16/18.
  **Verificato (29 ottobre):** finestra mobile1.024eventi/256KiB,8KiB/evento;
  input effettivo, firma UI parziale,16chiavi ROM/frame e dispatch/errori.
  Menu/CLI Freeze/Save e trigger errore salvano HTML/JSON senza snapshot.
  Un milione di eventi sintetici rispetta i cap; GUI resta attiva dopo31s,
  espelle60eventi e salva contesto utile su errore controllato senza cambiare
  stato guest.81test Windows/Wine e11Python passano, observer interprete due
  frame identico e browser desktop/stretto verificati. Default letture non
  disponibili distinto da zero; piattaforme native non verificate.
  [Uso](../debug_recent.md), [record](../changes/2026-10-29-add-recent-event-ring.json).

- [x] **DBG-20 — Controllo locale di una sessione aperta.**
  Valutare un canale strutturato locale per interrogare stato, mettere in pausa,
  avanzare e catturare, condiviso con i comandi del menu. Preferire la soluzione
  più piccola che consenta all'assistente di operare senza clic fragili.
  **Chiusura:** comandi versionati, timeout/cancellazione, identificazione della
  sessione e risposta strutturata; accesso locale esplicito e nessun servizio
  esposto per impostazione predefinita. **Dipende da:** DBG-01/02/10/16.
  **Verificato (29 ottobre):** esteso il canale su file già condiviso dal menu,
  senza networking. Query status/cancel separate dal client operativo in attesa;
  cancellazione con ID preciso, deadline e timeout configurabile20–120000ms,
  identità manifest/run/richiesta e risposte JSON anche su errore locale.
  30test Wine distinti e un rerun mirato sulla classificazione closed; GUI reale
  default19CLI prova stato durante attesa, ID errato, annullamento parziale,
  scadenza250ms con arresto server, cattura readonly e sessione errata/chiusa.
  Prove DBG10/11/12/16 riusate, nessun vecchio replay riaperto.
  [Uso](../debug_control.md), [record](../changes/2026-10-29-complete-local-session-control.json).

- [x] **DBG-21 — Campagne mirate di casi e regressioni.**
  Selezionare DS, famiglie o transizioni interessate da una modifica; mostrare
  esiti positivo/negativo/incompatibile/non eseguito e prove precedenti riusate.
  I dati di gioco restano locali: i test pubblici del runner usano fixture sintetiche.
  **Chiusura:** il rapporto distingue «zero letture in questa scena» da
  «funzionalità non usata nel gioco»; nessun replay automatico di tutte le prove
  concluse per ricostruire il contesto. **Dipende da:** DBG-03/05/07/09/18.
  **Verificato (29 ottobre):** manifest/pin e selezione esplicita DS/famiglia/
  transizione, quattro esiti e riuso delle prove. Report senza esecuzione;
  run-case delega un solo profilo al supervisore esistente con motivo/receipt
  e riapertura esplicita per prove precedenti.32casi/8output/2MiB, zero nella
  finestra distinto dall'uso globale sconosciuto.23test Python, dispatch pubblico
  sintetico, due prove DS20 riusate readonly, Bash/Docker e browser passano;
  zero nuovi frame guest, nessun build/runtime/vecchia accettazione ripetuto.
  [Uso](../debug_campaigns.md), [record](../changes/2026-10-29-add-targeted-debug-campaigns.json).

- [x] **DBG-22 — Distribuzione e guida comune degli strumenti.**
  Documentare pochi percorsi completi: segnalare un bug, registrare una scena,
  confrontare due runtime, cercare letture non coperte e liberare i temporanei.
  Allineare help/menu/CLI; prevedere avvio PowerShell e Docker per gli strumenti
  offline quando utile, con log d'errore sempre rintracciabile.
  **Chiusura:** procedure ripetibili dal percorso portable spostato, prerequisiti
  espliciti, pacchetti senza dati personali e matrice Wine/Windows nativo/Linux
  dichiarata. **Dipende da:** gli strumenti effettivamente selezionati; non
  richiede completare tutto il piano né il frontend Godot.
  **Verificato (29 ottobre):** [guida comune](../debug_tools.md) con cinque
  percorsi completi, prerequisiti/matrice e log persistenti; help allineato,
  launcher PS/Bash/Python/Docker. Dieci test pubblici packager,13PowerShell7.4.6
  Linux simulati, due TinyCC sintetici e ZIP offline readonly passano. Bundle
  spostato con spazi/Unicode: registrazione/menu, due runtime, bug/audit/pulizia,
  browser1400/620;5frame nuovi totali, endpoint completo/clock identico anche
  col binario finale rimappato.76file pubblici/75payload autenticati, Data vuota,
  nessun ROM/BIOS/progresso/log/fixture/traduzione, prefissi personali rifiutati.
  Windows nativo/PS5.1 e runtime Linux recente non certificati; normali runtime
  e input originali invariati. [Record](../changes/2026-10-29-distribute-shared-debug-tools.json).

## Mantenere gli strumenti completati

DBG-01…DBG-22 sono completati nell'ambito e sulle piattaforme dichiarate nei
record. Partire dalla [guida comune](../debug_tools.md) per scegliere un percorso;
la numerazione resta stabile. Le limitazioni native/hardware e le divergenze
segnalate non sono prove superate e non riaprono automaticamente le unità concluse.

Per nuovi lavori scegliere un criterio mancante o una regressione concreta,
registrando motivo, input cambiato e prossimo controllo nel
[TASK_HANDOFF.md](TASK_HANDOFF.md). Riusare le prove esistenti; non riprodurre
tutti i casi per ricostruire il contesto. Aggiornare documento e record coerenti
con l'esito effettivo. Questa checklist non sostituisce le priorità ROM in corso.
