# Riprodurre un caso con un profilo — DBG-05

Un profilo portable conserva caso, runtime, ROM/BIOS, eventuale dataset,
snapshot iniziale, input, endpoint atteso e timeout. Menu e CLI avviano lo stesso
worker isolato; il worker non ha un writer per SRAM, slot o backup. La partita
normale resta in pausa e riprende solo con una scelta esplicita.

Un profilo può includere un [audit delle letture ROM](debug_rom_audit.md):
`debug-rom-audit create` configura preset, finestra, intervalli e limite. Run,
Cancel e gli esiti del replay restano comuni. L'audit ha propri indicatori di
troncatura e finestra parziale, distinti dal confronto dell'endpoint.

## Preparare il profilo

Partire da una [registrazione completa](debug_recordings.md) e dalla sua
[copia di debug](debug_sessions.md) chiusa:

```powershell
.\recomp.exe debug-replay create `
  --recording 'C:\Debug\DS-20\Data\config\recordings\recording-…' `
  --session 'C:\Debug\DS-20' --dir 'C:\Debug\replay-DS-20' `
  --case 'DS-20-EUR' --timeout-ms 120000
```

La destinazione deve essere nuova. Il comando autentica la registrazione,
controlla origine/endpoint e ROM/BIOS/dataset, poi copia i soli input necessari
in una nuova sessione. `replay.json` viene pubblicato per ultimo: una copia
incompleta non è un profilo eseguibile. La raccolta DS e la registrazione sorgente
rimangono invariate. Il nome del caso è una descrizione, non una nuova prova DS.

Il runtime scelto è l'eseguibile che esegue create; `--runtime FILE` ne sceglie
un altro. Il profilo registra separatamente hash del runtime scelto e del binario
che ha registrato il caso. È quindi possibile confrontare una nuova build con
un vecchio endpoint senza rinominare la build o promuovere un esito divergente.
Il runtime selezionato deve supportare `debug-replay worker`; vecchi eseguibili
privi del comando terminano con errore esplicito e log conservato.

Il percorso predefinito usa l'interprete. `--library FILE` seleziona una
traduzione native già disponibile e compatibile: la libreria e TinyCC vengono
copiati e autenticati. Non viene compilata la ROM né consultata una cache esterna
per sostituire la libreria scelta. Una libreria con ABI/ROM/BIOS incompatibili
può fallire o divergere; il worker separato ne conserva l'esito. Non si deduce
la compatibilità dal nome `.dll`.

Il dataset viene ereditato dalla sessione sorgente e deve corrispondere alla
registrazione. Tutti i percorsi dei payload sono relativi; spostare l'intera
cartella del profilo preserva il caso. I file personali rimangono privati,
ignorati da Git e fuori dai pacchetti.

## Dalla CLI

```powershell
.\recomp.exe debug-replay run 'C:\Debug\replay-DS-20\replay.json' --json
```

L'output è JSON Lines schema 1, tipo `ffta-debug-replay-result`: righe con
`outcome = running`, frame completati/totali e messaggio, poi un esito terminale.
Ogni avvio ha un identificatore diverso. Il risultato identifica caso, runtime,
ROM, dataset, input e libreria; all'endpoint aggiunge hash dello stato e clock.
`--json` è facoltativo: la semantica dell'output rimane la stessa.

La condizione d'arresto è `exact-endpoint`: riprodurre esattamente i frame della
registrazione e confrontare tutto lo stato serializzato e il clock attesi.
Non basta un'immagine simile. L'origine dei frame guest rimane intatta; gli
input sono relativi allo snapshot, con i tasti iniziali espliciti. Le semantiche
mouse sono quelle dei runner esistenti.

Codici d'uscita:

- **0, passed:** endpoint completo e clock identici.
- **2, diverged:** endpoint o clock diversi.
- **3, incompatible:** profilo/input/build mancanti, alterati o incompatibili.
- **4, timeout:** limite di tempo raggiunto.
- **5, interrupted:** annullamento o chiusura del proprietario del replay.
- **6, error:** errore del worker o frame incompleto.

Il timeout è compreso fra 1 e 120.000 ms; il massimo è 360.000 frame. Il
supervisore limita anche l'avvio/preflight del worker. Due esecuzioni dello
stesso profilo sono rifiutate tramite lock esclusivi; profili diversi hanno
copie e risultati separati.

Per annullare da un secondo terminale:

```powershell
.\recomp.exe debug-replay cancel 'C:\Debug\replay-DS-20\replay.json'
```

Il worker controlla la richiesta fra i frame; se non risponde entro due secondi
il supervisore lo termina. Ctrl+C richiede lo stesso arresto quando il terminale
consegna il segnale. Un risultato già completato non viene annullato a posteriori.

## Dal gioco

Aprire **Debug → Replay case** e scegliere il `replay.json`. Il caso parte in
un processo separato. Il menu mostra avanzamento e risultato; **Cancel replay**
chiede l'arresto. **Choose replay profile** cambia la selezione senza avviarla.
Il profilo rimane selezionato per la durata del processo del gioco, così il
successivo Replay case richiede solo un pulsante.

Per preselezionarlo all'avvio:

```powershell
.\recomp.exe play 'C:\Giochi\FFTA EUR.gba' --no-bios --interp `
  --debug-replay 'C:\Debug\replay-DS-20\replay.json'
```

