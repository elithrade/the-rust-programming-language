// Adding the attribute to derive the Debug trait and printing the
// Rectangle instance using debug formatting.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 is {:?}", rect1);

    // we can use {:#?} instead of {:?} in the println! string.
    // In this example, using the {:#?} style will output the following:
    // Note that println! only takes the reference, not ownership.
    println!("rect1 is {:#?}", rect1);

    // dbg! macro takes the ownership, prints the line number and source code, then returns the
    // ownership.
    // We don't want dbg! to take the ownership of rect1, so we use reference to rect1.
    dbg!(&rect1);

    dbg_ownership();
}

fn dbg_ownership() {
    let scale = 2;
    let rect1 = Rectangle {
        // dgb! borrows and returns the value to width.
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("--- dbg!(&rect1) ---");
    dbg!(&rect1);

    let rect2 = scale_again(&rect1);

    println!("--- dbg!(rect1) ---");
    dbg!(rect1);

    println!("--- dbg!(rect2) ---");
    dbg!(&rect2);

    // This won't compile because the ownership is taken.
    // println!("rect1 is {:?}", rect1);
}

fn scale_again(rect1: &Rectangle) -> Rectangle {
    Rectangle {
        width: rect1.width * 2,
        height: rect1.height * 3,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
