use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let _username = read_username_from_file().expect("Error reading username");
    let _username = read_username_from_file_short().expect("Error reading username");
    let _username = read_username_from_file_shorter().expect("Error reading username");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    // ? works the same way as match.
    // If Ok the result will be returned, otherewise Err will be returned.
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();

    // ? can only be used in function returns Result or Option.
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
