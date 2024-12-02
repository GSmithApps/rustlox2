//! Contains the run function

use scanner::scanner_struct::Scanner;
use token::token::Token;

/// Run the code inside the interpreter.
/// 
/// for now, just print the code.
/// 
/// This will be used by `run_file` and `run_prompt`.
pub fn run(code: &str) {
    let mut myscanner = Scanner::new(code);
    let mut tokens: Vec<Token> = Vec::new();
    myscanner.scan_tokens(&mut tokens);
    println!("Running code:\n{}", code);
    println!("Tokens: {:?}", tokens);
}

