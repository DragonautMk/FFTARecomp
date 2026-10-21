# TODO — Bug fix e funzionalità desktop

Analisi e avanzamento del 21 ottobre 2026, basati sul codice e sulla cattura
fornita dall'utente (immagine rimossa dalla storia Git dopo la diagnosi).
Le caselle distinguono il lavoro verificato dalle attività ancora aperte;
le proposte delle sezioni successive restano da sviluppare.

Ambito: workspace desktop, con priorità di verifica su Windows.
`AndroidPort/` è un albero separato. Per riprendere lo sviluppo consultare
[TASK_HANDOFF.md](TASK_HANDOFF.md): riutilizzare le verifiche già concluse nel
[PERFORMANCE_TODO.md](PERFORMANCE_TODO.md), senza riaprire esperimenti scartati.

**BUG-01-LAWS** ha una correzione verificata nei limiti descritti sotto.
Ordine successivo proposto: diagnosi di **BUG-03-FAST-AUDIO** e
**BUG-02-FPS** (P1); **FEAT-01-MUTE** e **FEAT-02-FULLSCREEN** (P2);
**FEAT-03-TACTICAL-OVERLAY** (P3, prima dati e visualizzazione, poi previsione).
Le priorità indicano l'ordine di lavoro suggerito, non una gravità già misurata.

## BUG-01-LAWS — Schermata delle leggi in battaglia

**Stato:** correzione implementata e verificata per il layout osservato;
resta aperta la copertura estesa sotto. **Priorità:** P1. **Complessità:** media.

**Causa verificata.** La tabella leggi usa BG2=5C08/BG3=4A09, come il menu
azioni, ma non attiva WIN0 per l'aiuto. Il vecchio riconoscimento `Notice`
non la ammetteva. Senza HUD a bordo schermo la separazione era saltata;
con HUD poteva scattare il layout azioni e il ripiego a X=160, producendo
il contenuto diviso e il pannello destro vuoto della segnalazione.

**Soluzione implementata.** `laws_bounds` in
[ppu.rs](../../crates/gba-core/src/ppu.rs) riconosce cornice, righe e divisore
della tabella, senza leggere le parole tradotte. Riutilizza il livello
centrale `Notice` per tabella e aiuto, comprensivo di selettore e indicatore
di continuazione; l'HUD restante rimane separato. Il sottotipo host
`laws_layout` in [ffta_dialogue.rs](../../crates/gba-core/src/ffta_dialogue.rs)
è ricalcolato a ogni frame e non viene serializzato. La geometria completa
non subisce Length o il clamp dei pannelli laterali. Restano i controlli
esistenti di ROM, terreno e registri; firme incomplete conservano il vecchio
percorso. Vedi [documentazione e verifiche](../ffta_laws_layout.md).

- [x] Riprodurre lista e aiuto con ROM EUR e salvataggi locali, in italiano
  e inglese; osservare BG, finestre e sprite. Conservare dati e catture sotto
  `target/laws-fix/`, ignorato da Git.
- [x] Correggere classificazione, proprietà dei pixel e dei due indicatori,
  riutilizzando layout e modalità mouse dei pannelli modali.
- [x] Verificare testo su entrambi i lati del vecchio taglio, HUD presente o
  assente, tre larghezze, ricomposizione nativa esatta e VRAM/OAM/framebuffer
  invariati; la regressione fallisce disattivando il riconoscimento.
- [x] Verificare 72 casi da snapshot: lista/aiuto, 3:2/16:9/ultrawide, zoom
  1 e 2, Size 0,5/1/2 e Length 1/2. Ricomposizione nativa: zero pixel diversi.
  Parità GPU/software: zero differenze in 24 casi sintetici, incluso resize
  fra dimensioni pari/dispari e variazioni di Size.
- [x] Eseguire suite core e runtime native, suite core Windows/Wine e
  regressioni finali sui byte degli snapshot; ispezionare lista/aiuto nel
  frontend Linux e Windows/Wine e verificare caricamento e uscita normale.
