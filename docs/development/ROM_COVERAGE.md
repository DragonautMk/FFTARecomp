# Mappa della copertura ROM

La [mappa navigabile](rom-coverage/index.html) rappresenta tutta la ROM EUR da
16 MiB per offset, con selezione di celle da 64 KiB, salto a un indirizzo,
selezione delle famiglie, prove collegate e maggiori regioni non classificate.
Aprire il file HTML in un browser; funziona offline, senza server o dipendenze.
I [dati JSON](rom-coverage/coverage.json) contengono gli intervalli esatti,
mentre il colore delle celle ne riassume la densità.

La fotografia del **25 ottobre 2026 è aggiornata a EUR e61-d60**:
SHA-256 ROM `8fab1e390cb9d5e695bfdb86e81246892e1aa8c0cb4014db8235ff76538ea632`,
manifest `6bfbbed6fdb12a81438537de07004c6338a814b683c68eadb32d772f4bb95714`.
Le prove sono state rilette dai risultati conclusi, senza ripetere scene o test
runtime. La precedente e37-d36 aveva 31 famiglie runtime e 3.972.680 B di dati.

- **Codice candidato:** 260.472 B, **1,55%**. Stessi 28.455 ingressi ROM
  conservati, ora distribuiti come soli indirizzi in `code-seeds.json`;
  404 ingressi RAM esclusi all'origine. L'analizzatore rilegge la ROM, senza
  usare istruzioni o codice nativo precompilato nel report. Non è codice eseguito
  né una percentuale di decompilazione completata.
- **Dati identificati:** 10.207.348 B, **60,84%**. Unione autenticata delle
  56 risorse del manifest e di **33 intervalli / 6.436 B** del modello
  `unit-obj-script-special-normal`, ancora senza pubblicazione runtime.
  Sono inclusi ora audio, script/testi localizzati e grafica delle unità.
- **Disponibili al runtime:** 10.200.912 B, **60,80%**, in **56 famiglie**.
  Il modello non integrato resta escluso da questa misura. Ogni file decoded e
  intervallo ROM è verificato per hash; l'attivazione runtime resta opt-in.
- **Consumo ordinario positivo:** **52 delle 56 famiglie runtime** hanno una
  prova storica positiva circoscritta. Restano mirate soltanto: testi brevi,
  tastiera del nome, flag globali e pose alternative di battaglia. I byte
  distinti effettivamente letti sono **N/D**: i contatori includono riletture.
- **Dati condivisi per riferimento:** almeno 1.359.934 B, **8,11%**, in
  **294 intervalli**. Il censimento aggiunge gli header richiamati da più brani
  e le wave richiamate da più tone ai precedenti riferimenti BG e alias manifest.
  Copie uguali a indirizzi diversi restano distinte.
- **Non classificato:** 6.370.688 B, **37,97%**, dopo dati, header e candidati
  codice. Non significa spazio libero o inutilizzato.

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
inventata una singola sessione e61 con tutte le famiglie attive. Ad esempio, il
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

Entrambi usano il target `coverage` di `Dockerfile.linux`. Per il percorso
predefinito basta la **ROM EUR**: il container compila il CLI di estrazione e
l'analizzatore, prepara il dataset della generazione revisionata e lo conserva in
`target/rom-data/coverage/<SHA-ROM>/e61-d60/`. Alle esecuzioni successive lo
verifica e riusa senza riscriverlo. Nessuna GUI, traduzione nativa o gioco viene
eseguito. Python e Rust non servono sull'host.

I 28.455 punti d'ingresso sono metadati versionati, legati per hash alla ROM,
alla lista conservata e alla cache da cui furono estratti. **Non serve più la
vecchia `.dll` locale**. ROM e prove sono montate in sola lettura; solo cache
locale e cartella di output sono scrivibili. Gli asset estratti restano fuori
dall'immagine e da Git. La generazione usa la rete disabilitata; la prima build
può scaricare le dipendenze. Su Linux i file appartengono all'utente chiamante.

Output: **`docs/development/rom-coverage/index.html`** e **`coverage.json`**.
Il terminale indica revisione, data delle prove, conteggi e percorsi finali.
Se il JSON coincide con il precedente compare **“Contenuto invariato”**.
Ogni avvio conserva anche un log in **`target/build-logs/rom-coverage-*.log`**,
inclusi gli errori. Un errore di validazione non sostituisce il report esistente.
Su PowerShell, `-OpenReport` apre il risultato nel browser:

```powershell
.\scripts\windows\rom_coverage_docker.ps1 -OpenReport
```

Per usare un dataset esistente o un output separato:

```powershell
.\scripts\windows\rom_coverage_docker.ps1 -DatasetDir 'D:\FFTA dati\e61' -OutputDir 'target/coverage-preview'
```

```bash
bash scripts/linux/rom_coverage_docker.sh --dataset '/dati/FFTA e61' --output target/coverage-preview
```

Il dataset esplicito resta in sola lettura e deve corrispondere a **e61-d60**;
un vecchio e37 viene segnalato con revisione attesa e rimedio. `-RomPath`,
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
  --dataset 'target/rom-data/coverage/8fab1e390cb9d5e695bfdb86e81246892e1aa8c0cb4014db8235ff76538ea632/e61-d60' \
  --analyzer target/release/examples/rom_code_ranges \
  --output docs/development/rom-coverage
python3 -m unittest discover -s scripts -p test_rom_coverage.py -v
```

Per una nuova generazione, aggiornare prima l'evidenza e rivedere le singole
accettazioni. Il generatore rifiuta una ROM, generazione, source hash o file
decoded non coerente con gli input dichiarati. Conservare la fotografia
precedente quando serve confrontare avanzamenti. Gli intervalli usano offset
file e fine esclusiva `[inizio, fine)`; per il primo mirror CPU aggiungere
`0x08000000`. I mirror non moltiplicano i byte della ROM.

L'output contiene solo metadati. ROM, salvataggi, input, immagini del gioco,
asset decoded e codice generato restano fuori dai file versionati. Nessun nuovo
benchmark, registrazione o test runtime accettato è stato ripetuto per la mappa.
La validazione del report comprende conteggi contro un modello indipendente
di insiemi di byte, generazione reale autenticata e controlli del browser.

Il percorso Docker aggiornato è verificato con estrazione e generazione reali
EUR su Linux, confronto con i 57 file già accettati di e61 e controlli sui nuovi
seed portabili, intervalli source-only, condivisioni audio e messaggi di errore.
Il launcher PowerShell è verificato con Docker simulato su Linux; Windows
nativo/PowerShell 5.1/Docker Desktop restano non provati direttamente.
