# Contratto delle operazioni di debug

DBG2-01 aggiunge un inventario consultabile delle operazioni esistenti. Il
contratto versione 1 contiene 110 operazioni e 376 parametri (65/242 nella
prima unità DBG2-01, più catalogo, selettore e ruoli di DBG2-02/03/04/05/06): diagnosi, copie
isolate, catalogo, registrazioni, profili replay/audit/capture/compare, controllo
locale, ispettori, tracce, report, conservazione, campagne e checkpoint.

```sh
recomp debug-operations list
recomp debug-operations show control.step
recomp debug-operations show replay.create --json
recomp debug-operations list --json
```

Queste query leggono solo i descrittori incorporati nella build. Non aprono
sessioni, scandiscono cartelle, caricano dati di gioco o avviano gli strumenti
elencati. Un ID sconosciuto, argomenti aggiuntivi o una forma non supportata
producono JSON con `outcome: invalid-request` ed exit 1. Le query valide
restituiscono exit 0; `--json` deve essere l'ultimo argomento.

Ogni descrittore riporta ID stabile, titolo, sorgente e handler, sintassi CLI
attuale, input richiesti, parametri e relativi vincoli/default, stato consentito,
effetti, risultato e limiti. `menu.current` identifica l'ingresso esistente;
`menu.destination` indica la destinazione nel menu integrato previsto.
`menu.implemented: false` distingue una destinazione ancora da realizzare.
La presenza nell'inventario non rende un'operazione eseguibile da nuovi canali.

`parameters[].current_menu` distingue valori già selezionati, valori fissati
nell'handler, controlli mancanti e opzioni non pertinenti al menu, come la
scadenza del client locale. `open_work` conserva gli ID DBG2 responsabili delle
lacune. Per esempio, il menu permette passi di 1 o 10 frame con tasti rilasciati;
la CLI accetta 1…600 frame e una maschera esplicita. L'editor dei parametri
resta lavoro DBG2-05.

Gli ID delle operazioni compaiono anche nei nuovi eventi `host-action` della
traccia UI, nelle risposte del controllo locale e nel comando in attesa.
Il collegamento è descrittivo: menu e controllo continuano a usare gli handler
esistenti e le loro validazioni. Una scelta di profilo ha ID distinto
`replay.select`; l'avvio è `replay.run`. Gli ID dei tipi di replay conservano
la distinzione tra replay, audit, capture e compare.

Il formato delle query è `schema: 1`, `kind: ffta-debug-operations`, con
`contract_version`, `outcome`, `message`, `operations` e il contratto di risultato.
I risultati di esecuzione mantengono i formati già disponibili, descritti per
operazione. Il futuro involucro comune di richieste/esiti è esplicitamente
pianificato in DBG2-13/15; non è una nuova API di esecuzione. Il canale locale
mantiene schema 1 e aggiunge solo `operation_id` a risultato e stato pendente.
Timeout e richiesta di annullamento richiedono ancora verifica dello stato.

## Dialoghi e impostazioni ancora da migrare

Il censimento iniziale aveva tre siti automatici nel frontend: profilo replay,
catalogo e fallback del report issue. L'implementazione DBG2-04 li sostituisce con
la pagina interna e non avvia l'azione dopo la scelta. Rimangono soltanto
`content_picker.rs::Picker::browse_file` e `browse_folder`, da pulsanti espliciti.
I casi della nuova migrazione restano in verifica nel TODO. I descrittori riportano anche i valori fissi delle
tracce codice/gameplay/recent, stream/frame audio, confronto grafico precedente,
indirizzo/lunghezza di Inspect PC, sintomo issue e preparazione dei profili.
Gli strumenti host hanno una destinazione prevista e richiedono gli adattatori
espliciti di DBG2-06; un nome di script non diventa un comando shell.

## Verifica e manutenzione

I test del contratto verificano query positive e rifiuti, unicità degli ID e dei
parametri, destinazioni, lacune dichiarate e corrispondenza menu/controllo.
I match Rust sugli enum sono esaustivi. I controlli Python confrontano i flag
presenti nei parser, le varianti menu e i siti di dialogo con l'inventario;
i flag passati ai processi figli sono distinti dalle opzioni del comando padre.
Aggiornare descrittore e controllo quando si aggiunge una nuova operazione.

