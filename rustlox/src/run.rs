//! Contains the run function



/// Run the code inside the interpreter.
/// 
/// for now, just print the code.
/// 
/// This will be used by `run_file` and `run_prompt`.
pub fn run(code: &str) {
    let mut myscanner = scanner::scanner_struct::Scanner {
        source: code.to_string(),
        tokens: Vec::new(),
        start: 0,
        current: 0,
        line: 0,
    };
    scanner::scan_tokens::scan_tokens(&mut myscanner);
    println!("Running code:\n{}", code);
}

