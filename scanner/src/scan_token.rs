use crate::helpers::advance_and_return_previous_char;
use crate::helpers::add_token;
use crate::helpers::match_char;
use token::token_type::TokenType;



/// The scan_token method that scans a token.
/// 
/// This is the main method and purpose of the scanner.
/// 
/// This method is called by `scan_tokens` and is responsible for
/// scanning a single token.
pub fn scan_token(scanner: &mut crate::scanner_struct::Scanner) {
    let c = advance_and_return_previous_char(scanner);

    // handle comments
    if c == '/' {
        if match_char(scanner, '/') {
            // A comment goes until the end of the line.
            while scanner.source.chars().nth(scanner.current).unwrap() != '\n' && !crate::helpers::is_at_end(&*scanner) {
                scanner.current += 1;
            }
        } else {
            add_token(scanner, TokenType::Slash);
        }
        return;
    }

    // if c is a double quote then we have a string

    // if c is a digit then we have a number

    // if c is a letter then we have a keyword or identifier




    match c {
        '(' => add_token(scanner, TokenType::LeftParen),
        ')' => add_token(scanner, TokenType::RightParen),
        '{' => add_token(scanner, TokenType::LeftBrace),
        '}' => add_token(scanner, TokenType::RightBrace),
        ',' => add_token(scanner, TokenType::Comma),
        '.' => add_token(scanner, TokenType::Dot),
        '-' => add_token(scanner, TokenType::Minus),
        '+' => add_token(scanner, TokenType::Plus),
        ';' => add_token(scanner, TokenType::Semicolon),
        '*' => add_token(scanner, TokenType::Star),
        '!' => {
            let token_type = if match_char(scanner, '=') {
                TokenType::BangEqual
            } else {
                TokenType::Bang
            };
            add_token(scanner, token_type);
        },
        '=' => {
            let token_type = if match_char(scanner, '=') {
                TokenType::EqualEqual
            } else {
                TokenType::Equal
            };
            add_token(scanner, token_type);
        },
        '<' => {
            let token_type = if match_char(scanner, '=') {
                TokenType::LessEqual
            } else {
                TokenType::Less
            };
            add_token(scanner, token_type);
        },
        '>' => {
            let token_type = if match_char(scanner, '=') {
                TokenType::GreaterEqual
            } else {
                TokenType::Greater
            };
            add_token(scanner, token_type);
        },
        _ => {
            // crate::run_time_error::run_time_error(scanner.line, "Unexpected character.".to_string());
        }
    }
}

