#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod util;
pub mod linear_algebra;

#[cfg(test)]
mod tests {
    use std::os::raw::c_int;
    use crate::{Matrix, matrix_init};

    #[test]
    fn init_matrix_test() {
        unsafe {
            let mut matrix_ptr: *mut Matrix = std::ptr::null_mut();
            let rows = 3;
            let cols = 4;
            let result = matrix_init(&mut matrix_ptr, rows as c_int, cols as c_int);
            if result == 0 {
                let matrix = &*matrix_ptr;
                println!("{:?}", matrix); // 使用 Debug trait 打印
            } else {
                println!("矩阵初始化失败，错误代码: {}", result);
            }
        }
    }
}
