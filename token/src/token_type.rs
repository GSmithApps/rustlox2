

/// An enum for token types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Negative,
    Plus,
    Semicolon,
    Slash,
    Star,
    Not,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // one or two character tokens
    Equal,
    EqualEqual,

    // literals
    Identifier,
    String,
    Number,

    // keywords
    And,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    True,
    Var,
    While,

    // end of file
    EOF,

}