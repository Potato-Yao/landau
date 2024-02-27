use std::ffi::CStr;
use std::ops::{Add, Mul, Neg, Sub};
use std::os::raw::{c_char, c_int};
use std::ptr::null_mut;
use crate::{LaTeXExpression, Matrix, matrix_add, matrix_destroy, matrix_init, matrix_item_replace, matrix_latex, matrix_mul, matrix_row_exchange, matrix_row_replace, matrix_tim, matrix_transpose};

impl Matrix {
    pub fn new(rows: i32, cols: i32) -> Result<Box<Matrix>, String> {
        unsafe {
            let mut matrix_ptr: *mut Matrix = null_mut();
            let stat = matrix_init(&mut matrix_ptr, rows, cols);
            match stat {
                0 => Ok(Box::from_raw(matrix_ptr)),
                -1 => Err(format!("rows {rows} or cols {cols} is less than zero")),
                1 => Err("Malloc for matrix or array of Matrix failed".to_string()),
                _ => Err("Unknown err!".to_string()),
            }
        }
    }

    pub fn new_identity_matrix(dimension: i32) -> Result<Box<Matrix>, String> {
        let matrix = Matrix::new(dimension, dimension)?;

        for i in 0..dimension {
            let _ = matrix.set_item(i, i, 1.0);
        }

        Ok(matrix)
    }

    pub fn from_transpose(origin: &Matrix) -> Result<Box<Matrix>, String> {
        unsafe {
            let mut matrix_ptr: *mut Matrix = null_mut();
            let stat = matrix_transpose(origin as *const _, &mut matrix_ptr);
            match stat {
                0 => Ok(Box::from_raw(matrix_ptr)),
                1 => Err("Malloc for matrix or array of Matrix failed".to_string()),
                _ => Err("Unknown err!".to_string()),
            }
        }
    }

    pub fn set_row(&self, index: i32, row: Vec<f64>) -> Result<(), String> {
        unsafe {
            let stat =
                matrix_row_replace(self as *const _, index, row.as_ptr(), row.len() as c_int);
            match stat {
                -1 => Err(format!("index {index} is less than zero or greater than rows of matrix")),
                _ => Ok(()),
            }
        }
    }

    pub fn set_item(&self, row: i32, col: i32, value: f64) -> Result<(), String> {
        unsafe {
            let stat = matrix_item_replace(self as *const _, row, col, value);
            match stat {
                -1 => Err(format!("{row} or {col} is less than zero or greater than the row/col of matrix")),
                _ => Ok(()),
            }
        }
    }

    pub fn exchange_row(&self, row1: i32, row2: i32) -> Result<(), String> {
        unsafe {
            let stat = matrix_row_exchange(self as *const _, row1, row2);
            match stat {
                -1 => Err(format!("{row1} or {row2} is less than zero or greater than the row of matrix")),
                _ => Ok(()),
            }
        }
    }
}

macro_rules! stat_with_Box_and_panic {
    ($s: expr, $ptr: expr) => {
        {
            let stat: i32 = $s;
            let matrix: *mut Matrix = $ptr;
            match stat {
                0 => return Box::from_raw(matrix),
                -1 => panic!("input argument is null"),
                -2 => panic!("alloc for Matrix failed"),
                1 => panic!("index out of bounds"),
                2 => panic!("rows or cols mismatched"),
                _ => panic!("unknown error code"),
            }
        }
    };
}

impl Add<&Matrix> for &Matrix {
    type Output = Box<Matrix>;

    fn add(self, rhs: &Matrix) -> Self::Output {
        unsafe {
            let mut matrix_ptr: *mut Matrix = null_mut();
            let stat = matrix_add(&*self, &*rhs, &mut matrix_ptr);
            stat_with_Box_and_panic!(stat, matrix_ptr);
        }
    }
}

impl Sub<&Matrix> for &Matrix {
    type Output = Box<Matrix>;

    fn sub(self, rhs: &Matrix) -> Self::Output {
        self + &*(-rhs)
    }
}

