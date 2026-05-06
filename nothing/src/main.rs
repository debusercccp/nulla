use std::io::{self, Write}; // Per l'input/output
use std::thread;             // Per gestire lo sleep
use std::time::Duration;     // Per definire l'intervallo di tempo

fn do_nothing(n: u64) {
    // In Rust, i nomi delle funzioni usano lo snake_case
    thread::sleep(Duration::from_secs(n));
}

fn main() {
    let mut input = String::new();

    print!("How many times? ");
    // Rust bufferizza l'output, quindi dobbiamo forzare il flush per vedere la domanda
    io::stdout().flush().expect("Errore nel flush di stdout");

    // Leggiamo l'input dell'utente
    io::stdin()
        .read_line(&mut input)
        .expect("Errore nella lettura dell'input");

    // Convertiamo la stringa in un numero (u64 = intero a 64 bit senza segno)
    // Se l'utente inserisce qualcosa di non numerico, il programma andrà in crash ("panic")
    let n: u64 = input
        .trim()
        .parse()
        .expect("Per favore, inserisci un numero valido!");

    do_nothing(n);
}
