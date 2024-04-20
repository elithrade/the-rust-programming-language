fn main() {}

fn lifetime_1() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn lifetime_2() {
    let string1 = String::from("long string is long");
    let result;

    {
        let string2 = String::from("xyz");
        // For result to be valid in println! string2 must be valid until the end of the inner
        // scope.
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

// Note the function takes string slices which are references because we don't want the function to
// take ownership of its parameters.
// To user lifetime annotations in function signature, we need to declare the generic lifetime
// parameter ' inside angle bracket. The returned reference will be valid as long as both
// parameters are valid.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
