# Strumenti di debug portable — guida comune

Usare una copia isolata e un caso scelto esplicitamente. Questi percorsi non
modificano automaticamente lo stato delle prove DS e non avviano campagne
generali. I dati di gioco si aggiungono localmente dopo l'estrazione del pacchetto.

Il [nuovo TODO Debug integrato](development/DEBUG_TOOLS_V2_TODO.md) descrive
le estensioni pianificate: scelte e script dal menu senza picker automatici,
console eventi/comandi e cheat. L'implementazione inizia dal
[contratto delle operazioni](debug_operations.md): `debug-operations list/show`
elenca ingressi, parametri e lacune attuali senza eseguirli. Il
[catalogo contenuti](debug_content.md), `debug-content scan/check`, aggiunge un
indice readonly con radici esplicite, identità e stati.
[Debug > Content selector](debug_selector.md) offre la nuova scelta interna;
i ruoli tipizzati e `Parameters / prepare profiles` offrono selezione e bozze
sugli handler esistenti. `Debug > Scripts and tools` aggiunge adattatori dichiarati
per homepage, report campagne e anteprima artefatti, ricette readonly e cataloghi
guest separati. Avvio e annullamento sono espliciti; servono Python3.11+ scelto
nel menu per i tre adattatori, con output e log nella radice config corrente.
Le scelte e le viste del selettore sono salvate nella radice config e seguono
lo spostamento portable, con pin e spiegazioni per file mancanti/modificati.
Console e cheat seguono il nuovo TODO.

## Prerequisiti e cartella spostabile

Estrarre tutto il pacchetto in una cartella scrivibile, con spazi/Unicode ammessi.
Servono il proprio ROM FFTA USA/EUR e, se selezionati, SRAM, snapshot, BIOS,
registrazione, dataset o libreria compatibili. Questi file non sono distribuiti.
Tenere insieme eseguibili, portable.txt, tcc, Licenses, TinyCC, scripts e docs.

Il runtime gestisce sessioni/registrazione/replay/report senza Python o Docker.
Gli strumenti offline richiedono Python 3.11 disponibile come python3, oppure
Docker Desktop in modalità Linux con immagine già presente:
ffta-debug-tools:local per artefatti/campagne, ffta-rom-coverage:local per homepage.
Per artefatti/campagne PowerShell, -Docker deve precedere il comando;
i flag Python/runtime vengono inoltrati letteralmente, incluso --out.
I launcher non costruiscono o scaricano immagini. I build Docker sono operazioni
del checkout, non prerequisiti da avviare ogni volta.

Windows PowerShell 5.1+, dopo avere sostituito il percorso di esempio:

~~~powershell
$Bundle = 'D:\FFTARecomp-portable'
$Dbg = Join-Path $Bundle 'scripts/windows/debug_runtime.ps1'
& $Dbg help
& $Dbg help debug-control
~~~

Tutti gli argomenti relativi dei launcher partono da Bundle, anche se il terminale
è altrove. Su Linux usare bash scripts/linux/debug_runtime.sh COMMAND...;
per un pacchetto Windows sotto Wine aggiungere --wine /percorso/reale/wine64.
Gli argomenti di percorso del runtime Wine usano percorsi Windows, per esempio
Z:/percorso/Linux/caso. Il launcher non converte nomi/argomenti arbitrari.

Il launcher runtime stampa sempre il percorso del log in Data/config/debug-logs.
I launcher offline stampano il log in target/build-logs. Il player conserva
play-error.log per errori di avvio; i replay conservano worker.log/progress.json/
result.json nel profilo. Un errore non autorizza a cancellare la sua prova.
Nel manifest, source_sha256 identifica il file in ingresso al packager prima
della normalizzazione dei documenti; per un binario è l’hash del binario, non
della tree Rust. PACKAGE-MANIFEST.json identifica i payload; non certifica che
una piattaforma o un caso sia stato verificato.

## 1. Registrare una scena

Preparare un caso con destinazione nuova. Sostituire i tre percorsi dei propri
input; --state è facoltativo per partire dal boot:

