//! There is a main function that is the starting point.
//! From the main function, it can either do `run_file` or
//! `run_prompt` -- both of which are simply entry points into
//! the `run` function.


mod run_prompt;
mod run_file;
mod run;

use std::env::args;


/// Get the command line args.
/// - If there is more than one (in addition to
///   the first arg, which is the file path), then we
///   want to break and tell the user we only want one arg.
/// - if there is only one, then find that file and run the
///   code inside it.
/// - if there are no arguments, then run rustlox as an interpreter
fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        let args_string = format!("[{}]", args[1..].join(", "));
        let first_arg = format!("[{}]", args[1]);
        let lines_of_statements_in_response = vec![
            "We received too many parameters (the limit is 1).  We received:",
            &args_string,
            "but it should be something like:",
            &first_arg,
        ];

        utilities::print_with_surrounding_box::print_with_surrounding_box(lines_of_statements_in_response)

    } else if args.len() == 2 {
        run_file::run_file(&args[1]);
    } else {
        run_prompt::run_prompt();
    }
}

