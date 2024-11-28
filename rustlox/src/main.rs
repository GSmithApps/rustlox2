//! There is a main function that is the starting point.
//! From the main function, it can either do `run_file` or
//! `run_prompt` -- both of which are simply entry points into
//! the `run` function.


mod run_prompt;
mod run_file;
mod run;

use std::env::args;
use std::cmp::Ordering;

/// Get the command line args.
/// - If there is more than one (in addition to
///   the first arg, which is the file path), then we
///   want to break and tell the user we only want one arg.
/// - if there is only one, then find that file and run the
///   code inside it.
/// - if there are no arguments, then run rustlox as an interpreter
fn main() {
    let args: Vec<String> = args().collect();

    match args.len().cmp(&2) {
        Ordering::Greater => {
            utilities::print_with_surrounding_box::print_with_surrounding_box(vec![
                String::from("We received too many parameters (the limit is 1).  We received:"),
                format!("[{}]", args[1..].join(", ")),
                String::from("but it should be something like:"),
                format!("[{}]", args[1]),
            ]);
        },
        Ordering::Equal => {
            run_file::run_file(&args[1]);

        },
        Ordering::Less => {
            run_prompt::run_prompt();
        },
    }

}

