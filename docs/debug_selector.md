# Selettore integrato dei contenuti debug

DBG2-03 aggiunge **Debug > Content selector**, una pagina riutilizzabile sopra
il [catalogo comune](debug_content.md). La scelta confermata è un riferimento
host: nessun replay, script, caricamento di stato o cheat parte selezionandola.
La migrazione DBG2-04 è in corso; le prove concluse rimangono distinte dalla nuova integrazione.

Aprire la pagina mostra l'indice già disponibile oppure una lista vuota.
**Refresh** aggiorna esplicitamente le radici dichiarate in un worker limitato;
la pagina resta navigabile. **Browse external file…** aggiunge soltanto il file
scelto, senza esaminare i suoi vicini. **Add folder…** aggiunge una cartella
esplicita alle radici della scansione. Solo questi due pulsanti aprono dialoghi
OS; annullarli conserva radici e scelta corrente. Al ritorno il frontend
consuma gli eventi della chiusura e del ripristino del focus per almeno 400 ms e
due poll neutri del controller, evitando che Escape chiuda anche il menu o che
una conferma mantenuta avvii altre azioni. `host_modal_input` nello status indica
questa breve attesa. La CLI equivalente per
indicizzare un singolo file è `debug-content scan --add-file FILE --json`.

La lista offre ricerca per nome/ID/tipo/percorso/descrizione, categoria, filtri
di disponibilità e compatibilità, ordinamento per nome/tipo/recente e raccolte
All content/Favorites/Recent. Due omonimi restano distinti tramite ID e percorso
nei dettagli. L'evidenziazione cambia solo la riga osservata; **Select** conferma
la scelta dopo il ricontrollo del catalogo. File cambiati, mancanti, non ordinari,
di tipo sconosciuto o incompatibili vengono rifiutati. Un cambio di generazione
dell'indice impedisce di usare una riga precedente per un file diverso.

**Favorite** conserva al massimo 512 ID; Recent conserva le ultime 32 conferme.
Le scelte rimangono in memoria nella finestra corrente: persistenza e riferimenti
dopo riapertura sono DBG2-07. Il rientro dal selettore conserva il chiamante e
la scelta già confermata. Una selezione ancora in lavorazione viene scartata se
si annulla la pagina; un risultato tardivo non la riapre e non applica una scelta.

**Details / preview** rende scorrevoli ID completi, percorso, identità, stato,
motivo e messaggi. Per snapshot con immagine incorporata usa il verificatore
readonly già disponibile, senza caricare una Machine; snapshot raw non promettono
un'immagine. Il ricontrollo dell'hash precede la visualizzazione. La raccolta di
metadati e l'anteprima non sostituiscono il preflight dell'azione effettiva. Il
messaggio finale distingue anteprima autenticata, immagine assente ed errore;
una risposta relativa a una riga ormai diversa non sostituisce l'anteprima corrente.

Tastiera: frecce per muovere/cambiare filtri, Tab per cambiare area, Invio per
entrare nella ricerca o portare la lista sul pulsante Select. Invio su Select
conferma; Escape annulla l'editor/dettaglio o ritorna al chiamante. Ctrl+F apre
la ricerca. Nella ricerca i caratteri, Backspace, Tab, Invio ed Escape vengono
consumati dal menu prima dei binding guest e delle shortcut debugger/globali.
L'editor accetta 128 caratteri Unicode e preedit/commit IME; la copertura effettiva
di layout/composizione e glifi dipende dalla piattaforma e dai font disponibili.

Mouse: hover evidenzia, click sui pulsanti esegue la singola azione host; click
sulla lista non conferma la scelta. La rotella scorre la lista o i dettagli.
Controller: direzioni, South/Confirm e East/Cancel percorrono gli stessi controlli;
la ricerca offre una tastiera a schermo con pagine, spazio, cancellazione e Done.
Una pressione mantenuta non deve produrre conferme multiple. Mentre l'editor ha
focus, le shortcut non raggiungono il guest.

La pagina usa una sola operazione IO alla volta, con generazione dell'indice e
identità della vista. Mantiene i cap del catalogo; i dettagli avvolgono il testo
con gli stessi font, kerning e advance del renderer e limitano la finestra a
16.384 caratteri. `debug-control status` include lo stato host `content_picker`
per leggere conteggio, generazione, operazione pendente e scelta confermata.
Non è ancora un nuovo comando di esecuzione della console DBG2-13/15.

Le verifiche effettive, le piattaforme e gli eventuali criteri ancora aperti
sono registrati nel [TODO DBG2](development/DEBUG_TOOLS_V2_TODO.md) e nel
[passaggio di consegne](development/TASK_HANDOFF.md). La prova GUI usa una fixture
pubblica sintetica ferma al frame iniziale, distinta dalle scene DS e dai dati
personali. I tre vecchi fallback FileDialog sono rimossi da DBG2-04.

