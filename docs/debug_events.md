# Eventi comuni — DBG2-08

Le osservazioni host già disponibili usano un solo punto di pubblicazione nel
frontend. Quando la traccia UI o il registro recente sono attivi, un evento viene
creato una volta e consegnato ai due collector. Nessun observer invoca gli handler
del menu o del controllo locale. Con entrambi spenti non si costruiscono payload
input/focus né si serializzano eventi. Il pannello live resta lavoro DBG2-11.

Ogni riga mantiene `stage`, `data`, `frame` e `clock` e aggiunge:

- `event_schema: 1`, `event_id` e `event_sequence`: identità e ordine comuni nella
  stessa esecuzione. La sequenza non riparte quando si avvia un altro collector.
- `host_elapsed_ns`: tempo monotono dalla creazione del clock host; non è tempo
  guest né una misura della latenza hardware.
- `session.run_id` e `session.manifest_sha256`: esecuzione del canale locale e
  manifest autenticato della copia; fuori da quel canale il manifest è nullo.
- `category`, `severity`, `origin.producer`, `origin.source` e `observation`:
  famiglia, livello, punto di osservazione, contesto e granularità dichiarata.
- `request_id` e `job_id`: nulli senza una richiesta osservata. Navigazione e
  azione menu condividono un contesto; i comandi locali mantengono l'ID reale
  della mailbox. Passi e ricerche riportano quell'ID anche come job.

`sequence` e `host_ms` restano alias **locali al collector**, per compatibilità
con i report schema 1. Per unire due report usare `event_id` e ordinare per
`event_sequence`. Tempo e sequenza comuni sono assegnati nella pubblicazione
host; frame/clock guest possono ripetersi o tornare indietro. Le lacune possono
includere periodi non raccolti, eventi specifici dell'altro collector e scarti.
Non si deduce una cronologia delle istruzioni guest da questo ordine.

Il canale comune osserva i token Winit già strumentati, cambi di poll XInput e
rimappati, consumi input già presenti, binding, dispatch menu con focus prima e
dopo, azioni e comandi condivisi, confini di frame e ripristino ed errori già
instradati. Poll identici sono accorpati prima della consegna ai collector.
Le ripetizioni keyboard conservano il proprio flag. Gli observer recenti di
letture, dispatch e riconoscimento guest mantengono la loro granularità:
`rom-read-batch` e `frame` sono aggregati per frame, non una sequenza interna al
frame. Le chiavi di lettura sono ordinate e limitate come prima.

La navigazione tastiera/pad accettata pubblica `host-input-request`, poi
`host-menu-dispatch` e, se l'handler restituisce un'azione, `host-action`.
`host-action-result` significa solo ritorno dall'handler esistente. Non certifica
l'accettazione di una procedura che può avere già mostrato un rifiuto nel menu.
`control-result` distingue `accepted` e `refused`; l'accettazione non significa
fine di un job asincrono. Il controllo locale pubblica `control-complete` con lo
stesso request/job ID quando il budget di passi/ricerca termina, anche per
annullamento o timeout. La query `status` resta silenziosa.

`status.event_coverage` e `event_coverage` nei due report espongono la stessa
mappa versionata dei produttori supportati, condizionali, parziali e mancanti.
`status.event_clock` espone identità e prossimo numero. Restano mancanti gli
hook raw Gilrs/DirectInput, i percorsi touch/pen assenti dal frontend, tutti i
cicli di vita strumenti/job/sessione (DBG2-10), gli interni dei
blocchi native, la cronologia CPU completa e callback audio/cheat futuri.
Un produttore non registrato è marcato `unknown/unregistered`.

I limiti dei collector restano separati: [UI](debug_ui.md) conserva un prefisso
esplicito; [recent](debug_recent.md) mantiene una finestra mobile con contatori di
espulsione e oversize. Il payload non viene duplicato nell'envelope; i metadati
rientrano nei cap di byte esistenti. Un collector pieno o congelato non blocca
l'altro. Avviare la raccolta durante un comando può conservarne l'esito senza
la richiesta precedente; salvare/congelare può escluderne l'esito successivo.
L'export e il ridisegno non producono eventi ricorsivi né eseguono comandi.

