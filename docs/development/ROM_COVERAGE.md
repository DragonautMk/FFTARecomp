# Mappa della copertura ROM

La [mappa navigabile](rom-coverage/index.html) rappresenta tutta la ROM EUR da
16 MiB per offset, con selezione di celle da 64 KiB, salto a un indirizzo,
selezione delle famiglie, prove collegate e maggiori regioni non classificate.
Aprire il file HTML in un browser; funziona offline, senza server o dipendenze.
I [dati JSON](rom-coverage/coverage.json) contengono gli intervalli esatti,
mentre il colore delle celle ne riassume la densità.

La fotografia del **23 ottobre 2026 è congelata su EUR e37-d36**:
SHA-256 ROM `8fab1e390cb9d5e695bfdb86e81246892e1aa8c0cb4014db8235ff76538ea632`,
manifest `51b89fe76fcdca0c08535b131623f76f8695e0ff6634d81a8fe293ebb408a80e`.
Non descrive automaticamente successive integrazioni e38 o l'immagine USA.

- **Codice candidato:** 260.472 B, **1,55%**. Unione esatta degli intervalli
  decodificati dall'analizzatore corrente, usando i 28.455 ingressi ROM della
  cache nativa conservata; 404 ingressi RAM esclusi. Non è una percentuale di
  codice eseguito, di funzioni comprese o di decompilazione completata.
- **Dati identificati:** 3.972.680 B, **23,68%**. Comprendono 31 risorse
  autenticate del manifest, incluso
  `gameplay.progression-selection-gates` da 1.024 B. Ogni intervallo del
  manifest viene ricontrollato contro la ROM; tutti i file decoded vengono
  verificati per hash senza esportarne il contenuto.
- **Disponibili al runtime:** 3.972.680 B, **23,68%**, nelle 31 risorse della
  baseline. Questa è la capacità delle viste tipizzate, distinta dall'uso
  effettivamente osservato. L'attivazione rimane opt-in tramite `--rom-data`.
- **Consumo ordinario positivo:** **28 delle 31 famiglie runtime** hanno una
  prova storica positiva circoscritta. Testi brevi, tastiera del nome e flag
  globali hanno prove mirate ma non una scena ordinaria positiva accettata.
  I byte distinti effettivamente letti nelle scene sono **N/D**: i contatori
  disponibili includono riletture e non costituiscono una traccia di indirizzi.
- **Dati condivisi per riferimento:** almeno 610.710 B, **3,64%**, in 198
  intervalli censiti. Più riferimenti allo stesso indirizzo nei descrittori BG,
  negli indici packet/transfer e negli alias manifest. Sono una sovrapposizione
  ai dati identificati. Copie uguali a indirizzi diversi non vengono confuse
  con condivisioni di indirizzo.
- **Non classificato nella mappa:** 12.597.392 B, **75,09%**, dopo aver escluso
  dati, header e candidati codice. Include possibili ulteriori codice, mappe,
  audio, tabelle e risorse compresse; non significa spazio libero o inutilizzato.

Le categorie **non vanno sommate**. Sono inoltre visibili 24.120 B con più
famiglie proprietarie del medesimo intervallo e **53.516 B di sovrapposizione
fra dati identificati e candidati codice**. Questi ultimi sono un conflitto di
classificazione da verificare: l'analizzatore segue anche puntatori plausibili
e può interpretare dati come istruzioni. Il database distribuito contiene zero
funzioni per questa immagine EUR. Non si deducono estensioni dal simbolo
successivo. ARM/Thumb sovrapposti sono contati una sola volta nell'unione.
I 188 B dell'header tecnico `0x4..0xc0` sono separati dal branch iniziale.

## Interpretazione delle prove

[evidence.json](rom-coverage/evidence.json) associa ogni famiglia ai record
versionati e conserva i limiti sui campi realmente osservati. Le prove possono
appartenere a generazioni precedenti i cui file sono stati mantenuti: non viene
inventata una singola sessione e37 con tutte le famiglie attive. Ad esempio, il
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
audio, dinamici e indiretti. I blocchi copiati in RAM non vengono interamente
colorati come codice ROM solo perché ne è noto il contenitore misto.

## Rigenerazione

Richiede Python 3.11+, Rust/Cargo e `nm` per leggere i simboli della cache ELF
locale (l'estensione `.dll` di questa cache non ne cambia il formato). Non
ricompila la traduzione e non esegue il gioco. Si compila soltanto il piccolo
esportatore di metadati del progetto, senza modificare gli eseguibili runtime:

```sh
cargo build --release --locked --offline -p recomp-core --example rom_code_ranges
python3 scripts/rom_coverage.py \
  --rom 'rom/1194 - Final Fantasy Tactics Advance (E)(Surplus).gba' \
  --dataset 'target/rom-data-rank-gates/windows-managed-europe/offline dataset' \
  --translation 'out/1194 - Final Fantasy Tactics Advance (E)(Surplus).dll' \
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
