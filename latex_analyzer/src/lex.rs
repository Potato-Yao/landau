//! [crate::lex] used to converts LaTeX expressions into Vec<Token> (also called Proto)
//!
//! TODO: fix the terrible ownership in this file
use std::fs::File;
use std::io::{Read};
use std::string::ToString;
use lazy_static::lazy_static;

// we call \int, \sum and \prod as huge symbol
lazy_static! {
    static ref HUGE_SYMBOL: Vec<String> = {
        vec!["int".to_string(), "sum".to_string(), "prod".to_string()]
    };
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    // Expression only contains letters, numbers and decimal
    Expression(String),
    // Function`s name, optional arguments and required arguments
    // For example, \sqrt[3]{2} will be parsed as 'Function("sqrt", ["3"], ["2"])'
    Function(String, Vec<String>, Vec<String>),
    Equal,
    // symbol "="
    Add,
    // symbol "+"
    Div,
    // symbol "/"
    Sub,
    // symbol "-"
    Times,
    // symbol "*"
    // The expression within the parentheses and brackets should be represented as ParL(String) as well, in my opinion.
    // However, the nested expression would be too complex to parse, so just leave this task to exec :)
    ParL,
    // (
    ParR,
    // )
    SquareL,
    // [
    SquareR,
    // ]
    // we need } to divide blocks
    BraceR,
    // The string representing superscripts and subscripts follows the syntax of a carat (^) for superscripts and
    // an underscore (_) for subscripts.
    // For superscripts, "^2" should be translated to Superscript("2"), and "^{2}" should be handled in the same manner too.
    // For subscripts, "_{22}" is converted to Subscript("22").
    Superscript(String),
    // ^
    Subscript(String),
    // _
    Dot,
    // .
    Eos,
    // \n or \0
    NestFunction(String, Proto, Proto),
    NestExpression(Proto),
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
                    // In LaTeX expression, we often use empty braces {} to distinct blocks enhancing readability.
                    // For example, we opt for a^2{}b over a^2b to represent the multiplication of a^2 and b.
                    // The empty braces {} will be parsed as 'Expression("")', which is meaningless for exec,
                    // so we will filter it out
                    Token::Expression(e) if e.is_empty() => (),
                    t => vec.push(t),
                }
            }
        }

        Lex::post_process(vec).unwrap()
    }

    /// Some optimizations on parsed proto.
    fn post_process(proto: Proto) -> Result<Proto, String> {
        let mut proto = proto.into_iter();
        let mut vec = Vec::<Token>::new();

        loop {
            let po = proto.next();
            if po.is_none() {
                break;
            }
            match po.unwrap() {
                // Convert subscripts and superscripts of huge symbols into optional arguments.
                // Caution: For huge symbols, optional_arguments[0] stands for subscript and [1] for superscript
                Token::Function(fun, _, _) if HUGE_SYMBOL.contains(&fun) => {
                    let (sub, sup) = match (proto.next(), proto.next()) {
                        // fun_a^b -> Function("fun", ["a", "b"], [])
                        (Some(Token::Subscript(sub)), Some(Token::Superscript(sup))) =>
                            (sub.clone(), sup.clone()),
                        // fun^a_b -> Function("fun", ["b", "a"], [])
                        (Some(Token::Superscript(sup)), Some(Token::Subscript(sub))) =>
                            (sub.clone(), sup.clone()),
                        _ => return Err(format!("function {fun} miss args!")),
                    };
                    vec.push(Token::Function(fun.clone(), vec![sub, sup], vec![]))
                }
                t => {
                    vec.push(t);
                }
            }
        }

        Ok(vec)
    }

    fn string_parse(string: &String) -> Option<Token> {
        None
    }

    // Read next token
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

            '_' => Token::Subscript(self.read_subscript()),
            '^' => Token::Superscript(self.read_subscript()),
            'a'..='z' | 'A'..='Z' => {
                self.put_back();  // to read a full string
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

    /// Read string with a specific condition
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
        } else if s.chars().next().unwrap() == '{'  // Match paired braces
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

    /// The content of '_{abc}' is 'abc', the content of '_abc' is 'a'
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

    /// TODO Discuss if bounds checking is needed
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

    /// this test stands for a typical scene which contains some basic functions
    #[test]
    fn parse_test5() {
        let test5 = "\\frac{1}{2} + \\sqrt[3]{4}".to_string();
        let mut l5 = Lex::new(test5);
        let v5 = l5.parse();

        for i in v5 {
            println!("{:?}", i);
        }
    }
}
