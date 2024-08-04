//! There is a main function that is the starting point.
//! From the main function, it can either do `run_file` or
//! `run_prompt` -- both of which are simply entry points into
//! the `run` function.


use std::env::args;
use std::path::PathBuf;


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
        let temp_example = &format!("[{}]", args[1..].join(", ").as_str());
        let temp_example_2 = &format!("[{}]", args[1].as_str());
        let lines_of_statements_in_response = vec![
            "We received too many parameters (the limit is 1).  We received:",
            temp_example.as_str(),
            "but it should be something like:",
            temp_example_2.as_str(),
        ];

        utilities::print_with_surrounding_box::print_with_surrounding_box(lines_of_statements_in_response)

    } else if args.len() == 2 {
        let path: PathBuf = PathBuf::from(&args[1]);
        run_file::run_file(&path);
    } else {
        run_prompt::run_prompt();
    }
}

