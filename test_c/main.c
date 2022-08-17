#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../runicode.h"

int main(int argc, char** argv) {
    for (int i=1; i<argc; i++) {
        Ustr u = ustr_new(argv[i], strlen(argv[i]));
        printf("\n'%s':\n", argv[i]);
        printf("%ld bytes\n", ustr_lenb(&u));
        printf("%ld chars\n", ustr_lenc(&u));
        ustr_free(u);
    }

    if (argc>2) {
        Ustr u = ustr_new(argv[1], strlen(argv[1]));
        for (int i=2; i<argc; i++) {
            Ustr v = ustr_new(argv[i], strlen(argv[i]));
            u = ustr_cat(&u, &v);
            ustr_free(v);
        }
        char* t = ustr_to_c_string(u);
        printf("\ncat: %s", t);
        free(t);
    }
}
