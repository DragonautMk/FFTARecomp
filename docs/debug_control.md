# Controllo locale, pausa e avanzamento — DBG-10/20

Nelle [sessioni isolate](debug_sessions.md), **F6** mette in pausa o riprende,
**F7** avanza un frame e **F8** dieci. Il menu **Debug** offre Step 1 frame,
Step 10 frames e Resume. Per avanzare occorre essere già in pausa: la richiesta
non riavvolge la macchina e si ferma dopo il numero di frame completati richiesto.
I tasti F6/F7/F8 vengono consumati prima di testo, binding guest, Fast e cattura
scorciatoie. Sono riservati solo nella copia di debug; gli autorepeat non generano
altri passi. I comandi del menu sono utilizzabili anche con la navigazione gamepad.

I passi da menu/tastiera usano **input guest neutro** (`3ff`, tutti rilasciati).
La conferma del menu e i tasti fisici tenuti non diventano input guest. Per scegliere
una maschera esplicita usare la CLI. I clic sul gioco non vengono eseguiti durante
il passo. Il normale menu continua a mettere in pausa anche la partita principale.
I nuovi comandi non controllano la partita principale.

Il riquadro della copia, accanto al menu, indica frame guest, clock, motivo
(`PAUSE-COMMAND`, `MENU-PAUSE`, `STEPPING`, `STEP-COMPLETE`, `STATE-RESTORED`)
e frame rimanenti. Il frame è un confine PPU completato; non è un breakpoint a
istruzione. I backend mantengono la loro granularità di rientro.

## CLI sulla stessa finestra

Usare il runtime contenente questa funzione; le vecchie copie conservano il loro
eseguibile fissato. Creare una nuova sessione con quel runtime, senza sostituire
l'eseguibile di una sessione già autenticata. Si può aprire direttamente in pausa:

```powershell
.\recomp.exe debug-session open 'target\caso-debug' --paused
```

In un altro terminale:

```powershell
.\recomp.exe debug-control status 'target\caso-debug'
.\recomp.exe debug-control pause 'target\caso-debug'
.\recomp.exe debug-control step 'target\caso-debug' 3
.\recomp.exe debug-control step 'target\caso-debug' 1 --keys 3fe
.\recomp.exe debug-control resume 'target\caso-debug'
```

`step N` accetta **1–600 frame**. `--keys HEX` accetta **000–3ff**, active-low:
`3fe` tiene A premuto, `3ff` rilascia tutto. La maschera vale per tutti i frame
della richiesta. `resume` torna alla normale esecuzione e agli input correnti.
Per una sequenza variabile registrata usare [recording/replay](debug_recordings.md).
Una registrazione attiva nella copia può comprendere passi e pause: il tempo host
in pausa non aggiunge frame all'input registrato.

Ogni comando riceve JSON con identità di esecuzione/richiesta, successo/errore,
frame/clock/PC, motivo, budget restante, hash dello stato completo e dimensioni
delle code audio guest/host. L'hash viene calcolato soltanto su richiesta e a
completamento del comando, senza salvare uno snapshot aggiuntivo. Lo stato
restituito è quello del confine osservato: dopo `resume` il gioco continua.
Una richiesta incompatibile viene rifiutata senza applicarla e restituisce esito
nonzero; durante l'esecuzione libera il gioco continua comunque ad avanzare.
Un errore di esecuzione parziale non viene contato come frame completato.

Un solo client può attendere un comando operativo. **status**, **content-list** e **cancel**
usano un secondo mailbox con lock/risposta separati: restano disponibili durante
step o ricerca senza sovrascrivere il risultato in attesa. Nessun retry
automatico. F6/Escape e Load/Undo/Restart possono ancora interrompere il budget;
durante replay separato lo stato è leggibile, pausa/passi/ripresa sono rifiutati.

La console e `debug-command` possono scoprire e selezionare contenuti senza
aprire il selettore:

~~~text
content refresh
content list --offset 0 --limit 16
content list --offset 16 --limit 16 --generation GENERAZIONE
catalog select ID
~~~

