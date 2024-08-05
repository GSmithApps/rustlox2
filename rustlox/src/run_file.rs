//! Contains the `run_file` function
//! 
//! Requires on the `run` function.

use std::path::{self, PathBuf};
use std::io::Write;


/// Run the code from a file.
/// 
/// Get the file and pass the code to the `run` function.
/// 
/// This happens if the user passes an argument into the
/// command line. I say "an" argument because if it's zero,
/// it falls elsewhere (into the `run_prompt` function), and
/// if it's more than one, it falls elsewhere (into a message
/// that tells the user they can only pass one argument).
/// 
/// If the user's argument does not give a valid string from
/// the file, we'll go into a loop asking the user to try
/// again.  If they type in `exit` or `quit`, then we'll
/// break the loop.
pub fn run_file(first_path_string: &str) {

    let mut is_first_time = true;

    let mut string_from_file: Result<String, std::io::Error>;

    let mut path_string: String;

    loop {
        if is_first_time {

            path_string = first_path_string.to_string();

            let path: PathBuf = PathBuf::from(&path_string);
            string_from_file = std::fs::read_to_string(path);

            is_first_time = false;

        } else {

            print!("> ");
            std::io::stdout().flush().unwrap();

            path_string = String::new();
            
            std::io::stdin().read_line(&mut path_string).unwrap();
            
            if path_string.trim() == "exit" || path_string.trim() == "quit" {
                break;
            }
            string_from_file = std::fs::read_to_string(path_string.trim());
            
        }
        
        match string_from_file {
            Ok(string_from_file) => {
                crate::run::run(&string_from_file);
                let lines_of_statements_in_response = vec![
                    "That seemed to work!",
                    "If you want to run more, pass another file path.",
                    "Type 'exit' or 'quit' to exit.",
                ];
                utilities::print_with_surrounding_box::print_with_surrounding_box(lines_of_statements_in_response);
            },
            Err(e) => {
                let error_statement = format!("{:?}", e);
                let lines_of_statements_in_response = vec![
                    "There was an error reading the file:",
                    &path_string.trim(),
                    "Please try again.",
                    "Type 'exit' or 'quit' to exit.",
                    &error_statement,
                ];
                utilities::print_with_surrounding_box::print_with_surrounding_box(lines_of_statements_in_response);
            }
        }

    }
    
}

