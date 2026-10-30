# Mappa della copertura ROM

La [mappa navigabile](rom-coverage/index.html) rappresenta tutta la ROM EUR da
16 MiB per offset, con selezione di celle da 64 KiB, salto a un indirizzo,
selezione delle famiglie, prove collegate e maggiori regioni non classificate.
Aprire il file HTML in un browser; funziona offline, senza server o dipendenze.
I [dati JSON](rom-coverage/coverage.json) contengono gli intervalli esatti,
mentre il colore delle celle ne riassume la densità.

La verifica **DBG-07 del 28 ottobre 2026** usa la fotografia EUR **e76-d75**
già autenticata, senza nuova estrazione:
SHA-256 ROM `8fab1e390cb9d5e695bfdb86e81246892e1aa8c0cb4014db8235ff76538ea632`,
manifest `bd0881e0c16abbf9602a1faefd2243defbd64be3c884b37962b24081060ddf3b`.
L'estrattore committato durante la verifica è e79-d78: la pagina segnala quindi
esplicitamente il **dataset storico**. Il percorso predefinito continua a seguire
l'ultimo commit; generazione del dataset e revisione delle prove restano distinte.

- **Codice candidato:** 260.472 B, **1,55%**, da 28.455 ingressi ROM conservati;
  404 ingressi RAM esclusi. Non misura codice eseguito o decompilazione completata.
- **Dati identificati e disponibili al runtime:** 10.325.073 B, **61,54%**,
  in **68 famiglie**. Ogni file decoded e intervallo ROM è verificato per hash;
  il runtime resta opt-in. Nessun residuo solo sorgente in questa fotografia.
- **Consumo ordinario positivo:** **51 delle 68 famiglie**, ereditate dalle
  prove e61 solo a parità di file decoded e sorgenti. Tredici famiglie hanno prove
  da revisionare; una nuova mappa non promuove automaticamente i record storici.
  Gli intervalli delle famiglie positive misurano 9.789.234 B, non byte letti.
- **Audit importati:** tre report esistenti di **DS-20-EUR**, unione di
  **31.061 B osservati**; l'audit lungo è **TRONCATO**. I due brevi completi
  osservano entrambi **12.416 B**. Selezionando il solo breve typed: 11.495 B
  Typed e 997 B Raw, con 76 B sovrapposti. Non sono nuove scene validate.
- **Dati condivisi per riferimento:** almeno 1.359.934 B, **8,11%**, in
  **294 intervalli**. La lettura comune fra casi DS diversi è un altro livello;
  tre report del solo DS-20 non dimostrano condivisione fra scene diverse.
- **Non classificato:** 6.253.479 B, **37,27%**, dopo dati, header e candidati
  codice. Non significa spazio libero o inutilizzato. Gli audit non lo riducono.

Le categorie **non vanno sommate**. Restano 24.120 B con più famiglie proprietarie
ed emergono **61.996 B di sovrapposizione codice/dati**, da esaminare come
conflitti di classificazione. L'analizzatore può interpretare dati come
istruzioni. Il database distribuito contiene zero funzioni per questa ROM;
non si deducono estensioni dal simbolo successivo. I 188 B dell'header tecnico
`0x4..0xc0` restano separati dal branch iniziale.

## Interpretazione delle prove

[evidence.json](rom-coverage/evidence.json) associa ogni famiglia ai record
versionati e conserva i limiti sui campi realmente osservati. Le prove possono
appartenere a generazioni precedenti i cui file sono stati mantenuti: non viene
inventata una singola sessione con tutte le famiglie attive. File nuovi o
modificati entrano nella mappa come autenticati, con prove ordinarie da revisionare;
non bloccano l’aggiornamento dell’HTML. Il registro conserva gli hash sia dei file
decoded sia delle mappe delle sorgenti delle 56 risorse revisionate e61. Ad esempio, il
template di 48 B registra 96 source B in una scena: sono riletture, non una
copertura di 96 byte distinti. Anche output decompressi e byte encoded hanno
unità diverse e non sono addendi della copertura ROM.

Il livello blu della mappa rappresenta **gli intervalli delle famiglie con uso
positivo**, non i singoli byte letti. Anche quando una famiglia è positiva,
altri record, campi o lingue possono essere ancora aperti. I selection gates
conservano separatamente il proprio oracle mirato (831 field B distinti su
sorgente reale), la successiva integrazione runtime e la nuova prova ordinaria
dispatch da 1.000 frame (277 source B, primi 26 record fisici). Quest’ultima
non dimostra lettura di tutti i 1.024 B o dei 64 record.

