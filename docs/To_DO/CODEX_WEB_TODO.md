# FFTARecomp — TODO per Codex Web

**Stato:** pianificazione; nessuna voce è considerata completata solo perché presente in questo documento.  
**Repository:** `DragonautMk/FFTARecomp`  
**Branch di riferimento:** `decomp`  
**Obiettivo:** sviluppare strumenti complementari alla ricompilazione e all'estrazione ROMDATA, attraverso incarichi cloud indipendenti e pull request revisionabili.

> **Istruzione per Codex Web:** questa è una roadmap, **non** un ordine di implementare tutto in una sola sessione. Scegli la prima unità aperta compatibile con il codice corrente, concludila e proponi una PR limitata a quell'unità. Prima di ogni incarico rileggi la branch aggiornata: lo stato locale di Codex può essere più avanzato di GitHub.

## 0. Confini del lavoro e regole obbligatorie

- [ ] Leggere `AGENTS.md`, `README.md` e le istruzioni di sviluppo effettive della branch.
- [ ] Consultare, in sola lettura, `docs/development/ROM_DATA_TODO.md`, `ROM_DATA_INVENTORY.md`, `TASK_HANDOFF.md`, `DEBUG_TOOLS_TODO.md` e i contratti delle famiglie interessate.
- [ ] Individuare le funzionalità già implementate prima di progettare alternative: controllare anche `tools/`, `scripts/`, `docs/report_home.md` e i moduli esistenti.
- [ ] Annotare il commit SHA di partenza, la branch e i file che si intendono modificare.
- [ ] Creare una branch dedicata per **una sola** unità di lavoro; preparare una PR verso `decomp`, senza merge automatico.
- [ ] Non modificare senza autorizzazione specifica: estrattori/decoder originali, `rom_data.rs`, lettori runtime, mappa delle sorgenti, manifest, revisione ROMDATA, algoritmi guest, ABI, salvataggi, codice di ricompilazione nativa e test di equivalenza già accettati.
- [ ] Non modificare `TASK_HANDOFF.md` o le checklist condivise gestite dal lavoro locale; aggiornare soltanto la documentazione strettamente relativa alla PR e questa checklist quando il proprietario del progetto la integra.
- [ ] Non caricare né committare ROM, BIOS, salvataggi, registrazioni, dump, dataset estratti, asset originali, cataloghi cheat privati o materiale di terzi non autorizzato.
- [ ] Usare **fixture sintetiche** e riproducibili, con dati fittizi. Non richiedere che la ROM sia resa pubblica per sviluppare o testare uno strumento.
- [ ] Non dichiarare «verificato sul gioco» ciò che è stato soltanto testato con dati sintetici; distinguere compilazione, test unitari, test con dataset reale e parità runtime.
- [ ] Eseguire soltanto test pertinenti all'unità, senza campagne di performance o verifiche ROM/Wine già concluse dal Codex locale.
- [ ] Evitare operazioni che cancellino dati dell'utente o riscrivano in modo implicito file esistenti. Output privati, separati e ignorati da Git.

## P0 — Prima attività consigliata: esploratore e wiki offline ROMDATA

**Scopo:** trasformare un dataset già estratto in una documentazione consultabile, senza interferire con l'estrazione. **Questa è la prima attività da proporre a Codex Web.**

### CW-01 — Analisi e contratto del generatore

- [ ] Ispezionare il manifest effettivo, gli schemi JSON/TOML e le identità/versioni documentate nella branch attuale.
- [ ] Scegliere **una o due** famiglie con formato e relazioni sufficientemente verificati (per esempio oggetti e collegamenti), motivando la scelta; non partire necessariamente dagli ultimi record ancora opachi.
- [ ] Definire una CLI indipendente e un modulo collocato in `tools/` o posizione equivalente già coerente con il repository.
- [ ] Stabilire input, output, limiti dimensionali, comportamento su dati mancanti e modalità offline; non duplicare la logica di estrazione della ROM.
- [ ] Scrivere un breve contratto: quali campi sono semanticamente verificati, quali sono meri identificativi e quali restano opachi.

**Chiusura:** documento tecnico breve con i file coinvolti, esempio sintetico, schema supportato e comando previsto; nessuna modifica al core.

### CW-02 — MVP wiki statica

