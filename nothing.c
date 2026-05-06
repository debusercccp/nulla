#include <stdio.h>
#include <unistd.h>
#include <ctype.h>

void doNothing(int n);
    
int main() {

    int n;

    do {
        puts("How many times?");
        scanf("%d", &n);
    } while (isdigit(n));
    
    doNothing(n);
    
    return 0;
}

void doNothing(int n) {
    sleep(n);
}