La validazione di questa unità riguarda metadati e protocollo locale con fixture
sintetiche; non richiede ROM, replay o nuove prove di scene. I risultati effettivi
di build e test sono nel record collegato dal
[TODO DBG2](development/DEBUG_TOOLS_V2_TODO.md). Il selettore, i ruoli tipizzati e i parametri interni sono disponibili; console
e cheat seguono lo stato del TODO DBG2.

Le operazioni `workbench.*` distinguono apertura/modifica della bozza da verifica,
preset, preparazione e avvio. Gli ingressi precedenti audio/codice/gameplay/
recent/grafica/ispezione/issue aprono i parametri; il loro ID di apertura è
separato da `control.*`, che continua a indicare l'handler effettivo. La validazione DBG2-05 è collegata dal TODO; le console condivise
restano DBG2-13/15.

Gli ID `tools.homepage`, `tools.campaign-report`, `tools.artifact-preview`,
`tools.recipe` e `tools.guest-scripts` identificano le cinque schede di
`Debug > Scripts and tools`. Dettagli legge metadati/hash; solo Avvia lancia
l'adattatore dichiarato o la ricetta readonly. Annulla ferma il processo
supervisionato e conserva l'esito/log; la vista guest legge soltanto i dati.
Vedere [il selettore e la libreria](debug_selector.md) per prerequisiti e limiti.

Il [menu Atlante del Clan](menu_clan_atlas.md) raggruppa i percorsi in otto sezioni Debug e offre la scheda di sola lettura Riferimenti Menu / CLI. Il registro conserva ID, parametri, handler e protezioni; cambiano solo i percorsi del menu.

Le operazioni `cheats.open`, `cheats.choose`, `cheats.refresh`, `cheats.list`,
`cheats.show` e `cheats.info` descrivono la consultazione locale del catalogo.
`cheats.inspect-plaintext` descrive la sola ispezione CLI basata su un'ipotesi
esplicita; `cheats.inspect-seeded` descrive quella con chiave fornita dal
chiamante. Nessuna verifica automaticamente dispositivo o cifratura né abilita
codici. `cheats.inspect-source` richiede provenienza e master espliciti e può
confrontare il checksum con una ROM selezionata; il risultato resta consultivo.
`cheats.prepare-once` e `cheats.apply-once` descrivono anteprima e applicazione
manuale vincolata allo stato in una copia isolata; non rappresentano toggle
mantenuti né la verifica degli effetti del dispositivo originale.
`control.retry` descrive il recupero esplicito di una
richiesta con identità invariata. I descrittori non espongono nuove azioni al
canale locale oltre alle azioni effettivamente supportate. Selezione per ID,
refresh asincrono e letture paginate del catalogo sono disponibili nel canale
locale e nel linguaggio della console; il refresh della vista menu usa ancora
un percorso separato. Dettagli in [debug control](debug_control.md).

## Linguaggio debugger — DBG2-13 in sviluppo

Il parser testuale limitato a una riga supporta virgolette e numeri esadecimali
con prefisso `0x`; rifiuta conteggi decimali ambigui e non interpreta una shell.
La CLI sperimentale `recomp debug-command SESSION 'COMMAND'` traduce i comandi
supportati negli stessi tipi e nello stesso canale autenticato di `debug-control`.
`help control.step`, `status` e `step 1 --keys 0x3ff` sono stati verificati sotto
Wine; un input malformato produce `parse-error` senza mutare la sessione. Nella
console la bozza resta modificabile e Ctrl+Shift+C copia il JSON dell'errore
corrente, non il risultato precedente. I fallimenti locali distinti e la
possibile consegna sconosciuta sono descritti in [debug control](debug_control.md).
`profile
select ID` e `catalog select ID` sono collegati al selettore. Una sessione
Wine/Xvfb ha verificato la selezione di un catalogo sintetico e il rifiuto di
tipo errato o ID sconosciuto senza cambiare stato, frame o clock. Un'altra prova
Wine/Xvfb ha selezionato due ID distinti di profili replay sintetici completi,
uno dalla CLI e uno dalla console, con hash corretti e stato invariato. La
selezione verifica i riferimenti senza avviare il replay; la completa equivalenza
richiesta da DBG2-13 non è coperta da questa prova. `replay run` e le modalità
`audit`, `capture` e `compare` ora richiamano il supervisore condiviso sul
profilo selezionato, con stato, esito e cancellazione per ID tramite il canale
locale. Le istruzioni e i limiti sono in [debug control](debug_control.md).
`content refresh` e `content list` usano la stessa scansione e l'indice del
selettore attraverso il canale autenticato: la lettura è paginata e può
vincolare la generazione. Una prova Wine su copia sintetica ha scoperto e
selezionato un catalogo per ID senza aprire il menu; dettagli e limiti sono in
[debug control](debug_control.md).

