# Pausa e avanzamento controllato — DBG-10

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

Un solo client può attendere il risultato. `step` risponde al completamento,
non all'inserimento in coda. Il timeout client di 120 secondi **non annulla**
un comando: controllare la finestra prima di ritentare. Non esiste un retry
automatico. F6/Escape possono interrompere un budget in corso; Load/Undo/Restart
cambiano esplicitamente la timeline e interrompono il budget. La chiusura segnala
l'interruzione del comando in attesa. Durante un replay separato si può leggere
lo stato, ma pausa/passi/ripresa sono rifiutati.

## Audio, pacing e file locali

La pausa invalida le code e la storia DSP frontend e silenzia l'uscita host.
Ogni passo drena i tap guest senza accodarli al dispositivo. La ripresa parte
con code vuote, nuova priming audio e scadenza host corrente; il tempo trascorso
in pausa non crea frame da recuperare. Lo stato dei generatori audio guest resta
parte della normale emulazione. L'ascolto e il ritardo del dispositivo fisico
sono distinti dalle verifiche delle code.

Il canale locale vive in `Data/config/debug-control/` della copia portable:
`ready.json`, `request.json`, `result.json` e lock client. Pubblicazione atomica,
identità per processo e richiesta e lock OS separano avvii e client; vecchi
risultati non valgono come conferma di un nuovo comando. Si conserva soltanto
l'ultima risposta, senza timeline, nuovi slot o raccolte di snapshot. Tutti i file
restano nella sessione privata, ignorati da Git. Il canale viene chiuso con la
finestra e viene ricreato con nuova identità alla riapertura.

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
e callback, senza prova d'ascolto o latenza del dispositivo fisico. Il timeout
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

