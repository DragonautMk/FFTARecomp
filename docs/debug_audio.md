# Diagnostica audio circoscritta — DBG-14

La raccolta è esplicita e disponibile nelle [copie isolate](debug_sessions.md).
Separa i buffer generati dal guest, la decisione del producer frontend e il
buffer scritto dal callback dopo DSP, gain e conversione al formato del backend.
Quest'ultimo è ciò che l'applicazione consegna al callback del backend: buffering
del dispositivo, XRUN hardware, altoparlanti e ascolto fisico restano sconosciuti.
Non avvia benchmark, traduzioni native o nuove prove di correttezza ROM/DS.

## Dal menu

1. Aprire una copia di debug con il runtime aggiornato e metterla in pausa.
2. **Debug > Start audio capture** arma 60 frame, con mixed, PSG, HLE e i due
   FIFO. La raccolta non riavvolge la macchina e non salva snapshot.
3. Riprodurre il breve problema riprendendo il gioco oppure avanzando per frame.
4. Tornare in pausa e scegliere **Debug > Save audio report**.
5. Aprire `Data/config/audio/<run>/index.html` nella copia. Ogni stream offre
   inviluppo, metadati, WAV locale e dati grezzi; il JSON è `audio.json`.

La pausa e i passi controllati normalmente scartano l'audio frontend. I buffer
guest possono quindi essere non nulli mentre il callback è silenzioso. Il report
indica `discard-paused`, `discard-no-stream`, `discard-fast-silent`, `normal-queue`
o `rings` per ogni frame osservato; non presume che un buffer generato sia stato
accodato o ascoltato. Le dimensioni delle code nei blocchi callback sono quelle
osservate dopo il loro consumo, non copie dei campioni in coda.

## Stessa operazione da CLI

Aprire la copia in pausa, quindi usare un altro terminale:

```powershell
.\recomp.exe debug-session open 'target\caso-audio' --paused
```

```powershell
.\recomp.exe debug-control audio-start 'target\caso-audio' --frames 4 --streams mixed,hle
.\recomp.exe debug-control step 'target\caso-audio' 1 --keys 3fe
.\recomp.exe debug-control step 'target\caso-audio' 3 --keys 3ff
.\recomp.exe debug-control audio-report 'target\caso-audio'
```

`--frames` accetta **1–180**; `--streams` accetta una selezione senza duplicati
di `mixed,psg,hle,fifo0,fifo1`. Menu e CLI condividono lo stesso collector e
writer. Lo stato restituisce `audio_diagnostic` e il percorso `audio_report`.
Per catturare input riproducibili, avviare prima una
[registrazione da snapshot](debug_recordings.md), fermarla esplicitamente e
chiudere la copia prima di verificarla. Le chiavi active-low osservate sono anche
riportate per frame. Una raccolta audio da sola non certifica un replay.

Start e Save richiedono una copia in pausa, senza budget di passi/probe o replay
attivo. Non si sostituisce una raccolta non salvata e non si avvia un replay
separato finché esiste. Load/Undo/Restart congelano il prefisso audio prima del
normale reset; il report può identificare uno stato corrente successivo al
prefisso. Un errore di pubblicazione conserva la raccolta congelata per un altro
tentativo. Non si modificano i file di report già pubblicati.

## Confrontare buffer già conservati

Il comando offline legge snapshot esistenti compatibili con la stessa ROM/BIOS
e lo stesso frame guest. Non esegue frame, non carica dataset e non compila
codice native. Clock diversi restano visibili.

```powershell
.\recomp.exe debug-audio states --rom '.\ffta.gba' --left '.\sinistra.ffstate' --right '.\destra.ffstate' --dir '.\confronto-audio-nuovo' --streams mixed,psg,hle,fifo0,fifo1 --start 81800 --count 256
```

Per snapshot con BIOS reale aggiungere `--bios '.\gba_bios.bin'`. La directory
di output deve essere nuova. Start e count sono indici di valori interleaved
oppure eventi FIFO; devono essere pari e count è **2–262.144**. Il confronto
conserva byte esatti, differenze di lunghezza e le prime 16 coppie diverse, con
indici assoluti e decodifica. Non allinea ritardi, canali o fasi fra acquisizioni.

