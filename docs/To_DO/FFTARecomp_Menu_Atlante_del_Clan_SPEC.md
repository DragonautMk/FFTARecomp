# FFTARecomp — Specifica di implementazione del menu «Atlante del Clan»

> **Documento operativo per Codex / agente di sviluppo**  
> **Stato:** specifica approvata dell'interfaccia, non codice già implementato.  
> **Progetto:** `DragonautMk/FFTARecomp`, branch `decomp`  
> **Baseline verificata:** `999d0a617296cae3b2ad9b585f1df03faa45a134` (29 ottobre 2026)  
> **Obiettivo:** realizzare **nel menu nativo Rust** l'interfaccia approvata, preservando tutte le funzionalità disponibili e distinguendo chiaramente le funzioni Debug V2 ancora pianificate.

## 0. Istruzione principale per l'agente

Implementa nel progetto FFTARecomp un nuovo menu di pausa/debug nativo chiamato, a livello di *design*, **«Atlante del Clan»**. L'obiettivo è una nuova interfaccia grafica e un'organizzazione delle funzioni, **non** la sostituzione del motore di gioco, del sistema di salvataggi o del debugger. Preserva le API pubbliche, i formati dei file, gli handler e le semantiche esistenti; modifica il routing solo dove necessario e con test. Non convertire l'applicazione in HTML/Electron/WebView/Godot: il prototipo HTML era un riferimento visivo e comportamentale, non la tecnologia di destinazione.

**Vincoli non negoziabili:**

1. **Zero regressioni:** nessuna voce esistente deve scomparire, diventare irraggiungibile o perdere effetti/validazioni. Mantenere il menu originale come riferimento di confronto, non come seconda implementazione divergente.
2. **Navigazione laterale permanente** con il selettore `Gioco | Debug` in alto, sulla **sinistra**, non come due schede orizzontali sopra il contenuto.
3. **Dimensioni dei contenitori stabili:** intestazioni, sidebar, tab, footer, titolo della pagina, controlli globali e aree di lista/dettaglio non devono spostarsi durante lo scroll o il cambio selezione. Lo scorrimento avviene **solo negli elenchi interni**.
4. **Larghezza adattativa:** niente finestra fissata a 1048 px e niente scorrimento orizzontale della pagina. Il layout deve adattarsi alla finestra del gioco, usando una diversa disposizione **solo dei contenuti interni** quando lo spazio non basta.
5. **Torna al gioco** deve essere visivamente **un pannello dello stesso sistema di design**: stessa cornice, stesso materiale, margini, padding, hover e focus. Niente pulsante decorato in modo estraneo o fuori griglia.
6. Le nuove scorciatoie di volume e velocità devono funzionare **in qualunque posizione del menu aperto**, senza necessità di evidenziare un controllo. Le due combinazioni simultanee hanno priorità sui tasti singoli.
7. **Dati veri nella build reale:** nessuna data fittizia, conteggio artificiale di save state, sessione debug di esempio o risultati diagnostici simulati. Un valore mancante deve essere segnalato come tale.
8. **Debug attuale ≠ Debug V2:** non rendere operativa una voce V2 finché il relativo backend non esiste e non è verificato. Le attività V2 devono risultare chiaramente *Pianificate* o *Non disponibili* se mostrate nella UI.
9. Conservare l'accessibilità delle operazioni da **tastiera, mouse e gamepad** con focus visibile e corretta cattura dei binding.
10. Non dichiarare completata un'implementazione che non compila, non è collegata ai veri handler o non ha superato i test previsti.

Prima di modificare il codice, **rileggi la branch nel workspace**: i file possono essere cambiati dopo la baseline indicata.

---

## 1. Sorgenti da esaminare prima di intervenire

| Percorso | Ruolo |
| --- | --- |
| `crates/menu/src/lib.rs` | `Menu`, `Page`, `Row`, `MenuAction`, mapping delle pagine e gestione input |
| `crates/menu/src/theme.rs` | Rendering nativo della UI in pixel/RGBA, geometria dei pannelli, dropdown, hit test |
| `crates/menu/src/menu_icons.rs` | Risorse e icone del menu |
| `crates/menu/src/debug_catalog.rs` | Catalogo DS, filtri, selezione e anteprima |
| `crates/recomp/src/play.rs` | Integrazione del menu con gioco, ROM, stato, debugger e input |
| `crates/input-config/src/lib.rs` e `storage.rs` | Binding dinamici di tastiera/gamepad, `key.menu` / `pad.menu`, persistenza |
| `crates/screen/src/profile.rs` e relativi moduli | Modelli schermo e configurazione video |
| `docs/menu_shortcuts.md` | Semantica effettiva di apertura/chiusura menu e combo personalizzate |
| `docs/debug_tools.md` | Strumenti debugger esistenti, CLI e integrazioni |
| `docs/development/DEBUG_TOOLS_V2_TODO.md` | 28 estensioni V2: **pianificazione, non implementazione** |
| `docs/portable.md` | File, cartelle, save, migrazione e modalità portable |

