use latex_analyzer::lex::{Lex, Proto, Token};
use crate::function::{get_function, Known};

pub struct Exec {
    cursor: usize,
    proto: Proto,
}

struct ExecPair<'a> {
    token: &'a Token,
    pointer: i32,
}

impl ExecPair<'_> {
    fn new<'a, 'b>(token: &'b Token, pointer: i32) -> ExecPair<'a>
        where 'b: 'a
    {
        ExecPair { token, pointer }
    }
}

impl Exec {
    pub fn new(proto: Proto) -> Self {
        Exec { cursor: 0, proto }
    }

    pub fn from_lex(mut lex: Lex) -> Self {
        Exec::new(lex.parse())
    }

    pub fn calc(&self) -> Result<f64, String> {
        let mut result = 0.0;
        let mut value_stack = Vec::new();

        for token in self.proto.into_iter() {
            match token {
                Token::Function(fun, op, re) => {
                    let fun = get_function(&fun)?;
                    value_stack.push((fun.calc)(op, re));
                }

                Token::Eos => break,
                _ => ()
            }
        }

        Ok(result)
    }
}
