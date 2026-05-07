use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use clap::Parser;

#[derive(Parser)]
#[command(name = "nothing")]
#[command(about = "Dormi per un tempo variabile", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "3")]
    times: u64,
}

fn do_nothing(n: u64) {
    println!(" Dormo per {} secondi... ", n);
    thread::sleep(Duration::from_secs(n));
}

fn read_input() -> Result<u32, String> {
    let mut input = String::new();
    print!("Scelta: ");
    io::stdout().flush().expect("Errore nel flush di stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Errore nella lettura dell'input");
    input
        .trim()
        .parse()
        .map_err(|_| "Errore: Inserisci un numero intero valido!".to_string())
}

fn main() {
    let args = Args::parse();
    let mut durata_attuale = args.times;

    loop {
        do_nothing(durata_attuale);

        println!("\nCosa vuoi fare?");
        println!("1. Rifarlo con la stessa durata");
        println!("2. Cambiare durata e rifarlo");
        println!("0. Esci");

        match read_input() {
            Ok(0) => {
                println!("Arrivederci!");
                break;
            }
            Ok(1) => {
                continue;
            }
            Ok(2) => {
                print!("Inserisci i nuovi secondi: ");
                io::stdout().flush().expect("Errore nel flush di stdout");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Errore nella lettura dell'input");
                match input.trim().parse::<i64>() {
                    Ok(n) if n >= 0 => durata_attuale = n as u64,
                    Ok(_) => println!("Il tempo non può essere negativo! Mantengo quella precedente."),
                    Err(_) => println!("Errore: Inserisci un numero intero valido!\nMantengo la durata precedente."),
                }
            }
            Ok(_) => println!("Opzione non valida. Riprovo con la durata attuale."),
            Err(e) => {
                println!("{}", e);
                println!("Riprovo con l'ultima durata impostata...");
            }
        }
    }
}