Il selettore dispone in preparazione di un lavoro interno per ID: riusa la
verifica dei riferimenti tipizzati, mantiene il risultato in sospeso fino alla
conclusione e scarta annullamenti o contesti/cataloghi cambiati. È ora
collegato al trasporto dei comandi; l'esito rimane pendente fino al risultato
del lavoro. In una copia sintetica Wine, una ricerca pendente ha rifiutato un
ID errato ed è stata annullata dalla console con l'ID esatto; una seconda ha
raggiunto la scadenza, distinguendo il timeout client dalla conferma server.
La risposta terminale corretta non riporta più il job concluso come pendente;
una nuova prova Wine ha verificato il risultato e lo status successivo.
La CLI `debug-control retry` permette di recuperare un risultato con lo stesso
run, ID, scadenza e azione: una prova Wine ha confermato che il ritento di un
passo concluso o di una ricerca pendente non li esegue di nuovo. La console e
`debug-command` accettano `retry RUN ID SCADENZA -- COMANDO`; una prova Wine ha
confermato dalla console il recupero del risultato del passo e il rifiuto di
parametri cambiati. I dettagli e i limiti della memoria per run sono descritti
in [debug control](debug_control.md).

È in sviluppo una finestra «Console comandi», apribile da Debug / Panoramica
in una copia di debug isolata. La bozza supporta cursore, selezione, cronologia,
IME e appunti. Tab propone nomi, opzioni, valori, ID di contenuto e l'ID della
richiesta pendente; una riga mostra la sintassi contestuale. Se ci sono più
proposte, Tab le scorre nella bozza. Il completamento rispetta virgolette e
posizione del cursore e non invia comandi. Il pannello mostra etichetta della
sessione, prefisso hash ROM e run, insieme all'ID e ai parametri del job
pendente; passi e ricerche mostrano i rispettivi budget rimanenti, mentre per
scansioni e selezioni indica soltanto che sono in corso. I dati provengono dal
frontend, aggiornati al massimo quattro volte al secondo, senza richieste
automatiche al canale o hash dello stato a ogni aggiornamento. L'identità
visualizzata è verificata contro il manifesto del run all'apertura; i valori
lunghi possono essere abbreviati. Il JSON di `jobs status`, copiabile, conserva
ID e parametri completi del job pendente.
Una prova Wine/Xvfb ha mostrato una ricerca in avanzamento e, dopo il suo
annullamento, la tastiera su schermo in una finestra 480×360. Invio passa
dal client del canale locale usato dalla CLI. Il testo incollato su più righe resta nella bozza e viene
rifiutato dal parser finché non è corretto. Sotto Wine/Xvfb un comando `step 1`
ha prodotto lo stesso stato completo della CLI e la scorciatoia F9 è rimasta
isolata. La chiusura del pannello filtra brevemente le pressioni sintetiche di
ritorno del fuoco; una prova tracciata ha verificato riapertura ed Esc ordinario
senza alterare il gioco. Un'altra prova Wine/Xvfb con prefisso e appunti isolati
ha verificato copia e taglio della selezione, incolla Unicode multilinea e
copia del JSON risultato. Invio sulla bozza multilinea è stato rifiutato senza
pubblicare comandi o cambiare lo stato del gioco. IME reale, controller fisici
ed equivalenza completa restano da verificare: queste prove non chiudono
DBG2-13/14.

La console offre anche una tastiera su schermo navigabile con il controller:
direzioni e conferma scelgono caratteri, cronologia, preset e azioni; `Send`
invia esplicitamente. Dopo l'uscita, il pad deve tornare neutro prima che il
menu principale accetti nuovi input. Una prova Wine/Xvfb con XInput simulato
ha verificato preset `status`, cronologia, annullamento e chiusura con conferma
tenuta, senza modificare stato, frame o clock. I controller fisici restano da
verificare.
