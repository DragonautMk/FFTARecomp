# Eventi comuni — DBG2-08

Le osservazioni host già disponibili usano un solo punto di pubblicazione nel
frontend. Quando la traccia UI, il registro recente o la console sono attivi, un evento
host viene creato una volta e consegnato ai consumer attivi. Nessun observer
invoca gli handler del menu o del controllo locale. Con tutti spenti non si
costruiscono payload input/focus né si serializzano eventi.

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
dati dei cicli di vita non strumentati, gli interni dei
blocchi native, la cronologia CPU completa, i callback audio grezzi e il ciclo
di vita dei futuri cheat mantenuti.
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

## Finestra eventi live — DBG2-11

Aprire «Console eventi» nella sezione Debug delle catture, oppure usare F9
quando Debug è abilitato. `key.event_console` in `input.cfg` accetta F2, F3,
F4, F5, F9, F10 o F12; il valore predefinito è F9. La console è una finestra
host separata: aprirla non mette in pausa il gioco. Nasconderla interrompe la
raccolta e conserva la finestra già raccolta; riaprirla riprende la raccolta.
Gli eventi accaduti mentre era nascosta non sono ricostruiti.

La finestra ha un proprio buffer mobile di massimo1024eventi, 256KiB di JSON
serializzato e 8KiB per evento. Mostra intervallo corrente, conteggi espulsi e
oversize e stato del collector anche mentre la vista è in pausa. P congela o
riprende soltanto la vista; frecce e PagSu/PagGiù scorrono, Fine torna al
seguito automatico. C e L ciclano categoria e livello; `/` modifica la ricerca,
Invio termina la modifica. I filtri non cambiano il buffer originale. Ctrl+C
copia la vista JSON e Ctrl+S la salva esplicitamente in `Data/config/event-console`
della copia, massimo32file. Escape nasconde la finestra. L'export conserva gli
ID canonici, i filtri, lo stato del collector e la dichiarazione di copertura.

A alterna l'accorpamento dei poll identici XInput/rimappati: inizialmente è
attivo; disattivarlo pubblica ogni poll osservato verso i consumer attivi e può
raggiungere i loro cap più rapidamente. Gli aggregati di letture guest restano
per frame: non esiste qui una cronologia interna delle istruzioni. I produttori
esclusivi del registro recente compaiono solo quando quel collector è attivo.
«Tutte» include le categorie effettivamente pubblicate, comprese `unknown`;
non abilita hook non strumentati. Lo snapshot della vista in pausa e il buffer
live sono distinti e ciascuno rimane limitato.

Build Windows GNU e prove Wine sintetiche verificano menu, shortcut/focus,
filtri, pausa, scorrimento, esportazione e copia reale negli appunti, con stato
completo/frame/clock invariati. Le prove usano Xvfb e input simulato; Windows
nativo, Linux nativo e periferiche fisiche non sono verificati. I limiti e il
costo in uso prolungato rimangono il lavoro successivo DBG2-12.

## Limiti e costo — DBG2-12

Il collector accetta al massimo32eventi per ciclo del frontend; quelli oltre il
budget sono contati separatamente dalle espulsioni e dagli eventi troppo grandi.
Il buffer live limita a1024righe, 256KiB di JSON e 1MiB complessivo di testo
JSON più indice di ricerca; ogni evento ha un tetto di8KiB. La vista congelata
condivide le righe immutabili: può conservare fino a un'ulteriore finestra
limitata mentre quella live ruota. Questi sono limiti sui dati e sulle righe,
non una misura esatta degli overhead dell'allocatore o della finestra grafica.
Il disegno, massimo quattro volte al secondo, decodifica solo le righe visibili;
filtri ed export non mutano il log. La pausa della vista, la raccolta e la pausa
del gioco restano distinti.

Una prova sintetica Wine di128.000 osservazioni in2000cicli ha contato64.000
scarti per budget e rispettato i limiti, mantenendo invariata la vista congelata;
il tempo osservato è426ms per il test, non un benchmark del gioco. Due sessioni
isolate con lo stesso seed e dieci frame controllati hanno prodotto stato completo,
registri e clock identici con console spenta e accesa. La durata misurata include
il processo client e non stabilisce un costo prestazionale generale.

Se uno snapshot ha disattivato i tap audio separati, il frontend usa la miscela
fedele disponibile per pacing e output senza modificare lo snapshot o la scelta
Enhanced salvata. Una breve osservazione Wine con console aperta e vista congelata
ha mostrato code audio limitate e nessun messaggio di scarto, come il confronto
con console spenta. Restano non verificati qualità percepita, hardware fisico e
comportamento prolungato su Windows o Linux nativi.

## Runtime e strumenti — DBG2-10

`runtime-event` aggiunge famiglia, fase, esito e dettagli limitati sul medesimo
clock. `event_coverage.runtime_families` descrive origine, frequenza, requisiti e
scope di18famiglie. La famiglia cheat è parziale: l'handler manuale pubblica
preparazione, applicazione singola e rifiuto; non copre toggle mantenuti,
disattivazione o cronologia dell'hook originale.
La disponibilità degli hook non equivale a validazione hardware o parità guest.

`start_observed` indica che un consumer era attivo alla sorgente, non garantisce
la conservazione del record oltre i limiti del consumer. Gli errori di
pubblicazione del canale locale mantengono il contesto della sola risposta
validata in attesa; query di stato riuscite restano silenziose.

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

Le prove sulla build Windows GNU sotto Wine includono dieci casi mirati e le
interazioni di sessione, passi, stato, registrazione, catture, ricetta,
annullamento e replay. Il blocco temporaneo della risposta di controllo ha
prodotto errori associati all'ID effettivo della richiesta; il retry è riuscito.
Il manifest della registrazione punta al file `recording.json` esistente,
separato dalla sua directory. Il replay ha osservato le fasi dataset/cache non
selezionate; non prova il caricamento di input privati. Un frame controllato
ha conservato lo stato completo atteso. Dopo la revisione Clan Atlas una nuova
build Windows e dieci test Wine sono passati; avvio/arresto registrazione dal
menu e passo controllato mantengono identità e stato attesi. Windows nativo,
controller fisici e IME non sono verificati.