Riferimenti online:

- https://github.com/DragonautMk/FFTARecomp/tree/decomp
- https://github.com/DragonautMk/FFTARecomp/blob/decomp/crates/menu/src/lib.rs
- https://github.com/DragonautMk/FFTARecomp/blob/decomp/docs/menu_shortcuts.md
- https://github.com/DragonautMk/FFTARecomp/blob/decomp/docs/development/DEBUG_TOOLS_V2_TODO.md

**Importante:** prima delle modifiche genera un inventario programmatico delle azioni e delle voci attuali (incluse le sottomaschere, dropdown, dialoghi di conferma e comandi CLI). Mappa ogni azione alla destinazione nel nuovo menu. I nomi del prototipo non sostituiscono gli identificativi reali del codice.

---

## 2. Design visivo — «Atlante del Clan»

### 2.1 Carattere e direzione artistica

Interfaccia da **RPG tattico fantasy per GBA**, non dashboard SaaS né fantasy generico prodotto da un template. Identità ispirata a un **registro di clan / atlante di missioni**: piccoli fregi geometrici, stemma semplice originale, tipografia serif discreta nei titoli, numerazioni laterali, blu minerale scuro e ottone consumato. Evitare dorature eccessive, gradienti vistosi, ombre flottanti, card arrotondate ovunque o icone decorative arbitrarie.

Palette iniziale indicativa (da armonizzare con il renderer e contrasto effettivo):

| Ruolo | Colore |
| --- | --- |
| Sfondo profondo | `#091827` |
| Sidebar | `#112B40` → `#1B374D` |
| Pannelli | `#1C384E` |
| Pannelli selezionati | `#2D4E64` |
| Testo principale | `#E9EFEC` |
| Testo secondario | `#A8BDC7` |
| Ottone / elementi importanti | `#D1AF75` |
| Oro chiaro / focus | `#F0D7A4` |
| Bordi | `#59798B` |
| Conferma / stato valido | `#8BC6AD` |
| Avviso / effetto distruttivo | `#C78B77` |

- Titoli in serif esistente/disponibile nel progetto; testo delle opzioni in font leggibile e uniforme. **Non incorporare font esterni se non autorizzati e legalmente distribuibili.**
- Usare uno stemma **originale minimale** costruito con primitive geometriche native; evitare loghi o artwork copiati dal gioco.
- Evidenziare le righe attive con bordo ottone e cambio del tono blu; contrasto netto anche in hover/focus da gamepad.
- I bordi dei pannelli devono essere sottili e coerenti. È sufficiente un piccolo angolo metallico, non servono cornici intricate dappertutto.
- Ogni sezione deve avere una gerarchia leggibile: soprattitolo (facoltativo), titolo, descrizione breve, **area di lavoro stabile**, eventuale footer azioni.

### 2.2 Layout di riferimento

```text
┌─────────────────────────────────────────────────────────────────────────┐
│ ✥ FFTARecomp                                        THE CLAN ARCHIVE  │
├────────────────────┬────────────────────────────────────────────────────┤
│ [ stemma originale ]│ Percorso / sezione                    [Eventi][CMD]│
│ FFTARecomp          ├────────────────────────────────────────────────────┤
│ PAUSE & DEBUG MENU  │ VOL n% | FAST n% | SAVE/SESSIONE | FRAME ...      │
│ [ Gioco | Debug ]   ├────────────────────────────────────────────────────┤
│                    │ Titolo pagina / sottotitolo immobile              │
│ 01 Panoramica      │ ┌────────────────────────────────────────────────┐ │
│ 02 ...             │ │                                                │ │
│ 03 ...             │ │    CONTENUTO — cambia, con scroll interni      │ │
│ 04 ...             │ │                                                │ │
│ ...                │ └────────────────────────────────────────────────┘ │
│                    ├────────────────────────────────────────────────────┤
│ APRI / CHIUDI MENU │ VOLUME L1[-] L1+R1[MUTE] R1[+]                   │
│ [SELECT]+[START]   │ FAST   L2[-] L2+R2[ON/OFF] R2[+]                 │
│ binding dinamico   ├────────────────────────────────────────────────────┤
│ ◂ Indietro         │ Stato/risultato dell'operazione                   │
└────────────────────┴────────────────────────────────────────────────────┘
```

### 2.3 Adattività **senza spostamenti dovuti allo scroll**