## Verifica del 30 ottobre 2026

Build Windows GNU locked/offline senza warning su606file congelati/46overlay
host;11test Wine distinti passano, inclusi identità/fanout, contesti annidati,
poll, limiti UI/recent, un milione di eventi e controllo locale. La nuova GUI
usa lo stesso seed pubblico accettato con un runtime distinto:35eventi UI e
47recent, tutti35gli eventi comuni identici salvo gli alias locali. Tre
dispatch di navigazione producono una sola apertura del selettore; pausa
accettata e riavvio UI rifiutato mantengono gli ID reali e gli esiti nei due
report. Stato completo/frame0/clock0 restano esatti. GUI e Xvfb chiusi; pin e
runtime canonici conservati. Windows nativo, Linux e hardware fisico non
verificati da questa unità; nessuna accettazione DS o parità guest.

## Cambi del menu — DBG2-09

Il menu espone uno stato semantico readonly distinto dalla geometria: pagina,
ID e label della voce, valore attuale, dropdown, slot selezionati/disponibilità,
binding in cattura, editor, filtri/conferma/preferito del contenuto, campi dello
strumento e feedback realmente mostrato. Un ID del contenuto o catalogo resta
distinto dalla label anche per elementi omonimi. Gli ID dei controlli ordinari
usano pagina/variante del controllo; i campi tool usano strumento/chiave.

`host-menu-baseline` dichiara lo stato iniziale di una nuova raccolta;
`host-menu-change` registra soltanto proprietà cambiate o rimosse, con valore
precedente/corrente, voce precedente/corrente e causa. Ogni proprietà produce
una riga separata, così un aggiornamento di più campi resta entro il cap di una
riga. Massimo96proprietà: ulteriori campi sono dichiarati come omessi. Valori
lunghi conservano256caratteri, conteggio e hash completo, per distinguere testi
con prefisso uguale. Nessuna immagine di slot o dump guest entra nello snapshot.

Le osservazioni avvengono dopo gli input pubblici e gli handler esistenti; i
ritorni anticipati di editor e cancellazione sono inclusi. Le mutazioni interne
non presentate fra due confini di input non sono una cronologia dell'implementazione.
Un hover identico non emette cambi ripetuti. Il renderer e le sue celle di scroll
non vengono chiamati né regolati per leggere lo stato semantico.

`mouse-input` distingue pulsanti/rotella raw Winit dalla consegna al menu;
`host-pointer-dispatch` e `host-input-consumed` dichiarano instradamento e
consumo. I movimenti mouse producono eventi di selezione soltanto quando la
selezione cambia. Il contesto della richiesta segue anche la coda mouse fino
all'azione. `host-text-input` osserva preedit/commit IME già ricevuti da Winit;
i cambi della bozza rimangono separati dai campi confermati. `binding-capture-result`
riporta token disponibili/configurazione ed esito della cattura. I poll rimappati
indicano backend e dispositivo selezionati; sorgenti simultanee tastiera/pad
restano combinate. I token raw Gilrs/DirectInput non sono ancora strumentati.

`host-action-refused` usa gli errori sincroni realmente restituiti; gli altri
handler conservano il proprio feedback nel registro delle proprietà e un esito
`handler-returned`. La fine dei job asincroni resta separata. Nessun osservatore
chiama di nuovo un handler per ricostruirne l'esito. I risultati DBG2-09 distinguono Wine/XInput simulato da hardware fisico e
composizione IME; la sola compatibilità del codice non verifica questi ultimi.

### Verifica DBG2-09

