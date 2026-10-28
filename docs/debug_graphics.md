# Ispettore grafico — DBG-12

Nella [copia isolata in pausa](debug_control.md), **Debug > Capture graphics
report** produce un report HTML locale. Dalla CLI:

```powershell
.\recomp.exe debug-control graphics 'target\caso'
.\recomp.exe debug-control graphics 'target\caso' --compare-last
```

La risposta indica `status.graphics_report`, relativo alla copia. Aprire
`Data/config/graphics/<run>/index.html`. Il report funziona offline e conserva
JSON, PNG e gli stessi buffer grezzi dei precedenti dump PPU: VRAM, OAM,
palette e I/O backing, più framebuffer centrale/wide RGB555. Nessuno snapshot
aggiuntivo, patch guest o modifica ai registri. Lo stato completo viene controllato
prima e dopo; la copia deve essere ferma senza passi, ricerca o replay in corso.
Non serve la feature audit. Il runtime normale non espone questo canale.

## Output salvati e proiezioni

La prima vista confronta framebuffer centrale, wide attiva e composizione frontend.
Il campionatore CPU riusa il percorso delle anteprime snapshot: vista/crop, LUT,
risposta temporale e UI native alle dimensioni della finestra. Il report include
la stessa vista senza UI, conteggio/rettangolo dei pixel cambiati e rettangoli
UI/clip. Sono esclusi menu pausa, FPS e gli effetti finali GPU del display
(griglia/filtro/scaling). È una composizione diagnostica esplicitamente dichiarata,
non una cattura del monitor o una prova della latenza di presentazione.

Le altre viste derivano dai **registri e buffer congelati correnti**:

- BG text: proiezione nativa 240×160 con gli helper readonly già usati dal renderer;
- BG affine: mappa tile grezza, senza inventare i riferimenti delle scanline passate;
- bitmap: pagina corrente nei modi 3–5;
- OBJ: proiezione readonly del renderer, inclusi gli oggetti host correnti;
- finestre: maschere BG0–3/OBJ/effetti dai registri correnti e proiezione OBJ-window;
- palette, tile BG/OBJ 4/8bpp e texture OAM senza trasformazioni.

Un framebuffer salvato può comprendere un frame parziale o palette cambiate tra
scanline. La proiezione congelata non ricostruisce tale storia. Il report mantiene
questa distinzione anche quando le immagini sembrano uguali.

## Selezionare un pixel

Cliccare una tile, una voce palette o una proiezione BG mostra gli offset backing
VRAM/palette, entry della mappa, flip, banco e indice. Nelle texture OAM sono
espliciti slot, attributi, coordinate hardware, dimensioni, matrice affine,
priorità e owner host registrato. La texture è grezza: non applica flip o matrice;
il relativo pixel è un texel della sorgente, non una posizione sullo schermo.

Nel framebuffer finale o nell'OBJ composto, il vincitore e la sorgente ROM restano
**sconosciuti**: il colore uguale da solo non prova la provenienza. Non si collega
una tile VRAM a una risorsa ROM senza evidenza di trasferimento. Le coordinate
hardware OAM sono distinte dalle eventuali coordinate estese degli oggetti host.
L'owner host è metadato del renderer, non ownership ROM.

Gli indirizzi dimostrati riguardano solo la decodifica congelata dichiarata.
I percorsi typed realmente letti restano nell'[audit DBG-06](debug_rom_audit.md);
le differenze di esecuzione fra backend restano nei [confronti DBG-09](debug_comparisons.md).

## Confrontare e conservare

`--compare-last` seleziona l'ultimo report completo della stessa copia, autentica
ROM/schema e hash dei suoi buffer/immagini e pubblica un nuovo report. Mostra
immagini affiancate e differenze esatte nei byte di VRAM/OAM/palette/I/O/centrale/
wide, con i primi 16 offset. Gli offset sono nei buffer indicati, non nella ROM.
Con geometria, crop o impostazioni differenti le immagini non sono allineate;
le differenze nei byte restano dichiarate senza trasformazioni automatiche.

Il primo confronto senza un report precedente viene rifiutato. Un payload
precedente alterato viene rifiutato, senza sovrascrivere report già conservati.
Al massimo **otto cartelle per copia**, comprese le catture incomplete: niente
pruning automatico o sostituzione. Gli errori lasciano gli artefatti per diagnosi;
`graphics.json` viene pubblicato per ultimo. Un `status.graphics_report` precedente
resta una cattura precedente quando un nuovo comando fallisce: leggere `ok/error`
e l'identità della richiesta corrente.

