//! Contains the `run_file` function
//! 
//! Requires on the `run` function.

use std::path::PathBuf;
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
pub fn run_file(path_string: &str) {

    let mut i = 1;

    let mut string_from_file: Result<String, std::io::Error>;

    loop {
        if i == 1 {

            let path: PathBuf = PathBuf::from(path_string);
            string_from_file = std::fs::read_to_string(path);

        } else {

            let lines_of_statements_in_response = vec![
                "There was an error reading the file.",
                "Please try again.",
                "Type 'exit' or 'quit' to exit.",
                // &format!("{:?}", e),
            ];
            utilities::print_with_surrounding_box::print_with_surrounding_box(lines_of_statements_in_response);
        
            print!("> ");
            std::io::stdout().flush().unwrap();
        
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
        
            if input.trim() == "exit" || input.trim() == "quit" {
                break;
            }
            string_from_file = std::fs::read_to_string(input.trim());

        }

        i = i + 1;

        match string_from_file {
            Ok(string_from_file) => {
                crate::run::run(&string_from_file);
                break;
            },
            Err(e) => {
                continue;
            }
        }
    }

}

