#include <stdlib.h>
#include <time.h>

extern unsigned my_time() {
    return time(NULL);
}

extern int my_rand() {
    return rand();
}

extern void my_srand(int seed) {
    srand(seed);
}