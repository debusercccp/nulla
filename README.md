# Nulla

Tre implementazioni dello stesso programma in linguaggi diversi. Ciascuno chiede all'utente un numero di secondi e dorme per quel tempo.

Questo repo non fa assolutamente nulla. LOL

## nothing.rs (Rust)

Chiede all'utente quante volte eseguire l'operazione (in secondi) e poi pausa il programma.

**Caratteristiche:**
- Usa `std::io` per input/output interattivo
- Utilizza `thread::sleep()` e `Duration` per la pausa
- Converte l'input da stringa a `u64` (intero senza segno a 64 bit)
- Esegue il flush di stdout per visualizzare il prompt prima di leggere l'input
- Crash immediato se l'input non è un numero valido

**Build ed esecuzione:**
```bash
cd nothing/
cargo build      # Compila il progetto
cargo run        # Compila ed esegue
```

**Utilizzo:**
```bash
cargo run
# Output: How many times?
# Input: 5
# Pausa di 5 secondi
```
## nothing.py (Python)

Loop interattivo che dorme per un tempo variabile e consente di ripetere o modificare la durata.

**Caratteristiche:**
- Usa `argparse` per impostare la durata iniziale da CLI
- Opzione `-n` o `--times` per specificare i secondi iniziali (default: 3)
- Menu interattivo con tre opzioni: ripetere, cambiare durata, uscire
- Validazione dell'input con controllo su numeri negativi
- Gestione degli errori per input non numerico

**Utilizzo:**
```bash
python3 nothing.py              # Avvia con 3 secondi di default
python3 nothing.py -n 10        # Avvia con 10 secondi
python3 nothing.py --times 7    # Avvia con 7 secondi
python3 nothing.py -h           # Mostra l'help
```

**Durante l'esecuzione:**
```
 Dormo per 3 secondi... 

Cosa vuoi fare?
1. Rifarlo con la stessa durata
2. Cambiare durata e rifarlo
0. Esci
Scelta: 2
Inserisci i nuovi secondi: 5
 Dormo per 5 secondi... 
```

## nothing.c (C)

Chiede all'utente il numero in modo interattivo con un loop di validazione.

**Caratteristiche:**
- Usa `scanf()` per leggere input numerico
- Loop di convalida con `isdigit()` che continua finche non riceve un valore valido
- Chiama `sleep()` della libreria POSIX
- Leggera particolarità: `isdigit()` controlla caratteri singoli; il check nel while potrebbe non funzionare come previsto per numeri multi-cifra

**Compilazione:**
```bash
gcc nothing.c -o nothing
```

**Utilizzo:**
```bash
./nothing
# Output: How many times?
# Input: 5
# Pausa di 5 secondi
```

## Differenze Principali

| Aspetto | Rust | Python | C |
|---------|------|--------|---|
| Input | Interattivo (stdin) | CLI + Menu interattivo | Interattivo (stdin) |
| Validazione | Panic se non numerico | Argparse + Loop con controllo | Loop di convalida |
| Tempo di sleep | `thread::sleep()` | `time.sleep()` | `sleep()` POSIX |
| Tipo numerico | `u64` | `int` | `int` |
| Flusso | Una sola pausa | Ripeti/modifica in loop | Una sola pausa |
| Overhead | Minimo | Avvio Python | Minimo |
