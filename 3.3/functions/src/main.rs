fn main() {
    println!("Hello world!");

    another_function();

    print_labeled_measurement(5, 'h');

    assign_does_not_return_value();

    expression();

    let x = five();

    println!("The value of x is {x}");

    let x = plus_one(5);

    println!("The value of x is {x}");
}

fn another_function() {
    println!("Another function");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn assign_does_not_return_value() {
    // You cannot write x = y = 6 and have both x and y have the value 6.
    // let x = (let y = 6);
}

fn expression() {
    // Curly bracekts is an expression.
    // This block evaluates to 4 and bound to y.
    // Expressions do not include ending semicolins, if you add semicolon you turn it into a
    // statement and will not return a value.
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}")
}

fn five() -> i32 {
    // -> specify the return type.
    // No semicolon beacuse it's an expression whose value we want to return.
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
