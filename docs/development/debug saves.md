# TODO — Debug saves

Aggiornato il 23 ottobre 2026. Numerazione stabile **DS-01…DS-26**, nello stesso
ordine della lista originale. Il nome breve è riportato in ciascuna voce.

Prima ricognizione: **3 casistiche validate, 10 parziali, 13 non coperte**.
Una spunta vale soltanto per lo scenario e la prova indicati. Le verifiche già
concluse sono riutilizzate; non sono stati rifatti build, benchmark o confronti
completi per ricostruire il contesto. Nuove ispezioni: Windows/Wine, tastiera,
interprete e BIOS HLE, su copie isolate.

- **Validata**: evento o percorso richiesto osservato, con riscontro conservato.
- **Parziale**: snapshot candidato e alcuni controlli presenti; restano criteri espliciti.
- **Non coperta**: nessun punto mirato individuato nel campione. Non significa che
  sia impossibile raggiungerlo giocando dai salvataggi disponibili.

## Raccolta numerata locale

La cartella ignorata **`saves/debug-cases/`** contiene una copia di `ffta.sav`,
`catalog.json`, `README.local.md` e **13 slot** sotto
`SaveStates/<SHA-256-ROM-EUR>/`. Il numero dello slot corrisponde al caso:
DS-01 → `slot-1.ffstate`, DS-19 → `slot-19.ffstate`, ecc.
I casi non coperti restano vuoti; gli slot parziali sono candidati, non spunte.
Sono copie esatte di **sei snapshot distinti**: lo stesso punto può coprire più
casi. Il nome descrittivo e lo stato sono nel catalogo; il menu conserva le
etichette generiche Slot N. Nessun originale è stato rinominato o sovrascritto.

Per usare gli slot oltre 9 abilitare **Debug > Extended saves**. La cartella degli
slot è affiancata al `.sav` selezionato: usare la copia della raccolta, oppure
importare soltanto i file desiderati in posizioni libere della propria cartella.
Preservare prima eventuali slot personali. I dati di gioco restano fuori da Git
e dai pacchetti. Vedi [Extended saves](../extended_saves.md).

## Fonti disponibili

- **F-01 — Progresso cartuccia**: `saves/saves.zip` → `saves/ffta.sav`, copia di
  lavoro `target/desktop-perf/user-fixture/ffta.sav`. Uguaglianza verificata.
- **F-02 — Battaglia fornita**: slot 1 nello stesso ZIP, copia verificata in
  `target/desktop-perf/user-fixture/slot-1.ffstate`.
- **F-03 — Schieramento italiano**: `saves/slot-3.ffstate`.
- **F-04 — Mappa e menu**: `target/desktop-perf/user-fixture/worldmap.ffstate`.
- **F-05 — Mappa/pub inglese**: slot 5 in
  `target/rom-data-bg-alternate-callers/native-color-options/Data/saves/SaveStates/<EUR>/`.
- **F-06 — Spedizione accettata, giorno 9**: slot 2 in
  `target/rom-data-progression-gates/windows-dispatch/Data/saves/SaveStates/<EUR>/`.
- **F-07 — Prima del rientro, giorno 18**: slot 3 nella cartella di F-06.
- **F-08 — Mission Cleared, giorno 19**: slot 4 nella cartella di F-06.
- **F-09 — Prima del turno nemico**:
  `target/desktop-perf/ai-turn-replay/start.ffstate` e `input.input`.

Sono otto snapshot ordinari e un salvataggio cartuccia selezionati. SHA-256,
checksum delle anteprime e dei payload, versione 1, identità ROM e percorsi esatti sono nel manifest locale
`target/debug-saves-audit/source-manifest.json`. Il vecchio inventario di 49
payload include stati sintetici/esperimenti scartati: non sono nuove casistiche
ordinarie e non vengono promossi automaticamente a prove.

## Casistiche


### DS-01 — Crescita clan

- [x] **VALIDATA** — `crescita_clan`.

Richiesta: Missione che aumenta un’abilità/statistica del clan: prima dell’ultimo colpo o della conferma del risultato. È il passaggio più utile da verificare adesso.

Fonte: F-07; riscontro F-08. Raccolta: **slot 1**.

