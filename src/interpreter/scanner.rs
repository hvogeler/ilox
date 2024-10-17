use super::{token::Token, token_type::TokenType};
use crate::error::Errors;

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
                    let token_type = self.scan_operator('=', TokenType::Bang, TokenType::BangEqual);
                    self.add_token(token_type);
                }
                '=' => {
                    let token_type =
                        self.scan_operator('=', TokenType::Equal, TokenType::EqualEqual);
                    self.add_token(token_type);
                }
                '<' => {
                    let token_type = self.scan_operator('=', TokenType::Less, TokenType::LessEqual);
                    self.add_token(token_type)
                }
                '>' => {
                    let token_type =
                        self.scan_operator('=', TokenType::Greater, TokenType::GreaterEqual);
                    self.add_token(token_type);
                }
                '/' => {
                    if self.advance_if_match('/') {
                        // detect '//' and skip the comment line
                        self.skip_rest_of_line();
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                }
                '"' => self.handle_string(errors),
                '0'..='9' => self.handle_number(),
                ' ' | '\r' | '\t' => (),
                '\n' => self.new_line(),
                _ => errors.push(crate::error::Error::CodeError {
                    line: self.line,
                    location: None,
                    message: format!("Invalid character: '{}'", c),
                }),
            },
            None => return,
        }
    }

    fn handle_number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance(); // consume the '.'
            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        let slce = &self.source.as_slice()[self.start..self.current];
        let string_value: String = slce.iter().collect();
        let value: f64 = string_value.parse().unwrap_or(0.0);
        self.add_token(TokenType::Number(value));
    }

    fn is_digit(c: char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false,
        }
    }

    fn handle_string(&mut self, errors: &mut Errors) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.new_line();
            }
            self.advance();
        }
        if self.is_at_end() {
            errors.push(crate::error::Error::CodeError {
                line: self.line,
                location: None,
                message: format!("String not terminated"),
            });
        } else {
            self.advance(); // consume the terminating '"'
            let slce = &self.source.as_slice()[self.start + 1..self.current - 1];
            let string_value: String = slce.iter().collect();
            self.add_token(TokenType::String(string_value));
        }
    }

    fn new_line(&mut self) {
        self.line += 1;
    }

    fn skip_rest_of_line(&mut self) {
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }
    }

    // Operators can be single character types like '+', '-' or '!'.
    // Operators can be dual types like '!=', '>='
    fn scan_operator(
        &mut self,
        second_char: char,
        token_single: TokenType,
        token_dual: TokenType,
    ) -> TokenType {
        if self.advance_if_match(second_char) {
            token_dual
        } else {
            token_single
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
        self.get_current().unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        *(self.source.get(self.current + 1).unwrap_or(&'\0'))
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
        let mut scanner = Scanner::new(
            r#"{
        (*./
        >=&
        // some comment */.!=
        ==}!=
        "#,
        );
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
    fn test_scan_string() {
        let mut errors: Errors = Vec::new();
        let mut scanner = Scanner::new(
            r#"{(
        "Doris"
        *"hello world".)}
        "end of string""#,
        );
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
                &TokenType::String("Doris".to_owned()),
                &TokenType::Star,
                &TokenType::String("hello world".to_owned()),
                &TokenType::Dot,
                &TokenType::RightParen,
                &TokenType::RightBrace,
                &TokenType::String("end of string".to_owned()),
                &TokenType::Eof
            ]
        );
    }

    #[test]
    fn test_scan_numbwe() {
        let mut errors: Errors = Vec::new();
        let mut scanner = Scanner::new(
            r#"{(
        "Doris"
        *218.)}
        4711 + 3.1415 3.1.4..5"#,
        );
        scanner.scan_tokens(&mut errors);
        println!("Tokens: {:?}", scanner.tokens);
        assert_eq!(errors.len(), 0);
        assert_eq!(scanner.tokens.len(), 18);
        assert_eq!(scanner.tokens.last().unwrap().token_type(), &TokenType::Eof);
        let token_types: Vec<&TokenType> = scanner.tokens.iter().map(|t| t.token_type()).collect();
        assert_eq!(
            token_types,
            vec![
                &TokenType::LeftBrace,
                &TokenType::LeftParen,
                &TokenType::String("Doris".to_owned()),
                &TokenType::Star,
                &TokenType::Number(218.),
                &TokenType::Dot,
                &TokenType::RightParen,
                &TokenType::RightBrace,
                &TokenType::Number(4711.),
                &TokenType::Plus,
                &TokenType::Number(3.1415),
                &TokenType::Number(3.1),
                &TokenType::Dot,
                &TokenType::Number(4.),
                &TokenType::Dot,
                &TokenType::Dot,
                &TokenType::Number(5.),
                &TokenType::Eof
            ]
        );
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
