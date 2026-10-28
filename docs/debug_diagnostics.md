# Diagnosi della sessione — DBG-01

La diagnosi raccoglie identità e percorsi necessari per riprodurre un problema.
Il menu e il comando `diagnose` usano lo stesso collector e lo stesso JSON
versionato (`schema = 1`, `kind = ffta-session-diagnosis`). La raccolta è
esplicita: non aggiunge log a ogni frame.

## Dal gioco

Aprire **Debug → Copy diagnosis**. Non serve abilitare Extended saves.
Il gioco resta in pausa: il report viene copiato negli appunti e scritto in
`diagnostics/session.json` sotto la directory di configurazione. In modalità
portable il file è `Data/config/diagnostics/session.json` accanto all'eseguibile.
Ogni richiesta aggiorna lo stesso file, senza creare una serie di cartelle.

Il report contiene il percorso completo della propria cartella. Se gli appunti
non sono disponibili, il menu lo segnala e conserva il file; il log di avvio
indica il percorso. Il JSON può contenere percorsi personali: esaminarlo prima
di condividerlo.

La diagnosi della sessione usa gli hash dei byte ROM/BIOS effettivamente caricati,
la modalità effettiva, frame e clock guest e l'identità del dataset collegato.
Segnala se i file originali o il dataset su disco sono stati sostituiti.
Lo slot snapshot è la **selezione del menu**, non una promessa che lo stato
attuale coincida ancora con il file. Una registrazione attiva viene identificata
senza dichiararla già pronta per il replay.

## Prima dell'avvio, dalla CLI

Esempio PowerShell, dal percorso dell'eseguibile scelto:

```powershell
.\recomp.exe diagnose 'C:\Giochi\FFTA EUR.gba' --no-bios --interp --json
```

Per un caso registrato con dataset esplicito:

```powershell
.\recomp.exe diagnose 'C:\Giochi\FFTA EUR.gba' --no-bios --interp `
  --sav 'C:\Debug\partita.sav' --state 'C:\Debug\scena.ffstate' `
  --input 'C:\Debug\scena.input' --dataset 'C:\Debug\dataset' `
  --out 'C:\Debug\diagnosis.json' --json
```

Su Linux il comando è `./recomp diagnose ...`. `recomp help diagnose` elenca
le opzioni. `--bios FILE` seleziona un BIOS reale; senza le due opzioni BIOS si
usa la stessa ricerca delle immagini installate del runtime. `--rom-data`
controlla la generazione gestita selezionata da `current`; `--dataset DIR`
controlla una directory esplicita. Il controllo non prepara né ripara dati:
se mancano, il report suggerisce `recomp rom-data prepare <rom>`.

`--slot N` controlla uno dei 99 snapshot accanto al salvataggio selezionato;
è alternativo a `--state FILE`. `--sav` e `FFTA_SAVE_PATH` evitano ambiguità nella
scelta del salvataggio. La risoluzione usa le regole normali, ma non crea
cartelle né apre un selettore: se ci sono più candidati occorre sceglierne uno.
Un percorso `--sav` esplicito deve esistere; senza selezione esplicita una nuova
partita è valida e il report indica l'assenza del file.

Senza `--out` la CLI scrive solo su stdout. Senza `--json` mostra un riepilogo
breve con cause e rimedi. Gli input non validi producono `ok = false`, problemi
strutturati e codice di uscita **1**; una diagnosi valida restituisce **0**.
Gli errori di sintassi o di scrittura del report sono riportati su stderr.

## Contenuto e limiti

- Build: versione, revisione e working tree modificata al momento della build,
  target/profilo, feature disponibili, percorso e SHA-256 del binario effettivo.
  Una revisione sconosciuta resta esplicita nelle build senza metadati Git.
- File: hash e dimensioni di ROM, BIOS, SRAM, snapshot, input e configurazioni;
  profilo EUR/USA solo per immagini riconosciute esattamente.
- Dataset: directory e hash del manifest, revisione estrattore/decoder, risorse
  e identità autenticata contro la ROM tramite il validatore del runtime.
- Snapshot: decodifica completa nella CLI con il validatore normale, su una
  macchina temporanea; nessuna istruzione guest viene eseguita.
- Input: parsing del formato e dei valori; non dimostra raggiungibilità della
  scena, compatibilità temporale con lo snapshot o uguaglianza dell'endpoint.
- Modalità native: prima dell'avvio è una richiesta, non una libreria già
  caricata o una compilazione riuscita. Il report lo dichiara; dal menu indica
  invece la modalità realmente attiva.
- `--require-rom-audit` fallisce quando manca la feature compilata
  `rom-data-read-audit`. Non avvia una cattura: l'interfaccia audit è DBG-06.

Il report è limitato a 512 KiB e pubblicato atomicamente; viene sostituito solo
un precedente report dello stesso tipo/schema, mai un file arbitrario o un link.
Nessun contenuto ROM, BIOS, RAM, SRAM, screenshot o asset è incorporato nel JSON.
Il controllo non avvia traduzioni native, replay o benchmark e non salva la
partita. Le [sessioni isolate](debug_sessions.md) aggiungono copie portable del
caso scelto; i profili di replay restano una voce successiva della
[checklist](development/DEBUG_TOOLS_TODO.md).

## Verifica del 27 ottobre 2026

Build Windows con dipendenze bloccate e senza rete; 47 test sotto Wine passati
(5 diagnosi, 40 menu, 2 scelta salvataggi). Otto casi CLI reali EUR verificano
identità del dataset esplicito/gestito, vecchia generazione incompatibile, input
malformato, BIOS dello snapshot diverso, feature audit assente, protezione di un
salvataggio usato come destinazione del report e slot fuori intervallo.

Una sessione Wine/Xvfb con configurazione portable e salvataggio copiato verifica
la voce del menu, JSON negli appunti identico al file, frame/clock invariati tra
due raccolte, aggiornamento di un solo report e chiusura normale. Esaminato anche
lo screenshot del menu. I 139 file selezionati come input protetti, inclusi ROM,
snapshot, registrazione, salvataggio e dataset, conservano hash e date originali;
anche i due eseguibili di produzione Windows/Linux sono preservati.

La prima apertura grafica con un prefix già avviato senza display si è fermata
prima della creazione della finestra; il controllo è passato con un prefix
Wine grafico dedicato. Windows nativo, Linux, Wayland, controller fisici e una
sessione con traduzione nativa attiva non sono stati verificati in questa unità.
Il report identifica la SRAM senza validarne i checksum interni del gioco.
Le prove non includono benchmark, nuove scene DS o replay delle vecchie
accettazioni ROM. Evidenza locale in `target/debug-tools-diagnosis/`.