~~~powershell
$Case = 'Data/debug-sessions/caso'
& $Dbg debug-session create 'C:/Giochi/FFTA EUR.gba' --dir $Case --sav 'C:/Debug/progressi.sav' --state 'C:/Debug/scena.ffstate' --no-bios --interp
& $Dbg debug-session open $Case --paused
~~~

Il secondo comando resta aperto finché si chiude la finestra. Dentro la copia
scegliere **Debug > Start input recording**, poi Resume e riprodurre la scena;
tornare in pausa e scegliere **Stop input recording**. F6 pausa/riprende,
F7 avanza un frame neutro, F8 dieci. Il menu/controller usa gli stessi comandi.

Da un secondo terminale, con Bundle/Dbg/Case impostati come sopra:

~~~powershell
& $Dbg debug-control status $Case
& $Dbg debug-control step $Case 1 --keys 3fe
& $Dbg debug-control step $Case 1 --keys 3ff
~~~

Le maschere sono active-low: 3fe=A, 3ff=rilasciato. Non mescolare rewind/Load
con una sequenza lineare. Stop mostra il nome esatto della cartella; conservarlo
esplicitamente, senza scegliere automaticamente l'ultimo caso di un'altra prova.
Chiudere normalmente la copia prima di preparare il profilo:

~~~powershell
$Recording = "$Case/Data/config/recordings/recording-NOME_ESATTO"
$Profile = 'Data/debug-profiles/riproduzione'
& $Dbg debug-replay create --recording $Recording --session $Case --dir $Profile --case 'Locale-EUR' --timeout-ms 120000
& $Dbg debug-replay run "$Profile/replay.json" --json
~~~

Un passed confronta l'intero stato finale e il clock con l'endpoint registrato.
Diverged/incompatible/timeout/interrupted/error sono distinti; non trasformarli
in accettazione. Un replay crea risultati locali ma non scrive la partita
personale. Per annullare: debug-replay cancel PROFILO/replay.json.
[Registrazioni](debug_recordings.md), [profili](debug_replays.md).

## 2. Segnalare un bug con contesto riproducibile

Usare il profilo e la registrazione del percorso 1. La creazione del report è
readonly e non avvia un altro replay:

~~~powershell
& $Dbg debug-report create "$Profile/replay.json" --symptom 'Cosa accade, cosa era atteso, ultimo input e punto osservato'
~~~

Aprire l'HTML indicato dal JSON: si trova in Data/config/issues/ dentro quel
profilo. Mostra identità, risultato conservato, prossima verifica e launcher
reproduce.ps1/reproduce.sh relativi. Lo spostamento dell'intero profilo conserva
la ricetta; il singolo HTML non trasporta gli input. Esaminare summary.json,
percorsi e allegati prima di condividerli. Nulla viene inviato automaticamente.
Exit0 significa report compatibile creato, non bug risolto; exit3 può pubblicare
un report con input incompatibili.

Nella finestra isolata, **Copy diagnosis** salva anche la diagnosi se gli
appunti non sono disponibili. **Choose replay profile > Create issue report**
è l'accesso equivalente dal menu. Per raccogliere gli eventi precedenti
all'errore, nella copia in pausa:

~~~powershell
& $Dbg debug-control recent-start $Case --events 1024
# Riprodurre il problema, poi mettere in pausa con F6 o debug-control pause.
& $Dbg debug-control recent-report $Case
~~~

Gli errori osservati dal frontend congelano/salvano la finestra per default.
Un crash senza callback non può esportare memoria; scarti, finestra e letture
non disponibili restano espliciti. Le query status non riempiono il registro.
[Report del problema](debug_issues.md), [eventi recenti](debug_recent.md).

## 3. Confrontare due versioni del runtime

Preparare due profili dalla stessa registrazione, sessione chiusa e input,
selezionando esplicitamente A e B. Non sostituire un binario dentro un profilo
già autenticato:

