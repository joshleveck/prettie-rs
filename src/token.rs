#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: Kind,
    pub start: usize,
    pub end: usize,
    pub value: TokenValue,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    // Special
    Eof,
    // Operators
    Plus,

    Identifier,
    // Keywords
    If,
    While,
    For,
    Let,
    Var,
    Const,
    // Primitives
    Number,
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    None,
    Number(f64),
    String(String),
}
