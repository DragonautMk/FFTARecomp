# Menu, testo e input — DBG-13

Nella [copia isolata](debug_sessions.md), mettere in pausa e usare **Debug >
Start UI / input trace**, poi riprodurre il caso e **Save UI / input report**.
La CLI condivide le stesse azioni:

```powershell
.\recomp.exe debug-control ui-start 'target\caso'
# Usare tastiera/gamepad o i passi DBG-10 per riprodurre il caso.
.\recomp.exe debug-control ui-report 'target\caso'
```

`status.ui_trace` indica raccolta attiva, conteggio, motivo di arresto e percorso
relativo dell'ultimo report. Aprire `Data/config/ui/<run>/index.html`. La pagina
funziona offline, con menu PNG, focus, metriche del testo e sequenza filtrabile.
Il report resta privato e compare nella [homepage locale](report_home.md) dopo
averne rigenerato l'elenco.

`debug-control status` espone anche `status.host_menu`: pagina, riga, binding e
dropdown correnti. Leggere questo focus prima di automatizzare una navigazione:
un controller connesso può aprire direttamente la scheda Gamepad. Un dropdown
aperto consuma le frecce fino alla scelta/chiusura; non stanno muovendo le righe
del pannello sottostante.

## Quale passaggio ha consumato il comando

La sequenza distingue:

- `keyboard`: pressione/rilascio ricevuti da Winit, codice fisico e autorepeat;
- `controller-poll`: token XInput campionati, prima del blocco di cattura;
- `mapped-poll`: tasti tenuti, maschera GBA attiva bassa, navigazione host,
  `rising_confirm`, cattura binding, shortcut e maschera di soppressione;
- `host-menu-dispatch`: focus prima/dopo, input e azione realmente restituita;
- `menu-consumed-by-remapping`, `menu-debounce-refused`, `shortcut-consumed`:
  motivo esplicito per cui quel comando non prosegue nello stesso percorso;
- `bindings-changed` / `pad-bindings-changed`: configurazione dopo la rimappatura;
- `guest-input-latch` e `guest-frame`: maschera consegnata al guest e frame
  effettivamente completato, con sorgente passo controllato/host e stato recorder;
- `control-request` e `host-action`: richieste CLI/shortcut e azioni frontend.

Ogni evento ha numero di sequenza, tempo relativo **host**, frame e clock **guest**.
I down-state duplicati vengono deduplicati per poll; le ripetizioni fisiche della
tastiera restano esplicite. L'assenza di un edge non è un secondo dispatch. La
maschera rimappata non è automaticamente input guest: il menu in pausa non esegue
frame, e un passo F7 usa la maschera neutra dichiarata da DBG-10.

La raccolta non cambia input o filtri esistenti. Le pressioni sono osservate
nei punti correnti, senza un nuovo canale di iniezione. Le conferme del mouse non
hanno una traccia raw dedicata; anche le transizioni raw Gilrs/DirectInput restano
fuori dagli hook, mentre il risultato rimappato aggregato è osservabile. Non si
deduce una latenza hardware dai tempi dei poll.

## Testo e rettangoli

Il report ridisegna il menu host su un buffer temporaneo, con lo stesso percorso
font/pixel esistente. Un observer thread-local raccoglie al massimo 512 chiamate:
testo (prefisso 256 caratteri), font/size, origine, larghezza massima, ascent/
descent, advance completo/prefisso e bbox outline dei glyph effettivamente scelti.
La misurazione si ferma a 4.096 caratteri e segnala il limite.

`width_stopped` identifica il `break` originale quando il prossimo advance supera
la larghezza disponibile. `canvas_clipped` indica outline fuori dal canvas:
sono controlli separati. La bbox è quella degli outline, non una misura dei pixel
alpha nonzero. La pagina sovrappone bbox e area disponibile al PNG; non cambia
font, wrapping, ellissi o geometria del menu. Il focus host riporta pagina, riga,
binding in cattura, panel e rettangolo selezionato con scroll corrente.
Quando è aperto un dropdown include opzione, indice, scroll e geometria ottenuta
dallo stesso helper usato per disegno e hit test.

Per la UI guest sono dichiarati solo selector già riconosciuti dagli helper
esistenti, flag dialogue/panel, bbox dei buffer e layout/clip frontend. Altri menu,
focus e metriche dei glyph guest restano **sconosciuti**. Queste misure non provano
la provenienza ROM, i byte typed consumati o il vincitore del framebuffer.

## Limiti, registrazione e conservazione

Raccolta esplicita, disattivata normalmente. Si ferma a **30 secondi, 1.024 eventi
o 600 frame**, conservando il motivo e il prefisso. Start/Save richiedono copia
in pausa senza budget di passi/ricerca/replay in corso. La raccolta può osservare
passi, gioco e una [registrazione DBG-04](debug_recordings.md), da verificare con
il relativo comando: la sola traccia host non equivale a un replay completo.

