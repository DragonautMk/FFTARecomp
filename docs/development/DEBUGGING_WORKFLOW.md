# Debugging con Terra 6.1 e supporto Sol 6.1

Terra conduce il debugging, applica le modifiche e verifica il risultato.
Entrambi i modelli lavorano con thinking **Molto alto / Extra High (`xhigh`)**.
Quando non riesce a individuare la causa o una verifica utile, consulta Sol.
Sol restituisce una diagnosi con prove e una prossima azione; Terra riprende
il lavoro nella stessa conversazione.

Il comportamento è definito dalla skill
[ffta-debugging](../../.cursor/skills/ffta-debugging/SKILL.md) e richiamato da
[AGENTS.md](../../AGENTS.md). Vale per il debugging in questo repository;
restano applicabili le istruzioni di sviluppo, validazione e memoria.

## Avvio

Nell'app apri il progetto FFTARecomp, seleziona **GPT-6.1 Terra** come modello
principale e **Molto alto / Extra High** come livello di thinking. Poi chiedi:

> Usa ffta-debugging per capire e correggere questo bug: [sintomo e risultato atteso].

AGENTS richiede la skill anche per richieste di debugging che non la nominano.
Il caricamento automatico del catalogo dipende dal client; un agente che legge
AGENTS trova comunque il percorso locale da aprire.

Con una CLI compatibile, dalla cartella del progetto:

```sh
codex -m gpt-6.1-terra \
  -c 'model_reasoning_effort="xhigh"' \
  -c 'agents.default_subagent_model="gpt-6.1-sol"' \
  -c 'agents.default_subagent_reasoning_effort="xhigh"'
```

La selezione iniziale di Terra spetta al client. Il passaggio a Sol è una
consultazione tramite subagente; la conversazione principale resta su Terra.
Non serve cambiare manualmente modello a ogni consultazione.
Il comando imposta modello e thinking per quella sessione; la chiamata a Sol
nella skill richiede comunque `xhigh` esplicitamente. Il ritorno conserva il
thinking `xhigh` di Terra. Queste istruzioni non cambiano da sole le impostazioni
di una conversazione già aperta.

`xhigh` è supportato dalle schede ufficiali di
[Terra 6.1](https://developers.openai.com/api/docs/models/gpt-6.1-terra) e
[Sol 6.1](https://developers.openai.com/api/docs/models/gpt-6.1-sol).
I parametri di configurazione sono descritti nel
[riferimento Codex](https://learn.chatgpt.com/docs/config-file/config-reference).

La [documentazione ufficiale sui modelli](https://learn.chatgpt.com/docs/models)
indica Codex CLI 0.159.0 o successiva per Terra. La CLI locale ispezionata durante
la preparazione è 0.110.0: quel percorso richiede una versione compatibile.
Il workflow non aggiorna la CLI né modifica la configurazione globale.
L'uso nell'app richiede i due modelli e gli strumenti di delega disponibili
nella sessione. La disponibilità dei modelli dipende dall'account/client.

## Avvio del gioco

Su Windows, estrai `dist/docker-windows/FFTARecomp-windows-portable.zip` in una
cartella scrivibile e avvia `FFTARecomp.exe`. Alla prima apertura seleziona la
tua ROM FFTA USA/Europe; impostazioni e salvataggi restano nella cartella `Data`.

Su Linux, dal checkout puoi usare il runtime già compilato:

```sh
./target/release/recomp play "/percorso/FFTA.gba"
```

Sostituisci il percorso con quello della tua ROM. Per l'edizione portabile
Linux estratta, avvia invece `./FFTARecomp` dalla sua cartella. I binari locali
possono precedere gli ultimi sorgenti: indicare il comando non equivale a una
nuova build o validazione. Requisiti, compilazione e dettagli di avvio sono in
[README](../../README.md) e nella [guida portabile](../portable.md).

## Quando interviene Sol

Terra chiama Sol appena, dopo le prime letture, non sa formulare una causa
plausibile o scegliere una prova che distingua le ipotesi. Lo chiama anche se
due tentativi diagnostici mirati non riducono l'incertezza, oppure se le prove
contraddicono la correzione proposta. Non è necessario provare due patch.

Una compilazione ancora in corso o un permesso mancante richiedono la gestione
di quella dipendenza. Non costituiscono da soli un motivo per cambiare modello.

Terra invia a Sol obiettivo, errore, file coinvolti, diff corrente, prove già
raccolte, tentativi scartati e una domanda precisa. Sol legge i sorgenti e
risponde con causa/ipotesi, riferimenti verificabili, correzione minima o prova
discriminante e controllo da eseguire. Terra mantiene i file interessati stabili
durante la consultazione e resta responsabile di modifiche e validazione.

Il modello richiesto per il consulente è `gpt-6.1-sol`, con reasoning `xhigh`.
Il payload esplicito e il contratto sono nella skill. Gli override di modello
dei subagenti sono descritti nella
[documentazione ufficiale sui subagenti](https://learn.chatgpt.com/docs/agent-configuration/subagents).

## Ritorno a Terra e continuità

Quando Sol termina, Terra controlla le prove e riprende automaticamente.
Se serve un'ulteriore osservazione, raccoglie quella prima di applicare una
correzione. Una seconda consultazione deve contenere nuove evidenze.
Lo stesso pacchetto non viene ripetuto in ciclo.

Nel task handoff si annotano il task ID, il motivo della consultazione, il
consulente/modello, la conclusione, le piste scartate, la prossima azione di
Terra e i controlli realmente effettuati. Dopo compaction si riparte da quel
punto. I task chiusi e quelli in pausa conservano il loro stato.

## Se la delega non è disponibile

L'agente segnala il limite, compreso un thinking `xhigh` non supportato, e non
dichiara di aver applicato impostazioni non verificabili. Conserva nel
handoff il pacchetto e la prossima azione. Se entrambi i modelli sono accessibili
solo come modello principale, si può passare manualmente a Sol nel selettore
(oppure `/model` nella CLI), selezionando anche **Extra High**, chiedergli la
diagnosi usando quel pacchetto, poi tornare a **Terra + Extra High** per
applicarla. Se Sol non è accessibile, il limite resta
esplicito e Terra continua solo sulle attività sostenute dalle prove disponibili.

## Validazione e limiti

La skill supera la validazione strutturale. Una prova sintetica ha esercitato
la sequenza di delega e ritorno: riproduzione di tre azioni menu che eseguivano
tutte l'ultima azione, una consultazione diagnostica, correzione del principale
e controllo finale delle tre azioni superato. Il fixture resta separato dal gioco.
La prova precedente richiedeva Terra 6.1 con reasoning `medium` per il principale
e Sol 6.1 con `high` per il consulente; resta prova storica della delega e del
ritorno. Il successivo requisito `xhigh` per entrambi è stato aggiornato nella
skill e nei comandi di avvio, senza ripetere quella prova. L'API della prova
ha restituito gli handle senza metadati del
modello effettivo: la prova verifica l'instradamento richiesto e il ritorno,
senza una conferma indipendente dei modelli eseguiti.

Il dettaglio dei controlli, dei collegamenti e della memoria è nel
[record della modifica](../changes/2026-10-31-add-terra-sol-debugging-workflow.json).
La qualità della diagnosi su un bug reale dipende dalle evidenze del caso;
le istruzioni non sono un rilevatore automatico e deterministico di stallo.
