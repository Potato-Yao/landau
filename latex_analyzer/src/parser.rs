use crate::lex::{Proto, Token};

pub struct Parser {
    proto: Proto,
}

impl Parser {
    pub fn new(proto: Proto) -> Self {
        Parser { proto }
    }

    /// Make infix proto to postfix proto
    pub fn to_postfix_proto(&self) -> Proto {
        let mut postfix = Vec::<Token>::new();
        let mut stack = Vec::new();
        let proto = self.proto.clone();

        for p in proto.into_iter() {
            match p {
                Token::Expression(_) | Token::Function(_, _, _) => {
                    postfix.push(p);
                }
                Token::ParL => stack.push(p),
                Token::ParR => {
                    while !stack.is_empty() && *stack.last().unwrap() != Token::ParL {
                        postfix.push(stack.pop().unwrap());
                    }
                    stack.pop();
                }
                Token::Add | Token::Sub | Token::Times | Token::Div | Token::Superscript(_) => {
                    while !stack.is_empty()
                        && Parser::weight(&stack.last().unwrap()) >= Parser::weight(&p)
                    {
                        postfix.push(stack.pop().unwrap());
                    }
                    stack.push(p);
                }
                Token::Eos => break,
                _ => panic!("Token {p:?} should not occurred here!"),
            }
        }
        while !stack.is_empty() {
            postfix.push(stack.pop().unwrap());
        }
        postfix.push(Token::Eos);

        postfix
    }

    fn weight(token: &Token) -> u8 {
        match token {
            Token::Add | Token::Sub => 1,
            Token::Times | Token::Div => 2,
            Token::Superscript(_) => 3,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lex::Lex;
    use crate::parser::Parser;

    #[test]
    fn to_postfix_proto_test1() {
        let mut lex = Lex::new("a + b * (c - d) / e".to_string());
        let parser = Parser::new(lex.parse());
        let proto = parser.to_postfix_proto();

        // abcd-*e/+
        for p in proto.iter() {
            println!("{:?}", p);
        }
    }

    #[test]
    fn to_postfix_proto_test2() {
        let mut lex = Lex::new("a + (\\frac{1}{2} + 3) * \\sqrt[3]{2}".to_string());
        let parser = Parser::new(lex.parse());
        let proto = parser.to_postfix_proto();

        for p in proto.iter() {
            println!("{:?}", p);
        }
    }
}
