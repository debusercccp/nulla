# Nulla

Tre implementazioni dello stesso programma in linguaggi diversi. Ciascuno dorme per un tempo variabile con un loop interattivo.

Questo repo non fa assolutamente nulla. LOL

## nothing.rs (Rust)

Loop interattivo che dorme per un tempo variabile e consente di ripetere o modificare la durata.

**Caratteristiche:**
- Usa `clap` per argparse CLI
- Opzione `-n` o `--times` per specificare i secondi iniziali (default: 3)
- Menu interattivo con tre opzioni: ripetere, cambiare durata, uscire
- Validazione dell'input con controllo su numeri negativi
- Gestione degli errori per input non valido
- Performante con zero overhead

**Build ed esecuzione:**
```bash
cd nothing/
cargo build      # Compila il progetto
cargo run        # Compila ed esegue
```

**Utilizzo:**
```bash
cargo run                 # Avvia con 3 secondi di default
cargo run -- -n 5        # Avvia con 5 secondi
cargo run -- --times 7   # Avvia con 7 secondi
cargo run -- -h          # Mostra l'help
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

## nothing.py (Python)

Loop interattivo che dorme per un tempo variabile e consente di ripetere o modificare la durata.

**Caratteristiche:**
- Usa `argparse` per argparse CLI
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

Loop interattivo che dorme per un tempo variabile e consente di ripetere o modificare la durata.

**Caratteristiche:**
- Usa `getopt()` per argparse CLI
- Opzione `-n` per specificare i secondi iniziali (default: 3)
- Menu interattivo con tre opzioni: ripetere, cambiare durata, uscire
- Validazione dell'input con controllo su numeri negativi
- Gestione degli errori per input non valido
- Performante con minimo overhead

**Compilazione:**
```bash
gcc nothing.c -o nothing
```

**Utilizzo:**
```bash
./nothing              # Avvia con 3 secondi di default
./nothing -n 5         # Avvia con 5 secondi
./nothing -h           # Mostra l'help
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

## Differenze Principali

| Aspetto | Rust | Python | C |
|---------|------|--------|---|
| Argparse | clap | argparse | getopt |
| Menu interattivo | Sì | Sì | Sì |
| Validazione input | Robusta | Robusta | Robusta |
| Tempo di sleep | `thread::sleep()` | `time.sleep()` | `sleep()` POSIX |
| Tipo numerico | `u64` | `int` | `int` |
| Startup time | Minimo | ~100ms | Minimo |
| Performance | Ottimale | Buona | Ottimale |
