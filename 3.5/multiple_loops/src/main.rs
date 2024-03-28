fn main() {
    let mut count = 0;
    // 'counting_up is the label for outer loop.
    // You can optionally specify a label on loop then you can break or continue.
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // While loop.
    let mut number = 3;
    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // While loop through array.
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value at index {} is {}", index, a[index]);

        index += 1;
    }

    let a = [1, 2, 3, 4, 5];

    // For loop.
    for element in a {
        println!("the value is: {element}");
    }

    // Countdown in for loop.
    // (1..4) is a range, provided bbu standard library, which generates all numbers in sequence
    // starting from one number and ending BEFORE another number.
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!")
}