- [ ] Leggere e validare un dataset locale già estratto, senza accesso alla ROM.
- [ ] Generare una homepage HTML statica e un indice delle famiglie effettivamente presenti.
- [ ] Generare schede consultabili per le famiglie selezionate con ID stabili, campi documentati e riferimenti incrociati **solo quando realmente dimostrati**.
- [ ] Aggiungere ricerca/filtro locale, collegamenti navigabili e un'indicazione visibile dei campi sconosciuti o incompleti.
- [ ] Riportare provenienza, versione e hash del dataset, distinguendoli dalla versione del generatore.
- [ ] Escape di ogni stringa inserita nell'HTML; proteggere da path traversal, link non sicuri, JSON malformato e file troppo grandi.
- [ ] Nessun caricamento online, telemetria, CDN o dipendenza remota necessaria per consultare l'output.
- [ ] Preparare test con fixture sintetiche, inclusi casi di errore e relazioni mancanti.
- [ ] Documentare un comando d'uso su Windows e Linux, output previsto e limiti.

**Chiusura:** sito HTML generato localmente dalle fixture, pagine navigabili, test superati, nessun asset originale nel commit. Il test con dati reali **resta da fare in locale**.

### CW-03 — Estensione futura della wiki (non automatica)

- [ ] Aggiungere progressivamente missioni, unità, incontri, lavori, abilità, mappe e script **solo quando i relativi schemi e collegamenti sono comprovati**.
- [ ] Distinguere `identificato`, `integrato nel runtime`, `osservato in un test mirato` e `osservato in gameplay ordinario`.
- [ ] Collegare eventuali evidenze e source offset senza inventare significati dei campi.
- [ ] Supportare modifiche di schema e dataset incompleti con messaggi espliciti, senza correggerli silenziosamente.
- [ ] Consentire esportazioni tabellari (CSV/JSON) senza alterare il dataset sorgente.

**Chiusura:** una PR per ciascuna estensione; test per nuove famiglie e retrocompatibilità verificata su fixture precedenti.

## P1 — Esportazione degli asset (strumenti separati)

### CW-04 — Convertitore grafico offline

- [ ] Studiare i decoder effettivamente presenti (`obj_graphics`, `bg_graphics`, palette, font, animazioni) e documentare ciò che è già ricostruibile senza indovinare layout o trasparenza.
- [ ] Creare conversioni pure, verificabili su tile/palette **sintetiche**, per i formati supportati e realmente dimostrati.
- [ ] Generare PNG e metadati di provenienza soltanto quando la combinazione tile + palette + layout è definita con certezza.
- [ ] Preservare gli indici palette, i colori e i riferimenti originali in un file metadati; non sostituire i dati ROMDATA con PNG.
- [ ] Gestire errori, indici fuori intervallo, dimensioni anomale e trasparenza non dimostrata senza assumere valori arbitrari.
- [ ] Tenere spritesheet e ricostruzione delle animazioni come estensioni successive, con frame/timing verificati separatamente.

**Chiusura:** fixture grafiche sintetiche convertite con pixel esatti e test automatici; nessuna affermazione di copertura completa degli sprite del gioco.

### CW-05 — Convertitore audio offline

- [ ] Studiare i campioni PCM e i relativi parametri realmente decodificati in `audio_tones`.
- [ ] Esportare WAV da campioni sintetici con frequenza, signedness, lunghezza e canali esplicitamente definiti.
- [ ] Preservare in metadati gli eventuali loop e riferimenti a strumenti/toni.
- [ ] Non chiamare «brano completo» un campione isolato; il rendering delle sequenze M4A richiede un lavoro e una validazione specifici.
- [ ] Verificare forma d'onda, intestazioni WAV e dimensioni con test deterministici.

**Chiusura:** un convertitore WAV per il sottoinsieme dimostrato, documentato e testato su fixture sintetiche.

## P1 — Fondamenta del catalogo cheat, senza modifiche live alla memoria

### CW-06 — Catalogo e modello dei cheat

- [ ] Esaminare soltanto l'interfaccia pubblica e i contratti disponibili; **non** importare nel cloud il catalogo locale privato di codici raccolti da terzi.
- [ ] Definire un modello versionato di voci cheat con nome, descrizione, regione, formato, provenienza, requisiti e stato di verifica.
- [ ] Distinguere codici di semplice scrittura, codici condizionali, formati specifici dei dispositivi e master code; non trattarli tutti come indirizzo/valore.
- [ ] Realizzare ricerca, filtri, import/export e parsing limitato con codici completamente sintetici.
- [ ] Rifiutare codici non supportati o ambigui; niente esecuzione di espressioni arbitrarie o script forniti dal catalogo.
- [ ] Produrre un prototipo di interfaccia o CLI riutilizzabile, **senza** abilitare scritture RAM/ROM nel runtime.
- [ ] Documentare il contratto futuro con sessioni isolate, replay, audit e provenienza delle modifiche.

