use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}
fn main() {
    // 5 is allocated on heap, we can access the data in the box similar to how we would if this
    // data were on stack.
    let b = Box::new(5);
    println!("b = {}", b);

    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
