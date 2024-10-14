use std::{fmt, path::Display};

use super::token_type::TokenType;


#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, literal: Option<Literal>, line: usize) -> Self {
        Self { token_type, literal, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.token_type.to_string(), self.literal)
    }
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Numeric(String),
}