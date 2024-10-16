use std::fmt;

use super::token_type::TokenType;


#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType,line: usize) -> Self {
        Self { token_type, line }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }
    
    pub fn line(&self) -> usize {
        self.line
    }


}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.token_type)
    }
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Numeric(String),
}