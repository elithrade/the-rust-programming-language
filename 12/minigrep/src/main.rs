use std::{env, process};

use minigrep::{run, Config};

fn main() {
    // If value is error the method calls the code inside 'closure'.
    // Chapter 13.3: Change build method to take an iterator to avoid cloning.
    // env.args will be consumed bu build method.
    let config = Config::build(env::args()).unwrap_or_else(|err| {
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
