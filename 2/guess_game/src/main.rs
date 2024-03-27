use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

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
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // A match expression made up of arms.
    // An arm consist of a pattern to match against, and the code that should be run if the value
    // given to match fits the pattern.
    match guess.cmp(&secret_number) {
        std::cmp::Ordering::Less => println!("Too small!"),
        std::cmp::Ordering::Greater => println!("Too big!"),
        std::cmp::Ordering::Equal => println!("You win!"),
    }

    println!("You guessed: {guess}");
}