~~~powershell
& $Dbg debug-replay create --recording $Recording --session $Case --dir 'Data/debug-profiles/runtime-A' --case 'Versione-A' --runtime 'C:/Build-A/recomp.exe'
& $Dbg debug-replay create --recording $Recording --session $Case --dir 'Data/debug-profiles/runtime-B' --case 'Versione-B' --runtime 'C:/Build-B/recomp.exe'
& $Dbg debug-replay run 'Data/debug-profiles/runtime-A/replay.json' --json
& $Dbg debug-replay run 'Data/debug-profiles/runtime-B/replay.json' --json
~~~

Confrontare outcome, runtime/ROM/BIOS/dataset/input/endpoint e clock nei due
result.json. Il riferimento è l'endpoint della registrazione: A=passed/B=diverged
è una differenza riproducibile, non una misura di velocità. Se entrambi divergono,
il riferimento non è confermato; incompatibilità non significa parità.
Gli input e l'AV della registrazione restano comuni e i runtime sono identificati
separatamente da quello che registrò il caso.

Per localizzare una differenza tra interprete/raw e native o typed **dentro
lo stesso runtime scelto**, usare invece il confronto per frame:

~~~powershell
& $Dbg debug-compare create --recording $Recording --session $Case --dir 'Data/debug-profiles/prima-differenza' --case 'Raw-vs-typed' --dataset 'C:/Debug/dataset-verificato' --frames 100
& $Dbg debug-compare run 'Data/debug-profiles/prima-differenza/replay.json'
~~~

Scegliere frames entro la registrazione; per native selezionare una libreria
esistente con --library al posto del dataset. Non compila nuove traduzioni.
Non è un confronto per frame tra due eseguibili diversi. Nel menu:
**Choose replay profile > Compare first difference**; Cancel replay interrompe.
[Confronto e scope](debug_comparisons.md).

## 4. Cercare letture fuori dal piano noto

Serve un runtime con rom-data-read-audit: il build ordinario rifiuta l'audit,
non restituisce zero letture. Il JSON diagnose espone
build.features.rom_data_read_audit. Nel checkout, il build nativo corrispondente
è cargo build --locked --release -p recomp -p ffta-play --features recomp/rom-data-read-audit;
il packager non abilita feature su un binario già costruito. Selezionare una registrazione e un manifest TOML
già verificato per la stessa ROM; nessuna estrazione o rigenerazione è implicita.
Scegliere una finestra entro il numero di frame registrati:

~~~powershell
& $Dbg debug-rom-audit create --recording $Recording --session $Case --dir 'Data/debug-profiles/letture' --case 'Finestra-letture' --preset all --from 0 --to 100 --limit 20000 --known-manifest 'C:/Debug/dataset-verificato/manifest.toml'
& $Dbg debug-rom-audit run 'Data/debug-profiles/letture/replay.json'
~~~

Nel rom-audit.json del profilo leggere known_plan.outside_distinct_bytes,
intervalli fisici, servizio typed/raw/unclassified, reader, finestra, scarti e
troncamento. Senza piano autenticato il confronto outside è sconosciuto.
--range usa offset ROM fisici, non PC guest. Zero richieste riguarda i soli
reader/filtro/finestra osservati; non dimostra assenza di uso nel gioco.

Il menu **Audit ROM reads** usa un profilo audit selezionato. Nel checkout la
mappa coverage può importare l'audit conservato, senza ripetere il caso.
Il pacchetto non contiene una coverage privata o un dataset; il report JSON
è già consultabile localmente. [Audit](debug_rom_audit.md).

## 5. Liberare temporanei preservando prove e input

Chiudere la sessione e scegliere una destinazione nuova per il piano. Python:

~~~powershell
python3 "$Bundle/scripts/debug_artifacts.py" preview --session "$Bundle/$Case" --out "$Bundle/Data/debug-artifact-plans/revisione"
# Leggere index.html e plan.json, poi applicare QUEL piano invariato:
python3 "$Bundle/scripts/debug_artifacts.py" apply "$Bundle/Data/debug-artifact-plans/revisione/plan.json" --receipt "$Bundle/Data/debug-artifact-plans/apply-receipt.json"
~~~

