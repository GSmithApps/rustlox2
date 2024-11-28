//! Contains the `Scanner` struct.

use token::token::Token;
use token::token_type::TokenType;
use crate::helpers::add_token;
use crate::helpers::match_next;

/// The scanner struct.
#[derive(Debug, Clone, PartialEq)] 
pub struct Scanner<'a> {
    /// The raw source code.
    pub source: &'a str,

    /// The tokens that have been scanned.
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

impl Scanner<'_>  {

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

        loop{
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            match self.scan_token() {
                Some(result) => {
                    match result {
                        Some(token_type) => {
                            add_token(self, token_type);
                        },
                        None => {
                            // do nothing
                        }
                    }
                },
                None => {
                    break;
                }
            }
        }

        // We are at the end of the file.
        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            token::token::Literal::NoLexeme,
            self.line,
        ));
    }

    fn scan_token(&mut self) -> Option<Option<TokenType>> {
        //! The scan_token method that scans a token.
        //! 
        //! This is the main method and purpose of the scanner.
        //! 
        //! This method is called by `scan_tokens` and is responsible for
        //! scanning a single token.
        //! 
        //! ## Returns
        //! 
        //! The method returns an `Option<Option<TokenType>>`:
        //! - the outer option is `Some` if the scanner is not at the end of the file,
        //!   and `None` if the scanner is at the end of the file.
        //! - the inner option is `Some` if a token was found, and `None` if no token was found.
        //!   For example, if it processes a comment, that wouldn't add a token, so the 
        //!   inner option would be `None`. And if it wasn't the end of the file,
        //!   the outer option would be `Some`, so it would overall return `Some(None)`.
        //! 
        //! ---
        //! 
        //! This method sometimes advances a token 

        // We are at the beginning of the next lexeme.

        match self.current_char {
            None => {
                // We are at the end of the file.
                None
            },
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
                            None => {
                                Some(Some(TokenType::Slash))
                            },
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
                                        None => {
                                            return None
                                        },
                                        Some(current_char) => {
                                            if current_char == '\n' {
                                                return Some(None);
                                            }
                                        }
                                    }
                                }
                            }
                            Some(_) => {
                                Some(Some(TokenType::Slash))
                            }
                        }
                    }

                    // if c is a double quote then we have a string

                    // if c is a digit then we have a number

                    // if c is a letter then we have a keyword or identifier

                    '(' => {self.advance(1); Some(Some(TokenType::LeftParen))},
                    ')' => {self.advance(1); Some(Some(TokenType::RightParen))},
                    '{' => {self.advance(1); Some(Some(TokenType::LeftBrace))},
                    '}' => {self.advance(1); Some(Some(TokenType::RightBrace))},
                    ',' => {self.advance(1); Some(Some(TokenType::Comma))},
                    '.' => {self.advance(1); Some(Some(TokenType::Dot))},
                    '-' => {self.advance(1); Some(Some(TokenType::Minus))},
                    '+' => {self.advance(1); Some(Some(TokenType::Plus))},
                    ';' => {self.advance(1); Some(Some(TokenType::Semicolon))},
                    '*' => {self.advance(1); Some(Some(TokenType::Star))},

                    // these are two part tokens. The pattern is to do match_next
                    // then advance if it matches. This is because we want to consume
                    // the next character if it matches the second part of the token.
                    '!' => {
                        let token_type = match match_next(self, '=') {
                            Ok(()) => {
                                self.advance(2);
                                TokenType::BangEqual
                            },
                            Err(()) => {
                                self.advance(1);
                                TokenType::Bang
                            }
                        };
                        Some(Some(token_type))
                    },
                    '=' => {

                        let token_type = match match_next(self, '=') {
                            Ok(()) => {
                                self.advance(2);
                                TokenType::EqualEqual
                            },
                            Err(()) => {
                                self.advance(1);
                                TokenType::Equal
                            }
                        };
                        Some(Some(token_type))
                    },
                    '<' => {

                        let token_type = match match_next(self, '=') {
                            Ok(()) => {
                                self.advance(2);
                                TokenType::LessEqual
                            },
                            Err(()) => {
                                self.advance(1);
                                TokenType::Less
                            }
                        };

                        Some(Some(token_type))
                    },
                    '>' => {

                        let token_type = match match_next(self, '=') {
                            Ok(()) => {
                                self.advance(2);
                                TokenType::GreaterEqual
                            },
                            Err(()) => {
                                self.advance(1);
                                TokenType::Greater
                            }
                        };

                        Some(Some(token_type))
                    },
                    _ => {
                        // crate::run_time_error::run_time_error(self.line, "Unexpected character.".to_string());

                        // I don't really have this implemented yet
                        self.advance(1);
                        None
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
