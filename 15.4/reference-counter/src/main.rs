// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // Value has been moved.
    // let b = Cons(3, Box::new(a));
    // Value used aftere move.
    // let c = Cons(4, Box::new(a));
    //
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // let b = Cons(3, Rc::clone(&a));
    // let c = Cons(4, Rc::clone(&a));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // This function is named strong_count rather than count because the Rc<T>
    // type also has a weak_count.
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    let c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