Riscontro: Mad Alchemist conclusa con Sigmund: due abilità del clan salgono da 1 a 2 e da 3 a 4. Schermata RESULTS ispezionata; prova esistente di 1000 frame con stato completo, immagini e audio uguali. Riprendere il viaggio al giorno 18, attendere il giorno 19 e confermare i risultati con A.

Ambito restante: Valida per questo incremento; altri parametri e soglie restano fuori dal campione.

### DS-02 — Rientro da spedizione riuscita

- [x] **VALIDATA** — `rientro_spedizione`.

Richiesta: Rientro da una missione di spedizione riuscita: sulla mappa, poco prima che trascorrano gli ultimi giorni necessari, con indicazioni su come farla terminare.

Fonte: F-07; riscontro F-08, avvio F-06. Raccolta: **slot 2**.

Riscontro: Messaggi Mission Cleared e Dispatched unit returned osservati al giorno 19. Confermati ritorno di Sigmund e ricompense. Stessa prova completa già conclusa di DS-01, riutilizzata senza ripeterla.

Ambito restante: Valida per Mad Alchemist, durata 10 giorni; il fallimento è DS-20.

### DS-03 — Premio sbloccato dalla progressione del clan

- [ ] **NON COPERTA** — `premio_progressione_clan`.

Richiesta: Sblocco di un premio legato alla crescita del clan: prima del risultato che lo assegna, se riesci a individuare un caso.

Fonte: Nessun checkpoint ordinario individuato.

Riscontro: Il risultato di Mad Alchemist assegna ricompense della missione. Non dimostra un premio aggiuntivo sbloccato da una soglia del clan: i relativi campi return/output hanno zero uso ordinario nella prova disponibile.

Ambito restante: Serve un punto prima dello sblocco effettivo; i test isolati sulle tabelle non chiudono questa casistica.

### DS-04 — Schieramento della squadra

- [ ] **PARZIALE** — `schieramento_squadra`.

Richiesta: Schieramento prima della battaglia: con una squadra abbastanza varia da poter cambiare personaggi e posizioni.

Fonte: F-03. Raccolta: **slot 4**.

Riscontro: Lo slot 3 fornito apre lo schieramento italiano, con 0/6 unità e caselle di assegnazione. Nuova sessione Wine: caricamento, selezione/spostamento e ritorno allo snapshot; chiusura normale. La vista Party controllata separatamente mostra sei membri di razze diverse.

Ambito restante: Manca una sequenza verificata di assegnazione di più membri, cambio/rimozione e conferma finale della formazione. Lo slot è già disponibile: non serve ricrearlo.

### DS-05 — Più leggi attive e relativo aiuto

- [ ] **PARZIALE** — `leggi_multiple`.

Richiesta: Battaglia con più leggi attive: da cui aprire sia l’elenco sia l’aiuto delle leggi.

Fonte: F-03; altra battaglia F-02. Raccolta: **slot 5**.

Riscontro: Elenco/HELP già verificati nei layout noti. La nuova ispezione di F-03 mostra una sola coppia: divieto Cavalleria, consiglio Carica. Anche la battaglia storica mostra una sola legge.

Ambito restante: Serve una battaglia con almeno due leggi contemporaneamente; una tabella a più righe ma con righe vuote non soddisfa il caso.

### DS-06 — Infrazione e intervento del giudice

- [ ] **NON COPERTA** — `infrazione_legge`.

Richiesta: Infrazione di una legge: immediatamente prima dell’azione, per vedere intervento del giudice e penalità.

Fonte: Nessun checkpoint prima di un’infrazione individuato.

Riscontro: Nelle fonti selezionate non è documentata un’azione vietata eseguita con relativa penalità.

Ambito restante: Salvare prima dell’azione e indicare abilità, bersaglio e legge violata.

### DS-07 — Battaglia affollata e unità differenti

- [ ] **PARZIALE** — `battaglia_affollata`.

Richiesta: Battaglia affollata: molte unità, razze e mostri diversi contemporaneamente visibili.

Fonte: F-02; replay derivato F-09. Raccolta: **slot 7**.

Riscontro: Battaglia ordinaria e pubblicazione di più corpi già osservate. Esistono prove su sprite, HUD e turno nemico.

Ambito restante: Restano una scena sufficientemente affollata, varietà di razze/mostri e osservazione contemporanea dei gruppi richiesti. Non equivalgono al solo conteggio di sprite pubblicati.

