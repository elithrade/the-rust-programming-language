// Define struct.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Define method on the struct.
impl Rectangle {
    // & indicates the method borrows the reference, not taking the ownership.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        // Self in the return type and body are aliases for the type that appear after "impl".
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Declare the struct with initial values.
    let rect1 = Rectangle {
        width: 20,
        height: 50,
    };

    // Calling method area() on Rectangle.
    println!("The area of rectangle is {} square pixels.", rect1.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Calling method that doesn't require &self by using :: (static medhot).
    let sq = Rectangle::square(3);

    println!(
        "Rectangle::square(3) width: {} height: {}, sq.area: {}",
        sq.width,
        sq.height,
        sq.area()
    );
}
