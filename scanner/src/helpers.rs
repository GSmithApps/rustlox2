//! the scanner
//! 



use token::token::Token;
use token::token_type::TokenType;

pub fn before_or_on_last_char(scanner: &crate::scanner_struct::Scanner) -> bool {
    scanner.current <= scanner.source.len() - 1
}

pub fn beyond_last_char(scanner: &crate::scanner_struct::Scanner) -> bool {
    scanner.current > scanner.source.len() - 1
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


/// Checks if the next character matches the expected character, and advances the scanner if it does.
pub fn match_char(scanner: &mut crate::scanner_struct::Scanner, expected: char) -> bool {
    if beyond_last_char(&*scanner) {
        return false;
    }
    // this won't panic because the above check ensures that we are not at the end.
    if scanner.source.chars().nth(scanner.current).unwrap() != expected {
        return false;
    }

    scanner.current += 1;
    true
}