### DS-08 — Turno nemico lento o problematico

- [ ] **PARZIALE** — `turno_nemico_lento`.

Richiesta: Turno nemico particolarmente lungo o problematico: prima di passare il turno, indicando come riprodurlo.

Fonte: F-09 e target/desktop-perf/ai-turn-replay/input.input. Raccolta: **slot 8**.

Riscontro: Replay ordinario di 3530 frame: attivazione e movimento di un nemico, poi ritorno al giocatore, già verificati.

Ambito restante: Il rallentamento originale non è riprodotto. Serve identificarne un caso concreto; non sono stati avviati nuovi benchmark.

### DS-09 — Magie, evocazioni ed effetti ad area

- [ ] **NON COPERTA** — `magie_effetti_area`.

Richiesta: Magia o abilità con effetti vistosi: prima della conferma del bersaglio. Sono utili esempi diversi, soprattutto effetti ad area ed evocazioni.

Fonte: Nessun checkpoint prima della conferma individuato.

Riscontro: Il campione non contiene ancora una sequenza classificata che esegua gli effetti richiesti. I test dei dati grafici non dimostrano questa scena.

Ambito restante: Da cercare prima nelle unità della battaglia disponibile; altrimenti preparare punti prima di magia, area ed evocazione.

### DS-10 — Alterazioni di stato

- [ ] **NON COPERTA** — `alterazioni_stato`.

Richiesta: Unità con alterazioni di stato: icone visibili e, possibilmente, effetti che modificano movimento, azioni o ordine dei turni.

Fonte: Nessun checkpoint classificato con gli stati richiesti.

Riscontro: Non è verificata una sequenza con icone ed effetti di stato effettivamente attivi.

Ambito restante: Cercare prima nella battaglia esistente; servono anche effetti su movimento, azioni o ordine dei turni.

### DS-11 — KO e rianimazione

- [ ] **NON COPERTA** — `ko_rianimazione`.

Richiesta: KO e rianimazione: prima del colpo che manda KO oppure prima della conferma della rianimazione.

Fonte: Nessun checkpoint prima dei due eventi individuato.

Riscontro: La conferma di un altro oggetto, come Echo Grass, non dimostra una rianimazione.

Ambito restante: Cercare prima nella battaglia esistente; salvare prima del KO e prima della conferma della rianimazione.

### DS-12 — Terreno, dislivelli e ostacoli

- [ ] **PARZIALE** — `terreno_complesso`.

Richiesta: Terreno complesso: dislivelli, acqua, ostacoli e personaggi parzialmente coperti dallo scenario.

Fonte: F-03; altra mappa F-02. Raccolta: **slot 12**.

Riscontro: La scena di schieramento mostra dislivelli marcati, pareti e vegetazione; caricamento e rappresentazione ispezionati.

Ambito restante: Restano movimento tra quote, occlusioni durante il percorso e acqua. Le caselle blu di schieramento non sono acqua.

### DS-13 — Vittoria in battaglia e ricompense

- [ ] **NON COPERTA** — `vittoria_ricompense`.

Richiesta: Fine battaglia con ricompense: prima dell’ultimo colpo, possibilmente con aumento di livello o apprendimento di un’abilità.

Fonte: Nessun punto prima dell’ultimo colpo individuato.

Riscontro: Le ricompense della spedizione di DS-02 non sono una fine battaglia.

Ambito restante: Servono ultimo colpo, schermata ricompense e possibilmente aumento di livello o abilità appresa; da cercare nella battaglia attuale.

### DS-14 — Sconfitta o fallimento dell’obiettivo

- [ ] **NON COPERTA** — `sconfitta_obiettivo`.

Richiesta: Sconfitta o fallimento dell’obiettivo: poco prima dell’evento conclusivo.

Fonte: Nessun punto prima dell’esito individuato.

Riscontro: Nessuna sconfitta o condizione di fallimento osservata nel campione.

Ambito restante: Preparare un punto immediatamente precedente e indicare come provocare l’esito.

### DS-15 — Obiettivo di battaglia alternativo

- [ ] **NON COPERTA** — `obiettivo_alternativo`.

Richiesta: Battaglia con obiettivo diverso dall’eliminazione dei nemici: prima di soddisfarlo o fallirlo, indicando l’obiettivo.