Il prototipo finale ha mostrato un errore da **non riprodurre**: dimensioni orizzontali fisse che richiedono lo scorrimento dell'intera interfaccia. La UI definitiva deve rispettare la finestra disponibile.

Regole:

- **Altezza e posizione** di sidebar, barra superiore, titolo, barra di stato, footer scorciatoie e barra L/R determinate dal layout della finestra e tenute ferme mentre l'utente scorre.
- **Larghezza flessibile**: sidebar proporzionata e limitata da un minimo/massimo sensato; resto della larghezza al pannello principale. Misurare il testo reale e gli hit box nel renderer nativo.
- A larghezze ampie, le impostazioni lunghe e gli strumenti Debug si dispongono su **due colonne interne**. L'elenco Debug scorre nella propria colonna; dettagli, titolo e azioni di conferma **rimangono nella colonna accanto**.
- A larghezze ridotte le **colonne del contenuto** possono diventare due aree interne sovrapposte verticalmente, ciascuna con il proprio limite e scroll. La sidebar non si trasforma in una barra superiore. Non consentire al contenuto di far crescere la finestra o spingere fuori schermo i footer.
- Su schermi estremamente piccoli, prevedere una modalità compatta leggibile e navigabile, oppure segnalare un minimo tecnico reale del renderer. Non ricorrere a overflow orizzontale dell'intera UI.
- Quando si modifica un valore o si seleziona una riga, mantenere la posizione di scroll **della lista pertinente**, il focus e il rettangolo della zona dettagli. Cambia il contenuto, non la geometria.
- Solo le liste o i campi testuali che lo richiedono devono scorrere. Niente scrollbar generale che sposti `Torna al gioco`, sidebar o comandi.
- Il `Torna al gioco` della pagina iniziale è **dentro un pannello di altezza prefissata nella griglia**, usando gli stessi colori, bordo, focus, margini interni e tipografia delle altre voci. È un'azione primaria, non un banner estraneo alla griglia.

I valori numerici di larghezza/altezza vanno derivati dalle dimensioni del canvas/frame; **non codificare 1048×720 px come unica risoluzione supportata**. Per le liste usare clipping e scroll index/offset coerenti con i calcoli di hit-testing.

---

## 3. Sidebar: struttura da preservare esattamente

Il selettore **Gioco / Debug** si trova **nella parte alta della colonna sinistra**. Ogni scheda ha una lista propria. Cambiare scheda non azzera arbitrariamente la sezione precedente né il salvataggio rapido selezionato.

### Scheda **Gioco** — 8 voci

1. **Panoramica**
2. **Salvataggi**
3. **Fast Forward**
4. **Finestre di testo**
5. **Audio**
6. **Video**
7. **Controlli**
8. **Sistema**

### Scheda **Debug** — 8 voci

1. **Panoramica**
2. **Sessioni e casi**
3. **Replay e confronti**
4. **Esecuzione e memoria**
5. **Tracce e ispettori**
6. **Console e script**
7. **Cheat**
8. **Report e archivio**

La scheda Debug **non deve ridursi a poche card decorative**: l'utente deve poter accedere all'inventario reale delle opzioni. Ogni gruppo Debug supporta una vista **«Menu/CLI attuali»** e, quando rilevante, una vista **«Debug V2 pianificati»**.

In fondo alla sidebar, sempre visibile: **«APRI / CHIUDI MENU»** + pulsanti correnti (es. `SELECT + START`) + indicazione del dispositivo. Il binding va letto dalla configurazione reale (`InputConfig`, `menu_key`, `menu_pad`) e dalla mappatura display del controller. I default verificati sono `Escape` su tastiera e `Start + Back/Select` su controller; il comando può essere rimappato. `Guide/Mode` resta una via aggiuntiva dove supportata. Le combinazioni si attivano una sola volta per pressione completa, non vanno ripetute al rilascio parziale; `Escape` mantiene la via di recupero documentata.

---

## 4. Scheda Gioco: comportamento completo

### 4.1 Panoramica

**Ordine verticale fissato:**

1. **Torna al gioco** — dentro il suo pannello, allineato con gli altri.
2. **Salvataggi rapidi** — selettore immediato (senza entrare in Salvataggi).
3. **Volume** e **Fast Forward** — due riquadri affiancati su larghezze ampie, disposti nella stessa area interna compatta su larghezze minori.

La geometria dei primi due riquadri non deve dipendere dall'altezza dei due controlli inferiori. Se questi richiedono altro spazio, **scorrono nella sola zona a loro destinata**.

### 4.2 Salvataggi rapidi e salvataggio standard

Preservare **Save states**, **Load state**, **Save state**, **Replace state con conferma**, **Undo last operation** e relativi errori/stati di indisponibilità. Supportare i **9 slot standard** e i **99 slot** quando `Extended saves` è abilitato, senza nascondere i rimanenti o limitarli ai primi 9.

