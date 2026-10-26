# Mappa della copertura ROM

La [mappa navigabile](rom-coverage/index.html) rappresenta tutta la ROM EUR da
16 MiB per offset, con selezione di celle da 64 KiB, salto a un indirizzo,
selezione delle famiglie, prove collegate e maggiori regioni non classificate.
Aprire il file HTML in un browser; funziona offline, senza server o dipendenze.
I [dati JSON](rom-coverage/coverage.json) contengono gli intervalli esatti,
mentre il colore delle celle ne riassume la densità.

La fotografia del **26 ottobre 2026 è aggiornata a EUR e65-d64**:
SHA-256 ROM `8fab1e390cb9d5e695bfdb86e81246892e1aa8c0cb4014db8235ff76538ea632`,
manifest `7ec5b72ccbd4abc40bb69ecb26a6a0eb7ee086194f43c56435e501253b8841c8`.
La generazione segue automaticamente l’estrattore dell’ultimo commit; la revisione
delle prove di uso ordinario resta distinta. La precedente e61-d60 rimaneva
bloccata dalla versione del registro delle prove, anche dopo una rigenerazione.

- **Codice candidato:** 260.472 B, **1,55%**, stessi 28.455 ingressi ROM
  conservati in `code-seeds.json`; 404 ingressi RAM esclusi all’origine.
  Non è codice eseguito né una percentuale di decompilazione completata.
- **Dati identificati e disponibili al runtime:** 10.224.490 B, **60,94%**,
  in **60 famiglie**. Rispetto a e61: +17.142 B identificati e +23.578 B runtime.
  La proiezione solo sorgente di 6.436 B è ora migrata: nessun doppio conteggio.
  Ogni file decoded e intervallo ROM è verificato per hash; il runtime resta opt-in.
- **Consumo ordinario positivo:** **52 delle 60 famiglie runtime**, ereditate
  dalle prove revisionate e61 solo a parità di file decoded e mappa delle sorgenti.
  Quattro famiglie già revisionate hanno solo prove mirate (testi brevi, tastiera
  del nome, flag globali, pose alternative di battaglia). Quattro nuove famiglie
  hanno prove da revisionare: script-special-normal, script-special-alternate,
  script-override e script-registry-special. I byte distinti letti restano **N/D**.
- **Dati condivisi per riferimento:** almeno 1.359.934 B, **8,11%**, in
  **294 intervalli**: riferimenti BG, alias manifest, song/header e tone/wave.
  Copie uguali a indirizzi diversi restano distinte.
- **Non classificato:** 6.353.546 B, **37,87%**, dopo dati, header e candidati
  codice; 17.142 B in meno. Non significa spazio libero o inutilizzato.

Le categorie **non vanno sommate**. Restano 24.120 B con più famiglie proprietarie
ed emergono **61.480 B di sovrapposizione codice/dati**, da esaminare come
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

Per il percorso predefinito basta la **ROM EUR**: il container compila CLI e
analizzatore, legge la revisione dai medesimi sorgenti compilati, prepara il
dataset e lo conserva in `target/rom-data/coverage/<SHA-ROM>/e<extractor>-d<decoder>/`.
Alle esecuzioni successive lo autentica e riusa senza riscriverlo. Una nuova
revisione committata sceglie una nuova cache, senza modificare quella vecchia.
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
