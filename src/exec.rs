use latex_analyzer::lex::{Lex, Proto, Token};
use crate::function::{Function, get_function};

pub struct Exec {
    proto: Proto,
}

impl Exec {
    pub fn new(proto: Proto) -> Self {
        Exec { proto }
    }

    pub fn from_lex(mut lex: Lex) -> Self {
        Exec::new(lex.parse())
    }

    pub fn calc(&self) -> Result<f64, String> {
        let mut result = 0.0;
        let mut function_stack = Vec::<&Function>::new();
        // we call symbol \int, \sum and \prod as a huge symbol, they don`t use
        // [] for optional args but _ and ^, so they should be taken specially
        let mut huge_symbol = false;

        for token in self.proto.iter() {
            match token {
                Token::Function(fun, op, re) => {
                    let fun = get_function(fun).unwrap();
                    function_stack.push(fun);
                }
                Token::Subscript(e) => {

                }
                Token::Expression(e) => {

                }

                _ => continue,
            }
        }

        Ok(result)
    }
}
