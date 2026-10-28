# Breakpoint, watchpoint e memoria — DBG-11

Questi strumenti operano sulla [copia isolata in pausa](debug_control.md).
Non scrivono valori in RAM e non installano patch nel codice. Un solo comando
può attendere il risultato del canale locale `debug-control`.

## Leggere registri e memoria

```powershell
.\recomp.exe debug-control inspect 'target\caso' 080003d6 32
.\recomp.exe debug-control inspect 'target\caso' 02000000 256
```

L'indirizzo è esadecimale, la lunghezza decimale (**1–256 byte**). Il report
include r0–r15, CPSR e modo ARM/Thumb, frame/clock, byte, regione e offset fisico
per ciascun indirizzo. La voce **Debug > Inspect PC / registers** legge 16 byte
dal PC corrente. L'ultima ispezione è in
`Data/config/debug-control/inspection.json` e nella risposta JSON del comando;
`status` può recuperarla insieme allo stato corrente. Frame e clock della sezione
`inspection` identificano il punto letto, che può essere precedente allo stato
corrente dopo un altro comando. Non viene generato uno snapshot aggiuntivo.

L'ispezione usa riferimenti immutabili ai buffer: BIOS backing, EWRAM/IWRAM,
palette, VRAM, OAM e ROM backing. Espone esplicitamente mirror e ripiegamento
VRAM, senza chiamare le funzioni Bus. **I/O, open bus e protocolli di salvataggio
vengono rifiutati**, inclusi keypad/timer e Flash/EEPROM. Nella ROM si vedono i
byte backing anche dove il normale bus indirizzerebbe GPIO/EEPROM: il report
non simula tali dispositivi né la protezione BIOS. Le finestre fuori dai buffer
e gli indirizzi che si avvolgono vengono rifiutati interamente.

Le label facoltative provengono da `debug-labels.toml` nella radice della copia:
formato [gba-labels](labels.md), SHA della stessa ROM, nomi e limiti dichiarati.
Una label identifica una dichiarazione; il suo `end` non dimostra codice eseguito.
Un file di un'altra ROM viene rifiutato. Il report registra l'hash degli stessi byte analizzati dal parser; non rilegge
il file per interpretarne una versione diversa.