- [ ] Completare la copertura aggiuntiva: Windows nativo, ROM USA, altre
  disposizioni/quantità di leggi, scena tutorial esatta della cattura,
  zoom oltre 2 e un ciclo load/Undo specifico nella schermata leggi.

**Criterio di chiusura:** il layout verificato mantiene lista, testo e
indicatori integri e coerenti con il framebuffer originale. Le prove
aggiuntive elencate restano esplicite e non riaprono la correzione già
validata. Non sono stati aggiornati i pacchetti portable.

## BUG-02-FPS — Differenza tra framerate interno ed esterno

**Stato:** comportamento da classificare, non ancora bug confermato.
**Priorità:** P1 per la diagnosi. **Complessità:** bassa per l'analisi;
eventuali modifiche al pacing richiedono misure dedicate.

**Evidenza e analisi.** [fps.rs](../../crates/recomp/src/fps.rs) conta i frame
guest completati (**INT**) e le presentazioni host riuscite (**EXT**) sul tempo
reale. EXT non misura il refresh fisico del monitor. Il ciclo in `play.rs`
può simulare più frame prima di presentarne uno: in fast-forward la differenza
è quindi prevista. A 4×, se il carico lo consente, circa 239 INT e 60 EXT
sono coerenti; non sono una misura ottenuta in questa analisi. Anche pacing
audio a blocchi e presentazioni saltate possono separare i due contatori.

La cattura allegata mostra **59,7 INT / 59,7 EXT**: documenta il difetto grafico,
non la divergenza segnalata. Esistono già test del contatore,
[documentazione FPS](../fps_overlay.md) e diagnostica di cadenza completata in
`PERF-10-PRESENTATION-CADENCE`. Il pacing `clock` è già predefinito su
Linux/Windows; non va proposto come funzionalità mancante.

**Soluzione proposta.** Rendere comprensibili le due misure e correggere solo
un'eventuale differenza inattesa a velocità normale, dopo averne individuato
la causa. Non forzare INT ed EXT a coincidere e non legare il clock GBA al monitor.

- [ ] Raccogliere il caso preciso: valori, durata, scena, Fast on/off e
  moltiplicatore, VSync, refresh, backend GPU/audio, finestra visibile o ridotta.
- [ ] Riutilizzare `FFTA_PROFILE=1` e l'eventuale finestra
  `FFTA_PRESENT_PROFILE_FRAMES=START:END` per distinguere frame multipli,
  attese, carico e presentazioni saltate; usare i risultati esistenti come
  riferimento, senza ripetere l'intera matrice PERF-10.
- [ ] Aggiungere una spiegazione accessibile da Video > FPS counter:
  «INT: simulazione; EXT: presentazioni; in Fast possono differire».
- [ ] Se il caso a 1× è anomalo, collegare la correzione al criterio ancora
  aperto di PERF-02/04/10 e aggiungere una regressione mirata. Se è previsto,
  chiudere la segnalazione con spiegazione e prova del caso osservato.
- [ ] Verificare contatori dopo pausa/ripresa, Fast e presentazioni saltate;
  confrontare la cadenza con/senza VSync sul dispositivo interessato.

**Criterio di chiusura:** ogni divergenza riprodotta ha una spiegazione
misurabile; quelle patologiche sono corrette senza rallentare la simulazione
o alterare la definizione dei contatori. L'uguaglianza numerica non è un requisito.

## BUG-03-FAST-AUDIO — Audio fuori sincrono in modalità Fast

**Stato:** caso segnalato precisato (Fast audio, 4×); reset delle transizioni e
consumo sulla produzione effettiva implementati. Copertura hardware/eventi e
limite del backlog Normal restano aperti.
**Priorità:** P1. **Complessità:** medio-alta. Collegato a PERF-09/10.

