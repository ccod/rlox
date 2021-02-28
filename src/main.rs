// use std::io;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

// TODO option for running a interpreter prompt

fn extract_contents(s: String) -> std::io::Result<String> {
    let mut file = File::open(s)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// struct InterpreterError(bool);

// book describes a seperate reporter, and a flag, will
// come back and add them later as I need it.
#[allow(dead_code)]
fn error(line: usize, message: String) {
    println!("[ line: {} ] Error: {}", line, message);
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
enum TokenType {
    // single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    String,
    Number,
    Comment,

    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ScanOptions<T, E> {
    Some(T),
    Err(E),
    None,
}

#[allow(dead_code)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i32,
    // TODO literal
    // TODO object
}

fn keywords() -> HashMap<String, TokenType> {
    let mut keymap: HashMap<String, TokenType> = HashMap::new();
    keymap.insert("and".to_owned(), TokenType::And);
    keymap.insert("class".to_owned(), TokenType::Class);
    keymap.insert("else".to_owned(), TokenType::Else);
    keymap.insert("false".to_owned(), TokenType::False);
    keymap.insert("fun".to_owned(), TokenType::Fun);
    keymap.insert("for".to_owned(), TokenType::For);
    keymap.insert("if".to_owned(), TokenType::If);
    keymap.insert("nil".to_owned(), TokenType::Nil);
    keymap.insert("or".to_owned(), TokenType::Or);
    keymap.insert("print".to_owned(), TokenType::Print);
    keymap.insert("return".to_owned(), TokenType::Return);
    keymap.insert("super".to_owned(), TokenType::Super);
    keymap.insert("this".to_owned(), TokenType::This);
    keymap.insert("true".to_owned(), TokenType::True);
    keymap.insert("var".to_owned(), TokenType::Var);
    keymap.insert("while".to_owned(), TokenType::While);
    keymap
}

struct Scanner {
    current: usize,
    line: usize,
    content: Vec<char>,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    fn new(s: String) -> Self {
        Scanner {
            current: 0,
            line: 1,
            content: s.chars().collect(),
            keywords: keywords(),
        }
    }

    fn advance(&mut self) {
        self.current += 1
    }

    fn char_match(&self, c: char) -> bool {
        if let Some(c2) = self.peek() {
            return c == c2;
        }
        false
    }

    fn one_token(&mut self, token: TokenType) -> ScanOptions<TokenType, String> {
        self.advance();
        ScanOptions::Some(token)
    }

    fn two_token(
        &mut self,
        c: char,
        a_token: TokenType,
        b_token: TokenType,
    ) -> ScanOptions<TokenType, String> {
        match self.char_match(c) {
            true => {
                self.advance();
                self.advance();
                ScanOptions::Some(a_token)
            }
            false => {
                self.advance();
                ScanOptions::Some(b_token)
            }
        }
    }

    fn maybe_comment(&mut self) -> ScanOptions<TokenType, String> {
        // let mut start = self.content.clone();
        if self.char_match('/') {
            while !self.char_match('\n') {
                self.advance()
            }
            ScanOptions::Some(TokenType::Comment)
        } else {
            self.advance();
            ScanOptions::Some(TokenType::Slash)
        }
    }

    fn capture_string(&mut self) -> ScanOptions<TokenType, String> {
        while !self.char_match('"') {
            if self.is_end() {
                return ScanOptions::Err("String didn't terminate before end of file".to_owned());
            }
            self.advance()
        }
        ScanOptions::Some(TokenType::String)
    }

    fn is_alpha_numeric(&mut self) -> ScanOptions<TokenType, String> {
        while let Some(c) = self.peek() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => self.advance(),
                _ => return ScanOptions::Some(TokenType::Identifier),
            }
        }
        return ScanOptions::Err(
            "last character of file was alpha_numeric, which shoudln't happen".to_owned(),
        );
    }

    fn is_end(&self) -> bool {
        self.current >= self.content.len()
    }

    fn peek(&self) -> Option<char> {
        if self.current + 1 >= self.content.len() {
            None
        } else {
            Some(self.content[self.current + 1])
        }
    }

    fn scan_lexeme(&mut self) -> ScanOptions<TokenType, String> {
        if self.is_end() {
            return ScanOptions::Some(TokenType::EOF);
        }
        match self.content[self.current] {
            '{' => self.one_token(TokenType::LeftBrace),
            '}' => self.one_token(TokenType::LeftBrace),
            '(' => self.one_token(TokenType::LeftBrace),
            ')' => self.one_token(TokenType::LeftBrace),
            ',' => self.one_token(TokenType::LeftBrace),
            '.' => self.one_token(TokenType::LeftBrace),
            '-' => self.one_token(TokenType::LeftBrace),
            '+' => self.one_token(TokenType::LeftBrace),
            '*' => self.one_token(TokenType::LeftBrace),
            '!' => self.two_token('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.two_token('=', TokenType::EqualEqual, TokenType::Equal),
            '<' => self.two_token('=', TokenType::LessEqual, TokenType::Less),
            '>' => self.two_token('=', TokenType::GreaterEqual, TokenType::Greater),
            '/' => self.maybe_comment(),
            '"' => self.capture_string(),
            ' ' => ScanOptions::None,
            '\r' => ScanOptions::None,
            '\t' => ScanOptions::None,
            '\n' => {
                self.line += 1;
                ScanOptions::None
            }
            'a'..='z' | 'A'..='Z' | '_' => self.is_alpha_numeric(),

            _ => ScanOptions::Some(TokenType::EOF),
        }
    }

    fn scan(&mut self) {
        self.advance();
        println!("hello, this is going to work");
        println!("current: {}", self.current);
        println!("chars: {:?}", self.content)
    }
}

fn main() {
    let arg_list: Vec<String> = env::args().collect();
    match arg_list.len() {
        1 => println!("Looking for a script file"),
        2 => match extract_contents(arg_list[1].clone()) {
            Ok(v) => Scanner::new(v).scan(),
            Err(e) => println!("err: {}", e),
        },
        _ => println!("I'm assuming something went wrong"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_scanner() {
        let mut s = Scanner::new("{".to_owned());
        assert_eq!(s.scan_lexeme(), ScanOptions::Some(TokenType::LeftBrace));
        assert_eq!(s.current, 1);
    }
}
