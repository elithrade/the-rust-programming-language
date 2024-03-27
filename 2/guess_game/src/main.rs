use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // By default variables are immutable, adding mut make it muttable.
        let mut guess = String::new();

        io::stdin()
            // & indicate the argument is a reference which means multiple parts of the code
            // access the same variable without copying the data multiple times.
            .read_line(&mut guess)
            // read_line returns Result type which can be in multiple states, the Ok state idicates the
            // operation was successful, Err states meaning the operation has failed.
            // If Result is Err, expect will cause the program to crash and display the message passed
            // in.
            .expect("Failed to read line");

        // guess variable here is called Shadowing, it lets us to reuse the variable.
        // trim() to remove the newline character in the input string.
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        //
        // Handling invalid input. We switched from expect call to a match expression to move from
        // crashing on error to handling the error.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // A match expression made up of arms.
        // An arm consist of a pattern to match against, and the code that should be run if the value
        // given to match fits the pattern.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // Exiting the loop when match euqals.
                break;
            }
        }

        println!("You guessed: {guess}");
    }
}
