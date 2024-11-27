use token::token::Token;
use token::token_type::TokenType;
use crate::helpers::before_or_on_last_char;
use crate::scan_token::scan_token;

/// The scan_tokens method that scans the tokens.
/// 
/// This is the main method and purpose of the scanner.
pub fn scan_tokens(scanner: &mut crate::scanner_struct::Scanner) {
    while before_or_on_last_char(&*scanner) {
        // We are at the beginning of the next lexeme.
        scanner.start = scanner.current;
        scan_token(scanner);
    }

    // We are at the end of the file.
    scanner.tokens.push(Token::new(
        TokenType::EOF,
        "".to_string(),
        token::token::Literal::NoLexeme,
        scanner.line,
        scanner.current,
        0
    ));
}
