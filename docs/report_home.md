# Homepage dei report locali

Aprire `index.html` nella root del progetto: riunisce tutti i file `index.html`
e `index.htm` trovati nella copia locale, comprese le fotografie storiche e i
report privati sotto `target/`. Ricerca per nome/titolo/percorso e filtri separano
coverage ROM, ispettore grafico, menu/testo/input, confronti e altri indici. I collegamenti
funzionano anche con JavaScript disattivato; ricerca e filtri lo richiedono.

I tre accessi rapidi aprono la coverage nel percorso principale e i file grafico/
confronto modificati più recentemente. La data mostrata è il **mtime del file**,
non la data di una nuova validazione. Il gruppo viene riconosciuto dai nomi dei
JSON affiancati; il catalogo non verifica né modifica i risultati dei report.
I link alla documentazione aprono i Markdown locali nel browser o nell'editor
configurato, senza convertirli in un sito.

## Aggiornare l'elenco

La pagina è statica, offline e senza server. Il browser non può enumerare da
solo la cartella del progetto: dopo aver generato/spostato/eliminato report,
rigenerare l'indice e ricaricare la pagina.

Su Windows, da PowerShell, con Docker Desktop e l'immagine già usata dalla
[coverage](development/ROM_COVERAGE.md):

```powershell
.\scripts\windows\report_home.ps1 -OpenReport
```

Su Linux:

```bash
bash scripts/linux/report_home.sh
```

Entrambi riusano `ffta-rom-coverage:local`, disabilitano la rete e montano il
progetto in sola lettura nel container. Il processo host pubblica soltanto
`index.html`. Non compilano runtime, non estraggono dati, non rigenerano la mappa
e non eseguono il gioco. L'immagine deve già esistere: non viene scaricata o
costruita automaticamente. PowerShell accetta `-Image NOME` per un'altra immagine
locale compatibile con Python 3.9+. I percorsi partono dalla root del progetto,
anche se il launcher viene eseguito da un'altra directory.

Se Python 3.9+ è già disponibile, senza Docker:

```bash
python3 scripts/report_home.py
```

Il generatore usa solo la libreria standard. `--stdout` restituisce l'HTML per
un mount readonly; `--root CARTELLA` seleziona un'altra root. La pubblicazione
prepara un file temporaneo e sostituisce l'indice solo dopo la generazione
riuscita, conservando la pagina precedente in caso di errore.

## Ambito e conservazione

La scansione non segue symlink e salta soltanto directory di servizio note:
`.git`, `.hg`, `.svn`, `node_modules`, `__pycache__`, `.gradle`, `.cxx`. Include
anche gli output di vecchie verifiche: la loro presenza non significa che siano
attuali o accettati. Limiti: un milione di entry, cinquemila indici, primi 64 KiB
per il titolo di ogni HTML. Il superamento di un limite interrompe la generazione
senza pubblicare un catalogo parziale. ROM, salvataggi e JSON dei report non
vengono aperti: si leggono titolo HTML, nomi affiancati e mtime.

I collegamenti sono relativi e codificano spazi, caratteri Unicode, `#` e `%`.
Spostare l'intera cartella del progetto conserva la navigazione; per spostare
singoli report occorre rigenerare la pagina. I titoli/percorso vengono trattati
come testo e non eseguiti come HTML. Non ci sono font, script o servizi esterni.

`/index.html` è ignorato da Git: il catalogo contiene nomi e percorsi privati.
Template, generatore e launcher restano versionati. In un checkout nuovo basta
eseguire uno dei comandi per creare la pagina. Ogni aggiornamento sostituisce
un solo file; non produce copie di ROM, snapshot, immagini o report.

## Verifica del 28 ottobre 2026

Generazione reale con l'immagine Docker coverage esistente, checkout readonly e
rete disabilitata. Tutti i **23 indici** corrispondono all'inventario indipendente
`rg`: 8 coverage, 5 grafici, 10 confronti. Verificati **35 collegamenti locali**;
tutti i 23 HTML preesistenti mantengono SHA256 e mtime.

Chrome headless desktop e layout stretto verificano conteggi, tre accessi rapidi,
filtri, ricerca senza distinzione maiuscole/minuscole, nessun risultato/reset,
focus, date e assenza di overflow. Screenshot ispezionati. Fixture temporanee
verificano nomi Unicode/caratteri riservati, titoli HTML, token letterali,
esclusione symlink e limite di scansione senza alterare la pagina precedente.
I due primi tentativi del launcher nel sandbox non raggiungevano Docker;
l'esecuzione autorizzata passa, senza build o estrazioni.

Il launcher PowerShell non è stato eseguito: questo host Linux non dispone di
PowerShell. Windows nativo e altri browser restano da verificare. Evidenze locali
sotto `target/report-home-validation/`.
