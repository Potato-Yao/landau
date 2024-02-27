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

    Matrix *matrix2, *matrix3, *matrix4;
    matrix_init(&matrix2, 2, 2);
    matrix_init(&matrix3, 2, 2);
    const double r21[2] = {1.0, 1.0};
    const double r22[2] = {2.0, -1.0};
    const double r31[2] = {2.0, 2.0};
    const double r32[2] = {3.0, 4.0};
    matrix_row_replace(matrix2, 0, r21, 2);
    matrix_row_replace(matrix2, 1, r22, 2);
    matrix_row_replace(matrix3, 0, r31, 2);
    matrix_row_replace(matrix3, 1, r32, 2);
    matrix_mul(matrix2, matrix3, &matrix4);
    matrix_latex(matrix4, &pr);
    printf("%s\n", pr);
    matrix_destroy(matrix2);
    matrix_destroy(matrix3);
    matrix_destroy(matrix4);

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
