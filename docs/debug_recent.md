# Registro degli eventi recenti — DBG-19

DBG2-08 collega gli eventi host allo [schema comune](debug_events.md): identità,
ordine host, correlazione e mappa della copertura condivisi fra UI e recent.
I campi `sequence`/`host_ms` originali restano locali al collector.

Nella [copia isolata](debug_sessions.md) in pausa, scegliere **Debug > Start
recent events**. Il registro mantiene una finestra mobile; gli eventi più vecchi
vengono espulsi mentre il gioco continua. **Freeze / save recent events** congela
e salva il contesto in Data/config/recent/&lt;run&gt;/index.html e recent.json.

Per impostazione predefinita un errore dei comandi condivisi menu/CLI o un errore
fatale osservato dal frontend congela e salva automaticamente il registro.
Il report contiene il messaggio e il contesto precedente; non invia nulla.
Non vengono creati nuovi snapshot o dump continui dell'intera macchina.

## CLI

Aprire la copia con il runtime che contiene lo strumento; le vecchie copie
conservano il proprio eseguibile autenticato:

~~~powershell
.\recomp.exe debug-session open 'Data/debug-sessions/caso' --paused
~~~

Da un secondo terminale:

~~~powershell
.\recomp.exe debug-control recent-start 'Data/debug-sessions/caso' --events 1024 --reads on --start 00000000 --end 02000000 --on-error on
.\recomp.exe debug-control step 'Data/debug-sessions/caso' 10
.\recomp.exe debug-control recent-report 'Data/debug-sessions/caso'
~~~

**recent-freeze SESSION** congela senza scrivere; può essere usato anche durante
esecuzione libera, al confine gestito dal frontend. Per salvare esplicitamente
servono pausa e assenza di passi/probe/replay pendenti. Non interrompe un frame
guest a metà. **status.recent** espone finestra, contatori, stop ed eventuale errore
di scrittura; **recent_report** indica l'ultimo HTML. Le query status non riempiono
il registro.

**--events** ammette **1–1.024**; **--reads on|off** seleziona le letture strumentate;
**--start** incluso e **--end** escluso sono **offset ROM fisici** esadecimali,
entro 02000000. Non sono indirizzi PC. **--on-error off** conserva gli errori
come eventi e continua a scorrere; il default è on.

Un nuovo Start richiede che la finestra precedente sia salvata. Un doppio Start
è un errore osservato e può quindi congelare/salvare la vecchia finestra se
on-error è attivo. Un ripristino della timeline congela con motivo
state-restored; origine, limite congelato e stato corrente rimangono distinti.
Un replay separato richiede prima Save. Chiudere normalmente senza salvare
perde il buffer in memoria. Un errore d'uscita osservato tenta il salvataggio;
un processo ucciso senza callback non può esportare la propria memoria.

## Leggere il contesto

Gli eventi conservano sequenza, tempo host monotono, frame e clock guest:

- **guest-input**: latch active-low effettivamente usato prima del frame.
  Il report mantiene anche l'ultimo input osservato; i frame riportano i tasti
  anche quando l'evento del cambiamento è stato espulso. I poll grezzi dei
  dispositivi appartengono all'[ispettore UI](debug_ui.md).
- **recognized-ui-transition**: variazione della firma frontend readonly,
  con selettore riconosciuto, target limitati, segnali dialogo/pannello e name
  input. Include cambi di selezione. La copertura è parziale; un recognizer
  disabilitato o senza match rimane sconosciuto, non prova assenza di una scena.
  Non vengono inventati ID globali di scena o unità.
- **rom-read-batch**: fino a **16 chiavi uniche selezionate per frame** dal
  collector esistente, con servizio effettivo Typed, Raw o Unclassified,
  lettore, indirizzo/offset/ampiezza e scope interprete quando disponibile.
  Le chiavi sono aggregate e ordinate: non costituiscono l'ordine temporale
  delle singole letture. Fetch esclusi; scope HLE/DMA può essere ereditato.
- **frame**: input, confini e completamento, backend scelto e conteggi osservati
  native/missing/system/hook. Il backend interprete non effettua lookup
  native; solo native/missing dimostrano una selezione della tabella.
  Gli interni dei blocchi native e l'identità dell'immagine caricata restano
  fuori ambito.
- **control-request/result**, **error**, **freeze**: comandi condivisi, risultato,
  messaggio limitato, PC/tasti e motivo di stop. Altri canali di errore non
  collegati o un crash senza callback restano non osservati.

Le letture richiedono il build con rom-data-read-audit. Nel build ordinario
feature-unavailable è distinto da disabled, da collisione dell'observer e da
zero richieste nella finestra/range. Se un altro collector possiede già
l'observer, il gioco prosegue con indisponibilità esplicita. I gruppi persi per
cap sono conteggiati; non dimostrano non-uso globale di risorse o funzioni.

