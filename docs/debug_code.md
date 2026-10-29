# Codice, label e fallback — DBG-15

Nella copia isolata in pausa, **Debug > Start code trace** arma una raccolta di
un frame e al più 512 eventi. Avanzare o riprodurre il breve passaggio, tornare in
pausa e scegliere **Save code report**. Aprire `Data/config/code/<run>/index.html`:
la sequenza collega PC/modo, backing ROM o RAM corrente, disassemblato al punto
d'ingresso, label dichiarate e scelta effettiva del runtime. `code.json` contiene
gli stessi dati. Non vengono creati snapshot né abilitate misure di prestazione.

## CLI equivalente

Aprire la [sessione isolata](debug_sessions.md) con il runtime contenente lo
strumento. Copie precedenti conservano il proprio eseguibile autenticato.

```powershell
.\recomp.exe debug-session open 'target\caso-codice' --paused
```

Da un secondo terminale:

```powershell
.\recomp.exe debug-control code-start 'target\caso-codice' --frames 1 --events 128 --start 08000000 --end 0a000000 --mode thumb --kind all
.\recomp.exe debug-control step 'target\caso-codice' 1
.\recomp.exe debug-control code-report 'target\caso-codice'
```

Start e Save richiedono pausa, senza probe, passi pendenti o replay separati.
`status.code_trace` mostra filtri, conteggi e stop; `code_report` indica l'ultimo
HTML. Una raccolta non salvata impedisce un secondo Start e l'avvio di un replay
separato. Load/Undo/Restart congelano il prefisso. Lo stato corrente del report
può quindi appartenere alla timeline ripristinata, identificata separatamente.
Chiudere senza Save perde la raccolta in memoria.

Parametri: **1–60 frame**, **1–512 eventi**, 30 secondi host; `--start` incluso e
`--end` escluso sono indirizzi esadecimali. Modo `arm`, `thumb` o `all`; tipo
`all`, `native`, `missing`, `system`, `hook` o `interpreter`. I valori predefiniti
coincidono con il menu. I filtri di cattura escludono eventi prima del dump;
ricerca e filtro nel browser agiscono soltanto sul prefisso già conservato.

## Leggere il passaggio

- `native`: la tabella ha selezionato un ingresso compilato per il PC/modo
  esatto. Il report identifica la chiave, senza inventare corpo o lunghezza
  del blocco. Non osserva le singole istruzioni interne alla funzione native.
- `missing`: nessun ingresso compilato per quella chiave; il runner esegue il
  normale passo interprete. ARM e Thumb sono chiavi differenti.
- `system`: passo per Halt/risveglio, IRQ pendente o ritorno IRQ HLE. Può non
  eseguire alcuna istruzione al PC mostrato; il disassemblato viene omesso.
- `hook`: il runner ha scelto l'interprete per il normale hook mouse battaglia
  o sprite, prima della ricerca di un blocco. Non è una mancanza della tabella.
- `interpreter`: backend interprete selezionato, senza lookup native. La presenza
  di una label statica non cambia questa scelta.

Gli eventi precedono il dispatch/passo; il report conserva l'endpoint al confine
osservato e il motivo del limite. I byte vengono letti dal backing immutabile o
dalla RAM corrente, con offset fisico e distinzione dei mirror, senza simulare
fetch o leggere protocolli I/O. Il disassemblato è quello di una word ARM o
halfword Thumb all'ingresso, senza ricostruire una funzione o fondere BL Thumb.
Il codice RAM può cambiare nel tempo; il dump conserva soltanto il punto letto.

## Label per la ROM scelta

Mettere una copia esplicita del proprio file di [label](labels.md) nella root
della sessione come `debug-labels.toml`. Si riconoscono v1 testuale e v2 TOML dal
contenuto. Start legge al più 1 MiB, autentica la ROM e fissa l'hash degli stessi
byte che vengono analizzati. Una ROM diversa rifiuta l'avvio con errore leggibile
e risposta CLI; non viene modificato il file sorgente o l'accumulatore normale.
In assenza del file le label risultano non selezionate.