Un ripristino di timeline chiude il prefisso con `state-restored`; il report può
quindi mostrare lo stato corrente successivo al ripristino, senza attribuirlo
all'endpoint degli eventi precedenti. Dopo un arresto automatico salvare il
report prima di iniziare una nuova traccia. Gli eventi non vengono scritti
continuamente su disco.

Massimo **otto cartelle** per copia, comprese le catture fallite. Il report limita
il canvas a 1600×1000 e il JSON a 2 MiB. Stato completo controllato prima/dopo il
ridisegno; ROM/runtime e origine/corrente identificati. `ui.json` viene pubblicato
per ultimo; nessuna sovrascrittura, pruning o snapshot aggiuntivo dal collector.
Un errore conserva la traccia e le eventuali risorse per diagnosi. L'ultimo
percorso in status resta una cattura precedente se un nuovo comando fallisce:
leggere sempre esito e identità della richiesta.

## Risultati verificati il 28 ottobre 2026

Build Windows locked/offline con/senza audit, entrambe senza warning;
**69 test Wine distinti**: 2 collector, 7 controllo, 12 frontend, 47 menu iniziali
e un nuovo controllo dropdown. Dopo le revisioni sono stati ripetuti solo i
2 test report interessati (due volte): 73 esecuzioni totali. L'observer font
lascia identici tutti i pixel; rimuovendo i soli hook si recuperano gli stessi
corpi originali di mapping input, frame runner e canvas, più l'helper dropdown.

Tre casi rendono leggibili i passaggi:

- Tastiera con Enter rimappato ad A: 14 eventi autorepeat producono un solo
  `host-menu-dispatch`, che apre il dropdown Extended saves. I poll successivi
  mostrano confirm tenuto e `rising_confirm=false`. F7 consegna `3ff` neutro;
  i passi successivi consegnano `3fe/3ff` come richiesto. Questo prefisso è
  osservazione accettata; la sua prima registrazione, interrotta dal test, resta
  incompleta e non è stata certificata come replay.
- Copia separata con registrazione completata: i due frame CLI A/rilascio,
  esplicitamente fermati e seguiti da chiusura pulita, passano il verificatore
  completo di DBG-04. Endpoint SHA256
  `dd841ef6dba845974625e7e5ae5839f86069000066f876a32d0a86bf6fcb0b81`.
- XInput **simulato**, South tenuto 1.200ms nel binding A: un aggiornamento
  North→South, una sola conferma da tastiera per aprire la cattura, nessuna
  seconda conferma né frame guest. Focus/cattura/mapping e token raw distinguono
  il binding dalla navigazione. I rifiuti per traccia già presente, capacità,
  esecuzione attiva e copia chiusa passano.

Il report misura inoltre il titolo diagnostico lungo: **545,89px** di advance,
**472px** disponibili, **64/75 caratteri** scelti, `width_stopped=true` e
`canvas_clipped=false`. La causa è il limite della chiamata font. Il dropdown
Extended saves ha bbox `[266,202,320,76]` sul canvas 852×480; l'opzione corrente
e le frecce riguardano quel popup, prima del menu sottostante.

Sono conservati sei report. Chrome desktop 1400px/mobile 620px verifica focus,
selezione testo/bbox, immagini, filtro eventi e assenza di overflow; screenshot
ispezionati. La homepage aggiornata riconosce tutti i sei report UI fra 29 indici.
Tutti i 202 originali selezionati e i due runtime normali rimangono invariati.

I tentativi del test hanno rilevato anche errori nella sua navigazione: frecce
in un dropdown ancora aperto, verifica mentre la sessione aveva il lease,
Right dopo la selezione automatica Gamepad e riga Open menu scambiata per A.
Sono conservati con log/preimmagini; i prefissi accettati non sono stati rifatti.
Due copie terminano pulitamente; due vecchie copie diagnostiche furono interrotte
dalla pulizia del test e non hanno un'accettazione di uscita o replay. La prova
finale usa pagina, label e geometria restituite da `status`.

Windows/Linux nativi, controller fisici, altre scene/backend native, glyph guest
e transizioni raw Gilrs/DirectInput/mouse non sono certificati. Il PNG è un
ridisegno CPU diagnostico del menu su anteprima, non una cattura del monitor/GPU.
Nessuna nuova accettazione ROM/DS/native o misura di performance. Evidenze locali
sotto `target/debug-tools-ui/`; i profili Chrome temporanei sono stati rimossi,
conservando gli errori e gli input. [Record](changes/2026-10-28-add-bounded-menu-text-input-diagnostics.json).
