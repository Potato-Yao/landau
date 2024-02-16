use crate::function::get_function;
use crate::transformer::{string_to_known, strings_to_known};
use latex_analyzer::ast::{Node, NodeKind, AST};
use latex_analyzer::lex::{Lex, Token};
use latex_analyzer::parser::Parser;
use num::pow;
use crate::config;

pub struct Exec {
    node: Node,
}

impl Exec {
    pub fn from_lex(mut lex: Lex) -> Exec {
        let proto = lex.parse();
        let parser = Parser::new(proto);
        let proto = parser.to_postfix_proto();
        let ast = AST::new(proto);

        Exec { node: ast.0 }
    }

    pub fn calculate(&self) -> Result<f64, String> {
        Exec::evaluate_node(&self.node)
    }

    fn evaluate_node(node: &Node) -> Result<f64, String> {
        return if node.node_kind == NodeKind::Num {
            match node.value.clone().unwrap() {
                Token::Function(fun, op, re) => {
                    let fun = get_function(&fun)?;
                    let op = strings_to_known(op);
                    let re = strings_to_known(re);
                    let result = (fun.calc)(op, re);

                    Ok(result.unwrap())
                }
                Token::Expression(ref expr) => Ok(string_to_known(expr).get_value().unwrap()),
                _ => Err(format!("Can not evaluate {node:?}")),
            }
        } else {
            Exec::evaluate_op_node(node)
        };
    }

    fn evaluate_op_node(node: &Node) -> Result<f64, String> {
        let (left, right) = match (
            Exec::evaluate_node(&node.left.as_ref().unwrap()),
            Exec::evaluate_node(&node.right.as_ref().unwrap()),
        ) {
            (Ok(l), Ok(r)) => (l, r),
            (l, r) => return Err(format!("Node {l:?} or {r:?} can not be evaluated!")),
        };

        let result = match node.op.clone().unwrap() {
            Token::Add => left + right,
            Token::Sub => left - right,
            Token::Times => left * right,
            Token::Div => left / right,
            Token::Superscript(_) => {
                if config::CONFIG.high_accuracy {
                    math::pow::high_accuracy_pow(left, right)
                } else {
                    pow(left, right as usize)
                }
            }
            o => return Err(format!("Token {o:?} can not be a operator!")),
        };

        Ok(result)
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

        assert_eq!(custom_round(exec.calculate().unwrap(), 3).unwrap(), 1.754);
    }

    #[test]
    fn exec_test2() {
        let lex = Lex::new("2^2".to_string());
        let exec = Exec::from_lex(lex);

        assert_eq!(exec.calculate().unwrap(), 4.0);
    }

    #[test]
    fn exec_test3() {
        let lex = Lex::new("\\int_1^2x\\di{x}".to_string());
        let exec = Exec::from_lex(lex);

        assert_eq!(exec.calculate().unwrap(), 1.5);
    }
}