impl Mul<f64> for &Matrix {
    type Output = Box<Matrix>;

    fn mul(self, rhs: f64) -> Self::Output {
        unsafe {
            let mut matrix_ptr: *mut Matrix = null_mut();
            let stat = matrix_tim(&*self, rhs, &mut matrix_ptr);
            stat_with_Box_and_panic!(stat, matrix_ptr);
        }
    }
}

impl Neg for &Matrix {
    type Output = Box<Matrix>;

    fn neg(self) -> Self::Output {
        self * -1f64
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Box<Matrix>;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        unsafe {
            let mut matrix_ptr: *mut Matrix = null_mut();
            let stat = matrix_mul(&*self, &*rhs, &mut matrix_ptr);
            stat_with_Box_and_panic!(stat, matrix_ptr);
        }
    }
}

impl LaTeXExpression for Matrix {
    fn get_expression(&self) -> Result<String, String> {
        unsafe {
            let mut string_ptr: *mut c_char = null_mut();
            let stat = matrix_latex(self as *const _, &mut string_ptr);
            // See C source code for meaning of err code
            match stat {
                0 => {
                    let c_string = CStr::from_ptr(string_ptr);
                    let latex_string = c_string.to_str()
                        .map_err(|_| "Invalid UTF-8 string")?.to_owned();
                    Ok(latex_string)
                }
                1 => Err("Can not alloc for str".to_string()),
                _ => Err("Unknown err".to_string())
            }
        }
    }
}

impl Drop for Matrix {
    fn drop(&mut self) {
        unsafe {
            matrix_destroy(self as *mut _);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{LaTeXExpression, Matrix};

    #[test]
    fn new_test() {
        let m = Matrix::new(3, 4).unwrap();
        assert_eq!(m.rows, 3);
        assert_eq!(m.cols, 4);
    }

    #[test]
    fn latex_test() {
        let m = Matrix::new(3, 4).unwrap();
        println!("{}", m.get_expression().unwrap());
    }

    #[test]
    fn new_identity_matrix_test() {
        let m = Matrix::new_identity_matrix(3).unwrap();
        println!("{}", m.get_expression().unwrap());
    }

    #[test]
    fn set_row_test() {
        let m = Matrix::new(3, 4).unwrap();
        m.set_row(1, vec![2.0, 3.0, 4.0, 5.0]).unwrap();
        println!("{}", m.get_expression().unwrap());
    }

    #[test]
    fn set_item_test() {
        let m = Matrix::new(3, 4).unwrap();
        m.set_item(2, 2, 5.0).unwrap();
        println!("{}", m.get_expression().unwrap());
    }

    #[test]
    fn exchange_row_test() {
        let m = Matrix::new_identity_matrix(3).unwrap();
        m.exchange_row(0, 1).unwrap();
        println!("{}", m.get_expression().unwrap());
    }

    #[test]
    fn from_transpose_test() {
        let m = Matrix::new(3, 4).unwrap();
        m.set_row(0, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        m.set_item(2, 2, 9.0).unwrap();
        let m1 = Matrix::from_transpose(&m).unwrap();
        println!("{}", m1.get_expression().unwrap());
    }

    #[test]
    fn op_test() {
        let m1 = Matrix::new(2, 2).unwrap();
        m1.set_row(0, vec![1.0, 1.0]).unwrap();
        m1.set_row(1, vec![2.0, -1.0]).unwrap();

        let m2 = Matrix::new(2, 2).unwrap();
        m2.set_row(0, vec![2.0, 2.0]).unwrap();
        m2.set_row(1, vec![3.0, 4.0]).unwrap();

        let m3 = &*m1 * &*m2;
        println!("{}", m3.get_expression().unwrap());

        let m4 = &*m1 + &*m2;
        println!("{}", m4.get_expression().unwrap());

        let m5 = &*m1 - &*m2;
        println!("{}", m5.get_expression().unwrap());
    }
}