- Alla riapertura del menu, **l'ultimo slot rapido utilizzato/salvato deve rimanere selezionato**, pronto per **Sovrascrivi** o **Carica**. Conservare correttamente selezione, slot esistente, eventuale undo e stato corrente.
- Selezionare uno slot **non deve salvarlo**. Carica è disabilitato per slot vuoti. Sovrascrivere richiede conferma se già previsto dall'handler corrente.
- Mostrare timestamp reale dello slot quando reperibile, altrimenti «Data non disponibile». Non creare date di fantasia.
- Sotto il selettore mostrare **«Ultimo salvataggio standard del gioco»** con data/ora reali ricavate dalle informazioni attendibili sul salvataggio `.sav`. Se si usa la `mtime` del file, denominarla correttamente **«Ultima modifica file .sav»** anziché inventare un timestamp interno al gioco. Se non esiste alcun salvataggio, dire «Nessun salvataggio standard trovato».
- I salvataggi rapidi e quelli standard sono **due sistemi distinti**. Non leggere la data dell'uno dall'altro.

### 4.3 Salvataggi — gestione completa

Oltre all'accesso rapido in Panoramica:

- `Extended saves` ON/OFF, compatibilità con struttura su disco e 99 slot.
- `Game saves`: slot, disponibilità, slot cartuccia attivo, **Use and restart**, **Copy current save**; conferma esplicita per sostituzione/sovrascrittura.
- `Save backups`: **Copy 1 — latest**, **Copy 2 — previous**, **Restore and restart** con conferma; selezione di backup e gestione errori.
- Proteggere da perdita di dati in caso di annullamento, slot vuoto, operazione invalida, ROM cambiata o copia incompatibile.

### 4.4 Controlli globali di volume e Fast Forward

**Attivi sempre mentre il menu è aperto**, anche da Debug e quando il focus è su liste, titoli, altri controlli o dropdown dove l'interazione non deve essere consumata dal campo in editing.

| Combinazione proposta | Risultato |
| --- | --- |
| `L1` | Volume −10 punti percentuali; intervallo 0–100% (coerente con gli step 0.1 oggi presenti) |
| `R1` | Volume +10 punti percentuali |
| `L1 + R1` | Attiva/disattiva `Mute`, **preservando il valore di volume** |
| `L2` | Fast Forward −20 punti percentuali |
| `R2` | Fast Forward +20 punti percentuali |
| `L2 + R2` | Attiva/disattiva Fast Forward, **preservando la velocità selezionata** |

Le combinazioni vanno disambiguate: `L1+R1` non deve prima regolare il volume; `L2+R2` non deve prima alterare la velocità. Precedenza alle chord, con gestione di press/release/edge e ripetizione coerente. Il test deve includere la pressione quasi simultanea e il rilascio di un solo tasto. A menu chiuso i pulsanti rimangono assegnati al gioco come da binding esistente, senza sottrarre L/R o zoom al gameplay.

**Fast Forward a scatti del 20%:** impostazioni visive e runtime devono coincidere. Requisito progettato: `100%, 120%, 140%, …, 800%` (1.0×–8.0×); spegnendo il Fast Forward il gioco torna effettivamente a **100%** senza perdere il valore preferito. Il codice attuale supporta `2×`, `4×`, `8×` e audio `silent | fast | normal`: servirà estendere anche **scheduler/emulazione, audio-clock, serializzazione/configurazione e test**, non soltanto disegnare una percentuale nuova. Se questo sottocompito non è ancora funzionante **non visualizzare valori arbitrari come se fossero operativi**: mostra l'attuale comportamento e mantieni l'estensione come lavoro non completato.

La modalità di attivazione momentanea/toggle dei binding originali deve continuare a funzionare; non sostituire `ff_key`/`ff_pad` o le rispettive opzioni toggle senza integrare le due semantiche.

### 4.5 Fast Forward

- Velocità selezionata/effettiva, ON/OFF e controlli globali.
- **Audio mode**: `silent`, `fast`, `normal` (mantenere il comportamento implementato).
- Feedback separato per «velocità impostata» e «Fast Forward attivo».

### 4.6 Finestre di testo

Mantenere **Size** e **Length**, con intervalli e passi reali del progetto: `Size` 0.5–2.0 a passi 0.1, `Length` 1.0–2.0 a passi 0.1.

### 4.7 Audio

- Volume reale 0–100%; Mute persistente indipendente dal livello.
- **Enhanced Audio** ON/OFF.
- Cambiamenti applicati tramite i `Changed`/handler originali.

### 4.8 Video

Tutte le voci originali del menu:

