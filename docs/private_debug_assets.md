# Esportazione privata degli asset di debug

Lo script prepara uno ZIP **locale e privato** con una cartella portable Windows
separata. Contiene copie di salvataggi, snapshot e registrazioni: non è il
pacchetto di distribuzione e non va pubblicato o aggiunto a Git. La ROM e il BIOS
non sono inclusi. Occorre usare la propria ROM EUR compatibile al primo avvio.

Dal checkout, con Python 3 disponibile:

```bash
bash scripts/linux/export_debug_assets.sh
```

```powershell
.\scripts\windows\export_debug_assets.ps1
```

Oppure `python3 scripts/export_debug_assets.py` (su Windows anche `py -3`).
Lo script stampa il nome dello ZIP sotto `dist/debug-assets/`, con data e ora.
Estraendolo nella root del progetto si ottiene `FFTARecomp-debug-EUR/`; avviare
`FFTARecomp.exe` da quella cartella e selezionare la propria ROM EUR. Se esiste
una cartella con lo stesso nome, estrarre in una posizione nuova per conservare
la copia precedente. La cartella estratta è ignorata da Git nella root e
contiene anche un `.gitignore` protettivo. Il gioco scrive nella propria cartella portable;
Debug e Extended saves sono già abilitati nella configurazione iniziale.

Lo ZIP include:

- Il pacchetto Windows debug-tools selezionato, con launcher e runtime Windows
  selezionati separatamente. `PACKAGE-MANIFEST.json` descrive il solo pacchetto
  base; `DEBUG-ASSETS-MANIFEST.json` elenca tutti i file dell'esportazione.
- La raccolta `debug_saves/ready/` sotto `Data/config/debug-catalog/`, con
  catalogo, salvataggi e snapshot EUR/USA. Le voci USA sono conservate come
  raccolta preesistente; questa esportazione non ne verifica la giocabilità.
- Copie di lavoro degli slot EUR e di `ffta.sav` sotto `Data/saves/`, così
  **Gioco > Salvataggi** le trova direttamente. Il catalogo conserva copie
  distinte da selezionare sotto **Debug > Sessioni e casi > Debug catalog**.
- I restanti file di `debug_saves/`, la raccolta `saves/debug-cases/`, le fonti
  indicate dal manifest locale degli snapshot quando disponibile, le
  registrazioni complete della prova dedicata e la coppia di replay F-09 quando
  disponibile, in `DebugAssets/` o `Data/config/recordings/` come indicato da
  `LEGGIMI-DEBUG.txt` nello ZIP.

Queste sorgenti sono selezionate esplicitamente: lo script non copia l'intero
`target/`, dataset ROM, cache, prefissi Wine, sessioni incomplete o file
personali non presenti nelle raccolte indicate. Per un'altra raccolta usare
`--assets`, `--legacy`, `--source-manifest`, `--recordings` e `--replay`;
`--help` mostra i valori predefiniti. I file di una registrazione completa e
le voci del catalogo vengono confrontati con i rispettivi SHA-256. Snapshot e
registrazioni conservano i vincoli originali su ROM, BIOS, runtime e provenienza;
la presenza nel pacchetto non sostituisce le verifiche di compatibilità.

Gli input predefiniti necessari sono `dist/debug-tools/FFTARecomp-windows-debug-tools.zip`,
`target/x86_64-pc-windows-gnu/release/recomp.exe` e `ffta-play.exe` nella stessa
cartella. Costruire prima un pacchetto debug-tools e i binari Windows, oppure
selezionare input esistenti con `--base`, `--runtime` e `--launcher`. Lo script
controlla architettura PE, manifest e hash del pacchetto base, collegamenti
simbolici, percorsi, ROM/BIOS e CRC/hash dello ZIP prima di pubblicarlo. Una
posizione esplicita si passa con `--output`; un file già esistente non viene
sovrascritto. Non vengono eseguite build, emulazione o operazioni sui sorgenti.

La verifica locale del 31 ottobre 2026 ha controllato 104 file del catalogo,
10 fonti originali, due registrazioni e il replay F-09. Su Wine il runtime
Windows estratto ha elencato il catalogo EUR e creato una copia isolata DS-20;
la GUI della copia ha aperto il catalogo in pausa, con stato e clock invariati.
La copia finale aggiunge le fonti verificate e aggiorna istruzioni e protezione Git rispetto
allo ZIP usato nella GUI; runtime, configurazione, catalogo e salvataggi sono
identici. Linux nativo, Windows nativo, pad fisici e giocabilità USA restano
da verificare. Evidenze locali: `target/private-debug-assets-validation/`.
