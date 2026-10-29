# Catalogo comune dei contenuti debug

DBG2-02 introduce un servizio readonly condivisibile dal menu e dalla console.
La CLI permette di aggiornare esplicitamente l'indice e ricontrollare un elemento
scelto. Il [selettore visuale DBG2-03](debug_selector.md) riusa lo stesso servizio;
la migrazione delle azioni esistenti è in verifica in DBG2-04. L'indicizzazione non esegue file, importa salvataggi o ripara dataset.

```sh
recomp debug-content scan --root CARTELLA_PORTABLE --json
recomp debug-content scan --add-root CARTELLA_ESPLICITA --json
recomp debug-content scan --add-file FILE_ESPLICITO --json
recomp debug-content scan --root CARTELLA_PORTABLE --rom-sha256 HASH_ROM --json
recomp debug-content check INDICE_SALVATO.json ID --kind replay-profile --json
```

`--root` dichiara una cartella dell'app: legge i file riconosciuti direttamente
nella radice e gli alberi `Data`, `scripts`, `cheats`, `reports`. Dentro `Data`
sono ammessi `debug-sessions`, `config`, `saves`, `cache`, `resources`, `dataset`.
`--add-root` aggiunge una cartella scelta esplicitamente e ne permette la visita
limitata anche fuori da questi nomi. Può essere ripetuto entro otto radici totali.
Quando sono le uniche opzioni di radice, `--add-root` e `--add-file` esaminano
soltanto i riferimenti espliciti. `--add-file` aggiunge un singolo file, senza
enumerare la cartella o i suoi vicini; il campo opzionale `file` mantiene
leggibili le vecchie radici schema 1 che non lo contengono.

Senza opzioni di radice, la scansione usa la cartella portable quando il marker
è presente; altrimenti usa soltanto le posizioni dichiarate dell'app: cartella
dell'eseguibile, configurazione, risorse effettive e cache. Il resolver risorse
conserva l'override `FFTA_RESOURCES_DIR`. Non viene eseguita una ricerca sul disco
né un aggiornamento all'avvio del gioco. Cartelle mancanti producono motivi
nell'indice, senza creare directory o aprire un dialogo.

Gli elementi comprendono sessioni, registrazioni, sequenze `.input`, profili replay, ricette,
strumenti host, cataloghi di script guest, campagne, snapshot, runtime, dataset,
librerie native (incluso `native.bin`), label (incluso `debug-labels.toml`),
manifest noti, cataloghi salvataggi/cheat e report. Le label v1/v2 non vengono
confuse con metadati di catalogo schema 1; il ruolo tipizzato le verifica con il
parser delle label prima dell'uso.
Il riconoscimento usa nomi, estensioni e metadati dichiarati. Un file `.exe`
elencato come runtime non è stato caricato o dichiarato compatibile; un catalogo
cheat non è stato decodificato. Gli script host e i dati degli script guest
mantengono tipi distinti, senza inferire un adattatore eseguibile.

Ogni elemento conserva ID, tipo, nome, descrizione, radice e percorso relativo,
percorso da selezionare, dimensione/hash dove disponibili, ROM/regione dichiarate,
stato di presenza, compatibilità, livello di verifica e motivo. L'ID dipende da
radice logica, tipo e percorso, separatamente dai byte: due omonimi mantengono
identità diverse e una selezione alterata può essere riconosciuta come obsoleta.
La radice portable mantiene gli ID dopo lo spostamento dell'intera cartella.
Le radici esterne rimangono riferimenti espliciti distinti; la persistenza delle
scelte e la migrazione dei riferimenti saranno DBG2-07.

Gli hash uguali producono collegamenti di duplicazione senza unire gli ID o
avviare contenuti. Sono mostrati il numero totale degli altri duplicati e fino
a otto ID collegati per elemento. Due dichiarazioni della stessa radice sono
segnalate ed esaminate una sola volta. Link simbolici, junction, file speciali
e file condivisi tramite hard link sono rifiutati, riusando i controlli già
adottati dalle sessioni isolate; le directory collegate non vengono percorse.

`scan` restituisce `schema: 1`, `kind: ffta-debug-content-index`, radici, elementi,
problemi, contatori e limiti. L'assenza di hash o metadati non viene convertita in
verifica riuscita. `compatibility` distingue `unknown`, `declared-match`,
`incompatible` e `unsupported`; `declared-match` confronta solo l'hash ROM
**dichiarato** con `--rom-sha256`. Pin, payload, formati, feature, ABI e backend
richiedono sempre la validazione dell'handler prima dell'uso.

`check` legge un indice salvato esplicitamente, trova l'ID esatto e ricontrolla
solo il file selezionato. Conserva distinto un file cambiato, mancante,
non ordinario, illeggibile o di tipo incompatibile. I riferimenti con risalita
`..`, radici incoerenti e versioni non supportate sono rifiutati. Senza un hash
nella selezione originale non promette che i byte siano invariati. Exit 0
significa soltanto elemento elencato/ricontrollato; exit 3 segnala rifiuto;
exit 1 segnala query/indice non validi. Non avvia mai l'azione selezionata.

## Limiti e verifica

Una scansione conserva al massimo 512 elementi, visita 8.192 nodi entro 12 livelli
e calcola hash su 64 MiB complessivi. I metadati strutturati hanno un limite di
1 MiB per file e 8 MiB complessivi; l'hashing è a blocchi, senza mantenere in
memoria i runtime o gli snapshot completi. Un indice da ricontrollare è limitato
a 8 MiB. Sono conservati 64 problemi, con contatore degli ulteriori scarti.
Le directory `.git`, `.retention`, `target` e `node_modules` incontrate dentro
una radice non vengono percorse. Una di queste directory può comunque essere
la cartella aggiunta esplicitamente dall'utente.

La saturazione della visita marca `truncated`: il contenuto non visitato resta
sconosciuto. I limiti di hashing/metadati sono spiegati per elemento; la lista
può essere completa pur senza aver letto ogni payload. Una nuova query è un
aggiornamento esplicito, non una verifica o esecuzione automatica.

Il controllo confronta anche ogni componente canonico con il relativo padre:
Wine può nascondere nei normali attributi Windows i link creati sul filesystem
host. Le fixture Wine usano collegamenti reali host; la sola riuscita della
creazione tramite API Windows non viene accettata come prova della fixture.

Le fixture pubbliche sintetiche coprono elenco vuoto, omonimi, duplicati,
spostamento portable, tipi riconosciuti, script mai eseguiti, metadati non validi,
versioni future, cambi/mancanze/tipo errato, radici duplicate/mancanti, link,
hard link, limiti e query malformate. Non servono ROM, salvataggi personali o
riesecuzioni delle prove DBG già concluse. Gli esiti effettivi e i limiti di
piattaforma restano nel record collegato dal
[TODO DBG2](development/DEBUG_TOOLS_V2_TODO.md).
