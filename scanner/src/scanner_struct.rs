//! Contains the `Scanner` struct.

use token::token::Token;

/// The scanner struct.
#[derive(Debug, Clone, PartialEq)] 
pub struct Scanner {
    /// The raw source code.
    pub source: String,

    /// The tokens that have been scanned.
    pub tokens: Vec<Token>,

    /// Works with `current` as offsets that index into
    /// the `source` string.  This field (`start`) points
    /// to the first character in the lexeme being scanned.
    pub start: usize,

    /// Works with `current` as offsets that index into
    /// the `source` string.  This field (`current`) points
    /// at the character currently being considered.
    pub current: usize,

    /// Tracks what source line `current` is on.
    pub line: usize,
}
