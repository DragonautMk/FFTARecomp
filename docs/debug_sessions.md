# Sessioni di debug isolate — DBG-02

Una sessione è una copia portable del caso scelto. Contiene un eseguibile,
ROM, eventuale BIOS e dataset, configurazione, salvataggio e snapshot iniziale.
La copia lavora nella propria cartella: salvataggi periodici, uscita, slot,
Undo, backup, impostazioni e cache non usano i percorsi della partita originale.

Per partire da una scena della raccolta usare il [catalogo DS](debug_catalog.md):
**Debug → Debug catalog → Open isolated case** oppure `debug-catalog open`.
La copia conserva nome, provenienza e voce selezionata; gli snapshot si possono
anche importare in uno slot libero della copia, senza caricarli automaticamente.
Per registrare una riproduzione dal punto corrente usare
[Start/Stop input recording](debug_recordings.md); `debug-record start` avvia
la stessa registrazione dalla CLI in una copia chiusa con snapshot iniziale.
Da una registrazione completa si può creare un [profilo di replay](debug_replays.md):
menu e CLI eseguono il caso in un worker separato, senza scrivere i progressi
della sessione sorgente o della partita normale.

## Dal gioco

Mettere in pausa e aprire **Debug → Start debug session**. Il runtime cattura
il punto attuale e la memoria del salvataggio, copia la configurazione corrente
e apre una seconda finestra. La partita normale resta in pausa. La nuova
finestra porta il titolo **FFTARecomp - DEBUG: nome**; il suo menu mostra anche
un indicatore **DEBUG COPY** nel margine (abbreviato nelle finestre strette),
visibile a schermo intero.

Nella copia si possono caricare snapshot, salvare, annullare e riavviare usando
le funzioni già presenti. **Debug → Close debug session** chiude la copia;
ritornare alla finestra originale e scegliere Resume riprende la partita normale.
Non c'è una promozione automatica dei progressi di debug nella partita personale.
Nella copia sono bloccati Start debug session e Switch ROM: per cambiare caso si
chiude prima la copia. La creazione è rifiutata durante una registrazione input.

Le sessioni create dal menu sono sotto `Data/debug-sessions/` in modalità
portable, oppure `debug-sessions/` sotto la directory di configurazione installata.
Il log indica la cartella completa. Dopo otto cartelle, il menu chiede di
riaprire una sessione o spostare quelle da conservare: non elimina prove o errori.

## Dalla CLI

Esempio PowerShell:

```powershell
.\recomp.exe debug-session create 'C:\Giochi\FFTA EUR.gba' `
  --dir 'C:\Debug\DS-20' --sav 'C:\Salvataggi\partita.sav' `
  --state 'C:\Debug\DS-20.ffstate' --no-bios --interp --open
```

`--open` è facoltativo. Senza questo flag viene creata la cartella e stampato
un risultato JSON, senza avviare il gioco. La destinazione deve essere nuova.
Il salvataggio e lo snapshot sorgente vengono letti e copiati, mai spostati.
Se si omette `--sav`, valgono le regole di scelta del controllo `diagnose`;
una scelta ambigua richiede un percorso esplicito.

Per riaprire o controllare una cartella:

```powershell
.\recomp.exe debug-session inspect 'C:\Debug\DS-20'
.\recomp.exe debug-session open 'C:\Debug\DS-20'
```

`inspect` restituisce il manifest e verifica i file, senza eseguire il gioco.
`open` aspetta la chiusura della copia e propaga l'esito del processo. Si può
anche aprire direttamente il `recomp.exe` nella cartella. Linux usa la stessa
sintassi con `./recomp`; il binario copiato conserva il permesso di esecuzione.
La verifica documentata di questa unità riguarda Windows sotto Wine.

`--bios FILE` copia un BIOS reale, `--no-bios` sceglie HLE. `--dataset DIR`
copia un dataset già autenticato; `--rom-data` seleziona quello gestito corrente,
senza estrarlo o ripararlo. Sono opzioni alternative. Senza dataset il runtime
usa la ROM normalmente. Il menu copia il dataset effettivamente collegato.

`--interp` sceglie l'interprete. Per una copia native serve il TinyCC del runtime
sorgente: la libreria viene copiata nella sessione e le traduzioni sono create
nella sua cache. La creazione richiede il runtime di sviluppo; i pacchetti
full-recomp con manifest di distribuzione non vengono convertiti da questo tool.

## Riapertura e spostamento

Lo snapshot iniziale resta in `seed.ffstate` ed è copiato anche nello slot 1.
Ogni apertura della sessione con uno snapshot riparte da quel punto; gli altri
slot e le modifiche ai salvataggi restano nella copia e sono caricabili dal menu.
Restart ROM usa il normale riavvio con la memoria cartridge attuale, senza
caricare implicitamente il seed né ruotare i backup. La chiusura e riapertura del
processo crea invece il normale backup iniziale del salvataggio della copia.

