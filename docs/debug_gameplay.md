# Task, script e unità — DBG-17

Nella [copia isolata](debug_sessions.md) in pausa, **Debug > Start task / script
trace** arma il collector per un frame. Avanzare o riprodurre il passaggio,
tornare in pausa e scegliere **Save gameplay report**. Aprire
`Data/config/gameplay/<run>/index.html`; `gameplay.json` contiene gli stessi dati.
La pagina filtra famiglia, indirizzo del contesto e testo; i dettagli collegano
le risorse ai contratti autenticati e mostrano la provenienza di ogni campo.

## CLI equivalente

Aprire con il runtime che contiene lo strumento; una vecchia sessione conserva
il proprio eseguibile autenticato:

```powershell
.\recomp.exe debug-session open 'target\caso gameplay' --paused
```

Da un secondo terminale:

```powershell
.\recomp.exe debug-control gameplay-start 'target\caso gameplay' --frames 2 --events 512 --family all
.\recomp.exe debug-control step 'target\caso gameplay' 2
.\recomp.exe debug-control gameplay-report 'target\caso gameplay'
```

Filtri di raccolta: `--family all|task|script|unit` e `--context HEX` opzionale.
L'indirizzo è esatto, senza ricostruire un pool di oggetti. Per un descriptor il
contesto è la riga ROM, per uno script è il puntatore RAM, per le osservazioni
grafiche è l'actor o il task indicato dai registri. I filtri nel browser operano
sul prefisso già raccolto. `status.gameplay_trace` espone impostazioni, frame,
eventi, byte, esclusioni e stop; `gameplay_report` indica l'ultimo HTML.

Start e Save richiedono una copia in pausa, senza probe, passi pendenti o replay.
Un altro Start o un replay separato richiedono prima Save. Un ripristino dello
stato congela la traccia; origine e stato corrente possono appartenere a timeline
diverse. Chiudere senza Save perde il prefisso in memoria.

## Campi e provenienza

Start autentica la ROM europea esatta e i cataloghi esistenti di
[task](rom_data_tasks.md), [script](rom_data_scripts.md) e
[unità/OBJ](rom_data_unit_obj.md). Un'altra ROM è rifiutata. I contratti e i piani
sono identificati per hash; nessun limite della proiezione diventa un vincolo
aggiuntivo sul gioco.

- I loadsite dei descriptor mostrano callback nullable, dimensione del contesto
  e metadati grezzi, con indirizzo ROM e riga fisica. I selettori primary e
  secondary possono aliasare; primary345 ha un override separato. La callback
  del descriptor non viene presentata come callback finale già schedulata.
- Lo script principale mostra cursor signed16 a `context+0`, stream a `+12`,
  byte opcode e directory advance/callback; il dispatch conserva anche il
  registro della callback. Dopo lo store del cursore espone il risultato della
  callback: `0x110` ripete il fetch originale. Il collector non normalizza salti
  relativi, non esegue callback e non sostituisce l'interprete.
- Lo script ausiliario conserva action/slot a `+0/+1`, cursor signed16 a `+2` e
  stream a `+4`. L'opcode zero termina prima del dispatch: la sua riga ROM esiste
  ma non è una chiamata osservata. Advance e wrapping rimangono quelli guest;
  action, slot e opcode esterni alle proiezioni restano valori grezzi.
- I punti verificati del task grafico mostrano flags grezzi, sprite a `+6`,
  countdown a `+10`, cursore sequenza a `+12`, numero frame a `+14`, marker a
  `+0x20` e puntatori frame a `+0x34/+0x38`. Ingresso reset espone anche gli
  argomenti sprite/animazione/parte prima del reset. Query/risultato sprite
  conservano il puntatore actor, il registro grezzo e la derivazione low16.
  Le risorse initial-pose sono soltanto correlazioni con animazione0/parte0.

Ogni campo indica registro o indirizzo/ampiezza RAM/ROM, offset fisico e PC
osservato. Una derivazione come low16 è dichiarata. Le letture immutable usano il
backing, distinguendo mirror e memoria ROM/RAM; I/O, protocolli e contesti non
RAM sono rifiutati con valore sconosciuto. Una lettura sconosciuta è diversa da
zero/null, da una callback nullable e da un valore sentinella candidato.
`FFFF` sprite e `FFFFFFFF` marker restano grezzi: non provano un asset valido o
l'identità di un'unità.

Altri campi, semantica generale dei flag, allocatore, durata degli oggetti,
nome/ID persistente dell'unità e pool interi restano sconosciuti. Un indirizzo RAM
può essere riutilizzato. Un link di risorsa indica appartenenza a una proiezione,
non prova di raggiungibilità, consumo typed o accettazione di una nuova scena.

## Limiti e conservazione

Osservazione prima di istruzioni interprete eleggibili o dispatch native; Halt,
IRQ pendente e ritorno IRQ HLE non sono scambiati per un'istruzione eseguita al
PC. Gli interni dei blocchi native non sono osservati: punti mancanti non
significano assenza di attività. Con strumenti disarmati rimane il normale
runner; l'osservatore non modifica campi guest, formato stati o ABI.

Limiti **30 secondi host, 60 frame, 512 eventi, 1 MiB di byte evento**. Lo stop
congela il prefisso senza fermare il frame guest a metà. I frame completati e gli
input attivi sono registrati per la finestra raccolta; il suffisso è sconosciuto.
Sono consentite al più otto cartelle private, tentativi incompleti inclusi, e
2 MiB JSON. Nessuna sovrascrittura o pulizia automatica; un errore di Save conserva
la traccia congelata per riprovare. I percorsi di output devono essere directory
ordinarie. Non viene creato uno snapshot del collector.

Il report identifica runtime/build, ROM/BIOS, manifest e pin selezionati, stato
iniziale/corrente, input/AV e dataset effettivo. I valori e i percorsi sono
privati e ignorati da Git; rivederli prima di condividerli. La
[homepage](report_home.md) li trova dopo rigenerazione. La raccolta non abilita
benchmark e non certifica parità interprete/native o vecchie prove ROM/DS.

## Validazione

La validazione e i limiti effettivamente osservati sono registrati nel
[record DBG-17](changes/2026-10-29-add-bounded-gameplay-inspection.json).

Le build Windows feature/default offline e bloccate passano senza warning.
**79 test Wine** includono i controlli interessati e una prova locale opzionale:
stesso seed/interprete, observer spento/acceso per due frame A/rilascio, stato
completo e clock identici. I 70 eventi del prefisso collegano ingresso, checkpoint
e uscita dello stesso indirizzo task: countdown10→9→8, con RAM e puntatori frame
identificati, senza assegnare un'unità persistente. In questa finestra non sono
stati osservati script; non serve rieseguire un'altra scena per chiudere la
sequenza richiesta.

La GUI default supera 29 invocazioni CLI inclusa navigazione: report CLI70eventi,
menu35, filtro vuoto0con35esclusi e contesto selezionato3. La raccolta codice
simultanea conserva il suo prefisso8 e l'endpoint completo atteso del frontend.
Start/Save lasciano lo stato corrente intatto; capacità piena preserva i report.
I sei input/runtime realmente selezionati mantengono hash/mtime e la copia
chiude con server inattivo. Chrome verifica layout1400/620, filtri, dettagli e
link ai contratti; la homepage55indici include cinque gameplay e il report codice
di coesistenza. Screenshot ispezionati. Nessuna prova di Windows/Linux nativi,
controller fisici o cattura GUI native; nessuna vecchia accettazione ROM/DS
riaperta. Evidenze private in `target/debug-tools-gameplay/`.
