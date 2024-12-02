//! Contains the `Scanner` struct.

use crate::helpers::match_next;
use token::token::{Token, Literal};
use token::token_type::TokenType;



/// The add_token method that adds a token to the tokens.
///
/// This method is called by `scan_token` and is responsible for
/// adding a token to the tokens.
pub fn add_token(tokens: &mut Vec<Token>, token_type: TokenType) {

}

enum ScanTokenResult {
    /// The token was found.
    /// 
    /// This is the normal case.
    TokenFound(TokenType),

    /// No token was found from this pass of ScanToken.
    /// 
    /// This is normal and happens when we're scanning a comment
    NoTokenFromScanPass,

    /// The end of the file was reached.
    EndOfFile,
}


/// The scanner struct.
#[derive(Debug, Clone, PartialEq)]
pub struct Scanner<'a> {
    /// The raw source code.
    pub source: &'a str,

    /// Works with `current` as offsets that index into
    /// the `source` string.  This field (`start`) points
    /// to the first character in the lexeme being scanned.
    pub start: usize,

    /// Works with `start` as offsets that index into
    /// the `source` string.  This field (`current`) points
    /// at the character currently being considered.
    /// The character at `current` is in
    /// current_char.
    pub current: usize,

    /// The current character being considered.
    /// is an `Option` because it can be `None` if
    /// `current` is at the end of the file.
    pub current_char: Option<char>,

    /// Tracks what source line `current` is on.
    pub line: usize,
}

impl Scanner<'_> {
    pub fn new(source: &str) -> Scanner {
        //! Create a new `Scanner`.
        Scanner {
            source,
            start: 0,
            current: 0,
            current_char: source.chars().nth(0),
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self, tokens: &mut Vec<Token>) {
        //! The scan_tokens method that scans the tokens.
        //!
        //! This is the main method and purpose of the scanner.

        loop {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            match self.scan_token() {
                ScanTokenResult::TokenFound(token_type) => {

                    let lexeme = self.source[self.start..self.current].to_string();
                    tokens.push(Token::new(token_type, lexeme, Literal::NoLiteral, self.line));
                }
                ScanTokenResult::EndOfFile => {
                    break;
                }
                ScanTokenResult::NoTokenFromScanPass => {
                    // do nothing
                }
            }
        }

        // We are at the end of the file.
        tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            token::token::Literal::NoLiteral,
            self.line,
        ));
    }

    fn scan_token(&mut self) -> ScanTokenResult {
        //! The scan_token method that scans a token.
        //!
        //! This is the main method and purpose of the scanner.
        //!
        //! This method is called by `scan_tokens` and is responsible for
        //! scanning a single token.
        //!
        //! ## Side Effects
        //!
        //! This method sometimes advances a token

        // We are at the beginning of the next lexeme.

        match self.current_char {
            None => {
                ScanTokenResult::EndOfFile
            }
            Some(c) => {
                match c {
                    // handle comments and division
                    '/' => {
                        self.advance(1);

                        match self.current_char {
                            // if the next character is none, then we're at the end of the file,
                            // and we can add a token for the slash and return.
                            // in reality, this probably won't happen because there's no
                            // reason to end a file in a division sign.
                            None => ScanTokenResult::TokenFound(TokenType::Slash),
                            Some('/') => {
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
                                    self.advance(1);
                                    match self.current_char {
                                        None => return ScanTokenResult::EndOfFile,
                                        Some(current_char) if current_char == '\n' => {
                                            return ScanTokenResult::NoTokenFromScanPass;
                                        }
                                        _ => {} // do nothing
                                    }
                                }
                            }
                            Some(_) => ScanTokenResult::TokenFound(TokenType::Slash),
                        }
                    }

                    // if c is a double quote then we have a string

                    // if c is a digit then we have a number

                    // if c is a letter then we have a keyword or identifier
                    '(' => {
                        self.advance(1);
                        ScanTokenResult::TokenFound(TokenType::LeftParen)
                    }
                    ')' => {
                        self.advance(1);
                        ScanTokenResult::TokenFound(TokenType::RightParen)
                    }
                    '{' => {
                        self.advance(1);
                        ScanTokenResult::TokenFound(TokenType::LeftBrace)
                    }
                    '}' => {
                        self.advance(1);
                        ScanTokenResult::TokenFound(TokenType::RightBrace)
                    }
                    ',' => {
                        self.advance(1);
                        ScanTokenResult::TokenFound(TokenType::Comma)
                    }
                    '.' => {
                        self.advance(1);
                        ScanTokenResult::TokenFound(TokenType::Dot)
                    }
                    '-' => {
                        self.advance(1);
                        ScanTokenResult::TokenFound(TokenType::Minus)
                    }
                    '+' => {
                        self.advance(1);
                        ScanTokenResult::TokenFound(TokenType::Plus)
                    }
                    ';' => {
                        self.advance(1);
                        ScanTokenResult::TokenFound(TokenType::Semicolon)
                    }
                    '*' => {
                        self.advance(1);
                        ScanTokenResult::TokenFound(TokenType::Star)
                    }

                    // these are two part tokens. The pattern is to do match_next
                    // then advance if it matches. This is because we want to consume
                    // the next character if it matches the second part of the token.
                    '!' => {
                        let token_type = match match_next(self, '=') {
                            Ok(()) => {
                                self.advance(2);
                                TokenType::BangEqual
                            }
                            Err(()) => {
                                self.advance(1);
                                TokenType::Bang
                            }
                        };
                        ScanTokenResult::TokenFound(token_type)
                    }
                    '=' => {
                        let token_type = match match_next(self, '=') {
                            Ok(()) => {
                                self.advance(2);
                                TokenType::EqualEqual
                            }
                            Err(()) => {
                                self.advance(1);
                                TokenType::Equal
                            }
                        };
                        ScanTokenResult::TokenFound(token_type)
                    }
                    '<' => {
                        let token_type = match match_next(self, '=') {
                            Ok(()) => {
                                self.advance(2);
                                TokenType::LessEqual
                            }
                            Err(()) => {
                                self.advance(1);
                                TokenType::Less
                            }
                        };

                        ScanTokenResult::TokenFound(token_type)
                    }
                    '>' => {
                        let token_type = match match_next(self, '=') {
                            Ok(()) => {
                                self.advance(2);
                                TokenType::GreaterEqual
                            }
                            Err(()) => {
                                self.advance(1);
                                TokenType::Greater
                            }
                        };

                        ScanTokenResult::TokenFound(token_type)
                    }
                    _ => {
                        // crate::run_time_error::run_time_error(self.line, "Unexpected character.".to_string());

                        // I don't really have this implemented yet
                        self.advance(1);
                        ScanTokenResult::NoTokenFromScanPass
                    }
                }
            }
        }
    }

    pub fn advance(&mut self, increment_by: usize) {
        //! some stuff
        self.current += increment_by;
        self.current_char = self.source.chars().nth(self.current);
    }
}
