
use std::io::Write;

use run::run;

/// Call the interpreter to run the code inside the prompt.
/// 
/// This is a REPL (Read-Eval-Print-Loop) that will keep
/// running until the user exits.
/// 
/// to do this, do a loop that will keep printing the prompt
/// and then reading the input from the user. Then, run the
/// code inside the interpreter.  
/// 
/// Break the loop if the line is empty or if the user
/// types `exit` or `quit`.
/// 
/// At the very beginning, tell the user that they can exit by
/// typing `exit` or `quit`.
pub fn run_prompt() {
    println!("Running. Type 'exit' or 'quit' to exit");

    loop {

        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" || input.trim() == "quit" {
            break;
        }

        run(&(input.trim()));
    }
}