**Chiusura:** catalogo e parser con test positivi/negativi, nessuna modifica allo stato guest. L'eventuale motore di cheat e il menu in-game richiedono successiva integrazione **locale**, con test di parità interprete/native e log dei write deterministici.

## P2 — Report e qualità del progetto

### CW-07 — Report diagnostici unificati

- [ ] Verificare prima ciò che esiste già in `docs/report_home.md`, negli strumenti DBG e nel relativo HTML/generatore; **non creare un secondo indice duplicato**.
- [ ] Individuare una sola lacuna circoscritta (ad es. esportazione di un riepilogo leggibile, filtro di identità, collegamento a report esistenti).
- [ ] Estendere l'implementazione esistente soltanto se l'intervento è indipendente dal runtime e provabile con report JSON sintetici.
- [ ] Mostrare chiaramente `pass`, `divergente`, `non eseguito`, `parziale`, `troncato` e `incompatibile` senza promuovere automaticamente una prova.
- [ ] Mantenere privati i percorsi locali e gli allegati di gioco; nessun caricamento automatico.

**Chiusura:** una piccola PR con test di parsing/HTML e output offline verificato.

### CW-08 — Test di robustezza e integrazione continua

- [ ] Identificare una lacuna effettiva nella copertura di test del nuovo tool, non ripetere test ROMDATA già accettati.
- [ ] Aggiungere fixture sintetiche di input valido, troncato, incoerente, ostile e versioni incompatibili.
- [ ] Creare comandi automatici riproducibili che non richiedano ROM, Wine, BIOS, dati privati né servizi esterni.
- [ ] Se utile, aggiungere CI limitata ai nuovi tool, con permessi minimi e senza pubblicare artefatti di gioco.
- [ ] Documentare i limiti delle verifiche cloud rispetto ai test reali locali.

**Chiusura:** esecuzione cloud riproducibile, test verdi e log non contenenti dati di gioco.

## Flusso operativo obbligatorio per ogni incarico

1. [ ] **Rilettura:** aggiornare la branch di base, leggere i contratti e controllare che nessun'altra modifica abbia già completato l'incarico.
2. [ ] **Piano breve:** proporre soluzione minima, file toccati, dipendenze e test; se l'unità richiede di modificare ROMDATA/core/ABI, fermarsi e segnalarlo invece di farlo implicitamente.
3. [ ] **Implementazione circoscritta:** lavorare su una branch dedicata con modifiche minime, senza riformattazioni di massa.
4. [ ] **Verifica:** eseguire unit test e casi negativi con fixture sintetiche; controllare diff, file non previsti e artefatti generati.
5. [ ] **Documentazione:** scrivere comando d'uso, esempi fittizi, limiti e attività riservate al test locale.
6. [ ] **Consegna:** creare una PR verso `decomp` con riepilogo, file cambiati, risultati test, rischi, verifiche non eseguite e istruzioni per testare su un dataset reale **senza committarlo**.
7. [ ] **Integrazione locale:** dopo revisione, testare sul computer con i dati privati; fare merge solo quando compatibile con l'estrazione in corso.

## Ordine di esecuzione consigliato

1. **CW-01 → CW-02:** primo esploratore/wiki offline ROMDATA. *(Avviare da qui.)*
2. **CW-04 oppure CW-05:** esportatore grafico o audio isolato; scegliere quello con contratto più completo.
3. **CW-06:** modello e catalogo cheat, senza manipolazione runtime nel cloud.
4. **CW-07:** solo una lacuna reale dei report esistenti.
5. **CW-03/CW-08:** estensioni wiki e test/CI per i tool già nati.

**Non avviare più incarichi concorrenti sugli stessi file.** Non sostituire il lavoro locale di estrazione con una nuova analisi ROM non supportata da test reali.

## Prima richiesta da incollare in Codex Web

> Leggi `CODEX_WEB_TODO.md` e `AGENTS.md` nella branch `decomp` di `DragonautMk/FFTARecomp`. Esegui **solo CW-01**, il contratto e l'architettura minima dell'esploratore/wiki ROMDATA; analizza gli schemi reali prima di scegliere le prime famiglie. Non toccare gli estrattori, il runtime o le checklist del Codex locale. Usa esclusivamente dati sintetici e prepara una pull request separata con il piano e le verifiche possibili nell'ambiente cloud. Non procedere autonomamente alle unità successive.
