use strum_macros::Display;

#[derive(Debug, Display)]
#[allow(unused)]
pub enum TokenType {
    // Single Character Tokens
    LeftParen(String),
    RightParen(String),
    LeftBrace(String),
    RightBrace(String),
    Comma(String),
    Dot(String),
    Minus(String),
    Plus(String),
    Semicolon(String),
    Slash(String),
    Star(String),

    // One or two character tokens
    Bang(String),
    BangEqual(String),
    Equal(String),
    EqualEqual(String),
    Greater(String),
    GreaterEqual(String),
    Less(String),
    LessEqual(String),

    // Literals
    Identifier(String),
    String(String),
    Number(String),

    // Keywords
    And(String),
    Class(String),
    Else(String),
    False(String),
    Fun(String),
    For(String),
    If(String),
    Nil(String),
    Or(String),
    Print(String),
    Return(String),
    Super(String),
    This(String),
    True(String),
    Var(String),
    While(String),

    Eof,
}