Fonte: Nessun obiettivo alternativo identificato.

Riscontro: Nelle fonti selezionate non è documentata una vittoria/fallimento con requisito diverso dall’eliminazione dei nemici.

Ambito restante: Indicare missione e requisito; salvare prima di completarlo o fallirlo.

### DS-16 — Squadra e classi avanzate

- [ ] **PARZIALE** — `squadra_classi_avanzate`.

Richiesta: Squadra avanzata: molte classi sbloccate, razze diverse e numerose abilità; da cui esplorare cambio classe, statistiche ed equipaggiamento.

Fonte: F-04; nuova ispezione Party da F-05. Raccolta: **slot 16**.

Riscontro: Party con sei membri e razze diverse osservato. Menu Unit/Equip e Change Jobs hanno prove esistenti, compreso un confronto di 540 frame.

Ambito restante: Non sono ancora censite molte classi sbloccate e numerose abilità: una squadra varia da sola non chiude il requisito avanzato.

### DS-17 — Inventario, categorie e scorrimento

- [ ] **PARZIALE** — `inventario_categorie`.

Richiesta: Inventario ricco: categorie diverse, oggetti con descrizioni lunghe e liste che richiedono scorrimento.

Fonte: F-04; nuova ispezione da F-05. Raccolta: **slot 17**.

Riscontro: Prove esistenti sull’elenco copricapi e relative proprietà. Nuova GUI: Party → unità → Equip → elenco oggetti, con cinque copricapi visibili e indicatore di scorrimento. Seconda sessione breve ha ricontrollato Party, senza raggiungere l’inventario generale.

Ambito restante: Restano inventario generale, tutte le categorie, scorrimento effettivo e descrizioni lunghe. La prima sessione interattiva è terminata con SIGTERM143 senza risultato finale: nessuna chiusura normale o validazione completa attribuita a quella sessione.

### DS-18 — Negozio avanzato e transazioni

- [ ] **PARZIALE** — `negozio_avanzato`.

Richiesta: Negozio avanzato: molte categorie e oggetti disponibili, con denaro sufficiente per provare acquisto, vendita e conferme.

Fonte: F-05; altre prove da F-04. Raccolta: **slot 18**.

Riscontro: Shop → Buy, categorie Buy/Sell e prezzi hanno prove circoscritte esistenti; la scena Buy di 600 frame è già confrontata.

Ambito restante: Restano assortimento avanzato e transazioni di acquisto/vendita con conferma e annullamento; il solo menu non le dimostra.

### DS-19 — Pub e accettazione di una missione

- [x] **VALIDATA** — `pub_accettazione_missione`.

Richiesta: Pub con molte missioni disponibili: dettagli, requisiti, selezione del partecipante e conferma della spedizione accessibili.

Fonte: F-05; avvio accettato F-06. Raccolta: **slot 19**.

Riscontro: Navigazione tra missioni, pagine dettagli/requisiti, Mad Alchemist, scelta di Sigmund e conferma della spedizione già osservate nella GUI ordinaria. Costo di 600 gil applicato. Le catture e il risultato terminale sono stati ricontrollati.

Ambito restante: Valida per questo percorso del pub; non per ogni missione o requisito di classe/oggetto.

### DS-20 — Spedizione fallita

- [ ] **NON COPERTA** — `spedizione_fallita`.

Richiesta: Spedizione fallita: poco prima del rientro o della comunicazione del risultato.

Fonte: Nessun rientro fallito individuato.

Riscontro: La spedizione disponibile è riuscita e non copre questo esito.

Ambito restante: Serve un punto prima del rientro o della comunicazione del fallimento.

### DS-21 — Oggetti missione e utilizzo

- [ ] **PARZIALE** — `oggetti_missione`.

Richiesta: Oggetti missione: elenco popolato e una missione che ne richiede la selezione o l’utilizzo.

Fonte: F-04. Raccolta: **slot 21**.

Riscontro: Clan → Mission Item mostra Chocobo Egg, elenco 1/64; prova esistente di 520 frame con uso positivo dei dati di visualizzazione.

Ambito restante: Restano una missione che richieda l’oggetto, selezione effettiva e relativo utilizzo/consumo. Il solo elenco è parziale.

### DS-22 — Banca mostri popolata e cattura

- [ ] **NON COPERTA** — `banca_mostri_popolata`.