Con PowerShell e Docker già pronto, usare il launcher con gli stessi argomenti,
relativi alla root del bundle:

~~~powershell
& "$Bundle/scripts/windows/debug_artifacts.ps1" -Docker preview --session $Case --out 'Data/debug-artifact-plans/revisione-docker'
~~~

Su Linux: bash scripts/linux/debug_artifacts.sh [--docker] ARGOMENTI.
Il piano preserva ROM/snapshot/SRAM/input, report/errori, dataset, runtime
autenticati e file sconosciuti; seleziona solo cache native riconosciute e, nel
checkout, copie disposable di unità ROM già concluse. --keep DIR protegge una
sessione intera. Una lease attiva/interrotta, un piano cambiato o file inattesi
blocca le rimozioni; leggere il receipt di errore. Non rimuovere una lease solo
perché è vecchia. Spostamento o aggiornamento del tool richiede un piano nuovo.

Non elimina intere sessioni o report per liberare il cap: archiviarli
esplicitamente a gioco chiuso. [Conservazione e limiti](debug_artifacts.md).

## Trovare i report e selezionare il lavoro

Generare l'indice locale dopo avere creato/spostato report:

~~~powershell
& "$Bundle/scripts/windows/report_home.ps1" -Python -OpenReport
# Oppure, con l'immagine coverage già presente:
& "$Bundle/scripts/windows/report_home.ps1" -OpenReport
~~~

Linux: bash scripts/linux/report_home.sh --python oppure senza flag per Docker.
La pagina elenca i file esistenti; presenza/mtime non significano accettazione.
Le campagne selezionano --case/--ds/--family/--transition e riusano JSON con pin;
**report** non riproduce il gioco. **run-case** avvia un solo caso esplicito
tramite il supervisore esistente e richiede un motivo di riapertura per prove
precedenti. [Campagne](debug_campaigns.md).

Per controllare una finestra aperta senza coordinate, usare debug-control.
status funziona durante un comando pendente; cancel SESSION --request ID
annulla soltanto l'ID osservato. --timeout-ms N (20..120000ms) limita l'attesa e aggiunge
la scadenza del budget al prossimo rientro frontend. Un timeout client senza
conferma non prova che il lavoro sia già fermo: controllare status prima di
ritentare. Nessun server di rete è esposto. [Canale locale](debug_control.md).

## Matrice delle verifiche e limiti

- **Windows GNU tramite Wine/Xvfb:** strumenti DBG01–21 verificati nelle unità
  documentate; audit opzionale e default hanno scope separato. Per DBG22 vedere
  la verifica del pacchetto sotto. Wine non equivale a Windows nativo.
- **Windows nativo / PowerShell5.1:** non verificati. PowerShell7.4.6 in
  container Linux passa13controlli dei launcher con applicazioni pubbliche
  simulate; questo non è esecuzione su Windows nativo. Nessuna prova di controller fisici o ascolto audio hardware.
- **Linux host:** Python/Bash, report offline, Docker senza rete, fixture pubbliche
  e tar di packaging verificati. Non è una nuova accettazione del runtime Linux
  corrente; gli strumenti interattivi recenti richiedono verifica nativa dedicata.
- **Docker:** strumenti offline su immagini già presenti. Non esegue la GUI, non
  distribuisce dataset e non rende disponibili Wine o runtime Linux mancanti.
- **Android/Godot:** fuori dall'ambito di questi strumenti desktop.

Il pacchetto include solo input pubblici espliciti: binari selezionati, database
pubblico, TinyCC/sorgenti/licenze, script/template e guide. Data è vuota; non
include ROM/BIOS, salvataggi, snapshot, registrazioni, traduzioni generate,
fixture, log o risultati. I documenti distribuiti mantengono link ai file presenti;
riferimenti a prove e sorgenti del checkout non incluse sono resi testo esplicito.


## Verifica della distribuzione — 29 ottobre 2026

