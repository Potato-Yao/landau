#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings_matrix.rs"));

pub mod util;
pub mod linear_algebra;

pub trait LaTeXExpression {
    fn get_expression(&self) -> Result<String, String>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {

    }
}