La condivisione censita è un minimo. Le sovrapposizioni di proprietà tra font,
glyph o banche OBJ sono calcolate separatamente; non dimostrano da sole quanti
consumer le leggano. Non sono ancora enumerati tutti i riferimenti testuali,
dinamici e indiretti. Il nuovo censimento audio resta limitato ai collegamenti
song/header e tone/wave: non esaurisce sequenze, banche o altri grafi. I blocchi copiati in RAM non vengono interamente
colorati come codice ROM solo perché ne è noto il contenitore misto.

## Scene, letture e confronto (DBG-07)

I report `rom-audit.json` di [DBG-06](../debug_rom_audit.md) si importano
esplicitamente; non viene scandita la raccolta privata e non serve rieseguire
il gioco. I checkbox della pagina scelgono gli audit da visualizzare, anche
singolarmente per distinguere raw/typed o una finestra troncata. Il clic sulla
mappa e il salto per offset mostrano famiglie, intervalli nella cella, casi DS,
prove storiche e collegamenti all'identità/limiti dell'audit. La colonna delle
famiglie conta soltanto i byte osservati nella selezione corrente.

I livelli restano separati:

- `runtime_available`: intervalli autenticati delle risorse disponibili al runtime;
- `ordinary_resource_extents`: intere famiglie con una prova storica positiva;
- `scene_observed`: unione fisica delle letture conservate negli audit scelti;
- `scene_typed`, `scene_raw`, `scene_unclassified`: servizio dichiarato dai collector;
- `scene_shared`: byte osservati da almeno due **casi DS distinti**, non due run
  dello stesso caso; `shared_references` continua a censire i riferimenti statici;
- `scene_outside_identified`: letture esterne ai dati identificati, comprese
  eventuali sovrapposizioni con candidati codice, senza inventare proprietari.

Le unioni deduplicano riletture, larghezze e mirror. Bus/host, Typed/Raw,
risorse e scene possono sovrapporsi: non sommare i loro contatori. Un filtro
che interseca una lettura conserva la larghezza intera della lettura reale.
`ordinary_distinct_read_bytes` resta N/D: gli audit selezionati non sostituiscono
l'inventario mancante delle vecchie prove ordinarie.

Il generatore ricalcola le unioni dagli eventi, verifica ROM, schema, intervalli,
mirror, collector, filtri, frame e aritmetica richieste/scarti. Attribuisce le
letture agli intervalli del **manifest della mappa**; un piano noto diverso resta
solo provenienza. Se coincide, verifica anche le intersezioni dichiarate.
L'hash del report identifica il contenuto importato; esito e run sono dichiarati
dal collector, senza autenticare retroattivamente una nuova accettazione di scena.
Non incorpora eventi, byte del gioco o percorsi host nel risultato.

Limiti: massimo 32 report, 32 MiB per file e 128 MiB complessivi; schema audit 1,
sei reader noti e massimo 50.000 eventi per audit. `TRONCATO` e `PARZIALE` restano
visibili anche con replay riuscito. La finestra completa riguarda solo il thread,
i collector e i filtri dichiarati: mancano altri accessi host/HLE, altri thread,
costanti native, letture di autenticazione e altri percorsi elencati nel report.
I fetch interprete sono esclusi; il contesto PC può essere ereditato da HLE/DMA.
Assenze, scarti e hook mancanti non dimostrano byte mai letti.

Ogni generazione confronta il precedente `coverage.json` presente nell'output.
`--compare` / `-ComparePath` sceglie una fotografia diversa della stessa ROM.
La pagina mostra date, generazioni, commit, manifest e hash del report precedente,
più byte aggiunti/rimossi per livello, anche quando il totale non cambia.
Una selezione di audit diversa è segnalata: un delta osservato non è
avanzamento dell'estrazione. Un confronto esplicito incompatibile fallisce
preservando l'output; un vecchio output non confrontabile viene segnalato e
sostituito da una nuova fotografia valida. Il JSON passa a **schema 3**.

Esempio (sostituire i percorsi con i propri report):