**Evidenza e analisi.** `AvConfig` e il menu Fast forward espongono già
`silent`, `fast` e `normal`. `set_fast_forward` svuotava già le code al cambio
modalità, ma lasciava attive le memorie dei filtri e l'ultimo campione nel
callback. La regressione sintetica conferma l'uscita residua a code vuote.
Il nuovo epoch host invalida anche questi stati; pausa e perdita del device
scartano inoltre il backlog Normal. Vedi [comportamento e prove](../fast_forward_audio.md).
`fast` conserva il moltiplicatore selezionato come limite richiesto. Il callback
in [main.rs](../../crates/recomp/src/main.rs) ora ne adatta il consumo alla
produzione misurata dal clock del device e alla riserva in coda; il frontend
continua a eseguire il gruppo di frame indipendentemente dall'audio.

`normal` usa invece [NormalAudio](../../crates/recomp/src/ff_audio.rs): riproduce
a 1× il PCM prodotto dal gioco accelerato, conservandolo in un file temporaneo.
Se il guest avanza più rapidamente del consumo, il ritardo cresce per
costruzione. La RAM è limitata, ma il file non ha un limite esplicito di durata
o dimensione. Questo è distinto da un possibile difetto di `fast`: se il PC
non sostiene il moltiplicatore richiesto, consumo nominale e produzione reale
possono divergere. Il caso dell'utente riguarda Fast audio a 4×, non Normal.
Nella prova locale Linux/CPAL/llvmpipe, Faithful ed Enhanced raggiungono solo
circa 122–128 INT: tutti i 497/499 callback Fast hanno holds, con zero drop
del produttore e circa 47% delle richieste PSG/HLE vuote. Il consumo nominale
supera quindi la produzione; a 1× gli intervalli playing non hanno holds.
Hold/release, toggle e pausa/ripresa sono stati eseguiti con catture ispezionate.
Volume zero e scena inattiva non provano la qualità sonora né il ritardo di un
evento sul dispositivo dell'utente. Il solo reset non chiudeva questa seconda
causa: è stata quindi aggiunta la correzione del consumo descritta sotto.

**Soluzione implementata per Fast.** `fast_audio_rate::Clock` stima la produzione
su circa200ms, corregge la riserva con risposta250ms e varia un consumo comune
ai cinque percorsi su circa100ms. Riserva obiettivo60ms oltre al buffer del
callback, intervallo0,05×..moltiplicatore+2%; a 1× resta il vecchio controllo.
Il filtro sinc usa cutoff frazionari continui; priming/riempimenti emettono
silenzio esplicito, contato da `FFTA_FAST_AUDIO`, senza ripetere campioni vecchi.
Nessun nuovo lock, I/O, allocazione o clock host nel callback.

**Proposta ancora aperta per Normal.** Rendere
esplicito «audio 1× ritardato» e definire un limite al backlog. Un'eventuale
modalità accelerata che preservi il pitch richiederebbe time-stretch dedicato:
è una scelta ulteriore, non una proprietà della coda 1×.

- [x] Identificare modalità e velocità della segnalazione: l'utente conferma
  **Fast audio a 4×**, sospettando riproduzione di audio arretrato.
- [ ] Riprodurre a 2×/4×/8× con Faithful/Enhanced, transizioni hold/toggle e un
  evento audio-visivo riconoscibile. Misurare ritardo iniziale e sua evoluzione.
- [ ] Riutilizzare i profili audio/cadenza esistenti: correlare frame guest,
  consumo, riempimento, holds/drops e tempo dei callback; tenere distinti
  priming, pausa e Fast. Non usare un benchmark senza callback come prova sonora.
- [x] **BUG-03-EFFECTIVE-RATE:** correggere il consumo nominale quando la
  velocità richiesta non è raggiunta: clock comune smussato, riserva, cutoff
  frazionario e priming osservabile. Regressione callback4×/produzione2×,
  matrice72casi a2×/4×/8×, cambi di produzione/arresto/ripresa e filtraggio.
  Nessuna modifica al tempo guest o alla serializzazione.
