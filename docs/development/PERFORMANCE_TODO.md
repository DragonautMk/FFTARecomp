# TODO ottimizzazioni desktop FFTARecomp

Piano del 20 ottobre 2026, basato sul codice alla revisione `e39cdb6`.
Priorità: stabilità, correttezza del gioco, compatibilità Linux/Windows e
semplicità del codice. Il problema segnalato riguarda Ubuntu 26 e possibili
rallentamenti nei turni nemici. Le caselle indicano il completamento di tutti
i criteri di ciascuna attività; i progressi parziali sono registrati qui sotto.

La sequenza consigliata è **misurare il rallentamento reale, eliminare lavoro
ridondante, intervenire sul sottosistema dominante, poi valutare altro lavoro
sulla GPU**. Un aumento dell'uso GPU, da solo, non è un risultato utile.

## Stato dell'esecuzione

Prima tranche P0 in corso: confronto Linux nativo/Windows con verifica separata,
hash finale opzionale dello stato macchina, diagnostica bounded dei tempi per
frame e profilo CPU della finestra, con upload/acquisizione/encoding/invio
e presentazione separati. Il [workflow](PERFORMANCE_WORKFLOW.md)
descrive comandi, interpretazione e limiti. PERF-01/02/03 restano aperti: mancano
un replay identificato del turno nemico lento, misure PPU/GPU distinte e la
copertura completa dei criteri. Nessun guadagno sui turni AI è ancora dimostrato.

In assenza di un replay AI disponibile si procede sulle ridondanze indipendenti
verificabili: PERF-05 ha una prima implementazione di riuso delle LUT e della
texture FPS; PERF-06 conserva i buffer FIFO audio e risolve la modalità di pacing
all'avvio. Restano aperti i criteri più ampi, i confronti di prestazioni e le
ulteriori cache di pannelli/mondi; nessuna modifica accelera artificialmente
clock o logica del gioco.


Passi già implementati e verificati, senza chiudere prematuramente le attività:

- [x] Confronto nativo con verifica immagini e stato finale opzionale.
- [x] Profilo CPU per frame e per fasi della presentazione, disattivato di default.
- [x] Cache LUT per contenuto e revisioni del contatore FPS, con parità CPU/GPU.
- [x] Riuso dei buffer FIFO audio e modalità di pacing risolta all'avvio.

## Cosa sappiamo dal codice

- La GPU è già usata per scalatura, griglia e composizione dei livelli UI:
  [present.rs](../../crates/screen/src/present.rs), `Presenter::present`, e
  [ui_layer.rs](../../crates/screen/src/ui_layer.rs), `Gpu::draw`.
  Rendering GBA, preparazione dei livelli e filtri temporali restano sulla CPU.
- `Gpu::update` ricarica texture UI, mondo e LUT a ogni aggiornamento del
  livello. La LUT contiene 32.768 colori; anche il contatore
  [FPS](../../crates/recomp/src/fps.rs), ridisegnato circa una volta al secondo,
  percorre questo caricamento a ogni presentazione.
  I livelli possono avere mondi/LUT differenti: non vanno unificati alla cieca.
- Nella baseline di [play.rs](../../crates/recomp/src/play.rs), `emu_ema_ms` comprende esecuzione,
  aggiornamento UI e `Temporal::push`, ma esclude `Temporal::compose`,
  presentazione, attese e parte del lavoro audio/persistenza. Il valore
  `native %` confronta blocchi nativi con singole istruzioni interpretate:
  non misura la percentuale di tempo CPU. La prima tranche sostituisce questa
  percentuale con conteggi separati e aggiunge un profilo CPU per fase.
- Un errore restituito da `Presenter::present` fa abbandonare il presenter GPU
  per il resto della sessione. Anche il timeout nell'acquisizione del frame
  passa oggi da questa strada. È una possibile causa di peggioramento
  improvviso, da verificare nel log; non è ancora la diagnosi del lag segnalato.
- [Temporal](../../crates/screen/src/blend.rs) elabora la storia dei frame
  emulati, con sei frame per la modalità Smart. La fusione usa colori già
  codificati per il display: spostarla in spazio lineare cambierebbe l'immagine.