I nomi e gli intervalli sono **dichiarazioni statiche**. La correlazione con un
PC osservato non dimostra l'estensione di una funzione, la completezza del codice
o una nuova risorsa ROM. Il report mostra al più 64 nomi statici e quattro match
per evento, con prefissi di 128 caratteri e omissioni esplicite. Gli ingressi
ROM, IWRAM e quelli riservati restano conteggi distinti. Non viene calcolata
alcuna percentuale di codice estratto dai numeri di ingressi o dispatch.

## Identità e conservazione

Il report fissa runtime/build, ROM/BIOS, seed e altri pin del manifest della
sessione, stato iniziale/corrente, input/AV e dataset effettivo se presente.
Per native conserva il percorso usato dal loader, la dimensione della tabella
e l'hash del file letto all'avvio della traccia. Quest'ultima lettura **non
autentica retroattivamente l'immagine di codice già caricata in memoria**;
modifiche esterne del file fra load e Start non sono dimostrate assenti dal solo
report. I puntatori host e le lunghezze delle funzioni native restano sconosciuti.

Al più otto cartelle, compresi tentativi incompleti, e 2 MiB JSON. Nessuna
sovrascrittura o cancellazione automatica. Un errore di pubblicazione conserva
la traccia per un altro Save. Dati, byte di codice e percorsi sono privati,
ignorati da Git. La [homepage](report_home.md) li trova dopo rigenerazione.

L'osservazione è disabilitata normalmente. Il runner native con observer vuoto
mantiene il percorso usuale; con raccolta attiva si aggiunge una lettura readonly
prima del dispatch. Il loop interprete osservato mantiene la stessa successione
di `Machine::step`, senza nuovi campi guest o cambi al formato degli stati/ABI.
I limiti terminano la raccolta, senza interrompere a metà un frame normale.
Gli eventi successivi allo stop sono sconosciuti, non pari a zero.

Le prove effettive e i limiti sono nel
[record DBG-15](changes/2026-10-29-add-bounded-code-dispatch-diagnostics.json).
Il confronto di un observer con il medesimo runtime native non riapre né
certifica la parità interprete/native o vecchie accettazioni ROM/DS.


## Verifiche del 29 ottobre 2026

Build Windows feature/default bloccate e offline, senza warning. **74 test Wine**:
73 ordinari (collector, controller, menu, frontend e wrapper cost) e una prova
opzionale con ROM/seed/libreria locali espliciti. La prova sintetica ricostruisce
sei ingressi compilati e sei mancanti con stato identico. Quella sulla libreria
esistente confronta un solo frame osservato/non osservato, senza writer o nuovo
snapshot: stato completo/clock e 6.547 blocchi / 14.991 passi sono identici.
Il prefisso limitato conserva 41 native, 470 missing e un IRQ; eventi 17/18:
`08145c10 Thumb` → `0801bde8 Thumb`, con motivo del fallback esplicito.
Questa prova riguarda il nuovo observer e non certifica quella libreria contro
l'interprete.

La copia GUI default verifica Start/Save da menu e CLI, filtro IWRAM/ARM con
label autentica e 16 eventi, prefisso default da 512 e regione senza match con
106.554 dispatch esplicitamente esclusi. Una registrazione di due frame A/rilascio
raggiunge l'endpoint completo esatto dopo Stop e chiusura; la copia termina con
server inattivo. Passano label ROM diversa, parametri invalidi, doppio Start,
otto cartelle, esecuzione libera e sessione chiusa. I sei input/runtime realmente
selezionati conservano hash/mtime; nessun eseguibile normale viene sostituito.

Chrome desktop e layout stretto verificano ricerca PC/label, filtri dispatch,
dettaglio, prefisso vuoto e assenza di overflow della pagina. La homepage include
41 indici, quattro di codice. Screenshot ispezionati; payload e prove restano
in `target/debug-tools-code/`. Windows/Linux nativi, controller fisici, altre
scene e cattura GUI native restano da verificare.
