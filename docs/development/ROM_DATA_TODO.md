# TODO — Estrazione ROM, dati decodificati e mod

Richiesta aggiornata il 21 ottobre 2026. **FEAT-04-ROM-DATA** è in esecuzione;
inventario e contratto iniziale sono definiti. Le migrazioni runtime restano aperte.
Ambito: workspace desktop, Windows come piattaforma principale di verifica;
Linux come controllo aggiuntivo. `AndroidPort/` richiede lavoro separato.

## Obiettivo e ordine di lavoro

L'obiettivo principale è scompattare la ROM selezionata in file di risorse,
decodificarli in formati utilizzabili e far leggere al runtime quei dati.
Al primo avvio si prepara il dataset; agli avvii successivi si riutilizza la
versione valida. Il supporto alle mod e a modifiche più ampie viene dopo.

Ordine: **inventario → estrazione e decodifica → preparazione all'avvio →
consumo diretto → verifica → mod → modifiche estese**.
Creare file che il gioco non usa non soddisfa l'obiettivo. Anche ricomporre
soltanto una ROM dai file estratti è un passaggio eventualmente utile al
confronto, ma non completa il percorso di lettura dei dati decodificati.

Il caricamento dei file può avvenire all'avvio o prima del loro utilizzo, con
cache in memoria: «lettura dai file» non richiede accessi al disco per pixel,
campione audio o istruzione. I miglioramenti di velocità restano da misurare.

## Punto di partenza verificato e vincoli

- [play.rs](../../crates/recomp/src/play.rs) legge già tutta la ROM con
  `std::fs::read` e la passa alla macchina. In
  [mem.rs](../../crates/gba-core/src/mem.rs), `MemMap.rom` è un `Vec<u8>`;
  le normali letture guest accedono a quel buffer in RAM.
- Una ROM contiene codice, tabelle, puntatori e risorse anche compresse o
  condivise. Non è un archivio con un filesystem da estrarre genericamente:
  formati e confini vanno individuati per ciascuna famiglia di risorse.
- [ffta_rom.rs](../../crates/gba-core/src/ffta_rom.rs) riconosce profili EUR/USA
  tramite hash esatti. [ensure_native](../../crates/recomp/src/main.rs) lega
  la cache di traduzione all'immagine e agli altri parametri di compilazione.
  Non basta cambiare un hash dichiarato per rendere compatibili nuovi dati.
- Gli [stati macchina](../../crates/gba-core/src/save_state.rs) e gli
  [slot desktop](../../crates/recomp/src/save_state.rs) usano l'identità ROM.
  Il nuovo dataset dovrà restare coerente con salvataggi, load e Undo.
- Il codice eseguibile guest e le strutture non ancora migrate possono
  richiedere l'immagine originale. Eliminare del tutto la dipendenza dalla
  `.gba` è un criterio separato, da valutare dopo aver censito tali dipendenze;
  non è una conseguenza automatica dell'estrazione delle risorse.

## ROMDATA-01 — Inventario e contratto dei dati

**Dipendenze:** nessuna. **Priorità:** prima fase obbligatoria.

- [x] Censire grafica, palette, tile/mappe, sprite/animazioni, testi, tabelle
  di gioco e audio. Per ogni famiglia annotare formato, compressione, confini,
  puntatori, riferimenti incrociati, routine di lettura e grado di verifica.
  Distinguere dati statici della ROM e stato dinamico in RAM.
- [ ] Verificare separatamente EUR/USA e lingue. Riutilizzare i lettori già
  presenti come evidenza parziale; non dedurre offset per sola analogia.
- [x] Definire ID stabili delle risorse, schema versionato e manifest con
  hash della ROM sorgente, versione estrattore/decoder, file e hash dei contenuti,
  dipendenze e copertura. Conservare la provenienza dagli intervalli originali.
- [x] Scegliere una prima famiglia verificabile dall'estrazione fino al gioco,
  in base alle routine individuate. Separare contenuti decodificati e metadati
  necessari: per esempio una sola immagine non conserva palette indicizzate,
  priorità, trasparenza e animazioni; un PCM non conserva il sequenziamento audio.