`content refresh` scandisce esplicitamente le radici già dichiarate nella
copia, in un worker con gli stessi limiti del selettore; richiede pausa, nessun
passo, ricerca o replay attivo e nessun altro lavoro del selettore. La richiesta
resta pendente fino alla conclusione ed è
annullabile per ID: un risultato annullato o relativo a un contesto cambiato
non viene applicato. `content list` legge l'indice corrente senza riscansionare,
anche mentre un altro job è pendente. Il risultato è in `status.content_page`:
include generazione, totale, prossimo offset, indicazione di scansione parziale
e fino a 16 ID con tipo, hash, disponibilità e compatibilità. Specificare la
generazione ricevuta per le pagine successive: una diversa viene rifiutata.
Nome e percorso lunghi sono abbreviati con indicatori espliciti; l'ID conserva
il riferimento completo e la selezione lo ricontrolla. Le forme equivalenti
del trasporto diretto sono `debug-control content-refresh SESSION` e
`debug-control content-list SESSION [--offset N --limit N --generation N]`.
Un server precedente senza questa capacità viene rifiutato prima dell'invio.
Una prova Wine/Xvfb ha scandito una sessione sintetica e selezionato un catalogo
da pagine di un elemento, senza navigazione del menu né avanzamento del gioco.

Il catalogo cheat può essere consultato attraverso lo stesso canale locale:

~~~text
content refresh
content list
cheats select ID_CONTENUTO
cheats refresh
cheats list --offset 0 --limit 16
cheats show CH-0001 --offset 0 --generation SHA_SNAPSHOT
~~~

La selezione usa un ID `cheat-catalog` scoperto dal selettore; non accetta un
percorso nel linguaggio testuale. `cheats refresh` richiede una copia isolata
in pausa, ricontrolla il file selezionato in un worker e pubblica uno snapshot
immutabile identificato da hash della sorgente, ROM e generazione. Un cancel con
l'ID esatto o una scadenza fanno scartare il risultato: il worker termina la
lettura prima della risposta definitiva. Se selezione o contesto cambiano,
lo snapshot precedente non è più interrogabile. Le query `list` e `show`
usano il mailbox di sola lettura e restano disponibili durante altri job.
Il file non viene riletto a ogni query: lo snapshot certifica la verifica al
momento del refresh, non l'assenza di successive modifiche su disco.

La pagina contiene al massimo 16 righe con indicatori di troncamento. Il
dettaglio JSON originale è restituito in parti UTF-8 di massimo 4096 byte:
concatenare `json_part` seguendo `next_offset_bytes` e verificare
`entry_sha256`. Una generazione diversa o un offset non valido viene rifiutato.
Le forme dirette sono `debug-control cheat-select SESSION ID`, `cheat-refresh
SESSION`, `cheat-list SESSION [--offset N --limit N --generation SHA]` e
`cheat-show SESSION CH-ID [--offset N --generation SHA]`. Un server senza
`cheat-catalog-v1` viene rifiutato prima dell'invio. Nessuna di queste azioni
abilita, decodifica o applica un codice.

