
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    NoLexeme,
}


/// Token struct
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: crate::token_type::TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

impl Token {
    pub fn new(token_type: crate::token_type::TokenType, lexeme: String, literal: Literal,line: usize, column: usize, length: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
            column,
            length
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {} {} {}", self.token_type, self.lexeme, self.line, self.column)
    }
}

