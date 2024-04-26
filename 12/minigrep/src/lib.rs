use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return the errors from current function for the caller to handle.
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

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

// Explicit lifetime 'a in the signature of function and use the lifetime with the contents
// argument and return value. The argument's lifetime is connected to the lifetime of the return
// value. The data returned by search function will live as long as the data passed into the search
// function in the contents argument.
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents));
    }
}