Durante il replay Resume è sospeso; Quit chiude anche il worker. Alla fine
scegliere Resume per riprendere la partita originale. Una registrazione attiva
va terminata prima di avviare un replay. Il risultato completo viene anche
riportato nel log del gioco.

## Output locale

Dentro il profilo, `Data/config/replay/` contiene solo l'ultimo avanzamento,
risultato, richiesta di annullamento e `worker.log`, sostituiti a ogni avvio.
Ogni JSON contiene l'ID del tentativo: un vecchio risultato non vale per quello
nuovo. La pubblicazione è atomica con tentativi limitati per una breve contesa
Windows con il lettore. Non vengono accumulate copie a ogni run.

Il worker carica lo snapshot e consuma l'audio come il player, senza aprire audio
o finestra e senza misurare prestazioni. Se esecuzione, host/RTC, build o input
producono differenze, l'esito rimane visibile. Questo strumento non aggiorna
automaticamente stati DS, copertura ROM o prove di estrazione.

## Verifiche del 28 ottobre 2026

Build release Windows offline su base pubblica congelata `2fbf9f7` e quindici
file degli strumenti sovrapposti, con cache Cargo separata. Candidato finale
`target/debug-tools-replay/recomp.exe`, SHA-256
`29b135eb5f267909b4b05d0bb4e92aca5d952a3783f23acd4a1a3dbc6557047e`.
Gli eseguibili normali, la DLL sorgente e il lavoro concorrente di estrazione
rimangono invariati.

- **60 test distinti sotto Wine:** 8 del replay, 7 del preflight di registrazione
  condiviso e 45 del menu. Coprono ripetibilità dello stato completo, origine e
  tasti iniziali, progresso/codici di timeout e annullamento, divergenza, input
  errati, limiti, sessione attiva, percorsi portable e spostamento.
- **CLI reale:** il caso EUR DS-20 di 102 frame ripete stato e clock identici;
  passa anche dopo lo spostamento del profilo. Due avvii concorrenti danno un
  passaggio e un rifiuto, senza interrompere il primo. File mancanti, alterati
  e fuori ordine danno codice 3; input alternativi validi danno divergenza 2;
  timeout e annullamento danno rispettivamente 4 e 5. Sono passati 16 controlli
  di comportamento; il confronto native è descritto separatamente sotto.
- **GUI Wine/Xvfb:** Replay case raggiunge l'endpoint della CLI, Cancel replay
  interrompe il worker e chiudere il genitore lo termina. Stato/frame/clock e
  progressi del genitore sono invariati; 171 file selezionati fra raccolta,
  ROM, sorgenti di registrazione, DLL e TinyCC mantengono hash/mtime.
  Le schermate finali di risultato, annullamento e diagnosi sono state ispezionate.

Il secondo replay iniziale ha rilevato una contesa Windows sul JSON di progresso.
Il fix aggiunge un retry atomico limitato a 32 ms: il test Windows mantiene
appositamente aperto un lettore senza condivisione della cancellazione, poi lo
rilascia. Quel test e gli otto del replay passano; ripetibilità/concorrenza/GUI
sono provate sulla build corretta. La prima navigazione GUI con pressioni brevi
non ha raggiunto Cancel prima della fine del caso; il collaudo finale usa clic
reali accodati. Tentativi, log e profili sono conservati localmente.

La DLL EUR già presente (`ab7aa1808c4c14550c46ec2410f18ef47e301dbcf07a6ed021dbd015cf8f38ca`)
è stata copiata senza ricompilarla. Completa i 102 frame con lo stesso clock
`6269795841`, ma lo stato completo differisce dall'endpoint interpreter:
`76a654dc…` contro `d8c4dfc8…`. Il runner restituisce **diverged, codice 2**.
La prima aspettativa del collaudo era un passaggio: quel confronto è rimasto
fallito e non viene trasformato in una prova di parità. La causa e la parità
di quella DLL restano da valutare separatamente; nessun byte o confronto è stato
corretto per farlo passare.

Evidenza sotto `target/debug-tools-replay/`: `test-results.json`,
`publication-test-results.json`, `cli-results.json`, `cli-summary.json`,
`gui-results.json` e i tentativi conservati. Windows nativo, Linux/Wayland,
controller fisici, Ctrl+C sui terminali reali e replay con dataset non vuoto non
sono verificati da questa unità. Nessun benchmark, nuova accettazione DS,
estrazione ROM, pacchetto o modifica ad Android/Godot.

I [profili di cattura DBG-08](debug_captures.md) estendono il formato con una
condizione esplicita e `stop: "capture-trigger"`. Condividono lease, supervisione,
annullamento e selezione del runtime, ma si fermano al confine sicuro dopo il
primo match o al limite. `captured` (0) e `not-reached` (7) non sono una verifica
dell'endpoint completo né una nuova accettazione DS. I profili senza condizione
mantengono il precedente comportamento `exact-endpoint`.
