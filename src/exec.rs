use latex_analyzer::lex::{Lex, Proto, Token};
use crate::function::{Function, get_function, HUGE_SYMBOL};

pub struct Exec {
    cursor: usize,
    proto: Proto,
}

struct ExecPair<'a> {
    token: &'a Token,
    pointer: i32,
}

impl ExecPair {
    fn new(token: &Token, pointer: i32) -> Self {
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

    pub fn load(&mut self) -> (Vec<ExecPair>, Vec<ExecPair>) {
        let mut fun_stack = Vec::<ExecPair>::new();
        let mut expr_stack = Vec::<ExecPair>::new();

        (fun_stack, expr_stack)
    }

    pub fn calc(&self) -> Result<f64, String> {
        let mut result = 0.0;

        Ok(result)
    }
}
