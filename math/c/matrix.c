//
// Created by Potato Yao on 2024/2/25.
//

#include "matrix.h"

#include <math.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include "string.h"

int matrix_init(Matrix **matrix, const int rows, const int cols) {
    if (rows < 0 || cols < 0) return ARGUMENT_NULL_ERROR;

    double **ptr = malloc(sizeof(double *) * rows);
    if (ptr == NULL) return ALLOCATION_FAILURE_ERROR;

    for (int i = 0; i < rows; i++) {
        double *p = malloc(sizeof(double) * cols);
        if (p == NULL) {
            for (int j = 0; j < i; j++) {
                free(ptr[j]);
            }
            free(ptr);
            return ALLOCATION_FAILURE_ERROR;
        }
        ptr[i] = p;
    }

    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            ptr[i][j] = 0.0;
        }
    }

    Matrix *m = malloc(sizeof(Matrix));
    if (m == NULL) {
        for (int i = 0; i < rows; i++) {
            free(ptr[i]);
        }
        free(ptr);
        return ALLOCATION_FAILURE_ERROR;
    }
    m->rows = rows;
    m->cols = cols;
    m->data = ptr;
    *matrix = m;

    return SUCCESS_CODE;
}

int matrix_destroy(Matrix *matrix) {
    if (matrix->data == NULL) return ARGUMENT_NULL_ERROR;

    for (int i = 0; i < matrix->rows; i++) {
        free(matrix->data[i]);
    }

    free(matrix->data);
    matrix->data = NULL;
    matrix->rows = 0;
    matrix->cols = 0;

    return SUCCESS_CODE;
}

int matrix_row_replace(const Matrix *matrix, const int index, const double *row, const int sz) {
    if (matrix->data == NULL) return ARGUMENT_NULL_ERROR;
    if (index < 0 || index > matrix->rows) return INDEX_OUT_OF_BOUNDS_ERROR;

    for (int i = 0; i < matrix->cols && i < sz; i++) {
        matrix->data[index][i] = row[i];
    }

    return SUCCESS_CODE;
}

int matrix_item_replace(const Matrix *matrix, const int row, const int col, const double value) {
    if (matrix->data == NULL) return ARGUMENT_NULL_ERROR;
    if (row > matrix->rows || col > matrix->cols || row < 0 || col < 0) return INDEX_OUT_OF_BOUNDS_ERROR;

    matrix->data[row][col] = value;

    return SUCCESS_CODE;
}

int matrix_identity_matrix(Matrix **matrix, const int dimension) {
    Matrix *m;
    int stat = matrix_init(&m, dimension, dimension);
    if (stat != SUCCESS_CODE) return stat;

    for (int i = 0; i < dimension; i++) {
        stat = matrix_item_replace(m, i, i, 1.0);
        if (stat != SUCCESS_CODE) {
            matrix_destroy(m);
            return stat;
        }
    }
    *matrix = m;

    return SUCCESS_CODE;
}

int matrix_row_exchange(const Matrix *matrix, const int row1, const int row2) {
    if (matrix->data == NULL) return ARGUMENT_NULL_ERROR;
    if (row1 > matrix->rows || row2 > matrix->rows || row1 < 0 || row2 < 0) return INDEX_OUT_OF_BOUNDS_ERROR;
    if (row1 == row2) return SUCCESS_CODE;

    double *temp = matrix->data[row1];
    matrix->data[row1] = matrix->data[row2];
    matrix->data[row2] = temp;

    return SUCCESS_CODE;
}

int matrix_transpose(const Matrix *origin, Matrix **matrix) {
    Matrix *m;
    const int stat = matrix_init(&m, origin->cols, origin->rows);
    if (stat != SUCCESS_CODE) return stat;

    for (int i = 0; i < origin->rows; i++) {
        for (int j = 0; j < origin->cols; j++) {
            m->data[j][i] = origin->data[i][j];
        }
    }
    *matrix = m;

    return SUCCESS_CODE;
}

int matrix_add(const Matrix *matrix1, const Matrix *matrix2, Matrix **add) {
    if (matrix1 == NULL || matrix2 == NULL) return ARGUMENT_NULL_ERROR;
    if (matrix1->cols != matrix2->cols || matrix1->rows != matrix2->rows) return INDEX_MISMATCH_ERROR;

    Matrix *mat;
    const int stat = matrix_init(&mat, matrix1->rows, matrix1->cols);
    if (stat != SUCCESS_CODE) return stat;

    for (int i = 0; i < matrix1->rows; i++) {
        for (int j = 0; j < matrix1->cols; j++) {
            mat->data[i][j] = matrix1->data[i][j] + matrix2->data[i][j];
        }
    }
    *add = mat;

    return SUCCESS_CODE;
}

int matrix_tim(const Matrix *origin, const double factor, Matrix **matrix) {
    if (origin== NULL) return ARGUMENT_NULL_ERROR;

    Matrix *mat;
    const int stat = matrix_init(&mat, origin->rows, origin->cols);
    if (stat != 0 ) return stat;

    for (int i = 0; i <origin->rows; i++) {
        for (int j = 0; j <origin->cols; j++) {
            mat->data[i][j] = origin->data[i][j] * factor;
        }
    }
    *matrix = mat;

    return SUCCESS_CODE;
}

int matrix_mul(const Matrix *matrix1, const Matrix *matrix2, Matrix **mul) {
    if (matrix1 == NULL || matrix2 == NULL) return ARGUMENT_NULL_ERROR;
    if (matrix1->cols != matrix2->rows) return INDEX_MISMATCH_ERROR;

    Matrix *mat;
    const int stat = matrix_init(&mat, matrix1->rows, matrix2->cols);
    if (stat != SUCCESS_CODE) return stat;

    for (int i = 0; i < mat->rows; i++) {
        for (int j = 0; j < mat->cols; j++) {
            for (int k = 0; k < matrix1->cols; k++) {
                mat->data[i][j] += matrix1->data[i][k] * matrix2->data[k][j];
            }
        }
    }
    *mul = mat;

    return SUCCESS_CODE;
}

int matrix_latex(const Matrix *matrix, char **string) {
    if (matrix == NULL) return ARGUMENT_NULL_ERROR;

    String *s;
    string_init(&s);
    string_append(s, "\\begin{pmatrix}\n");

    for (int i = 0; i < matrix->rows; i++) {
        for (int j = 0; j < matrix->cols - 1; j++) {
            string_append(s, "{");
            string_append(s, double_to_str(matrix->data[i][j]));
            string_append(s, "} & ");
        }
        string_append(s, "{");
        string_append(s, double_to_str(matrix->data[i][matrix->cols - 1]));
        string_append(s, "}\\\\\n");
    }
    string_append(s, "\\end{pmatrix}\n");

    char *str = malloc((s->size + 1) * sizeof(char));
    if (str == NULL) {
        string_destroy(s);
        return ALLOCATION_FAILURE_ERROR;
    }

    strncpy(str, s->data, s->size);
    str[s->size] = '\0';
    string_destroy(s);
    *string = str;

    return SUCCESS_CODE;
}
