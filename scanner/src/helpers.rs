//! the scanner
//!


pub fn match_next(scanner: &mut crate::scanner_struct::Scanner, expected: char) -> Result<(), ()> {
    //! Checks if the next character matches the expected character.
    //! - If it does, it returns `Ok(())`.
    //! - If it does not, returns `Err(())`.
    match scanner.source.chars().nth(scanner.current + 1) {
        Some(c) if c == expected => Ok(()),
        _ => Err(()),
    }
}
