#include <stdio.h>
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
        printf("\nconcatenation: %s", ustr_chars(&u));

        ustr_make_ascii_uppercase(&u);
        printf("\nupper-cased: %s", ustr_chars(&u));

        ustr_make_ascii_lowercase(&u);
        printf("\nlower-cased: %s", ustr_chars(&u));

        ustr_free(u);
    }
}