- **Aspect**: `auto`, `3:2`, `16:10`, `16:9`, `20:9`, `21:9`, `32:9` dove confermate dalle costanti correnti.
- **Fullscreen (F11)**.
- **Screen**: `raw`, `unlit`, `frontlit`, `backlit`, `classic`.
- **Brightness** 0.5–1.5 a passi 0.1.
- **Darken**: `auto` oppure 0.0–1.0.
- **Response**: leggere esattamente `ResponseMode::ALL` dalla branch.
- **Grid**: `auto` oppure 0.0–1.0.
- **Scaling**: usare i valori esatti di `SCALING_MODES` dal codice; non inventare opzioni.
- **Vsync** e **FPS counter** (`off`, `internal`, `external`, `both`).

I parametri presenti **solo in configurazione** e non nel menu originale, come `display_gamut` e `response_keep`, possono essere raggruppati sotto «Avanzate» purché siano etichettati correttamente, con validazione effettiva e senza perdere la possibilità di impostarli dal file. Dove una modifica richiede il riavvio/nuovo lancio (es. certe proporzioni), dichiararlo invece di presentarla falsamente come immediata.

### 4.9 Controlli

Tre tab interni: **Tastiera**, **Gamepad**, **Mouse + tastiera**. Preservare:

- Binding `A`, `B`, `L`, `R`, `Start`, `Select`, `Up`, `Down`, `Left`, `Right`.
- Binding **Open menu**, **Fast Fwd**, **Toggle Fast Fwd**, **Zoom In**, **Zoom Out**.
- **Default layout** per gamepad.
- **Mouse & Keyboard** ON/OFF e logica di cambio tab.
- Rilevamento etichette gamepad, configurazione sorgente D-pad e deadzone dove presenti nella configurazione, senza inventare funzioni UI già esistenti.
- Cattura chord e persistenza `input.cfg` senza rompere press/release, annullamento e `Escape` di recupero.

Gli input di testo usati dal prototipo non devono diventare la nuova modalità principale di rimappatura: mantenere la **cattura nativa del dispositivo**.

### 4.10 Sistema

- **Restart ROM**.
- **Switch ROM**: `Return to game` / `Choose ROM…`.
- **Exit**: conferma `Yes, exit` / `No, go back`.
- Messaggi/annullamenti coerenti; nessuna chiusura imprevista del gioco.

---

## 5. Scheda Debug: inventario da mantenere

Le voci sono raggruppate secondo l'interfaccia approvata; **gli handler restano quelli della repo**. Dove il comando è accessibile solo da CLI, contrassegnarlo «CLI» fino a un'integrazione effettiva nella UI. Ogni azione che richiede una copia isolata deve continuare a bloccare l'operazione in assenza di sessione.

### 5.1 Sessioni e casi

- `Extended saves`
- `Game saves`
- `Start debug session`
- `Close debug session`
- `Debug catalog`
- `Start input recording`
- `Stop input recording`
- `Copy diagnosis` (può anche essere visibile in Report)
- Debug catalog: **Region**, **Recorded status**, **Kind**, **DS case**, **Entry**, **Import to free snapshot slot**, **Open isolated case**, **Choose / reload catalog**, con anteprima e messaggi di errore.
- CLI collegate a sessioni, catalogo e registrazioni, se già supportate e validate.

### 5.2 Replay e confronti

- `Replay case`
- `Choose replay profile`
- `Cancel replay`
- `Audit replay`
- `Capture on trigger`
- `Compare first difference`
- Eventuali campagne e audit CLI **esistenti**, chiaramente etichettati quando non integrati nel menu.

### 5.3 Esecuzione e memoria

- `Step 1 frame (F7)`
- `Step 10 frames (F8)`
- `Resume (F6)`
- `Inspect PC / registers`
- Operazioni disponibili in `debug-control` (pause/status/inspect/break/watch) con distinzioni di supporto per esecuzione compiled/interpreter; non inventare capacità mancanti.
- I futuri trigger nominati e valori fissati a schermo sono *V2*, non funzionalità già presenti.

### 5.4 Tracce e ispettori

- `Capture graphics report`
- `Start UI / input trace`
- `Save UI / input report`
- `Start audio capture`
- `Save audio report`
- `Start code trace`
- `Save code report`
- `Start task / script trace`
- `Save gameplay report`
- `Start recent events`
- `Freeze / save recent events`

### 5.5 Console e script

- Mostrare gli strumenti locali/CLI già presenti e le loro capacità reali.
- Prevedere gli ingressi per **Console eventi**, **Console comandi**, **Script e ricette**. In baseline sono **estensioni V2 pianificate**: le finestre demo non costituiscono un'implementazione funzionante.
- Quando le console diventeranno operative, devono essere **apribili dalla barra superiore come overlay ancorato** e non modificare la geometria dei pannelli sottostanti. Console eventi sola lettura; console comandi con validazione e dispatcher condiviso con il menu, non shell arbitraria.

