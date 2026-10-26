# TODO — Estrazione ROM, dati decodificati e mod

Richiesta aggiornata il 22 ottobre 2026. **FEAT-04-ROM-DATA** è in esecuzione;
inventario e contratto iniziale sono definiti. Le migrazioni runtime restano aperte.
Ambito: workspace desktop, Windows come piattaforma principale di verifica;
Linux come controllo aggiuntivo. `AndroidPort/` richiede lavoro separato.

## Obiettivo e ordine di lavoro

L'obiettivo principale è scompattare la ROM selezionata in file di risorse,
decodificarli in formati utilizzabili e far leggere al runtime quei dati.
Al primo avvio si prepara il dataset; agli avvii successivi si riutilizza la
versione valida. Il supporto alle mod e a modifiche più ampie viene dopo.

Priorità utente aggiornata: per ora **non misurare le regressioni di performance**;
l'ottimizzazione verrà dopo. Non avviare benchmark, profili o campagne di avvio/
RSS, né comprimere o ritoccare formati per rispettare vecchi budget di spazio.
Le misure già raccolte restano storiche; non bloccano il lavoro funzionale.
Priorità regionale aggiornata: **concentrarsi sulla versione europea**;
le nuove verifiche USA restano aperte. Conservare le prove USA già concluse
come evidenza storica, senza ripeterle durante questa fase EUR.
Priorità piattaforma aggiornata: **ridurre i test ed eseguirli solo su Wine**;
segnare i nuovi controlli Linux come da fare. Le verifiche Linux già concluse
restano storiche, senza ripeterle per le nuove integrazioni.
Restano necessarie le prove di equivalenza, incluso il clock guest, l'uso reale
dei file e la compatibilità di native/save/Load.

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

**ROMDATA-01-GRAPHICS-LZ77 completato per i produttori iniziali osservati:**
quattro sorgenti LZ77 per regione verificate su avvii puliti1200frame Linux e
Windows/Wine; confini dei flussi e uscite guest coincidono. Individuati anche
due root regionali di descrittori da88byte. Limiti del catalogo, campi interni,
interpretazione dei pacchetti e lingue restano aperti.
[Contratto e prove](../rom_data_graphics.md).

