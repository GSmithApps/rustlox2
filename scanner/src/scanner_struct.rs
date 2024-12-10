//! Contains the `Scanner` struct.

use token::token::{Token, Literal};
use token::token_type::TokenType;



enum ScanTokenResult {
    /// The token was found.
    /// 
    /// This is the normal case.
    TokenFound(TokenType),

    /// A token was found and added to the `tokens` vector.
    /// 
    /// This is used when we want to customize the way the token from the scan pass
    /// is added to the token vector.
    TokenFoundAndAdded,

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

    pub tokens: Vec<Token>,

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
            tokens: Vec::new(),
            start: 0,
            current: 0,
            current_char: source.chars().nth(0),
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) {
        //! The scan_tokens method that scans the tokens.
        //!
        //! This is the main method and purpose of the scanner.

        loop {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            match self.scan_token() {
                ScanTokenResult::TokenFound(token_type) => {

                    let lexeme = self.source[self.start..self.current].to_string();
                    self.tokens.push(Token::new(token_type, lexeme, Literal::NoLiteral, self.line));
                }
                ScanTokenResult::EndOfFile => {
                    break;
                }
                ScanTokenResult::NoTokenFromScanPass => {
                    // do nothing
                }
                ScanTokenResult::TokenFoundAndAdded => {
                    // do nothing
                }
            }
        }

        // We are at the end of the file.
        self.tokens.push(Token::new(
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

                    // if c is an open quote then we have a string
                    '«' => {
                        self.advance(1);
                        loop {
                            match self.current_char {
                                None => {
                                    // crate::run_time_error::run_time_error(self.line, "Unterminated string.".to_string());
                                    return ScanTokenResult::EndOfFile;
                                }
                                Some(current_char) if current_char == '»' => {
                                    self.advance(1);
                                    break;
                                }
                                Some(_) => {
                                    self.advance(1);
                                }
                            }
                        }
                        let literal = self.source[self.start..self.current].to_string();
                        self.tokens.push(Token::new(
                            TokenType::String,
                            format!("«{}»", literal),
                            Literal::String(literal),
                            self.line,
                        ));
                        ScanTokenResult::TokenFoundAndAdded
                    }

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
                    '⁻' => {
                        self.advance(1);
                        ScanTokenResult::TokenFound(TokenType::Negative)
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
                    '≤' => {
                        self.advance(1);
                        ScanTokenResult::TokenFound(TokenType::LessEqual)
                    }
                    '<' => {
                        self.advance(1);
                        ScanTokenResult::TokenFound(TokenType::Less)
                    }
                    '≥' => {
                        self.advance(1);
                        ScanTokenResult::TokenFound(TokenType::GreaterEqual)
                    }
                    '>' => {
                        self.advance(1);
                        ScanTokenResult::TokenFound(TokenType::Greater)
                    }
                    '≠' => {
                        self.advance(1);
                        ScanTokenResult::TokenFound(TokenType::NotEqual)
                    }
                    '¬' => {
                        self.advance(1);
                        ScanTokenResult::TokenFound(TokenType::Not)
                    }
                    '≟' => {
                        self.advance(1);
                        ScanTokenResult::TokenFound(TokenType::EqualityTest)
                    }
                    '←' => {
                        self.advance(1);
                        ScanTokenResult::TokenFound(TokenType::Assignment)
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

    /// increment the current index and current character by `increment_by`.
    pub fn advance(&mut self, increment_by: usize) {

        self.current += increment_by;
        self.current_char = self.source.chars().nth(self.current);
    }
}
