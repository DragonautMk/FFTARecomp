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

Per scegliere il prossimo lavoro leggere prima [TASK_HANDOFF.md](TASK_HANDOFF.md).
Le caselle PERF rappresentano criteri complessivi: **non riaprire i sottopassi
conclusi** perché il task padre è ancora aperto. Le sezioni delle prime prove
qui sotto sono storiche; il registro distingue risultati superati, esperimenti
scartati e lavoro residuo. Il replay con attivazione nemica e la sua baseline
sono già conclusi, così come l'attribuzione alle fasi interne della PPU. Il
precalcolo delle finestre per riga è stato mantenuto dopo i confronti. Il
riuso dei BG nativi e il percorso rapido per righe UI vuote sono stati
mantenuti e le build desktop correnti sono validate su Linux e Windows/Wine.
Il prossimo sottopasso è caratterizzare lo zoom elevato sulle fixture esistenti,
prima di valutare parallelismo o altro lavoro GPU.

Prima tranche P0 in corso: confronto Linux nativo/Windows con verifica separata,
hash finale opzionale dello stato macchina, diagnostica bounded dei tempi per
frame e profilo CPU della finestra, con upload/acquisizione/encoding/invio
e presentazione separati. Il [workflow](PERFORMANCE_WORKFLOW.md)
descrive comandi, interpretazione e limiti. PERF-01/02/03 restano aperti: mancano
un replay identificato del turno nemico lento, misure PPU/GPU distinte e la
copertura completa dei criteri. Nessun guadagno sui turni AI è ancora dimostrato.

Le ridondanze indipendenti già affrontate sono conservate: PERF-05 ha una prima
implementazione di riuso delle LUT e della
texture FPS; PERF-06 conserva i buffer FIFO audio e risolve la modalità di pacing
all'avvio. Restano aperti i criteri più ampi, i confronti di prestazioni e le
ulteriori cache di pannelli/mondi; nessuna modifica accelera artificialmente
clock o logica del gioco.


Passi già implementati e verificati, senza chiudere prematuramente le attività:

- [x] Confronto nativo con verifica immagini e stato finale opzionale.
- [x] Profilo CPU per frame e per fasi della presentazione, disattivato di default.
- [x] Cache LUT per contenuto e revisioni del contatore FPS, con parità CPU/GPU.
- [x] Riuso dei buffer FIFO audio e modalità di pacing risolta all'avvio.

## Fixture locali e prima verifica della battaglia

Il nuovo archivio locale `saves.zip` e la sua eventuale estrazione `saves/`
sono ignorati da Git; `.sav` e `.ffstate` sono già esclusi anche altrove.
Le prove usano copie in `target/`: nessun progresso personale entra nei
record di progetto, nel contesto Docker o nei pacchetti.

Il caricamento effettivo dello slot fornito è riuscito con il runtime Linux
esportato dal container. Il salvataggio cartuccia permette inoltre di aprire
la world map dalla voce Saved Game > Load. Entrambe le scene sono state
riconosciute visivamente, senza dedurle dal solo nome del file.

Una sessione esplorativa della battaglia, ripresa esplicitamente dal menu
host e lasciata avanzare per 20 secondi, ha prodotto 1.074 campioni core,
171.840 scanline e nessun intervallo PPU invalido. Il profilo include anche
l'avvio: mediana core 23,084 ms e scanline 17,545 ms; p95 rispettivamente
25,520 e 19,434 ms. Lo zoom non ha eseguito righe aggiuntive.
Questa è una **misura preliminare Xvfb/llvmpipe**, non una baseline Radeon né
un replay identificato del turno nemico. I tentativi rimasti nel menu host
sono esclusi da questa valutazione.

La PPU CPU è quindi una priorità da approfondire in PERF-02/08: separare
rendering BG/OBJ, composizione ed estrazione dei pannelli; verificare in
particolare i passaggi ripetuti per riga in `capture_dialogue_part`.
Quella funzione ordina i candidati BG/OBJ per ogni pixel, pur avendo priorità
BG costanti per riga: misurare separatamente questo lavoro e confrontare una
selezione dei primi due livelli con il riferimento prima di cambiarlo.
Non è ancora dimostrato quale funzione domini né un beneficio dello
spostamento sulla GPU. Il runner ora supporta `--state` anche nel confronto
Python: 600 frame aggiuntivi, 120 di warm-up e cinque coppie native sulla stessa
build hanno prodotto framebuffer/OAM, stato finale e conteggi identici.
Le mediane di lavoro sono 20,950/20,856 ms; le due catture diagnostiche attribuiscono
circa il 76% del core alle scanline. Il caricamento e l'avvio sono esclusi dal
campione. Un controllo separato verifica anche lo zoom relativo allo snapshot.
Questa è una baseline della battaglia inattiva, senza ancora un turno nemico
identificato: non è un guadagno di prestazioni e PERF-01/02/03 restano aperti.