- [x] Pubblicare la matrice di migrazione: per ogni famiglia indicare estrattore,
  decoder, lettore runtime, verifiche e dipendenze residue dalla ROM.

**Chiusura:** contratto e prima famiglia definiti con evidenza; famiglie ancora
ignote elencate esplicitamente. L'inventario non è la migrazione completa.

**ROMDATA-01-CONTRACT completato; ROMDATA-01 parziale:**
[inventario e contratto](ROM_DATA_INVENTORY.md).
Prima famiglia: `graphics.obj-dimensions`, metadati statici verificati sui due
profili esatti. L'inventario distingue le fonti note da confini/cataloghi/lingue
non verificati; non dichiara un catalogo completo delle risorse del gioco.
La verifica EUR/USA è completa per la prima famiglia; cataloghi regionali e
linguistici delle altre famiglie restano da individuare e verificare.

## ROMDATA-02 — Estrattori e decoder

**Dipendenze:** ROMDATA-01. Procedere per famiglie senza dichiarare finito il
resto del catalogo quando funziona il primo prototipo.

- [x] Implementare parser con controlli su lunghezze, offset, puntatori,
  dimensioni decodificate e riferimenti; rifiutare input troncati o incoerenti.
- [x] Estrarre e decodificare la prima famiglia nel formato concordato,
  mantenendo le informazioni necessarie a riprodurre il comportamento originale.
  Blocchi sconosciuti possono restare grezzi, marcati come non decodificati.
- [x] Rendere l'output deterministico e ispezionabile: gli stessi input e la
  stessa versione producono manifest e dati equivalenti, senza percorsi assoluti.
- [x] Verificare con fixture sintetiche e confronti locali con il decoder
  originale o con dati osservati nel gioco. Quando il formato lo consente,
  verificare anche il roundtrip, senza richiedere compressione byte-identica
  se più codifiche rappresentano gli stessi dati.
- [ ] Estendere estrattori e decoder alle altre famiglie della matrice,
  registrando per ciascuna contenuti supportati e limiti.

**Chiusura per famiglia:** file decodificati corretti, metadati sufficienti e
input errati gestiti. La sola esportazione di blocchi binari non basta.

**ROMDATA-02-OBJ-DIMENSIONS completato:** estrattore e decoder tipizzato,
CLI `rom-data extract|verify`, manifest e controlli di provenienza.
Le prime quattro voci sono completate per questa famiglia (senza puntatori o
compressione); parser delle altre famiglie rimangono da implementare.
Test Linux e Windows/Wine passano anche con le due ROM locali. Dettagli e
limiti in [risorse decodificate](../rom_data.md). Il runtime non è collegato.

## ROMDATA-03 — Preparazione automatica e riuso all'avvio

**Dipendenze:** ROMDATA-02 per la prima famiglia.

- [ ] Integrare la preparazione dopo selezione/verifica della ROM e prima
  dell'avvio del gioco: estrarre quando il dataset manca o la versione cambia;
  riutilizzarlo quando identità e contenuti sono validi.
- [ ] Definire i percorsi attraverso lo storage installed/portable esistente.
  Proposta da confermare: `Data/resources/<rom-hash>/<schema>/` in portable;
  rispettare [le regole di rilocazione](../portable.md). Tenere separati dati
  base rigenerabili, future mod utente, cache native e salvataggi.
- [ ] Scrivere in una directory temporanea, validare e pubblicare soltanto un
  dataset completo; gestire annullamento, spazio insufficiente, avvio concorrente
  e crash senza sostituire l'ultima versione valida con un risultato parziale.
- [ ] Mostrare avanzamento ed errori comprensibili; prevedere la rigenerazione
  di dati mancanti/corrotti dalla ROM sorgente senza sovrascrivere mod o save.
