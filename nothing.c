#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <getopt.h>

void do_nothing(int n) {
    printf(" Dormo per %d secondi... \n", n);
    sleep(n);
}

int main(int argc, char *argv[]) {
    int durata_attuale = 3;
    int scelta;
    int nuova_durata;
    int opt;
    
    // Parse argomenti CLI
    while ((opt = getopt(argc, argv, "n:h")) != -1) {
        switch (opt) {
            case 'n':
                durata_attuale = atoi(optarg);
                if (durata_attuale < 0) {
                    fprintf(stderr, "Errore: il tempo non può essere negativo!\n");
                    return 1;
                }
                break;
            case 'h':
                printf("Utilizzo: %s [-n secondi]\n", argv[0]);
                printf("Opzioni:\n");
                printf("  -n secondi    Durata iniziale di default (default: 3)\n");
                printf("  -h            Mostra questo messaggio d'aiuto\n");
                return 0;
            case '?':
                fprintf(stderr, "Opzione non valida\n");
                return 1;
        }
    }
    
    while (1) {
        // Esegui l'azione con la durata attuale
        do_nothing(durata_attuale);
        
        // Menu interattivo
        printf("\nCosa vuoi fare?\n");
        printf("1. Rifarlo con la stessa durata\n");
        printf("2. Cambiare durata e rifarlo\n");
        printf("0. Esci\n");
        printf("Scelta: ");
        
        // Leggi la scelta
        if (scanf("%d", &scelta) != 1) {
            // Input non valido
            printf("Errore: Inserisci un numero intero valido!\n");
            printf("Riprovo con l'ultima durata impostata...\n");
            // Pulisci il buffer
            while (getchar() != '\n');
            continue;
        }
        
        if (scelta == 0) {
            printf("Arrivederci!\n");
            break;
        }
        else if (scelta == 1) {
            // Continua il ciclo con la stessa durata
            continue;
        }
        else if (scelta == 2) {
            // Chiedi la nuova durata
            printf("Inserisci i nuovi secondi: ");
            if (scanf("%d", &nuova_durata) != 1) {
                printf("Errore: Inserisci un numero intero valido!\n");
                printf("Mantengo la durata precedente.\n");
                // Pulisci il buffer
                while (getchar() != '\n');
            }
            else if (nuova_durata < 0) {
                printf("Il tempo non può essere negativo! Mantengo quella precedente.\n");
            }
            else {
                durata_attuale = nuova_durata;
            }
        }
        else {
            printf("Opzione non valida. Riprovo con la durata attuale.\n");
        }
    }
    
    return 0;
}