## Correzione delle condizioni di misura e selettore UI valutato

Una ricerca automatica npm di VS Code nella workspace seguiva anche gli alberi
ignorati ed era attiva da oltre mezz'ora, occupando circa dieci core. È stata
interrotta dopo averne identificato l'origine automatica; `npm.autoDetect` è
stato disattivato soltanto nelle impostazioni locali ignorate della workspace.
Nessuna impostazione globale o preferenza preesistente è stata cambiata.
Il portable con renderer originale ha poi superato caricamento, Annulla,
ricaricamento e chiusura in Xvfb/llvmpipe: nella battaglia inattiva il contatore
osservato è circa 59–60 FPS INT/EXT. Non è una misura Radeon né del turno AI.
Il [workflow](PERFORMANCE_WORKFLOW.md#host-load-before-timing) descrive il controllo.

Con **lo stesso intervallo da 1.200 frame**, snapshot, binario baseline,
traduzione, input, audio e affinità CPU, la mediana baseline è 18,655 ms con
quel carico e 10,777 ms dopo la sua rimozione. Le precedenti prove da 600 frame
intorno ai 21 ms restano dati storici sotto contesa, non una baseline senza
interferenze; non confrontare direttamente lunghezze diverse. Questo è un
cambiamento dell'ambiente di sviluppo, non un'accelerazione del codice di gioco.

Sono stati valutati due selettori dei soli due livelli UI frontali al posto
dei due ordinamenti per pixel. Entrambi conservano risultati esatti su 196.608
combinazioni di priorità/presenza e sui replay. Sul sistema senza la scansione:
la variante a chiave compatta peggiora la mediana dell'1,64%; quella semplice
migliora la mediana dell'1,56%, ma la cattura diagnostica non mostra un vantaggio
sui picchi (p95/p99 maggiori). Il beneficio complessivo non giustifica sostituire
l'ordinamento standard: **nessuna variante è stata mantenuta nel runtime**.
I singoli profili dei picchi non provano una regressione causale universale;
sono comunque insufficienti per accettare un'ottimizzazione del pacing.

La world map è stata raggiunta di nuovo partendo da una copia del `.sav`,
registrando gli input e creando uno snapshot locale classificato visivamente.
Il replay da quello snapshot passa cinque coppie da 1.200 frame sul renderer
originale: mediana circa 2,1 ms/frame, stato finale e immagini identici,
nessun frame oltre budget nei due profili. È una calibrazione sullo stesso
binario, non un miglioramento. I file restano sotto `target/`, fuori da Git
e dai pacchetti. Prima di altri
interventi PPU/GPU va identificato il turno nemico lento sul sistema senza
questa interferenza; PERF-01/02/03/08 restano aperti.

## Replay con attivazione nemica identificata

È stata preparata una nuova sequenza locale dalla copia dello snapshot fornito:
salvataggio sospeso tramite il comando del gioco, nuovo avvio con registrazione,
ripresa della battaglia e attesa delle unità del giocatore. La sequenza comprende
l'attivazione e il movimento di un mostro nemico, poi il ritorno al menu di
un personaggio del giocatore. Questi passaggi sono stati osservati nella finestra;
non si tratta più del solo replay della battaglia inattiva.

Uno snapshot intermedio e gli input successivi ribasati delimitano 3.530 frame.
Il replay headless raggiunge lo stesso frame finale e lo stesso hash del
framebuffer della sessione grafica; anche l'immagine finale è stata ispezionata.
Le due verifiche A/B coincidono su framebuffer e OAM di tutti i frame.
Il [workflow](PERFORMANCE_WORKFLOW.md#record-a-battle-from-a-supplied-snapshot)
descrive la procedura senza rimuovere la protezione che vieta di caricare uno
snapshot durante la registrazione. Tutte le fixture rimangono ignorate.

Il confronto completo passa cinque coppie alternate sullo stesso codice, con
120 frame di warm-up: mediane 8,346/8,290 ms per i 3.410 frame misurati,
stato macchina finale e conteggi di esecuzione identici. I due profili separati
hanno zero frame oltre 16,74 ms, p99 10,055/10,129 ms e massimi
12,535/16,518 ms. Le scanline occupano circa il 73% del core (6,07/6,08 ms
medi); il resto circa 2,25/2,24 ms, senza righe zoom aggiuntive. È una nuova
baseline, non uno speedup: la differenza fra binari identici calibra il rumore.
La prima esecuzione interrotta con errore di output resta conservata come
parziale; questi risultati provengono dal successivo confronto completato.

Questo intervallo include input del giocatore, animazioni e attese: non misura
solo il calcolo AI e non dimostra che sia il turno lento originariamente
segnalato. PERF-01 resta aperto per gli altri scenari e la caratterizzazione del
rallentamento; nessuna accelerazione del runtime è ancora attribuita al replay.

## Attribuzione delle fasi PPU completata

`FFTA_PPU_STAGES=1` aggiunge intervalli disgiunti per BG, OBJ, composizione,
estrazione UI, setup UI e residuo. Il confronto del nuovo codice diagnostico
con la baseline passa cinque coppie sul replay esistente, immagini/OAM per
frame, stato macchina e conteggi. Mediane con raccolta spenta 8,289/8,317 ms
(differenza +0,34% in questa prova). La cattura dettagliata usa 8,373 ms/frame,
circa +0,67% rispetto alla mediana dello stesso candidato: è una stima singola
soggetta al rumore, non una garanzia universale di overhead.

Medie per frame: BG 0,246 ms, OBJ 0,288 ms, composizione 0,531 ms,
UI capture 4,912 ms, setup UI 0,071 ms, residuo scanline 0,078 ms.
L'estrazione UI occupa circa l'80% delle scanline e il 59% del core.
Nessun intervallo invalido o frame oltre budget nei due profili; p99 core
baseline/candidato 10,237/10,116 ms. Questo chiude il sottopasso diagnostico
PERF-02-PPU-STAGES su Linux, non l'intero PERF-02 o una nuova ottimizzazione.
Il profilo ha guidato il precalcolo delle finestre per riga di
`capture_dialogue_part`, già concluso nella sezione seguente; i selettori frontali
scartati non sono stati ripetuti.

## Prima ottimizzazione dell'estrazione UI mantenuta

PERF-08-UI-ROW-INVARIANTS è concluso: registri finestra e appartenenza verticale,
origine nativa e flag modale sono calcolati una volta per chiamata. La precedenza
WIN0/WIN1/OBJ/esterno e gli intervalli circolari restano identici; 2.457.600 casi
confrontati con il riferimento precedente, 169 test core e 48 runtime passati.
Nessuna cache persistente, modifica del timing guest o versione stato nuova.

Cinque coppie sul replay esistente: mediana 8,412 → 8,229 ms/frame, circa
**2,18% di lavoro host in meno**, immagini/OAM per frame, stato finale e conteggi
identici. Il profilo separato attribuisce UI capture 4,941 → 4,779 ms medi.
p95/p99 core migliorano (10,117/10,540 → 9,890/10,346 ms); i rari frame oltre
budget sono 2/4 nelle singole catture, con massimi 18,489/18,634 ms. Non è una
prova di eliminazione dei picchi, né del lag originariamente segnalato.

Il nuovo codice passa anche cinque coppie sulla world map: mediane
1,958/1,967 ms (+0,43%), stato/immagini/conteggi identici e zero frame oltre
budget nei profili. Una sessione Xvfb/llvmpipe con il nuovo runtime passa
load, Annulla, reload, ripresa e chiusura; anteprime e messaggi ispezionati,
profilazione dettagliata senza intervalli invalidi, slot e ZIP originali invariati.
Windows nativo, Radeon e controller fisico restano fuori da questa validazione.
Non ripetere questi task senza una nuova modifica o un criterio scoperto.

## Riuso dei BG nativi UI mantenuto

PERF-08-UI-PREPARED-BG-ROWS è concluso: BG2/BG3 nativi sono preparati una volta
per scanline e condivisi fra pannelli/passaggi. Il backing filtra una copia
privata del testo; Stats conserva la sua maschera nulla. Il riferimento con
preparazione fresca per passaggio coincide nello stato macchina finale su 48
configurazioni e tutte le 160 righe, con piani non vuoti, filtri e BG abilitati
in combinazioni diverse. Passati 170 test core e 48 runtime.

Cinque coppie sul replay esistente contro la versione con finestre precalcolate:
**8,244 → 7,903 ms/frame, 4,14% di lavoro host in meno**. Immagini/OAM per
frame, stato finale e conteggi identici. UI capture media 4,781 → 4,476 ms;
p95/p99 core 9,895/10,328 → 9,487/9,909 ms. Nessun frame oltre budget nei due
profili, ma il massimo isolato cresce da 10,862 a 12,277 ms: nessuna promessa
universale di eliminazione dei picchi.

Cinque coppie world map: mediane 1,962/1,950 ms, differenza piccola, immagini,
stato e conteggi identici, zero frame oltre budget. Nuovo runtime verificato
anche su load/Annulla/reload/ripresa/chiusura in Xvfb/llvmpipe, con ispezione
delle anteprime e del gioco; 989 frame e zero intervalli PPU invalidi. Slot e
ZIP originali invariati. Nessun nuovo dato persistente, ABI o formato stato;
nessuna misura Windows nativa o Radeon è dedotta da questi risultati.

## Righe UI vuote: ordinamento e blending evitati

PERF-08-UI-EMPTY-ROWS è concluso. Dopo preparazione, cattura sprite e reset dei
flag, una riga con tutte le sorgenti UI vuote viene azzerata esplicitamente e
non ordina/fonde i candidati del mondo: questi non possono diventare pixel UI.
Non vengono saltati frame, scanline hardware, cicli o decisioni guest. Il
vecchio ciclo completo resta il riferimento di test; le righe non vuote lo
mantengono invariato. Passati 171 test core e 48 runtime, inclusi contenuti
precedenti da cancellare, maschere Stats, modali e blending di oggetti mondo.

Cinque coppie sul replay esistente contro la versione con BG condivisi:
**7,891 → 6,194 ms/frame, 21,50% di lavoro host in meno**. Immagini/OAM per
frame, stato finale e conteggi identici. UI capture media 4,467 → 2,770 ms;
p95/p99 core 9,481/9,906 → 7,497/7,793 ms, zero frame oltre budget nei due
profili. Il massimo isolato cresce 10,770 → 14,502 ms: non dedurre una garanzia
sui picchi rari o la risoluzione del lag originariamente segnalato.

Cinque coppie world map: 1,978/1,942 ms, uguaglianza immagini/stato/conteggi e
zero frame oltre budget. Il nuovo runtime passa load/Annulla/reload/ripresa/
chiusura in Xvfb/llvmpipe, con ispezione di anteprime e gioco; 987 frame e zero
intervalli PPU invalidi. Slot e archivio originale restano invariati. Il passo
di validazione delle build correnti Windows/Linux è stato completato nella
sezione seguente; gli archivi Docker aggiornati incorporano il renderer corrente.

## Build e archivi desktop aggiornati

PERF-03-CURRENT-DESKTOP-BUILDS è concluso per il suo ambito: Windows GNU/MinGW
in Docker con **317 test Wine**, Linux Bookworm con **318 test**; 9 casi opzionali
ignorati per piattaforma. Integrità, percorsi e contenuti degli archivi verificati:
solo lo scaffold Data vuoto, nessuna ROM/save/registrazione/cache gioco o DLL di
test, TinyCC e licenze inclusi. PE con risorse e import invariati; ELF eseguibili
con requisito massimo GLIBC 2.35. Hash degli input registrati invariati.

Il nuovo runtime Windows esportato passa il replay esistente da 3.530 frame:
framebuffer/OAM per frame, stato finale e conteggi coincidono sia col precedente
runtime Windows sia con la build Linux già validata. È una prova di correttezza
sotto Wine, non un confronto di prestazioni Windows nativo. Il nuovo archivio
Linux passa avvio a cache vuota, load/Annulla/reload/ripresa/chiusura in
Xvfb/llvmpipe: anteprime e gioco ispezionati, 1.001 frame e zero intervalli PPU
invalidi; slot e ZIP originali invariati.

Gli archivi e BUILD-INFO correnti sono aggiornati in `dist/docker-windows/` e
`dist/docker-linux/`; i precedenti restano conservati nella directory locale di
validazione. Nessuna pubblicazione remota. Restano aperti Windows nativo,
controller fisici, GPU hardware/pacing e gli scenari non ancora caratterizzati,
fra cui zoom elevato: non ripetere la baseline ordinaria per coprirli.

## Osservazioni sulla revisione iniziale (storiche)

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
