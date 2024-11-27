
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

    // We are at the beginning of the next lexeme.

    let c: char = scanner.source.chars().nth(scanner.current).unwrap();

    // handle comments and division
    if c == '/' {

        // if the lead is a slash, then we're certainly consuming
        // at least one character, so we can increment the current
        scanner.current += 1;

        let current_char = scanner.source.chars().nth(scanner.current);

        match current_char {
            // if the next character is none, then we're at the end of the file,
            // and we can add a token for the slash and return.
            // in reality, this probably won't happen because there's no
            // reason to end a file in a division sign.
            None => {
                add_token(scanner, TokenType::Slash);
                return;
            },
            Some(current_char) => {
                // if the next character is a slash, then we have a comment
                // and we need to consume the rest of the line
                if current_char == '/' {
                    //increment to consume the second slash
                    scanner.current += 1;

                    // A comment goes until the end of the line.
                    // continue moving forward until we end the file
                    // or hit a new line. But stop before the new line,
                    // meaning the current character after this will be
                    // a new line, or the file will be over and there
                    // will be no more characters, and the `current` will
                    // be over the limit.
                    // if the new line is the condition that stops it (and
                    // we stop before the new line, that's good and intended
                    // because something else will process the new line.
                    // Also, notice that we don't add a token for the comment --
                    // we just move forward until the end of the line.
                    loop {
                        let current_char = scanner.source.chars().nth(scanner.current);
                        match current_char {
                            None => {
                                return;
                            },
                            Some(current_char) => {
                                if current_char == '\n' {
                                    return;
                                }
                            }
                        }
                        scanner.current += 1;
                    }
                } else {
                    add_token(scanner, TokenType::Slash);
                    // don't increment current because we already did that (consumed the single slash)
                    return;
                }
            }
        }
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

    scanner.current += 1;
}