- [ ] Verificare primo avvio, riavvio senza riestrazione, cambio ROM/regione,
  aggiornamento decoder e spostamento della cartella portable. Definire come
  riconoscere una sorgente cambiata prima di riutilizzare il suo dataset.

**Chiusura:** un primo avvio prepara le risorse; quelli successivi riusano un
dataset valido. L'estrazione resta locale e non modifica la ROM sorgente.

## ROMDATA-04 — Uso diretto dei dati estratti nel runtime

**Dipendenze:** ROMDATA-01/02/03. **Cuore dell'obiettivo principale.**

- [ ] Introdurre un accesso alle risorse tramite ID e strutture tipizzate,
  con caricamento/precaricamento dai file decodificati. Definire budget di memoria,
  durata dei dati e invalidazione; evitare I/O nei callback audio e nei percorsi
  per pixel o istruzione.
- [ ] Per ogni famiglia seguire tutti i consumatori: codice nativo tradotto,
  interprete, letture CPU/DMA e percorsi host come rendering e audio. Individuare
  dove il gioco richiede byte grezzi, decomprime, copia o interpreta le risorse.
- [ ] Collegare la prima famiglia al caricatore e sostituire il consumo della
  corrispondente risorsa ROM con quello dei dati estratti. Se occorre trasferire
  dati in RAM/VRAM guest, preservarli con layout, registri ed effetti osservabili
  corretti; non applicare lookup generici a ogni lettura del bus.
- [ ] Quando si sostituiscono routine guest, preservare cicli, IRQ, DMA, RNG e
  ordine degli eventi. Dimostrare come si conserva il tempo guest se una
  decompressione viene anticipata all'estrazione: saltarla senza contabilizzarne
  gli effetti non è una sostituzione equivalente.
- [ ] Distinguere identità della ROM base, dataset e codice compilato. Rivedere
  costanti incorporate, hook e cache native: nessuna libreria deve riutilizzare
  dati o ipotesi incompatibili con il dataset effettivamente caricato.
- [ ] Aggiungere diagnostica di provenienza e contatori di uso/fallback per
  famiglia. Nei test delle risorse migrate rendere rilevabile un accesso al
  vecchio percorso; verificare anche che file mancanti non vengano ignorati
  silenziosamente a favore della ROM.
- [ ] Migrare progressivamente tutte le famiglie inventariate. Registrare
  separatamente codice, dati ancora grezzi e letture ROM residue; un fallback
  temporaneo mantiene la compatibilità ma lascia aperta la relativa migrazione.

**Chiusura per famiglia:** il gioco usa il file decodificato e la provenienza è
verificata. **Chiusura complessiva:** tutte le famiglie di dati previste dalla
matrice sono migrate e verificate; le esclusioni motivate sono dichiarate.
Un solo esempio funzionante è una consegna parziale, non il completamento.

## ROMDATA-05 — Equivalenza, salvataggi e prestazioni

**Dipendenze:** ogni integrazione ROMDATA-04, prima di abilitarla normalmente.

- [ ] Confrontare originale e dataset estratto senza modifiche, con stessi
  input: stato guest a fine frame, RNG, clock, RAM/VRAM/OAM, immagini e audio
  deterministico. Coprire intro, mappa, battaglia, menu e transizioni pertinenti;
  riutilizzare le prove già valide senza confondere copertura e completezza.
- [ ] Verificare percorsi nativi e interprete, Fast, pausa, load/Undo e ripristino
  backup. Ricostruire risorse host e cache dopo il caricamento senza serializzare
  puntatori o duplicare tutti i file estratti negli stati macchina.
- [ ] Definire identità e compatibilità di `.ffstate` e `.sav`: il dataset base
  equivalente deve preservare gli stati esistenti quando possibile. Se cambiano
  layout o semantica, versionare e rifiutare combinazioni incompatibili prima di
  modificare la macchina o scrivere il salvataggio.
