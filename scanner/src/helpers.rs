//! the scanner
//! 



use token::token::Token;
use token::token_type::TokenType;

fn add_token_literal_with_literal(
    scanner: &mut crate::scanner_struct::Scanner,
    token_type: TokenType,
    literal: token::token::Literal
) {

    let lexeme = scanner.source[scanner.start..scanner.current].to_string();
    scanner.tokens.push(Token::new(
        token_type,
        lexeme,
        literal,
        scanner.line,
    ));

}

/// The add_token method that adds a token to the tokens.
/// 
/// This method is called by `scan_token` and is responsible for
/// adding a token to the tokens.
pub fn add_token(
    scanner: &mut crate::scanner_struct::Scanner,
    token_type: TokenType
) {
    add_token_literal_with_literal(scanner, token_type, token::token::Literal::NoLexeme);
}


pub fn match_next(scanner: &mut crate::scanner_struct::Scanner, expected: char) -> Result<(),()> {
    //! Checks if the next character matches the expected character.
    //! - If it does, advances the scanner and returns `Ok(())`.
    //! - If it does not, returns `Err(())`.

    match scanner.current_char {
        Some(c) => {
            if c == expected {
                scanner.advance();
                return Ok(());
            }
            else {
                return Err(());
            }
        },
        None => {
            return Err(());
        }
    
    }

}