DBG2-03 è verificato con build Windows GNU e Wine: tutti i 55 casi menu,
10 catalogo, 4 bridge, 3 contratto, 2 collector UI e 2 ritorno modale nei
rispettivi scope; 3 test Python inventario e 10 packaging. La GUI esercita
ricerca, omonimi, conferma mantenuta singola, preferiti/recenti, XInput simulato,
annullamento di entrambi i dialoghi, importazione singola e anteprima a tre
dimensioni. La prima prova Cancel ha esposto e conservato una regressione che
riprendeva la copia sintetica; dopo la correzione la nuova prova conserva stato
completo/frame0/clock0. Windows nativo, Linux runtime, controller fisici e IME
nativo restano non verificati. [Record completo](changes/2026-10-29-add-integrated-debug-content-selector.json).

DBG2-04 aggiunge **Debug > Tool inputs / current selections**:
profili replay/audit/capture/compare, catalogo salvataggi, sessione, registrazione,
snapshot, sequenza input, dataset, runtime A/B, libreria nativa, label e piano
noto. Le liste sono vincolate al ruolo e le scelte conservano hash, ID e contesto
ROM. Un file non hashato nel budget viene rifiutato con richiesta di restringere
la radice e aggiornare. Un riferimento alterato resta visibile e viene rifiutato
prima dell'uso; non si ripiega su un altro file o sui dialoghi OS.

Choose replay profile e Choose catalog aprono la pagina interna. Replay, Audit,
Capture, Compare e Create issue senza profilo aprono la stessa pagina con il
requisito mancante: scegliere non riprende l'azione automaticamente. Ritornare
al menu e avviare esplicitamente. I riferimenti passati dalla CLI e il catalogo
predefinito vengono autenticati prima dell'uso. Le label scelte alimentano la
traccia codice; gli input per preparare nuovi profili sono separati dai payload
immutabili dei profili già autenticati (editor DBG2-05).

La scelta tipizzata controlla formato e identità usando i servizi readonly
esistenti per profili, cataloghi, sessioni e registrazioni; snapshot e sequenze
sono controllati senza caricarli nella macchina corrente. Le label sono parse
per la ROM; i manifest controllano schema/ROM. Runtime e librerie restano
riferimenti hashati: il worker deve ancora verificare formato, feature e ABI,
senza caricare codice alla semplice selezione.

La verifica Wine esercita i dodici ruoli, i sette ingressi senza input, gli
avvii espliciti, dieci rifiuti di profili alterati/mancanti, ROM diversa e sei
rifiuti Choose/Open del catalogo. La traccia conferma un dialogo per pulsante
esplicito e Cancel conserva tutte le scelte. Il processo principale delle prove
accettate resta al frame zero; worker sintetici e un precedente errore di
navigazione dell'harness sono documentati separatamente nel
[record DBG2-04](changes/2026-10-29-migrate-debug-tool-input-selection.json).

DBG2-05 aggiunge **Debug > Parameters / prepare profiles** aggiunge bozze
per replay, audit, cattura, confronto, passi, ispezione, audio, codice, gameplay,
eventi recenti, grafica e issue. Le righe input usano il selettore tipizzato;
frecce cambiano le scelte, Confirm apre l'editor testuale con tastiera a schermo.
Escape annulla l'editor o ritorna dal pannello; Tab cambia campo/azione.

**Verify draft** usa i parser/preflight condivisi senza eseguire. **Save new
preset** conserva soltanto valori scalari nella configurazione portable:
`debug-presets/<tool>/`, massimo otto file da 16 KiB per strumento e 8 KiB di
valori. **Load last preset** ripristina una bozza, senza cambiare riferimenti o
avviare strumenti. Input mancanti/alterati e combinazioni non supportate hanno
un rifiuto leggibile. Le risposte tardive non sovrascrivono una bozza riaperta.

**Prepare NEW profile** crea una nuova copia in `debug-profiles/`, fino a otto,
con parametri e input autenticati; non sovrascrive un profilo esistente e non
avvia replay. **Run prepared profile** esegue quel profilo separatamente fissato:
modificare la bozza richiede una nuova preparazione. Negli strumenti locali,
**Start / apply** passa i parametri all'handler esistente; armare un observer non
avanza automaticamente il gioco. Passi, ripresa e report restano espliciti.
Le opzioni comprendono timeout/frame/input/range/filtri/stream/trigger/modalità
confronto/sintomo; le label selezionate restano riferimenti con ricontrollo.

La preparazione può leggere i pin immutabili della stessa sessione già aperta,
riconoscendone root/PID/lease runtime, senza concedere un altro writer. Le altre
sessioni attive continuano a essere rifiutate. La validazione del profilo non
promette ABI/feature di un eseguibile arbitrario; il worker mantiene il preflight
finale. Le verifiche effettive sono nel
[record DBG2-05](changes/2026-10-29-add-debug-parameter-workbench.json).

DBG2-05 è verificato con 121 casi Rust distinti nei rispettivi scope Windows/
Wine e con la catena GUI registrazione → un passo esplicito → preparazione
replay/audit/cattura/confronto → avvio separato → report. I quattro profili
creati con gli stessi parametri CLI coincidono in JSON ed esito/stato/clock.
La pagina di punteggiatura permette virgole e separatori da controller;
parametri incompleti vengono rifiutati prima dell'esecuzione. Le prime prove
hanno corretto un canvas non pubblicato e il badge sovrapposto; immagini e log
negativi restano distinti dalle verifiche finali a tre dimensioni.