Nel form dei parametri, **Recent** con `--reads on` viene rifiutato in fase di
validazione se il runtime non include `rom-data-read-audit`; `--reads off` resta
valido in entrambe le build. Il test del form verifica separatamente questi due
casi, senza aspettarsi che un build ordinario disponga degli hook opzionali.
Gli esiti della verifica mirata sono nel
[record della correzione del test](changes/2026-10-30-correct-recent-form-feature-expectations.json).

## Limiti, identità e conservazione

Il buffer conserva al più **1.024 eventi / 256 KiB di eventi serializzati**,
con **8 KiB per evento**. Un evento troppo grande viene scartato e conteggiato;
non espelle il contesto valido. La firma UI è limitata a 2 KiB, l'origine a
128 KiB; il messaggio d'errore mantiene al più 1.024 caratteri con troncamento
esplicito. Anche con un solo posto resta l'errore che ha congelato la finestra.

Non esiste il limite di durata dei collector a prefisso: il registro continua
a scorrere oltre 30 secondi. Sequenze iniziale/finale, host ms e frame/clock
definiscono la finestra effettiva. I totali di tentativi/espulsioni/scarti e delle
letture appartengono all'intera raccolta, non soltanto agli eventi conservati.
Dopo Freeze il suffisso è non osservato, non pari a zero.

Start e Save identificano stato completo tramite hash soltanto ai confini
espliciti, più ROM/BIOS, runtime/build, manifest/pin della sessione, dataset
effettivo, input e AV. Nessuna nuova proprietà guest, ABI o misura di prestazione.
Il report è diagnostico e non certifica nuove scene, risorse o parità native.

Al più **otto cartelle / 2 MiB JSON per report**, tentativi incompleti inclusi;
nessuna sovrascrittura o pulizia automatica. Save fallito conserva il buffer
congelato e il motivo. Save di una finestra già pubblicata restituisce lo stesso
percorso. I percorsi e i metadati sono privati, ignorati da Git; rivederli prima
di condividerli. La [homepage](report_home.md) li trova dopo rigenerazione.
Il [gestore degli artefatti](debug_artifacts.md) conserva questi report e mostra
i relativi limiti.

Le prove effettive e i limiti di piattaforma sono nel
[record DBG-19](changes/2026-10-29-add-recent-event-ring.json).

Il 30 ottobre sono stati eliminati i warning `unused_mut` nel controllo delle
directory di output, conservando il rifiuto dei link simbolici e dei reparse
point Windows. I blocchi modificati compilano senza warning per Linux e Windows;
è una verifica mirata del compilatore, non una nuova prova GUI o del report.
Vedere il [record della correzione](changes/2026-10-30-remove-report-link-flag-warnings.json).

## Verifica del 29 ottobre 2026

Build Windows GNU release locked/offline, con e senza audit, senza warning.
**81 test distinti Wine passati**:79 feature (rolling di un milione di eventi,
entrambi i cap, oversize, errore con un solo posto, salvataggio fallito, servizi
reali/collisione e dispatch più menu/control/frontend), un observer reale e un
test default non disponibile. Due primi test cercavano il batch nella posizione
iniziale occupata dall'input; corretti soltanto nei test. Tentativi e risultati
restano conservati. Gli11test Python interessati dal riepilogo dei limiti passano.

L'observer nuovo usa il seed DS20 già conservato: stessi due frame A/rilascio con
e senza osservazione hanno identico stato completo a ogni confine, fino al frame
22221/clock6241706243. Non ripete vecchie accettazioni ROM/native. La GUI feature
usa25invocazioni CLI: dopo26frame una finestra di8eventi ha60espulsioni e conserva
input/firma UI/frame/letture più l'errore controllato **Start a code trace first**.
Il salvataggio automatico non cambia lo stato guest. Menu Start/Save resta
attivo oltre31secondi in pausa; Freeze durante esecuzione libera, letture off,
idempotenza e cap delle otto cartelle verificati. La GUI default usa cinque
invocazioni e pubblica feature-unavailable senza inventare zero letture.

Il runtime default era già uscito con0 quando il vecchio helper di chiusura
ha cercato una variabile log non definita: ready inattivo, lease rilasciato,
risposta/report e integrità verificati con letture successive, senza rilancio.
Chrome offline desktop/stretto verifica cinque viste, filtri/ricerca/reset,
errore e indisponibilità; screenshot ispezionati. Homepage67indici/5recenti
al momento della prova, senza riaccettare gli altri report.

Verificatore finale autentica19overlay sorgenti, due runtime candidati e cinque
coppie HTML/JSON con contabilità byte/sequenze. Sei input/runtime normali
mantengono hash e mtime; helper chiusi. Ritirati soltanto cinque profili browser
terminati; report, input, sorgenti/preimages, tentativi, log e candidati conservati.
Nessuna fixture disposable managed/native: il completion helper ROM non si
applica. Evidenze locali in **target/debug-tools-recent/**. Windows nativo,
Linux GUI nativa, controller fisici e osservazione reale native restano non
verificati. Nessuna sostituzione del runtime normale, misura prestazionale,
pubblicazione o modifica Android/Godot.
