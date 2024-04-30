use std::ops::Deref;

fn main() {
    let x = 5;
    let y = x;

    assert_eq!(5, x);
    assert_eq!(5, y);

    // Change y to a pointer.
    let y = &x;
    // Won't compile as comparing integer to pointer to integer.
    // assert_eq!(5, y);
    // Deref to get the value.
    assert_eq!(5, *y);

    let y = Box::new(x);
    // Dereferencing y, the main difference is that here we set y to be an instance of a Box<T>
    // pointing to a copied value of x rather than a reference pointing to the value of x.
    assert_eq!(5, *y);

    let y = MyBox::new(x);
    // MyBox can be dereferenced because we implemented Deref.
    assert_eq!(5, *y);

    // Deref coercion converts a reference to a type that implements the Deref trait into a
    // reference to another type.
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // Syntax defines an associated type for the Deref trait to use.
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // .0 accesses the first value in a tuple struct.
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
