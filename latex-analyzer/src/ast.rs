use crate::lex::{Proto, Token};

#[derive(PartialEq, Debug)]
pub enum NodeKind {
    Num,
    Op,
}

#[derive(Debug)]
pub struct Node {
    pub node_kind: NodeKind,
    pub value: Option<Token>,
    pub op: Option<Token>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub struct AST(pub Node, pub Vec<String>);

impl AST {
    pub fn new(proto: Proto) -> Self {
        let (node, var) = Node::parse_rpn(proto);
        AST(node, var)
    }
}

impl Node {
    fn new_value_node(value: Token) -> Result<Node, String> {
        match value {
            Token::Expression(_) | Token::Function(_, _, _) => {
                let node = Node {
                    node_kind: NodeKind::Num,
                    value: Some(value),
                    op: None,
                    left: None,
                    right: None,
                };

                Ok(node)
            }
            _ => Err(format!("Token {value:?} can not be value of AST!")),
        }
    }

    fn new_op_node(op: Token, left: Node, right: Node) -> Result<Node, String> {
        match op {
            Token::Add | Token::Sub | Token::Div | Token::Times | Token::Superscript(_) => {
                let node = Node {
                    node_kind: NodeKind::Op,
                    value: None,
                    op: Some(op),
                    left: Some(Box::from(left)),
                    right: Some(Box::from(right)),
                };

                Ok(node)
            }
            _ => Err(format!("Token {op:?} can not be a operator!")),
        }
    }

    fn parse_rpn(expr: Proto) -> (Node, Vec<String>) {
        let mut stack = Vec::new();
        let mut var = Vec::new();

        for e in expr.into_iter() {
            match e {
                Token::Expression(_) | Token::Function(_, _, _) => {
                    stack.push(Node::new_value_node(e).unwrap());
                }
                Token::Superscript(content) => {
                    let op2 = Node::new_value_node(Token::Expression(content)).unwrap();
                    let op1 = stack.pop().unwrap();

                    stack.push(Node::new_op_node(e, op1, op2).unwrap())
                }
                Token::Var(s) => var.push(s),
                Token::Eos => break,
                _ => {
                    let op2 = stack.pop().unwrap();
                    let op1 = stack.pop().unwrap();

                    stack.push(Node::new_op_node(e, op1, op2).unwrap());
                }
            }
        }

        (stack.pop().unwrap(), var)
    }
}