**ROMDATA-01-BG-DESCRIPTOR-PREFIX completato per il blocco contiguo verificato:**
162record/14256byte, tutti i riferimenti entro i limiti, primo payload distinto
dai record e metadati identici nei due profili. Selettori condizionati verificati
con le routine originali per ogni record, sentinelle e fallback. Riferimenti
relativi, triple di trasferimento, puntatore variante e byte non interpretati
sono distinti. La famiglia grafica01resta parziale per asset/programmi collegati,
altri cataloghi e lingue. [Contratto](../rom_data_graphics.md#authenticated-descriptor-metadata).


**ROMDATA-01-BG-PALETTES completato per il catalogo base dichiarato:** tutte
le162voci puntano a127sorgenti per regione,125LZ77e2grezze. Intervalli, alias e
buffer decodificati coincidono tra EUR/USA. Quattro oracoli originali Linux/Wine
passano1296chiamate/72792istruzioni ciascuno, output/guardie completi e bit15
preservato. I flussi per palette alternative sono classificati separatamente sotto; le
precondizioni dei cinque indici FFrestano da chiarire. Modello tipizzato e consumo delle palette sono completati
separatamente nelle unità02/04sotto.
[Contratto e prove](../rom_data_graphics.md#base-background-palette-catalog).

**ROMDATA-01-BG-ALTERNATE-PALETTE-PROGRAMS completato per gli indici validi:**
due cataloghi da157chiavi0..156per regione,314flussi con header BE e formato
LZ/RLE distinto dal BIOS; ciascuno produce256B. Tabelle, sorgenti e output
coincidono tra EUR/USA; quattro oracoli originali Linux/Wine passano1256chiamate
ciascuno, output/guardie completi ed EWRAM/SP invariati. Cinque record con FF
restano distinti: nessuna dereferenziazione o fallback inventato, precondizioni
dei chiamanti ancora da verificare. Il parser tipizzato02è ora verificato sotto; estrazione gestita e consumo04
sono conclusi nelle unità02/04sotto. Nessuna misura prestazionale.
[Contratto](../rom_data_graphics.md#alternate-background-palette-catalogs).

**ROMDATA-01-BG-ALTERNATE-CALLER-CONTRACT: inventario diretto e modalità EUR
completati; precondizioni globaliFF aperte.** Quattro chiamanti Thumb per regione,
setter originale e relativa conferma individuati. Il menu Options applica LCD A=0,
LCD B=1(secondo catalogo), TV=2(primo catalogo) solo dopo Save changes→Yes.
Snapshot della GUI nativa EUR confermano i valori senza forzare RAM. Nessun
controllo locale del byte56FF nei percorsi ispezionati; il controllo ID00ff è
distinto. GUI mappa/Party e deployment non leggono gli stream alternativi:
le scene pub con uso positivo sono concluse sotto; restano le precondizioniFF.
[Contratto e limiti](../rom_data_graphics.md#alternate-palette-callers-and-color-mode).

**ROMDATA-01-BG-PACKET-CATALOG completato per i campi04/08/10 dichiarati:**
352sorgenti distinte per regione (122paired,117halfword,113auxiliary), involucri
01grezzo/11LZ77con padre opzionale, inizializzazione e dizionari verificati.
Tutti i riferimenti/alias/hash coincidono EUR/USA, niente sovrapposizioni/cicli,
catene al massimo di2nodi. Quattro casi Linux/Wine con i lettori originali
passano486invocazioni ciascuno, buffer completi/guardie/SP. Il decoder ora
espone anche l'estensione letta dei dizionari. Modello tipizzato/file gestito e
consumo effettivo restano02/03/04; campo00e programmi transfer/variant separati.
[Contratto](../rom_data_graphics.md#complete-descriptor-packet-catalogs).

**ROMDATA-01-BG-MAIN-GRAPHICS completato per il catalogo EUR dichiarato:**
162link/50sorgenti disgiunte,23tag20e27tag22,621923byte sorgente e835232uscita.
Produttore a stadi0801b688e codec custom LZ/RLE080054a0verificati; tag22conserva
offset di destinazione e header20annidato. Tutte le50uscite/cursori/guardie/SP
coincidono con l'originale su Wine; i due trasferimenti DMA sono verificati per
un caso20e uno22. Nuovi controlli Linux/USA da fare.
[Contratto](../rom_data_graphics.md#european-main-background-graphics).

**ROMDATA-01-BG-TRANSFER-VARIANT completato per il catalogo EUR dichiarato:**
97triple attive nei cinque slot (83/6/5/3/0),42controlli e34banchi tile grezzi
condivisi,76intervalli disgiunti. Tutti i162puntatori variant sono nulli in
questo blocco; nessun programma variant inventato. Produttori originali EUR
confermano durata, ordine dei frame e DMA con limite dinamico; Linux/USA da fare.
[Contratto](../rom_data_graphics.md#european-background-transfer-programs).

**ROMDATA-01-TEXT-AUXILIARY-ROOTS / ROMDATA-02-FONT-KEYBOARD-DATA completati
per EUR/Wine dichiarato:** inventario16chiamanti diretti e archivio tastiera
nome separato, tre programmi/18larghezze righe/180celle. L'ultimo glifo3174
verifica il termine del banco completo3175glifi/metriche. Due controlli Wine
mirati passano: JSON/sorgente/guardie e3047nuovi glifi originali, tre pagine/
180selettori (3230chiamate); nessuna ripetizione dei128glifi già chiusi.
Unicode/altri puntatori dinamici e nuovi controlli Linux/USA da fare.
[Contratto](../rom_data_text.md#complete-european-font-and-name-keyboard).

**ROMDATA-01-OBJ-STATIC-LISTS / ROMDATA-02-OBJ-STATIC-DATA completati per EUR/Wine
dichiarato:**107riferimenti letterali diretti individuano66liste disgiunte,
160oggetti/1092byte sorgente. Coordinate firmate e tutti i campi attributi
conservati. Due controlli Wine mirati passano: campi/confini/JSON e1056chiamate
alle due routine originali con clipping/flip/wrap/capacità/ritorno anticipato,
interi buffer OAM/matrici/guardie uguali. Altre radici indirette, animazioni e
grafica OBJ restano aperte; Linux/USA da fare. [Contratto](../rom_data_obj.md).

**ROMDATA-01-GAMEPLAY-ITEM-ROOTS: inventario dichiarato completato,
contratto originale ora verificato sotto.** Lettore proprietà080ce1ec/tabella6e0bd0,
376record×32byte/12032byte fino al root indipendente6e3ad0. Chiamanti negozio/
classificatore e enumerazione originale1..361/362..375confermano i limiti;
19proprietà espongono halfword/byte/signed e modificatori live02002030.
Tutti32byte per record conservati, nessuna sovrapposizione e22; semantiche non
provate non inventate. Oracolo/modello/gestione/consumo EUR ora completati sotto;
tabella seguente20byte, altre famiglie e Linux/USA restano aperti.
[Contratto](../rom_data_gameplay.md).

## ROMDATA-02 — Estrattori e decoder

**ROMDATA-01-TEXT-REMAINING-CATALOGS / ROMDATA-02-TEXT-CATALOG-DATA completati
per il blocco lettori EUR dichiarato:**9cataloghi a puntatori/45tabelle/16100
stringhe,29selettori relativi/145archivi/17035programmi. Flag, comandi/operandi,
LZ/RLE custom esatto, padding decodificato e128glifi referenziati conservati;
33581intervalli disgiunti. Due test Wine mirati passano, inclusi inventari
indipendenti completi e80bordi puntatori/145radici/300uscite script/63nuovi
glifi originali (968chiamate). Contesto dinamico valido esplicito nell'oracolo;
nessuna prova scena dedotta. Collegamento gestito/runtime esteso ancora aperto,
e11conserva il sottoinsieme precedente. Linux/USA da fare.
[Contratto e limiti](../rom_data_text.md#broad-pointer-catalogs-and-relative-text-archives).

**ROMDATA-02-TEXT-CATALOG-MANAGED completato per EUR/Wine:** decimo file fisso
text-catalogs.json, revisione1/12/11,33581intervalli e autenticazione/identità
dell'insieme. Pubblicazione/riuso, errore atomico/riparazione, controlli reali
EUR su dati/modifiche coerenti/metadata/percorsi/dipendenze/limiti, move e
attivazione fallita transazionale passano in tre controlli Wine ridotti. CLI
reale prepare/riuso senza scritture/attivazione di un frame con cache vecchia
passano. e11e otto file precedenti invariati, USA resta e8. Consumo esteso04
verified-only e lifecycle/scene05ancora aperti; Linux/USA da fare.

**ROMDATA-01-TEXT-SHORT-STRINGS / ROMDATA-02-TEXT-SHORT-DATA completati
per il catalogo EUR dichiarato:** cinque tabelle originali da107voci,
535stringhe distinte (478compatte/57estese),5468byte sorgente. Modello tipizzato
con codici glifo/byte controllo esatti e soli65glifi packed4bpp/metriche usati;
670intervalli con provenienza/hash. Due test Wine mirati passano: codec/guardie
e tutte le routine originali di selezione/copia/glifi/larghezze,1331chiamate/
89446istruzioni. JSON rigoroso e modifiche coerenti rifiutate. Pubblicazione
gestita/startup/consumo runtime restano aperti, schema e10invariato. Altri
cataloghi, font completo/Unicode/comandi, Linux/USA da fare.
[Contratto e prove](../rom_data_text.md).

**ROMDATA-02-TEXT-SHORT-MANAGED completato per EUR/Wine:** nono file fisso
short-texts.json, revisione1/11/10e670intervalli con provenienza. Autenticazione
dell'insieme completo/identità, pubblicazione atomica, riuso/riparazione/move e
attivazione fallita transazionale passano in tre test Wine mirati. CLI reale:
prepare/riuso senza scritture/attivazione di un frame con cache nativa conservata
passano. e10e sette file dati precedenti invariati; USA resta e8. Il nuovo
catalogo è verified-only; consumo04e scene/lifecycle05aperti, Linux/USA da fare.

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
limiti in [risorse decodificate](../rom_data.md). Il consumo diretto della prima
famiglia è ora collegato nel percorso opt-in04.

**ROMDATA-02-LZ77-DECODER completato come componente offline:** input e memoria
limitati, riferimenti sovrapposti, consumo esatto senza padding e rifiuto di
input malformati. Quattro test sintetici per target e confronto con le uscite
guest già catturate passano su Linux/Wine. Non aggiunge ancora una famiglia al
manifest: servono prima i contratti semantici del catalogo grafico.

**ROMDATA-02-BG-PACKETS completato come componente offline:** parser tipizzati
dei pacchetti a16bit e delle coppie indicizzate, con limiti di destinazione,
scritture e numero di pacchetti. Tre test sintetici per target e confronti delle
routine originali EUR/USA passano su Linux/Wine:7invocazioni/87160istruzioni
per caso, tutta l'EWRAM confrontata e continuazione a blocchi verificata.
Il ramo con coppie letterali conserva la sintassi, con guardia sulle destinazioni
dopo una prova originale contraria all'ipotesi iniziale.
[Contratto, prove e limiti](../rom_data_graphics.md#typed-background-packets).
Catalogo e integrazione delle nuove risorse restano aperti.

**ROMDATA-02-BG-DESCRIPTOR-METADATA completato come componente offline:** decoder
tipizzato, controlli sui limiti, autenticazione della sorgente, roundtrip esatto
e TOML deterministico. Tre test sintetici per target e quattro casi EUR/USA su
Linux/Wine passano:2612chiamate originali/134629istruzioni per caso. I file di
prova da106309byte coincidono sui quattro percorsi. Integrazione nel manifest,
preparazione e consumo runtime sono stati selezionati separatamente come segue.

**ROMDATA-02-BG-MANAGED completato per estrazione/verifica/preparazione:**
nuovo file descrittori nel manifest, schema1/estrattore3/decoder2 e directory
v1/e3-d2, con versioni precedenti conservate. Limiti16KiB manifest/OBJ,
256KiB BG e272KiB totali; pubblicazione atomica di tre file, riuso completo,
recupero, identità dell'intero dataset e attivazione autenticata. Passano25test
Linux/23Wine,18controlli dei veri CLI per target e scenari privacy tar/zip.
[Prove e limiti](../rom_data_graphics.md#managed-descriptor-extraction-and-preparation).
Questa unità prova la preparazione; il successivo consumo diretto04è descritto sotto.


**ROMDATA-02-BG-BASE-PALETTE-DATA completato:** modello di banchi da16valori
u16, bit15conservato, parser limitato raw0/LZ7710e intervalli esatti senza
padding. Tutte le162voci sono estratte in JSON leggibile; manifest con tutte
le provenienze, quattro file e revisione1/5/4, vecchie generazioni conservate.
Passano39test Linux/37Wine e22controlli CLI per target, inclusi oracoli originali,
integrità/riparazione/rilocazione e compatibilità delle risorse già consumate.
[Contratto e limiti](../rom_data_graphics.md#typed-palette-extraction-and-preparation).
Questa unità copre la preparazione; il successivo consumo04è completato sotto
con revisione1/6/5e metadati di compatibilità. Nessun lavoro di compressione/spazio.

**ROMDATA-02-BG-ALTERNATE-PALETTE-CODEC completato per il catalogo verificato:**
codec custom limitato, header BE, tutti i token e controlli originali03..0F,
consumo esatto e rifiuto di troncamenti, riferimenti fuori storia e overrun.
Modello con banchiu16completi, metadati sorgente/link e cinqueFFconservati.
Linux/Wine:4test core,314buffer per regione uguali agli oracoli conservati,
70flussi sintetici/1120chiamate originali per caso regionale/target e prova
negativa dell'overrun originale. JSON leggibile deterministico sui quattro
percorsi; nessuna misura prestazionale. Estrazione/preparazione gestita e
consumo dei file alternativi sono selezionati separatamente sotto.
[Contratto e prove](../rom_data_graphics.md#typed-alternate-palette-codec).

**ROMDATA-02-BG-ALTERNATE-PALETTE-MANAGED completato:** quinto file JSON
leggibile con tutti i banchi/chiavi/link e316intervalli di provenienza; schema1/
estrattore7/decoder6, vecchie generazionie6conservate. Scrittura/verifica atomica
dell'insieme, riuso senza scritture, riparazione e attivazione autenticata con
identità completa. Linux23default+4privati e Wine21+4passano;23verifiche dei
veri CLI per target, inclusi EUR/USA, recupero e rilocazione. Nessuna misura
prestazionale o riduzione di spazio. L'attivazione degli alternativi è ancora
`verified-only`al termine di questa unità; il consumo04è completato sotto.
[Prove e limiti](../rom_data_graphics.md#managed-alternate-palette-datasets).

**ROMDATA-02-BG-PACKET-DATA completato per EUR; verifica USA rinviata:**
modello offline con352programmi unici/486link, scritture ordinate tipizzate,
padri/alias e metadati originali wrapper/body. Limiti, cicli, ID, sovrapposizioni,
incoerenze e modifiche base coerenti sono controllati. Otto test core, un test
JSON e un caso EUR con tutti i buffer conservati passano su Linux/Wine; JSON
leggibile identico. Nessun nuovo replay originale, schema attivo o consumer
runtime. Il sesto file gestito è concluso sotto; rimane il consumo04. [Contratto e prove](../rom_data_graphics.md#typed-descriptor-packet-programs).

**ROMDATA-02-BG-PACKET-MANAGED completato per EUR/Wine:** sesto file fisso
`bg-packet-programs.json`, manifest con352intervalli e dipendenza BG, revisione
1/8/7con generazioni e7conservate. Guardie input64MiBper file packet/set,
parse/verifica dell'intero dataset prima di pubblicazione/attivazione. Quattro
casi mirati Wine e sette comandi Windows reali passano; canonici/originali
protetti. Stato runtime verified-only al termine della pubblicazione; il
consumoEUR04è ora concluso sotto.
Nuovi controlli Linux/USA da fare, nessuna campagna performance o suite ampia.
[Contratto e prove](../rom_data_graphics.md#managed-packet-program-datasets).

**ROMDATA-02-BG-GRAPHICS-DATA completato per EUR/Wine:** modello tipizzato
offline con word little-endian completi, offset, sorgente esatta, alias e ID
canonici; parser limitato con autenticazione della ROM. Codec condiviso con
palette alternative mantenendo le loro guardie256/4096byte. Nove test Wine
mirati passano, inclusi JSON leggibile/roundtrip e54chiamate originali a stadi.
Nessuna nuova revisione manifest o migrazione runtime; estrazione gestita e
consumo restano da collegare. Linux/USA da fare, nessuna misura prestazionale.
[Contratto e prove](../rom_data_graphics.md#european-main-background-graphics).

**ROMDATA-02-BG-GRAPHICS-MANAGED completato per EUR/Wine:** settimo file fisso
`bg-main-graphics.json`, revisione1/9/8e50intervalli autenticati; USA conserva
sei file/e8-d7senza nuove verifiche. Pubblicazione atomica, riuso senza scritture,
riparazione, identità completa e generazioni e8precedenti conservate. Quattro
test Wine mirati e sette veri comandi Windows CLI passano, inclusa attivazione
con cache nativa precedente. Runtime della nuova grafica verified-only a quel
confine; nessuna prova d'uso dedotta dal primo frame. Consumo04completato sotto. Linux/USA da
fare, nessun benchmark o riduzione di spazio.
[Contratto e prove](../rom_data_graphics.md#managed-european-main-graphics).

**ROMDATA-02-BG-TRANSFER-DATA completato offline per EUR/Wine:** controlli
con duratau16/offset tile32byte, dimensione trasferimento e byte3conservato;
tile come wordu32completi, alias/ID indipendenti e destinazioni tipizzate.
Cinque controlli mirati Wine passano (3core/1JSON/1oracolo privato): quattro
animazioni originali, due cicli completi, VRAM/guardie dopo ogni copia e34chunk
DMA parziali;634chiamate/80766istruzioni. Provenienza completa, roundtrip e
modifiche coerenti rifiutate. Nessun nuovo manifest o consumer a quel confine:
file gestito completato sotto, consumo04successivo. Linux/USA da fare, nessuna misura prestazionale.
[Contratto e prove](../rom_data_graphics.md#european-background-transfer-programs).

**ROMDATA-02-BG-TRANSFER-MANAGED completato per EUR/Wine:** ottavo file fisso
`bg-transfer-programs.json`, revisione1/10/9,76intervalli autenticati e identità
completa dell'insieme. Quattro test Wine mirati e tre veri comandi CLI Windows
passano preparazione/riuso/riparazione/rilocazione/attivazione transazionale e
cache nativa precedente. Generazioni e9e sei file decodificati precedenti
conservati; USA resta con sei filee8-d7. Runtime transfer verified-only a quel
confine, nessuna prova d'uso dedotta dal primo frame; consumo04concluso sotto.
Linux/USA da fare, nessun benchmark.
[Contratto e prove](../rom_data_graphics.md#managed-european-transfer-programs).

**ROMDATA-02-FONT-KEYBOARD-MANAGED / ROMDATA-04-FONT-KEYBOARD-CONSUMPTION
completati per EUR/Wine dichiarato:** due nuovi file fissi font-bank.json e
name-keyboard.json, dataset1/13/12da12file/11risorse; e12e precedenti file
conservati, USA resta e8-d7. Autenticazione delle sette provenienze e dell'intero
insieme; owner tipizzato immutabile per glifi/metriche/indici/programmi/righe.
Tre controlli Wine mirati passano pubblicazione/errore atomico/riuso/riparazione/
spostamento, rifiuto modifiche e attivazione transazionale; cinque chiamate
originali/native con tutte sette sorgenti alterate e stati/clock uguali;
Load/Undo/backup/riavvio con copie temporanee indisponibili. Nessun cambio ABI
save/native o istruzione guest saltata. Linux/USA da fare.

**ROMDATA-05-FONT-KEYBOARD-SCENE-CLI completato per una transizione EUR/Wine:**
397frame pubCyril con stati/clock/immagini/OAM/dispatch/audio uguali e8064byte
glifi/93metriche dal banco completo. CLI reale prepara/riusa e13senza scritture
e vecchia cache, stesso risultato legacy conservato. La scena non usa la
tastiera: scena nome ordinaria/GUI/archivi restano da fare. Nessuna misura
prestazionale o ottimizzazione spazio. [Prove](../rom_data_text.md#complete-european-font-and-name-keyboard).

**ROMDATA-02-OBJ-STATIC-MANAGED / ROMDATA-04-OBJ-STATIC-CONSUMPTION completati
per EUR/Wine dichiarato:** file fisso obj-static-lists.json,1/14/13con13file/
12risorse e66provenienze; e13e file precedenti conservati, USA resta e8-d7.
Tre controlli Wine mirati passano pubblicazione/riuso/riparazione/spostamento,
rifiuto modifiche e attivazione transazionale, otto chiamate originali/native
con tutte66sorgenti alterate e stati/clock uguali, bordi/mirror/SWI/DMA,
otto casi host draw/pubblicazione e Load/Undo/backup/riavvio. Nessun cambio ABI
o istruzione guest saltata. Un controllo aggiuntivo Wine passa792casi host draw/visibilità/battaglia e
modifiche vive di liste RAM/ROM fuori catalogo; nuovi Linux/USA da fare.

**ROMDATA-05-OBJ-STATIC-SCENE-CLI completato per una transizione EUR/Wine:**
397frame pub con ogni stato/clock/immagine/OAM/dispatch/audio uguale e5470byte
OBJ tipizzati/958letture host. Tre processi CLI reali da cwd estranea preparano/
riusano senza scritture e riproducono con cache vecchia, risultato legacy
conservato uguale. Nessuna misura prestazionale/spazio. GUI/archivi più ampi e
altri cataloghi OBJ restano aperti. [Prove](../rom_data_obj.md#verification).

**ROMDATA-01-OBJ-UI-ANIMATION-ROOTS / ROMDATA-02-OBJ-UI-ANIMATION-DATA completati
per EUR/Wine dichiarato:**30coppie letterali individuano10banchi OBJ/sequenze,
167liste/284oggetti/95sequenze/184frame e282intervalli. Indici relativi, campi
OBJ e durateu16conservati; contatori restano RAM. Due controlli Wine passano
JSON/sorgente/guardie,115costruttori originali e2292tick originali con task,
loop/ultimo frame, OAM/matrici/guardie uguali. Tre sequenze vuote conservate,
domini diretti ora classificati sotto ROMDATA-01-OBJ-EMPTY-CALLERS;
scheduling indiretto/live non verificato. Le sette ulteriori liste dirette
sono ora migrate sotto e16; altri chiamanti indiretti restano aperti. Linux/USA da fare.

**ROMDATA-02-OBJ-ANIMATION-MANAGED / ROMDATA-04-OBJ-ANIMATION-CONSUMPTION
completati per EUR/Wine dichiarato:** obj-animation-banks.json,1/15/14con14file/
13risorse e282provenienze; e14e tutti i file precedenti conservati. Tre prove
Wine mirate passano autenticazione/pubblicazione/riuso/riparazione/spostamento,
49chiamate costruttori/body originali/native con sorgenti alterate e stati/clock
uguali, tutti sei tipi letti da codice compilato, bordi/mirror/SWI/DMA, dieci
banchi host e Load/Undo/backup/riavvio senza file. ABI e lavoro guest invariati.
La breve scena battaglia240frame coincide interamente ma usa0byte di questi
banchi: il gate positivo la rifiuta. Una scena mappa+Start160frame ora passa
ogni stato/clock/immagine/OAM/dispatch/audio con36513byte tipizzati/3200letture
host; tre CLI reali preparano/riusano senza scritture e riproducono con cache
vecchia. Menu finale ispezionato. Linux/USA da fare. [Contratto e prove](../rom_data_obj.md#indexed-ui-animation-banks).

**ROMDATA-02-OBJ-EXTRA-DIRECT-LISTS / ROMDATA-04-OBJ-EXTRA-CONSUMPTION /
ROMDATA-05-OBJ-EXTRA-SCENE-CLI completati per EUR/Wine dichiarato:** sedici
riferimenti aggiuntivi/sette nuove liste portano il catalogo statico a 123
riferimenti, 73 liste, 173 oggetti e 1184 byte sorgente. e16-d15 conserva e15,
tutti i 66 record precedenti e gli altri file decodificati; restano 14 file/
13 risorse. Tre controlli Wine mirati passano autenticazione/pubblicazione/
riuso/riparazione/spostamento, 14 chiamate originali/interprete/native con le
sette sorgenti alterate e stati/clock uguali, bordi/mirror/SWI/DMA, 14 casi host
e Load/Undo/backup/riavvio senza file. Flag guest/host per sorgente impediscono
che liste precedenti soddisfino il gate nuovo. Battaglia/Party/menu personaggio
sono osservazioni a copertura negativa conservate; la selezione lingua EUR
all'avvio usa tre nuove liste. Un solo confronto di 240 frame passa ogni stato/
clock/immagine/OAM/dispatch/audio; CLI reale prepara/riusa senza scritture e
riproduce con cache vecchia da cwd estranea, immagine finale ispezionata.
Nessun test chiuso ripetuto. Altre scene/GUI/archivi, nuovi Linux/USA e radici
switch/indirette da fare. [Prove](../rom_data_obj.md#additional-direct-lists-in-e16).

**ROMDATA-01-OBJ-ZERO-COUNT-PAIR-CONTRACT / ROMDATA-02-OBJ-ZERO-COUNT-DATA /
ROMDATA-04-OBJ-ZERO-COUNT-CONSUMPTION / ROMDATA-05-OBJ-ZERO-COUNT-SCENE-CLI
completati per EUR/Wine nonrepeat dichiarato:** undicesimo banco b4d8e8/b4f4e8,
una lista/due oggetti/cinque conteggi zero. Selettori espliciti e riferimenti
alla durata conservano gli header zero e riusano proprietari tipizzati degli
header/indici successivi. Otto intervalli nuovi/58 byte; e17-d16 conserva i
dieci record precedenti, gli altri file ed e16, senza cambiare ABI. Un controllo
Wine originale passa 14 costruttori/104 tick e indirizzi letti/stati task/OAM/
matrici/guardie. Due prove Wine mirate passano autenticazione/alias/modifiche
coerenti/pubblicazione/riuso/riparazione/spostamento/Load/Undo/backup/riavvio,
otto confronti interprete/native con sorgenti e durata condivisa alterate e
stati/clock uguali, bordi/mirror/SWI/DMA e host. Una scena Party di 240 frame
passa ogni stato/clock/immagine/OAM/dispatch/audio con 288 byte selettori e
288 letture host nuovi; CLI reale prepara/riusa senza scritture e riproduce
con cache precedente, immagine ispezionata. Domini diretti delle tre sequenze
vuote precedenti ora classificati sotto ROMDATA-01-OBJ-EMPTY-CALLERS;
repeat1/precondizioni diverse e altre radici restano aperti. Linux/USA da fare.
[Contratto](../rom_data_obj.md#zero-count-hold-bank-in-e17).

**ROMDATA-02-OBJ-LOCALIZED-UI-BANKS / ROMDATA-04-OBJ-LOCALIZED-CONSUMPTION /
ROMDATA-05-OBJ-LOCALIZED-SCENE-CLI completati per EUR/Wine dichiarato:** cinque
banchi selezionati dalla lingua tramite due tabelle di puntatori; 149 liste/
574 oggetti/144 frame e 166 intervalli/4984 byte aggiuntivi. e18-d17 conserva
11 banchi precedenti, gli altri file e le generazioni e17. Il catalogo ha ora
16 coppie/317 liste/105 sequenze/328 frame, sempre 14 file/13 risorse. Originali
selezioni e durata u16 256 con contatore u8 conservati. Un controllo Wine passa
15 selezioni/10 costruttori/1589 tick e stati task/OAM/matrici/guardie; due prove
mirate passano autenticazione/riuso/riparazione/spostamento/Load/Undo/backup/
riavvio, 30 confronti originali/interprete/native con 166 sorgenti alterate,
nuovi bordi/mirror/SWI/DMA e cinque pubblicazioni host. La battaglia conservata
e i primi tentativi direzionali mappa hanno copertura zero, conservati senza
confronto doppio o ripetizione. La scena reale “Engage!” da mappa usa il normale
puntamento mouse a Lutia Pass e A: 400 frame con ogni stato/clock/immagine/OAM/
dispatch/audio uguale, 8 byte tabella/5906 byte banco/228 letture host nuovi.
CLI reale prepara/riusa senza scritture e usa cache vecchia da cwd estranea;
immagine finale ispezionata. Nuovi Linux/USA da fare. Grafica OBJ compressa
collegata resta aperta; domini diretti dei tre vecchi record vuoti ora
classificati sotto ROMDATA-01-OBJ-EMPTY-CALLERS, ingressi indiretti/live non
verificati.
[Prove](../rom_data_obj.md#language-selected-ui-banks-in-e18).

**ROMDATA-01-OBJ-GRAPHICS-PAYLOADS / ROMDATA-02-OBJ-CUSTOM-GRAPHICS /
ROMDATA-04-OBJ-CUSTOM-CONSUMPTION / ROMDATA-05-OBJ-CUSTOM-SCENE-CLI completati per il catalogo EUR/Wine dichiarato:**
39banchi custom e sei tabelle da cinque puntatori,45intervalli/62674byte sorgente/
131872byte decodificati; tredici riferimenti diretti e sei percorsi indicizzati
canonici. EUR e19-d18aggiunge obj-graphics.json (15file/14risorse), conserva
tutti i13file decodificati precedenti e le generazioni e18. Letterali/puntatori
tipizzati alimentano il decoder/copie originali; scratch, trasformazioni,
trasferimenti parziali e coda RAM viva restano attivi. Wine verifica39decoder/
30selettori/6copie originali, pubblicazione/riuso/recupero/spostamento/Load/Undo/
backup/restart e39decoder/30selettori/4copie interprete/native con sorgenti
alterate e stati/clock uguali. Una correzione indirizzo fixture DMA è verificata
separatamente con20byte tipizzati positivi. Una scena Engage400frame passa
stati/immagini/OAM/dispatch/audio con1194byte sorgente/460letterali/4selettore.
CLI Windows reale crea/riusa senza scritture/esegue da cwd estranea con cache
nativa precedente; eseguibile canonico e sorgenti originali invariati.
Altri payload/palette OBJ, scheduling indiretto/live dei tre record vuoti e
Linux/USA restano aperti; domini diretti ora classificati sotto
ROMDATA-01-OBJ-EMPTY-CALLERS;
nessuna misura prestazionale. [Contratto e prove](../rom_data_obj.md#custom-obj-word-banks-in-e19).

**ROMDATA-01-OBJ-PALETTE-SOURCES / ROMDATA-02-UI-RAW-PALETTE-MANAGED /
ROMDATA-04-UI-RAW-PALETTE-CONSUMPTION / ROMDATA-05-UI-RAW-PALETTE-SCENE-CLI
completati per il catalogo EUR/Wine dichiarato:**21banchi/23riferimenti diretti,
derivati o Color Mode,1328u16/2656byte senza sovrapposizioni, per entrambe le
metà BG/OBJ del buffer palette. EUR e20-d19aggiunge ui-palettes.json,
16file/15risorse; tutti i14file decodificati precedenti/e19restano invariati.
Wine verifica l’inizializzatore originale176byte,27copie/10casi Color Mode,
JSON/fonti/bit15, pubblicazione/riuso/recupero/spostamento/Load/Undo/backup/
restart e parità completa interprete/native con tutte le21fonti alterate e
3528byte compilati positivi. Bus/DMA160byte e pubblicazione di cambi RAM vivi
passano. Avvio ordinario240frame/CLI reale coincide per stati/clock/immagini/
OAM/dispatch/audio con33letture/130byte palette. Altre fonti indirette e
Linux/USA/Windows nativo restano aperti; nessuna misura prestazionale.
[Contratto e prove](../rom_data_ui_palettes.md).

**ROMDATA-01-RETURN-LENGTH-CUSTOM-GRAPHICS / ROMDATA-02-UI-CUSTOM-GRAPHICS /
ROMDATA-04-UI-CUSTOM-CONSUMPTION / ROMDATA-05-UI-CUSTOM-SCENE-CLI completati
per il catalogo EUR/Wine dichiarato:** nove banchi custom(ottoBG/unOBJ),
dieci percorsi originali literal/decoder/upload,57038byte sorgente/84160decodificati.
I conteggi derivati da r0e la copia parziale544/2944byte restano istruzioni
originali. EUR e21-d20aggiunge ui-graphics.json(17file/16risorse), conserva
tutti i15file decodificati precedenti/e20. Wine passa sorgenti/JSON/metadati,
pubblicazione/riuso/recupero/spostamento/Load/Undo/backup/restart e tutti i dieci
percorsi interprete/native con nove intervalli alterati e stati/clock uguali;
68906byte compilati/39861letterali e tutti i banchi positivi. Bus/mirrors/BIOS/
CpuSet/FastSet/DMA32byte passano. Primo locator Party→Unit320frame negativo
conservato; scena Clan320frame positiva con1642byte sorgente/764letterali,
stati/clock/immagini/OAM/dispatch/audio uguali e CLI reale create/no-write
reuse/vecchia cache. Altre sorgenti/varianti indirette restano aperte, Linux/USA
da fare, nessuna misura prestazionale. [Contratto](../rom_data_ui_graphics.md).

**ROMDATA-01-UI-LANGUAGE-GRAPHICS / ROMDATA-02-UI-LOCALIZED-GRAPHICS /
ROMDATA-04-UI-LOCALIZED-CONSUMPTION / ROMDATA-05-UI-LOCALIZED-SCENE-CLI
completati per l'ambito EUR/Wine dichiarato:** sette banchi nuovi/due selettori
per cinque lingue/due link a proprietari OBJ esistenti.17nuovi intervalli/
14436byte(14396payload+40puntatori)/35008decodificati; totale16banchi/
26intervalli/71474byte sorgente/119168decodificati. EUR e22-d21mantiene17file/
16risorse, nove record UI/tutti15altri file decodificati/e21immutati. Lingua live,
jump table di codice, fallback inglese vs salto e copia5056OBJ/2688BG restano
originali. Tre controlli Wine mirati passano14percorsi selettori/sette nuovi
produttori/due sole code BG condivise, autenticazione/proprietari/transazioni/
pubblicazione/riuso/riparazione/spostamento/Load/Undo/backup/restart e native con
17intervalli alterati/stati-clock uguali;48byte selettori compilati/14310nuove
sorgenti/6449letterali/tutti7banchi positivi, bus/BIOS/SWI/DMA4byte. Corrette solo
fixture nuove fallite: modulo duplicato e chain nativa oltre gli stop dei soli
selettori; evidenza conservata, nessuna modifica runtime per quei problemi.
CLI create/no-write reuse passa; Clan320frame è negativo per i nuovi dati e
non viene confrontato di nuovo. Avvio normale1200frame con input esistente usa
4byte selettore/2347nuova sorgente b7d9e4/1124letterali: una coppia completa
stati/clock/immagini/OAM/dispatch/audio passa, immagine finale ispezionata.
Altre scene linguistiche ordinarie/sorgenti indirette restano aperte, Linux/USA/
Windows nativo da fare; nessuna misura prestazionale/spazio.
[Contratto](../rom_data_ui_graphics.md#localized-ui-selectors-in-e22).

**ROMDATA-02-GAMEPLAY-ITEM-SOURCE-ORACLE / ROMDATA-02-GAMEPLAY-ITEM-DATA /
ROMDATA-04-GAMEPLAY-ITEM-CONSUMPTION completati per EUR/Wine dichiarato:**
376record/32byte/12032sorgente, items.json/resource gameplay.item-records,
EUR e23-d22/18file/17risorse, tutti16file decodificati precedenti/e22immutati.
Campi halfword/byte/signed/tail conservano ogni bit; proprietà/bonus live/
inventario/clamp unsigned e tabella secondaria restano originali. Un oracolo
Wine passa14371chiamate su tutti i record/19proprietà/due rami bonus, speciali/
troncamenti/invalidi e240casi signed negativi isolati; i campi ROM distribuiti
sono tutti non negativi. Prima assunzione errata della fixture corretta prima
dell'esecuzione, log fallito conservato. Due integrazioni Wine passano gestione/
recupero/retention/transazioni/provenienza/Load/Undo/backup/restart e86chiamate
native con12032byte raw alterati/stati-clock uguali;100byte compilati(24prefix/
40signed/36altri). Tutti byte/edges/mirrors/BIOS/SWI/DMA4byte passano.
ROMDATA-05-GAMEPLAY-ITEM-SCENE-CLI è completo per una scena Unit320frame:
CLI create/no-write reuse/vecchia cache e una coppia completa stati/clock/
immagini/OAM/dispatch/audio passano con40nuovi byte signed; immagine finale
ispezionata. Prefix/tail ordinari non rivendicati; altri cataloghi e Linux/USA/
Windows nativo restano aperti, nessuna misura prestazionale/spazio.
[Contratto](../rom_data_gameplay.md).

**ROMDATA-01-GAMEPLAY-ITEM-SECONDARY-ROOTS / ROMDATA-02-GAMEPLAY-ITEM-LINK-SOURCE-ORACLE
completati per il contratto EUR/Wine dichiarato:**6e3ad0..6e4c64,
225record×20byte/4500byte, count0..9/byte opaco/nove coppie,472attive/ottoFF.
Dodici literal LDR e root seguente indipendente confermano i limiti;376oggetti
riferiscono220righe, tutte225conservate. Catalogo tipizzato lossless/JSON/source/
owner/extent/coherent edit passa Wine:378selettori/1394ricerche/464prefissi normali/
8prefissiFF,4query sintetiche zero/duplicati/unused e8casi separati dal confronto
precedente al ramoFF.2244casi/61906istruzioni principali+80del gate, nessuna
misura prestazionale. Unit bit7/byte3drestano live; chiamate job/abilità successive
non rivendicate. Il contratto precede la successiva integrazione e24riportata sotto;
nuove verifiche Linux/USA/Windows nativo restano da fare.
[Contratto](../rom_data_gameplay.md#item-linked-pair-source-contract).

**ROMDATA-02-GAMEPLAY-ITEM-LINK-DATA / ROMDATA-04-GAMEPLAY-ITEM-LINK-CONSUMPTION
completati per EUR/Wine dichiarato:** e24-d23aggiunge item-links.json/resource
 gameplay.item-links con dipendenza esplicita dai376oggetti parsed;19file/
18risorse, tutti17file decodificati precedenti/e23immutati.225record/4500byte
count/selector/value/opaque alimentano i lettori CPU/DMA/native; query, unit
bit7/FFbyte3d, chiamate successive, clock/save/native ABI restano originali.
Due controlli Wine passano pubblicazione/failure/riuso/riparazione/spostamento/
retention/transazioni/lifecycle e62percorsi nativi con sorgente alterata/stati-
clock uguali:36byte count/114selector/56value, otto forme count e gateFFreali.
Opaque non letto da questi percorsi ma verificato con tutti i4500byte bus;
edges/mirrors/BIOS/SWI/DMA4byte e crossing2byte vecchi+2nuovi con entrambi i
raw alterati passano. Nessun vecchio oracolo ripetuto.
ROMDATA-05-GAMEPLAY-ITEM-LINK-SCENE-CLI è completo per scena EUR/Wine dichiarata:
CLI create/no-write reuse/vecchia cache passa; locator Equip slot400frame legge0
nuovi byte, conservato senza coppia/ripetizione. Down/Aordinari aprono la lista
copricapi(immagine finale ispezionata); CLI560frame e una coppia completa stati/
clock/immagini/OAM/dispatch/audio passano con574byte nuovi(287count/205selector/
82value). I nomi privati weapon-list sono locator, non prova di una scena armi.
Linux/USA/Windows nativo e resto TODO aperti, niente
misure prestazionali/spazio. [Contratto](../rom_data_gameplay.md#managed-and-direct-item-link-consumption).

**ROMDATA-01-GAMEPLAY-METADATA16-ROOTS / ROMDATA-02-GAMEPLAY-METADATA16-SOURCE-ORACLE
completati per il contratto EUR/Wine dichiarato:**128record×16byte/2048byte,
6e4c64..6e5464,23literal LDR e root successivo indipendente. Modello lossless
u16alias+14byte/JSON/source/extent e inventario indipendente passano. Un solo
controllo Wine passa2581casi originali display/alias nonzero/lista live/somme/
flag/scelta e19sintetici per priorità1assente nei dati/somma u8/shift. Stato LCG
originale e guardie RAM/stack/callee verificati; nessun nome legge/missione/
abilità inventato, alias0non rivendicato.156038istruzioni/319PC sono copertura,
nessuna misura prestazionale. EUR gestito resta e24-d23/19file/18risorse;
preparazione, consumo diretto/lifecycle/scena/CLI di questa nuova tabella aperti.
Linux/USA/Windows nativo da fare. [Contratto](../rom_data_gameplay.md#sixteen-byte-linked-metadata-source-contract).

**ROMDATA-02-GAMEPLAY-METADATA16-DATA / ROMDATA-04-GAMEPLAY-METADATA16-CONSUMPTION
completati per EUR/Wine dichiarato:** e25-d24aggiunge linked-metadata16.json/
resource gameplay.linked-metadata16,20file/19risorse; tutti18file decodificati
precedenti/e24conservati, USAe8-d7. Campi alias/display/flag/value/opaque tipizzati
alimentano CPU/DMA/native mantenendo lista live/RNG/somme/shift/clock/save/native
ABI originali. Due controlli Wine passano pubblicazione/failure/riuso/riparazione/
spostamento/retention/transazioni/lifecycle e42percorsi nativi con2048byte raw
alterati/stati-clock uguali:8byte alias/17display/22flag/24value compilati.
Opaque verificato al bus con tutti i byte/edges/mirrors/BIOS/SWI/DMA4byte e
crossing2byte vecchi link+2nuovi metadata entrambi alterati. Oracolo sorgente
2581+19non ripetuto. ROMDATA-05-GAMEPLAY-METADATA16-SCENE-CLI è completo:
CLI crea/riusa e controllo22file hash/mtime senza scritture passa; locator
Area List/Start/Party/inventario normale leggono0nuovi byte e non sono ripetuti
né confrontati a coppia. Clan→Mission Item520frame legge6byte display nuovi,
immagine Chocobo Egg ispezionata; una coppia stati/clock/immagini/OAM/dispatch/
audio passa con gli stessi6byte. Alias/flag/value ordinari non rivendicati.
Errore iniziale header input corretto prima dell'esecuzione; solo il replay
corretto e poi il criterio no-write prima non raggiunto sono stati eseguiti.
Nuovi Linux/USA/Windows nativo e resto TODO aperti; nessuna misura prestazionale/spazio.
[Contratto](../rom_data_gameplay.md#managed-and-direct-metadata16-consumption).

**ROMDATA-01-GAMEPLAY-METADATA52-ROOTS / ROMDATA-02-GAMEPLAY-METADATA52-SOURCE-ORACLE
completati per il contratto EUR/Wine dichiarato:** 116 record da 52 byte /
6032 byte, 6e5464..6e6bf4, otto literal LDR, enumeratore 0..115 e root
successivo indipendente. Modello lossless con route esplicita e tre array fissi;
48 proprietà unsigned/bitfield mantengono rinvii FF all'indice del chiamante,
rinvii nonzero e cicli originali, senza fallback inventati. Un controllo Wine
passa 6369 chiamate reali / 4 probe non ritornanti limitati e 294 chiamate
sintetiche high-bit/rinvii / 2 probe ciclici; lettori tail32/33 da indice live e
guardie JSON/source/extent/RAM/stack/callee passano. I 464 selettori normali
precedenti rientrano nel nuovo owner; otto FF restano distinti. Nessun vecchio
oracolo ripetuto, 223421 istruzioni / 524 PC sono solo copertura. Dataset
rimane e25-d24 / 20 file / 19 risorse: preparazione, consumo diretto/lifecycle/
scena/CLI di questa tabella e nuovi Linux/USA/Windows nativo restano aperti.
[Contratto](../rom_data_gameplay.md#fifty-two-byte-linked-metadata-source-contract).

**ROMDATA-02-GAMEPLAY-METADATA52-DATA / ROMDATA-04-GAMEPLAY-METADATA52-CONSUMPTION
completati per EUR/Wine dichiarato:** e26-d25 aggiunge linked-metadata52.json /
resource gameplay.linked-metadata52, 21 file / 20 risorse; tutti i 19 file
decodificati precedenti e le generazioni e25 conservati, USA e8-d7. Owner parsed
dei selettori normali validato entro 116, FF distinto. Letture tipizzate header/
routing/packed/tail/opaque mantengono getter, indici alternativi, rinvii, record
live, clock e ABI save/native originali. Due controlli Wine passano pubblicazione/
failure/riuso/riparazione/spostamento/retention/transazioni/lifecycle e 75 percorsi
nativi con tutti i 6032 byte raw alterati e stati-clock uguali: 18 byte header /
83 routing / 68 packed / 8 tail compilati. Opaque al bus con tutti i byte/edges/
mirrors/BIOS/SWI/DMA4 e crossing 2 byte metadata16 + 2 metadata52 entrambi
alterati. Oracolo sorgente/cicli non ripetuto.
ROMDATA-05-GAMEPLAY-METADATA52-SCENE-CLI completo: CLI crea/riusa senza cambiare
hash/mtime, usa la vecchia cache da cwd diverso e apre Change Jobs con input
ordinario. Una coppia 540 frame stati/clock/immagini/OAM/dispatch/audio passa con
664 byte nuovi (74 header / 335 routing / 255 packed), immagine finale Marsh/
Thief ispezionata. Tail/opaque ordinari non rivendicati. Nessun locator fallito,
harness/test fallito o controllo precedente ripetuto. Nuovi Linux/USA/Windows
nativo e resto TODO aperti; nessuna misura prestazionale/spazio.
[Contratto](../rom_data_gameplay.md#managed-and-direct-metadata52-consumption).

**ROMDATA-01-GAMEPLAY-METADATA4-ROOTS / ROMDATA-02-GAMEPLAY-METADATA4-SOURCE-ORACLE
completati per i prefissi EUR/Wine dichiarati:** 20 record da quattro byte /
80 byte, 6e6bf4..6e6c44, due coppie selector/value, 26 attive. Due literal e
root successivo indipendente, indici cross-owner parsed con metadata52 passano.
Un controllo Wine passa 320 casi reali / 24 sintetici Count/Select/Gate/Threshold
per entrambe le versioni originali: tutti i record, job chiamanti 2..43, soglie
sotto/uguale/sopra, buchi, payload unused, value255, confronto signed e tally u8.
Conteggio di entrambi gli slot e cammino fino al primo zero restano distinti;
scansioni live delle abilità e intere funzioni non rivendicate o congelate.
9560 istruzioni / 136 PC sono copertura, nessun vecchio oracolo ripetuto.
Dataset rimane e26-d25 / 21 file / 20 risorse: preparazione, consumo diretto,
lifecycle, scena/CLI e nuovi Linux/USA/Windows nativo restano aperti.
[Contratto](../rom_data_gameplay.md#four-byte-two-pair-source-contract).

**ROMDATA-02-GAMEPLAY-METADATA4-DATA / ROMDATA-04-GAMEPLAY-METADATA4-CONSUMPTION
completati per EUR/Wine dichiarato:** e27-d26 aggiunge linked-metadata4.json /
resource gameplay.linked-metadata4, 22 file / 21 risorse; tutti i 20 file
decodificati precedenti/e26 conservati, USA e8-d7. Owner metadata52 parsed e
indici/selettori validati; byte tipizzati mantengono conteggio/cammino contiguo,
scansioni live, valori unsigned/confronto signed/tally u8 e clock/ABI originali.
Due controlli Wine passano pubblicazione/failure/riuso/riparazione/spostamento/
retention/transazioni/lifecycle e 56 prefissi nativi con 80 byte raw alterati e
stati-clock uguali: 54 byte selector / 34 value compilati, tutti i byte/edges/
mirrors/BIOS/SWI/DMA4 e crossing 2 byte metadata52 + 2 metadata4 passano.
ROMDATA-05-GAMEPLAY-METADATA4-SCENE-CLI completo: CLI crea/riusa senza cambiare
hash/mtime, vecchia cache/cwd diverso; una coppia 540 frame Change Jobs stati/
clock/immagini/OAM/dispatch/audio passa con 32 byte selector / 6 value nuovi,
immagine Marsh/Thief ispezionata. Input conservato giustificato dalla nuova
famiglia/nuovo gate; e26 non riaperto. Oracolo sorgente320+24 non ripetuto,
prefissi non rivendicati come intere funzioni. Nuovi Linux/USA/Windows nativo e
resto TODO aperti; nessuna misura prestazionale/spazio.
[Contratto](../rom_data_gameplay.md#managed-and-direct-metadata4-consumption).

**ROMDATA-01-GAMEPLAY-METADATA2-MAP-ROOTS / ROMDATA-02-GAMEPLAY-METADATA2-SOURCE-ORACLE
completati per EUR/Wine dichiarato:** 44 coppie key/value / 88 byte a
6e6c44..6e6c9c, ultimo record43 reale e successiva sorgente48B indipendente.
Forward originale tronca u8 e legge indice0..43 senza bound locale; inverse
tronca u8, cerca solo0..42 e restituisce primo stored key / zero se assente.
Owner metadata52 parsed/JSON/source/extent e 348 casi reali + 13 sintetici
Wine passano, inclusi alias, high byte, chiavi non uguali all'indice e match
solo nell'ultimo record escluso inversamente. Wrapper live byte7 provato fino
al ritorno del getter con frame mantenuto; intere funzioni, bound globali e
semantica dei valori non rivendicati. Dataset e27-d26/22file/21risorse invariato;
preparazione/consumo/lifecycle/scena-CLI e nuovi Linux/USA/Windows nativo da fare.
Nessuna misura prestazionale/spazio, nessun vecchio oracolo ripetuto.
[Contratto](../rom_data_gameplay.md#two-byte-forward-and-inverse-source-contract).

**ROMDATA-02-GAMEPLAY-METADATA2-DATA / ROMDATA-04-GAMEPLAY-METADATA2-CONSUMPTION
completati per EUR/Wine dichiarato:** e28-d27 aggiunge linked-metadata2.json /
resource gameplay.linked-metadata2,23file/22risorse, tutti i21file precedenti/e27
conservati; USAe8-d7. Owner metadata52parsed/byte key-value autenticati conservano
istruzioni originali forward44/inverse43, alias/first storedkey/miss0/u8 e live
byte7/clock/save/native ABI. Due controlli Wine passano pubblicazione/transazioni/
riuso/riparazione/spostamento/retention/lifecycle e21percorsi originali/nativi,
88byte raw alterati/stati-clock uguali,7key/235value compilati, tutti byte/bordi/
mirrors/BIOS/SWI/DMA4/crossing2metadata4+2metadata2. Build corretta warning-free
(dopo tre argomenti zero mancanti nei vecchi test budget); sorgente348+13 chiusa.
ROMDATA-05-GAMEPLAY-METADATA2-SCENE-CLI completo: CLI prepara/riusa senza scritture
con cache vecchia/cwd diverso. ChangeJobs540/Rumors560/prefissi e Missions620/
1260sono locator negativi conservati; conferme ordinarie aggiuntive danno una
coppia Pub1840stati/clock/immagini/OAM/dispatch/audio uguali e4value/0keyBnuovi.
Inverse key provato solo nei test mirati, immagine finale Pub con pannelli nativi
separati non prova unità/outcome missione. Nessun vecchio oracolo/full pair sui
negativi, nessuna misura performance/spazio. Nuovi Linux/USA/Windows nativo e
resto TODO aperti. [Contratto](../rom_data_gameplay.md#managed-and-direct-metadata2-consumption).

**ROMDATA-01-GAMEPLAY-DEFAULT48-TEMPLATE / ROMDATA-02-GAMEPLAY-DEFAULT48-SOURCE-ORACLE
completati per EUR/Wine dichiarato:**48byte strutturali6e6c9c..6e6ccc, root e
lunghezza copia originali. Costruttore mode1/2, quattro ID20..23con sorgenti
alternate, patch signedID/selector>1/argomenti/ORflag distinti. Un controllo Wine
passa271casi reali+30sintetici con inizializzatore IWRAM originale, tutti256ID,
mode/troncamento/bordi signed/selector/flag e preservazione bit/byte inutilizzati.
48Broundtrip/JSON/array/source/stack/args/callee-invalid/outerRAM passano; nessun
oracolo vecchio ripetuto,48812istruzioni/155PCsolo copertura. Allocazione/liveunit
dopo127EB0esclusi; alternate192B/puntatore ROM e successivi240Bnon ancora owned.
Dataset e28-d27/23file/22risorse invariato; preparazione/consumo/lifecycle/scenaCLI
e nuovi Linux/USA/Windows nativo da fare, nessuna misura performance/spazio.
[Contratto](../rom_data_gameplay.md#default-constructor-template-source-contract).

**ROMDATA-02-GAMEPLAY-DEFAULT48-DATA / ROMDATA-04-GAMEPLAY-DEFAULT48-CONSUMPTION
completati per EUR/Wine dichiarato:**e29-d28aggiunge default-constructor-template.json /
resource gameplay.default-constructor-template,24file/23risorse,22file precedenti/
e28conservati; USAe8-d7. Byte header/argomenti/halfword/flag/tail conservano copia
originale ROM/IWRAM, patch live, mode/alternative/allocazione/clock/save/native ABI.
Due controlli Wine passano pubblicazione/transazioni/lifecycle/retention e16prefissi
originali/interprete/nativi con48rawBalterati/stati-clock uguali;8rami alternativi/
rifiutati0newreads,39/21/272/16/32Bcompilati, tutti byte/bordi/BIOS/mirror/SWI/DMA4/
crossing2metadata2+2default48. Build corretta warning-free dopo macrocontatore
rimasta a2elementi anziché5; log iniziale conservato. Sorgente271+30non ripetuta.
ROMDATA-05-GAMEPLAY-DEFAULT48-SCENE-CLI completo: CLI crea/riusa senza scritture
con vecchia cache/cwd diverso; una coppia397Pub stati/clock/immagini/OAM/dispatch/
audio passa con96Bnuovi=10header/6argomenti/68halfword/4flag/8tail, immagine
ispezionata. Nuova famiglia/gate giustifica input conservato; nessun locator
negativo, RAMedit/recording nuovo o vecchio oracolo ripetuto. Tail allocazione,
alternate192B/puntatore ROM e successivi240Bnon ancora owned. Nuovi Linux/USA/
Windows nativo/restoTODOaperti; nessuna misura performance/spazio.
[Contratto](../rom_data_gameplay.md#managed-and-direct-default-constructor-template).

**ROMDATA-01-GAMEPLAY-CONSTRUCTOR-ALTERNATES completato staticamente:**4record
48B/192Ba6eca78..6ecb38+puntatore4Ba70cd58,196Btotali/noe29overlap. RootLDR,
LDR+4, IDu8gated20..23/stride48/copia48 e roundtrip indipendente passano.
Consumatore CCF9C..CD046legge cinque equipment halfword08..10perrecord attraverso
item proprietà6/3;20linkentro376item parsed. Byte01e header vicini non ricevono
semantiche non provate. Modello/oracolo sorgente/preparazione/consumo/scena restano
da fare; EUR e29-d28invariato, prove default271+30/native16/397chiuse.
[Contratto](../rom_data_gameplay.md#next-rooted-constructor-sources).

**ROMDATA-02-GAMEPLAY-CONSTRUCTOR-ALTERNATES-SOURCE-ORACLE completato per EUR/Wine:**
modello strict4record/192B+puntatore4B/196B, ID20..23/offset/campi byte/halfword e
cinque equipment halfword perrecord con owner376item parsed. Un controllo Wine
passa24casi reali+8sintetici di copia/patch originali, troncamento u8, selector/
flag/mode, highbit/unused/highitem/ORpreservati e JSON/source/array/order/ID/extent/
stack/args/outerRAM.5560istruzioni/137PCsolo copertura; tail allocazione esclusa,
synthetic non autenticati/non pubblicati. Default271+30/native16/397non ripetuti.
Dataset e29-d28/24file/23risorse invariato; preparazione/consumo/lifecycle/scenaCLI
e nuovi Linux/USA/Windows nativo da fare, nessuna misura performance/spazio.
[Contratto](../rom_data_gameplay.md#typed-constructor-alternates-source-proof).

**ROMDATA-02-GAMEPLAY-CONSTRUCTOR-ALTERNATES-DATA / ROMDATA-04-GAMEPLAY-CONSTRUCTOR-ALTERNATES-CONSUMPTION
completati per EUR/Wine dichiarato:**e30-d29aggiunge constructor-alternate-templates.json /
resource gameplay.constructor-alternate-templates,25file/24risorse,23file precedenti/
e29conservati; USAe8-d7. Owner Items parsed, puntatore4B/record192Btipizzati e mask
perrecord conservano copia ROM/IWRAM/patch live/mode/allocazione/clock/save/native ABI.
Due controlli Wine passano pubblicazione/transazioni/lifecycle/retention e14percorsi
con196rawBalterati/stati-clock uguali, tutti4record mask15/puntatore compilati,
5default/rifiutati0newB/vecchio default positivo. Tutti byte/bordi record/BIOS/mirror/
SWI/DMA4per entrambe le sorgenti/crossing2typed+2raw passano. Build warning-free;
guardia zero-use corretta in review prima di Wine, nessun failure. Source24+8e
vecchie prove default271+30/native16/397chiuse.
ROMDATA-05-GAMEPLAY-CONSTRUCTOR-ALTERNATES-SCENE-CLI completo: CLI crea/riusa senza
scritture/vecchia cache/cwd diverso. Intro3000cortile scuola e Lutia1700NOTICE sono
locator negativi conservati, nessuna coppia completa. Conferme/schieramento normali
danno una coppia3600Lutia stati/clock/immagini/OAM/dispatch/audio uguali e45nuoviB:
4pointer/6header/3argomenti/28halfword/2flag/2tail, mask1soloID20. Tutti4record e
copia completa restano prove mirate; immagine finale ispezionata, nessun RAMedit/
recording nuovo/salvataggio persistito/vecchio oracolo ripetuto. Nuovi Linux/USA/
Windows nativo/header vicini/successivi240B/restoTODOaperti, nessuna misura performance/
spazio. [Contratto](../rom_data_gameplay.md#managed-and-direct-constructor-alternates).

**ROMDATA-01-GAMEPLAY-METADATA14-ROOTS completato staticamente nel catalogo dichiarato:**
93record/1302Bfisici6e6f6c..6e7482,IDlogici2..94/base biased6e6f50. Predicate>1
nei getter mantiene0/1sul ramo job live; root successivo indipendente6e7484usa
stride12e lascia2Bnonowned. Getter halfword4/6,byte11/12/13 e palette packed8..10
individuati; roundtrip/noe30overlap/186valori01/02entro116metadata52passano. Byte02
copia al job liveconoverride0; owner semantico byte01non provato solo dal range.
Ipotesi6e6dbc14Brespinta(reader*4), presunti alias7484/78d4respinti(stride12).
Modello/oracolo sorgente/consumo da fare,e30invariato; nessun build/test/ripetizione
vecchia accettazione, nuovi Linux/USA/Windows nativo da fare.
[Contratto](../rom_data_gameplay.md#next-bounded-fourteen-byte-records).

**ROMDATA-02-GAMEPLAY-METADATA14-SOURCE-ORACLE completato per EUR/Wine dichiarato:**
modello strict93record/1302B,ID2..94/basebiased6e6f50/fisici6e6f6c..6e7482,
tutti byte/halfword/packed, owner metadata52parsed per byte02solo; byte01non
vincolato semanticamente dal solo range. Un controllo Wine passa504casi reali:
471getter/tutti93ID/troncamento u8,22palette con book RAM0/1/2/default3/copia32B/
guardie512B e11prefissi inizializzazioneCCF9C→CD0B6/override/flag/frame264Bunit.
13sintetici highu16/u8/bank15/unused/byte01unproved/altojob valido passano insieme
JSON/source/array/bias/ID/order/count/owner/callee/stack/descriptor/unit/palette/
outerRAM.23627istruzioni/380PCsolo copertura, nessun vecchio oracolo ripetuto.
Allocazione e bound globali esclusi;0/1/95plus/2Bpadding nonowned, nessun fallback.
Dataset e30-d29/25file/24risorse invariato; preparazione/consumo/scenaCLI/lifecycle
e nuoviLinux/USA/Windows nativo da fare, nessuna misura performance/spazio.
[Contratto](../rom_data_gameplay.md#typed-fourteen-byte-records-source-proof).

**ROMDATA-02-GAMEPLAY-METADATA14-DATA / ROMDATA-04-GAMEPLAY-METADATA14-CONSUMPTION /
ROMDATA-05-GAMEPLAY-METADATA14-SCENE-CLI completati per EUR/Wine dichiarato:**
e31-d30 aggiunge linked-metadata14.json/gameplay.linked-metadata14, 26 file /
25 risorse; tutti i 24 file precedenti/e30 conservati, USAe8-d7. Owner metadata52
parsed solo per byte02; byte01 senza vincolo semantico. Proiezione tipizzata di
1302B, contatori e maschere dei 93 record preservano getter originali, book RAM,
copie palette, override, clock e ABI native/save. Due controlli Wine passano
pubblicazione/riuso/riparazione/spostamento/retention/guardie transazionali e
Load/Undo/backup/restart con copia dataset indisponibile. 37 percorsi originali/
interprete/native con raw1302B alterati hanno stati-clock identici; ID2/20/28/94
positivi, normali0/1 senza nuove letture. Bus/edge/width/mirror/BIOS/SWI/DMA e
confini typed/raw passano. CLI reale crea/riusa senza scritture e usa la vecchia
cache da cwd estraneo; una sola coppia397Pub completa stato/clock/immagini/OAM/
dispatch/audio con27B/due record/tutti i gruppi di campi, immagine ispezionata.
Oracolo sorgente504+13 e prove precedenti non ripetuti. Bound globali, payload
palette, code di allocazione, famiglie adiacenti e nuoviLinux/USA/Windows nativo
da fare; nessuna misura performance/spazio o modificaAndroidPort.
[Contratto](../rom_data_gameplay.md#managed-and-direct-fourteen-byte-special-unit-metadata).

**ROMDATA-01-GAMEPLAY-METADATA12-ROOTS / ROMDATA-02-GAMEPLAY-METADATA12-SOURCE-ORACLE
completati per EUR/Wine sorgente dichiarato:** due banchi con stride12 a
6e7484..6e78d4/92record e6e78d4..6e7994/16record, totale108record/1296B/5184coppie
di bit. Root/callee135a1c/sorgente successiva indipendente6e7994 verificati;
modello strict gruppi12×4 valori0..3 conserva48coppie anche se i loop usano0..43.
Indici row32 e slot signed/wrapping originali, mode troncato u8, nessun bound o
fallback guest nuovo. Un controllo Wine passa1640reali/10raw-fuori-wrap/384
sintetici, indirizzo/LDRB esatti, JSON/source/callee/stack/RAM. Build iniziale con
errori setup test corretto, rebuild Windows senza avvisi; nessun errore Wine.
EUR e31-d30 invariato; preparazione/consumo/lifecycle/scena, significato campi,
bound globali e nuoviLinux/USA/Windows nativo da fare. Nessuna misura performance/
spazio o prova precedente ripetuta.
[Contratto](../rom_data_gameplay.md#indexed-twelve-byte-packed-pair-source-contract).

**ROMDATA-02-GAMEPLAY-METADATA12-DATA / ROMDATA-04-GAMEPLAY-METADATA12-CONSUMPTION /
ROMDATA-05-GAMEPLAY-METADATA12-SCENE-CLI completati per EUR/Wine dichiarato:**
e32-d31 aggiunge linked-metadata12.json/gameplay.linked-metadata12,27file/26risorse,
25file precedenti/e31 conservati; USAe8-d7. Campi a due bit tipizzati, due owner
sorgente, contatori di banco/maschere108record mantengono row32/slot signed-wrap/
modeu8/clock/ABI originali senza dipendenze semantiche inventate. Due controlli
Wine passano pubblicazione/riuso/riparazione/spostamento/retention/guardie
transazionali/Load-Undo-backup-restart con copia dataset indisponibile.
49percorsi originali/interprete/native con1296rawB alterati hanno stati-clock
identici; entrambi i banchi positivi, tre percorsi esterni0nuove letture.
Bus/edge/width/mirror/BIOS/SWI/DMA e confini raw/typed/tra banchi passano.
Una coppia2200-Echo Grass confirmation stato/clock/immagini/OAM/dispatch/audio/CLI positiva
completa il perimetro ordinario dichiarato. Oracolo sorgente1640+10+384/e31 non
ripetuti. Bound globali/semantica flag/banco globale separato/altre famiglie e
nuoviLinux/USA/Windows nativo da fare; nessuna misura performance/spazio.
[Contratto](../rom_data_gameplay.md#managed-and-direct-twelve-byte-packed-pair-banks).

**ROMDATA-01-GAMEPLAY-GLOBAL-PAIR-FLAGS / ROMDATA-02-GAMEPLAY-GLOBAL-PAIR-FLAGS-SOURCE-ORACLE
completati per EUR/Wine sorgente dichiarato:** root singolo6e7994,44coppie/11B
fino6e799f da loop RAM0..43/mode0; nessuno stride12,3Bprima root indipendente
6e79a2 nonowned. Modello strict11×4 valori0..3 conserva entrambi i bit. Un controllo
Wine passa102reali/10raw-fuori-wrap/91sintetici, wrapper conr0ignorato/slot signed-
wrap/modeu8/filtro maschera live/uscita anticipata/scansione44/JSON/source/callee/
stack/RAM.13601istruzioni/70PC solo copertura; nessun build/Winefailure/prova
precedente ripetuta. EUR e32-d31 invariato; preparazione/consumo/lifecycle/scena,
highmode ordinario, semantica/padding e nuoviLinux/USA/Windows nativo da fare.
[Contratto](../rom_data_gameplay.md#global-packed-pair-flags-source-proof).

**ROMDATA-02-GAMEPLAY-GLOBAL-PAIR-FLAGS-DATA / ROMDATA-04-GAMEPLAY-GLOBAL-PAIR-FLAGS-CONSUMPTION
completati per EUR/Wine; ROMDATA-05CLI concluso, uso ordinario globale da fare:**
e33-d32 aggiunge global-pair-flags.json/gameplay.global-pair-flags,28file/27risorse,
26file precedenti/e32conservati, USAe8-d7.11Btipizzati/maschera posizioni byte
mantengono ignoredr0/slotwrap/modeu8/liveRAM/uscita anticipata/clock/ABI senza
owner/padding inventati. Due controlli Wine passano pubblicazione/retention/
guardie transazionali/Load-Undo-backup-restart;28percorsi native/interprete con
raw11Balterati hanno stati-clock identici,17Bcompiled/8esterni0nuoviB.
Tutti11B/bus/mirror/BIOS/confine2vecchi2nuovi/SWI11typed/DMA3typed1raw e1typed3raw
passano. CLI crea/riusa senza scritture, vecchia cache/cwd estraneo; unica scena
2200EchoGrass negativa(0globali/44metadata12), nessuna coppia completa su di essa.
Mancano uso ordinario globale positivo/relativa coppia completa; reader131db8
richiede descriptorhword4 e maschera live nonzero, servono condizioni concrete
prima di altri replay. Highmode/newLinux/USA/Windows nativo/3Badiacenti da fare.
Oracoli102+10+91/e32 non ripetuti; nessuna misura performance/spazio.
[Contratto](../rom_data_gameplay.md#managed-and-direct-global-packed-pair-flags).

**ROMDATA-01-GAMEPLAY-NUMERIC-PAIR-ROOTS / ROMDATA-02-GAMEPLAY-EVALUATION-PARAMETERS-SOURCE-ORACLE
completati per EUR/Wine sorgente:**38B in due sorgenti:7coppie divisore/peso e2
bonus condizionali a6e79a2..6e79b4;5classi soglia1..5con stride4 a6e79c6..6e79da.
Classe0saltaRNG/lettura,14Btra sorgenti e4Brow0nonowned; ipotesi32/24B respinta.
Modello strict7/2/5array/ID/offset/campi/opaque e un controllo Wine passano132casi
sorgente originale (12prefissi score live, frammenti aritmetici/bonus/RNG/suffix
con argomenti iniettati) e52sorgenti mutate. Prima divisioneunsigned/u8,6signed32,
wrap, bonuslow8==1/==0, LCG originale/sogliacomparison signedlow16%256 preservati.
Guardie JSON/source/ID/order/offset/callee/stack/unit/context passano; nessun
build/Winefailure/prova precedente ripetuta.24147istruzioni/442PC solo copertura.
Zerodivisor solo diagnosticNone/guest invariato, non eseguito qui; wholefunction
nonclaimed. e33invariato; preparazione/consumo/lifecycle/scena/semantica e nuovi
Linux/USA/Windows nativo da fare, nessuna misura performance/spazio.
[Contratto](../rom_data_gameplay.md#evaluation-parameters-source-proof).

**ROMDATA-02-GAMEPLAY-EVALUATION-PARAMETERS-DATA / ROMDATA-04-GAMEPLAY-EVALUATION-PARAMETERS-CONSUMPTION /
ROMDATA-05-GAMEPLAY-EVALUATION-PARAMETERS-SCENE-CLI completati per EUR/Wine score dichiarato:**
e34-d33 aggiunge evaluation-parameters.json/gameplay.evaluation-parameters,
29file/28risorse/27file precedenti ed e33conservati, USAe8-d7.38Btipizzati e
contatori classi/campi preservano produttori live/divisioni unsigned-u8 e signed/
wrap/bonus/RNG/clock/ABI. Due controlli Wine passano preparazione/retention/
guardie transazionali/Load-Undo-backup-restart e27prefissi native/interprete con
raw38Balterati/stati-clock identici; classmask31,classe0zero,opaque0compiled.
Bus/edge/mirror/BIOS/SWI18-20typed/DMA2typed2raw/18Bgap-row0raw passano.
Build iniziale con avviso import inutilizzato rifiutato dal wrapper; campioni
native sovrapposti corretti prima di Wine, rebuild senza avvisi, nessun errore
Wine. CLI crea/riusa senza scritture/vecchia cache/cwd estraneo; primo1840Pubmission
positivo15scoreB(7divisori/7pesi/1bonus), una sola coppia completa stato/clock/
immagini/OAM/dispatch/audio passa. La successiva400firma/fee chiudeclass1; classi2..5restano targeted-only,
fonte132+52/provee28/e33 non ripetute. Wholefunction/semantica/gap/global11Bordinary/
nuoviLinux/USA/Windows nativo da fare; nessuna misura performance/spazio.
[Contratto](../rom_data_gameplay.md#managed-and-direct-evaluation-parameters).

**ROMDATA-01-GAMEPLAY-METADATA10-ROOTS completato nel perimetro statico dichiarato:**
root6e79da/stride10/default0 e producer020021b4in04B3E0..04B416 fissano0..99,
100record/1000Bfino6e7dc2. Candidato101respinto;12Bzero prima root6e7dcenonowned,
01A5A6display non è prova setter. Otto u8delta sono passati al consumer04870c
con stato live/carry/wrap/cap99/flag; byte08usato in prezzo Items con optional+10u8,
divisione signed/minimounsigned/zero→1, byte09structuralunused. Roundtrip/noe34overlap/
root/stride/field opcodes passano. Modello e sourceoracle completati sotto.
Preparazione/consumo/scena/nomi campi/newLinux-USA da fare.
[Contratto](../rom_data_gameplay.md#next-ten-byte-progression-and-price-records).

**ROMDATA-02-GAMEPLAY-PROGRESSION-REWARDS-SOURCE-ORACLE completato per EUR/Wine:**
modello strict100record/1000Bcon8delta/discount08/opaque09, senza vincoli basati
sui soli valori osservati. Build13783senza avvisi; unico Wine62460passa362casi
reali (7publisher/232caller+helper04870c/123prezzi completi) e68sintetici.
Items parsed autenticati, mask/predicato originali, overflowu8/carry/cap99/
oldinvalid/flag, signeddivisione/minimounsigned/zero, JSON/roundtrip/ID/ordine/
sourceoffset/SP/callee/EWRAM passano. Fuori100/255 resta raw; sintetici non
autenticati/pubblicati;37file protetti invariati. Fonte chiusa, preparazione/
consumo/scena/nuoviLinux-USA-Windowsnativo restano; nessuna misura prestazionale.

**ROMDATA-02-GAMEPLAY-PROGRESSION-REWARDS-DATA / ROMDATA-04-GAMEPLAY-PROGRESSION-REWARDS-CONSUMPTION completati per EUR/Wine:**
e35-d34 aggiunge progression-rewards.json/gameplay.progression-rewards,
30file/29risorse, tutti28decoded precedenti/e34invariati, USAe8-d7. Build84154
senza avvisi/Wine80590due controlli mirati passano pubblicazione/riuso/repair/
move/coherent edits/transactional/load-undo-backup-restart.27percorsi originali/
interprete/native con1000Braw alterati hanno stati-clock identici,17delta/
4discount/0opaque compiled,6percorsi publisher/ungated/outsidezero. Tutti1000B/
100record/boundary/width/mirror/BIOS/SWI32typed/DMA2typed2raw/crossing2eval2new
passano. Fonte362+68chiusa/non ripetuta; CLI/scena completati sotto, newLinux-USA-
Windowsnativo da fare.
[Contratto](../rom_data_gameplay.md#managed-and-direct-progression-rewards).

**ROMDATA-01-GAMEPLAY-PROGRESSION-GATES-ROOTS completato nel perimetro statico:**
0487F6→048868/stride12/index0..63fissano64record/768B6e7dce..6e80ce,
nextrootD23E6→D24A8indipendente allo stessoend. Byte00unused, halfword01return,
byte03output,8u8minimi contro020021b6+2*k; originale masku64/ordine prima
riga unmarked idonea/bitfisico/zero-return con bit63successo distinti dal miss.
Tutti768Broundtrip/noe35overlap/root/field/stride/count/caller/next opcodes
passano; ritorni attuali0..375compatibili Items, receipt/FK completo nonprovato.
Il solo inventario non aggiungeva modello/build/Wine; sourceoracle completato sotto,
managed/runtime/newLinux-USA-Windowsnativo restano.
[Contratto](../rom_data_gameplay.md#next-progression-gates-and-returned-receipts).

**ROMDATA-02-GAMEPLAY-PROGRESSION-GATES-SOURCE-ORACLE completato per EUR/Wine:**
modello strict64record/768Bbyte00/halfword01/byte03/8minimi, nessun vincolo
byte00==ID/minimo<=50/receipt==item. Outcome distingue ID idoneo/valore/output/
maschera. Build75099senza avvisi/unicoWine11775passa80casi reali+25sintetici,
originale0487c8completo+helperu64: tutti64row/equal-below8dimensioni/unsigned128/
bit31-32-63/ordine primo idoneo/fullmask/row63successo valore0;704Bcampi letti,
768Broundtrip/JSON/ID-offset-order/coherentedit/SP/callee/EWRAM passano.
38file protetti invariati, e35unchanged; managed/runtime/lifecycle/ordinaryuse/
receiptUI-FK/newLinux-USA-Windowsnativo da fare. Nessun vecchio test ripetuto.
[Contratto](../rom_data_gameplay.md#progression-gates-source-contract).

**ROMDATA-02-GAMEPLAY-PROGRESSION-GATES-DATA / ROMDATA-04-GAMEPLAY-PROGRESSION-GATES-CONSUMPTION completati per EUR/Wine:**
e36-d35aggiunge progression-gates.json/gameplay.progression-gates,31file/
30risorse/all29decoded precedenti/e35retained/USAe8-d7. Windows68535senzaavvisi/
Wine97635duecontrolli passano pubblicazione/interruzione/create-reuse-repair-move/
retention/coherent requirement-return-output-unused-lastrow-reorder-source-
dependency-path-role/transactionalownerstate/load-undo-backup-restart.19percorsi
originale/interprete/native con768rawBalterati/stato-clock identici; compiled369
requirements/18return/9output/0unused,2successivalore0marcanobit,2fullmaskzero.
All768B/64record/boundary/width/mirror/BIOS/twopages/SWIcrosspage32typed/DMA2typed2raw/
12Brawgap/oldreward2newgate4coexistence passano. Fonte80+25chiusa/nonripetuta.
CLI/scena05completati sotto; receiptUI-FK/newLinux-USA-Windowsnativo restano,
nessunamisuraperformance.
[Contratto](../rom_data_gameplay.md#managed-and-direct-progression-gates).

## ROMDATA-03 — Preparazione automatica e riuso all'avvio


**Dipendenze:** ROMDATA-02 per la prima famiglia.

- [x] Integrare la preparazione dopo selezione/verifica della ROM e prima
  dell'avvio del gioco: estrarre quando il dataset manca o la versione cambia;
  riutilizzarlo quando identità e contenuti sono validi.
- [x] Definire i percorsi attraverso lo storage installed/portable esistente.
  Proposta da confermare: `Data/resources/<rom-hash>/<schema>/` in portable;
  rispettare [le regole di rilocazione](../portable.md). Tenere separati dati
  base rigenerabili, future mod utente, cache native e salvataggi.
- [x] Scrivere in una directory temporanea, validare e pubblicare soltanto un
  dataset completo; gestire annullamento, spazio insufficiente, avvio concorrente
  e crash senza sostituire l'ultima versione valida con un risultato parziale.
- [x] Mostrare avanzamento ed errori comprensibili; prevedere la rigenerazione
  di dati mancanti/corrotti dalla ROM sorgente senza sovrascrivere mod o save.
- [x] Verificare primo avvio, riavvio senza riestrazione, cambio ROM/regione,
  aggiornamento decoder e spostamento della cartella portable. Definire come
  riconoscere una sorgente cambiata prima di riutilizzare il suo dataset.

**Chiusura:** un primo avvio prepara le risorse; quelli successivi riusano un
dataset valido. L'estrazione resta locale e non modifica la ROM sorgente.

**ROMDATA-03-OBJ-STARTUP completato nel percorso opt-in della prima famiglia:**
`rom-data prepare` e `play --rom-data`, generazioni immutabili, selezione atomica,
lock OS, riuso/verifica e recupero. Le voci sopra si riferiscono alla famiglia
`graphics.obj-dimensions`; le altre famiglie non sono implicitamente migrate.
Test Linux e Windows/Wine, compresi arresti forzati e due avvii GUI prima/dopo
lo spostamento portable, passano. Esaurimento spazio simulato; power-loss fisico
e Windows nativo non verificati. Dettagli in [risorse decodificate](../rom_data.md).
Il consumo diretto è ora collegato in04; l'abilitazione normale attende i gate05.

## ROMDATA-04 — Uso diretto dei dati estratti nel runtime

**Dipendenze:** ROMDATA-01/02/03. **Cuore dell'obiettivo principale.**

- [x] Introdurre un accesso alle risorse tramite ID e strutture tipizzate,
  con caricamento/precaricamento dai file decodificati. Definire budget di memoria,
  durata dei dati e invalidazione; evitare I/O nei callback audio e nei percorsi
  per pixel o istruzione.
- [ ] Per ogni famiglia seguire tutti i consumatori: codice nativo tradotto,
  interprete, letture CPU/DMA e percorsi host come rendering e audio. Individuare
  dove il gioco richiede byte grezzi, decomprime, copia o interpreta le risorse.
- [x] Collegare la prima famiglia al caricatore e sostituire il consumo della
  corrispondente risorsa ROM con quello dei dati estratti. Se occorre trasferire
  dati in RAM/VRAM guest, preservarli con layout, registri ed effetti osservabili
  corretti; non applicare lookup generici a ogni lettura del bus.
- [ ] Quando si sostituiscono routine guest, preservare cicli, IRQ, DMA, RNG e
  ordine degli eventi. Dimostrare come si conserva il tempo guest se una
  decompressione viene anticipata all'estrazione: saltarla senza contabilizzarne
  gli effetti non è una sostituzione equivalente.
- [x] Distinguere identità della ROM base, dataset e codice compilato. Rivedere
  costanti incorporate, hook e cache native: nessuna libreria deve riutilizzare
  dati o ipotesi incompatibili con il dataset effettivamente caricato.
- [x] Aggiungere diagnostica di provenienza e contatori di uso/fallback per
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

**ROMDATA-04-OBJ-CONSUMPTION completato per implementazione e prove iniziali:**
la prima famiglia viene letta dai file in `play/frames/runc --rom-data`;
proiezioni fisse per CPU/DMA/API native, letture host tipizzate, RAM modificata
preservata, contatori e risorse non serializzate mantenute in Load/Undo/backup.
Budget misurato432B; nessuna istruzione o decompressione saltata. Corretto anche
il lettore host EUR, verificando la sorgente ROM contro le istruzioni originali.
L'estrattore2 registra l'ulteriore alias EUR e mantiene le vecchie generazioni.
Dettagli e prove in [consumo runtime](../rom_data_runtime.md).
Le voci spuntate coprono l'infrastruttura e la prima famiglia, non le altre.
La matrice di routine originali copre48casi EUR/USA con letture guest effettive;
il confronto nativo600frame prova uso host e parità di stati/immagini/audio,
senza attribuire letture guest non eseguite in quella schermata iniziale.
La nuova tranche05 verifica scene/transizione EUR e costi locali descritti sotto;
il trace USA è ora verificato separatamente come descritto sotto. Gli altri
gate e le altre famiglie restano aperti.

**ROMDATA-04-BG-CONSUMPTION completato per metadati e prove iniziali:**
162descrittori autenticati conservati come dati tipizzati immutabili; proiezioni
CPU/DMA/API native e contatori BG separati. Schema/estrattore/decoder1/3/2,
identità ROM/save/native e ABI invariati. Misurati440Bcondivisi/OBJ+28672BBG,
entro i limiti1KiB/64KiB dichiarati prima dei test. Linux/Wine204core e98/97
frontend passano; quattro casi regionali da2608chiamate confrontano originale,
interprete e TinyCC anche con ROM sorgente in memoria alterata dopo attivazione.
3867letture BG native per caso; Load/Undo/backup e file mancante verificati.
Due avvii EUR nativi1200frame coincidono per tutti gli stati/immagini/OAM/audio,
con53letture BG effettive. [Prove e limiti](../rom_data_graphics.md#direct-descriptor-consumption).
Scene/GUI/costi05BG, asset/programmi collegati e altre famiglie restano aperti.

**ROMDATA-04-BG-BASE-PALETTE-CONSUMPTION completato per il catalogo base:**
162palette conservate nel proprietario immutabile; banchi decodificati per HLE
LZ77e rappresentazione sorgente esatta per CPU/DMA/API native/BIOS guest.
Revisione1/6/5, e5conservata; scritture, clock e identità ROM/save/native invariati.
Linux/Wine210core e101/100frontend passano. Quattro casi originali/native da
1296chiamate/72792istruzioni coincidono anche alterando tutti i vecchi intervalli
ROM dopo attivazione;1272chiamate LZ77leggono i banchi per caso. Load/Undo/
backup/file rimossi passano. Nuovi avvii nativi EUR1200frame coincidono per
stati/immagini/OAM/audio con una palette160B effettivamente letta dai file;
CLI e5refusal/retention e coppie interprete/native passano. Proprietario123833B
su entrambi i target; nessuna ottimizzazione di spazio.
[Prove e limiti](../rom_data_graphics.md#direct-base-palette-consumption).
Programmi alternativi, altre famiglie/lingue e ulteriori gate05restano aperti.

**ROMDATA-04-BG-ALTERNATE-PALETTE-CONSUMPTION completato per il catalogo valido:**
owner immutabile e letture CPU/DMA/native dai banchi decodificati per i letterali,
link tipizzati e metadati di controllo esatti. Nessuna istruzione/copia/ciclo
saltato, I/O nel gameplay o cambio di schema/cache nativa; dataset ancorae7-d6.
Linux216core/104runtime e Wine216/103passano, più quattro prove regionali dei
produttori originali/nativi con ROM dati avvelenati:1256chiamate/caso, stato
completo e clock uguali, uso positivo dei banchi e prova separata dei byte
letterali tramiteAPI nativa. Load/Undo/backup/riavvio conservano owner/contatori
anche con file eliminati. Restano scene alternate reali, precondizioniFF,
GUI/archivi e Windows nativo; nessuna misura prestazionale.
[Contratto e prove](../rom_data_graphics.md#direct-alternate-palette-consumption).

**ROMDATA-04-BG-PACKET-CONSUMPTION completato per EUR/Wine ridotto:**
352programmi tipizzati conservati nell'owner immutabile; valori raw/letterali
LZ77dai word decodificati, body tipizzato per HLE, controlli originali esatti.
Letture CPU/DMA/API native e paginaEUR895collegate; letture miste palette/packet
composte correttamente. Inizializzazione dinamica, padri, scritture e clock guest
conservati; schema1/8/7e identità save/native invariati. Tre test Wine mirati
passano: sintetico letture/SWI/DMA/owner, nove chiamate originali/nativeEUR con
intervalli ROM packet alterati e stato completo uguale, Load/Undo/backup/riavvio
con copia temporanea dei file resa indisponibile.18byte tipizzati letti dalle
istruzioni native e10506byte HLE. USA resta verified-only; nuovi controlli
Linux/USA da fare, nessun benchmark o suite ampia.
[Contratto e prove](../rom_data_graphics.md#direct-european-packet-consumption).

**ROMDATA-05-BG-PACKET-SCENE completato per una transizione EUR/Wine:**
slot5LCD B/input397frame già conservati, entrata pubCyrilBG62. Ogni stato
completo/clock, immagine/OAM/dispatch e campione audio coincide; tre chiamate HLE
consumano4720byte body/2640byte tipizzati. Le12letture wrapper non contengono
valori raw: la prova nativa04copre quel percorso separatamente. Altre scene,
GUI/archivi, nuovi controlli Linux/USA e Windows nativo restano aperti.

**ROMDATA-04-BG-GRAPHICS-CONSUMPTION completato per EUR/Wine ridotto:**
word immutabili del settimo filee9e mappe dei letterali del codec originale;
letture CPU/DMA/API native, header/controlli esatti e pagineEUR769..895.
Nessuna istruzione/scrittura/ciclo guest saltato. Tre test Wine mirati passano:
sintetico con sorgenti/letterali duplicati alterati, quattro chiamate originali/
native con tutte50sorgenti ROM alterate e stato completo/clock uguale, owner
conservato in Load/Undo/backup/riavvio con sole copie temporanee indisponibili.
29023byte tipizzati letti dalle istruzioni native. Schema1/9/8e identità
save/native invariati. Linux/USA da fare, nessuna misura prestazionale.
[Contratto e prove](../rom_data_graphics.md#direct-european-main-graphics-consumption).

**ROMDATA-05-BG-GRAPHICS-SCENE completato per una transizione EUR/Wine:**
stessoslot5LCD B/input397frame conservati, ingresso pubCyrilBG62. Ogni stato
completo/clock, immagine/OAM/dispatch e campione audio coincide;14815letture
grafica/16209byte sorgente/10334byte letterali decodificati. Altre scene/GUI/
archivi, nuovi controlli Linux/USA e Windows nativo restano aperti.

**ROMDATA-04-BG-TRANSFER-CONSUMPTION completato per EUR/Wine ridotto:**
controlli/tile del filee10conservati come strutture immutabili; letture CPU/DMA/
native tipizzate nelle pagine778..897e contatori separati. Countdown/flag/
progresso DMA/clock guest invariati. Tre test Wine mirati passano, inclusi due
casi originali/native con tutte76sorgenti ROM alterate, stato completo uguale
dopo60chiamate,66byte controlli compilati e9984byte tile tramite DMA nativo.
Load/Undo/backup/riavvio mantengono l'owner con copie temporanee indisponibili.
Schema1/10/9e identità save/native invariati. Linux/USA da fare.
[Contratto e prove](../rom_data_graphics.md#direct-european-transfer-consumption).

**ROMDATA-05-BG-TRANSFER-SCENE/CLI completati per EUR/Wine dichiarato:**
transizione LCD B397frame/pubCyrilBG62con ogni stato/clock/immagine/OAM/dispatch/
audio uguale,1203byte controlli/21504tile effettivi. Un nuovo CLI decoded riusa
e10senza scritture e la cache nativa, stesso risultato del riferimento legacy
conservato senza ripeterlo. Altre scene/GUI/archivi, Linux/USA e Windows nativo
restano aperti; nessuna misura prestazionale.

## ROMDATA-05 — Equivalenza, salvataggi e prestazioni

**ROMDATA-04-TEXT-CATALOG-CONSUMPTION completato per EUR/Wine dichiarato:**
modello completo dal decimo file, letture tipizzate CPU/DMA/native per puntatori,
stringhe, indici relativi, header, programmi, glifi/metriche; letterali LZ/RLE
da unità decodificate, controlli originali esatti. Tre controlli Wine passano:
sintetico con sorgenti/letterali duplicati alterati,15catene originali/native
con33581intervalli ROM alterati e90stati/clock uguali, Load/Undo/backup/riavvio
con copie dei file indisponibili. Uso compilato positivo negli8tipi; contesto
dinamico valido esplicito, nessuna scena ordinaria dedotta. Sottoinsieme vecchio
autenticato senza doppio owner/contatori. e12-d11/save/native ABI invariati.
Scene/CLI05, altre radici/font/Unicode, Linux/USA da fare.
[Prove](../rom_data_text.md#broad-direct-consumption).

**ROMDATA-04-TEXT-SHORT-CONSUMPTION completato per EUR/Wine dichiarato:**
owner immutabile con indici/codici/glifi/metriche dal nono file, pagine sorgente
fisse e contatori distinti. Tre controlli Wine passano: sintetico larghezze/
mirror/bordi/SWI/DMA/Load, cinque catene originali/native con tutti670intervalli
ROM alterati e20chiamate/stati/clock uguali, Load/Undo/backup/riavvio con copie
temporanee dei file indisponibili. API native leggono20byte tabelle/124stringhe/
636glifi/6metriche tipizzati; modalità fissa salta le metriche come l'originale.
e11-d10/save/native ABI invariati. Scene gameplay ordinarie, altri cataloghi/font/
Unicode/comandi, Linux/USA da fare. [Prove](../rom_data_text.md#direct-consumption).

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
- [ ] **Rinviato dall’utente:** misurare separatamente estrazione iniziale, avvio con dataset pronto,
  spazio su disco, RAM e tempi di frame/caricamento/audio rispetto al percorso
  attuale in memoria. Fissare i limiti di regressione prima del confronto.
  Registrare anche un risultato neutro o peggiore: l'estrazione non prova velocità.
- [ ] Verificare Windows nativo; usare Windows/Wine e Linux come prove distinte.
  Aggiornare launcher/documentazione/packager senza includere ROM, risorse estratte,
  save personali o codice generato nei repository e nei pacchetti distribuiti.

**Consegna principale:** estrazione automatica, riuso all'avvio e consumo diretto
verificati per la matrice dichiarata, equivalenza senza mod e dipendenze residue
documentate. Le misure dei costi sono rinviate e non bloccano il lavoro funzionale. Le fasi mod seguenti non bloccano questa consegna.

**ROMDATA-05-OBJ-SCENES-COSTS completato nel perimetro locale dichiarato:**
[protocollo e risultati](ROM_DATA_VALIDATION.md),
[record](../changes/2026-10-21-validate-decoded-obj-scenes-and-costs.json).
Confronti Linux e Windows/Wine su mappa240, battaglia/menu3530 e avvio fino alla
battaglia8476frame: ogni stato completo, immagine/OAM, conteggio e flusso audio
coincide; lettura effettiva dell'alias EUR al frame5400. Compatibilità base degli
stati e dei save preservata, senza nuova versione. Cinque coppie Linux contro il
legacy corretto passano i limiti fissati prima della misura: +1,0081%/-0,2355%
di costo medio; risultato neutro. Sette primi avvii/riusi: mediane32,416/21,019ms
Linux e109,222/91,891ms Wine; massimo riuso Wine183,130ms conservato, con gate
sulla mediana. RSS Linux e spazio disco rispettano i budget. Test sintetico
archivi tar/zip esclude dati privati e include le nuove istruzioni.

**ROMDATA-05-OBJ-GUI completato nel perimetro EUR Linux/Windows-Wine:**
Fast tenuto e commutato, pausa/ripresa mantenendo Fast, ritorno1x, Load/Undo/
ricaricamento e ripristino backup dal menu passano con uso delle risorse,
catture ispezionate, veri callback44,1/48kHz e file protetti invariati.
[Record e limiti](../changes/2026-10-21-validate-decoded-resource-gui-lifecycle.json).
Il gain finale è0: nessuna prova di udibilità o prestazioni callback A/B.
**ROMDATA-05-OBJ-ARCHIVES completato nel perimetro dichiarato:** corrette le
copie/allowlist Docker per includere le guide richieste dal packager; archivi
reali con395test Linux/392Windows-Wine passati (24opzionali ignorati ciascuno).
Privacy, PE/ELF, TinyCC automatico e GUI del runtime effettivamente estratto con
nuova preparazione portable/Load/Undo/ricaricamento passano.
[Record e limiti](../changes/2026-10-21-validate-decoded-resource-portable-archives.json).
Archivi/binari precedenti invariati; variazione di un metadato BUILD-INFO
registrata esplicitamente.

**ROMDATA-05-OBJ-USA-INITIALIZER completato su Linux e Windows/Wine:** avvio
pulito senza save né PC forzato fino al copier originale USA, frame791. Tutte
le13istruzioni coincidono per stato completo; CpuSet legge24halfword dal file
e copia correttamente3648byte, inclusi48di metadati. Anche la prova con il solo
vecchio alias alterato dopo l'attivazione autentica passa.
[Prova e limiti](ROM_DATA_VALIDATION.md#original-usa-initializer).
Non è una verifica completa delle scene/audio USA. ROMDATA-05 resta parziale
per Windows nativo, non disponibile all'utente: per ora solo Linux/Wine.
Abilitazione normale e altre famiglie/mod restano aperte.

**ROMDATA-05-BG-SCENES-COSTS parziale:** parità completa Linux/Wine su mappa240,
battaglia/menu3530e transizione8476frame; letture BG53nella transizione. Cinque
coppie Linux passano i limiti di frame (+1,3600%/-0,6017%mediana media), così
come avvio/riuso e spazio disco. Il picco RSS invece supera il limite2MiB:
+2864KiBmediana/+2944KiBmassimo. Risultato conservato, nessuna abilitazione
normale. L'attribuzione individua la preparazione/parsing TOML; i prototipi
TOML alternativo e inline sono respinti. Il prototipo JSON ha individuato il parsing diretto con22528Bdi heap; la
revisione successiva adotta JSON standard leggibile, verificato nel runtime,
seguendo la nuova preferenza utente descritta sotto.
[Protocollo e risultati](ROM_DATA_VALIDATION.md#bg-metadata-e3-results-and-rss-failure).
Le prove scene/tempi concluse non vanno ripetute per ricostruire il contesto;
RSS, GUI/archivi BG e Windows nativo restano aperti.


**ROMDATA-02-BG-JSON completato:** revisione1/4/3, BG in JSON leggibile standard
con parsing diretto nelle strutture tipizzate; manifest/OBJ restano TOML.
Generazioni e3precedenti conservate; autenticazione, guardie sugli input,
riuso/riparazione e consumo nativo/interprete passano su Linux/Wine.
[Prove](../rom_data_graphics.md#readable-json-datasets).
**Preferenza aggiornata:** nessun lavoro per comprimere o risparmiare spazio
nei dati estratti. I149261byte BG/150818del dataset sono accettati per
leggibilità; limiti del parser proteggono da input malformati. I risultati dei
costi e dei prototipi restano informativi, senza nuovi cicli per centrare il
precedente limite disco. Prossima priorità: payload e cataloghi ancora in ROM.

**ROMDATA-05-BG-BASE-PALETTE-SCENES completato per le scene EUR dichiarate:**
nuovi confronti con dataset e6passano240frame mappa,3530battaglia/menu e8476
transizione su Linux/Wine. Tutti gli stati completi, immagini/OAM, dispatch e
flussi audio coincidono. Le scene stabili non caricano palette; la transizione
legge una palette decodificata da160B e53descrittori BG. Fixture e cache precedenti
riusate e protette. [Risultati](ROM_DATA_VALIDATION.md#base-palette-e6-scene-acceptance).
Costi/GUI/archivi e Windows nativo restano gate separati; nessuna abilitazione
normale o migrazione di altre famiglie è dedotta da queste scene.

**ROMDATA-05-BG-ALTERNATE-SCENES / GUI completati nel perimetro EUR dichiarato:**
LCD B/TV confermati nel menu originale, ingresso nel pub di Cyril con BG62.
Quattro coppie native397frame Linux/Wine coincidono per ogni stato completo,
immagine centrale/wide, OAM, dispatch e tutti gli stream audio non vuoti.
Uso alternativo positivo: LCD B165byte letterali/2link, TV153/2. Le GUI native
Linux/Wine presentano entrambe le modalità e chiudono0con318byte letterali/4link.
Snapshot/ROM/cache/dataset/canonici protetti; nessuna forzatura RAM/PC o misura
prestazionale. Restano scene/regione/lingue più ampie, FFglobali, archivi e Windows
nativo. [Prove](ROM_DATA_VALIDATION.md#alternate-palette-e7-scene-and-gui-acceptance).

**ROMDATA-05-GAMEPLAY-PROGRESSION-REWARDS-SCENE-CLI completato nel perimetro EUR/Wine:**
produzione33726senza avvisi/canonicalripristinato; CLI25924create/reuse/no-write/
vecchia cache/cwd estraneo passano,32file gestiti invariati perhash/mtime.
Primo600town→Shop→Buy positivo7discountB/record4(mask16), una sola coppiaWine52469
ha ogni stato/clock/immagine/OAM/dispatch/audio identici. SfondoShop ispezionato;
pannello inventory separato non visibile nel PPM, selezione/transazione non
inferite. Delta ordinary resta targeted-only. Nessun negativo/nuovo recording/
RAMedit/scritture save/prove362+68o27native ripetute; nuoviLinux-USA-Windowsnativo/
wholecaller/campi semantici da fare. Nessuna misura performance/spazio.
[Contratto](../rom_data_gameplay.md#managed-and-direct-progression-rewards).

**ROMDATA-05-GAMEPLAY-PROGRESSION-GATES-SCENE-CLI completato nel perimetro EUR/Wine dichiarato:**
produzione65421senzaavvisi/canonicalripristinato; CLI70968prepare/reuse-no-write/
oldcache/unrelatedcwd/1framebattlecompat passano,33file hash-mtime invariati,
zero newuse non scambiato per scena positiva. Locatorstatic174EURheaders/49payload
rappresentativi non ha flagsidonei, nessunafalsaprovaordinary. UnicoGUIWine92614
normalMadAlchemist10giorni/Sigmund/fee600/day9→19concheckpointUI2-3-4 nuovi
necessari (nessunpayloadedit/savepersonale); MissionCleared/rewards/2skilllevelup
ispezionati,209requirements/8delta/1thresholdGUI. Nuovo1000CLI29523positivo209/8;
unicoWine57387fullpair1000stati-clock-immagini-OAM-dispatch-audio identici. E35
row0otto deltaordinary chiusi; e36return/outputordinary targeted-only, threshold1
GUIavviene prima slot3; il successivo400frame Yes/fee pair sotto chiudeclass1. Sharedtestexe
cambiato dal lavoroDebug parallelo: verificatoree36immutato, copiapinnata9e5e6f0c
conquelcodicecompilatoparallelo; nessunafalsaidentitàcolvecchio19native. Provefonte/
native/lifecycle vecchie nonripetute, nessunfullpairsuicoperturanegativa. Nuovi
Linux/USA/Windowsnativo/receiptUI-FK restano, niente performance/spazio/Android.
[Contratto](../rom_data_gameplay.md#ordinary-progression-gates-dispatch-completion).

**ROMDATA-05-GAMEPLAY-EVALUATION-THRESHOLD-ACCEPT-SCENE completato per EUR/Wine class1:**
unicoGUI17436salva normale slot2 prima di Yes, conferma600gil/funds7109 e chiude0;
nessuneditpayload o savepersonale, originali/copia1protetti. Il GUIfermo alla fee
legge0threshold: confineeffettivo include conferma e acknowledgementfee. Primo
CLI70012400framepositivo16evaluationB(7divisor/7weight/1bonus/1threshold), mask1.
UnicoWine96264fullpair400stati-clock-immagini-OAM-dispatch-tuttiaudio identici,
stesso threshold1. Binari46c8cb78/9e5e6f0c e dataset/cache riusati; nessunbuild/
132+52/native27/1840score/1000completion ripetuto. Classi2..5/wholecaller/newLinux/
USA/Windowsnativo ancora dafare; nessunuso gates/rewards o outcomeinferito.
[Prova](../rom_data_gameplay.md#ordinary-evaluation-threshold-acceptance).

**ROMDATA-01-GAMEPLAY-PROGRESSION-RANK-GATES-ROOTS / ROMDATA-02-GAMEPLAY-PROGRESSION-SELECTION-GATES-SOURCE-ORACLE completati EUR/Wine sorgente:**
64record×16/1024B6e80de..6e84de daD2390/shift4/indici1..64; riga0/16Bnonowned,
nextrootindipendente6e84e0lascia2B. Minimiunsigned01/02/04tutti necessari;
06..0d richiedeANYsogliattiva, flag0e originale, primoeligible scriveselector00.
Senza condizioni scrive0eferma, esaurire64non scrive; riga52sentinel/53..64storage.
Resetlow8/input e count/flaghelper originali, nessunselector==IDschema. Modello
strictsource-only/roundtrip1024B e singolobuild senzaavvisi/oracleWine82907passano
138reali+80sintetici fullD2390/reset-count-flag:831fieldBoriginali/ANY/unsigned/
zero-terminal-vs-exhausted-no-store/fullRAM-SP-callee/guardieJSON-source.39input
protetti invariati; nessunprova vecchia ripetuta. E36immutato, pubblicazione/
runtime/lifecycle/scena/nuoviLinux-USA dafare.
[Contratto](../rom_data_gameplay.md#progression-selection-gates-source-contract).

**ROMDATA-02-GAMEPLAY-PROGRESSION-SELECTION-GATES-DATA / ROMDATA-04-GAMEPLAY-PROGRESSION-SELECTION-GATES-CONSUMPTION completati EUR/Wine:**
e37-d36aggiunge progression-selection-gates.json/gameplay.progression-selection-gates,
32file/31risorse/all30precedenti/e36conservati/USAe8-d7. DueWine73517passano
pubblicazione/retention/guardiecoherenti/binding-source-role-path-hash/transazioni/
Load-Undo-backup-restart e18fullD2390native-interprete/raw1024Balterati/stati-clock
identici. Compiledselector15/min01282/min02505/min04462/ANY1635/flag162; all64mask/
tuttibyte-confini-mirror-BIOS/row0raw16+gapraw2/SWI32typedciascuno/oldgate2-raw16-
new2/DMA2typed2raw. Primo48254compileE0061fixturebudget32argcorretto,77778senza
avvisi; nessunWinefailure. Fonte138+80chiusa/nonripetuta, helperstesseistruzioni/
clock/ABI. ProduzioneCLI/scenaordinaria/nuoviLinux-USA-Windowsnativo dafare.
[Contratto](../rom_data_gameplay.md#managed-and-direct-progression-selection-gates).

**ROMDATA-05-GAMEPLAY-PROGRESSION-SELECTION-GATES-SCENE-CLI completato EUR/Wine:**
produzione34174senzaavvisi/canonicalripristinato; CLI43381create/riuso-no-write/
vecchiacache/cwdestraneo passa,34file hash-mtime invariati. Primo400Yes-fee0newB,
negativochiuso/nofullpair. Primo1000checkpointday18dispatchcompletion CLI9298
positivo277B(selector1/min0126/min0246/min0442/ANY152/flag10), mask67108863:
prim26recordosservati. UnicoWine3138fullpair1000stati-clock-immagini-OAM-dispatch-
audio identici; immagineSigmund/MadAlchemistsuccess/skill08-02-11-04-02-08-42-02
ispezionata. NessunGUI/fixture/payloadedit/savepersonale nuovo. Nuova prova e37,
fonte138+80/native18/e36/delta8/thresholdclass1chiusi/nonripetuti. NuoviLinux-USA-
Windowsnativo/all64ordinary/semanticadisplay dafare; niente performance/spazio.
[Prova](../rom_data_gameplay.md#ordinary-progression-selection-dispatch-completion).

**ROMDATA-01-GAMEPLAY-AGGREGATE-FLAG-KEYS-ROOTS / ROMDATA-02-GAMEPLAY-AGGREGATE-FLAG-KEYS-SOURCE-ORACLE completati EUR/Wine sorgente:**
602B6e9a5c..6e9cb6,300slotu16+terminatoreffff, nextrootindipendente29AF4esattoend.
Duecontatori completi D22FCglobale/13FB1Ccontesto mantengono(key+2ff)u16,
ordine/duplicati/earlyffff, liveflag36ORlinked017fextra1/returnu16. Modello strict
lossless e unicoWine38100passano32reali+26sintetici, tutti602B osservati,
isolamentocontesto/overflow/duplicati/terminazioneprecoce/shortcircuit/readonly
fullEWRAM-SP-callee-stack/JSON-ID-order-offset-source guards. Build5092senzaavvisi,
nessun errore/ripetizioneprova vecchia. E37invariato; preparazione/consumo/
lifecycle/scena e nuoviLinux-USA-Windowsnativo dafare; altri4readerUIstatic-only.
[Contratto](../rom_data_gameplay.md#aggregate-flag-keys-source-contract).

**ROMDATA-02-GAMEPLAY-AGGREGATE-FLAG-KEYS-DATA / ROMDATA-04-GAMEPLAY-AGGREGATE-FLAG-KEYS-CONSUMPTION completati EUR/Wine:**
e38-d37aggiunge aggregate-flag-keys.json/gameplay.aggregate-flag-keys,33file/
32risorse/all31precedenti/e37conservati/USAe8-d7. Windows76657senzaavvisi e due
Wine87184passano pubblicazione/retention/coherentkey-duplicate-earlyffff-terminal-
binding-source-role-path-hash-coverage/transactionalowner/Load-Undo-backup-restart;
16fullcounterglobale-contesto originali/interprete/native con raw602Balterati
hanno stati-clockidentici. Compiledkey9540/terminal30; all602B/300slotmask5word/
confini-mirror-BIOS/preceding8-following4raw/SWI32typed ciascuno+terminal2typed2raw/
wordterminal2typed2raw/DMA2typed2raw passano. Fonte32+26chiusa/nonripetuta,
istruzioni-contesto-predicati-wrapping-clock-ABI originali. CLI/scenaordinaria/
nuoviLinux-USA-Windowsnativo dafare, nessunmisuraperformance/spazio.
[Contratto](../rom_data_gameplay.md#managed-and-direct-aggregate-flag-keys).

**ROMDATA-05-GAMEPLAY-AGGREGATE-FLAG-KEYS-SCENE-CLI completato EUR/Wine:**
produzione59636senzaavvisi/canonicalripristinato; CLI44401crea/riusa-no-write/
vecchiacache/cwdestraneo passa,35filehash-mtime protetti. Primo1000dispatchday18
positivo1204sourceB(1200key/4terminal)/all300slotmask(5wordultimi44bit). Riletture
incluse, nessun1204byte distinti o entrambicounterordinary inferiti. UnicoWine11196
fullpair1000stati-clock-immagini-OAM-dispatch-tuttiaudio identici con stessi
contatori; finalSigmund/MadAlchemistsuccess/skills08-02-11-04-02-08-42-02ispezionato.
Stato/inputesistenti, nessunGUI/fixture/payloadedit/savepersonale nuovo. Nuova
prova solo e38; fonte32+26/native16/e37sameinput chiusi/nonripetuti. NuoviLinux-
USA-Windowsnativo/altrireaderUI/entrambicounterordinary/restTODO dafare; nessuna
misuraperformance/spazio o AndroidPort.
[Prova](../rom_data_gameplay.md#ordinary-aggregate-keys-dispatch-completion).

**ROMDATA-01-GAMEPLAY-ADJACENT-HALFWORD-LOOKUP-ROOTS parziale, selezione chiusa:**
16riferimenti verificati a6e9cb6 e nextroot6e9d8a delimitano212B candidati,
ma non provano dominio106halfword. Indici fissi64/67/liste16..20 e21,22,4,23
verificati; produttori RAM/UI ancora da dimostrare. Nessun modello da soli
valori o confine successivo; riaprire solo con prova di dominio/ownership.

**ROMDATA-01-GAMEPLAY-ADJACENT-STRIDE96-ROOTS completato staticamente:**
D0744 seleziona cinque record96B/ID2..6 a6e85bc..6e879c,480B; root biased prezzo
D26AC e nextroot indipendente concordano. Ipotesi30record respinta: D2728conta
30righeRAM per l'argomento interno0..30. Modello sorgente strict/lossless con
64Bopachi/u16markup40-41/30discount42-5f, count0 alias byte41. D1F30 originale
usa un'altra tabella70B/property36, non items.json.
**ROMDATA-02-GAMEPLAY-TRADE-PARAMETERS-SOURCE-ORACLE completato EUR/Wine:**
Windows73640senzaavvisi/unicoWine52757passano499reali+160sorgentesintetica+
15operandiaritmeticiiniettati separati; all160Bcampo prezzo osservati/320Bopachi
solo roundtrip/tutti480Blossless. Originali D0744/D26AC/D1F30/146474/D2728,
low8-nulloutside/zero-skip/alias41/count0..30/unsigneddivide/wrap32/fullEWRAMreadonly/
SP-callee-stack/strictJSON-ID-order-offset-source passano. 72810istruzioni/184PC
solo coverage,146file protetti invariati. Pubblicazione/consumo/lifecycle/scena
e nuoviLinux-USA-Windowsnativo dafare; prove precedenti chiuse/nonripetute.
EUR e38-d37 invariato, nessuna misura performance/spazio.
[Contratto](../rom_data_gameplay.md#trade-parameters-source-contract).

**ROMDATA-02-GAMEPLAY-TRADE-PARAMETERS-DATA / ROMDATA-04-GAMEPLAY-TRADE-PARAMETERS-CONSUMPTION completati EUR/Wine:**
e39-d38/trade-parameters.json/gameplay.trade-parameters/34file/33risorse,
all32precedenti/e38conservati/USAe8-d7. Windows21694senzaavvisi e dueWine20164
passano pubblicazione-failure/riuso-riparo-spostamento/retention/20coherentedits/
missingfile-owner-transaction/Load-Undo-backup-restart/all480B/mask31. 28percorsi
originali/interprete/native/raw480Balterati hanno stati-clock identici; compiled
opaque/markup-low/shared41/discount0-5-10-10, opachi prezzo non inferiti. Tutti
byte/field-recordedges/mirror-BIOS/gap222raw/wordtail2typed2raw/SWI32typed each+
tail2typed2raw+oldselection2/raw222/new2/DMA2typed2raw passano. Fonte499+160+15
chiusa/nonripetuta; istruzioni/getter/liveflag-count/clock/ABI originali.
CLI/scenaordinaria/nuoviLinux-USA-Windowsnativo dafare, nessuna misura prestazioni/spazio.
[Contratto](../rom_data_gameplay.md#managed-and-direct-trade-parameters).

**ROMDATA-05-GAMEPLAY-TRADE-PARAMETERS-SCENE-CLI completato EUR/Wine:**
produzione46740senzaavvisi/canonicalripristinato, CLI17191crea/riusa-no-write/
vecchiacache/cwdestraneo passa,36filemanaged hash-mtime protetti. Primo400normale
checkpointpre-Yes/threshold-accept positivo14B(markup-low7/shared417),mask1ID2;
rilettureincluse, nessun14Bdistinti. UnicoWine89539full400pair: tutti stati-clock-
immagini-OAM-dispatch/mixed-PSG-FIFO-HLEaudio identici, stessicontatori. FinalPublist/
7109fondi ispezionato, nessunGUI/fixture/payloadedit/savepersonale nuovo. Prova
nuovae39, fonte499+160+15/native28/managed/e34threshold400/e38chiusi/nonripetuti.
NuoviLinux-USA-Windowsnativo/altricaller/discounttail-opachi-tutti5ordinary dafare;
nessuna misura prestazioni/spazio o AndroidPort.
[Prova](../rom_data_gameplay.md#ordinary-trade-parameter-fee-acceptance).

**ROMDATA-01-GAMEPLAY-ADJACENT-STRIDE7-ROOTS completato staticamente:**
11literalroot6e879c, nextindipendente6e87fe e enumerazioneitem362..375/subtract362
fissano14record7B/98B, nessunoverlape39. Nove coefficienti5bit/cap8bit/3bitopachi
roundtrip con getter indipendenti. CFE28property0..19/index low8; CFDA8cap usa
full(itemu16−362)u16, non low8. CurrentRAM/quantity/capclamp originali; UIcap>99
solo statico, arbitrarialtriindici raw/nonclaimed. Modello/oracolosorgente/
managed/runtime/scena/nuoviLinux-USA-Windowsnativo dafare, e39invariato.
[Contratto](../rom_data_gameplay.md#item-effect-parameter-static-contract).

**ROMDATA-02-GAMEPLAY-ITEM-EFFECT-PARAMETERS-SOURCE-ORACLE completato EUR/Wine:**
strict14record98B/9coeff5bit-cap8-opaque3/ID-item-order-offset-array-bitwidth/
itemowner376/source guards passano. Windows94173senzaavvisi; unicoWine94185
originalCFE28/CFDA8 passa540reali+4sondecaprawfuoriowner+166sinteticiisolati.
Tutti98B letti/roundtrip; low8getter/invalidpropertyearlyzero/fullu16capindex
vsgetterlow8/livecurrent/quantityu8/capfit-zero-remaining/returnu16/readonlyfullRAM/
SP-callee-stack passano. 31740istruzioni/156PCsolo coverage,258file protetti.
Sonde capfuoriowner non sono nuovi dati typed/fallback. Nessun vecchio itemoracle
14371/e39ripetuto; e39-d38/34file/33risorse/USAe8-d7invariati. Pubblicazione/
consumo/lifecycle/scena/nuoviLinux-USA-Windowsnativo/fullUIwrapper dafare,
nessuna misura prestazioni/spazio.
[Prova](../rom_data_gameplay.md#item-effect-parameter-source-model-and-original-oracle).

**ROMDATA-02-GAMEPLAY-ITEM-EFFECT-PARAMETERS-DATA / ROMDATA-04-GAMEPLAY-ITEM-EFFECT-PARAMETERS-CONSUMPTION completati EUR/Wine:**
e40-d39/item-effect-parameters.json/gameplay.item-effect-parameters,35file/
34risorse/all33precedenti/e39conservati/USAe8-d7, owner items reale. Windows93110
senzaavvisi e dueWine21177passano pubblicazione-failure/riuso-riparo-spostamento/
retention/22coherentedits/bitwidth-ID-item-dependency-source-role-path-hash-coverage/
missingfile-transactionalowner/Load-Undo-backup-restart/all98B/mask16383. 50percorsi
originali-interprete-native/raw98Balterati hanno stati-clock identici; compiled
coefficient/shared05/shared06bytes36-37-23. Tutti7Brecord-fieldedges/mirror-BIOS/
following4raw/wordtail2typed2raw/SWI32typed each+tail2typed2raw+trade2-itemeffect2
crossing/DMA2typed2raw passano. Fonte540+4+166chiusa/nonripetuta, istruzioni/
indexwidths/rawcapalias/liveRAM/clock/ABI originali. CLI/ordinary/nuoviLinux-USA-
Windowsnativo/fullUI dafare. DS22bancapopolata èUSA-only, non prova EUR.
[Contratto](../rom_data_gameplay.md#managed-and-direct-item-effect-parameters).

**ROMDATA-05-GAMEPLAY-ITEM-EFFECT-PARAMETERS-SCENE-CLI completato EUR/Wine:**
produzione4620senzaavvisi/canonicalripristinato; CLI59404crea/riusa-no-write37file
hash-mtimeprotetti e50707vecchiacache/cwdestraneo passano. Nuovo checkpointEUR
IN13con soli inputguest: CyrilBank/List/GoblinLv4/Play, sourceheader-hashvalidi;
nessunRAM/statepayloadedit o savepersonale. Primo400CLIpositivo3B(coefficiente1/
shared051/shared061),mask1record0. UnicoWine5779full400pair con tutti stati-clock-
immagini-OAM-dispatch/mixed-PSG-FIFO-HLEaudio identici/stessicontatori. Replay
misto native/interprete, nessunall-native/performanceclaim. Vecchieprovesorgente/
native/e39chiuse/nonripetute. NavigazioneBaguba/Sprohm/coordinateipotizzaterespinta
conservata; coordCyril120,216da statoaccettatogiàesistente, raggiunta con impulsi
brevi. FinalCLIbankbackground ispezionato ma pannelli separateUIassenti come
limiteprecedente: nessunfood-count/confirmclaim. Altri14item/20monstri/cap>99UI/
nuoviLinux-USA-Windowsnativo dafare; restoTODO aperto, niente performance/spazio.
[Prova](../rom_data_gameplay.md#ordinary-item-effect-parameters-in-the-european-monster-bank).

## ROMDATA-06 — Mod tramite sostituzione delle risorse

**ROMDATA-05-TEXT-CATALOG-SCENE-CLI completato per EUR/Wine dichiarato:**
una coppia ordinaria397frame LCD B/ingresso pubCyrilBG62riusa slot/input/cache.
Ogni stato/clock/immagine/OAM/dispatch/audio coincide, con52byte puntatori/
185stringhe/2indici/4header/27programma decodificato/8064glifi/93metriche letti
dai file. È un programma grezzo: uso compresso provato separatamente nel04.
Un solo nuovo CLI decoded riusa e12senza scritture e la cache vecchia, stesso
risultato del riferimento legacy conservato senza ripeterlo. Binario Windows
senza warning/canonico ripristinato; altri gate/GUI/archivi/Linux/USAaperti.
[Prove e limiti](../rom_data_text.md#ordinary-transition-and-production-cli).

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

**ROMDATA-01-AUDIO-TYPED-CATALOG-ROOTS completato per le radici iniziali EUR:**
M4A originale/live verificato senza nuovo replay;3player36B, directory640voci/
5120B,455header distinti/10080B,186alias header vuoto,1610radici tracce e4banchi.
Roundtrip indipendente di tutti i campi; nessun guard640/128toni inventato,
alias header150940/WaveData150944 esplicito. Banchi/campioni/bytecode completi
restano aperti. [Contratto](../rom_data_audio.md).

**ROMDATA-02-AUDIO-M4A-DIRECTORY-SOURCE-ORACLE completato EUR/Wine:** modello
strict3player/640voci/455header/457range/15236B, ID header minimo songID e alias
espliciti; fonte esatta/roundtrip/17guard JSON-shape-link-source passano. Build
Windows finale senza warning; unico oracolo Wine passa640selezioni+9input mirati,
1lookup raw640 fermato prima del deref header null e6varianti sintetiche non
pubblicate. EW-RAM completa/IW-RAM fuori scratch stack/tracce/SP/callee uguali
alle attese indipendenti,13477B effettivamente letti/1759B solo conservati.
Primo compileE0283 e prima attesa errata word20 tracce conservati; correzione
attesa zero provata dal backedge1492e6. Modello invariato; nessuna accettazione
ordinaria/native/clock rivendicata. Dataset e40-d39/USAe8-d7 invariati.

**ROMDATA-02-AUDIO-M4A-DIRECTORY-DATA / ROMDATA-04-AUDIO-M4A-DIRECTORY-CONSUMPTION
completati EUR/Wine dichiarato:** e41-d40aggiunge audio-directory.json/risorsa
audio.m4a-directory,36file/35risorse/34decoded precedenti ed e40conservati,
USAe8-d7. Proprietario tipizzato457span/15236Bsenza overlay raw, contatori
quattro categorie e maschere3player/640song/455header. BuildWindows5021senza
warning e due controlliWine51686passano pubblicazione/25guard coerenti/riuso-
riparazione-move-retention/Load-Undo-backup-Restart senza cartella,44percorsi
originale-interprete-native con raw15236Balterati/stati-clock completi uguali.
Ogni byte/span/bordo/width/mirror/BIOS/SWI-DMA passa, inclusi2typed2raw al bordo
dummy-wave e CPUSet2directory+8header+2raw. Mixer/sequencer/RAM/ABI restano;
fonte649+1+6ed e40non ripetuti, nessun nuovo errore build/Wine.

**ROMDATA-05-AUDIO-M4A-DIRECTORY-SCENE-CLI completato EUR/Wine dichiarato:**
production78108senza warning/canonico ripristinato; CLI14580crea/riusa e41senza
riscrivere38file,27876riusa vecchia cache da cwd estranea. Vecchi checkpoint
EURGoblinPlay/input400senza nuova navigazione/fixture/RAM edit:179nuoviB
(36directory/79header/64puntatori),3song/header/0configplayer letti. Unico
Wine91327fullpair400stati-clock-immagini-OAM-dispatch-mixedPSGFIFOHLEaudio
identici/stessi179B; esecuzione mista, non all-native. Immagine568×160sfondo
Bank ispezionata; pannelli omessi da separateUI serializzata come già noto,
nessuna nuova conferma/conteggio cibo rivendicata. Fonte649+1+6/native44/
e40scene chiusi/non ripetuti; nessun errore build/Wine in questa sottofase.

**ROMDATA-01-AUDIO-M4A-TONE-SEQUENCE-PRODUCERS completato per i domini prodotti
dichiarati:** grafo1610tracce/177047B/805range/zeroerrori,36comandi copia+patch
uguali a RAM conservata.390coppie banco-programma,429tone/5148B,348linkkeymap/
347B/1alias,277headerPCM/1931528payload nominali;276endpoint extra interpolati/
1fixed8entro lunghezza.2fontiCGB16Bstatiche. Piano736range/1941759nuoviBesclude
4Bgià e41. Build32264senza warning/unicoWine8457passa390BD/348map/276+1endpoint;
note/kernel sono checkpoint parziali, non mixer o scene completi, non tutti i
PCM letti. Sweep128keymap respinto come ownership: invade tabelle adiacenti.
Parser/model/nuova pubblicazione-runtime da fare; e41invariato.
[Contratto](../rom_data_audio.md#produced-tone-keymap-and-pcm-source-contracts).

**ROMDATA-02-AUDIO-M4A-TONE-MODEL completato EUR/Wine (2026-10-24):** parser
condiviso bounded riproduce1610grafi/147424stati/390programmi/2065coppie-key/
805range177047B; modello strict429tone/347mapbyte-348link/277PCM/2CGB,
736range1941759Bexact roundtrip. PCM signedi8+endpoint opzionale/CGB32nibble,
ID da prime referenze/alias canonici, nessun mixer riscritto. Build61632senza
warning/unicoWine19316passa domini/tutti i campi-hash indipendenti/JSON/source
roundtrip/28guard e fonte coerente/alias edit con stessi nuovi span respinto
dalla dipendenza directory reale.2nuove copie CGBoriginali passano32BwaveRAM/
RAM/SP in checkpoint interno, non mixer completo.492originali/evidenze protetti;
nessun390BD/348map/277endpoint/e41replay ripetuto, e41-d40/USAe8-d7invariati.

**ROMDATA-02-AUDIO-M4A-TONE-DATA / ROMDATA-04-AUDIO-M4A-TONE-CONSUMPTION
completati EUR/Wine dichiarato (2026-10-24):** e42-d41/37file/36risorse, tutti
35file precedenti/e41conservati/USAe8-d7.736source1941759Bdisgiunti/dipendenza
reale directory4B. DueWine84702/3937passano26guard/pubblicazione-riuso-ripara-
sposta/lifecycle/tutti i byte e mask;25percorsi original/interpreter/native con
rawalterato/fullstate-clock,736bordi/mirror/BIOS/SWI-DMA/alias2+2/rawfixed8.
Entrambi i callsite MP2Khook/tick leggono4428headerB+35458PCM Btipizzati da277onde,
audio nonzero bitexact e stato/clock completi uguali.276endpoint hostdiretti
separati dal clampHLE esistente; non tutti i PCM letti dal mixer. Checkpoint
interni note/header/ARM/CGBnon equivalgono a mixer originale intero. Buildfinale
49176senza warning; due errori di preparazione test corretti e conservati,
nessun cambio modello/produzione; nessun controllo già chiuso riaperto.

**ROMDATA-05-AUDIO-M4A-TONE-SCENE-CLI completato EUR/Wine (2026-10-24):**
production16173senza warning/canonico ripristinato, CLI79505crea/riusa senza
scrivere39file/vecchia cache/cwdestraneo/400bankPlay usa516956guestB(247tone/
5map/11362header/504912PCM/430endpoint),0CGB. DefaultCLIshadowoff esplicito:
unicoWine36911pair loarma dopo load e passa400stati-clock/immagini/OAM/dispatch/
audio mixed-PSG-FIFO-HLE con stessi guestB e3093208hostB(1056header/3092152PCM).
Entrambi gli usi obbligatori positivi; snapshot/input esistenti invariati,
nessuna navigazione/fixture nuova. Capturebankbackground568x160ispezionata;
separateUIomette pannelli come già noto, nessuna nuova conferma foodcount.

**ROMDATA-02-AUDIO-M4A-SEQUENCE-MODEL completato EUR/Wine (2026-10-24):**
105253istruzioni fisiche/unica interpretazione/nessun overlap,1610root/805range/
177047Bprecedenti invariati. Enum strict per wait/note/parameter/pattern/end/
return/jump/endtie/extended, ID canonici per salti/pattern/tracce; opcode esplicito
vs ereditato e operandi conservati senza hostsequencer/RAM congelata.49clockB/
36commandword144B/soloCD8-9dispatch8B aggiungono3span201B:808range177248Bdisgiunti
dalle36risorse e42. Buildfinale76630senza warning/unicoWine4313passa tutti i
campi-hash/fonti/JSON/27guard/metadata ereditati a byte invariati/directory reale.
404nuove chiamate/checkpoint originali:161prefixdispatch/158noteargs/16jump-
pattern-return/12parameter/4CD8-9/49wait/3CE/1copy36words; RAM/cursori/SP/stack
passano. Non fullsequencer/mixer/stati-clock/native/ordinary. Prima Wine29418
fallita su due PCtrascritti+2 nelle asserzioni driver, corretti dai byteROM;
log/exe/source conservati.598input/evidenze protetti; vecchie prove non ripetute.

**ROMDATA-02-AUDIO-M4A-SEQUENCE-DATA / ROMDATA-04-AUDIO-M4A-SEQUENCE-CONSUMPTION
completati EUR/Wine (2026-10-24):** e43-d42aggiunge audio-sequences.json/
audio.m4a-sequences,38file37risorse/tutti36decoded precedenti/e42conservati/
USAe8-d7. Dipendenze directory+tone reali/390domini programma-tasto da147424stati,
metadata ereditati/banchi a stessi byte respinti. DueWine49125/build11482senza
warning passano pubblicazione/25guard/lifecycle/tutti177248B/mask105253istruzioni-
49clock-36command-2ext e35percorsi original/interpreter/native con rawalterato/
fullstate-clock. Bordi808/mirror/BIOS/SWI-DMA/fallbackraw/tone2+sequence2guest-host/
runningstatusRAM vivo passano. Generic host non è hostsequencer; checkpoint
interni/lettori stretti non fullsequencer/mixerABI. Nessun nuovo errore build/Wine;
vecchio404/e42prove non ripetuti,627vecchi file protetti invariati.

**ROMDATA-05-AUDIO-M4A-SEQUENCE-SCENE-CLI completato EUR/Wine (2026-10-24):**
production48965senza warning/canonico ripristinato, CLI48325crea/riusa senza
riscrivere40file/vecchia cache/cwdestraneo/400bankPlay legge819nuoviB(366opcode/
262operand/191clock),185record sorgente osservati incluso lookahead, non conteggio
istruzioni eseguite. UnicoWine29123pair passa400stati-clock/immagini/OAM/dispatch/
audio mixed-PSG-FIFO-HLE identici/stessi819B.0command/extended/generic-host in
questa scena, non inizializzazione ordinaria. Checkpoint/input esistenti invariati,
nessuna navigazione/fixture nuova. Capture568x160sfondoBankispezionata, separateUI
omette pannelli come noto, nessuna nuova conferma/foodcount. Fonte404/native35/
e42prove non ripetuti; nessun nuovo errore build/Wine in questa sottofase.

**ROMDATA-01-AUDIO-M4A-DRIVER-LUTS / ROMDATA-02-AUDIO-M4A-DRIVER-MODEL completati
EUR/Wine dichiarato (2026-10-24):**7tabelle370b20..370d04/484Bdisgiunti e43:
180PCMkeycode/12u32base/12u16samplecount/132CGBkeycode/12i16base/60noise/16volume.
Nibbles decomposti/segno/bit opachi/alias preservati. PCMkeylow8clamp178+endpoint179/
frac255 e UMULLunsigned; CGBtonalkey<=35→0frac0/meno36clamp130+endpoint131/signedASR;
noise type4/key<=20→0/meno21clamp59. Samplefreq nibble0..15senza clamp12:0alias
ultimabasePCM,13..15aliasCGB; SoundModepubblicoskipzero. VolumerawRAM16/64delegano
clockprecedente/255raw, nessun guardguest inventato. Build87853senza warning/
unicoWine60589passa1138nuove chiamate originali (541PCM/300tonal/259noise fullreturn,
18rate/19volume prefix/1zeromode fullreturn), tutti484B alle loadsite osservate,
RAM/SP/callee/stack, fonte+JSON/15guard/fonte coerente/ownerconfine sequenze.
Non fullinit/mixer/stati-clock/native/ordinary; e43-d42/USAe8-d7invariati.
879input/evidenze protetti; nessuna vecchia accettazione ripetuta/errorebuild-Wine.
Ricognizione finale corregge ipotesi cap15globale: ramo bilanciato149640salta
clamp14964c. Ulteriore Wine27372/build59773esegue solo8nuovi return gain e passa
128/128→16,255/128→23,255/255→31; ramo unilaterale clamp15, RAM/SP/callee/stack8B
esatti. Nessun byte nuovo/cambio modello/1138ripetuto; dominio ordinario globale16
non provato, aliases da preservare. Prova/correzione produttore chiusa.

**ROMDATA-02-AUDIO-M4A-DRIVER-DATA / ROMDATA-04-AUDIO-M4A-DRIVER-CONSUMPTION
completati EUR/Wine (2026-10-24):** e44-d43pubblica audio-driver-tables.json/
audio.m4a-driver-tables,39file38risorse/tutti37decoded precedenti/e43conservati,
USAe8-d7.7span484B/dipendenza sequenze reale. Build70748senza warning/Wine51847
due controlli ridotti passano19guard/pubblicazione-riuso-riparazione-spostamento/
retention/lifecycle/tutti484B/424entrybits e28percorsi original-interpreter-native
con rawalterato/stati-clock esatti, UMULLThumb-ARM/categorie compilate tuttepositive
[10,42,6,18,24,4,2]. Alias sample0/13..15 e volume16/31/64ownerclock/255raw,
tutti7bordi/width/mirror/BIOS/SWI/DMA/guest-host crossing passano.844istruzioni/
150PC/70dispatch coverageonly; non fullinit/mixerABI.917protetti+exe immutati;
fonte1138/gain8 e vecchie prove chiuse non ripetute. Nessun errorebuild/Wine.

**ROMDATA-05-AUDIO-M4A-DRIVER-SCENE-CLI completato EUR/Wine (2026-10-24):**
Build85643senza warning/canonicalripristinato; Wine30529CLIrealecrea/riusa senza
scrivere41filegestiti/vecchiocache/unrelatedcwd400bancoEUR.756letturebyteaggregate
[132PCMkey,528PCMbase,0samplecount,32CGBkey,64CGBbase,0noise,0volume];67entryfisiche
[31,12,0,16,8,0,0], host0. UnicoWine21679pair400stati-clock/immagini-OAM/dispatch/
mixedPSGFIFOHLEaudio esatti, stesso uso positivo. Non byte distinti/performance/
allnative. Immagine568x160background ispezionata, UIseparata omessa come noto,
nessunfoodcount/conferma nuovo. Stato/input esistentiimmutati/nessuna GUI-fixture-
RAMalterazione. Fonte1138/gain8/native28/vecchie prove non ripetuti, nessun nuovo
errorebuild/Wine/livejob.

**ROMDATA-01-AUDIO-FFTA-WRAPPER-SOURCES completato, riclassificato (2026-10-24):**
370eb0è vettore eseguibile di helperIWRAMclear/copy/fill, non audio; b94690è
palette-effect dispatch grafico. Verificati7literal/5produttorifissi1/2/4/8/16,
lettore state1/u16nonmascherato/stride4, proiezione17word68Bcon12buchinull/disgiunta
38owner e44. Nessun cap16globale/totalextent dedotto; indici superiori raw.
Tre prefissi32Bnel vecchio statoEUR corrispondono univocamente a ROMcode, senza
nuovo gioco/fixture/build/prove vecchie ripetute. Catalogo audio invariato.

**ROMDATA-02-PALETTE-EFFECT-DISPATCH-MODEL completato EUR/Wine (2026-10-24):**
Modello17slot68B/kindu16canonici/5targetThumb come offsetcodice/12null. Build71666
senza warning/unicoWine22360passa campi-hash/JSON-source/12guard+wrongROM e31prefix
originali (23lookup/3stategate/5producerstore), tutti68B osservati a0814ac70.
Highwordtroncato/liveu16/null/raw17-31-255-65535/RAM-SP-callee/write2B esatti.
134istruzioni18PC coverageonly, non fullcallback/native/stateclock/ordinary.
1216protetti immutati/nessun errorebuild-Wine/vecchie prove ripetute; e44invariato.

**ROMDATA-02-PALETTE-EFFECT-DISPATCH-DATA / ROMDATA-04-PALETTE-EFFECT-DISPATCH-CONSUMPTION
completati EUR/Wine (2026-10-24):** e45-d44pubblica40file39risorse/tutti38decoded
precedenti/e44conservati/USAe8-d7. Build67564senza warning/Wine46138due checkridotti
passano12guard/pubblicazione-retention-lifecycle/tutti68B/17slotmask e10prefix
original/interpreter/native con rawalterato/stati-clock esatti,24callback/4null
compilati. Liveu16kind/troncamento/null/stategate/raw17-65535/tutti bordi-width-
mirror/BIOS/SWI/DMA/2typed-2raw passano.48istruzioni8PC10dispatch coverageonly,
non wholecallback/allocatorABI.1226protetti+exe immutati; fonte31/e44vecchie prove
non ripetute/nessun errorebuild-Wine.

**ROMDATA-05-PALETTE-EFFECT-DISPATCH-SCENE-CLI / SCENE-TRANSITION completati
EUR/Wine dichiarato (2026-10-24):** build95401senza warning/canonicalripristinato,
Wine56076CLIrealecrea/riusa senza scrivere42file/oldcache/unrelatedcwd. Boot240
neutro0uso chiuso/no fullpair. Nuovo inputnormale confermaEnglish A240/release252,
poi neutral1000: Wine60304CLIpositivo1486letture/5944bytecallbackaggregate/null0/
slotmask2(kind1solo). UnicoWine35467pair1000passa tutti stati-clock/immagini-OAM/
dispatch/mixedPSGFIFOHLEaudio, stesso uso. Non5944byte distinti/performance/allnative.
Immagine568x160intro ProductDevelopmentDivision-4/snow ispezionata. Altri4callback
/nulltargeted-only, non wholeABI.10snapshotprotetti ispezionati staticamentecon
lista vuota, nessun replay/nuovi snapshot/RAM-statepayload-savealterati. Fonte31/
native10/vecchieprove chiuse non ripetute; nessun nuovo errorebuild-Wine/livejob.

**ROMDATA-01-PALETTE-EFFECT-PROGRAM-PRODUCERS parziale (2026-10-24):**
Chiuso il percorso iniziale main080d7d24→0814b9f8→callback4. Taskpalette reale
080d7fac/public080d8214-080d8234 distinto dal vicino task080d79ac/79d0 (248candidati,
non palette).308BLdiretti;285espressioniargomento esatte/179programmiROM.3listecon
conteggi3/3/2 e8row8B aggiungono5programmi;2copieLDM/STM3word aggiungono1. BridgeR4
conservato riusa rootgiànota. Totale185programmi/1000comandi/16135Bcandidate in124
range+64liste/24stack=16223B disgiunti39owner e45. Header6B(countu8),comandi6B/
offsetrelativou16, copycolori diretto vs queued e min1sequenza documentati;
lunghezzeparametri ancora da confrontareconlettori originali Wine.
20produttoriindicizzati ora risolti come proiezioni bounded da taskID/copiemessaggi/
storecostanti/loopcontati:97celle/388B,45programmi aggiunti. Totale230programmi/
1135comandi/18221programB+476producerB=18697B/165range, disgiunti39owner.185record
precedenti identici;39bcc4solo2puntatori, coda null/scalari esclusa. Nessun nuovo
capRAM/domino globale/estensione fisica completa. Nessun modello/runtime/e46/
guest/build/test; e45/USAe8-d7invariati. Prove chiuse nonripetute.

**ROMDATA-01-PALETTE-EFFECT-INDEXED-PRODUCER-DOMAINS completato per le20proiezioni**
(2026-10-24). Fonte task080d7018usa segmenti distinti ID1..341eID342+;
ID354/355non derivano da un'unica base estrapolata. Prove/fingerprint espliciti
in indexed-producer-proofs; nuovo indexed-program-discovery autorevole230record.

**ROMDATA-02-PALETTE-EFFECT-PROGRAM-MODEL completato per EUR/Wine (2026-10-24):**
Modello lossless230programmi1135comandi/25tabelle,165range18697B, fingerprint120,
IDcanonici/puntatorinull/alias/delay/reserved e parametri conservati. Windows23752
senzaavvisi; unicoWine96989passa tutti18221programB/18681Btotali osservati sui
lettori originali,16Breservedlista conservati/nonletti;97celle/8row/2stack.6limiti
copywrap-clamp/3mincount-period/1direction255 e16guardshape-JSON-source passano.
Non fullallocator/callback/native/stato-clock/ordinary.1645artefatti+nuovotestexe
immutati; e45-d44/USAe8-d7 invariati, nessuna misura performance.

**ROMDATA-03-PALETTE-PROGRAM-RUNTIME integrazione/targeted completati EUR/Wine**
(2026-10-24): e46-d45/41file40risorse/all39decodedprecedenti/e45preservati. Binding
165/18697B dai campitipizzati; Header/command/parameter/producer e IDcanonici,
nessun guardindice RAM/sequencer/save-nativeABI nuovo. Wine78294managedpassa
12guard/ciclodivita/tutti18697B/230program25table. Nativeiniziale/diagnostico
fallivano per endpoint dentrograph concatenato; guardneltest correggeilconfine,
backendproduzione invariato. Native-only33298passa24prefissi stato-clock completi,
APIgenerate[16header52command66parameter12producer]B, rawcommand65535/offset65534/
index2/BIOS-SWI-DMA/mirror/bordi. Fonte/managed giàchiusi nonripetuti.

**ROMDATA-04-PALETTE-PROGRAM-CLI completato EUR/Wine (2026-10-24):** Windows19390
senzaavvisi/canonico ripristinato; Wine60759create/reuse-no-write43filemanaged e
cache vecchia da cwddiversa passano. NormalEnglishconfirm1000legge0nuovi byte:
locator negativo chiuso, nessun fullpair/nonripetere. Scansione staticadistinta
nuovotask080d7fac nei10snapshotprotetti worldmap/bank: managerzero/nocallback,
nessun replay/nuovosnapshot. Non ripetere percontesto.

**ROMDATA-05-PALETTE-PROGRAM-SCENE completato EUR/Wine (2026-10-24):**
Montblanc Richiamo/Pecorelle, da checkpoint di conferma ottenuto con input normali
EUR, passa CLI reale e un unico confronto1000frame in modalità240x160: stato
completo/clock/immagini/OAM/dispatch/audio misto-PSG-FIFO-HLE identici.170letture
byte aggregate=32header+90command+48parameter,4programmi,0producer. Gli altri226
programmi e le tabelle producer mantengono la prova targeted. Il primo confronto
aveva completato tutti1000frame e l'assert della nuova famiglia, poi falliva solo
il vecchio requisito generico hostOBJdimensions. Il test accetta ora letture
bus o host tipizzate; requisito header/command/parameter positivo invariato.
Ricompilato solo il test Windows senza avvisi e ripetuto solo questo confronto,
che passa. Runtime di produzione invariato; nessun nuovo build/retest fonte,
managed/native/CLI create-reuse. Conservati entrambi gli esiti.
Nessuna modifica a RAM/statepayload/save originali. F09 e preparazione sul fiume
restano navigazione privata; i locator senza esecuzione non sono prove positive.
Modello230/sourceWine/20proiezioni/native24/managed/CLI/ordinary sono chiusi;
intro1000negativo e snapshot10non sono nextaction. NuoviLinux/USA/Windowsnativo
ed una nuova scena32:9 restano da fare.

**ROMDATA-01-BLEND-PROGRAM-PRODUCERS completato per le proiezioni dichiarate
EUR (2026-10-24):** task080d77c4/ctor080d79ac-080d79d0controllano BLDCNT,
BLDALPHA, BLDY. Classificate248chiamate dirette:235costanti/2condizionali/
6indicizzate/1inline/1templateRAMmutabile/3RAMdinamiche.160programmi/345comandi,
960Bheader+1380Bcomandi+88Bproducer(22puntatori/6tabelle)=2428B/158range,
disgiunti da tutte40le risorse e46. Header6B(u16control/coeff/count), comando4B
(u16duration,u8first/second). Nessun cap16/guardindice RAM/estensione fisica
completa dedotto. Falso rootcodice080e2ec8respinto: il caller copia10B da39a5f0
in RAM e sovrascrive la durata. Ricostruzione/hash indipendenti,44opcode dei
consumatori e mapping DMAregistri passano staticamente; nessun nuovo guest run,
build o test runtime. Fonte e comportamento Wine restano criteri distinti.
[Contratto](../rom_data_graphics.md#blend-program-producer-inventory).

**ROMDATA-02-BLEND-PROGRAM-DATA completato EUR/Wine (2026-10-24):** modello
strict lossless160programmi/345comandi/6tabelle22puntatori,2428B/158range;
bitfield e riservati conservati, nessun cap16/guardRAM nuovo. Identità EUR esatta,
110opcode e riferimenti canonici autenticati. Windows12341senzaavvisi; unico
Wine94935passa160header/345comandi/22celle/5cateneinstant/templatecopy+RAMoverride/
2header massimi-mode0/6durate/3delay/index65535raw.17guardforma/3fonteeditata/
2JSON/2identitàROM passano.2376B osservati;52byte secondtarget nonalpha timed
restano conservati/hashverificati per questi initial-mode probe, nessuna
irraggiungibilità globale dedotta.52943istruzioni/264PC solo copertura.
[Modello e limiti](../rom_data_graphics.md#typed-blend-program-source-model).

**ROMDATA-03-BLEND-PROGRAM-RUNTIME integrazione/targeted completati EUR/Wine
(2026-10-24):** e47-d46/42file41risorse/all40decodedprecedenti/e46preservati.
158binding/2428B da header-command-producer tipizzati,13pagine; nessun guardRAM/
cap16/sequencer/save-nativeABI nuovo. Due prove Wine ridotte passano12guard/
ciclodivita/tutti2428B/160program6table e20prefissi original-interpreter-native
stato completo/clock, con[34header70command16producer]B daAPI compilate.
TemplateIWRAM/liveRAM/mode2→1/rawcommand65535/index3/precedenteownerpaletteindex5/
width-mirror-BIOS-SWI-DMA/new2raw2/new2old2 passano.1034istruzioni295PC93dispatch
solo copertura. Dopo review, corretto il gate di coerenza dell'insieme estratto:
una sola verifica Wine negativa aggiuntiva passa senza ripetere native/lifecycle.
Build Windows interessati senza avvisi; primo errore d'arità nei test conservato.
Nessuna prova di ABI completa allocator/callback/scheduler oordinary ancora.
[Consumo](../rom_data_graphics.md#managed-blend-programs-and-typed-guest-reads).

**ROMDATA-04-BLEND-PROGRAM-CLI / ROMDATA-05-BLEND-PROGRAM-SCENE completati
EUR/Wine (2026-10-24):** Windows81247production senza avvisi/canonico ripristinato.
Wine46190CLI create/reuse-no-write44filemanaged/cachevecchia/cwddiversa passa;
tutti42file della generazione coincidono col dataset accettato. Scena EUR reale
Montblanc Richiamo/Pecorelle da checkpoint/input conservati:18letture byte
aggregate=10header+8command,1programma,0producer. Unico confronto Wine91145di
1000frame sulla generazione pubblicata dal CLI passa tutti stati completi/clock/
immagini240x160/OAM/dispatch/audio misto-PSG-FIFO-HLE. Altri159programmi e6tabelle
producer mantengono copertura targeted; nuova scena32:9 ancora da fare.
Nessun rebuild test, navigazione/fixture/payload edit o repeat fonte160/inventario
248/e46/native20/lifecycle. USAe8-d7 invariato; nuoviLinux/USA da fare.

**ROMDATA-01-TASK-DESCRIPTOR-SOURCE completato per la proiezione EUR dichiarata
(2026-10-24):**341record primari/20ausiliari/51script,412record8B/3296B/3range,
disgiunti41owner e47. CallbackThumbnullable/contextsizeu16/metadatau16grezzi;
268callback nonzero distinti/3recordnull. Confine39a490provato dal consumatore
grafico indipendente e dalla riga directory kind0/ID413che seleziona39a488.
IDprimario362≡secondario363, ultimo412≡413; nessun guardglobale ai selettori.
59halfword/12wordfingerprint e roundtrip/hash statici passano;53BLcandidati
conservati, CFG incompleti restanoignoti. Ipotesi456record respinta, non riaprire.
[Contratto](../rom_data_tasks.md).

**ROMDATA-02-TASK-DESCRIPTOR-MODEL completato EUR/Wine (2026-10-24):** modello
strict lossless341+20+51record/3296B/3range, ordinale/Thumbnullable/contextsize/
rawmetadata e due formule ID esatte. Windows24937senzaavvisi; unicoWine82576passa
415loadprimari/59secondari/346metadatahalfword/346byte/18low16/9branch e23guard.
3156Bdistinti osservati/140metadataBconservati nonletti inquesteprove; alias e
indici raw0/65535/413-414preservati.7191istruzioni39PCsolocopertura, nonABI completa
allocator/callback/scheduler/native/stato-clock/scenaordinaria. e47-d46/USAe8-d7
invariati; nessuna ripetizione provechiuse/performance. Linux/USA dafare.
[Modello e prove](../rom_data_tasks.md#reduced-wine-source-validation).

**ROMDATA-03-TASK-DESCRIPTOR-RUNTIME completato nei criteri mirati EUR/Wine
(2026-10-24):** e48-d47pubblica task-descriptors.json/engine.task-descriptors,
43file42risorse/all41decodedprecedenti/e47conservati/USAe8-d7invariato.3binding
3296B/pages399-39a, callback/contextsize/metadata tipizzati, selettori/ABI invariati.
Windows72886senzaavvisi/Wine32369duecheck passano13guard/lifecycle/all3296B/
412record3cataloghi e29prefix original/interpreter/native stati-clock completi.
CompiledAPI [44callback22context9metadata]B, alias/raw0-65535-413/414/low16/special
metadata/oldblendowner6B alterato, bordi-width-mirror-BIOS-SWI-DMA/new2raw2 passano.
206istruzioni41PC29dispatch solocopertura, non ABIintera/scenaordinaria. Fonte e
provechiuse nonripetute. [Consumo](../rom_data_tasks.md#managed-task-descriptors-and-typed-guest-reads).

**ROMDATA-04-TASK-DESCRIPTOR-CLI / ROMDATA-05-TASK-DESCRIPTOR-SCENE completati
EUR/Wine (2026-10-24):** Windows87514production senzaavvisi/canonicalripristinato;
Wine38310CLI realecrea/riusa senza scrivere45filemanaged/carica cachevecchia da
cwddiversa; tutti43filegenerazione coincidono coldatasetaccettato. Scena autentica
EURPecorelle1000/classic240x160passa28Baggregate=16callback8context4metadata/
4record2cataloghi. UnicoWine58452pair dallagenerazioneCLIpassa tutti1000stati-clock/
immagini-OAM-dispatch/audio mistoPSGFIFObitwiseHLE. Altri408record/terzocatalogo
mirati solamente; nonABIintera/new32:9. Nessuna ricreazionefixture/navigazione/
modifica RAM-payload-save/ripetizione source-native-lifecycle vecchieprove.
[CLI e scena](../rom_data_tasks.md#production-cli-and-ordinary-scene).

**ROMDATA-01-EFFECT-EVENT-DIRECTORIES / ROMDATA-02-EFFECT-EVENT-DIRECTORY-MODEL
completati per EUR/Wine dichiarato (2026-10-24):**75eventi16B/51selettori4B/
28worddispatch pariThumbsame-state,1516B/4range disgiunti42owner e48. Incremento/
confronto74 e catalogo indipendente3acebc provano1200B; lettore indipendente3ad240
prova204Bselettori senza dedurreestensione dai51task. Corretto candidato128b4c
(literal→tabella128b50), conservata guardia iniziale troppo stretta125394→125398.
19+51riferimenti logici al catalogo task; q0/index65535/ID362/jumpRAM/default
restanooriginali. Windows78776senzaavvisi/unicoWine43580passa77header77delay/
53selector43param/32dispatch-default/19low16-wrap e27guard.1106Bdistinti osservati,
300eventraw+110paramBconservati-hashchecked/nonletti inquesteprove.2582istruzioni/
70PCsolocopertura; noninterpreter-allocator/native/stato-clock/ordinaryABI o prova
reachability124e1c. e48-d47/USAe8-d7invariati, Linux/USA dafare. [Contratto](../rom_data_events.md).

**ROMDATA-03-EFFECT-EVENT-DIRECTORY-RUNTIME completato nei criteri mirati
EUR/Wine (2026-10-24):** e49-d48pubblica44file43risorse/all42decodedprecedenti/
e48conservati/USAe8-d7invariato, dipendenza realeengine.task-descriptors.4binding
1516B/pages124/128/3ac/3ad daeventi/selettori/dispatch pari, senzaROMshadow/guestcap/
sequencer/save-nativeABI nuovo. Windows93869senzaavvisi/Wine91663duecheckfirstPASS:
13guard/lifecycle/all1516B/75event51selector2table e29prefix original/interpreter/
native statieclockcompleti. APIgenerated[24event6selector24dispatch]B; index75/
51-65535/q0/wrap/default/evenThumb, event65535→fontprecedente11B, paletteprecedente
stessapagina4B, bordi-widths-mirrors-BIOS-SWI-DMA/new2raw2 passano.186istruzioni70PC/
29dispatchsolocopertura, nonABIintera/ordinary/reachability. [Consumo](../rom_data_events.md#managed-publication-and-typed-guest-reads).

**ROMDATA-04-EFFECT-EVENT-DIRECTORY-CLI completato EUR/Wine (2026-10-24):**
Windows48886production senzaavvisi/canonicalripristinato; Wine76313CLI realecrea/
riusa senza scrivere46filemanaged/carica cachevecchia da cwddiversa; tutti44file
generazione coincidono coldatasetaccettato e49.

**ROMDATA-05-EFFECT-EVENT-DIRECTORY-SCENE completato EUR/Wine (2026-10-24):** locator autenticiEUR
Pecorelle1000, schoolcoldboot3000 e suspendboot-to-battle8476/classic0 terminano
con0letture nuovafamiglia; chiusi negativi, nessun fullpair richiesto per eventi. Tracciaopcode59/5a→128af4 e letturaautenticata
snapshotPecorelle/F09 conroot20159d4zero restringono ilcontesto; candidatocon
identitàUSA esclusoprima dell'ispezione/replay. Nuova progressione normale EUR
avvio/nome/neve/strada/camera produce19snapshot autentici, roundtrip esatti e
hash input/artefatti verificati; nessuna patch RAM/state/save o modifica originali.
Due nuovi locator limitati3180tutorial/6000turn-transition negativi chiusi,
nessunfullpair. Il successivo stato39420camera/libro dà un nuovo locator1800
Wine59085positivo:2bus read/5B=0event+1selector+4dispatch, unselector/unatable.
UnicoWine91753pair1800stesso stato/input/generazioneCLI e52/cache/testexe passa
tutti stati-clock/immagini central-wide/OAM/dispatch/audio mistoPSGFIFObitwiseHLE,
campioni[3950100,3950100,475200,475200,3950026], primo uso39946/stessi5B/
470hash-mtime guard. Criterio event-OR-selector chiuso con lettura selector;
eventrows0/altri50selector/altratable restano mirati. Nessun rebuild prodotto/
core/prepare/vecchie prove/performance. Linux-USA-Windowsnativo/new32:9/ABIintera/
reachability124e1c non verificati. [Prova](../rom_data_events.md#positive-ordinary-book-transition).

**ROMDATA-01-SCRIPT-COMMAND-DIRECTORY / ROMDATA-02-SCRIPT-COMMAND-DIRECTORY-MODEL
completati per EUR/Wine dichiarato (2026-10-24):** 115 comandi da6B/690B a
3acebc..3ad16e, tre root originali e limite indipendente da due lettori lookup
1263c4/126468. Candidato iniziale116 respinto prima del piano/modello: includeva
la tabella distinta a3ad16e. Callback Thumb BX non-null unici e advanceu16,
nessuna dipendenza asset; disgiunto da43owner e49. Modello strict lossless e
106halfword/6wordfingerprint/hashroundtrip passano. Primo Wine49512 fallisce
aspettativa testADDSr1 invece di r0; corrette solamente due asserzioni, prova
fallita conservata. Windows12070 senzaavvisi e unico filtro Wine67188 corretto
passano117callback/117advance/115BX/4signedcursor/10relative-target/5loop-return,
tutti690B distinti osservati e19guard.2905istruzioni/67PC solo copertura; non
interpreter/native/fullstate-clock/ordinaryABI. e49-d48/USAe8-d7 invariati;
pubblicazione/consumo runtime/CLI/scena positiva completati nelle unità sotto.
[Contratto](../rom_data_scripts.md).

**ROMDATA-03-SCRIPT-COMMAND-DIRECTORY-RUNTIME completato nei criteri mirati
EUR/Wine (2026-10-24):** e50-d49 pubblica45file44risorse/all43decodedprecedenti/
e49conservati/USAe8-d7 invariato. Nessuna dipendenza asset, coerenza source-set
richiede owner precedente.690B/pages3ac-3ad da115comandi tipizzati, senzaROMshadow/
nuovi cap guest/ABI save-native. Windows93228 senzaavvisi; Wine6355duefiltri
passano al primo tentativo13guard/lifecycle/all690B e26prefix original-interpreter-
native stati/clock completi. APIcompiled14advance/28callback B; raw115/255, cursore
signed/wrapu16/targetrelativi/BL-BX/callreturnloop, ownerprecedenti event4B/selector4B,
previous2new2/new2raw2, bordi-width-mirror-BIOS-SWI-DMA passano.314istruzioni/70PC/
32dispatch solo copertura, non intera ABI script/callback/scena ordinaria.
[Consumo](../rom_data_scripts.md#managed-publication-and-direct-command-fields).

**ROMDATA-04-SCRIPT-COMMAND-DIRECTORY-CLI completato EUR/Wine (2026-10-24):**
Windows18103production senzaavvisi/canonicalripristinato; Wine20133CLI realecrea/
riusa senza scrivere47filemanaged da cwddiversa, carica cachevecchia. Tutti45file
generazione coincidono coldatasetaccettato. Unsolo frame boot di attivazione,
0letturecomandi: non equivale ad accettazione di una scena ordinaria.

**ROMDATA-05-SCRIPT-COMMAND-DIRECTORY-SCENE completato EUR/Wine (2026-10-24):**
Traccia produttore1265cc←125b6a/manager125ac8 e callback dinamici;63snapshot EUR
transitori autentici conservati hanno rootqueued0, nessun replay né modifica.
Unico locatorCLI Wine81783 con input+suspend save autentici già conservati fino
all'ingresso battaglia8476/classic240x160 legge42B=14advance28callback/7comandi.
Unico Wine93633 fullpair dallagenerazioneCLI passa tutti8476stati-clock/immagini/
OAM/dispatch e audio misto-PSG-FIFO-HLE bitwise, stessa copertura positiva.
Riusa testexe/cache/input/save, nessun rebuild/prepare/oracolo/native/lifecycle/
vecchieprove o creazionefixture/navigazione/payloadedit. Altri108comandi mirati
solamente; non intera ABI/new32:9. e49event/selector/dispatch0: terzo locator
negativo chiuso, niente fullpair richiesto per eventi.316hash-mtimeguards passano.
[Scena](../rom_data_scripts.md#production-cli-and-positive-ordinary-scene).

**ROMDATA-01-SCRIPT-ARCHIVE completato nell'inventario statico EUR (2026-10-24):**
115464B/2range: archivio relativo114528B B946EC..BB064C e234chiavi4B/936B
D68C2C..D68FD4. Limiti da root indipendenti testo localizzato/coordinatedirectory.
Domini delle chiavi e primi offset concordano con62banchi/233slot,248+932B indici.
213target distinti/20link ripetuti includono5link aheader e2aFFfill: conservati,
non respinti/marcati morti.207segmenti fisici/35048comandi/110opcode/78212paramB;
45fillspan88B.493link5a verso49selettori esistenti e dipendenza reale dai comandi.
Byte2chiave seleziona cinque archivi testo ancora nonmigrati; byte3raw conservato.
Ricostruzione indipendente/tutti115464B/234chiavi/233formule target non-null/128halfword5wordguard/
disgiunto44owner passano senza guestexecution. Prima assunzione di programma
nonvuoto respinta su46:1/49:1; corretta prima del piano. Non CFG/intera semantica
argomenti/reachability/nuovi cap guest. Inventario e modello chiusi; integrazione
runtime/CLI/scena trattata nelle unità sotto. Inventario lasciava e50-d49/USAe8-d7 invariati. [Contratto](../rom_data_scripts.md#relative-script-archive-and-event-key-source-contract).

**ROMDATA-02-SCRIPT-ARCHIVE-MODEL completato EUR/Wine (2026-10-24):** modello
strict lossless dell'intero ambito115464B,234chiavi/62banchi233slot/207programmi
fisici35048istruzioni/45fill; target Program/Header/Fill e parole relative
ricostruite, nienteROMshadow/copiealias. Lunghezze e493link dipendono dagli asset
comandi/effetti reali. Windows91283senzaavvisi/unicoWine85436 passa alprimo giro:
238keyaddress/234target2livelli(compreso raw0unaligned)/234textbank/35054opcode/
2397relative-target/493selectorparam. Tutti115464B ricostruiti;42333Bdistinti letti
ai loadsite originali,72811param+234keyraw+86fill=73131Bconservati/hashchecked non
letti inquesto ambito.40guard passano;224382istruzioni62PC solocopertura. Nonintera
ABI/interpreter/native/stato-clock/ordinary. e50-d49/USAe8-d7 invariati; nuovi
Linux/USA/Windowsnativo dafare. [Modello](../rom_data_scripts.md#typed-relative-archive-model-and-reduced-wine-source-oracle).

**ROMDATA-03-SCRIPT-ARCHIVE-RUNTIME completato nei criteri mirati EUR/Wine
(2026-10-24):** e51-d50 pubblica script-archive.json/engine.script-archive,
46file45risorse/all44decodedprecedenti/e50conservati/USAe8-d7 invariato. Dipendenze
reali comandi+effetti validate; due binding115464B ricostruiscono indici/opcode/
parametri/chiavi/fill tipizzati, senza shadow/nuovi limiti guest/sequencer/ABI.
Windows22414senzaavvisi/Wine76182duefiltri passano alprimo tentativo13guard/
lifecycle/all115464B=1180index35048opcode78212parameter936key88fill e28prefix
original/interpreter/native stati-clock completi. APIcompiled37index4opcode/
7parameter12key1fillB; raw0/234/239/65535/low16/rotation/header-fill-target/q0/
relative-target/returnloop/oldpalette4B stessapagina/mixedraw-new/bordi-width-
mirror-realBIOS-CpuSet-FastSet-DMA passano.169istruzioni66PC28dispatch solocopertura;
noninteraABI/ordinary. [Consumo](../rom_data_scripts.md#managed-relative-archive-and-typed-guest-reads).

**ROMDATA-04-SCRIPT-ARCHIVE-CLI completato EUR/Wine (2026-10-24):** Windows48522
production senzaavvisi/canonicalripristinato; Wine66454CLI realecrea/riusa senza
scrivere48filemanaged da cwddiversa/carica cachevecchia. Tutti46filegenerazione
coincidono coldatasetaccettato e51. Locator autentico suspendboot-to-battle8476/
classic240x160 da input/save conservati positivo:27B=8index7opcode10parameter2key,
un banco/programma/chiave,0fill.365hash-mtimeguard passano; nessun rebuild vecchie
prove/fixture/navigazione/payload edit.

**ROMDATA-05-SCRIPT-ARCHIVE-SCENE completato EUR/Wine (2026-10-24):** unico
Wine44039fullpair8476frame dalla generazioneCLI e51, stessi input/save autentici/
testexe/cache già esistenti, passa tutti stati-clock/immagini central-wide/OAM/
dispatch/audio mistoPSGFIFObitwiseHLE, uso27B invariato e366hash-mtimeguard.
Non ripete l'accettazione e50/source-model/native/lifecycle/CLI/prepare/build.
Nessun test guest fallito/fixture/navigazione/payload edit. Altri61banchi/206programmi/233chiavi/
tutti45fill mirati solamente; noninteraABI/new32:9. e49usoordinario positivo
resta aperto dopo tre locator negativi; Linux/USA/Windowsnativo dafare.

**ROMDATA-01-SCRIPT-LOCALIZED-TEXT completato nell'inventario statico EUR
(2026-10-24):** cinque root BB064C/C076F0/C5FBF0/CB7C48/D10644, limiti dalle root
lingua indipendenti e script-keyD68C2C:1803744B/5range disgiunti45owner e51.
63voci top perlingua da keymax62/minoffsetFC, prima voce aliasbanco0nonlunghezza.
11link normali condivisi/lingua,26varianti banco61 selezionate da divisione del
byte vivo per10; resto separato rimappa il messaggio per24con troncamento u16.
Branch full-u16/low-byte/special/fallback/rootkey originali, nessun nuovo cap.
385banchi/41395indici-u16 inclusi385sentinel/41010programmi8202perlingua; grammar
Program/customLZ già presente.9813compressed extended/5709raw extended/25488raw
compact,212011comandi11forme/147glifi1..3112 già nel font; dipendenze reali chiavi
script e font.1260top+520variant+82790bankindex+1684407program+34767alignment B.
34314gap1..3B, quattro misti: respinta ipotesi uniforme prima del piano/modello/
Wine, bytes esatti conservati. Ricostruzione/hash interi1803744B/15195formule/
349halfword13wordguard/deps/glifi/disgiunzione passano senza guestexecution/build/
test o repeat prove precedenti. e51-d50/USAe8-d7 invariati; modello/Wine chiusi
nell'unità sotto, runtime/CLI/ordinary restano figli aperti. [Contratto](../rom_data_text.md#script-selected-localized-text-source-contract).

**ROMDATA-02-SCRIPT-LOCALIZED-TEXT-MODEL completato EUR/Wine (2026-10-24):**
modello strict interi1803744B con riferimenti Bank/Variants tipizzati, payload
fisici non duplicati, u16indici/sentinel ricostruiti e gap esatti. Program codec
esistente/header16bit/argomenti/encoded-decoded tails/deps chiavi-font conservati.
Windows42377senzaavvisi/primoWine95147fallisce JSON: variante unit accettava
campoextra; corretto a emptystruct mantenendo asserzione, prima di casi guest.
Tutti artefatti prima prova conservati. Windows55123senzaavvisi/unicoWine34745
corretto passa tutti dati/hash/1803744B/385banchi41010programmi e45guard:
22shape6source-edit3dependency12JSON-shape2identity. Nuovi casi originali7lingue/
1170key-bank/1280quotient-vivo/256remainder-vivo/90raw-special/1170u16indici/
837header/837source-vs-typed output+guard, inclusi249corpi/code decompressi.
2116900istruzioni834PCsolo copertura, non native stato-clock/ABIintera/ordinary.
Checkpoint inizializzato conservato e immutato, argomenti scratch controllati;
no fixture gameplay/navigazione/modifica checkpoint-save-input o vecchie prove
ripetute.4060artefatti precedenti/prima prova/newexe e368hash-mtime protetti.
Solo due dichiarazioni modulo in file esistenti; e51-d50/USAe8-d7/ABI invariati.
[Modello](../rom_data_text.md#typed-script-selected-text-model-and-reduced-wine-source-oracle).

**ROMDATA-03-SCRIPT-LOCALIZED-TEXT-RUNTIME completato nei criteri mirati EUR/Wine
(2026-10-24):** e52-d51/script-text.json/text.script-archives,47file46risorse,
all45decodedprecedenti/e51/USAe8-d7 conservati, dipendenze reali chiavi/font.
Cinque binding1803744B per indici/header/control/glifi/comandi/operandi/terminator/
slack, literalmap condiviso con testi precedenti; ABI/save/live guest invariati.
Coda decodificata conservata,0literal sorgenti perché generata da LZ/backref:
respinta solo ipotesi privata all-class-positive. Windows23146core senzaavvisi;
primo frontend fallisce solo array test5vs11/importunused, corretto test e
ricompilato solo frontend39830senzaavvisi, log iniziale conservato. Wine95863
passa tre filtri alprimo avvio: unico core literal-poison per helper cambiato,
managed13guard/lifecycle/all1803744B/5lingue385banchi41010programmi130varianti34314gap
più163Btesti precedenti0/1/2, native35prefissi original/interpreter/compiled stati-
clock completi. CompiledAPI[21,17,17,13,6,4,5,1,3,0,3]B; raw255/wrap/sentinel/
BB0-D68previous-new/rawdopoownerchiavi/width-mirror-realBIOS-CpuSet-FastSet-DMA
passano.379istruzioni65PC71dispatchsolo copertura, nonABIintera/reachability.
[Consumo](../rom_data_text.md#managed-script-text-and-direct-typed-reads).

**ROMDATA-04-SCRIPT-LOCALIZED-TEXT-CLI completato EUR/Wine (2026-10-24):**
Windows51427production senzaavvisi/canonicalripristinato. Wine71122CLI realecrea/
riusa senza scrivere49filemanaged/cwddiversa/cachevecchia, tutti47filegenerazione
uguali aldatasetaccettato;468hash-mtimeprotetti. Locator suspendboot8476legge0
nuovitesti: negativochiuso/niente fullpair, nonricontarevecchiocontesto e49.

**ROMDATA-05-SCRIPT-LOCALIZED-TEXT-LOCATOR completato EUR/Wine (2026-10-24):**
Audit statico74snapshot EUR su campi testo non dà cue manager; nonprova deadcode,
quattro header USA esclusi prima delpayload/replay. Nuovo locator Wine24902
coldboot school3000/classic0 da input autentico conservato, senza save/state:
289B=8top4bankindex8header93control136glyph39command1terminator,decoded176B,
unalingua/banco/dueprogrammi,0variant/operand/tail/slack.468hash-mtime passano.
Nessun build/prepare/fixture/navigazione o modifica checkpoint-save-input.

**ROMDATA-05-SCRIPT-LOCALIZED-TEXT-PAIR completato EUR/Wine (2026-10-24):**
unicoWine43805pair3000frame dalla generazioneCLI e52/stesso input/nessun sav-state
override/testexe-cache esistenti passa ogni stato-clock/immagine central-wide/
OAM/dispatch/audio mistoPSGFIFObitwiseHLE, stesso uso289B/468hash-mtimeguard.
Nessun rebuild/prepare/vecchie prove/nuova fixture gameplay/navigazione/modifica
checkpoint-save-input. Altre4lingue384banchi41008programmi/
130varianti34314gap restano mirati;0operand/tail/slack ordinari.

**ROMDATA-01-SCRIPT-COORDINATE-INDEX-SOURCE / ROMDATA-02-SCRIPT-COORDINATE-PATHS-MODEL
completati EUR/Wine dichiarato (2026-10-24):** reader128914/rootD68FD4 e
readerA1F8/rootD69BC4 delimitano3056B/210liste/856coppieXY/42paddingFF84B.
Directoryrelativa840B/countu16 420B/XY1712B ricostruiti esattamente, disgiunti
da46owner e52;194halfword5word/hash completi.68linkscript/56liste:66opcode44
indicebyte2/byte3ignorato (respinta ipotesiu16 prima delmodello),2opcode45u16.
Dipendenze reali script-archive/script-command-directory; altezza da mappa RAM,
low16indice/low8destinazione/countu32/ritornolow8 originali, nessuncapguest nuovo.
Modellostrict lossless; Windows25513senzaavvisi/unicoWine70105passa alprimo avvio
tuttiicampi/hash/24guard ed esecuzioneoriginale212copie (tutte210liste+due celle
altezzaRAMvariate),13prefissi rawindice/destinazione e13producer-bytewidth/
ignored/nibble/taskargs.2972Bsorgente osservati,84padding preservati-nonletti;
40116istruzioni134PC solo copertura. Non native/fullstate-clock/ABIintera/
allocator/taskmovement/ordinary. e52-d51/USAe8-d7 invariati, nessunafixturno/save/
fixture/navigazione; nuove verificheLinux-USA-Windowsnativo dafare.
[Contratto e modello](../rom_data_scripts.md#typed-coordinate-path-source-model).

**ROMDATA-03-SCRIPT-COORDINATE-PATHS-RUNTIME completato nei criteri mirati EUR/Wine
(2026-10-24):** e53-d52/script-coordinate-paths.json/engine.script-coordinate-paths,
48file47risorse/all46decodedprecedenti/e52/USAe8-d7 conservati, dipendenze reali
archive/command. Owner immutabile condiviso per index/count/XY/alignment, nessuna
ROMshadow/hostmovement/capguest/ABI nuova; triploD68testo/chiavi/coordinate e raw
D69BC4 preservati. Primo build10180passacore/fallisce solo arraycontatoretest5vs4;
corretto frontend27578senzaavvisi/riusato core. Wine30996managed alprimo avvio
passa14guard/lifecycle/all3056B/210liste856punti42alignment/tutti46precedenti/e52.
Primo native si ferma su looposservatoretest0..5; corretto a zip4, conservati log/
EXE/ELF/sorgente fallito, ricompilato solo frontend75147senzaavvisi. Wine31012
solo native passa24prefissi original/interpreter/compiled stati-clock completi,
quattrocopie intere/altezzeRAM/rawarg/allclassi[28index85count76XY2alignment]B,
mixedowner/bordi-width-mirror-realBIOS-CpuSet-FastSet-DMA.1443istruzioni87PC143
dispatch solo copertura, non intera ABI allocator/movementtask/ordinary.
[Consumo](../rom_data_scripts.md#managed-coordinate-path-publication-and-typed-reads).

**ROMDATA-04-SCRIPT-COORDINATE-PATHS-CLI / ROMDATA-05-SCRIPT-COORDINATE-PATHS-PAIR
completati EUR/Wine (2026-10-24):** Windows29223production senzaavvisi/canonical
ripristinato esattamente. Wine45975CLI realecrea/riusa senza scrivere50filemanaged/
cwddiversa/cachevecchia, tutti48filegenerazione uguali aldatasetaccettato.
Locator autentico6000frame da21420/stato/input giàconservati positivo:25busreads/
28B=4index14count10XY/unalista,0alignment. UnicoWine76926pair stessi6000frame/
stato/input/generazioneCLI passa tutti statieclock/immagini central-wide/OAM/
dispatch/audio mistoPSGFIFObitwiseHLE, campioni[13167000,13167000,1584000,1584000,
13166910], stessi28B/581hash-mtimeguard. Nessuna nuova fixture/navigazione/prepare/
locator/vecchia prova ripetuta. Altre209liste/tutti42alignment restano mirati;
nuovi32:9/Linux-USA-Windowsnativo/ABIintera non verificati, nessuna misura spazio/
performance. [CLI e scena](../rom_data_scripts.md#coordinate-path-production-cli-and-ordinary-scene).

**ROMDATA-01-AUXILIARY-SCRIPT-ARCHIVE-SOURCE completato per la proiezione EUR
 dichiarata (2026-10-24):** readerA1F8/rootD69BC4 distinto dall'archivio primario,
indici relativiu16/action-slotu8.4200B:4098archivio+102directory17comandi a370F94.
8banchi/442slot/126targetdistinti/30entryinterne/133programmi fisici913istruzioni,
900index+913opcode+2275param+10alignmentB. Rootwalknoncopre69istruzioni: preservate
colparsefisico.489targetrelativi su confiniistruzione; CFGstrutturale908/913nonè
reachability, altre5preservate. FineproiezioneD6ABC6/ultimoterminator; FF2seguenti
esclusi. GraphicsbaseD6B4C8 è solo ceiling,2306Bfuoriproiezione; directorysuccessor
370FFC indipendente, zero2gap esclusi. Ricostruzione/hash4200B/723halfword4word/
formule489/disgiunto47owner e53passano senza build/guestexecution/testLinux.
Nessuncapguest/hostinterpreter/dipendenzaprimario/ABIintera. e53-d52/USAe8-d7
invariati; modello/oracoloWine verificati sotto; managed/runtime/CLI/ordinary
ancoraaperti.
[Contratto](../rom_data_scripts.md#european-auxiliary-script-archive-source-contract).

**ROMDATA-02-AUXILIARY-SCRIPT-MODELS completato EUR/Wine dichiarato (2026-10-24):**
modellostrict directory/archivio4200B/133programmi/913istruzioni con30entryinterne
program+instruction, parametri/gap/zero-row preservati; dipendenza reale callback/
advance propri, nessun capguest/ROMshadow/interpretehost. Windows66583senzaavvisi;
primoWine20886fallisce aspettativatest raw8:0/indirizzodispari. Soloosservatore/
expectationtest corretti, frontend54240senzaavvisi/Wine3148passa. Letturaexec.rs
stabilisce rotazione32-bit: ulteriorecaso8:1 verifica highbits/target14D69E97;
frontend52724senzaavvisi/Wine77162finalepassa, modello/coreimmutati, conservati
fallimento/eseguibili/log/provaintermedia.40guard/tutticampi-hash/723halfword4word,
442lookup/913dispatch+RAMopcode3/80advance/489suffissirelativi/10wrap/15raw/
11wrapper.2887Bdistinti osservati;1313B=1297param+10gap+6directoryzero conservati
hashchecked-nonletti.31839istruzioni177PCsolocopertura, nonpredicati/interaABI/
native/fullstate-clock/ordinary. Nessuna fixture/save/input/navigazione o vecchia
prova ripetuta; e53-d52/USAe8-d7 invariati, nuoviLinux-USA-Windowsnativo dafare.
[Modello](../rom_data_scripts.md#typed-auxiliary-archive-and-reduced-wine-source-oracle).

**ROMDATA-03-AUXILIARY-SCRIPT-RUNTIME completato nei criteri mirati EUR/Wine
(2026-10-24):** e54-d53pubblica directory/archivio ausiliari,50file49risorse/
all47decoded precedenti/e53retained/USAe8-d7. Dipendenza reale archivio→propria
 directory, ownerimmutabile unico4200B senzaROMshadow/hostscript/capguest/ABI
save-native nuova. Pagine370audio/D69coordinate e raw370FFA+/D6ABC6+ preservate,
control/livecursor/predicati/LDRH32-bit originali. Windows44279core+frontend
senzaavvisi; Wine61473duefiltri nuovi passanoalprimogiro26guard/lifecycle/
create-reuse-repair-move/failuretransaction/e53retained/unavailablefile/Load-Undo-
backup-Restart/stessoowner/all4200B/17comandi8banchi133programmi5gap e44prefissi
original-interpreter-compiled stati-clock completi. APIcompiled[6advance65callback
18index19opcode15param2alignment]B, entryinterne/signedcursor/rawopcode/8:1highbits/
fullaction2vs102/mixedowners/bordi-width-mirror-realBIOS-CpuSet-FastSet-DMA passano.
624istruzioni92PC52dispatch solocopertura, noninteraABI predicati/interpreter/
callback/ordinary.587hash-mtimeguard passano. Nessun vecchiotest ripetuto/nuova
fixture/navigazione/performance/spazio; CLIreale/ordinario positivo verificati
nelle unità sotto.
[Consumo](../rom_data_scripts.md#managed-auxiliary-script-fields).

**ROMDATA-04-AUXILIARY-SCRIPT-CLI / ROMDATA-05-AUXILIARY-SCRIPT-PAIR completati
EUR/Wine (2026-10-24):** Windows10336production senzaavvisi/canonicalhash-mtime
ripristinato esattamente, main.rscontenutoimmutato/mtime-only invalidazioneCargo.
Wine23153CLI realecrea/riusa senza scrivere52filemanaged/cwddiversa/vecchiacache,
tutti50filegenerazione uguali aldatasetaccettato e54. Unicolocator autentico6000
frame da21420/stato/inputgiàconservati positivoentramberisorse: directory96B=
32advance64callback/6comandi, archivio82B=16index20opcode46param/2banchi2programmi,
0alignment. UnicoWine93545pair medesimi6000frame/stato/input/generazioneCLI/
testexe-cacheconservati passa ogni statocompleto-clock/immaginecentral-wide/OAM/
dispatch/audio mistoPSGFIFObitwiseHLE, campioni[13167000,13167000,1584000,1584000,
13166910], stesso usonuovo/640hash-mtimeguard. Altri11comandi6banchi131programmi/
5alignmentspan miratisolamente; noninteraABI/new32:9/Linux-USA-Windowsnativo.
Nessun rebuild/prepare/locator/vecchiaprova ripetuta/nuovafixture/navigazione o
misura performance/spazio. [CLI e scena](../rom_data_scripts.md#auxiliary-script-production-cli-and-positive-ordinary-scene).

**ROMDATA-01-TERRAIN-MAP-SOURCE completato per il percorso EUR dichiarato
(2026-10-24):** il loader originale1B688/state6→1AE88 usa root7662A8/162record
e tutti486link field04/08/10dei352programmi BG già estratti e consumati.
Verificati14256descrittoriB/398227packetB/134aliaslink, hash e proprietà esclusiva
contro49owner,40halfword11literal9BL. Field10inizializza griglia altezze/flag
16×16a02007CB0; puntatore/dimensioni/flag vivi e piani EWRAM/VRAM restano dinamici.
Nessuna nuova risorsa/decoder/revisione o vecchio test/build rieseguito; e54-d53/
USAe8-d7 invariati. Accessor10B1DFB4/altri contesti/semantica flag restano ignoti.
[Classificazione e limiti](../rom_data_maps.md).

**ROMDATA-01-MAP-RASTER-PALETTE-SOURCE completato EUR statico (2026-10-24):**
root4872B0..489BB0/10496B,32record noncompressi/33range/324link selettori54/55,
64index192header10240colorB. Header3u16e160coloriu16perrecord;26contenuti diversi
ma32identitàfisiche. Decoderstatico ricostruisce tuttifilebyte/hash/link, verifica
67halfword13literal10BL e nessun overlap49owner. Originali1EAF0e1EDECconservano
diversi indiciunsigned/signed e fallbackFFFF; nessun nuovo cap guest32. Copia
originale161halfword/322Blegge headerseguente o prima word489BB0già appartenente
alle palettealternative, poi riscrive word160dest con word0. Nessuna duplicazione
dell'owner adiacente o eliminazione lettura/clock. Nessun build/esecuzione guest;
e54-d53/USAe8-d7 invariati. [Contratto](../rom_data_maps.md#map-raster-palette-source-contract).

**ROMDATA-02-MAP-RASTER-PALETTE-MODEL completato EUR/Wine (2026-10-24):**
modelloisolato lossless32record/33range/10496B/324link dipendenti dai descrittori;
header3u16/coloriu16/indexderivato/identitàfisica senzaROMshadow. Windows30586
frontendtestbuild senzaavvisi; unicoWine39763nuovofiltro primoPASS. Tutti campi/
hash/JSON/15guard,64inizializzazioni32getter canonici11rawgetter7prefix unsigned/
3disabled/2update differiti2setterespliciti1setupDMA passano.36osservazioni
CpuSetconfrontano tutti322Bprima del wrap, inclusa013Cdel vecchioowner adiacente.
Variante sintetica bit15/rangeword/RAMcompletafuori stack/SP/callee passano;
10496B distinti letti ai loadsite/copieHLEverificate.4753istruzioni202PCsolocopertura,
non fullstate-clock/native/ordinary o DMA raster eseguito;643hash-mtimeguard.
e54-d53/USAe8-d7 invariati, nuovoLinux-USA-Windowsnativo pending.
[Modello e prova](../rom_data_maps.md#typed-raster-palette-model-and-original-reader-oracle).

**ROMDATA-03-MAP-RASTER-PALETTE-RUNTIME completato nel perimetro mirato EUR/Wine
(2026-10-24):** e55-d54pubblica `map-raster-palettes.json` /
`graphics.map-raster-palettes`,51file50risorse/tutti49precedenti identici/e54
conservato/USAe8-d7invariato. Dipendenza realeBGdescriptor/unicoownertipizzato
serve33range10496BsenzaROMshadow; ownerprecedente conserva word489BB0/crossread.
Windows82612core/frontend senzaavvisi; unicoWine41457due nuovifiltri primoPASS:
17guard/lifecycle/Load-Undo-backup-Restart/fileindisponibili/all10496B/32programmi;
28percorsi original-interpreter-native stato-clock completi/APIcompiled[15index
46header24color]B, raw signed/unsigned/sentinelle/window/copie322B/wrap/queuedDMA/
confini-oldowner-raw. Width/mirror/BIOSbusmode/quattroSWI/unDMAimmediato passano;
646hash-mtimeguard.638istruzioni204PC55dispatch solocopertura, non intera camera/
rasterDMA/ordinaryABI. Fonte/modello già chiusi non rieseguiti.
[Runtime e limiti](../rom_data_maps.md#managed-raster-palette-publication-and-typed-reads).

**ROMDATA-04-MAP-RASTER-PALETTE-CLI / ROMDATA-05-MAP-RASTER-PALETTE-PAIR
completati EUR/Wine dichiarato (2026-10-24):** Windows26915production senza
avvisi/canonico hash-mtime ripristinato/maincontenutoimmutato-mtimeCargo.
Wine40160CLI realecrea/riusa senza scrivere53filemanaged/cwddiversa/vecchiacache,
tutti51filegenerazione uguali all'e55accettato. Unicolocator autentico coldboot
1500inputdemo conservato/senza stato-save/override positivo50880aggregateB=
2index322header50556color/25440busreads/mask2record (include header del vicino,
non due palette complete o byte distinti). UnicoWine90978pair medesimi1500frame
0..1500/generazioneCLI/testexe-cache passa ogni stato-clock/immagine/OAM/dispatch/
audio mistoPSGFIFObitwiseHLE, campioni[3291096,3291096,395212,395212,3269758],
stesso uso nuovo/701guardhash-mtime. Source/model/targeted e vecchieprove non
rieseguite; nessuna nuova fixture/navigazione/performance/spazio. Altri contesti/
intera camera-rasterABI/new32:9/Linux-USA-Windowsnativo restano pending.
[CLI e scena](../rom_data_maps.md#raster-palette-production-cli-and-positive-ordinary-boot).

**ROMDATA-01-OBJ-EMPTY-CALLERS completato nel perimetro statico diretto EUR
(2026-10-24); estensione hold non provata respinta:** i tre vecchi header vuoti
B4BD20/bank0seq1, B4BD34/bank1seq1, B4FA10/bank14seq6 restano invariati con
frames=[]/senza zero_count_hold. Dodici costruttori dei primi due banchi usano0;
18chiamanti dell'inizializzatore8BBF0producono modeword0..3→sequenze0..3;
due aggiornamenti usano2/3. Tutti30letterali delle radici/47fingerprint istruzioni/
50owner verificati; costanti attraverso chiamate secondo ABI r4..r11preservati.
SeiBdi possibili selettori restano raw; durate8/16/4alias di indici già tipizzati.
Rawmode7/65539→6/7, repeat1: nessun cap guest o prova globale di irraggiungibilità.
Nuovi producer/trace ordinaria con scheduling stabilito sono il solo motivo per
riaprire; chiamate indirette/mutazioni RAM non verificate. Nessun nuovo modello/
risorsa/revisione/build/esecuzione guest; e55-d54/USAe8-d7 invariati.
[Contratto](../rom_data_obj.md#earlier-empty-sequence-caller-domains).

**ROMDATA-01-UI-LIST-ENTRIES-SOURCE completato nel perimetro statico EUR
(2026-10-24):**18chiamanti diretti8BBF0→16ingressiROM/12liste36voci e due
filtriRAM0200F250con template397A9Cda6voci. Tredici intervalli42voci336B,
disgiunti50owner; tutti18letterali/93fingerprint e ricostruzione byte passano.
Campiu8ID/X/Y/opaque e left/right/up/downraw; niente pointer/cap/count/target
normalizzato. Countwordfirmato della copia distinto dal byte live; getter8CA64/
input8BDECcon coordinate da indice raw non mascherato. Proiezioni32maschere live
stabiliscono estensione template; ID/Y/up/down e selezione restano RAM. Ordine
clear-down→copia base→up preserva down1con zero slot, corretto durante revisione
statica privata. Nessun nuovo modello/risorsa/revisione/build/guestexecution;
e55-d54/USAe8-d7 invariati. [Contratto](../rom_data_ui.md).

**ROMDATA-02-UI-CHOICE-LISTS-DATA completato per EUR/Wine (2026-10-24):**
modello pubblico strictlossless/sharedsourceplan per13tabelle42voci336B/IDfisici,
byte3opaque e4navigationrawsenza cap guest. Un Windows build mirato warning-free
più un solo filtro Wine passano17guardie/all336Boriginal loadsites/18copy prefix
(13fonti+1variante+4count raw0/-1/-65536/257)/44getter/43idle/64prefixduefiltri32
maschere/2prefixcoordinateflag80-86nonmascherate. EWRAM/IWRAMfuoristack e SP/callee
ritorni uguali; niente intera widget/native/fullstate-clock/ordinario.703guardhash-
mtime;12231istruzioni212PCsolo copertura. Variante tutti8campiu8passaforma e
fallisce autenticazione; copia257preserva overwrite metadata originale. Nessuna
vecchia prova/sourcecensus ripetuta o nuova fixture/navigazione/performance/spazio.
e55-d54/USAe8-d7 invariati. [Modello e prove](../rom_data_ui.md#lossless-model-and-original-wine-readers).

**ROMDATA-03-UI-CHOICE-LISTS-RUNTIME completato nel perimetro mirato EUR/Wine
(2026-10-24):** e56-d55pubblica ui-choice-lists.json/engine.ui-choice-lists,
52filegenerazione51risorse/tutti50precedenti decodificati uguali/e55conservato/
USAe8-d7 invariato. No dipendenza asset, fullsetcoerente. Owner immutabile13span
336Bserve ID/X/Y/opaque/navigationraw a CPU/API native e copie BIOS/DMA; counts/
filtriRAM/copie/input/cursori/task e save-nativeABI invariati. Windows36248unico
build core/frontend warning-free; coretest solo compilati. Wine82471duefiltri
esatti firstPASS17guardie/lifecycle/all336typedB=[42ID84coordinate42opaque168nav]/
13liste/failedattachtransazionale/pubblicazione fallita-create-reuse-repair-move/
e55retention/unavailableLoad-Undo-backup-Restart/sharedowner. Native29percorsi
originale-interprete-compilato fullstate-clock/compiled[92ID183coordinate92opaque
368nav]Baggregati; tutti336newraw+14oldOBJrawBalterati. Counts0/-1/257/RAMcopy/
getter-idle/4prefixfiltri zero-tutti, widths/mirrors/realBIOSbus/3SWI/1DMA passano.
Copiamista2old+2raw+2newB/SWI-DMA e APIword2raw+2new, oldOBJstessapagina4B;
706guardhash-mtime.3010istruzioni186PC38dispatch solo copertura, niente intera
widget/ordinaryABI. Source/model/oldproof non ripetuti; nessuna nuova fixture/
navigazione/performance/spazio. [Consumo e prove](../rom_data_ui.md#managed-publication-and-typed-runtime-reads).

**ROMDATA-04-UI-CHOICE-LISTS-CLI / ROMDATA-05-UI-CHOICE-LISTS-PAIR completati
per il perimetro EUR/Wine dichiarato (2026-10-24):** un Windowsproductionbuild
60923firstPASSsenzaavvisi/frozen444c2db9..; canonico hash/mtime esatti ripristinati,
maincontentinvariato/mtime-onlyCargoinvalidation. CLI36895crea/riusa52filegenerazione
54gestitino-write/tutti52uguali accettati e56/cwd estranea/oldcache. Pecorelle1000
0newUI, Moogle-menu6000newUI: negativiterminali/no fullpairs/estensioni. Ispezione
readonly110snapshotEURsenza copie dirette non prova irraggiungibilità; battlemenu
componente diverso. Screenshot/inputPartyunit esistenti guidano unicoCLI54355
320positivo32Baggregati=[4ID8coordinate4opaque16nav]/8busreads/1lista. Snapshot
worldmap7701a360../inputPartyunitbf757e69.. invariati, nessuna fixture/navigazione
nuova/payloadedit. UnicoWine95582fullpair stessi320frame4519→4839/ogni stato-
clock/immaginecentrale-wide/OAM/dispatch/mixedPSGFIFObitwiseHLEaudio coincide;
audio[702240,702240,84480,84480,702164]. Stesso32Bnuovi/1lista/769guardhash-mtime.
Frozen0f942581testexe/actualCLI DQ1nbu/oldcache/width0, misto native-fallback.
Source/model/managed/native/oldscene non ripetuti; niente allnative/interaWidget/
new32:9. Usoordinario altre12liste pending; verifichemirate chiuse; nuovi Linux/USA/Windowsnativo/32:9
restano pending. [CLI e scena](../rom_data_ui.md#production-cli-and-positive-ordinary-party-scene).

**ROMDATA-01-UNIT-OBJ-INITIAL-POSE / ROMDATA-02-UNIT-OBJ-INITIAL-POSE completati
nel perimetro fonte/modello EUR/Wine (2026-10-24):** originale2D394→233A8→CF144
produce ID da metadata52property4/metadata14halfword04 già migrati;2D3F8→22788
passa fullwordanimation0/part0.110jobroute0+93special proiettano121ID, senza
capguest o catalogofisicocompleto; job0/1valore65535resta irrisolto/inesportato.
Directory394834→primecelle→121sequenze477frame20Boperation1, OAMbaseA50020/
6liste31oggetti→tilecountoriginale221D4→333richieste135168B4bppgrezzi dalla
base898A40.00B14accodaDMA, non decodifica;00800esegueDMA3reale.415span146358B
disgiuntida51owner;94instruction13BL6literal fingerprintsstaticiPASS. Modello
strictlossless/sharedsourceplan/4224tile8x8indicepalette conserva header/tail/
attributiopachi. Windows20506unicofrontendtestbuildwarning-free/frozen1d759b0d;
Wine19197unicofiltro firstPASS121resetoriginali22E20/1constructorprefix22788/
477scheduler→queue→DMAupload/tuttaVRAM/all135168pixelB/16shape-sizeinclusoshape3/
11JSON-authguard/SP-callee/RAMesterna.9393loadsiteBdistinti+135168DMA=144561B;
1797Bheader/tail/OAMpreservatinonletti;245830istruzioni499PCsolocopertura.
703guardhash-mtime; niente fullunitABI/native/fullstate-clock/ordinario,
productionbuild/oldtest/fixture/navigazione/performance/spazio. EUR e56-d55/
USAe8-d7invariati. [Contratto/modello](../rom_data_unit_obj.md).

**ROMDATA-03-UNIT-OBJ-INITIAL-POSE completato nel perimetro mirato EUR/Wine
(2026-10-24):** e57-d56pubblica unit-obj-initial-pose.json/graphics.unit-obj-initial-
pose con dipendenzemetadata52/14;53file52risorse/tutti51precedentidecodificati/
recordmanifestcoperturaidentici/e56conservato/USAe8-d7. Ownerimmutabile restituisce
directory/index/header/frame/OAM da campitipizzati e pixel da indici4bpp; niente
ROMshadow/RAM-scheduler-DMA-VRAM/save-nativeABIalterati. Windows73480unicobuild
core/frontend warning-free; coretestsolocompilati7a95a911../frontendd0995de8..
frozen. Wine19825duefiltriesatti firstPASS17guardie/lifecycle/tutti146358typedB=
[484directory484index484header9540frame198OAM135168pixel]/121sequenze, failedattach
transazionale/pubblicazionefallita-create-reuse-repair-move/unavailableLoad-Undo-
backup-Restart/sharedowner. Native30percorsioriginale-interprete-compilato stato-
clockcompleti/API[28dir20index8header127frame164OAM1pixel]Baggregati; pixelAPIprobe
distinto dai veri executorDMAcompilati. Raw146358new+20oldOBJBalterati, ID65535/
animation65537/liveRAMframe passano.415edges/width/mirrors/BIOSbus/2SWI/1DMA:
copiamista2new+1276raw+8oldOBJconaddebiti2/8; APIword2new+2raweditati; oldOBJsamepage
4Bsenzaaddebitionuovi.706guardhash-mtime/8compiledinputsimmutati;3542istruzioni496PC
177dispatchsolocopertura. Source/model/oldproofnonripetuti; nessuna interaunità/
allocatorABI/ordinario/fixture/navigazione/performance/spazio.
[Consumo e prove](../rom_data_unit_obj.md#managed-publication-and-direct-runtime-reads).

**ROMDATA-04-UNIT-OBJ-INITIAL-POSE-CLI / ROMDATA-05-UNIT-OBJ-INITIAL-POSE-PAIR
completati nel perimetro EUR/Wine dichiarato (2026-10-24):** Windows49887unico
productionbuildwarning-free/frozend4c782c6..; canonico hash/mtimeoriginali ripristinati,
maincontentinvariato/mtime-onlyCargo invalidation. Wine57046treCLI reali create/
reuse-no-write55filegestiti/all53generazioneugualiaccettati/cwdestranea/oldcache.
Actuale57generation-ZV0TqS/ab9286b2..; originalworldmap7701a360../Partyunitinput
bf757e69..320senza nuova fixture/navigazione/payloadedit. Locatorpositivo10962B
aggregati=[8dir16index4header252frame7162OAM3520pixel]/8170busreads/2sequenze.
UnicoWine12785fullpair stessi320frame4519→4839/ogni stato-clock/immaginecentrale-
wide/OAM/dispatch/mixedPSGFIFObitwiseHLEaudio coincide; audio[702240,702240,84480,
84480,702164]. Stessi10962B/3520pixelB/2sequenze/764guardhash-mtime; frozen d0995de8
testexe/actualCLIdataset/oldcache/width0, misto native-fallback. Piccolo preflight
wrapperprocess-exit corretto prima di qualsiasiWine, nessun testfunzionaleripetuto.
Source/model/managed/native/oldscene/build/prepare/locator non ripetuti; endpoint
Partyunit con preview animata240x160ispezionato. No allnative/interaunità-allocator/
new32:9claim. Altre119sequenzeordinary target-only; nuovi Linux/USA/Windowsnativo
restano pending. [CLI e scena](../rom_data_unit_obj.md#production-cli-and-positive-ordinary-party-scene).

**ROMDATA-01-UNIT-OBJ-PALETTE-SOURCES / ROMDATA-02-COLOR-MODE-PALETTES
completati nel perimetro fonte/modello EUR/Wine (2026-10-24):** cinquehelper
CF49C/CF4E4/CF51C/CF5A0/CF5DC e70directcall provano30span5750B=2873u16colors+
4u8slice selectors senza overlap52owner. Copiecomplete512B, supplemental
frammentarie,12Bshared e9palette speciali da51proiezionimetadata14 verificate;
nessuna estensione daadiacenza o capguest.86instruction19literal11BLpins passano.
Modello strictlossless/sharedsourceplan e filtroWinecorretto passano165original
pointer/99copy/30stagedreader/54specialcopy/12slice prefix/tutti5750B/eightguards.
Primofiltrofermo da seedr5/r6errato neltest30632; modello senza differenze;
correzione usa MOV/LDRoriginali. DuebuildfrontendWindows senzaavvisi, nessun
productionbuild o vecchia prova ripetuta.764guardhash-mtime/fourcompiledinputs
intatti;33654istruzioni770PCsolo copertura. EUR e57-d56/USAe8-d7 invariati;
nessun fullstate-clock/native/ordinaryclaim. [Contratto e prove](../rom_data_color_palettes.md).

**ROMDATA-03-COLOR-MODE-PALETTES completato nel perimetro mirato EUR/Wine
(2026-10-24):** e58-d57pubblica color-mode-palettes.json/graphics.color-mode-
palettes con metadata14dependency/54file53risorse/tutti52precedenti decodificati/
recordmanifestcoperturaidentici/e57conservato/USAe8-d7. Ownerimmutabileu16colors/
u8selectors perbyte conserva gapraw/priorowner, liveMode/copie/fades/RAM/save-
nativeABI. Primobuild16832fermo3E0061nei costruttoritestprimaWine; trezeroargomenti
aggiunti, nessun vecchiotest eseguito. Build13589core/frontendwarning-free/core
solocompilato/frozenfrontend6725b65c... Wine12913duefiltriesatti firstPASS17guardie/
lifecycle/tutti5750typedB=[1536unit1024companion2286supplemental36shared864special
4selector]/30frammenti/failedattachtransazionale/pubblicazionefallita-create-
reuse-repair-move/unavailableLoad-Undo-backup-Restartsharedowner. Native28percorsi
originale-interprete-compilato stato-clockcompleti; APIaggregati[1532,1536,158,44,
192,7]B sei categorie. Raw5750new+4oldUIBalterati; low8index65535/mode3/liveRAM/
30edges/width/mirrors/BIOSbus/2SWI/1DMA passano. Copiamista2new+20raw+4oldUIB con
addebitiesatti2/4; APIword2new+2raweditati; oldUI40FCB0stessapagina4Bsenza nuovi
addebiti.767buildguard769runtimeguard/ninecompiledinputsintatti;6805istruzioni
451PC5177dispatchsolocopertura. Niente interoallocator-scheduler/ordinaryclaim;
fonte/modello/vecchieprove nonripetuti. [Consumo e prove](../rom_data_color_palettes.md#managed-publication-and-typed-runtime-reads).

**ROMDATA-04-COLOR-MODE-PALETTES-CLI / ROMDATA-05-COLOR-MODE-PALETTES-LUTIA-PAIR
completati nel perimetro EUR/Wine dichiarato (2026-10-24):** Windows28568unico
productionbuildwarning-free/frozena052914b..; canonico hash/mtimeesatti ripristinati,
maincontentinvariato/mtime-onlyCargoinvalidation. Wine22334CLIcreate/reuse-no-write
56gestiti/all54generazioneugualiaccettati/cwdestranea/oldcache, actuale58generation-
XbOkZs/e7abf8bb.. . Partyunit3200newcolors; neutralboot240Wine468380newcolors:
negativiterminali/no fullpairs/niente nuovi input/fixture. Recordesistente Lutia
maploadguida unicoWine229031700positivo1102Baggregati=[512unit512companion78
supplemental0shared0special0selector]/277busreads/4frammenti. Worldmap7701a360../
worldmap-engage-battle.input invariati, nessuna nuova navigazione/payloadedit.
Nuovo guardtest REQUIRE_COLOR_MODE_PALETTES richiede colori typedpositivi;
Windows46600unicobuildfrontendwarning-free/frozen449a1d2c.. . UnicoWine16883fullpair
stessi1700frame4519→6219/ogni stato-clock/immaginecentrale-wide/OAM/dispatch/
mixedPSGFIFObitwiseHLEaudio coincide; audio[3730650,3730650,448800,448800,3730574].
Stessi1102B/4frammenti/828guardie/tencompiledinputsintatti; vecchi9runtimeinputs
immutati. EndpointLutiaNOTICE240x160ispezionato; mistonative-fallback, niente
allnative/interoallocator-scheduler/new32:9. Fonte/modello/managed/native/vecchie
prove non ripetuti. Altri26frammenti ordinary/altreMode/shared-special-selector
restano targeted-only; nuoviLinux-USA-Windowsnativo pending.
[CLI e scena](../rom_data_color_palettes.md#production-cli-and-ordinary-lutia-palette-consumption).

**ROMDATA-01-UNIT-OBJ-EQUIPMENT / ROMDATA-02-UNIT-OBJ-EQUIPMENT completati nel
perimetro fonte/modello EUR/Wine (2026-10-24):** originale9BA84→CE1EC proprietà
3/9 seleziona252item/61sprite124..184, basi0/4/8, parti0/1; overrideespliciti
185/basi0-4 e124-126/base16 inclusi.748celle/86null/258sequenze3439frame:
1911op1/609op6/888op8/31op0, soloop1produce riferimenti grafici.593intervalli
110666B, senza overlap con tutti53owner e58:248directory2992indici69812sequenze
174OAM37440pixel4bpp/1170tile.73instruction/4BL/4literal/2dispatch statici;
modello tipizzato lossless con dipendenzaitem e nullreali, senza capguest.
Unico buildtestWindows warning-free; unico nuovo filtroWine passa al primo
tentativo10guardie/1496prefixcostruttore/86resetnull/258resetsequenze/seicomandi/
280scheduler-DMA/tutti37440pixelB.828guardieprecedenti intatte, settepreimage
compilati conservati; correzione successiva del solo commento action12/13 non
cambia l'implementazione. Nessuna vecchia prova/build/fixture ripetuta.
Pubblicazione gestita/runtime/scenaordinary ancora dafare, EUR e58-d57/USAe8-d7
invariati. Censimento attori normali/alternativi e script49 resta parziale:
nessun prodotto cartesiano animazionescript×sprite. Linux/USA/Windowsnativo da
fare, nessuna misura prestazionale.
[Contratto e prove](../rom_data_unit_obj.md#equipment-poses-source-model).

**ROMDATA-03-UNIT-OBJ-EQUIPMENT completato nel perimetro mirato EUR/Wine
(2026-10-24):** e59-d58pubblica55file54risorse, unit-obj-equipment.json/
graphics.unit-obj-equipment con dipendenza effettiva item-records; tutti53file/
recordrisorsa/coperture precedenti identici, e58 conservato, USAe8-d7. Owner
immutabile legge campi e indici4bpp per i593intervalli110666B senza ROMshadow,
con nullreali e composizione byteperbyte con owner precedenti/rawholes. Task/
comandi/facing/copieRAM/scheduler/DMA/VRAM/save-nativeABI restano originali.
Unico buildtestWindows core/frontend warning-free, core solo compilato. Due
nuovi filtriWine firstPASS17guardie/lifecycle/tutti110666typedB/258sequenze e
26percorsi originali-interprete-compilato con stati-clock completi uguali,
null/parti0-1/fullwordID-animazione/comandi0-6-8/liveRAMframe/scheduler-DMA.
APIcompilate aggregano[32directory24indici6header47frame32OAM1pixel]B; DMAreale
confrontato separatamente.593edges/width/mirror/BIOS/2SWI/1DMA passano; misto
2new+1028raw+8oldOBJ con addebiti2/8 e APIunaligned2new+2raweditati.831guardie
build/833runtime/11sourceinputintatti.1511istruzioni502PC97dispatchsolocopertura,
nessun interoallocator-scheduler/ordinaryclaim; fonte/modello/vecchieprove non
ripetuti. Identitàdataset2c200038../manifestfileSHAaa8bba05.. distinti; CLI e
positiveordinary ancora aperti. Linux/USA/Windowsnativo/new32:9 dafare; no metriche.
[Pubblicazione e consumo](../rom_data_unit_obj.md#equipment-managed-publication-and-runtime-reads).

**ROMDATA-04-UNIT-OBJ-EQUIPMENT-CLI completato (2026-10-24):** unico
Windows85707productionbuildwarning-free/frozen0795d057..; canonico hash/mtime
esatti ripristinati, maincontent invariato/mtime-onlyCargoinvalidation. Wine87287
actualCLIcreate/reuse-no-write55generazione/57gestiti/tutti55ugualiaccettati/
cwdestranea/oldcache; e59generation-nXJuGC/2c200038.. . Sheep1000locator e
Wine22920Fight700locator sono equipment-negative terminali:0tuttecategorie/
0sequenze, niente fullpair/testguardbuild/ripetizione invariata. Vecchie prove
paletteprogrammiscene non sono prova d'uso dell'equipaggiamento. Precondizione
snapshotspell inizialmente assume wrapperdesktop; effettivo rawFFTA-ST/version1/
EUR/payload verificato senza build/guest precedente. F09preview/core/hashpass.
833runtime/891CLI/893Fightguardie/11sourceinput/tutti7623artifactprecedenti intatti.
Ispezione staticariutilizza211callsitecandidati9BA84, non provavita/definizioni
univoche. dump_frame mostraframebufferguest, non necessariamente nativeUI.
[CLI e limiti ordinary](../rom_data_unit_obj.md#equipment-production-cli-and-ordinary-coverage-limits).

**ROMDATA-05-UNIT-OBJ-EQUIPMENT-SCENE completato nel perimetro EUR/Wine dichiarato
(2026-10-25).** L'utente autorizza esplicitamente checkpoint locali da una copia
EUR tramite soli comandi normali, originali/cartucce invariati e artefatti fuori
Git; precedente rifiuto automatico superato, nessuna modifica RAM/HP/status/PC.
Derivazione F09frame8476→8668in192frame normali conferma Action/Fight/alleata
Hildegard. Due caselle vuote e primoA500che si ferma su Do it sono negativi
chiusi, non ripetuti. Input nuovoAat0/48(release4/52) conferma realmente l'attacco:
actuale61CLI500positivo4477B=8dir16index4header276frame2765OBJ1408pixel/twosequenze.
Solo guardia positiva pixel+sequenze nel testscena; unica buildfrontendWindows
warning-free/frozena13b3aa6..; unica coppiaWine500firstPASS frame8668→9168ogni
stato-clock/immagine centrale-wide/OAM/dispatch/audio, contatori equipment identici
alCLI(3269letture). Fonte/modello/managed/native/actualCLI/ordinary equipment
chiusi, altre256sequenzeordinary restano targeted-only. VecchiSheep1000/Fight700/
source/native/scene/build non ripetuti; e61/USAe8invariati. Restano generale
attori/alternate/script/altreposepalette e nuoviLinux/USA/nativeWindows/32:9;
principale01..05parziale/mod06-07dipendenti. Nessuna misura performance/spazio.
[Scena autorizzata e limiti](../rom_data_unit_obj.md#authorized-ordinary-equipment-attack).


**ROMDATA-01-UNIT-OBJ-NORMAL-BATTLE / ROMDATA-02-UNIT-OBJ-NORMAL-BATTLE completati
per la proiezione EUR/Wine dichiarata (2026-10-24):**121IDnormali metadata52/14+
42esplicito,29basi corpo/8parti extra risolvono10980chiavi/8374celle fisiche/
5293nulle/2581sequenze17894frame.150liste676oggetti e3216richieste4bpp conservano
pixel indicizzati e comandi0/1/2/3/6/7/8. Viste condivise146358Binitial+6984B
equipment verificate; nuovi6021intervalli1455474B, nessun altro overlap nei54
owner correnti/nessun limite guest o estensione per adiacenza. Unica build
Windows senza warning/unico filtro Wine nuovo al primoPASS:12guardie/16474
prefissi costruttore sulle8237celle nuove/5293reset nulli/2451reset sequenze/
100comandi/2858scheduler-DMA/tutti1063200Bpixel nuovi/VRAM e guardie intere.
I controlli originali dei vecchi owner non sono ripetuti. Proiezione finita,
non prova di raggiungibilità di ogni combinazione attore/azione. Il prodotto
indiscriminato con gli IDalternativi è respinto: sprite247/animazione104legge
la directory394834come indice, nessun filtro sul numero frame risolve il legame
mancante. Alternate/script49/streamRAM restano aperti. e59-d58/55file54risorse e
USAe8-d7 invariati; prossimo03pubblicazione gestita/consumo tipizzato, poiCLI/
uso ordinario. Linux/USA/nativeWindows/nuovo32:9 da fare, nessuna misura.
[Contratto e prove](../rom_data_unit_obj.md#normal-mode-battle-poses-source-projection-and-typed-model).


**ROMDATA-03-UNIT-OBJ-NORMAL-BATTLE completato nel perimetro EUR/Wine mirato
(2026-10-24):** e60-d59pubblica/consuma unit-obj-normal-battle.json con dipendenze
reali metadata52/14/initial/equipment,56file55risorse/tutti54precedenti record e
filedecodificati invariati/e59conservato/USAe8invariato. Il lettore interseca le
viste semantiche con i6021nuovi intervalli1455474B; nessun owner duplicato o
shadowgrezzo. Una build Windows senza warning/due nuovi filtri Wine al primoPASS:
17guardie/lifecycle/tutti1455474Btipizzati/2451sequenze e32prefissi originali/
interprete/compilato con stati e clock guest uguali. Parti0/1/2/nulli/IDfullword/
RAMlive/comandi0-2-3-6-7-8/dueupload+dueDMA/tutte6classi API/6021bordi/mirror/BIOS/
SWI-DMA passano. Finestramista2nuovi1036grezzi316initial248equipment8OBJprecedenti
preserva addebiti esatti; API disallineata2nuovi+2grezzi modificati passa. Modello/
sorgente e controlli precedenti restano chiusi/non ripetuti. Prossimi04CLIreale
create/riuso-senza-scritture/vecchiacache e05uso ordinario positivo della nuova
famiglia corpo normale; alternate/script/streamRAM eLinux/USA/nativeWindows/
nuovo32:9restano aperti. Nessuna navigazione/nuovocheckpoint o misura performance.
[Contratto e prove](../rom_data_unit_obj.md#normal-battle-managed-publication-and-runtime-reads).


**ROMDATA-04-UNIT-OBJ-NORMAL-BATTLE-CLI / ROMDATA-05-UNIT-OBJ-NORMAL-BATTLE-PAIR
completati nel perimetro EUR/Wine dichiarato (2026-10-24):** unica buildproduzione
Windows senza warning/canonico hash-mtimeesatto ripristinato; CLIreale crea e
riusa senza scritture56filegenerazione/58gestiti, tutti uguali all'e60accettato,
da cwddiversa/vecchiacache. RegistrazioneF09Wait3530esistente/immutata conmovimento
mostri è positiva:842236Bnuovi=[40index10header26826frame815360pixel]/5sequenze.
Un solo guardpositivo test-only/unica buildfrontend/unico fullpair Wine3530al
primoPASS: ogni stato completo/clockguest/immagine centrale-wide/OAM/dispatch e
campione audio mixed/PSG/FIFO/HLEbitwise uguale; contatori esatti uguali alCLI.
Tutti958guardie/14inputcompilati/preimage/8083artefatti precedenti preservati.
Altre2446nuovesequenze restano mirate/non playthrough esaustivo. Figli source/
model/managed/native/CLI/ordinary chiusi, nessuna ripetizione o nuova fixture/
navigazione/checkpoint/payload/save. Alternate/script/streamRAM rimangono aperti;
05equipmentpositivo poi chiuso il25ottobre dopo autorizzazione esplicita e
singola nuova coppiaFight500.
Linux/USA/nativeWindows/nuovo32:9 da fare, nessuna misura prestazionale/spazio.
[Prove e limiti](../rom_data_unit_obj.md#normal-battle-actual-cli-and-ordinary-equivalence).


**ROMDATA-01-UNIT-OBJ-ALTERNATE-TARGET-VIEWS — contratto statico completato
(2026-10-24), modello/oracolo ancora da fare:**65IDalternativi/5850chiavi,
5682celle fisiche=4709nulle+953con706sequenze2771frame+20vistedi radiciindice
condivise. Tutte20hanno campidirectory/indice initial-pose indipendenti come
testimoni; l'unico overlapinterno4B è247/anim104→directory/rootsprite0. La vecchia
interpretazione blanketcome sequenze rimane respinta: nessun filtro count/cap,
chiave rimossa o limiteperadiacenza.30listeOBJ108oggetti/1203richieste pixel
387008B; nuovi3321intervalli410086B dopo6232initial+52604normal-battle+22equipment
condivisi, nessun altro overlap nei55owner e60. Nuove606sequenze2210frame/5252
celle4428nulle/1079richieste pixel342112B. Contratto eSHA bdf15f7d.. verificati
staticamente; prossimo02modello con targettaggati e dipendenze esplicite, poi
unico nuovo oracolo Wine dei lettori originali. Nessuna nuova generazione/runtime/
scena dichiarata; e60/USAe8invariati. Raggiungibilità attore/azione, script49/
streamRAM/Linux/USA/nativeWindows/nuovo32:9 restano aperti, nessuna misura.
[Contratto e limiti](../rom_data_unit_obj.md#alternate-battle-selectors-and-explicit-shared-target-views).


**ROMDATA-02-UNIT-OBJ-ALTERNATE-BATTLE completato nel perimetro EUR/Wine mirato
(2026-10-24):** modello pubblico lossless con targettaggati null/sequence/shared-
index-root conserva tutte5850chiavi/20vistetestate contro i campi initial-pose,
706sequenze2771frame/30listeOBJ/1203blocchi4bpp.3321nuovi intervalli410086B dopo
validazione dei treownercondivisi; unico overlap4Bdir/indice con testimoneesatto.
Una buildfrontend Windows senza warning/unico filtro Wine al primoPASS:15guardie/
10544prefissi costruttore (40alias condivisi)/4428reset nulli/606reset sequenze/
cinquecomandi nuovi2-3-8/1079scheduler-DMA/tutti342112Bpixel nuovi/VRAMintera.
Nessun falsoframe parsato o schedulato; opcode6e precedenti originali non
ripetuti. Source/modello chiusi; prossimo03pubblicazione gestita/consumo tipizzato,
poiCLI/uso ordinario. e60/USAe8invariati; Linux/USA/nativeWindows/nuovo32:9da fare,
nessuna misura/spazio/performance o nuovo checkpoint di gioco.
[Modello e prove](../rom_data_unit_obj.md#typed-alternate-target-model).


**ROMDATA-03-UNIT-OBJ-ALTERNATE-BATTLE completato nel perimetro EUR/Wine mirato
(2026-10-24):** e61-d60pubblica/consuma unit-obj-alternate-battle.json con cinque
reali dipendenze metadata52/14/initial/equipment/normal;57file56risorse/tutti55
precedenti record/coverage/decodificati invariati/e60conservato/USAe8invariato.
Solo3321nuovi intervalli410086B sono owned; venti radiciindice condivise restano
nonsequenze e gli owner precedenti mantengono la responsabilità. Una buildcore/
frontend Windows senza warning (core solo compilato), due nuovi filtri Wine al
primoPASS:20guardie/lifecycle/tutti410086Btipizzati/606sequenze;34percorsi originali/
interprete/compilato con stati completi e clock uguali, part0sequenza/part1-2nulli/
prefissi sharedroot/IDfullword/RAMlive/comandi2-3-8/dueupload+dueDMA/tutte6API/
3321bordi/mirror/BIOS/SWI-DMA. Finestramista2nuovi780grezzi8OBJprecedenti e API
2nuovi+2grezzi modificati preservano addebiti esatti. Target::Null emptystruct
rifiuta campi extra prima ignorati da serde; JSON valido identico cad7771d..,
nessuna ripetizione dell'oracolo fisico. Tutti8216artefatti precedenti/15input
compilati/preimage/961build+963Wineguard preservati. Figli source/model/managed/
native chiusi; prossimi04CLIreale e05uso ordinario positivo della nuova famiglia.
Raggiungibilità/script49/streamRAM/05equipment positivo eLinux/USA/nativeWindows/
nuovo32:9aperti; nessuna misura performance-spazio o nuovo checkpoint di gioco.
[Contratto e prove](../rom_data_unit_obj.md#alternate-managed-publication-and-typed-runtime-reads).


**ROMDATA-04-UNIT-OBJ-ALTERNATE-BATTLE-CLI completato EUR/Wine;05positivo aperto
(2026-10-24):** unica buildproduzione senza warning/frozend3db6585../canonico
hash-mtimeesatto ripristinato; CLIreale crea/riusa57filegenerazione59gestiti senza
riscritture, tutti uguali all'e61accettato, da cwdestranea. Un frameconcachevecchia
ed autentico snapshotF09immutato passa:0usoalternativo, solo smokeattivazione e
nessuna ripetizione della coppia normale3530già chiusa. Lettura statica/autenticata
81snapshotEUResistenti non trova taskcorrente con sequenzanuovaalternativa che
fornisca un concreto candidato per il gatepositivo. Non esclude usofuturo nelle
registrazioni; nessun locatorlungo/guardscena/buildtest/fullpair o nuova navigazione/
checkpoint/payloadedit. Primohelperstatico confondeva puntatoreguest/offsetfile:
preimage/risultatiinvalidi conservati, corretta solo ispezionestatica economica,
nessuna accettazionegioco invalidata.8543artefattiprecedenti/1023guardCLI/15input
runtimecompilati-preimage immutati. Source/model/managed/native/actualCLI chiusi;
05usoordinario alternativo positivo ancora aperto;05equipmentpositivo poi chiuso
il25ottobre con checkpoint autorizzati e singola nuova coppiaFight500.
Script49/live/nuoviLinux-USA-nativeWindows-32:9 restano aperti, nessuna misura.
[CLI e limiti](../rom_data_unit_obj.md#alternate-actual-cli-and-remaining-ordinary-use-gate).


**ROMDATA-01-SCRIPT-OBJ-ACTOR-PAIRING: classificazione selettori completata,
accoppiamento sprite ancora aperto (2026-10-24).** Decimale49(0x31), callback
126CC0, conserva167istruzioni/18selettori/13basi.151ricerche nelle unitàRAM
(12+24recordstride264/identitàbyte4),16nel registro/fallback vivo;0ID sprite
immediati. La ricercacontext confronta puntatoriunità e usa spritecorrente+34.
Gruppou16×4low16/facingsigned+1F/bodyflag0/medesima baseextra sono originali.
31riusa unitàesistenti senza cambiare byte01;32solo riposiziona, quindi l'ordine
nel file non prova dominanza o spriteimmutato. Script e provenienza esistenti/
67halfword9literal18BL autenticati; nessuna nuovaownership/modello/runtime/
revisione/build/testguest. Prima notazionehex/decimale corretta e un pinBLspostato
alla prima halfword, senza ripetere accettazioni chiuse. Prossima evidenza utile:
sole querysprite/animazione/parte risolte durante49in una registrazioneEUR
immutata, mantenendo collegamentoistruzione/attore/lettore. Nessun prodotto
indiscriminato scripts×sprites. e61/USAe8invariati; nuoveLinux-USA-nativeWindows-
32:9 da fare, nessuna misura o nuova fixture/navigazione/checkpoint.
[Contratto dei selettori vivi](../rom_data_unit_obj.md#script-animation-selectors-require-live-actor-bindings).


**ROMDATA-01-SCRIPT-OBJ-RESOLVED-QUERIES completato per la registrazioneEUR
mirata (2026-10-24):** osservatoresolo-test/registrazione cfg(test), nessun codice
produzione/sourceowner/ABI/save/cache modificato. Primo preflightWine fallisce
prima di snapshot/frame per ipotesi tuttiingressinativi; prova/preimage conservati.
Corretto solo osservatore: forwarding funzione nativa doveesiste, originale
Machine::step dovecachemancava. Buildfrontendcorretta warning-free/frozen4342f2..;
unico censimento effettivo Wine6000frame21420→27420passa con sixhandler49/sixbody
SP-LRlinked/18querycostruttore reali. Seiistruzioni selector16/group2risolvono
sprite46/anim8/parti0-1-2; disponibilitànative[false,false,true], contabilità
strumentata non prova equivalenzadispatch. AuditROM/tutti56owner: corpo giànormal,
duecelle4Bnulle37C760/37C764nonowned,8B/SHAaf5570f5.., nessuna nuova sequenza/pixel.
1025guardie/6input-preimage corretti intatti. Nuovo contratto staticostabile per
queste nullcelle ed altre161istruzioni/associazionistato restano aperti: nessun
selector16→sprite46 universale, modello/risorsa o prodotto scripts×all-sprites.
Nessuna nuova fixture/navigazione/checkpoint/payloadedit/fullpair o ripetizione
vecchiaaccettazione. e61/USAe8invariati; Linux-USA-nativeWindows-32:9da fare,
nessuna misura performance/spazio.
[Query effettive e limiti](../rom_data_unit_obj.md#source-linked-script-query-observation).


**ROMDATA-01-SCRIPT-OBJ-SPECIAL-NORMAL-PAIRS completato come contratto fonte
condizionale EUR (2026-10-25).** CBD24confronta unit+4col selettore; CBAD4nonnull
unit+4>1eCF144legano il suo veroIDlogico metadata14al normalhalfword04;9B824lo
assegna ad actor+34prima dell'override42. Condizioni esplicite selezionateprima
dell'audit: attore mantiene assegnazionenormale della stessaunità, override20clear,
facing0..3. Alternate/42-230/high16/statoattore diverso restano separati, nessun
prodotto gruppi×tuttisprite o riga rimossa. Tutte151istruzionilow→20coppieselettore/
base14IDlogici→18coppie sprite/base12sprite/108query108celle76nulli/32sequenze241
frame/11OBJ64oggetti/95richieste pixel35968B. Audit56owner:4780initial+30586normal
condivisi e zeroaltrooverlap/conflitti/itemmisti/estensionivarianti. Nuovi33span
6436B/SHA6c3ef9c9..=[240index28header920frame5248pixel],60celle53nulle/sequenze7
frame46(tuttiop1)/17richiestepixel. Nullcelle37C760/64osservate incluse nel piano
ROMstabile, nessundatasetdipendentedal save.27halfword/1literal/3BLpins passano,
nessun build/testguest. Contratto01chiuso; prossimo02modello lossless con151refs/
20testimoni/108chiavi e solo nuovo oracoloWine60celle/7reset/17upload, poi03/04/05.
Vecchieaccettazioni/censimento6000nonripetuti; e61/USAe8invariati, Linux-USA-
nativeWindows-32:9da fare, nessun performance/spazio/fixture/navigazione/checkpoint.
[Contratto e limiti](../rom_data_unit_obj.md#conditional-normal-script-poses-for-special-unit-selectors).


**ROMDATA-02-SCRIPT-OBJ-SPECIAL-NORMAL completato EUR/Wine mirato (2026-10-25).**
Modello lossless20testimoni metadata14logici/151refs script/108chiavi con formati
fisici condivisi; valida initial e normal indipendentemente anche su intervalli
sovrapposti, sottraendone l'unione. Full41802B/SHAf64755ef.. e nuovi33span6436B/
SHA6c3ef9c9.. uguali al piano indipendente. Unica buildfrontendWindows senza
warning/frozen2eaa5653..; unico nuovo filtroWine firstPASS18guardie/120prefix/
53resetnull7proper/17scheduler-DMA3/all5248newpixelB/wholeVRAM. Nuoviframe tuttiop1,
nessun replay dei comandi2/6/8già provati; vecchie accettazioni/censimento6000chiusi.
Solo registrazioni moduli e nuovi file, e61/USAe8invariati. Restano managed/runtime/
actualCLI/ordinary e condizioni alternate/42-230/high16/statoattore; nuoveLinux/
USA/nativeWindows/32:9da fare. Nessuna misura performance/spazio. Dopo la chiusura
memoria, derivazione ordinary equipment poi conclusa con autorizzazione esplicita
ed unica coppiaFight500positiva; pipeline03script-special prosegue sotto.
[Modello e prove](../rom_data_unit_obj.md#typed-normal-script-special-pose-model).


**ROMDATA-03/04-SCRIPT-OBJ-SPECIAL-NORMAL managed/runtime completati nel
perimetro EUR/Wine mirato (2026-10-26).** e62-d61pubblica58file57risorse con
metadata14/scriptarchive/initial/normal,151refs20testimoni logici autentici;
tutti56record/file/coperture precedenti ed e61 conservati, USAe8-d7. Ownerimmutabile
serve solo33span6436B typed, condivisi initial/normal agliowner precedenti, senza
ROMshadow/capID/riscrittura task/ABI-save-native. Unica buildWindowscore/frontend
warning-free(core solo compilato); due nuovi filtriWine firstPASS23attachguardie+
6dipendenze/lifecycle/tutti6436typedB/sequenze7 e23percorsi originali-interprete-
compilato con stato-clock completo uguale. Part0proper/part1-2null/twoorientations/
fullwordrawID/liveRAM/two uploads-DMA/all33edges/mirror/BIOS/SWI-DMA passano;
nuoviAPI[0dir36index6header43frame0OBJ1pixel]B, misto2new4normal esatto e API
2new2raweditati. Tutti41802graphB avvelenati, vecchi comandi2-6-8/source/censimento/
scene non ripetuti. Datasetf131b439../manifestfile2b84c40c../modelfile02e7362e..
con identità distinte.04CLIproduzione reale poi concluso sotto; restano05ordinary
source/nuovepixel della famiglia e condizioni alternate/override/highregistry/
statoattore; Linux-USA-
nativeWindows-new32:9 da fare. Nessuna misura performance/spazio/nuovafixture.
[Pubblicazione e runtime](../rom_data_unit_obj.md#conditional-script-special-normal-managed-runtime).


**ROMDATA-04-SCRIPT-OBJ-SPECIAL-NORMAL-CLI completato EUR/Wine (2026-10-26).**
Unica buildproduzioneWindows warning-free/frozen6455958e..; canonico esattamente
ripristinato(hash+mtime), maincontent invariato/mtime-onlyCargoinvalidation.
ActualWine26817create/reuse58generation60managed/all58ugualiaccettati/no rewrite,
cwdestranea, generation-SHz1vX/f131b439.. . Un frameF09immutabile/vecchiacache
caricaowner mazero nuovibyte: solosmokecompatibilità, nessun05ordinary/nuovipixel/
fullpairclaim. Vecchieprove3530/500/census6000/source/native/CLI/scene non ripetute.
05source-use/newsequence/pixelordinary ancora aperto: Snow6000ha due celle nulle
nuove effettive, è un lead e non prova nuovipixel. Linux-USA-nativeWindows-new32:9/
altri casi fonte restano da fare; nessuna misura o nuova fixture/navigazione.
[CLI e limiti](../rom_data_unit_obj.md#conditional-script-special-normal-actual-production-cli).