### 5.6 Cheat

- Catalogo locale CH e import **solo quando saranno completate le dipendenze DBG2**.
- Non includere o pubblicare il catalogo privato gitignored né i relativi codici nel repository.
- Non affermare che i **341 blocchi documentati** siano verificati, utilizzabili o decodificati; nella baseline risultano `not-runtime-tested`.
- Applicazione limitata a copia di debug isolata, con conferma, compatibilità e provenienza; `Disattiva` non equivale a rollback. Per annullare gli effetti richiedere checkpoint esplicito.

### 5.7 Report e archivio

- `Copy diagnosis`
- `Create issue report`
- Report UI/audio/codice/gameplay/recenti in continuità con gli handler già esistenti.
- Strumenti e script CLI disponibili per homepage e pulizia/preview degli artefatti, con protezione delle prove e dei salvataggi.

### 5.8 Debug Tools V2 — esattamente 28 attività pianificate

Mostrare un filtro o una vista separata «**Debug V2 — pianificato**», con i codici stabili e i nomi della checklist originale. La UI non deve far passare queste voci per completate.

| Gruppo nella nuova sidebar | Attività |
| --- | --- |
| Sessioni e casi | `DBG2-01` Inventario azioni; `02` Catalogo comune; `03` Selettore interno; `04` Migrazione delle scelte; `05` Parametri e preset; `06` Libreria script e strumenti; `07` Persistenza portable |
| Tracce e ispettori | `DBG2-08` Schema eventi; `09` Selezioni/azioni host; `10` Eventi runtime; `11` Console eventi live; `12` Limiti e prestazioni |
| Console e script | `DBG2-13` Linguaggio comandi; `14` Editor/completamento; `15` Risposte strutturate/job; `16` Ricette e batch |
| Cheat | `DBG2-17` Import catalogo; `18` Decoder; `19` Applicazione controllata; `20` Pagina cheat; `21` Primo sottoinsieme EUR; `22` Cheat avanzati; `23` Provenienza/checkpoint/replay |
| Esecuzione e memoria | `DBG2-24` Trigger e arresti nominati; `25` Valori e differenze fissati a schermo |
| Report e archivio | `DBG2-26` Timeline; `27` Verifiche di regressione; `28` Guida/distribuzione |

**Sono attività trasversali:** questa collocazione serve alla navigazione, non sostituisce la sequenza di dipendenze della checklist `DEBUG_TOOLS_V2_TODO.md`.

---

## 6. Pattern di interazione da implementare

### 6.1 Pannello a due aree per gli strumenti Debug

La lista e i dettagli devono essere due aree **con geometria indipendente**:

```text
┌───────────────────────────────┬────────────────────────────────────────┐
│ [Menu + CLI] [V2 pianificati] │ Titolo strumento + disponibilità      │
├───────────────────────────────┤────────────────────────────────────────┤
│ [01 Strumento A]              │ Parametri / filtri / info            │
│ [02 Strumento B]              │ Scroll interno SOLO se necessario    │
│ [03 Strumento C]              │                                        │
│     SCROLL INTERNO ↓          │                                        │
├───────────────────────────────┤────────────────────────────────────────┤
│                               │ [stato/prerequisiti] [Avvia / Conferma]│
└───────────────────────────────┴────────────────────────────────────────┘
```

Il titolo e il pulsante di avvio rimangono fermi anche con descrizioni lunghe. La selezione della voce non porta il pannello dettagli a un'altra coordinata. Quando la larghezza non è sufficiente, **ridisporre soltanto queste aree interne** in una divisione verticale, assegnando un'altezza fissa relativa a ciascuna.

### 6.2 Focus, mouse e controller

- Navigazione per focus/tab e D-pad, tasti di conferma/annulla, mouse hover/click, wheel e scrollbar quando presenti.
- Focus e selezione sono stati distinti dal semplice mouse hover, salvo le regole già implementate. Non introdurre doppia esecuzione per input tenuto.
- Non cambiare scroll offset di una lista quando si aggiorna un valore o si seleziona un elemento. Resettarlo solo quando si entra volutamente in una sezione diversa o cambia la fonte dei dati.
- Modali di conferma sovrapposte all'area corrente senza modificarne la geometria; `Cancel` torna al contesto precedente, focus ripristinato.
- Prima di eseguire un comando distruttivo mostrare target, slot, profilo o sessione e il suo effetto reale. Non accoppiare selezione ed esecuzione.

### 6.3 Stato e persistenza

Separare chiaramente:

