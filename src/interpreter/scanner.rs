use super::token::Token;

#[derive(Debug, Default)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.into(),
            line: 1,
            ..Default::default()
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.isAtEnd() {
            self.start = self.current;
            todo!("scan_token()");
        }
        self.tokens.push(Token::new(super::token_type::TokenType::Eof, None, self.line));
        &self.tokens
    }

    fn scan_token(&mut self) {
        
    }

    fn isAtEnd(&self) -> bool {
        self.current >= self.source.len()
    }
}
