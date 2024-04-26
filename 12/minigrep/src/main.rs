use std::{env, process};

use minigrep::{run, Config};

fn main() {
    // collect() turns iterator into a vector.
    let args: Vec<String> = env::args().collect();

    // If value is error the method calls the code inside 'closure'.
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // if let rather than unwrap_or_else to check errors from run function.
    // Since run doesnot return value we only handles the case when there is an error.
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
