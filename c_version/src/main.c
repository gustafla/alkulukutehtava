#include <stdio.h>
#include <stdlib.h>

int main() {
    size_t alaraja, ylaraja;
    printf("Anna alueen alaraja: ");
    scanf("%zu", &alaraja);
    printf("Anna alueen yläraja: ");
    scanf("%zu", &ylaraja);

    size_t *tekija = calloc(sizeof(size_t), ylaraja + 1);

    for (size_t i = 2; i * i <= ylaraja; i++) {
        if (tekija[i] == 0) {
            for (size_t j = i * i; j <= ylaraja; j += i) {
                tekija[j] = i;
            }
        }
    }

    size_t viimeinen = 0;
    for (size_t i = 0; i <= ylaraja; i++) {
        printf("%zu ", i);
        if (i < 2) {
            printf("ei ole kelvollinen alkuluku.\n");
        } else if (tekija[i] == 0) {
            printf("on alkuluku.\n");
            viimeinen = i;
        } else {
            size_t a = tekija[i], b = i / tekija[i];
            printf("ei ole alkuluku, koska %zu * %zu = %zu\n", a, b, i);
        }
    }

    if (viimeinen != 0) {
        printf("Viimeinen löydetty alkuluku oli %zu\n", viimeinen);
    }

    printf("Kiitos ohjelman käytöstä.\n");

    free(tekija);
    return 0;
}
