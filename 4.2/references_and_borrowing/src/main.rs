fn main() {
    let s1 = String::from("hello");

    // let len = calculate_length_take_ownership(s1);

    // This is not allowed as s1 ownership moved to calculate_length function.
    // println!("The length of '{}' is '{}'", s1, len);

    // By passing s1's reference to function without taking onwership.
    let len = calculate_length(&s1);

    println!("The length of '{}' is '{}'", s1, len);

    let s2 = String::from("hello");

    // change_reference(&s2);

    // Mutable references.
    let mut s2 = String::from("hello");

    change(&mut s2);

    // Will print "hello world!"
    println!("{s2}");

    // Mutable references have one big restriction: if you have a mutable references
    // to a value, you can have no other references to that value.
    // This code that attempts to create two mutable references to s will fail:
    let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;
    // The benefit of having this restriction is that Rust can prevent data races at compile time.
    // println!("{}, {}", r1, r2);
    //
    // combining mutable and immutable references is not allowed.
    //
    //let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // Not allowed
    //
    // println!("{}, {}, and {}", r1, r2, r3);
    //
    // Note that reference's scope starts from where it is created and continue through the last
    // time that reference is used.
    //
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    let r3 = &mut s;

    println!("{}", r3);

    // Dangling reference is not allowed.
    // let reference_to_nothing = dangle();
}

// fn dangle() -> &String {
//     // s is a new string;
//     let s = String::from("hello");
//
//     // s goes out of scope here, the reference to memory will be dropped.
//     &s
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn change_reference(some_string: &String) {
    // References are immutable by default.
    // some_string.push_str(", world!");
}

// s1 a reference to a string.
fn calculate_length(s1: &String) -> usize {
    s1.len()
}

fn calculate_length_take_ownership(s1: String) -> usize {
    s1.len()
}
