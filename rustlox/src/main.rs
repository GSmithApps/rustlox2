/* There is a main function that is the starting point.
From the main function, it can either do `run_file` or
`run_prompt` -- both of which are simply entrypoints into
the `run` function. */


use std::env::args;
use std::path::PathBuf;

use run_file::run_file;

use run_prompt::run_prompt;

fn main() {
    /* get the command line args.
    - If there is more than one (in addition to
      the first arg, which is the file path), then we
      want to break and tell the user we only want one arg.
    - if there is only one, then find that file and run the
      code inside it.
    - if there are no arguments, then run rustlox as an interpreter */
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        println!("Usage: rustlox [script]");
    } else if args.len() == 2 {
        let path: PathBuf = PathBuf::from(&args[1]);
        run_file(&path);
    } else {
        run_prompt();
    }
}