- [x] Verificare il caso4× con callback Linux reali e finestra guest9400:11400:
  Faithful357/Enhanced359callback, zero holds/PSG-HLE empty/drops/riempimenti
  e zero silenzio di priming. Consumo medio2,2032/2,1906× segue produzione
 2,2030/2,1906×; coda stimata massima107,3/108,0ms. Volume zero e scena
  inattiva: non è una misura di latenza sonora su Windows nativo.
- [ ] Stabilire per `normal` un budget in secondi/byte e il comportamento al
  limite, per esempio salto controllato verso l'audio recente con breve fade;
  aggiornare nome/descrizione e rimuovere il backlog quando non serve.
  Spostare eventuale I/O bloccante fuori dal callback se quel percorso cambia.
- [x] **BUG-03-TRANSITION-RESET:** invalidare anche DAC, interpolatori, filtri e
  crossfade all'ingresso/uscita Fast e al cambio moltiplicatore/modalità;
  svuotare l'audio in pausa e alla perdita del device. Load/Undo conserva la
  ricreazione dello stream. Regressioni su uscita residua e nuovi campioni
  confrontati con un callback appena inizializzato, native e Windows/Wine.
- [ ] Completare ascolto e misure sul dispositivo: hold/toggle, pausa, load/Undo
  e riconnessione fisica; il reset host non ritira i campioni già nel backend.
- [ ] Testare sinteticamente consumo, limite del backlog e transizioni; poi
  verificare su Windows con audio reale. Fissare prima del confronto una soglia
  di latenza coerente con il buffer del device e controllare che non cresca
  durante una sessione prolungata; verificare stabilità di memoria e file.

**Criterio di chiusura:** `fast` non accumula deriva rispetto al video nel caso
riprodotto, e il ritorno a 1× non trascina audio arretrato. `normal` ha ritardo
dichiarato e limitato; `silent` resta silenzioso. Nessun cambio al mix guest,
alla serializzazione o ai timer è giustificato dalla sola segnalazione.

## FEAT-01-MUTE — Comando Mute nel menu Audio

**Stato:** da implementare. **Priorità:** P2. **Complessità:** bassa.
Coordinare il comportamento con BUG-03-FAST-AUDIO.

**Evidenza e analisi.** Il menu Audio contiene Volume e modalità audio.
`audio_volume = 0` silenzia l'uscita, ma manca un interruttore che conservi il
volume scelto. `AudioStreams::muted` è già usato da Fast silent e dagli errori
del backlog; riutilizzarlo come unico stato utente perderebbe il mute durante
le transizioni. Il callback dispone già di una rampa del volume di circa 20 ms.

**Soluzione proposta.** Aggiungere `Audio > Mute: On/Off` e un booleano
persistente `audio.mute`, predefinito Off. Separare il mute utente dalle cause
temporanee; applicare l'unione delle cause all'uscita finale, conservando
`audio_volume` e continuando a consumare i campioni.

- [ ] Estendere [AvConfig](../../crates/input-config/src/lib.rs): default,
  parsing, serializzazione e compatibilità dei vecchi `av.cfg`; usare la
  risoluzione esistente dei percorsi installed/portable.
- [ ] Aggiungere riga, valore e gestione mouse/tastiera/gamepad in
  [menu](../../crates/menu/src/lib.rs); applicare il cambiamento dal frontend.
- [ ] Separare stato persistente e mute temporaneo in `AudioStreams` e
  `apply_av_change`; utilizzare la rampa esistente senza fermare stream,
  consumo o pacing. Disattivare Mute non deve annullare Fast silent o un errore.
- [ ] Testare persistenza, mute/unmute al volume precedente, cambio volume
  mentre Mute è attivo, Faithful/Enhanced, Fast e assenza di backlog al ritorno.

**Criterio di chiusura:** mute accessibile dal menu e persistente; nessun suono
dopo la rampa, nessun salto di volume al ripristino, contatori e velocità di
gioco invariati. Volume 0 resta distinto da Mute On.

