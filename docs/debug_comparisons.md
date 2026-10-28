# Primo punto di divergenza (DBG-09)

**Debug > Compare first difference** e `debug-compare create/run/cancel`
confrontano ogni confine di frame completato, partendo da snapshot e input
registrati. Il riferimento usa **interprete/raw**; il candidato può usare
interprete o una traduzione native esistente, con dataset typed facoltativo.
Il worker contiene due macchine indipendenti e le avanza in sequenza, secondo
il contratto già usato dalle verifiche ROM raw/typed. Le impostazioni di vista
sono comuni. Il gioco principale rimane in pausa.

Il candidato Windows isolato verificato è
`target/debug-tools-compare/recomp.exe`; il normale eseguibile non viene
sostituito. La funzione è disponibile anche senza `rom-data-read-audit`.
La validazione Windows descritta qui usa Wine/Xvfb.

## Preparare ed eseguire

Usare una [registrazione completa](debug_recordings.md) e la relativa
[sessione isolata chiusa](debug_sessions.md). La creazione controlla le stesse
identità di [DBG-05](debug_replays.md): runtime selezionato e runtime della
registrazione, ROM/BIOS, dataset, origine, input ed endpoint memorizzato.
La directory di destinazione deve essere nuova.

Esempio PowerShell, interprete/raw contro interprete/typed:

```powershell
.\recomp.exe debug-compare create `
  --recording 'target/sessione/Data/config/recordings/registrazione' `
  --session 'target/sessione' --dir 'target/confronto' --case DS-20-EUR `
  --dataset 'target/dataset-autenticato' --frames 100
