#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char buf[512];

int run_round(int *is_empty_ptr) {
    memset(buf, 0, sizeof(buf));
    fflush(stdout);
    if (!fgets(buf, sizeof(buf), stdin)) return 0;
    *is_empty_ptr = strlen(buf) <= 1; // Allow length 1 for newline character
    printf(buf);
}

int main() {
    printf("I'll let you make one printf call. You control the format string. No do-overs.\n");
    int is_empty = 1;

    while (is_empty) {
        if (!run_round(&is_empty)) return 0;

        if (is_empty) {
            printf("...That was an empty string. Come on, you've at least gotta try!\nOkay, I'll give you another chance.\n");
        }
    }

    return 0;
}