## FEAT-02-FULLSCREEN — Fullscreen On/Off nel menu Video

**Stato:** scorciatoia già presente; integrazione nel menu da implementare.
**Priorità:** P2. **Complessità:** bassa-media.

**Evidenza e analisi.** `play.rs` gestisce già F11 con
`Fullscreen::Borderless(None)` e ritorno alla finestra. `VIDEO_ROWS` e
`AvConfig` non espongono Fullscreen. Resize, DPI, cursore e recupero della
superficie hanno percorsi esistenti da riutilizzare; fullscreen esclusivo
non è necessario per questa richiesta.

**Soluzione proposta.** Esporre `Video > Fullscreen: On/Off` tramite un'unica
operazione condivisa con F11. Proposta di persistenza: `video.fullscreen`,
predefinito Off per le configurazioni esistenti, applicato alla creazione
della finestra. Sincronizzare la voce con lo stato effettivo della finestra.

- [ ] Aggiungere impostazione e riga del menu; verificare navigazione e
  scorrimento dopo l'aumento delle voci Video.
- [ ] Centralizzare F11 e menu nello stesso setter, evitando auto-repeat e
  doppi toggle; gestire anche cambiamenti di stato richiesti dal sistema.
- [ ] Conservare/ripristinare geometria windowed e stato massimizzato;
  mantenere posizione valida se il monitor precedente non è disponibile.
- [ ] Riutilizzare resize/DPI/cursore/presenter esistenti; non ricreare la
  macchina guest né cambiare aspect, zoom o velocità di emulazione.
- [ ] Verificare menu con tastiera/gamepad, F11 durante il menu, riavvio,
  Alt-Tab, minimize/restore, VSync e ritorno windowed su Windows. Riutilizzare
  le prove PERF-04 già concluse e aggiungere solo i casi fullscreen mancanti;
  distinguere prove Wine da Windows nativo.

**Criterio di chiusura:** menu, F11 e finestra concordano sullo stato, si può
entrare/uscire senza riavviare il gioco e senza perdere geometria, focus o
contenuti; la preferenza viene ripristinata al successivo avvio.

## FEAT-03-TACTICAL-OVERLAY — Riepilogo battaglia e previsione turni

**Stato:** da progettare; fonti dei dati ancora da verificare.
**Priorità:** P3. **Complessità:** alta, soprattutto per la previsione.

**Richiesta preservata.** Quadrato mostra leggi, numero di turni, missione e
ordine dei turni; la pressione prolungata rende l'overlay fisso. Prima della
conferma di un'azione l'overlay fisso mostra eventuali modifiche all'ordine.

**Evidenza e analisi.** Il progetto ha binding host, livelli UI e profili ROM
EUR/USA, ma nei percorsi ispezionati non emerge un lettore verificato che
esponga insieme questi dati o un predittore dell'ordine. La schermata leggi
può aiutare a confrontare i valori, ma i pixel non sono una fonte strutturata.
Il token fisico da valutare per Quadrato è `West` (posizione X su Xbox);
l'attuale UI dei binding usa etichette Xbox. Serve anche un binding tastiera
e va gestito il conflitto con rimappature già salvate.

**Soluzione proposta, prima fase: dati e overlay consultivo.** Introdurre un
lettore tipizzato e in sola lettura dello stato di battaglia, abilitato per
profili ROM verificati. Esporre al frontend uno snapshot coerente con validità
e provenienza dei dati; renderizzarlo su un livello host indipendente dallo
zoom. Non dedurre indirizzi EUR/USA per semplice analogia.

- [ ] Identificare e verificare leggi attive, missione/obiettivo, unità,
  contatore di turno e dati dello scheduler; definire che cosa conta come
  «turno» rispetto alle attivazioni individuali. Confrontare con schermate e
  sequenze reali; mostrare «non disponibile» per dati non verificati.