Richiesta: Banca mostri popolata: con più mostri consultabili; utile anche un punto da cui provare una cattura.

Fonte: Nessun elenco popolato individuato.

Riscontro: La precedente verifica della banca riguarda il menu e l’elenco vuoto.

Ambito restante: Occorre un salvataggio con mostri già presenti; utile un secondo punto prima di una cattura.

### DS-23 — Reclutamento o inserimento del nome

- [ ] **NON COPERTA** — `reclutamento_nome`.

Richiesta: Reclutamento o inserimento di un nome: subito prima della scelta o dell’apertura della tastiera.

Fonte: Nessun checkpoint ordinario prima dell’evento individuato.

Riscontro: Font e tastiera hanno test mirati sui dati, senza una scena ordinaria equivalente nei salvataggi selezionati.

Ambito restante: Serve un reclutamento o l’apertura della tastiera ottenuti dal gioco.

### DS-24 — Mappa avanzata ed eventi

- [ ] **PARZIALE** — `mappa_avanzata`.

Richiesta: Mappa avanzata: molte località sbloccate, eventi disponibili e possibilità di entrare in città e battaglie diverse.

Fonte: F-04; F-05. Raccolta: **slot 24**.

Riscontro: Mappa del mondo e accesso a città/pub/battaglie già disponibili e verificati per i percorsi iniziali.

Ambito restante: Restano molte località sbloccate ed eventi avanzati. La mappa iniziale non soddisfa questa estensione.

### DS-25 — Scene narrative differenti

- [ ] **NON COPERTA** — `scene_narrative`.

Richiesta: Scene narrative differenti: dialoghi lunghi, più interlocutori, cambi di sfondo, dissolvenze o animazioni particolari. Bastano alcuni esempi rappresentativi.

Fonte: Nessun checkpoint narrativo dedicato individuato.

Riscontro: I dialoghi del pub non coprono la varietà richiesta di scene narrative. L’introduzione riproducibile da avvio è una prova separata.

Ambito restante: Raccogliere alcuni punti prima di dialoghi lunghi, più interlocutori, cambi sfondo o dissolvenze.

### DS-26 — Boss o finale

- [ ] **NON COPERTA** — `boss_finale`.

Richiesta: Boss, battaglia finale o finale del gioco: se hai già progressi vicini a questi punti.

Fonte: Nessun checkpoint corrispondente individuato.

Riscontro: Nessuna battaglia classificata come boss/finale né finale del gioco nel campione.

Ambito restante: Preparare uno o più punti prima di questi eventi se il progresso lo permette.

## Prove e prossimi controlli

Le prove del rientro/crescita e del percorso al pub sono descritte nel
[record della spedizione ordinaria](../changes/2026-10-23-consume-european-progression-gates-and-receipt-records.json).
La coppia di 1000 frame è conclusa e non è stata ripetuta. Le immagini RESULTS,
Mission Cleared e accettazione sono state ricontrollate. Premi di missione e
premi di progressione del clan rimangono casi separati, DS-02 e DS-03.

Per gli altri sottocasi riutilizzare le prove di [leggi e schieramento](../ffta_laws_layout.md),
[negozio/banca](../ffta_shop_bank_mouse.md), il
[replay nemico](../changes/2026-10-20-record-observed-enemy-activation-replay.json)
e i contratti/scenari già registrati in [dati di gioco](../rom_data_gameplay.md).

Nuove osservazioni e risultati terminali sono in
`target/debug-saves-audit/{DS-04,DS-17-global}/`, con catture e guardie sugli
originali. La sessione intermedia `DS-17/` è incompleta (SIGTERM143), conserva
solo le osservazioni fino a Equip e non dà una prova di chiusura normale.
Le nuove ispezioni non attestano equivalenza completa, prestazioni, Windows
nativo o copertura hardware. Non sono stati creati nuovi snapshot di gioco.

Prossimi controlli sui dati già presenti: completare assegnazione/rimozione di
più unità in DS-04 e inventario generale/scorrimento/descrizioni in DS-17;
cercare magia, stati e fine battaglia nelle unità disponibili prima di richiedere
nuovi punti per DS-09…DS-15. Per ampliare la raccolta sono particolarmente utili
DS-03, DS-05 con più leggi, DS-06, DS-20, DS-22 e DS-26.
