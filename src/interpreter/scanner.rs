
use crate::error::Errors;
use super::{token::Token, token_type::TokenType};

#[derive(Debug, Default)]
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            line: 1,
            ..Default::default()
        }
    }

    pub fn scan_tokens(&mut self, errors: &mut Errors) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(errors);
        }
        self.tokens
            .push(Token::new(super::token_type::TokenType::Eof, self.line));
        &self.tokens
    }

    fn scan_token(&mut self, errors: &mut Errors) {
        let next_char = self.advance();
        match next_char {
            Some(ref c) => match c {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                '-' => self.add_token(TokenType::Minus),
                '+' => self.add_token(TokenType::Plus),
                ';' => self.add_token(TokenType::Semicolon),
                '*' => self.add_token(TokenType::Star),
                '!' => {
                    let token_type = if self.advance_if_match('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    self.add_token(token_type);
                }
                '=' => {
                    let token_type = if self.advance_if_match('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(token_type);
                }
                '<' => {
                    let token_type = if self.advance_if_match('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };
                    self.add_token(token_type)
                }
                '>' => {
                    let token_type = if self.advance_if_match('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    };
                    self.add_token(token_type);
                },
                '/' => {
                    if self.advance_if_match('/') {
                        // this is a comment line. skip it.
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                },
                ' ' | '\r' | '\t' => (),
                '\n' => self.line += 1,
                _ => errors.push(crate::error::Error::CodeError {
                    line: self.line,
                    location: None,
                    message: format!("Invalid character: '{}'", c),
                }),
            },
            None => return,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.get_current();
        if let Some(c) = c {
            self.current += 1;
            return Some(c);
        }
        None
    }

    fn advance_if_match(&mut self, expected_char: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.get_current().unwrap() != expected_char {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(token_type, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> char { 
        self.get_current().unwrap_or('A')
    }

    fn get_current(&self) -> Option<char> {
        self.source.get(self.current).map(|c| c.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_tokens() {
        let mut errors: Errors = Vec::new();
        let mut scanner = Scanner::new("{(*.)}");
        scanner.scan_tokens(&mut errors);
        println!("Tokens: {:?}", scanner.tokens);
        assert_eq!(errors.len(), 0);
        assert_eq!(scanner.tokens.len(), 7);
        assert_eq!(scanner.tokens.last().unwrap().token_type(), &TokenType::Eof);
        let token_types: Vec<&TokenType> = scanner.tokens.iter().map(|t| t.token_type()).collect();
        assert_eq!(
            token_types,
            vec![
                &TokenType::LeftBrace,
                &TokenType::LeftParen,
                &TokenType::Star,
                &TokenType::Dot,
                &TokenType::RightParen,
                &TokenType::RightBrace,
                &TokenType::Eof
            ]
        );
    }

    #[test]
    fn test_scan_operators() {
        let mut errors: Errors = Vec::new();
        let mut scanner = Scanner::new("{(*.>=)==}!=");
        scanner.scan_tokens(&mut errors);
        println!("Tokens: {:?}", scanner.tokens);
        assert_eq!(errors.len(), 0);
        assert_eq!(scanner.tokens.len(), 10);
        assert_eq!(scanner.tokens.last().unwrap().token_type(), &TokenType::Eof);
        let token_types: Vec<&TokenType> = scanner.tokens.iter().map(|t| t.token_type()).collect();
        assert_eq!(
            token_types,
            vec![
                &TokenType::LeftBrace,
                &TokenType::LeftParen,
                &TokenType::Star,
                &TokenType::Dot,
                &TokenType::GreaterEqual,
                &TokenType::RightParen,
                &TokenType::EqualEqual,
                &TokenType::RightBrace,
                &TokenType::BangEqual,
                &TokenType::Eof
            ]
        );
    }

    #[test]
    fn test_scan_comment() {
        let mut errors: Errors = Vec::new();
        let mut scanner = Scanner::new(r#"{
        (*./
        >=&
        // some comment */.!=
        ==}!=
        "#);
        scanner.scan_tokens(&mut errors);
        println!("Tokens: {:?}", scanner.tokens);
        assert_eq!(errors.len(), 1);
        assert_eq!(scanner.tokens.len(), 10);
        assert_eq!(scanner.tokens.last().unwrap().token_type(), &TokenType::Eof);
        let token_types: Vec<&TokenType> = scanner.tokens.iter().map(|t| t.token_type()).collect();
        assert_eq!(
            token_types,
            vec![
                &TokenType::LeftBrace,
                &TokenType::LeftParen,
                &TokenType::Star,
                &TokenType::Dot,
                &TokenType::Slash,
                &TokenType::GreaterEqual,
                // &TokenType::RightParen,
                &TokenType::EqualEqual,
                &TokenType::RightBrace,
                &TokenType::BangEqual,
                &TokenType::Eof
            ]
        );
        let line_numbers: Vec<usize> = scanner.tokens.iter().map(|t| t.line()).collect();
        assert_eq!(line_numbers, vec![1, 2, 2, 2, 2, 3, 5, 5, 5, 6]);
    }

    #[test]
    fn test_scan_tokens_error() {
        let mut errors: Errors = Vec::new();
        let mut scanner = Scanner::new("{(*.%)&}");
        scanner.scan_tokens(&mut errors);
        println!("Errors: {:?}", errors);
        assert_eq!(errors.len(), 2);
    }
}
