use std::collections::HashMap;

use crate::types::{ScanOptions, Token, TokenType};

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

pub struct Scanner {
    start: usize,
    current: usize,
    line: usize,
    content: Vec<char>,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(s: String) -> Self {
        Scanner {
            start: 0,
            current: 0,
            line: 1,
            content: s.chars().collect(),
            keywords: keywords(),
        }
    }

    fn advance(&mut self) {
        self.current += 1
    }

    fn get_substring(&mut self) -> String {
        let mut s = String::new();
        for c in &self.content[self.start..self.current] {
            s.push(c.clone())
        }
        s
    }

    fn char_match(&self, c: char) -> bool {
        if let Some(c2) = self.peek() {
            return c == c2;
        }
        false
    }

    fn one_token(&mut self, token: TokenType) -> ScanOptions<Token, String> {
        self.advance();
        ScanOptions::Some(Token::new(token, self.get_substring(), self.line))
    }

    fn two_token(
        &mut self,
        c: char,
        a_token: TokenType,
        b_token: TokenType,
    ) -> ScanOptions<Token, String> {
        match self.char_match(c) {
            true => {
                self.advance();
                self.advance();
                ScanOptions::Some(Token::new(a_token, self.get_substring(), self.line))
            }
            false => {
                self.advance();
                ScanOptions::Some(Token::new(b_token, self.get_substring(), self.line))
            }
        }
    }

    fn maybe_comment(&mut self) -> ScanOptions<Token, String> {
        if self.char_match('/') {
            while !self.char_match('\n') {
                self.advance()
            }
            ScanOptions::Some(Token::new(
                TokenType::Comment,
                self.get_substring(),
                self.line,
            ))
        } else {
            self.advance();
            ScanOptions::Some(Token::new(
                TokenType::Slash,
                self.get_substring(),
                self.line,
            ))
        }
    }

    fn capture_string(&mut self) -> ScanOptions<Token, String> {
        self.start += 1;
        while !self.char_match('"') {
            if self.content[self.current] == '\n' {
                self.line += 1
            }
            if self.is_end() {
                return ScanOptions::Err("String didn't terminate before end of file".to_owned());
            }
            self.advance()
        }
        self.advance();
        ScanOptions::Some(Token::new(
            TokenType::String,
            self.get_substring(),
            self.line,
        ))
    }

    fn is_alpha_numeric(&mut self) -> ScanOptions<Token, String> {
        while let Some(c) = self.peek() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => self.advance(),
                _ => {
                    self.advance();
                    let s = self.get_substring();
                    if let Some(t) = self.keywords.get(&s) {
                        return ScanOptions::Some(Token::new(*t, self.get_substring(), self.line));
                    }
                    return ScanOptions::Some(Token::new(
                        TokenType::Identifier,
                        self.get_substring(),
                        self.line,
                    ));
                }
            }
        }
        return ScanOptions::Err(
            "last character of file was alpha_numeric, which shoudln't happen".to_owned(),
        );
    }

    fn is_end(&self) -> bool {
        self.current >= self.content.len() - 1
    }

    fn peek(&self) -> Option<char> {
        if self.current + 1 >= self.content.len() {
            None
        } else {
            Some(self.content[self.current + 1])
        }
    }

    fn no_token(&mut self) -> ScanOptions<Token, String> {
        self.advance();
        ScanOptions::None
    }

    // ignoring floating point numbers for the moment
    fn is_numeric(&mut self) -> ScanOptions<Token, String> {
        while let Some(c) = self.peek() {
            match c {
                '0'..='9' => self.advance(),
                _ => (),
            }
        }
        self.advance();
        ScanOptions::Some(Token::new(
            TokenType::Number,
            self.get_substring(),
            self.line,
        ))
    }

    fn scan_lexeme(&mut self) -> ScanOptions<Token, String> {
        match self.content[self.current] {
            '{' => self.one_token(TokenType::LeftBrace),
            '}' => self.one_token(TokenType::RightBrace),
            '(' => self.one_token(TokenType::LeftParen),
            ')' => self.one_token(TokenType::RightParen),
            ',' => self.one_token(TokenType::Comma),
            '.' => self.one_token(TokenType::Dot),
            '-' => self.one_token(TokenType::Minus),
            '+' => self.one_token(TokenType::Plus),
            '*' => self.one_token(TokenType::Star),
            ';' => self.one_token(TokenType::Semicolon),
            '!' => self.two_token('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.two_token('=', TokenType::EqualEqual, TokenType::Equal),
            '<' => self.two_token('=', TokenType::LessEqual, TokenType::Less),
            '>' => self.two_token('=', TokenType::GreaterEqual, TokenType::Greater),
            '/' => self.maybe_comment(),
            '"' => self.capture_string(),
            ' ' => self.no_token(),
            '\r' => self.no_token(),
            '\t' => self.no_token(),
            '\n' => {
                self.line += 1;
                self.no_token()
            }
            'a'..='z' | 'A'..='Z' | '_' => self.is_alpha_numeric(),
            '0'..='9' => self.is_numeric(),
            _ => ScanOptions::Err("Did not recognize the token".to_owned()),
        }
    }

    pub fn scan_file(&mut self) {
        let mut result: Vec<Token> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        while !self.is_end() {
            self.start = self.current;
            match self.scan_lexeme() {
                ScanOptions::Some(t) => {
                    println!("lexeme: {:?}", t);
                    result.push(t)
                }
                ScanOptions::Err(e) => errors.push(e),
                _ => (),
            }
        }
        println!("Errors: {:?}", errors);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_scan_lexeme() {
        let mut s = Scanner::new("{".to_owned());
        assert_eq!(
            s.scan_lexeme(),
            ScanOptions::Some(Token::new(TokenType::LeftBrace, "{".to_owned(), 1))
        );
        assert_eq!(s.current, 1);
    }

    #[test]
    fn check_scan_file() {
        let mut s = Scanner::new(r#"var hello = "moonshot""#.to_owned());
        s.scan_file();
        s = Scanner::new(r#"var foo = 64"#.to_owned());
        s.scan_file()
    }
}
