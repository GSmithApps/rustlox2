//! the scanner
//! 



use token::token::Token;
use token::token_type::TokenType;

pub fn is_at_end(scanner: &crate::scanner_struct::Scanner) -> bool {
    scanner.current >= scanner.source.len()
}

fn add_token_literal_with_literal(
    scanner: &mut crate::scanner_struct::Scanner,
    token_type: TokenType,
    literal: token::token::Literal
) {

    let text = scanner.source[scanner.start..scanner.current].to_string();
    scanner.tokens.push(Token::new(
        token_type,
        text,
        literal,
        scanner.line,
        scanner.start,
        scanner.current - scanner.start
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


/// Checks if the next 
pub fn match_char(scanner: &mut crate::scanner_struct::Scanner, expected: char) -> bool {
    if is_at_end(&*scanner) {
        return false;
    }
    // this won't panic because the above check ensures that we are not at the end.
    if scanner.source.chars().nth(scanner.current).unwrap() != expected {
        return false;
    }

    scanner.current += 1;
    true
}

/// The advance method that advances the scanner.
/// 
/// This method is called by `scan_token` and is responsible for
/// advancing the scanner.
/// 
/// # Returns
/// 
/// A character.
/// 
/// # Panics
/// 
/// If the scanner tries to advance when it is at the end of the file.
pub fn advance_and_return_previous_char(scanner: &mut crate::scanner_struct::Scanner) -> char {
    scanner.current += 1;
    let next_character = scanner.source.chars().nth(scanner.current - 1);
    match next_character {
        Some(c) => c,
        None => {
            panic!("The scanner tried to advance when it was at the end of the file.");
        }
    }
}




