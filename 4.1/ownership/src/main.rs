fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");
    println!("{}", s);

    // s is string literal, stored on stack in memory, cannot be mutated.
    // let s = "hello";
    // s.push_str(", world");

    // This is "double free" error, Rust consider s1 is no longer valid.
    // The concept of copying the pointer, length, and capacity without
    // copying the data probably sounds like making a shallow copy.
    // But because Rust also invalidates the first variable,
    // instead of being called a shallow copy, it’s known as a move.
    // let s1 = String::from("hello");
    // let s2 = s1;
    //
    // println!("{} world!", s1)

    let s1 = String::from("hello");
    // clone() makes a deep copy of the heap data in memory.
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let mut s1 = String::from("hello");
    let s2 = s1.clone();
    // Mutating s1 resulting heap memory mutated.
    s1.push_str(", world!");

    // s2 still pointing to the heap memory of "hello".
    println!("s1 = {}, s2 = {}", s1, s2);
    // Output: s1 = hello, world!, s2 = hello

    // Stack only data
    // Types such as integer that have known size at compile time are stored on the stack, so
    // copies of the values are quick to make.
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // Mutating x doesn't change y.
    let mut x = 5;
    let y = x;
    x = 10;

    println!("x = {}, y = {}", x, y);

    // Ownership and functions
    // s comes into scope.
    let s = String::from("hello");
    // This will not compile, becuase ownership changed from s to t and we are using s in
    // takes_ownership function below.
    // let t = s;

    takes_ownership(s);
    // s's value moves into a function and no longer valid.
    // println!("{s}");

    let x = 5;

    makes_copy(x);

    // Return values and scope
    let s1 = gives_ownership();
    println!("s1: {s1}");

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
    // d2 moved, out of scope.
    // println!("s2: {s2}");

    println!("s3: {s3}");

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}", s2, len);
}

fn calculate_length(s1: String) -> (String, usize) {
    let len = s1.len();
    (s1, len)
}

fn takes_and_gives_back(s2: String) -> String {
    s2
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn makes_copy(x: i32) {
    println!("{}", x)
}

fn takes_ownership(s: String) {
    println!("{}", s);
}