```powershell
.\scripts\windows\rom_coverage_docker.ps1 `
  -AuditPath 'target/caso-a/Data/config/replay/rom-audit.json','target/caso-b/Data/config/replay/rom-audit.json' `
  -ComparePath 'target/fotografia-precedente/coverage.json' -OpenReport
```

```bash
bash scripts/linux/rom_coverage_docker.sh \
  --audit target/caso-a/Data/config/replay/rom-audit.json \
  --audit target/caso-b/Data/config/replay/rom-audit.json \
  --compare target/fotografia-precedente/coverage.json
```

Audit e confronto vengono montati in sola lettura a runtime e restano fuori dal
contesto Docker. Python non serve sull'host. Senza questi argomenti si genera
una mappa **senza audit**: non si riutilizza silenziosamente la vecchia selezione.
Per ripetere la selezione passare nuovamente gli stessi report.

## Rigenerazione

Con Docker non serve installare Python, Rust/Cargo o `nm` sull'host. Su Windows
avviare Docker Desktop con **container Linux**, poi da PowerShell:

```powershell
.\scripts\windows\rom_coverage_docker.ps1
```

Su Linux, con Docker e Buildx:

```bash
bash scripts/linux/rom_coverage_docker.sh
```

Entrambi usano **`Dockerfile.coverage`**, con Git e Docker sull’host.
Preparano un contesto temporaneo con i sorgenti runtime di **un commit HEAD
fissato all’avvio**, archiviati da Git, e gli strumenti/prove pubblici correnti.
Le modifiche di estrazione non ancora committate restano fuori dalla build.
Il normale eseguibile, la cache Cargo dell’host e i job di estrazione non vengono
modificati. I sorgenti archiviati hanno date di estrazione nuove per impedire che
una cache Cargo calda riusi binari di sorgenti diversi.

