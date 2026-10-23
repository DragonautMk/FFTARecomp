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

**ROMDATA-01-GAMEPLAY-ITEM-ROOTS: inventario statico dichiarato completato,
esecuzione originale ancora da fare.** Lettore proprietà080ce1ec/tabella6e0bd0,
376record×32byte/12032byte fino al root indipendente6e3ad0. Chiamanti negozio/
classificatore e enumerazione originale1..361/362..375confermano i limiti;
19proprietà espongono halfword/byte/signed e modificatori live02002030.
Tutti32byte per record conservati, nessuna sovrapposizione e22; semantiche non
provate non inventate. Nessun nuovo modello/estrattore/consumo/esecuzione/build/
test rivendicato. Prossimo criterio: catalogo tipizzato/oracolo originale Wine,
poi gestione e consumo; tabella seguente20byte, altre famiglie e Linux/USA aperti.
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
precondizioni di schedulazione ancora ignote. Le sette ulteriori liste dirette
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
con cache precedente, immagine ispezionata. Tre sequenze vuote precedenti,
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
collegata e tre vecchi contratti di sequenze vuote restano aperti.
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
Altri payload/palette OBJ, tre contratti vuoti e Linux/USA restano aperti;
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

**Prossima azione:** ampliare le sorgenti grafiche indirette/localizzate ancora
non migrate, a partire dal selettore originale del banco Clan43bc28in08ed28
e dai chiamanti adiacenti. Il catalogo letterale con conteggi derivati da r0
è ora concluso per il perimetro dichiarato; conservare le sue prove.
Il catalogo raw UI di21palette è ora concluso per il perimetro dichiarato;
altre fonti indirette restano aperte. Conservare senza ripetizione
il loro decoder/consumo/scena400frame verificati. Il catalogo
statico esteso a 73 liste e i sedici banchi OBJ hanno concluso le proprie
prove 01/02/04 e scena/CLI 05 dichiarate: conservarle senza ripetizione.
La coppia a conteggio zero b4d8e8/b4f4e8 è ora migrata e verificata su Wine;
anche i costruttori in38008/88328sono ora risolti nelle cinque coppie localizzate. Radici testuali dinamiche e scena tastiera nome
ordinaria restano gate separati. Font completo/tastiera01/02/04e transizione
font/CLI05sono conclusi nel perimetro dichiarato: riutilizzare le prove.
Test ridotti solo Wine/EUR; nuovi controlli Linux e USA da fare. Restano mappe,
font Unicode/semantica controlli, precondizioniFFglobali, audio/gameplay,
GUI/archivi e fasi mod. Nessuna misura prestazionale o lavoro sui limiti spazio.
