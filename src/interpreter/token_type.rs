use strum_macros::Display;

#[derive(Debug, Display, PartialEq, strum_macros::EnumString, strum_macros::VariantNames)]
#[allow(unused)]
pub enum TokenType {
    // Single Character Tokens
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

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords
    #[strum(serialize = "and")]
    And,

    #[strum(serialize = "class")]
    Class,

    #[strum(serialize = "else")]
    Else,

    #[strum(serialize = "false")]
    False,

    #[strum(serialize = "fun")]
    Fun,

    #[strum(serialize = "for")]
    For,

    #[strum(serialize = "if")]
    If,

    #[strum(serialize = "nil")]
    Nil,

    #[strum(serialize = "or")]
    Or,

    #[strum(serialize = "print")]
    Print,

    #[strum(serialize = "return")]
    Return,

    #[strum(serialize = "super")]
    Super,

    #[strum(serialize = "this")]
    This,

    #[strum(serialize = "true")]
    True,

    #[strum(serialize = "var")]
    Var,

    #[strum(serialize = "while")]
    While,

    Eof,
}