Windows GNU locked/offline senza warning,609sorgenti congelati/50overlay host.
Undici casi Rust distinti passano nelle esecuzioni mirate; il tentativo con filtro
XInput errato che non selezionava test è escluso dal conteggio. Sei catture GUI
sullo stesso seed pubblico contengono580eventi UI e739recent: tutti580gli eventi
comuni hanno identità e dati uguali, senza espulsioni o oversize. Conferma tenuta,
autorepeat, dropdown, hover, editor, ispezione/rifiuto e binding sono verificati;
stato completo/frame0/clock0 restano esatti. I prefissi completati prima di errori
del test sono conservati; si riprendono soltanto i criteri mancanti. La cattura
tastiera mantiene il vocabolario esistente: G si assegna, F4 resta in attesa.

GUI e display isolato sono chiusi; pin dei seed e runtime canonici conservati.
La guida è nella lista pubblica e nei COPY/ignore Docker espliciti;10test Python
su archivi sintetici passano, senza generare una release o ricompilare Docker.
Le prove non attestano Windows/Linux nativi, hardware fisico o IME reale.

## Runtime e strumenti — DBG2-10 in verifica

`runtime-event` aggiunge famiglia, fase, esito e dettagli limitati sul medesimo
clock. `event_coverage.runtime_families` descrive origine, frequenza, requisiti e
scope di18famiglie; cheat resta riservata finché esiste un handler verificato.
La disponibilità degli hook non equivale a validazione hardware o parità guest.

Il registro mantiene al massimo16job attivi, con request/job ID iniziali anche
quando cambia la vista. Annullamenti e altri interventi mantengono il contesto
iniziale e riportano `cause_request_id` separatamente. Passi e ricerche riusano
l'ID reale del canale locale; il medesimo contesto circonda i tick esistenti.
Il registro non avvia né ripete un'operazione. Un rimpiazzo accettato dichiara
`superseded`/`rearmed`, senza trasformarlo in completamento.

I dettagli sono al massimo4096byte; valori maggiori conservano prefisso/hash e
flag di troncamento. L'esito resta fuori dai dettagli troncati. I progressi sono
emessi solo quando cambiano e al massimo ogni250ms. I campioni UI/recent
escludono i propri conteggi e clock, per non generare progressi da se stessi.
Codice/gameplay/audio leggono contatori degli osservatori esistenti dal frontend:
nessun nuovo I/O o lettura del clock nei callback audio, né serializzazioni
continue dell'intera macchina. L'hash ROM del baseline avviene soltanto all'avvio
esplicito della raccolta.

Salvataggio, caricamento, Undo, reset e backup dichiarano richiesta ed esito nei
punti degli handler reali. Il ripristino viene osservato prima del freeze per
discontinuità. Registrazioni e worker conservano il proprio contesto iniziale;
la fine dell'handler non sostituisce la fine del job. Il risultato host mantiene
i veri esiti completed/cancelled/timeout/refused, senza interpretarli dal testo
del menu o rileggere file per produrre l'evento. I risultati di una bozza vecchia
restano identificati e non sostituiscono quella corrente.

Il worker replay conserva fino a8fasi di preparazione: richiesta/attach del
dataset e richiesta/load dell'input native/cache, oppure non selezionato/rifiuto.
Sono ingressi offline già preparati; non attestano nuova estrazione o
compilazione. Dettagli per fase limitati a512byte, pubblicazione preparatoria
best effort e non bloccante, record mantenuti nei progressi/risultati e fallback
del supervisore. I lettori di questi report hanno un cap32KiB. Sequenza e tempo
host del worker restano distinti dalla pubblicazione nel frontend; i frame del
worker non sono frame eseguiti dalla macchina principale.

Restano fuori la storia CLI prima dell'avvio del frontend, la generazione
arbitraria di dataset/cache e gli interni native/CPU non strumentati. Il registro
recente non salvato continua a impedire un replay separato; la traccia UI può
osservarlo. La chiusura emette un'intenzione nel buffer già attivo, senza export
implicito: salvare il prefisso desiderato esplicitamente prima di uscire.
Il gameplay inspector continua a richiedere la ROM europea autenticata; una ROM
sintetica rifiutata non viene reinterpretata come modello guest verificato.

Sono in corso le prove DBG2-10: build Windows senza warning e9casi Wine mirati
passano; GUI, conservazione finale e chiusura della voce restano da completare.
