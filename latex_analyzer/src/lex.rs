//! [crate::lex] used to converts LaTeX expressions into Vec<Token> (also called Proto)
//!
//! TODO: fix the terrible ownership in this file
use std::fs::File;
use std::io::{Read};
use std::string::ToString;
use lazy_static::lazy_static;

lazy_static! {
    static ref HUGE_SYMBOL: Vec<String> = {
        vec!["int".to_string(), "sum".to_string(), "prod".to_string()]
    };
}

#[derive(PartialEq, Debug, Clone)]
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
    // in fact, I think () [] should be represented as ParL(String)
    // but the nested expression would be too complex to parse
    // so just leave this work to exec :)
    // (
    ParL,
    // )
    ParR,
    // [
    SquareL,
    // ]
    SquareR,
    // }
    // we need } to divide blocks
    BraceR,
    // the string of Superscript and Subscript is the expression of a ^ or _
    // for example, ^2 will be turned to Superscript("2"), ^{2} will be turned to it as well
    // _{22} will be turned to Subscript("22")
    Superscript(String),
    Subscript(String),
    Dot,
    Eos,
    NestFunction(String, Vec<Proto>, Vec<Proto>),
    NestExpression(Vec<Proto>),
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

        Lex::post_process(vec).unwrap()
    }

    fn post_process(proto: Proto) -> Result<Proto, String> {
        let mut proto = proto.into_iter();
        let mut vec = Vec::<Token>::new();

        loop {
            let po = proto.next();
            if po.is_none() {
                break;
            }
            match po.unwrap() {
                Token::Function(fun, _, _) => {
                    if HUGE_SYMBOL.contains(&fun) {
                        let (sub, sup) = match (proto.next(), proto.next()) {
                            (Some(Token::Subscript(sub)), Some(Token::Superscript(sup))) =>
                                (sub.clone(), sup.clone()),
                            (Some(Token::Superscript(sup)), Some(Token::Subscript(sub))) =>
                                (sub.clone(), sup.clone()),
                            _ => return Err(format!("function {fun} miss args!")),
                        };
                        vec.push(Token::Function(fun.clone(), vec![sub, sup], vec![]))
                    } else {}
                }
                t => {
                    vec.push(t);
                }
            }
        }

        Ok(vec)
    }

    fn string_parse(string: &String) -> Option<Token> {
        let mut lex = Lex::new(string.clone());
        let mut pro = Vec::<Token>::new();

        for i in lex.parse().into_iter() {
            match i {
                Token::Function(_, op, re) => {
                    let mut temp_op = Vec::<Token>::new();
                    let mut temp_re = Vec::<Token>::new();
                    let func = |string: &Vec<String>, mut target: &mut Vec<Token>| {
                        for j in string.iter() {
                            let p = Lex::string_parse(j);
                            if p.is_some() {
                                target.push(p.unwrap());
                            }
                        }
                    };
                    func(&op, &mut temp_op);
                    func(&re, &mut temp_re);
                }
                Token::Expression(e) => {

                }
                _ => ()
            }
        }

        None
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
            '(' => Token::ParL,
            ')' => Token::ParR,
            '[' => Token::SquareL,
            ']' => Token::SquareR,
            ',' => Token::Dot,

            // '^' => Token::Superscript(self.read_pure_string(None)),
            // '_' => Token::Subscript(self.read_pure_string(None)),
            '_' => Token::Subscript(self.read_subscript()),
            '^' => Token::Superscript(self.read_subscript()),
            'a'..='z' | 'A'..='Z' => {
                self.put_back();
                Token::Expression(self.read_pure_string())
            }
            '{' => Token::Expression(self.read_until_brace_r()),
            '}' => Token::BraceR,

            _ => panic!("I can`t read char: {ch}"),
        }
    }

    /// Note: This function does not verify whether the number of arguments provided
    /// matches the expected number for the function it parses.
    ///
    /// For example, "\\frac{\\mu{}c_p}{k}" will be parsed as Function("frac", [], \["\\mu{}c_p", "k"]),
    /// However, "\\frac{\\mu{}c_p}{k}{s}" will be parsed as Function("frac", [], \["\\mu{}c_p", "k", "s"]).
    ///
    /// Treason for not addressing this issue is that the function "get_function"
    /// resides in another crate and I don`t want to break the modularity and independence
    /// of this crate.
    /// Moreover, it is not our duty to validate if the LaTeX input has anything wrong.
    /// If incorrect LaTeX is provided, we can throw an error in exec :)
    fn read_function(&mut self) -> Token {
        let name: String;
        let mut optional_args = Vec::<String>::new();
        let mut required_args = Vec::<String>::new();

        name = self.read_pure_string();

        loop {
            match self.read_char() {
                '[' => {
                    match self.read_pure_string() {
                        s if s.is_empty() => (),
                        s => optional_args.push(s),
                    }
                }
                '{' => {
                    match self.read_until_brace_r() {
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

    /// If the input string begins with a '{', this function will read until it finds the
    /// corresponding '}' that pairs with the initial '{'.
    /// If the input string does not start with a '{', the function will read until it encounters
    /// a standalone '}'.
    fn read_until_brace_r(&mut self) -> String {
        let mut count = 0;

        let mut s = self.read_string(|ch| {
            match ch {
                '{' => {
                    count += 1;
                    true
                }
                '}' => {
                    if count > 0 {
                        count -= 1;
                        true
                    } else {
                        false
                    }
                }
                _ => {
                    ch.is_alphanumeric() || ch == '_' || ch == '^' || ch == '.'
                        || ch == '+' || ch == '-' || ch == '*' || ch == '/'
                        || ch == ' ' || ch == '\\'
                }
            }
        });

        return if s.len() == 0 {
            "".to_string()
        } else if s.chars().next().unwrap() == '{'
            && s.chars().last().unwrap() == '}' {
            s.drain(..1);  // remove the first char, which is a '{'
            s.pop();  // remove the last char, which is a '}'
            s
        } else {
            s
        };
    }

    /// Contains letters, numbers and decimal
    fn read_pure_string(&mut self) -> String {
        self.read_string(|ch| ch.is_alphanumeric() || ch == '.')
    }

    fn read_subscript(&mut self) -> String {
        match self.read_char() {
            '{' => {
                self.put_back();
                self.read_until_brace_r()
            }
            c => String::from(c)
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

        assert_eq!(l.read_pure_string(), "ax".to_string());
    }

    #[test]
    fn read_until_brace_r_test() {
        let mut l = Lex::new("{aaa}".to_string());
        let mut l1 = Lex::new("a_b^c".to_string());

        assert_eq!(l.read_until_brace_r(), "aaa".to_string());
        assert_eq!(l1.read_until_brace_r(), "a_b^c".to_string());
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