.\recomp.exe debug-compare run 'target/confronto/replay.json'
```

Per confrontare interprete/raw con native/raw, sostituire `--dataset` con
`--library 'out/cache-esistente.dll'`. `--backend native` viene scelto
automaticamente quando è presente la libreria; è possibile dichiararlo
esplicitamente. La cache deve essere quella desiderata per la ROM, compatibile
con il runtime selezionato. Viene copiata, identificata e caricata tramite TinyCC;
non si compila una nuova traduzione. Senza libreria il candidato è interprete.
Un backend incoerente con la presenza della libreria viene rifiutato.

I parametri comuni `--runtime` e `--timeout-ms` restano quelli del replay.
`--frames` è compreso fra 1 e il minimo di lunghezza registrata e **3.600**;
il valore predefinito è tale minimo. Non si generano nuovi input oltre la
registrazione. Il dataset selezionato viene autenticato e copiato nel profilo,
attivo sul candidato. Il riferimento ricrea sempre la vista raw dallo stesso
stato; ROM/BIOS e dati guest iniziali restano identici.

Nel menu selezionare il `replay.json` con **Choose replay profile**, quindi
**Compare first difference**. Un profilo ordinario o di cattura viene rifiutato
da questa azione. **Cancel replay** e la chiusura della finestra proprietaria
interrompono anche il confronto. Il comando equivalente è:

```powershell
.\recomp.exe debug-compare cancel 'target/confronto/replay.json'
```

La supervisione conserva lock esclusivi, identificazione del run, deadline,
progress/result JSON e gestione dell'interruzione. Il worker non dispone dei
writer di SRAM, slot o backup personali. Non si possono combinare nella stessa
esecuzione confronto, inventario audit e trigger di cattura.

## Cosa viene confrontato

L'origine viene controllata prima di iniziare. A ogni confine allineato per
**numero di frame** si confrontano:

- stato completo serializzato CPU/Bus, con identità ROM/BIOS;
- clock guest, contatore frame e `frame_ready`;
- CPU completa, framebuffer centrale RGB555, intera allocazione framebuffer
  wide e OAM;
- mixed e PSG i16, HLE per bit f32, eventi FIFO A/B come sample e periodo.

L'osservazione legge strutture e buffer; non esegue letture I/O diagnostiche,
non avanza il clock e non cambia lo stato per far coincidere i risultati.
Le code audio frontend vengono osservate prima dello svuotamento e drenate
simmetricamente prima del frame successivo, inclusa l'origine. Si conserva
solo l'ultimo punto uguale in memoria, senza una timeline di dump.

`--audio preserve` mantiene la configurazione della sorgente. Il report
registra presenza HLE, tap attivi e conteggi effettivi per ogni stream.
`--audio arm-hle` abilita esplicitamente tap e shadow HLE in modo simmetrico
prima della finestra: la configurazione aggiunta è dichiarata nel report.
Conteggi zero o percorsi spenti non dimostrano equivalenza audio di tali percorsi.
I conteggi sono **valori/eventi**, distinti dai byte dei file.

I frame hanno la stessa origine e lo stesso indice; i backend possono rientrare
a granularità diversa, per istruzione o blocco native. Clock e PC osservati
restano quindi nel confronto, senza sincronizzazioni artificiali. Il primo
frame diverso localizza un confine, non l'istruzione responsabile né da solo
la causa o la correttezza di un backend.

## Risultato e report

Il profilo conserva `comparison` e `stop: "first-divergence"`. Il JSON del
supervisore continua in `Data/config/replay/`, con `comparison_report` relativo
alla cartella del profilo:

- **0, `matched`:** tutti i componenti dichiarati coincidono nel prefisso scelto;
- **2, `diverged`:** primo confine diverso, con indice e componenti;
- **3:** input/profilo incompatibile, **4:** timeout, **5:** interrotto,
  **6:** errore del worker.

Il menu mostra **DIFF FRAME N** oppure **MATCHED N/N**. Un passo incompleto
per superamento della guardia non viene presentato come primo frame diverso.
Il confronto non certifica l'endpoint completo della registrazione: tale
verifica resta l'operazione ordinaria di DBG-05. Un prefisso uguale non promuove
parità globale, risorse, proprietà dei dati o stato dei casi DS.

Aprire `comparisons/<run>/index.html` nel browser. Funziona offline e include
`comparison.json`, identità effettive, impostazioni, ambito, contatori e hash.
Quando emerge una differenza si conservano soltanto in quel punto:

- snapshot `left.ffstate`, `right.ffstate` e ultimo `previous.ffstate` uguale,
  quando esiste;
- PNG affiancate centrale/wide, con scelta della vista e precedente espandibile;
- file dei cinque stream non vuoti: i16 little endian interleaved stereo per
  mixed/PSG, f32 IEEE little endian per HLE, record FIFO di cinque byte
  (`i8 sample`, `u32le periodo`);
- campi CPU/Bus diversi, registri general purpose, differenze nei vettori
  EWRAM/IWRAM/OAM, pixel e stream, con al massimo 16 esempi per voce.

Le PNG rappresentano i buffer PPU attivi. Overlay e composizione frontend
restano fuori dal confronto; i pixel padding della memoria wide restano inclusi
nei confronti di stato e del buffer completo. Le immagini possono coincidere
quando differiscono solo CPU, clock, metadati pixel o audio.

I nomi dei campi vengono dal **Serialize effettivo dello schema corrente**.
I valori sono codificati con postcard come nei salvataggi, inclusi i membri
privati e i bit float. Gli offset mostrati in questa sezione appartengono al
singolo valore codificato: non sono offset guest o del file snapshot completo.
I registri e gli indici dei vettori dichiarati sono esposti separatamente.
Non si tenta di indovinare la struttura dai byte varint del salvataggio.

Per un confronto uguale o interrotto si pubblicano solo HTML/JSON, senza
immagini, snapshot o stream. Una divergenza iniziale può non avere un punto
precedente. Gli artefatti restano diagnostici, senza catalogo o promozione DS;
conservare il profilo con ROM/BIOS, input e dataset necessari a riprodurli.
Al massimo **otto report per profilo**: il nono avvio viene rifiutato prima
del worker. I report vecchi non vengono sovrascritti; archiviare esplicitamente
o creare un nuovo profilo. I payload sono privati e restano ignorati da Git,
fuori da commit e pacchetti. Il risultato più recente va associato al proprio
run, senza scambiarlo con vecchi report conservati.

## Verifiche effettuate e limiti

La build isolata usa il commit pubblico `7bb07bf` (**e80-d79**) più dieci file
selezionati; le modifiche concorrenti di estrazione restano escluse. Feature e
default compilano senza warning. Passano **67 test Wine**: otto confronto, due
serializer per campi, dodici replay e 45 menu. Un fault sintetico al secondo
frame verifica campo, registro, memoria, immagini, ultimo punto uguale e
riconoscimento esatto del frame; i controlli coprono bit NaN, conteggio audio,
limiti, frame incompleto, annullamento e pubblicazione senza sovrascrittura.
L'input sintetico iniziale mancava dell'header `gba-input v1`: il validatore lo
ha rifiutato correttamente. È stata corretta la sola fixture, mantenendo i log.

**20 controlli CLI**, un processo controllato di annullamento e tre verifiche
default/feature passano. Nei due frame DS-20 selezionati, raw/raw, raw/typed e
setup audio simmetrico coincidono. Il default senza hook audit produce gli
stessi hash del candidato con feature. La diagnosi della cache native già
nota come divergente si ferma al **primo frame**, senza rieseguire il vecchio
confronto di endpoint da 102 frame:

- frame guest 22.220 da origine 22.219;
- clock interprete 6.241.425.344, native 6.241.425.348, delta **4 cicli**;
- differiscono soltanto i campi serializzati `bus.clock` e `cpu.regs`;
- `r15`: `0x0800045c` / `0x0800044c`;
- framebuffer centrale/wide, OAM e i cinque stream osservati coincidono:
  2.194 valori mixed/PSG/HLE e 264 eventi per FIFO da ciascun lato.

Sono risultati osservati al confine, senza riparazione della cache o nuova
accettazione della sua parità. Il confronto typed usa il dataset e80 già
accettato; non è un censimento dei byte consumati per risorsa, per cui resta
[DBG-06](debug_rom_audit.md). La GUI Wine/Xvfb verifica lo stesso primo diverso,
prefisso uguale, annullamento e chiusura proprietario; stato/frame/clock e
progressi della finestra principale restano invariati. Il browser è verificato
a 1.400/620 px, incluse immagini wide e report uguale senza payload pesanti.
Tutti i **276 input originali selezionati**, compresi i due eseguibili normali,
conservano hash e data di modifica.

Il confronto usa un solo runtime con due macchine guest; il confronto fra due
versioni eseguibili distinte non è implementato. Windows nativo, Linux/Wayland,
controller fisici, altre ROM/dataset/scene e la ricerca a granularità di istruzione
restano da verificare o fuori ambito. Non si inferiscono nuovi benchmark,
accettazioni ROM/DS, correttezza completa native o sviluppo Godot. Evidenza locale
sotto `target/debug-tools-compare/`; la sua eventuale indisponibilità non riapre
le verifiche già chiuse.

Per ispezionare i buffer PPU e la composizione frontend di una copia ferma,
usare [l'ispettore DBG-12](debug_graphics.md): report offline con palette/tile/OAM,
finestre e pixel backing. Il confronto per frame fra backend descritto qui resta
separato dalla cattura readonly e dalle proiezioni di registri congelati.
