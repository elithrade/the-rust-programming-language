use std::{env, fs, process};

fn main() {
    // collect() turns iterator into a vector.
    let args: Vec<String> = env::args().collect();

    // If value is error the method calls the code inside 'closure'.
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}")
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // Error message will always be a string literal have static lifetime.
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