- [ ] Progettare il lettore vicino a
  [ffta_rom.rs](../../crates/gba-core/src/ffta_rom.rs) e ai moduli battaglia,
  con controlli di limiti, identità e fase; aggiornare a fine frame/evento
  senza scrivere nella RAM guest o modificare RNG, input e clock.
- [ ] Aggiungere un'azione host rimappabile in Input, con Quadrato/`West` come
  proposta iniziale; risolvere i conflitti senza sovrascrivere i binding GBA.
  Verificare XInput, DirectInput e Gilrs e rendere comprensibile l'etichetta
  per un controller PlayStation.
- [ ] Implementare una macchina a stati per la pressione. Proposta da validare:
  visibile alla pressione, nascosto al rilascio breve; tenere premuto almeno
  400 ms lo fissa anche dopo il rilascio; una nuova pressione breve lo chiude.
  Usare tempo reale, così Fast non modifica la soglia; ignorare auto-repeat.
- [ ] Disegnare leggi, missione, turno e ordine corrente in un pannello
  leggibile che non copra conferme e informazioni essenziali. Riutilizzare
  [ui_layer](../../crates/screen/src/ui_layer.rs) e la composizione software.
  Gestire resize, DPI, fullscreen e testo lungo; mostrare lo stato «fisso».
- [ ] Nascondere durante i menu host e fuori battaglia; su perdita focus,
  disconnessione pad, load/Undo, cambio ROM o fine battaglia azzerare pressioni
  e dati obsoleti. Ricalcolare solo da una scena valida prima di riapparire.

**Soluzione proposta, seconda fase: previsione prima della conferma.**
Prima verificare come il gioco determina l'ordine e quali azioni lo cambiano.
Preferire dati di previsione già preparati dal guest, se esistono; altrimenti
estrarre una funzione pura su uno snapshot. Non eseguire azioni speculative
sulla macchina attiva e non consumare RNG per costruire l'anteprima.

- [ ] Individuare selezione azione/bersaglio, fase di conferma e regole reali
  dello scheduler, inclusi spareggi ed effetti di stato; delimitare le azioni
  per cui una previsione deterministica è possibile.
- [ ] Calcolare ordine corrente e previsto su dati immutabili; evidenziare
  solo gli spostamenti quando l'overlay è fisso. Per esiti casuali o dati
  insufficienti indicare previsione condizionale/non disponibile, senza
  presentare un ordine ipotizzato come certo.
- [ ] Aggiornare l'anteprima al cambio di azione/bersaglio e rimuoverla su
  annullamento; dopo la conferma sostituirla con il nuovo stato autorevole.
- [ ] Confrontare previsione e ordine osservato per azioni che lo lasciano
  invariato e che lo modificano, unità rimosse e stati rilevanti; aggiungere
  regressioni dopo aver verificato le regole, non prima.
- [ ] Confrontare replay overlay acceso/spento: stesso stato guest, RNG,
  frame nativi e audio deterministico a parità di input GBA; misurare il costo
  della lettura/renderizzazione e verificare che la pressione non confermi
  contemporaneamente un'azione di gioco.

**Criterio di chiusura:** prima fase utilizzabile e verificata separatamente;
la funzionalità completa si chiude solo con anteprima attendibile nei casi
supportati, limiti visibili e nessun effetto dell'overlay sull'esecuzione guest.
La correzione BUG-01 aiuta il confronto delle leggi, ma non sostituisce la
verifica delle strutture dati e dello scheduler.

## Consegna di ciascuna attività futura

- [ ] Registrare nel handoff l'ID selezionato, ciò che è già concluso e il
  criterio mancante; chiudere ogni sottofase senza riaprirla al successivo resume.
- [ ] Eseguire verifiche mirate al cambiamento; suite core in serie quando
  condividono globali. Annotare separatamente Windows nativo, Wine e Linux.
- [ ] Aggiornare documentazione pertinente e record in `docs/changes/`,
  rigenerare `CHANGELOG.md` ed eseguire `python3 scripts/project_memory.py check`.
  Conservare ROM, salvataggi e registrazioni fuori da commit e pacchetti.
