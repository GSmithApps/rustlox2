//! Contains the run function

use scanner::scanner_struct::Scanner;

/// Run the code inside the interpreter.
/// 
/// for now, just print the code.
/// 
/// This will be used by `run_file` and `run_prompt`.
pub fn run(code: &str) {
    let mut myscanner = Scanner::new(code);
    myscanner.scan_tokens();
    println!("Running code:\n{}", code);
}