Le azioni esplicite `cheats prepare CH-ID INDICE MASTER GENERAZIONE` e
`cheats apply-once CH-ID INDICE MASTER GENERAZIONE TOKEN` usano lo stesso
controllo locale e richiedono la capacità `cheat-once-v1`. La prima restituisce
il piano, le aree e un token legato a selezione, contenuto, ROM e stato in pausa;
la seconda ricontrolla tutto e applica una volta. Il retry con la stessa identità
recupera l'esito senza ripetere le scritture. Prerequisiti e limiti, inclusi
interprete, regioni di memoria consentite e guardie su registrazione e nuovi
salvataggi, sono in [cheat](cheats.md#manual-one-shot-in-an-isolated-session--dbg2-19-in-progress).

Il profilo replay selezionato può essere avviato con `replay run`, oppure
`replay run audit`, `replay run capture` o `replay run compare`. Il trasporto
diretto usa `debug-control replay-run SESSION [--mode plain|audit|capture|compare]`.
La copia deve essere in pausa e il profilo compatibile deve essere già stato
selezionato: il comando non apre un selettore. L'esecuzione usa lo stesso
supervisore isolato del menu. `status.replay` distingue worker attivo,
avanzamento ricevuto e ultimo risultato; i suoi frame sono quelli del worker,
non della copia principale. Il risultato della richiesta conserva il report e
rimane pendente fino alla terminazione del worker. Un server precedente senza
`replay-job-v1` viene rifiutato prima dell'invio.
In una prova Wine/Xvfb con profili sintetici copiati, replay ha restituito
`passed`, capture `captured` e compare `matched`; audit ha riportato
correttamente una divergenza del punto finale, senza dichiararla un successo.
Un ritento del replay ha restituito lo stesso JSON e lo stesso run del worker.
Un ID errato è stato rifiutato mentre un altro replay era pendente. La prova
separata di cancellazione e scadenza usa un processo sintetico lento, non un
worker che esegue il gioco: conferma l'interruzione del supervisore e,
rispettivamente, `cancelled` e `timeout` terminali. Nelle prove la copia
principale è rimasta al frame e clock iniziali, con stato invariato.
Una seconda sessione sintetica ha inviato `replay run` dalla finestra Console
comandi e copiato il JSON terminale identico alla risposta del canale; la
clipboard e il display usati per la prova erano isolati.

## Richieste, cancellazione e scadenza — DBG-20

Gli accessi sono espliciti alla cartella della copia isolata. Nessuna porta,
socket o servizio di rete viene aperto; la partita principale non crea il canale.
Chi può scrivere nella cartella può controllare la copia: non è un confine di
autenticazione verso altri utenti del sistema.

**ready.json** dichiara schema1, run univoco per avvio, PID, capability e identità
della sessione (SHA256 del manifest e label limitata). Ogni richiesta/risposta
porta run e ID; il nuovo client confronta anche l'identità del manifest.
Una vecchia copia senza queste capability richiede una nuova sessione creata
con il runtime corrente; non sostituire eseguibili già autenticati.

Per interrogare un comando lungo e annullare soltanto quello osservato:

~~~powershell
.\recomp.exe debug-control step 'target\caso-debug' 600 --timeout-ms 120000
# Da un altro terminale, leggere status.pending_control.request_id:
.\recomp.exe debug-control status 'target\caso-debug'
.\recomp.exe debug-control cancel 'target\caso-debug' --request 'ID_ESATTO'
~~~

**cancel** ferma il budget step/ricerca CLI al prossimo rientro del frontend e
lascia la copia in pausa. Un ID vecchio/diverso viene rifiutato, senza annullare
un altro comando. L'operativo termina con outcome **cancelled**, remaining0 e
stato parziale effettivo; la query cancel conferma separatamente. Non riavvolge
i frame già completati. Alla chiusura una risposta senza stato finale non inventa un confine guest:
status è null e l’esito è closed. Per un replay avviato dal canale, cancel richiede
l'ID esatto e conferma soltanto la richiesta: il risultato terminale conferma
l'eventuale interruzione. Alla scadenza viene richiesta la cancellazione e il
risultato rimane pendente fino al termine del worker. Un replay avviato dal menu
usa il proprio Cancel replay;
un budget da menu/tastiera resta interrompibile con F6/Escape.

Tutti i comandi accettano **--timeout-ms N**,20–120.000ms, default120.000.
La richiesta include deadline_unix_ms: una richiesta già scaduta non viene
eseguita; un budget pendente scaduto viene fermato dal frontend con motivo
control-timeout. L'attesa del client usa un clock monotono. La scadenza server
è di tempo civile e può risentire di cambi dell'orologio; non interrompe
un'istruzione, un frame, una callback bloccata o un'operazione sincrona a metà.
Un'operazione sincrona già completata dopo scadenza restituisce timeout con
lo stato osservato, senza rollback.

Il client restituisce JSON anche per errori locali o assenza di conferma:
outcome **ok / refused / cancelled / timeout / closed**, oltre a
**identity-conflict / result-unavailable / capacity** per i ritenti, con ok,
error, run/id/session quando disponibili. Prima dell'invio, sintassi o argomenti
non validi producono `parse-error`, un protocollo, manifesto, capacità o run
incompatibile produce `incompatible` e una sessione o lock non disponibili
producono `unavailable`: questi esiti locali indicano `delivery: not-sent`.
Un errore di trasporto o del worker può invece riportare `delivery: unknown`:
non dimostra che il server non abbia ricevuto la richiesta. Il server continua
a riportare `refused` per i propri rifiuti, compresi quelli di compatibilità
specifici di un contenuto, con il dettaglio nell'errore. Sono possibili anche
`invalid-request` per un'azione tipizzata non valida e `composition-active`
quando la console blocca Invio durante la composizione. Un timeout client con
server_acknowledged=false non prova
che il frontend abbia già fermato il lavoro. Usare status e l'ID per controllare
prima di ritentare. La risposta server successiva può essere in result.json;
viene conservata solo l'ultima di ciascun mailbox. Query concorrenti vengono
rifiutate dal proprio lock OS; la morte del client lo rilascia automaticamente.
La risposta terminale di un job riporta `pending_control: null`: la richiesta
conclusa non compare come ancora attiva. Se la pubblicazione deve essere
ritentata, la correlazione conserva l'ID del job originale.

Il runtime corrente offre anche un ritento esplicito con la stessa identità:

~~~powershell
.\recomp.exe debug-control retry 'target\caso-debug' --run RUN --request ID --deadline-unix-ms SCADENZA --timeout-ms 5000 -- step 1 --keys 0x3ff
~~~

`RUN`, `ID` e `SCADENZA` sono quelli della richiesta originale; dopo `--` si
ripetono esattamente azione e argomenti. Sono presenti nei risultati e nel
timeout client. Si possono anche scegliere prima dell'invio e conservare per
un eventuale ritento. Il server confronta un'impronta dell'intera richiesta:
un ID con parametri diversi è rifiutato, quello pendente non è rieseguito e
quello concluso restituisce il risultato conservato. Il limite per avvio è
4096 identità, con uno spazio riservato all'annullamento del job attivo;
al massimo 128 risposte, 128 KiB ciascuna e 2 MiB serializzati complessivi restano
in cache. Se la risposta
non è più disponibile il server lo dichiara senza rieseguire l'azione. La
memoria vale solo per il run corrente: un run diverso rifiuta il ritento.
Un nuovo comando dotato di un altro ID costituisce un'azione distinta. Una prova
Wine/Xvfb ha verificato che il ritento di un passo concluso non esegue un altro
frame e che una ricerca pendente mantiene il suo proprietario; parametri e run
diversi sono stati rifiutati. Le risposte includono `request_sha256`, che il
client confronta prima di accettare una risposta del runtime aggiornato.
Il ritento richiede un server che supporti questa funzione; `status` è una
lettura non registrata e si interroga con una nuova richiesta. Anche il
linguaggio condiviso dalla console e da `debug-command` accetta, per esempio,
`retry RUN ID SCADENZA -- step 1 --keys 0x3ff`, oppure dopo `--` un comando
`profile select ID` o `catalog select ID`. La scadenza è il numero decimale
`deadline_unix_ms` originale. I campi della risposta JSON si copiano dalla
console con Ctrl+Shift+C. La richiesta è sempre esplicita: se identità o azione
originali non sono disponibili, non si deve inventare un ID per il recupero.
Non sono ammessi ritenti annidati, help o status; la console attende fino a
120 secondi. Una prova Wine/Xvfb ha ritentato un passo concluso dalla console:
stesso JSON, nessun secondo frame e rifiuto dei parametri cambiati.

Le richieste schema1 senza i nuovi campi opzionali restano decodificabili per
compatibilità locale, senza deadline garantita. Il client corrente pubblica
sempre deadline e manifest. Canale primario e query rifiutano azioni incompatibili;
richieste stale/malformate vengono consumate senza applicarle e diagnosticate
nel log della copia. La riapertura cambia run, quindi un risultato precedente
non conferma una richiesta nuova.

## Audio, pacing e file locali

La pausa invalida le code e la storia DSP frontend e silenzia l'uscita host.
Ogni passo drena i tap guest senza accodarli al dispositivo. La ripresa parte
con code vuote, nuova priming audio e scadenza host corrente; il tempo trascorso
in pausa non crea frame da recuperare. Lo stato dei generatori audio guest resta
parte della normale emulazione. L'ascolto e il ritardo del dispositivo fisico
sono distinti dalle verifiche delle code.

Il canale locale vive in `Data/config/debug-control/` della copia portable:
`ready.json`, `request.json`, `result.json` e lock client; query usa query.json, query-result.json e query.lock.
Pubblicazione atomica,
identità per processo e richiesta e lock OS separano avvii e client; vecchi
risultati non valgono come conferma di un nuovo comando. Si conserva soltanto
l'ultima risposta, senza timeline, nuovi slot o raccolte di snapshot. Tutti i file
restano nella sessione privata, ignorati da Git. Il canale viene chiuso con la
finestra e viene ricreato con nuova identità alla riapertura.

Prima di leggere una richiesta il server la sposta atomicamente nel proprio
file `request.claim` o `query.claim`, uno per canale. Una richiesta pubblicata
successivamente non può essere rimossa al posto di quella già acquisita; se
lettura o rimozione falliscono temporaneamente, la presa in carico viene
ritentata prima di acquisirne un'altra. Un residuo di un run precedente viene
scartato dalla verifica dell'identità e non eseguito. Input troppo grandi o
malformati vengono consumati e segnalati nel log, senza garantire una risposta
al client; un file non ordinario può richiedere
intervento. Gli errori distinguono canale e fase. Il mailbox resta a un solo
posto: dopo un timeout non confermato, una richiesta non ancora acquisita può
essere sostituita e non costituisce una conferma di esecuzione.

## Ambito della verifica

Le verifiche di questa unità sono registrate nel relativo record di modifica.
Il candidato Windows isolato è `target/debug-tools-step/recomp.exe`; i normali
eseguibili restano separati. Nessuna nuova accettazione ROM/DS o parità native,
benchmark, frontend Godot o modifica Android deriva da questi comandi.

## Verifiche del 28 ottobre 2026

Build Windows release feature/default senza warning, da `07be4d2` più cinque
file selezionati, con dipendenze bloccate/offline e output separati dalla produzione.
Passano **69 test Wine**: quattro del controllo, sei sessioni, dodici del frontend,
un reset audio del callback e 46 menu. Il primo tentativo sandbox non poteva
creare il socket Wine: nessun test era partito; gli stessi eseguibili passano
con l'eccezione per il socket. Nessun comportamento del prodotto è stato cambiato
per quel limite dell'ambiente.

**24 controlli CLI/GUI**, nove sul worker default e due processi controllati di
avanzamento passano. F7 tenuto oltre l'autorepeat, deliberatamente rimappato
anche ad A, produce soltanto un frame con input neutro. F8 e i due comandi menu
producono dieci/uno/dieci frame: il totale di 22 coincide nello stato completo
con un singolo comando CLI da 22. Le maschere A/rilascio esplicite funzionano.
Pausa e attesa conservano hash, frame e clock, con code audio guest/host vuote.
Le registrazioni di **22 frame a passi** e **80 frame con pausa/ripresa** superano
la verifica indipendente dello stato completo/clock con l'interprete continuo.

Sono verificati rifiuto dei limiti/mask incompatibili, client concorrente, comando
a sessione chiusa, interruzione con Escape, chiusura proprietario con budget ancora
attivo, ripresa F6/CLI/menu e rifiuto di passi mentre si esegue liberamente. La
build default esegue la funzione senza feature audit. La revisione ha aggiunto
il rifiuto di Replay/Choose profile/Audit/Capture/Compare durante un budget, prima
di aprire il selettore o avviare un worker. Dopo questa sola guardia e un commento
sono stati ricompilati entrambi i candidati e verificato il nuovo ramo nella GUI
default; i risultati precedenti sul resto del comportamento sono conservati.

Il primo script GUI assumeva il cursore iniziale del menu sulla riga Resume.
Il normale hover aveva selezionato un'altra riga e lo script era entrato nei
binding: tentativo rifiutato e conservato. Il test corretto usa un hit esplicito
su Debug e parcheggia il puntatore; le schermate finali sono state ispezionate.
Tutti i **202 input originali controllati**, inclusi i normali eseguibili Windows/
Linux, conservano hash e data. Tre sessioni accettate si chiudono normalmente;
risultati e payload privati restano in `target/debug-tools-step/`. Le directory
di pubblicazione managed non sono state create; il relativo helper ROM non si
applica a questa unità. Si conservano copie diagnostiche e preimmagini, riusando
la cache di build e sostituendo soltanto i candidati locali superati.

Windows nativo, Linux/Wayland, controller fisici, altre scene/ROM/dataset e passi
con traduzione native restano da verificare. Il controllo audio riguarda buffer
e callback, senza prova d'ascolto o latenza del dispositivo fisico. In questa prima verifica DBG-10, il timeout
client reale da 120 secondi e la morte forzata del server non sono stati simulati;
i risultati di un processo ucciso possono restare locali fino alla riapertura.

Per fermarsi su PC/modo o accessi bus, e leggere registri e memoria senza I/O,
usare [DBG-11: breakpoint, watchpoint e ispezione](debug_probes.md). Si tratta
di ricerche interprete limitate a un punto anche interno al frame, distinte
dall'avanzamento per frame completo descritto qui.

Per riunire framebuffer, layer, palette/tile/OAM e finestre in un report offline
usare [DBG-12: ispettore grafico](debug_graphics.md), dal menu o con
`debug-control graphics SESSION [--compare-last]`. La cattura resta readonly.

## UI, testo e input

`debug-control ui-start SESSION` inizia una raccolta limitata;
`ui-report SESSION` salva focus, metriche del font e sequenza osservata in un
report offline. Il campo `status.host_menu` espone anche il dropdown e la scheda
effettivamente attiva. Vedi [DBG-13: uso e limiti](debug_ui.md).

## Cattura audio

`debug-control audio-start SESSION [--frames 1..180] [--streams mixed,psg,hle,fifo0,fifo1]`
arma una finestra breve; `audio-report SESSION` pubblica il report dopo la pausa.
`status.audio_diagnostic` distingue raccolta, limiti e motivo di stop, mentre
`audio_report` indica l'ultimo HTML. Menu e CLI condividono la raccolta.
Vedi [DBG-14: buffer guest, callback, confronto e WAV locali](debug_audio.md).


## Codice, label e fallback

`debug-control code-start SESSION [--frames N --events N --start HEX --end HEX --mode arm|thumb|all --kind TIPO]`
arma il prefisso; `code-report SESSION` salva PC/modo/backing/disassemblato/label
e la scelta native/interprete. `status.code_trace` riporta limiti e filtri.
Vedi [DBG-15: significato del dispatch e limiti](debug_code.md).

## Report locale del problema

`debug-control issue-report SESSION [--profile FILE --symptom TEXT]` usa il profilo
selezionato e salva un indice locale, senza avviare il replay o inviare allegati.
`status.issue_report` indica il file. [DBG-16](debug_issues.md) descrive comando
riproducibile, metadati e allegati privati.


## Task, script e unità

`debug-control gameplay-start SESSION [--frames N --events N --family all|task|script|unit --context HEX]`
arma la raccolta nella copia in pausa; `gameplay-report SESSION` salva gli eventi
con provenienza per campo e correlazioni alle risorse. `status.gameplay_trace`
espone filtri e limiti; `gameplay_report` indica l'HTML.
[DBG-17](debug_gameplay.md) distingue contesti RAM, proiezioni ROM, sentinelle e
campi sconosciuti, senza inventare identità persistenti delle unità.

## Finestra recente e trigger d'errore

recent-start SESSION [--events N --reads on|off --start HEX --end HEX --on-error on|off]
arma il registro circolare. recent-freeze SESSION ferma la raccolta;
recent-report SESSION salva nella copia in pausa. Lo stato recent distingue
finestra, totali di espulsioni/scarti, stop e scrittura; recent_report conserva
l'ultimo percorso. [DBG-19](debug_recent.md) descrive letture e copertura parziale,
salvataggio automatico su errore e limiti.


## Verifica DBG-20 del 29 ottobre 2026

L'audit riusa i risultati già conclusi DBG-10/11/12/16 su pausa/passi, ispezioni,
catture, menu e profili spostati. Aggiunge soltanto le lacune reali: query durante
l'attesa, annullamento mirato, scadenza del budget e identità del manifest.

**30 test Wine distinti passati** (16protocollo,14frontend, un frontend già
ignorato): lock/risposte separati, ID/run/sessione/schema, azioni nel canale
sbagliato, richiesta scaduta/client timeout, ack tardivo e retry della sola
risposta query. La revisione della prova GUI corregge l'etichetta di chiusura
da refused a closed; il solo test protocollo interessato viene rieseguito
sul build finale, senza rifare i frame GUI o i30test già conclusi.

**19 invocazioni CLI sulla GUI default Wine/Xvfb**: query mentre un client
attende step600, ID errato rifiutato mantenendo il budget, cancellazione esatta
dopo10frame, stato fermo e audio vuoto/muto. Una scadenza250ms restituisce
timeout non confermato al client; la successiva query prova controllo fermo,
remaining0/control-timeout e risposta server con stesso ID. Cattura grafica
readonly via protocollo, manifest diverso rifiutato prima di pubblicare,
chiusura con lavoro pendente e rifiuto a finestra chiusa verificati.

Il primo runtime GUI resta autenticato nella copia; le sole differenze finali
sono classificazione della risposta closed, spaziatura help e relativo test.
Nessuna variazione del ciclo guest o comportamento di annullamento/scadenza.
Screenshot ispezionato, sei input/runtime normali con hash/mtime invariati.
Sorgenti, preimages, risultati, report e log conservati sotto
**target/debug-tools-control/**; nessuna nuova fixture ROM disposable o vecchia
accettazione ricreata. Nessun server di rete aggiunto.

Build Windows GNU release feature/default locked/offline senza warning.
Windows nativo/Linux GUI nativa, controller fisici, annullamento durante native
e cambio reale dell'orologio restano non verificati; timeout lungo120s non atteso
di nuovo, prova reale nuova250ms. Cancellazione di una ricerca usa lo stesso
rientro con interrupt_probe; nuova prova GUI focalizzata sui passi.
[Record DBG-20](changes/2026-10-29-complete-local-session-control.json).