Il Dockerfile usa il frontend già incluso in BuildKit: non scarica separatamente
`docker/dockerfile:1` da Docker Hub. Questo evita il fallimento preliminare
`failed to fetch anonymous token` del frontend quando il DNS Docker non risponde.
La build richiede comunque rete/DNS per immagini Rust, pacchetti Debian e crate
non ancora in cache; la generazione del report continua a usare `--network none`.
Vedere [la diagnostica Docker](../docker.md#rom-coverage-atlas).

Per il percorso predefinito basta la **ROM EUR**: il container compila CLI e
analizzatore, legge la revisione dai medesimi sorgenti compilati, prepara il
dataset e lo conserva in `target/rom-data/coverage/<SHA-ROM>/e<extractor>-d<decoder>/`.
Alle esecuzioni successive lo autentica e riusa senza riscriverlo. Una nuova
revisione committata sceglie una nuova cache.
I launcher passano `--cache-keep 2`: dopo una pubblicazione riuscita conservano
il dataset corrente e la più recente altra generazione per quella ROM, rimuovendo
le cache automatiche superate. I dataset espliciti restano intatti. Il comando
Python diretto lascia la pulizia disabilitata (`--cache-keep 0`); può abilitarla
con un conteggio positivo. Una pulizia non riuscita non invalida il report già
pubblicato. Vedi [conservazione degli artefatti](LOCAL_ARTIFACTS.md).
Nessuna GUI, traduzione nativa o gioco viene eseguito. Python e Rust non servono
sull’host. Il vecchio target diretto `Dockerfile.linux --target coverage` rimane
un percorso per sviluppatori sui sorgenti della working tree, senza l’isolamento
Git offerto dai due launcher.

I 28.455 punti d'ingresso sono metadati versionati, legati per hash alla ROM,
alla lista conservata e alla cache da cui furono estratti. **Non serve più la
vecchia `.dll` locale**. ROM e prove sono montate in sola lettura; solo cache
locale e cartella di output sono scrivibili. Gli asset estratti restano fuori
dall'immagine e da Git. La generazione usa la rete disabilitata; la prima build
può scaricare le dipendenze. Su Linux i file appartengono all'utente chiamante.

Output: **`docs/development/rom-coverage/index.html`** e **`coverage.json`**.
L’HTML e il terminale mostrano **data/ora effettive di rigenerazione**, commit,
generazione del dataset e data/revisione separate delle prove ordinarie.
Il log confronta i conteggi con il report precedente, inclusi i piccoli incrementi
in byte che possono sparire nell’arrotondamento percentuale. Se la copertura
coincide compare **“Copertura invariata”**, ma HTML/JSON vengono comunque
riscritti con una nuova data. Ricompilazione dell’analizzatore e data non vengono
scambiate per un aumento della copertura.
Ogni avvio conserva anche un log in **`target/build-logs/rom-coverage-*.log`**,
inclusi gli errori. Un errore di validazione non sostituisce il report esistente.
Su PowerShell, `-OpenReport` apre il risultato nel browser con un URL versionato
per richiedere la nuova copia della pagina:

```powershell
.\scripts\windows\rom_coverage_docker.ps1 -OpenReport
```

Per usare un dataset esistente o un output separato:

```powershell
.\scripts\windows\rom_coverage_docker.ps1 -DatasetDir 'D:\FFTA dati\e65' -OutputDir 'target/coverage-preview'
```

```bash
bash scripts/linux/rom_coverage_docker.sh --dataset '/dati/FFTA e65' --output target/coverage-preview
```

Il dataset esplicito resta in sola lettura. Una versione diversa da quella
dell’estrattore viene accettata come **fotografia storica**, con avviso visibile
nell’HTML e nel terminale. Omettere `--dataset` / `-DatasetDir` per seguire la
versione corrente. ROM, file decoded e sorgenti devono comunque autenticarsi. `-RomPath`,
`-EvidencePath` e `-OutputDir` corrispondono a `--rom`, `--evidence`, `--output`.
`-TranslationPath` / `--translation` rimane un controllo facoltativo sulla
cache ELF originale conservata; non è richiesto dal percorso normale.
Tutti i percorsi relativi partono dalla radice del progetto. Gli spazi sono
supportati; virgole, doppi apici e ritorni a capo non sono supportati nei mount.
Docker deve poter accedere ai percorsi host.

Alternativa per sviluppatori con Python 3.11+ e Rust/Cargo:

```sh
cargo build --release --locked --offline -p recomp-core --example rom_code_ranges
python3 scripts/rom_coverage.py \
  --rom 'rom/1194 - Final Fantasy Tactics Advance (E)(Surplus).gba' \
  --dataset 'target/rom-data/coverage/8fab1e390cb9d5e695bfdb86e81246892e1aa8c0cb4014db8235ff76538ea632/e65-d64' \
  --analyzer target/release/examples/rom_code_ranges \
  --output docs/development/rom-coverage
python3 -m unittest discover -s scripts -p test_rom_coverage.py -v
```

Una nuova generazione committata aggiorna automaticamente i dati. Per promuovere
le prove di uso ordinario occorre invece rivedere le singole accettazioni e
aggiornare esplicitamente `evidence.json`. Il generatore rifiuta ROM, source hash,
file decoded o cache con revisione diversa dall’estrattore che la deve produrre. Conservare la fotografia
precedente quando serve confrontare avanzamenti. Gli intervalli usano offset
file e fine esclusiva `[inizio, fine)`; per il primo mirror CPU aggiungere
`0x08000000`. I mirror non moltiplicano i byte della ROM.

L'output contiene solo metadati. ROM, salvataggi, input, immagini del gioco,
asset decoded e codice generato restano fuori dai file versionati. Nessun nuovo
benchmark, registrazione o test runtime accettato è stato ripetuto per la mappa.
La validazione del report comprende conteggi contro un modello indipendente
di insiemi di byte, generazione reale autenticata e controlli del browser.

Il percorso Docker è verificato con estrazione EUR e65 reale, confronto con i
61 file già accettati, riuso senza scrittura e rigenerazione HTML/JSON. I test
coprono generazioni nuove/storiche, risorse in attesa di revisione, migrazione
source-only, isolamento Git, timestamp, conservazione del report su errore e
conteggi indipendenti. Il launcher PowerShell è verificato con Docker simulato
su Linux; Windows nativo/PowerShell 5.1/Docker Desktop restano non provati direttamente.

DBG-07 è verificato con 26 test del generatore/importazione, otto test Bash e
12 controlli PowerShell 7.4 in container (Docker simulato), importazione Docker
reale degli audit conservati, rifiuto di identità errate e controlli browser
1400/620 px. Il dataset e76 e gli audit restano in sola lettura; nessuna scena
viene rieseguita. Windows nativo/PowerShell 5.1/Docker Desktop restano da provare.
