//
// Created by Potato Yao on 2024/2/25.
//

#ifndef MATRIX_H
#define MATRIX_H

typedef struct {
    int rows;
    int cols;
    double **data;
} Matrix;

int matrix_init(Matrix **matrix, int rows, int cols);

int matrix_destroy(Matrix *matrix);

int matrix_identity_matrix(Matrix **matrix, int dimension);

int matrix_row_replace(const Matrix *matrix, int index, const double *row, int sz);

int matrix_row_exchange(const Matrix *matrix, int row1, int row2);

int matrix_item_replace(const Matrix *matrix, int row, int col, double value);

int matrix_transpose(const Matrix *origin, Matrix **matrix);

int matrix_det(const Matrix *matrix, double *value);

int matrix_lu_decomposition(const Matrix *origin, Matrix **l_matrix, Matrix **u_matrix);

int matrix_add(const Matrix *matrix1, const Matrix *matrix2, Matrix **add);

int matrix_tim(const Matrix *origin, double factor, Matrix **matrix);

int matrix_mul(const Matrix *matrix1, const Matrix *matrix2, Matrix **mul);

int matrix_lu_decompose(const Matrix *origin, Matrix **matrix);

int matrix_latex(const Matrix *matrix, char **string);

#endif //MATRIX_H
