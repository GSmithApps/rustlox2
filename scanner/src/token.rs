
/// Token struct
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: crate::token_type::TokenType,
    pub lexeme: String,
    // pub literal: Option<Literal>,
    pub line: usize,
}


impl Token {
    pub fn new(token_type: crate::token_type::TokenType, lexeme: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {}", self.token_type, self.lexeme)
    }
}