L'esito JSON e il codice di uscita distinguono **0: equal**, **2: diverged**,
**3: incompatible**. Equal riguarda esclusivamente gli stream e la finestra
selezionati, compresi eventuali buffer vuoti; non certifica lo stato completo o
la qualità percepita. Anche in caso di divergenza il report viene pubblicato.
Input incompatibili producono un risultato JSON senza sovrascrivere output.

Il browser permette di scegliere lo stream e andare alla prima differenza
nelle preview a campioni. Per i FIFO mostra gli eventi esatti nel dettaglio;
non converte un indice evento in un istante dedotto.

## Formati, limiti e identità

- Mixed e PSG: valori i16 little-endian, stereo a 65.536 Hz. HLE: bit IEEE f32
  little-endian nello stesso ordine. FIFO: campione i8 seguito dal periodo u32
  little-endian, cinque byte per evento. I file `.bin` conservano i valori esatti.
- Le WAV sono anteprime PCM16 con clipping e sostituzione dei non finiti con
  zero. La preview FIFO usa sample-hold isolato a 65.536 Hz con `period/256`;
  filtro, routing e fase iniziale restano sconosciuti. Ogni preview dura al più
  tre secondi; questo limite non modifica il dump grezzo selezionato.
- Cattura live: 30 secondi host, al più 180 frame guest e **2 MiB per stream**.
  Callback: tre secondi oppure 524.288 valori normalizzati f32, al più 512
  blocchi e 64 reset. Omissioni, troncature e motivi di stop sono espliciti.
  Il limite guest si applica ai frame successivi all'origine; eventuali buffer
  già pendenti all'avvio sono dichiarati e possono comparire nel primo handoff.
- Il report conserva frame/clock/stato esatto, ROM/runtime, BIOS effettivo,
  dataset effettivo se presente, manifest della sessione con hash dei seed e
  input immutabili, build, configurazione input/AV e chiavi osservate. Un percorso
  di registrazione in corso non significa registrazione completa/verificata.
  Il confronto offline identifica runtime, ROM, BIOS e snapshot; configurazione
  frontend storica e input non presenti negli snapshot restano sconosciuti.
- Sono ammessi al più **otto report** per copia, compresi tentativi incompleti.
  JSON al più 2 MiB; nessuna cancellazione o sovrascrittura automatica. Le cartelle
  e tutti i dump restano privati e ignorati da Git. La
  [homepage locale](report_home.md) li trova dopo rigenerazione.

Nel callback si copiano solo valori e contatori in memoria preallocata sotto il
mutex già esistente. Non si leggono orologi, non si allocano buffer, non si
scrivono file/log e non si acquisiscono altri lock. WAV/JSON/HTML vengono scritti
dal thread principale dopo il rilascio del mutex. Il frame pubblicato è un
watermark del producer, non l'origine dimostrata di ciascun campione callback.
Mute utente/temporaneo, gain, modalità enhanced/HLE, Fast richiesto/stimato,
code residue, contatori e reset permettono di leggere il contesto del buffer.

## Validazione

I controlli Windows vengono eseguiti sotto Wine/Xvfb, in copie portable mute.
La divergenza conservata al frame 8514 e clock 2391464782 viene localizzata a
mixed 81918/81919: −704 contro −576, anche selezionando solo 256 valori attorno
al punto. Questo riusa snapshot precedenti e non ripete il replay o il benchmark.

La prova live registra A/rilascio per quattro frame, confronta lo stato completo
alla fine e cattura selezione CLI, default menu e Fast/reset. Il source peak è
0,16015625 mentre il callback con mute è zero; è una distinzione fra generazione
e buffer applicativo, senza certificare un listener hardware. Rifiuti di doppio
Start, parametri invalidi, otto cartelle, sessione attiva/non in pausa/chiusa
conservano i report. I test confrontano bit per bit uscita/code/contatori con e
senza observer nei formati f32/i16/u16 e nelle transizioni audio esistenti.

I risultati dettagliati e i limiti effettivi sono nel
[record DBG-14](changes/2026-10-29-add-bounded-audio-diagnostics.json).
Windows nativo, Linux nativo, controller fisici, altri dispositivi/scene e ascolto
fisico restano da verificare. Nessuna accettazione ROM/DS/native o prestazione
deriva dalla sola presenza del report.