- Preferenze persistenti (`AvConfig`, input cfg, opzioni video, velocità preferita quando supportata).
- Stato della sessione corrente (Fast Forward ON/OFF, debug copy attiva, registrazioni, job).
- Stato della UI (scheda corrente, voce selezionata, scroll offset, filtri, dropdown aperti).
- Metadati letti dal disco (save timestamp, file disponibili, backup, cataloghi).

Non sovrascrivere file di configurazione per operazioni puramente visive. Non far dipendere il gameplay dai dati temporanei della UI.

---

## 7. Architettura tecnica consigliata

**Riutilizza** il renderer presente in `crates/menu/src/theme.rs` e gli handler di `crates/menu/src/lib.rs`. Possibile refactor incrementale:

1. **Catalogo di navigazione:** per ogni voce mostrare `id`, `label`, `group`, stato supporto, azione o pagina di destinazione, eventuali prerequisiti. Fare riferimento alle vere `Row`/`MenuAction`, senza duplicare la logica del debugger.
2. **Layout nativo:** funzione deterministica che calcola rettangoli di `sidebar`, `header`, `body`, `footer`, `tool_list`, `detail`, `modal` a partire da larghezza/altezza effettive; unico risultato per rendering e hit-testing.
3. **Rendering:** palette e componenti riutilizzabili (`Panel`, `NavRow`, `SettingRow`, `SlotButton`, `KeyCap`, `ConsolePanel`, `ConfirmDialog`). Nessun asset esterno non necessario.
4. **Input:** routing globale delle combinazioni L/R **solo quando il menu è aperto**; priorità alla combo e cattura/consumo dei singoli eventi. Gestire conflitti con rimappatura dei tasti e i controlli di gameplay.
5. **Sottomenu:** le funzioni reali restano collegate agli handler; conservare le transizioni `Changed` perché il runtime applichi immediatamente gli aggiornamenti che già supporta.
6. **Menu Debug:** riutilizzare selettori reali/catalogo DS esistenti; funzioni solo CLI con integrazione reale solo se è progettata/validata. Evitare falsi pulsanti «Esegui» che non fanno nulla.
7. **Prestazioni:** evitare rebuild, allocazioni e invalidazioni di framebuffer quando non cambiano pagina, stato, animazione o selezione. Testare entrambe le vie di presentazione GPU/CPU dove supportate.

Se il refactor richiede nuovi moduli, mantenerli dentro `crates/menu/src/` con responsabilità precise (es. `layout`, `navigation`, `widgets`), evitando di trasformare tutto in un unico file enorme.

---

## 8. Piano di lavoro raccomandato

### Fase A — Sicurezza / inventario

- Leggere e mappare tutte le righe di `MAIN_ROWS`, `rows_of(Page::...)`, `MenuAction`, tutte le opzioni configurabili e i shortcut.
- Preparare un **inventario prima/dopo** e una piccola matrice «voce originale → sezione nuova → handler» da conservare nei documenti di sviluppo.
- Eseguire i test attuali e registrare l'esito. Non rimuovere gli esistenti.

### Fase B — Struttura e stile nativo

- Implementare navbar sinistra e selettore `Gioco/Debug`.
- Aggiungere layout fluido con limiti minimi e scroll confinati.
- Implementare pannelli e stile «Atlante del Clan», incluso `Torna al gioco` coerente.
- Collegare tutte le azioni esistenti preservando conferme e state handling.

### Fase C — Funzioni approvate oltre la baseline

- Save state rapido in Panoramica con ultimo slot ricordato, sovrascrittura/caricamento e metadata veri.
- Data dell'ultimo salvataggio standard senza fonti fittizie.
- Footer scorciatoia di apertura/chiusura basata sui binding effettivi.
- Volume L1/R1 e mute chord; Fast Forward L2/R2 e toggle chord con consumo corretto.
- Estensione della scala Fast Forward 100–800%, passi 20%, **solo dopo** aver integrato il motore e i test di timing/audio.

### Fase D — Debug e validazione completa

- Migrare tutte le voci Debug attuali nella sidebar organizzata in 8 gruppi, mantenendo la distinzione `Menu / CLI / V2`.
- Collegare il catalogo DS, il debug session guard e gli altri handler reali.
- Mettere le V2 come *pianificate* oppure implementarle **separatamente** seguendo i task DBG2 e i loro criteri di chiusura, senza proclamare funzionalità future come già funzionanti.
- Eseguire test e smoke visuali del renderer con diversi aspect ratio e lunghezze di testo.

---

## 9. Test di accettazione obbligatori

### Layout e navigazione

