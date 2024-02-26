use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::{LaTeXExpression, Matrix, matrix_init, matrix_latex};

impl Matrix {
    fn new(rows: i32, cols: i32) -> Result<Box<Matrix>, String> {
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
        todo!()
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
}
