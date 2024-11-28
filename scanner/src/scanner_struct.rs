//! Contains the `Scanner` struct.

use token::token::Token;
use token::token_type::TokenType;
use crate::helpers::before_or_on_last_char;

use crate::helpers::add_token;
use crate::helpers::match_char;

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

        while before_or_on_last_char(&*self) {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        // We are at the end of the file.
        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            token::token::Literal::NoLexeme,
            self.line,
            self.current,
            0
        ));
    }

    fn scan_token(&mut self) {
        //! The scan_token method that scans a token.
        //! 
        //! This is the main method and purpose of the scanner.
        //! 
        //! This method is called by `scan_tokens` and is responsible for
        //! scanning a single token.

        // We are at the beginning of the next lexeme.

        match self.current_char {
            None => {
                // We are at the end of the file.
                return;
            },
            Some(c) => {


                // handle comments and division
                if c == '/' {

                    // if the lead is a slash, then we're certainly consuming
                    // at least one character, so we can increment the current
                    self.advance();

                    match self.current_char {
                        // if the next character is none, then we're at the end of the file,
                        // and we can add a token for the slash and return.
                        // in reality, this probably won't happen because there's no
                        // reason to end a file in a division sign.
                        None => {
                            add_token(self, TokenType::Slash);
                            return;
                        },
                        Some(current_char) => {
                            // if the next character is a slash, then we have a comment
                            // and we need to consume the rest of the line
                            if current_char == '/' {
                                //increment to consume the second slash
                                self.advance();

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
                                    
                                    match self.current_char {
                                        None => {
                                            return;
                                        },
                                        Some(current_char) => {
                                            if current_char == '\n' {
                                                return;
                                            }
                                        }
                                    }
                                    self.advance();
                                }
                            } else {
                                add_token(self, TokenType::Slash);
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
                    '(' => add_token(self, TokenType::LeftParen),
                    ')' => add_token(self, TokenType::RightParen),
                    '{' => add_token(self, TokenType::LeftBrace),
                    '}' => add_token(self, TokenType::RightBrace),
                    ',' => add_token(self, TokenType::Comma),
                    '.' => add_token(self, TokenType::Dot),
                    '-' => add_token(self, TokenType::Minus),
                    '+' => add_token(self, TokenType::Plus),
                    ';' => add_token(self, TokenType::Semicolon),
                    '*' => add_token(self, TokenType::Star),
                    '!' => {
                        let token_type = if match_char(self, '=') {
                            TokenType::BangEqual
                        } else {
                            TokenType::Bang
                        };
                        add_token(self, token_type);
                    },
                    '=' => {
                        let token_type = if match_char(self, '=') {
                            TokenType::EqualEqual
                        } else {
                            TokenType::Equal
                        };
                        add_token(self, token_type);
                    },
                    '<' => {
                        let token_type = if match_char(self, '=') {
                            TokenType::LessEqual
                        } else {
                            TokenType::Less
                        };
                        add_token(self, token_type);
                    },
                    '>' => {
                        let token_type = if match_char(self, '=') {
                            TokenType::GreaterEqual
                        } else {
                            TokenType::Greater
                        };
                        add_token(self, token_type);
                    },
                    _ => {
                        // crate::run_time_error::run_time_error(self.line, "Unexpected character.".to_string());
                    }
                }

                self.advance();

            }
        }
    }

    pub fn advance(&mut self) {
        //! some stuff
        self.current += 1;
        self.current_char = self.source.chars().nth(self.current);
    }

}