Build Windows GNU audit release locked/offline su sorgenti pubblici congelati
più overlay espliciti; il launcher proviene dalla stessa selezione. Nessuna
modifica al ciclo guest: il delta Rust finale allinea soltanto help/sinossi
alla cancellazione e alle scadenze già implementate.

Dieci test pubblici del packager verificano ZIP/tar, whitelist e manifest,
assenza di file personali anche tra i vicini degli input, licenze inalterate,
link dei documenti, permessi eseguibili Linux, symlink/header ROM rifiutati,
errore senza perdere l'archivio precedente e log da cartella spostata.
Il decimo verifica anche il rifiuto dei prefissi privati UTF-8/UTF-16LE.
Tredici controlli PowerShell7.4.6 simulati in container Linux passano anche
inoltro --out, esito nonzero, stderr, Docker readonly/senza rete e conservazione
della homepage precedente. Corrette conversione del parametro nullo di Replace,
rimozione del temporaneo nascosto e intercettazione dei flag da parametri comuni.

Dal pacchetto Windows estratto e spostato in un percorso con spazi/apostrofo/
Unicode, con terminale in un'altra directory:19invocazioni CLI tramite launcher,
menu Start/Stop per una registrazione di1frame, due runtime distinti che
riproducono quel singolo endpoint, report problema e audit di1frame.
Sono4frame nuovi per la sola integrazione della guida, non vecchie accettazioni
DS/native o benchmark rieseguiti. Clock/endpoint dei due replay coincidono;
l'audit distingue27.403richieste/885byte fuori dal piano noto nella sola finestra.

La diagnosi iniziale del test cercava il flag audit nel campo sbagliato: corretto
lo script, riusati i tre comandi già riusciti prima della GUI. Una contesa Windows
transitoria sul JSON è stata recuperata dal canale esistente; la finestra recente
può congelarsi su quell'errore osservato prima di un rifiuto successivo.
Nessun comando o frame viene rieseguito per ricostruire il contesto.

Dieci controlli offline dal bundle spostato verificano host/Docker, tre prove
riusate senza altri frame, rifiuto di una ripetizione senza motivo, homepage,
piano/apply e receipt obsoleto senza rimozioni. Rimossi soltanto66byte di cache
SYNTHETIC creata dopo la chiusura e mai caricata;28file reali della copia e
11guardie degli input/runtime sorgente conservano hash/mtime. Screenshot GUI
e browser1400/620dei6report nuovi ispezionati, filtri/ricerca/reset/link passano.

La scansione dei primi binari ha trovato percorsi personali del builder. I
binari finali sono ricompilati con --remap-path-prefix Rust e -ffile-prefix-map
TinyCC, con identica selezione dei sorgenti; non si modificano i binari a posteriori.
Due prove sintetiche TinyCC con Wine passano: errore di compilazione con rilascio
lock/contesto e cache ELF con callback/stack ampio. Nessun frame di gioco.
Il packager accetta --private-prefix PREFISSO ripetibile, rifiuta UTF-8/UTF-16LE
nei payload e conserva l'archivio precedente in caso d'errore. Registrare il
prefisso non è necessario: il manifest conserva solo il numero di controlli.

Il record di distribuzione conserva identità e verifica CRC/manifest dell'archivio
finale, senza ROM/BIOS/Data personali. Il binario finale estratto/spostato supera un solo replay aggiuntivo della
registrazione nuova conservata, con identico stato completo/clock:5frame nuovi
in totale per DBG22. Le correzioni dei soli documenti non richiedono altri frame. Le verifiche storiche
DBG01–21 restano riferimenti con il proprio ambito; la loro disponibilità locale
non autorizza ripetizioni automatiche. [Record](changes/2026-10-29-distribute-shared-debug-tools.json).

Gli [eventi comuni DBG2-08](debug_events.md) collegano UI e registro recente con
identità e correlazione condivise; la mappa pubblicata dichiara hook parziali e
mancanti. Le console live e i produttori più estesi restano nel TODO DBG2.
