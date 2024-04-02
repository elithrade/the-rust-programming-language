fn main() {
    test_1();

    test_2();

    test_3();

    test_4();
}

fn test_1() {
    println!("--- test_1 ---");
    let mut s = String::from("hello world");

    let len = first_word_1(&s);

    println!("{len}");

    s.clear();

    // len is stil 5 after s is cleared.
    println!("{len}");
}

fn first_word_1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn test_2() {
    println!("--- test_2 ---");
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello}");
    println!("{world}");
}

fn test_3() {
    println!("--- test_3 ---");
    let mut s = String::from("hello world");

    // s is borrowed as immutable in the function.
    let word = first_word_2(&s);

    // s.clear() needs a mutable reference in order to call clear().
    // We cannot mix immutable and mutable in the same scope.
    // s.clear(); // Compile error!

    println!("the first word is: {}", word);
}

fn test_4() {
    println!("--- test_4 ---");
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word_3(&my_string[0..6]);
    println!("slices: {word}");

    let word = first_word_3(&my_string[..]);
    println!("slices: {word}");

    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_3(&my_string);
    println!("&String: {word}");

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word_3(&my_string_literal[0..6]);
    println!("slices of string literal: {word}");

    let word = first_word_3(&my_string_literal[..]);
    println!("slices of string literal: {word}");

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_3(my_string_literal);
    println!("string literals are string slices: {word}");
}

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// By changing &String to &str we allow &Strin, &str and string literal passed in.
fn first_word_3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