Se la copia ha un dataset autenticato, gli owner sono intersezioni con i source
range del suo manifest. I range sovrapposti vengono autenticati contro la ROM;
identità del manifest, ID, offset, lunghezza e ruolo restano nel report. Nessun
match significa **owner sconosciuto**. Questi sono dati identificati: non provano
una lettura typed, raggiungibilità o nuova accettazione ROM/DS. Per le letture
realmente servite usare [l'audit DBG-06](debug_rom_audit.md).

## Fermarsi su PC e modo

```powershell
.\recomp.exe debug-control break 'target\caso' 080003d6 thumb --steps 100000
```

Il PC deve essere allineato per `arm` o `thumb`. La ricerca parte dal punto
corrente e si ferma **prima del passo interprete** quando PC e modo coincidono,
purché quel passo sia eleggibile per eseguire un'istruzione: Halt, consegna IRQ
e stub di ritorno BIOS HLE non vengono presentati come istruzioni target.
Un match già al punto corrente esegue zero passi e conserva lo stato guest.
La condizione CPU dell'opcode non viene valutata dal debugger: un'istruzione
condizionale che non esegue il proprio corpo resta un ingresso istruzione.

## Fermarsi su accessi bus

```powershell
.\recomp.exe debug-control watch 'target\caso' read 03007ffc 03008000 --width 4 --steps 100000
.\recomp.exe debug-control watch 'target\caso' write 02000000 02000004 --width 4 --value 00000001 --steps 100000
```

Serve la feature **rom-data-read-audit** nel runtime della copia. Il range
`START END` è guest ed esclusivo alla fine, senza normalizzare mirror. Si
seleziona `read` o `write`; `--width 1|2|4` e `--value HEX` filtrano la chiamata
bus osservata e il suo valore. Un valore deve entrare nella larghezza scelta.
Il debugger non legge un indirizzo I/O per controllare il valore: confronta
quello dell'accesso realmente eseguito dal gioco. Per le scritture è il valore
richiesto al bus, anche quando il dispositivo ignora o trasforma la scrittura.

Il primo match viene conservato, escludendo i fetch interprete. L'arresto avviene
**dopo il passo interprete completo**, compresi HLE, DMA e tick hardware avviati
da quel passo. La sezione `site` descrive il punto prima del passo; non attribuisce
automaticamente l'accesso a un load/store dell'opcode. Gli accessi implementati
tramite chiamate bus annidate possono comparire alla larghezza interna se non si
specifica `--width`. Il primo accesso abbinato resta quello dichiarato nel report.
Accessi host diretti ai vettori, fetch native e codice interno a blocchi native
non sono intercettati da questo watchpoint.

## Limiti, ripresa e interruzione

Entrambe le ricerche richiedono copia in pausa, interprete, nessun budget di
frame, recording o replay attivo. Il backend native viene rifiutato esplicitamente:
non si dichiara un arresto per istruzione dentro i suoi blocchi. L'ispezione
readonly può invece descrivere una copia native ferma, senza eseguirla.

`--steps` accetta **1–5.000.000**, con default **100.000**; esiste anche un limite
di **600 frame completati**. La ricerca avanza in tranche di massimo 2.000 passi e lascia il menu
responsivo. Durante la ricerca il contatore laterale indica i passi rimanenti;
negli avanzamenti DBG-10 indica i frame. Usa input guest neutro. Un match restituisce `probe.outcome:
"hit"`; limite esaurito restituisce `"not-reached"`. Entrambi completano il
comando con successo di trasporto e lasciano la copia in pausa. Leggere il JSON
per distinguere i due esiti. `status.probe` conserva l'ultima ricerca terminata;
un comando incompatibile non la sostituisce. Per l'esito della richiesta corrente
leggere `ok/error` e la sua identità, senza scambiarla con quel risultato precedente.
Frame, clock e snapshot hash descrivono il punto di arresto effettivo, che può essere dentro un frame; non è un frame completo fittizio.

F6/Escape, Resume, Load/Undo/Restart e chiusura possono interrompere una ricerca.
Un comando pendente riceve l'interruzione; il timeout client di 120 secondi resta
un'attesa scaduta, non una cancellazione. Non ci sono retry automatici.
La ripresa o il normale Step 1 completano l'emulazione dal punto corrente senza
riavvolgere. I tap guest prodotti in un frame parziale restano fino al successivo
frame completo; l'audio host resta silenzioso e senza backlog mentre si cerca.
Le immagini mostrate sono l'ultimo frame completato, non un rendering inventato
del punto parziale. Le impostazioni e i cambi espliciti di timeline della copia
restano sotto il controllo dell'utente.

Se Windows impedisce temporaneamente di sostituire il risultato perché un client
lo sta leggendo, il server conserva la stessa risposta e ne ritenta la pubblicazione
nei tick successivi, prima di consumare altre richieste. Non riesegue il comando.
Alla chiusura il tentativo di pubblicazione finale è limitato a circa mezzo secondo;
un errore I/O persistente resta segnalato dal timeout del client e dai log.

I file restano privati nella copia, ignorati da Git. Si conservano solo ultima
ispezione e ultima risposta, senza una cronologia di dump. Il runtime candidato
isolato è sotto `target/debug-tools-probes/`; il normale eseguibile resta separato.

## Verifiche del 28 ottobre 2026

Build Windows release feature/default senza warning da `07be4d2` più undici
file selezionati, includendo le basi DBG-10 già accettate ed escludendo il codice
concorrente di estrazione. Le verifiche Wine coprono **88 test distinti**: cinque
ricerca/ispezione, tre core readonly/watch, 25 regressioni bus, sei controllo/canale,
46 menu e tre parser label. Esecuzioni: 84 iniziali, otto mirate dopo il parsing
dal buffer autenticato e sei sul controller dopo la correzione della risposta.
I controlli non modificano Machine, salvataggi o ABI native.

**21 controlli CLI/GUI** sulla copia DS-20 verificano letture readonly di 256 byte,
mirror RAM, registri/label, rifiuto I/O/backup/wrap/lunghezza/ROM estranea senza
cambiare hash completo/frame/clock. Tre trigger reali passano:

- PC Thumb `080003d6`, prima del passo dopo 66.720 passi di ricerca, clock
  6.241.285.276 nel frame guest 22.219. Un secondo match al punto corrente esegue
  zero passi e conserva esattamente lo stato;
- lettura word `03007ffc` con valore `03000f10`, arresto dopo il passo di servizio
  IRQ: la sezione `site` segnala `eligible_instruction: false`;
- scrittura word `03007f78` con valore richiesto `6000003f`, arresto dopo il passo.

Passano anche un limite di un passo senza match, attesa immobile al punto parziale
e successivo completamento di un frame. Test sintetici confrontano lo stato
completo con esecuzione non osservata, escludono i fetch e verificano provenance
degli owner e rifiuto di fonti alterate. Questi owner sintetici non costituiscono
una nuova accettazione di risorse reali.

**Nove controlli finali sulla build default**, più due ricerche controllate,
verificano PC/ispezione senza audit, rifiuto watch senza feature, hash della stessa
sorgente label analizzata, menu readonly, recording/replay rifiutati durante la
ricerca, annullamento F6 e chiusura del proprietario con conferma di interruzione.
Le schermate sono state ispezionate. Tutti i **202 input originali controllati**
e i normali eseguibili Windows/Linux conservano hash e data.

La prima prova delle guardie premeva Return prima che il client Wine avesse
consegnato la richiesta; la registrazione partiva legittimamente e la ricerca
la rifiutava. Conservata come errore di ordinamento del test, senza ripetere i
21 controlli già conclusi. Una successiva prova ha trovato una corsa reale nel
canale Windows: pubblicazione di `result.json` negata mentre il client leggeva
il vecchio file, con timeout effettivo di 120 secondi. Il server ora conserva
la risposta da ripubblicare e un test ne forza fallimento/recupero senza rieseguire
il comando. Le prove finali passano anche quando il log rileva un rifiuto transitorio.
Il tentativo Cargo con `--lib` sul solo package binario è stato rifiutato prima
della compilazione; corretto soltanto il comando di test e conservato il log.

La verifica finale autentica gli undici overlay, i candidati e gli originali;
conferma che gli hook conservino le espressioni originali del bus e che il ciclo
di ricerca sia identico a quello delle tre prove accettate. Risultati/preimmagini/
tentativi rimangono in `target/debug-tools-probes/`. Nessuna directory di pubblicazione
managed o profilo browser è stata creata: il relativo cleanup ROM non si applica.

Windows nativo, Linux/Wayland, controller fisici, altre scene/ROM e ricerca native
restano fuori dalla validazione. Il limite di 600 frame e i 5.000.000 passi sono
controllati nel codice e nei limiti di configurazione; non è stata completata una
ricerca reale di tale durata. Il processo ucciso forzatamente e gli errori I/O
persistenti restano distinti dalla chiusura pulita verificata. Non sono stati
eseguiti benchmark, nuove accettazioni ROM/DS, riparazioni native o lavoro Godot.