- [ ] A 1280×720, 1024×768, 854×480 e un viewport stretto supportato: **nessuno scroll orizzontale della finestra principale**.
- [ ] Side menu, switch Gioco/Debug, barra superiore, titolo, footer shortcut, pulsanti L/R e status finale rimangono alle stesse coordinate durante scroll.
- [ ] Le liste Debug da 3, 9 e 20+ voci scorrono **solo all'interno** del loro riquadro; il pannello dettagli non salta.
- [ ] Cambiando strumento o modificando un parametro, scroll dell'elenco e selezione rimangono stabili.
- [ ] A larghezza stretta, solo il contenuto interno si dispone verticalmente; sidebar, tabs e footer rimangono in posizione.
- [ ] `Torna al gioco` condivide cornice/padding/focus/hover dei pannelli e non esce dalla griglia.
- [ ] Nessun testo importante viene troncato senza possibilità di aprire il dettaglio o leggere il valore intero.

### Gioco e input

- [ ] Tutte le 12 opzioni originali del menu sono raggiungibili nella nuova navigazione.
- [ ] Save/load/replace/undo, 9/99 slot, game saves, copia e restore backup funzionano come prima, con conferme.
- [ ] Ultimo slot rapido correttamente preselezionato alla riapertura; selezione non scrive sul disco.
- [ ] Timestamp standard reale o assenza esplicita; data del `.sav` non confusa con quella degli save states.
- [ ] L1/R1 regolano il volume da **ogni pagina** del menu; combo simultanea muta senza variare il valore.
- [ ] L2/R2 regolano la velocità da **ogni pagina**; combo simultanea toggla senza variarla.
- [ ] A menu chiuso non ci sono regressioni sui comandi L/R, zoom, fast toggle e input GBA.
- [ ] Se l'FF 20% è implementato: prove su 100, 120, 140, 200, 400, 800%, on/off, audio modes, serializzazione, clock e runtime. Altrimenti evidenziare espressamente il mancato completamento.
- [ ] Rimapparsi la combinazione di menu aggiorna subito i keycap in basso a sinistra; `Escape` di recupero e combo press/release funzionano.
- [ ] Navigazione e conferme mouse/tastiera/gamepad testate, inclusa cancellazione dropdown e modali.

### Debug e sicurezza

- [ ] Confronto **uno a uno** tra tutte le righe di `Page::Debug` e l'inventario delle nuove sezioni: nessuna azione orfana.
- [ ] Catalogo DS conserva filtri, anteprima, importazione e segnalazione degli errori.
- [ ] Le funzioni che richiedono copia isolata non si avviano senza sessione.
- [ ] Step frame, resume, trace, report, replay e diagnosi richiamano gli handler esistenti.
- [ ] Le CLI non diventano automaticamente «azioni Menu» se manca un'integrazione verificata.
- [ ] Le 28 DBG2 restano marcate *pianificate* fino a reale completamento; nessun cheat non verificato compare come attivo.
- [ ] Nessun file `.sav`, stato, log/prova o artefatto viene sovrascritto senza le condizioni e conferme previste.

### Qualità del progetto

- [ ] Compilazione e test dei crate interessati secondo i comandi documentati nella repository.
- [ ] Test unitari per geometria/hit-testing, clamp/scroll, switch scheda, combo controller, persistenza e conferme.
- [ ] Smoke test grafici per canvas piccolo/grande, renderer CPU/GPU dove disponibili.
- [ ] Documentazione aggiornata quando cambiano shortcut e opzioni.

---

## 10. Output richiesto all'agente che implementa

Consegnare:

1. **Codice reale nel repository** (non un nuovo mockup HTML) e lista dei file cambiati.
2. **Inventario «prima → dopo»** completo delle voci menu/debug con relativo handler.
3. Breve documentazione della struttura/sidebar e della logica del layout adattativo.
4. Risultati dei test e istruzioni per avviare il menu modificato.
5. Elenco preciso delle funzionalità **completate**, **rimaste operative come prima**, **ancora da realizzare** e degli eventuali limiti incontrati.
6. Se il Fast Forward 20% non è stato implementato nel motore, dirlo esplicitamente e non far apparire la UI come se applicasse velocità inesistenti.

### Definizione di completamento

Il compito è completato **solo** quando il menu nativo presenta lo stile «Atlante del Clan», la barra laterale approvata, il layout orizzontale adattativo con geometrie stabili, il pulsante `Torna al gioco` integrato, i controlli rapidi desiderati e **tutte le funzioni preesistenti ancora raggiungibili e operative**, con test superati. Ogni funzionalità Debug V2 non implementata deve restare dichiarata come futura, non simulata come reale.

---

*Specifica redatta a partire dalle richieste di progettazione del menu e dall'inventario della branch `decomp` consultato il 29/10/2026. Prima di procedere, verificare eventuali commit successivi alla baseline e adattare la mappa senza perdere le funzioni introdotte nel frattempo.*
