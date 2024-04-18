fn main() {
    // New string.
    let mut _s = String::new();
    let data = "Initial commit";
    let s = data.to_string();
    println!("{s}");

    // Update.
    let mut s1 = String::from("foo");
    let s2 = "bar";
    // push_str takes a string slice because we don't want to take the ownership.
    s1.push_str(s2);
    println!("s2 is {s2}");
    println!("s1 is {s1}");

    let mut s = String::from("lo");
    s.push('l');
    println!("After pushing 'l' to 'lo' {s}");

    // String concat.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // The + operator uses the add method, whose signature looks something like this:
    // fn add(self, s: &str) -> String
    // s1 is no longer valid after +.
    let s3 = s1 + &s2;
    println!("s3 is {s3}");

    // User format! to concat strings.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("format! concat {s}");

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("[0..4] of Здравствуйте is {s}");

    // The best way to operate on pieces of strings is to be explicit about whether
    // you want characters or bytes.
    for c in "Зд".chars() {
        // Iterating over chars.
        println!("{c}");
    }
    for b in "Зд".bytes() {
        // Iterating over bytes.
        println!("{b}");
    }
}