Le catture frontend accettano finestre fino a **1600×1000**; dimensioni maggiori
vengono rifiutate senza ridimensionare di nascosto la UI. Si conservano soltanto
la porzione attiva wide e le proiezioni necessarie, con dati JSON limitati a 2 MiB.
ROM, immagini, palette e dump sono privati: restano ignorati da Git e fuori dai
pacchetti. Spostare l'intera copia preserva i collegamenti relativi fra report.

## Caso verificato: pannello DS-20 e ritaglio wide

Nella copia DS-20 al frame guest **22220**, il pannello «Vuoi fuggire dalla
battaglia?» permette di distinguere il canvas UI dalla sua area visibile.
Con finestra **852×480**, il canvas sorgente 480×160, ingrandito 3×, ha rettangolo
`[x=-294, y=0, w=1440, h=480]` e clip `[0,0,852,480]`. L'origine negativa
comprende il padding trasparente del canvas wide; da sola non dimostra che il
testo del pannello sia tagliato.

Il confronto fra composizione frontend e vista senza UI misura **89.856 pixel**
modificati nel rettangolo `[114,48,738,192)`, largo 624 e alto 144: il contenuto
effettivo del pannello rientra nel clip. La vista affiancata, i rettangoli e il
conteggio sono conservati nello stesso report; i byte PNG sono verificati anche
indipendentemente dai metadati. Per riprodurre l'osservazione, aprire il seed
DS-20 scelto in una copia isolata con `--paused`, avanzare un frame con DBG-10,
poi catturare due report, il secondo con `--compare-last`, mantenendo vista e
impostazioni del caso. L'identità del seed e del runtime resta nel manifest
privato: un altro salvataggio DS-20 non implica lo stesso punto iniziale.

La cattura ripetuta senza passi intermedi conserva lo stato completo e produce
zero differenze nei sei buffer grezzi. Nella proiezione BG0, il pixel `(65,10)`
è decodificato dal backing VRAM `0x06002091`; nel browser il colore/alpha coincide
con il pixel del PNG generato e con palette/mappa dichiarate. Questo dimostra
la decodifica della proiezione congelata, senza attribuire quel pixel al vincitore
del framebuffer o a una sorgente ROM.

## Verifiche e limiti

Validazione Windows cross-build/Wine/Xvfb del 2026-10-28:

- Build release locked/offline con e senza audit, entrambe senza warning;
  **58 test distinti**: 3 report, 2 PPU, 6 controllo, 1 anteprima, 46 menu.
  Prima della GUI è stata distinta la codifica RGB555 salvata (0xffff bianco)
  dalla trasparenza delle proiezioni; ripetuti solo i 3 test report interessati.
- **21 casi CLI** e GUI: cattura dal menu, confronto identico e dopo un passo,
  assenza di precedente, payload precedente alterato, limite otto cartelle,
  rifiuto durante esecuzione/dopo chiusura. Cattura e rifiuti preservano stato,
  salvataggi, progressi e report già conservati. La copia senza audit produce
  gli stessi hash grezzi e stato della copia con audit.
- Cinque report verificati: hash/forme dei buffer, immagini centrale/wide
  confrontate pixel per pixel con RGB555, attributi OAM e differenze esatte;
  composizione UI e bbox ricontati da PNG. Browser desktop 1400px e mobile
  620px verificano immagini affiancate, selezione pixel/palette/tile/OAM/finestre
  e assenza di overflow orizzontale; screenshot ispezionati.
- Tutti i 202 input originali selezionati e i runtime normali Windows/Linux
  mantengono hash e mtime. Entrambe le copie terminano correttamente, senza
  nuovi snapshot. Profili Chrome temporanei rimossi; report e diagnostica
  conservati in `target/debug-tools-graphics/`, ignorato da Git.

Windows nativo, Linux nativo/Wayland, controller fisici, cattura da un runtime
native, altre scene e la matrice completa dei modi grafici restano da verificare.
La composizione diagnostica CPU non certifica gli effetti finali GPU o il monitor.
Le proiezioni correnti non ricostruiscono effetti fra scanline o la provenienza
ROM dei trasferimenti. Il renderer guest e il campionatore originale delle
anteprime restano preservati. Nessuna nuova accettazione ROM/DS/native deriva
da questa verifica. [Record della modifica](changes/2026-10-28-add-readonly-graphics-inspector.json).