- La [PPU](../../crates/gba-core/src/ppu.rs) dispone già di un percorso parallelo
  per le sole righe aggiuntive dello zoom, con test di equivalenza. È
  disabilitato per default su desktop; il pool persistente richiede la feature
  `parallel-ppu`, altrimenti vengono creati thread scoped durante il rendering.
- Il [benchmark precedente](../windows_runtime_performance.md) misura una
  sequenza introduttiva headless sotto Wine. Non dimostra il costo di un turno
  nemico, la fluidità della finestra o il comportamento di Windows nativo.

## P0 — Rendere il problema riproducibile

- [ ] **PERF-01 — Registrare scene e baseline rappresentative.**
  Rischio basso; primo passo, nessuna dipendenza.
  Preparare replay separati per turno nemico lento, movimento/effetti in
  battaglia, dialoghi/menu e zoom elevato. Conservare ROM, save e registrazioni
  solo nelle directory locali ignorate. Registrare build, backend, risoluzione,
  zoom, filtri, VSync, modalità audio, driver e cache fredda/calda.
  Estendere [benchmark_windows_runtime.py](../../scripts/benchmark_windows_runtime.py)
  al rilevamento corretto della piattaforma e riusare il confronto deterministico.
  **Completato quando:** lo stesso turno si ripete da input/stato iniziale
  controllati e ci sono almeno cinque esecuzioni alternate per confronto.
  Distinguere rallentamento host da un'attesa già presente nel gioco quando
  l'emulazione mantiene circa 59,73 frame al secondo.

