# Contratto delle operazioni di debug

DBG2-01 aggiunge un inventario consultabile delle operazioni esistenti. Il
contratto versione 1 contiene 97 operazioni e 329 parametri (65/242 nella
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

## Linguaggio debugger — DBG2-13 in sviluppo

Il parser testuale limitato a una riga supporta virgolette e numeri esadecimali
con prefisso `0x`; rifiuta conteggi decimali ambigui e non interpreta una shell.
La CLI sperimentale `recomp debug-command SESSION 'COMMAND'` traduce i comandi
supportati negli stessi tipi e nello stesso canale autenticato di `debug-control`.
`help control.step`, `status` e `step 1 --keys 0x3ff` sono stati verificati sotto
Wine; un input malformato viene rifiutato senza mutare la sessione. `profile
select ID` e `catalog select ID` sono analizzati, ma il loro collegamento al
selettore è ancora aperto e la CLI li rifiuta esplicitamente. L'editor nel menu,
la selezione per ID e la completa equivalenza richiesta da DBG2-13 restano da
implementare e verificare.
