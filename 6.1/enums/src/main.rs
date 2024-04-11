fn main() {
    let some_number = Some(5);
    let some_char = Some('c');
    // None represents a invalid value or value is absent.
    let absent_number: Option<i32> = None;
    // Compiler will enforoce converting from Option<T> to T before using it.

    println!("{}", some_number.expect("panic"));
    println!("{}", some_char.expect("panic"));

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap();
    println!("x + y.unwrap() is {}", sum);

    // This will throw and stops executing.
    println!("{}", absent_number.expect("value is null"));
}
