use std::fs::File;
use std::io::{Read, Seek};

#[derive(PartialEq, Debug)]
pub enum Token {
    // such as "ax^2 + bx + c"
    Expression(String),
    // Function`s name, args in [], args in {}
    Function(String, Vec<String>, Vec<String>),
    Equal,
    Add,
    Div,
    Sub,
    Times,
    // (
    LPar,
    // )
    RPar,
    // [
    SquareL,
    // ]
    SquareR,
    Pow,
    Subscript,
    Dot,
    Eos,
}

pub struct Proto(Vec<Token>);

pub struct Lex {
    cursor: usize,
    input: Vec<char>,
}

impl Lex {
    pub fn new(input: String) -> Self {
        let mut v: Vec<char> = input.chars().collect();
        v.push('\0');

        Lex { cursor: 0, input: v }
    }

    pub fn from(mut input: File) -> Self {
        let mut s = String::new();
        input.read_to_string(&mut s).unwrap();

        Lex::new(s)
    }

    pub fn parse(&mut self) -> Proto {
        let mut vec = Vec::new();
        loop {
            match self.next() {
                Token::Eos => break,
                t => vec.push(t),
            }
        }

        Proto(vec)
    }

    fn next(&mut self) -> Token {
        let ch = self.read_char();

        match ch {
            ' ' => self.next(),
            '\0' | '\n' => Token::Eos,

            '\\' => self.read_function(),
            '=' => Token::Equal,
            '+' => Token::Add,
            '-' => Token::Sub,
            '/' => Token::Div,
            '*' => Token::Times,
            '(' => Token::LPar,
            ')' => Token::RPar,
            '[' => Token::SquareL,
            ']' => Token::SquareR,

            '^' => Token::Pow,
            '_' => Token::Subscript,
            ',' => Token::Dot,

            'a'..='z' | 'A'..='Z' => Token::Expression(String::from(ch)),
            '{' => Token::Expression(self.read_expression()),
            '}' => self.next(),

            _ => panic!("I can`t read char: {ch}"),
        }
    }

    fn read_function(&mut self) -> Token {
        let name: String;
        let mut optional_args = Vec::<String>::new();
        let mut required_args = Vec::<String>::new();

        name = self.read_pure_string();

        loop {
            match self.read_char() {
                '[' => {
                    optional_args.push(self.read_pure_string());
                }
                '{' => {
                    required_args.push(self.read_expression());
                }
                ']' | '}' => {
                    continue;
                }
                _ => {
                    self.put_back();
                    break;
                }
            }
        }

        Token::Function(name, optional_args, required_args)
    }

    fn read_string<F>(&mut self, mut con: F) -> String
        where
            F: FnMut(char) -> bool,
    {
        let mut s = String::new();

        loop {
            match self.read_char() {
                ch if con(ch) => s.push(ch),
                _ => {
                    self.put_back();
                    break;
                }
            }
        }

        s
    }

    /// contains numbers, letters, ^, _ and paired {}
    /// decimal is also contained
    fn read_expression(&mut self) -> String {
        let mut count = 0;

        self.read_string(|ch| {
            match ch {
                '{' => {
                    count += 1;
                    true
                }
                '}' => {
                    if count > 0 {
                        count -= 0;
                        true
                    } else {
                        false
                    }
                }
                _ => {
                    ch.is_alphanumeric() || ch == '_' || ch == '^' || ch == '.'
                }
            }
        })
    }

    /// Only contains numbers and letters,
    /// decimal is also contained
    fn read_pure_string(&mut self) -> String {
        self.read_string(|ch| ch.is_alphanumeric() || ch == '.')
    }

    fn put_back(&mut self) {
        self.cursor -= 1;
    }

    fn read_char(&mut self) -> char {
        self.cursor += 1;
        self.input[self.cursor - 1]
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use crate::lex::{Lex, Token};

    #[test]
    fn read_char_test() {
        let f = File::open("test/test1.tex").unwrap();
        let mut l = Lex::from(f);
        let mut res = String::new();
        loop {
            match l.read_char() {
                '\0' => break,
                t => res.push(t),
            }
        }

        assert_eq!(res, "ax^2 + bx + c = 0".to_string());
    }

    #[test]
    fn read_pure_string_test() {
        let f = File::open("test/test1.tex").unwrap();
        let mut l = Lex::from(f);

        assert_eq!(l.read_pure_string(), "ax".to_string());
    }

    #[test]
    fn read_function_test() {
        let f = File::open("test/test2.tex").unwrap();
        let mut l = Lex::from(f);
        l.read_char();

        assert_eq!(l.read_function(), Token::Function(
            "frac".to_string(),
            vec![],
            vec!["1".to_string(),
                 "2".to_string()],
        ));

        l.read_char();
        l.read_char();

        assert_eq!(l.read_function(), Token::Function(
            "sqrt".to_string(),
            vec!["2".to_string()],
            vec!["1".to_string()],
        ));

        l.read_char();
        l.read_char();

        assert_eq!(l.read_function(), Token::Function(
            "rho".to_string(),
            vec![],
            vec![],
        ));
    }

    #[test]
    fn parse_test() {
        let test1 = "\\frac{k}{k_0} = \\left(\\frac{T}{T_0}\\right)^{1.5}\\left(\\frac{T_0 + T_s}{T + T_{s}}\\right)
                ".to_string();
        let test2 = "Pr = \\frac{\\mu{}c_p}{k}".to_string();
        let test3 = "\\vec{u}_A(x + \\Delta{}x, y + \\Delta{}y, z + \\Delta{}z, t)".to_string();
        let test4 = "\\int_{V}\\frac{\\par{}R}{\\par{}x_j}\\d{}v".to_string();

        let mut l1 = Lex::new(test1);
        let v1 = l1.parse();
        let mut l2 = Lex::new(test2);
        let v2 = l2.parse();
        let mut l3 = Lex::new(test3);
        let v3 = l3.parse();
        let mut l4 = Lex::new(test4);
        let v4 = l4.parse();

        for i in v1.0 {
            println!("{:?}", i);
        }
        for i in v2.0 {
            println!("{:?}", i);
        }
        for i in v3.0 {
            println!("{:?}", i);
        }
        for i in v4.0 {
            println!("{:?}", i);
        }
    }
}
