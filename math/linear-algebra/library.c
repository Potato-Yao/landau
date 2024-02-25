#include "library.h"

#include <stdio.h>
#include "string.h"
#include "matrix.h"

void hello(void) {
    printf("Hello, World!\n");
}

int main() {
    hello();

    Matrix *matrix;
    Matrix *matrix_t;
    matrix_init(&matrix, 3, 4);
    char *pr;
    const double row[4] = {1.0, 2.0, 3.0, 4.0};
    matrix_row_replace(matrix, 0, row, matrix->cols);
    matrix_item_replace(matrix, 2, 2, 9);
    matrix_latex(matrix, &pr);
    printf("%s\n", pr);

    matrix_transpose(matrix, &matrix_t);
    matrix_latex(matrix_t, &pr);
    printf("%s\n", pr);
    matrix_destroy(matrix);

    Matrix *matrix1;
    matrix_identity_matrix(&matrix1, 5);
    matrix_row_exchange(matrix1, 0, 1);
    matrix_latex(matrix1, &pr);
    printf("%s\n", pr);
    matrix_destroy(matrix);

    String *string;
    string_init(&string);
    int a = string_append(string, "Hi");
    printf("append state %d\n", a);
    printf("%s\n", string->data);
    a = string_append(string, " Potato");
    printf("append state %d\n", a);
    printf("%s\n", string->data);
    string_destroy(string);

    printf("%s\n", double_to_str(3.12345));

    return 0;
}