- [ ] **PERF-02 — Misurare tempi, picchi e attese separatamente.**
  Rischio basso; dipende da PERF-01.
  In `play.rs`, `main.rs`, PPU e presenter aggiungere diagnostica opt-in:
  esecuzione guest, PPU, preparazione UI, filtri, handoff audio, salvataggio,
  upload, acquisizione della superficie, invio dei comandi e attesa del clock.
  Evitare timer per singola istruzione e log sincroni per frame. Non sommare
  intervalli annidati come se fossero indipendenti. Contare distintamente
  blocchi nativi, fallback, hook intenzionalmente interpretati e frame incompleti.
  **Avanzamento:** il censimento `RECOMP_COST_FROM` usa ora lo stesso dispatch
  nativo del runtime, preservando gli hook audio/sprite/mouse. Distingue blocchi
  nativi, traduzioni mancanti, passi di sistema e hook interpretati; attribuisce
  cicli guest, non tempo CPU host, e resta separato dai confronti temporali.
  Sono disponibili anche tempi CPU per frame delle scanline GBA, delle righe
  zoom e del resto del core (`--ppu-profile` / `FFTA_PROFILE=1`), con controllo
  dei conteggi, campioni limitati e nessuna modifica del clock/stato guest.
  **Completato quando:** il report contiene mediana, p95, p99, massimo,
  frame oltre 16,74/33,49 ms, durata del turno e underrun audio; l'overhead della
  diagnostica è misurato. Il tempo GPU usa timestamp opzionali e lettura
  differita dove supportati: il tempo di `submit` non è il tempo di esecuzione
  GPU. Le [API Queue di wgpu 26](https://docs.rs/wgpu/26.0.1/wgpu/struct.Queue.html)
  descrivono invio, trasferimenti e timestamp.

- [ ] **PERF-03 — Fissare i controlli di correttezza e la procedura A/B.**
  Rischio basso; dipende da PERF-01.
  Riutilizzare replay, confronto interprete/nativo, framebuffer/OAM e test
  savestate esistenti. Per modifiche al core confrontare anche registri, RAM,
  IO, VRAM, cicli guest, save e campioni audio deterministici prima del device.
  I contatori di blocchi possono cambiare con una diversa traduzione:
  non confonderli con lo stato del gioco.
  **Completato quando:** ogni confronto ha configurazione e hash dei binari,
  verifica separata dal timing e una procedura per tornare alla baseline.
  Controllare che il limite `max_steps` non faccia avanzare storia temporale e
  contatori di frame completi quando `run_frame_native` non completa un frame.

## P1 — Correzioni circoscritte e lavoro ridondante

- [ ] **PERF-04 — Recuperare dagli errori grafici temporanei.**
  Rischio medio; dipende da PERF-02/03.
  In `Presenter::present` e `PlayApp::present_rgba` distinguere timeout, superficie
  da riconfigurare, perdita del device e fallimento definitivo. Un timeout
  isolato dovrebbe poter saltare quella presentazione e riprovare, senza
  disattivare per sempre la GPU. Definire tentativi limitati, conteggi e un
  fallback software funzionante; evitare cicli di ricreazione o log continui.
  **Avanzamento:** implementati timeout con frame saltato senza perdere la GPU,
  un solo tentativo di riconfigurazione per frame su Lost/Outdated e fallback
  dopo tre recuperi consecutivi falliti. Memoria esaurita/errori sconosciuti
  attivano subito il fallback; solo le presentazioni riuscite contano negli FPS EXT.
  I test simulano timeout prolungati, recupero, budget e fallimenti definitivi.
  **Completato quando:** resize, minimizza/ripristina, cambio schermo e un errore
  transitorio simulato recuperano senza blocchi. Verificare chiusura Wayland
  anche dopo recupero e fallback, preservando l'ordine di rilascio in `exiting`.

- [ ] **PERF-05 — Aggiornare texture UI e LUT solo quando cambiano.**
  Rischio basso/medio; dipende da PERF-02/03.
  In `ui_layer.rs`, `fps.rs` e `refresh_ui_layers` introdurre revisioni esplicite
  dei contenuti: LUT alla modifica dei colori, FPS alla modifica del testo,
  UI quando cambia davvero. Separare variazioni di layout da variazioni dei
  pixel. Condividere l'upload del mondo solo per livelli con stessa sorgente,
  dimensioni e generazione; il pannello statistiche ha uno sfondo proprio.
  **Completato quando:** contatori di byte/upload dimostrano la riduzione e
  immagini, HUD, colori, menu, zoom e caricamento/Undo restano equivalenti.
  Invalidare le revisioni al cambio ROM, caricamento stato e ricreazione GPU.
  Valutare staging riutilizzabile solo se il profilo lo giustifica: anche gli
  upload hanno costi di copia/allocazione
  ([documentazione wgpu](https://docs.rs/wgpu/26.0.1/wgpu/struct.Queue.html#method.write_texture)).

- [ ] **PERF-06 — Riutilizzare buffer e risultati intermedi misurati come costosi.**
  Rischio basso/medio; dipende da PERF-02/03.
  Controllare `handoff_audio`: il consumo con `mem::take` di `fifo_tap`
  abbandona il buffer dopo il trasferimento. Valutare un drenaggio che conservi
  la capacità. Misurare anche packing/bounds ripetuti dei pannelli e conservarli
  per revisione del contenuto. Evitare cache basate solo sull'indirizzo o
  ottimizzazioni di allocazioni presenti esclusivamente nei test.
  **Esperimento valutato:** la scansione dei soli estremi visibili per riga
  riduce il costo della funzione sui pannelli sintetici, ma due confronti
  end-to-end da 3.000 frame hanno mostrato una regressione del candidato.
  Non attivata nel runtime: conservata solo nei test con equivalenza e
  microbenchmark release a cinque coppie alternate. Restano da valutare
  riuso/packing su scene con pannelli attivi e beneficio complessivo dimostrato.
  **Completato quando:** diminuiscono allocazioni o tempo nel percorso misurato,
  senza crescita continua della memoria, eventi audio persi o pannelli obsoleti.

## P2 — Ottimizzare il collo di bottiglia dimostrato

- [ ] **PERF-07 — Ridurre i fallback costosi durante i turni nemici.**
  Rischio medio/alto; dipende da PERF-01/02/03.
  Profilare `run_frame_native`, `BlockTable`, interprete e codice generato in
  [recomp-core](../../crates/recomp-core/src). Il lookup ROM/IWRAM è già diretto:
  non sostituirlo senza evidenza. Usare il censimento dei fallback per trovare
  ingressi mancanti, distinguendoli dagli hook interpretati intenzionalmente.
  Estendere la traduzione solo per codice la cui validità è garantita; RAM
  modificabile e overlay richiedono controlli e invalidazione.
  **Completato quando:** il replay AI impiega meno tempo host oltre il rumore
  delle misure e conserva decisioni, RNG, cicli, interrupt e stato finale.
  Aggiornare revisione cache/ABI quando cambia il contratto del codice generato.

- [ ] **PERF-08 — Ridurre il costo della PPU CPU senza cambiarne il timing.**
  Rischio medio; dipende da PERF-02/03.
  Profilare `render_scanline`, tile, sprite e `compose_into`, separando i
  240×160 pixel originali dall'estensione widescreen/zoom. Riutilizzare dati
  invarianti dentro la stessa scanline e limitare l'elaborazione alle aree
  necessarie. Valutare cache di tile decodificati solo con invalidazione
  completa per scritture VRAM/palette, DMA, caricamento e ripristino.
  **Completato quando:** immagini raw e stato guest sono identici anche con
  finestre, blending, sprite semitrasparenti ed effetti a scanline.
  Gli effetti fuori schermo che influenzano lo stato GBA devono restare attivi.

- [ ] **PERF-09 — Ridurre contesa e lavoro bloccante nell'audio.**
  Rischio medio/alto; dipende da PERF-02/03.
  Misurare il mutex comune fra `build_stream` in
  [main.rs](../../crates/recomp/src/main.rs) e `handoff_audio`: il callback lo
  mantiene durante l'elaborazione dei campioni. Controllare anche il percorso
  fast-forward in [ff_audio.rs](../../crates/recomp/src/ff_audio.rs).
  Prima ridurre sezioni critiche e allocazioni; considerare ring SPSC limitati
  solo se la contesa rimane dominante. Mantenere ordine degli eventi, capacità,
  reset, crossfade e recupero del device.
  **Completato quando:** migliorano picchi/underrun senza aumentare la latenza,
  con prove di pausa, cambio modalità, fast-forward, load/Undo e chiusura.

- [ ] **PERF-10 — Correggere il pacing solo dopo aver distinto lavoro e attese.**
  Rischio medio; dipende da PERF-02/03.
  Confrontare modalità `FFTA_FRAME_PACING=clock`/`audio`, VSync e tempi di
  acquisizione superficie. Verificare l'interazione fra sleep e blocchi della
  presentazione prima di cambiare la schedulazione. Valutare un'attesa gestita
  dall'event loop invece di sleep nel callback, mantenendo la gestione input.
  Misurare separatamente eventuali pause causate dall'autosave ogni 300 frame:
  spostarlo su un worker richiede ordine delle scritture e flush affidabile.
  **Completato quando:** migliora il p99 della cadenza su monitor a 60 e alta
  frequenza, senza deriva audio, rallentamento guest o input meno reattivo.
  Non legare il clock GBA alla frequenza del monitor e non saltare simulazione.

## P3 — GPU e parallelismo incrementali

- [ ] **PERF-11 — Prototipare LUT e filtri temporali sulla GPU.**
  Rischio medio/alto; dipende da PERF-02/03/05. Procedere solo se questi passaggi
  occupano una parte significativa del budget CPU.
  Conservare core e framebuffer BGR555 sulla CPU; caricare il frame raw e usare
  LUT/storia in texture. Partire da Off/Simple, poi Smart e infine Persistence.
  Preferire un pass grafico semplice; un compute shader non è obbligatorio.
  Preservare arrotondamenti e fusione nello spazio dei colori già codificati.
  **Completato quando:** costo totale CPU+trasferimenti+GPU e p99 migliorano,
  con confronti CPU/GPU e fallback CPU. La storia avanza per ogni frame guest,
  anche se una presentazione viene saltata o il fast-forward produce più frame.
  Pausa, caricamento, Undo, zoom e cambio LUT devono mantenere il comportamento
  attuale. Niente readback sincrono per frame; anteprime e salvataggi devono
  funzionare senza introdurre attese GPU nel normale ciclo del gioco.

- [ ] **PERF-12 — Valutare sul desktop il parallelismo già presente per lo zoom.**
  Rischio medio; dipende da PERF-02/03/08.
  Provare `render_zoom_parallel` solo sulle righe aggiuntive, a stato guest
  congelato nel VBlank. Usare un pool persistente, una soglia misurata e il
  percorso seriale per lavori piccoli. Non aggiungere thread per ogni frame.
  **Completato quando:** equivalenza raw anche per pannelli e blending, nessuna
  mutazione concorrente del guest e vantaggio su CPU con pochi e molti core,
  senza peggiorare audio o p99. Il test `parallel_rows_match_serial...` è una
  base, non una prova di prestazioni desktop.

## P4 — Esperimenti da rimandare

- [ ] **PERF-13 — PPU estesa sulla GPU, solo con evidenza residua forte.**
  Rischio alto; dipende dalle misure dopo PERF-08/11/12.
  Valutare prima una porzione isolata delle estensioni host a snapshot stabile.
  Una PPU GPU completa deve rappresentare correttamente cambi di registri,
  VRAM, palette, finestre e DMA durante il frame: una fotografia a fine frame
  non basta in generale. Servono confronto col riferimento CPU e percorso
  sperimentale disattivato per default.
  **Completato quando:** vantaggio sostanziale e ripetibile anche con costi di
  sincronizzazione, equivalenza e recupero su backend differenti. Scartare
  l'esperimento se la complessità supera il beneficio.

Non metterei l'IA guest sulla GPU nel piano iniziale: il percorso attuale
esegue istruzioni ARM/Thumb con dipendenze da memoria, RNG e interrupt.
Trasformarlo in kernel o sostituire l'IA con una riscrittura host richiederebbe
una nuova prova di equivalenza. Anche parallelizzare arbitrariamente CPU,
PPU e audio, usare fast-math o eliminare controlli di sicurezza per aumentare
gli FPS introduce rischi sproporzionati rispetto alle opzioni precedenti.

## Regole per accettare ogni intervento

Una modifica alla volta, con diff piccolo e una responsabilità chiara. Estrarre
moduli solo quando aiuta l'intervento corrente; evitare refactoring massivi di
`play.rs`/PPU insieme alla modifica algoritmica. Riutilizzare `wgpu`, strumenti
e test esistenti; nuove dipendenze o `unsafe` richiedono una necessità concreta.

- Correttezza: replay e controlli PERF-03 passano; test core seriali per i
  globali di display condivisi. Per filtri GPU, specificare prima eventuali
  tolleranze motivate; nessuna tolleranza sullo stato guest o sui save.
- Prestazioni: beneficio ripetibile oltre la variabilità fra run nella scena
  bersaglio, senza regressioni significative nelle altre. Conservare anche
  misure sfavorevoli; scartare modifiche senza beneficio utile.
- Compatibilità: Ubuntu Wayland/RADV e X11, fallback software, Windows nativo
  quando disponibile; Wine è una verifica distinta. Backend alternativi e
  hardware non provati devono restare esplicitamente non verificati.
- Persistenza e lifecycle: caricamento/Undo, backup/ripristino, cambio ROM,
  spostamento portable, resize e chiusura pulita secondo l'area toccata.
  Cache e risorse host devono essere ricostruibili e invalidabili. Cambiare
  versione dei savestate solo se cambia il loro contratto, non per un filtro.
- Memoria di progetto: registrare risultati reali, limitazioni e decisione
  tenere/scartare in `docs/changes`, poi rigenerare il changelog. Un TODO resta
  aperto fino a implementazione e verifica; AndroidPort richiede lavoro separato.

**Prima tranche consigliata:** PERF-01/02/03, quindi PERF-04/05/06.
Le priorità successive vanno riordinate con i dati: PERF-07 se domina
l'esecuzione dei turni, PERF-08/12 se domina lo zoom, PERF-09 se domina
l'audio, PERF-10 se dominano le attese, PERF-11 se dominano LUT e filtri.
