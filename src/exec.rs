use crate::function::{get_function, Known};
use crate::transformer::strings_to_known;
use latex_analyzer::lex::{Lex, Proto, Token};
use std::collections::VecDeque;

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
    where
        'b: 'a,
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

    fn load(&self) -> Result<(VecDeque<f64>, VecDeque<Token>), String> {
        let mut value_queue = VecDeque::<f64>::new();
        let mut operator_queue = VecDeque::<Token>::new();

        let proto = self.proto.clone();
        for token in proto.into_iter() {
            let mut front_needed = false;

            match token {
                Token::Function(fun, op, re) => {
                    let fun = get_function(&fun)?;
                    let op = strings_to_known(op);
                    let re = strings_to_known(re);

                    value_queue.push_back((fun.calc)(op, re).unwrap());
                }

                Token::Add | Token::Sub => operator_queue.push_back(token),

                Token::Times | Token::Div => {
                    front_needed = true;
                    operator_queue.push_back(token);
                }

                Token::Eos => break,
                _ => (),
            }
        }

        Ok((value_queue, operator_queue))
    }

    pub fn calc(&self) -> Result<f64, String> {
        let (mut value_queue, operator_queue) = self.load()?;
        let operator_disorder_error = Err("There are too many operators!".to_string());

        for opera in operator_queue.iter() {
            let (v1, v2) = match (value_queue.pop_front(), value_queue.pop_front()) {
                (Some(val1), Some(val2)) => (val1, val2),
                _ => return operator_disorder_error,
            };

            value_queue.push_back(match opera {
                Token::Add => v1 + v2,
                Token::Sub => v2 - v1,
                Token::Times => v1 * v2,
                Token::Div => v2 / v1,
                _ => {
                    return Err(format!("Unexpected token {:?} occurred!", opera));
                }
            });
        }

        return if value_queue.len() != 1 {
            Err("Error occurred in calc!".to_string())
        } else {
            Ok(value_queue.pop_front().unwrap())
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::exec::Exec;
    use latex_analyzer::lex::Lex;
    use math::round::custom_round;

    #[test]
    fn exec_test1() {
        let lex = Lex::new("\\frac{1}{2} + \\sqrt[3]{4} - \\frac{1}{3}".to_string());
        let exec = Exec::from_lex(lex);

        assert_eq!(custom_round(exec.calc().unwrap(), 3).unwrap(), 1.754);
    }

    #[test]
    fn exec_test2() {
        // qrt is not defined
        let lex = Lex::new("\\frac{1}{2} + \\qrt[3]{4}".to_string());
        let exec = Exec::from_lex(lex);

        assert!(exec.calc().is_err());
    }

    #[test]
    fn exec_test3() {
        // too many operators
        let lex = Lex::new("\\frac{1}{2} ++ \\sqrt[3]{4}".to_string());
        let exec = Exec::from_lex(lex);

        assert!(exec.calc().is_err());
    }
}