- [ ] Misurare separatamente estrazione iniziale, avvio con dataset pronto,
  spazio su disco, RAM e tempi di frame/caricamento/audio rispetto al percorso
  attuale in memoria. Fissare i limiti di regressione prima del confronto.
  Registrare anche un risultato neutro o peggiore: l'estrazione non prova velocità.
- [ ] Verificare Windows nativo; usare Windows/Wine e Linux come prove distinte.
  Aggiornare launcher/documentazione/packager senza includere ROM, risorse estratte,
  save personali o codice generato nei repository e nei pacchetti distribuiti.

**Consegna principale:** estrazione automatica, riuso all'avvio e consumo diretto
verificati per la matrice dichiarata, equivalenza senza mod, costi misurati e
dipendenze residue documentate. Le fasi mod seguenti non bloccano questa consegna.

## ROMDATA-06 — Mod tramite sostituzione delle risorse

**Dipendenze:** completare la consegna principale ROMDATA-01…05.

- [ ] Definire una directory `mods/` nello storage applicativo e un manifest
  versionato con ID, versione, profili/dataset compatibili, dipendenze e risorse
  sostituite. Risolvere percorsi relativi entro il pacchetto, con limiti su
  dimensioni e formati; preservare i file utente durante la rigenerazione base.
- [ ] Definire ordine deterministico e gestione dei conflitti. Validare gli
  override con gli stessi contratti delle risorse base prima di avviare il gioco;
  mostrare errori di dipendenze/versione invece di caricare una combinazione parziale.
- [ ] Aggiungere attivazione/disattivazione persistente e indicazione delle mod
  attive. Per la prima versione applicare i cambi al riavvio della sessione;
  nessuna necessità di ricaricamento a caldo per completare il supporto iniziale.
- [ ] Collegare hash dei contenuti e ordine delle mod a cache e salvataggi;
  isolare gli slot incompatibili e proteggere i `.sav` originali quando la mod
  cambia dati di gioco. Disattivare le mod deve ripristinare le risorse base.
- [ ] Provare mod originali/minime per le famiglie supportate, modifica visibile
  nel gioco, conflitti, file mancanti e ripristino. Distribuire soltanto esempi
  originali o descrizioni/patch, non gli asset estratti dalla ROM dell'utente.

**Chiusura:** mod caricate davvero dal percorso risorse, selezione riproducibile,
base recuperabile, compatibilità ed errori verificati.

## ROMDATA-07 — Modifiche estese a contenuti e logica

**Dipendenze:** ROMDATA-06 e verifica delle strutture interessate.

- [ ] Estendere i formati a testi più lunghi, nuove risorse, mappe, animazioni e
  tabelle di gameplay, affrontando limiti di ID, dimensioni, puntatori e interfacce.
  Distinguere sostituzione di contenuti esistenti e aggiunta di nuovi contenuti.
- [ ] Progettare separatamente script, patch di codice e regole di gioco:
  stabilire necessità di ricompilazione, revisione ABI, invalidazione delle cache
  e disabilitazione degli hook non compatibili. Un override grafico non abilita
  automaticamente modifiche alla logica.
- [ ] Definire strumenti e documentazione per autori, migrazioni di schema e
  verifica dei pacchetti; aggiungere esempi originali e regressioni pertinenti.

**Chiusura:** ogni nuova capacità ha un contratto, esempi e compatibilità
verificata; le modifiche non supportate sono dichiarate.

## Regole di avanzamento

Selezionare una sottofase in [TASK_HANDOFF.md](TASK_HANDOFF.md), registrando
risultati e criteri mancanti per famiglia. Non chiudere il genitore finché i
suoi criteri restano aperti e non riaprire figli conclusi senza nuova evidenza.
Seguire [MEMORY_WORKFLOW.md](MEMORY_WORKFLOW.md) per documentazione, record e
verifiche; mantenere ROM, risorse estratte e prove personali fuori da Git.

**Prossima azione:** ROMDATA-03, preparazione automatica della prima famiglia;
poi consumo diretto e verifica ROMDATA-04/05. Gli altri cataloghi restano aperti.
