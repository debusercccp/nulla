#!/usr/bin/env python3

import argparse
import time

def get_args():
    parser = argparse.ArgumentParser(
        description='Dorme per un tempo variabile',
        formatter_class=argparse.ArgumentDefaultsHelpFormatter)

    parser.add_argument('-n', '--times', type=int, default=3, 
                        help='durata iniziale di default')

    return parser.parse_args()

def do_nothing(n: int):
    print(f" Dormo per {n} secondi... ")
    time.sleep(n)

def main():
    args = get_args()
    durata_attuale = args.times
    
    while True:
        # Esegui l'azione con la durata attuale
        do_nothing(durata_attuale)
        
        # Chiedi all'utente cosa fare dopo
        print("\nCosa vuoi fare?")
        print("1. Rifarlo con la stessa durata")
        print("2. Cambiare durata e rifarlo")
        print("0. Esci")
        
        try:
            scelta = int(input("Scelta: "))
            
            if scelta == 0:
                print("Arrivederci!")
                break
            
            elif scelta == 1:
                continue # Riesegue il ciclo con la stessa durata_attuale
            
            elif scelta == 2:
                # Chiediamo la nuova durata
                nuova_durata = int(input("Inserisci i nuovi secondi: "))
                if nuova_durata < 0:
                    print("Il tempo non può essere negativo! Mantengo quella precedente.")
                else:
                    durata_attuale = nuova_durata
            
            else:
                print("Opzione non valida. Riprovo con la durata attuale.")
                
        except ValueError:
            print("Errore: Inserisci un numero intero valido!")
            print("Riprovo con l'ultima durata impostata...")

if __name__ == "__main__":
    main()
