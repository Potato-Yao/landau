use std::fs::File;
use std::io::{Read};

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
    // in fact, I think () [] should be expressed as LPar(String)
    // but the nested expression will be too complex to parse
    // so just leave this work to exec :)
    // (
    LPar,
    // )
    RPar,
    // [
    SquareL,
    // ]
    SquareR,
    // }
    // we need } to divide blocks
    BraceR,
    // the string of Pow and Subscript is the expression of a ^ or _
    // for example, ^2 will be turned to Pow("2"), ^{2} will be turned to it as well
    // _{22} will be turned to Subscript("22")
    Superscript(String),
    Subscript(String),
    Dot,
    Eos,
}

pub type Proto = Vec<Token>;

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
                Token::Eos => {
                    vec.push(Token::Eos);
                    break;
                }
                t => match t {
                    Token::Expression(e) if e.is_empty() => (),
                    t => vec.push(t),
                }
            }
        }

        vec
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
            ',' => Token::Dot,

            '^' => Token::Superscript(self.read_pure_string(None)),
            '_' => Token::Subscript(self.read_pure_string(None)),
            'a'..='z' | 'A'..='Z' => {
                self.put_back();
                Token::Expression(self.read_pure_string(None))
            }
            '{' => Token::Expression(self.read_expression()),
            '}' => Token::BraceR,

            _ => panic!("I can`t read char: {ch}"),
        }
    }

    fn read_function(&mut self) -> Token {
        let name: String;
        let mut optional_args = Vec::<String>::new();
        let mut required_args = Vec::<String>::new();

        name = self.read_pure_string(None);

        loop {
            match self.read_char() {
                '[' => {
                    match self.read_pure_string(None) {
                        s if s.is_empty() => (),
                        s => optional_args.push(s),
                    }
                }
                '{' => {
                    match self.read_expression() {
                        s if s.is_empty() => (),
                        s => required_args.push(s),
                    }
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
    /// It is worth noticing that if a string starts with { and ends with },
    /// the first { and the last } will not be contained.
    /// If { and } can`t not pair to each other for some reason, the { and } will
    /// not be deleted.
    /// See fn read_expression_test() for a visual example.
    fn read_expression(&mut self) -> String {
        let mut count = 0;

        let mut s = self.read_string(|ch| {
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
        });

        return if s.len() == 0 {
            "".to_string()
        } else if s.chars().next().unwrap() == '{'
            && s.chars().last().unwrap() == '}' {
            s.drain(..1);  // remove the first char, which is {
            s.pop();  // remove the last char, which is }
            s
        } else {
            s
        };
    }

    /// Contains letters, numbers and decimal
    /// if set '_' as unlimited, then _ will be contained as well.
    fn read_pure_string(&mut self, unlimited: Option<char>) -> String {
        if unlimited.is_none() {
            self.read_string(|ch| ch.is_alphanumeric() || ch == '.')
        } else {
            self.read_string(|ch| ch.is_alphanumeric() || ch == '.'
                || ch == unlimited.unwrap())
        }
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
        let mut l = Lex::new("ax^2".to_string());
        let mut l1 = Lex::new("ax^2".to_string());

        assert_eq!(l.read_pure_string(None), "ax".to_string());
        assert_eq!(l1.read_pure_string(Some('^')), "ax^2".to_string());
    }

    #[test]
    fn read_expression_test() {
        let mut l = Lex::new("{aaa}".to_string());
        let mut l1 = Lex::new("a_b^c".to_string());

        assert_eq!(l.read_expression(), "aaa".to_string());
        assert_eq!(l1.read_expression(), "a_b^c".to_string());
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
    fn parse_test1() {
        let test1 = "\\frac{k}{k_0} = \\left(\\frac{T}{T_0}\\right)^{1.5}\\left(\\frac{T_0 + T_s}{T + T_{s}}\\right)
                ".to_string();
        let mut l1 = Lex::new(test1);
        let v1 = l1.parse();

        for i in v1 {
            println!("{:?}", i);
        }
    }

    #[test]
    fn parse_test2() {
        let test2 = "Pr = \\frac{\\mu{}c_p}{k}".to_string();
        let mut l2 = Lex::new(test2);
        let v2 = l2.parse();

        for i in v2 {
            println!("{:?}", i);
        }
    }

    #[test]
    fn parse_test3() {
        let test3 = "\\vec{u}_A(x + \\Delta{}x, y + \\Delta{}y, z + \\Delta{}z, t)".to_string();
        let mut l3 = Lex::new(test3);
        let v3 = l3.parse();

        for i in v3 {
            println!("{:?}", i);
        }
    }

    #[test]
    fn parse_test4() {
        let test4 = "\\int_a^b{x}\\d{}x".to_string();
        let mut l4 = Lex::new(test4);
        let v4 = l4.parse();

        for i in v4 {
            println!("{:?}", i);
        }
    }
}
