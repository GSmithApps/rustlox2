//! There is a main function that is the starting point.
//! From the main function, it can either do `run_file` or
//! `run_prompt` -- both of which are simply entry points into
//! the `run` function.


use std::env::args;
// use std::path::PathBuf;

// use run_file::run_file;

use run_prompt::run_prompt;

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

        print_with_surrounding_box(lines_of_statements_in_response);

    } else if args.len() == 2 {
        // let path: PathBuf = PathBuf::from(&args[1]);
        // run_file(&path);
    } else {
        run_prompt();
    }
}

/// Print the lines with a surrounding box.
/// 
/// For example, if the lines are:
/// 
/// ```text
/// [
///   "We received too many parameters (the limit is 1).  We received:",
///   "[grant, smith]",
///   "but it should be something like:",
///   "[grant]",
/// ]
/// ```
/// 
/// it will print:
/// 
/// ```text
///  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
///  @@                                                                   @@
///  @@  We received too many parameters (the limit is 1).  We received:  @@
///  @@  [grant, smith]                                                   @@
///  @@  but it should be something like:                                 @@
///  @@  [grant]                                                          @@
///  @@                                                                   @@
///  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
/// ```
fn print_with_surrounding_box(lines_of_statements_in_response: Vec<&str>) -> () {
    // map or iterate through the vector of lines and return the greatest length
    let max_len = lines_of_statements_in_response.iter().map(|line| line.len()).max().unwrap();

    // make a vector of filler spaces to add to the end of each line
    let filler_spaces_for_statements = lines_of_statements_in_response.iter().map(|line| std::iter::repeat(' ').take(max_len - line.len()).collect::<String>()).collect::<Vec<String>>();

    let at_signs = std::iter::repeat('@').take(max_len).collect::<String>();
    let spaces = std::iter::repeat(' ').take(max_len).collect::<String>();
    println!();
    println!(" @@@@{}@@@@", at_signs);
    println!(" @@  {}  @@", spaces);
    for (line, filler) in lines_of_statements_in_response.iter().zip(filler_spaces_for_statements.iter()) {
        println!(" @@  {}{}  @@", line, filler);
    }
    println!(" @@  {}  @@", spaces);
    println!(" @@@@{}@@@@", at_signs);
    println!();
}
