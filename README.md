# Nothing

Tre implementazioni dello stesso programma in linguaggi diversi. Ciascuno chiede all'utente un numero di secondi e dorme per quel tempo.

Questa roba non fa assolutamente nulla.

## nothing.rs (Rust)

Chiede all'utente quante volte eseguire l'operazione (in secondi) e poi pausa il programma.

**Caratteristiche:**
- Usa `std::io` per input/output interattivo
- Utilizza `thread::sleep()` e `Duration` per la pausa
- Converte l'input da stringa a `u64` (intero senza segno a 64 bit)
- Esegue il flush di stdout per visualizzare il prompt prima di leggere l'input
- Crash immediato se l'input non è un numero valido

**Compilazione:**
```bash
rustc nothing.rs -o nothing
```

**Utilizzo:**
```bash
./nothing
# Output: How many times?
# Input: 5
# Pausa di 5 secondi
```

## nothing.py (Python)

Accetta il numero di secondi tramite argomenti da riga di comando oppure usa il valore di default (3 secondi).

**Caratteristiche:**
- Usa `argparse` per la gestione degli argomenti
- Opzione `-n` o `--times` per specificare i secondi
- Valore di default: 3 secondi
- Messaggio informativo prima della pausa
- Robusto rispetto a input non numerico (argparse valida il tipo)

**Utilizzo:**
```bash
python3 nothing.py              # Dorme 3 secondi
python3 nothing.py -n 10        # Dorme 10 secondi
python3 nothing.py --times 7    # Dorme 7 secondi
python3 nothing.py -h           # Mostra l'help
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
| Input | Interattivo (stdin) | Argomenti CLI | Interattivo (stdin) |
| Validazione | Panic se non numerico | Argparse (robusta) | Loop di convalida |
| Tempo di sleep | `thread::sleep()` | `time.sleep()` | `sleep()` POSIX |
| Tipo numerico | `u64` | `int` | `int` |
| Overhead | Minimo | Avvio Python | Minimo |
