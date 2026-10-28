# Catalogo dei casi di debug — DBG-03

Il browser legge il `catalog.json` della raccolta locale esistente. Menu e CLI
usano la stessa proiezione: nome, regione, slot sorgente, variante, provenienza,
stato registrato, ambito della prova e integrità del file. La raccolta e le sue
classificazioni non vengono riscritte.

**DS è una casistica; IN e GEN sono fonti; lo slot è una destinazione distinta.**
Un checkpoint DS-20 può essere importato nello slot 2 di una copia. Gli alias di
una variante possono condividere byte, senza diventare una nuova prova DS.
DS-03 e DS-15 rimangono non coperti quando non hanno checkpoint.

## Dal gioco

Per usare direttamente la raccolta del progetto:

```powershell
.\recomp.exe play 'C:\Giochi\FFTA EUR.gba' --no-bios --interp `
  --debug-catalog 'C:\FFTARecomp\debug_saves\ready\catalog.json'
```

Aprire **Debug → Debug catalog**. Senza il flag, il menu cerca
`debug-catalog/catalog.json` nella directory di configurazione; in modalità
portable è `Data/config/debug-catalog/catalog.json`. Se manca, apre il selettore
JSON. Per cambiare file o ricontrollarlo usare **Choose / reload catalog**.
Copiare una raccolta significa includere anche i file a cui il JSON si riferisce.

Up/Down sceglie la riga. Left/Right oppure Confirm cambia il filtro o la voce:

- **Region** parte dalla ROM attiva; All include le altre regioni. I casi senza
  checkpoint rimangono visibili anche nel filtro EUR o USA.
- **Recorded status**, **Kind** e **DS case** filtrano le voci senza cambiare lo
  stato delle prove. Il filtro DS non interpreta IN-20 come DS-20.
- **Entry** scorre i risultati. Il pannello mostra anteprima quando disponibile,
  provenienza, slot sorgente e ambito della prova. Un legacy raw senza anteprima
  mostra No preview; non viene fabbricata una cattura.
- **Open isolated case** crea e apre una [copia portable](debug_sessions.md).
  I checkpoint ripartono dallo snapshot selezionato; IN/GEN standard partono dal
  normale boot con quella SRAM. La ROM e il BIOS devono essere compatibili;
  un dataset già attivo viene copiato usando le verifiche ordinarie.
- **Import to free snapshot slot** è disponibile dentro una copia isolata.
  Copia il checkpoint esatto nel primo slot libero e apre Save states sul nuovo
  slot. La partita non viene caricata automaticamente: scegliere Load state.
  L'importazione è annullabile con Undo last operation, come un nuovo salvataggio.

La finestra normale resta in pausa mentre si usa la copia. Nella copia, per
aprire un'altra sessione si deve prima chiudere quella corrente; si possono
invece importare altri checkpoint compatibili negli slot liberi.
Da quel punto **Debug → Start input recording** avvia una
[registrazione da snapshot](debug_recordings.md) con frame relativi ed endpoint
verificabile; il catalogo e lo stato delle casistiche rimangono invariati.

Ogni copia aperta dal catalogo conserva `catalog-entry.json` con la provenienza e
un `debug-catalog.json` locale che riferisce il seed già presente. La voce mantiene
la chiave originale anche dopo uno spostamento e non richiede una terza copia
dello snapshot. Il menu può quindi riaprire il catalogo della voce selezionata
senza ritrovare la raccolta esterna. Choose / reload permette di scegliere ancora
la raccolta completa. Per un salvataggio standard, la voce locale verifica la
copia SRAM corrente: se il gioco la modifica, la differenza dal checksum iniziale
rimane visibile.

## Dalla CLI

Il JSON versionato contiene `schema = 1`, `kind = ffta-debug-catalog` e le voci
filtrate. Esempi PowerShell:

```powershell
.\recomp.exe debug-catalog list '.\debug_saves\ready\catalog.json' `
  --case DS-20 --region EUR --kind case --json

.\recomp.exe debug-catalog list '.\debug_saves\ready\catalog.json' `
  --status non_coperta --json

.\recomp.exe debug-catalog open '.\debug_saves\ready\catalog.json' `
  --entry 'DS-20/EUR/20' --rom 'C:\Giochi\FFTA EUR.gba' `
  --dir 'C:\Debug\DS-20' --no-bios --interp --launch
