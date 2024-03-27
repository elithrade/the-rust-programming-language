fn main() {
    // This program first creates a tuple and binds it to the variable tup.
    // It then uses a pattern with let to take tup and turn it into three separate variables, x, y, and z.
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access.
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("The value of x.0 is: {five_hundred}");
    println!("The value of x.1 is: {six_point_four}");
    println!("The value of x.2 is: {one}");
}