Chiudere la sessione prima di spostare **tutta la cartella**. Manifest e input
usano nomi relativi; `portable.txt` individua la directory dal vero eseguibile,
anche se il comando parte da una directory di lavoro diversa. Non modificare
file o collegamenti mentre la sessione è aperta.

Il manifest `debug-session.json` identifica il caso e conserva gli hash di ROM,
BIOS, eseguibile, seed, dataset e TinyCC. Salvataggi, configurazioni e cache sono
mutabili. `session.lock` impedisce due aperture simultanee della stessa copia;
`last-result.json` conserva l'ultimo esito di avvio/uscita. Il report di
[diagnosi](debug_diagnostics.md) si trova nella configurazione della copia.

Gli avvii tramite `open` o menu eliminano le variabili ereditate `FFTA_*`,
`RECOMP_*` e `GBA_*`: gli override non possono riagganciare il salvataggio, la
registrazione o i dump personali. L'avvio diretto con tali variabili presenti
viene rifiutato. Gli override di `play` nella copia vengono rifiutati: usare
`debug-session create` per configurare un nuovo caso.

La cartella nuova riceve subito un `.gitignore` che ne esclude tutto il contenuto.
ROM, BIOS, dataset, snapshot e salvataggi restano privati; non vanno inseriti
nei pacchetti. La copia iniziale è limitata a 768 MiB; l'ispezione rifiuta alberi
oltre 2 GiB, 4096 voci o 16 livelli e file collegati. Una copia incompleta per
un errore I/O rimane per la diagnosi, priva di un manifest valido: non viene
avviata né rimossa automaticamente.

## Verifica del 27 ottobre 2026

Build Windows release offline con dipendenze bloccate, da una copia pubblica
stabile dei sorgenti più le modifiche di questo strumento. L'estrazione in corso
e gli eseguibili di produzione restano separati dagli output della compilazione.

Sotto Wine sono passati sei test delle sessioni, 41 del menu, dieci degli slot e
sette dei backup; il precedente test backup che richiede un dataset specifico
resta ignorato. I 41 test del menu finale sono passati anche dopo la correzione
visiva dell'indicatore. Sette casi CLI reali EUR verificano creazione, ispezione,
copia di TinyCC e rifiuto di destinazioni/input/override incompatibili.

Le prove Wine/Xvfb verificano partenza dallo snapshot, dataset copiato, esclusione
degli override esterni, apertura esclusiva, Save/Load/Undo, riavvio, ripristino
backup e chiusura. La cartella spostata si riapre da una directory di lavoro
estranea, conservando snapshot e due backup. La copia creata dal menu si chiude
normalmente; tornando alla finestra originale, frame, clock e file del progresso
sono invariati. Tutti i 73 file sorgente protetti conservano hash e date, così
come i due eseguibili di produzione Windows/Linux.

Il rilevamento X11 e l'attesa dei report nel solo harness hanno richiesto
correzioni. Nel ritorno alla finestra originale, XSetInputFocus da solo mostrava
un'immagine nera: una sonda Windows ha verificato finestra visibile, non ridotta
a icona e responsiva; ShowWindow/SetForegroundWindow, come l'attivazione dalla
barra delle applicazioni, completa correttamente il ritorno. Le schermate finali
sono state ispezionate; l'indicatore non copre più i titoli degli slot.

Windows nativo, Linux/Wayland, controller fisici e gameplay con traduzione native
nella copia restano da verificare. La prova native copre la copia e identità della
libreria TinyCC. Non sono stati eseguiti benchmark, nuove scene DS o vecchie prove
di accettazione ROM. Evidenza locale in `target/debug-tools-sessions/`; sorgenti
pubblici e cache riutilizzabile in `target/debug-tools-build/`.

Per mettere in pausa la copia, avanzare di un numero esatto di frame e riprendere
usare [DBG-10: menu, F6/F7/F8 e CLI debug-control](debug_control.md).
`debug-session open DIR --paused` parte dal seed senza eseguire frame prima
della prima richiesta; il normale `open DIR` mantiene l'avvio diretto.


## Coordinamento con la pulizia — DBG-18

Le nuove copie dichiarano il protocollo host directory-v1 e mantengono un lease
anche durante creazione e uso della sessione. Un contesto attivo o interrotto
resta protetto; una vecchia copia senza protocollo non viene considerata sicura
per la pulizia automatica. Il normale lock della sessione resta presente.
[Gestore, ricevute e recupero dei lease](debug_artifacts.md).