```

`open` richiede una destinazione nuova e una sola voce. Senza `--launch` prepara
la copia e restituisce la cartella senza eseguire il gioco. `--bios FILE` e
`--no-bios` sono alternativi. `--dataset DIR` copia un dataset autenticato opzionale.
Una selezione DS può includere più regioni o varianti: usare la chiave esatta
`--entry`, riportata da list, per eliminare l'ambiguità.

Per importare in una copia **chiusa**:

```powershell
.\recomp.exe debug-catalog import '.\debug_saves\ready\catalog.json' `
  --entry 'DS-20/EUR/20' --session 'C:\Debug\DS-20' --json
```

Il risultato indica lo slot scelto. `--slot N` sceglie una destinazione esplicita,
sempre fra 1 e 99 e sempre libera. La CLI usa ROM e BIOS fissati nella sessione;
non accetta override di avvio. Oltre lo slot 9, il risultato richiede Extended
saves: abilitarlo dal menu della copia. Non viene alterata la configurazione
per un'importazione CLI. Una sessione aperta è protetta dal suo lock esclusivo.

`list` senza `--json` stampa un riepilogo. Un file mancante o modificato è
riportato nelle voci con `integrity = invalid`, `available = false` e la causa;
la CLI termina con codice 1 se i risultati selezionati contengono tali errori.
Un caso non coperto, senza file per definizione, non è una corruzione. Le altre
operazioni terminano con codice 1 su incompatibilità, ambiguità o slot occupato.
Linux usa gli stessi argomenti con `./recomp`.

## Integrità e limiti

Il caricamento accetta il catalogo privato schema 1, con un massimo di 4 MiB e
1024 voci. I soli percorsi `file` vengono aperti, all'interno della raccolta;
i percorsi di provenienza sono metadati. Per gli snapshot si controllano anche
involucro, anteprima e checksum del payload. Apertura e importazione ricontrollano
i byte e usano il validatore completo con ROM/BIOS selezionati prima di scrivere.
Uno slot occupato o creato contemporaneamente non viene sovrascritto.

`available` e `integrity` indicano disponibilità e integrità dell'artefatto;
non dimostrano da soli compatibilità completa o raggiungibilità di una scena.
Lo stato registrato e il suo ambito restano storici. Non vengono aggiornate
spunte DS, inventati checkpoint per DS-03/15, riparati checksum o modificati RAM,
flag, inventario o file sorgente. Cataloghi, copie e prove restano privati e fuori
da Git e pacchetti, come la raccolta originale.

## Verifiche del 28 ottobre 2026

Build release Windows offline su base pubblica congelata `2fbf9f7`, con i soli
file degli strumenti sovrapposti e cache Cargo separata. Il candidato locale è
`target/debug-tools-catalog/recomp.exe`, SHA-256
`d55a7e115f9fd1ca924e6f1b2f3e709818df8b24ea4a194bda69eedf2b3dfccd`.
Gli eseguibili normali Windows/Linux e il lavoro di estrazione concorrente sono
preservati; il candidato non è installato al loro posto.

- **61 test sotto Wine:** 8 del catalogo, 43 del menu e 10 degli slot. Coprono
  filtri e stati regionali, identificatori distinti, checksum/involucri, percorsi,
  compatibilità, importazione esatta senza caricamento, Undo e rifiuto di overwrite.
- **15 casi CLI reali:** 92 voci della raccolta, selezione DS-20 EUR, apertura
  portable, importazione libera ed esplicita nello slot 99; rifiuti di ambiguità,
  slot occupato, standard usato come snapshot, caso senza file e file modificati.
- **GUI Wine/Xvfb:** selezione DS-20, rifiuto di importare nella partita normale,
  avvio della copia con seed identico, catalogo locale, importazione esatta nello
  slot 2, anteprima e Load esplicito, chiusura della copia e ritorno al genitore.
  Stato/clock/progressi del genitore e hash/mtime dei 140 input protetti invariati.
  Una prima navigazione errata del collaudo è conservata con i log; la correzione
  riguarda il solo script locale. Le schermate finali sono state ispezionate.

Risultati locali: `test-results.json`, `cli-results.json`, `cli-summary.json` e
`gui-results.json` sotto `target/debug-tools-catalog/`. Windows nativo, Linux,
controller fisici e gameplay native nella copia non sono verificati da questa
unità. Nessuna nuova accettazione DS, prova di performance o estrazione ROM.
