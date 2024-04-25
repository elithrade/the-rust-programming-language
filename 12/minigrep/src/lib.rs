use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return the errors from current function for the caller to handle.
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

impl Config {
    // Error message will always be a string literal have static lifetime.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        println!("Searching for '{}'", query);
        println!("In file '{}'", file_path);

        Ok(Config { query, file_path })
    }
}
