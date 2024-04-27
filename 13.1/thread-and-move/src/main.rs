use std::{thread, time::Duration};

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // We spawn a new thread, giving the thread a closure to run. The closure body prints out a
    // list. The closure only captures an immutable reference of the list in order to print it.
    // The **move** keyword specify that list should be moved into the close.
    //
    // The compiler requires **move** because the main thread may finish before the new thread and
    // if it is the case list will be invalid once the new thread finishes.
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    let expensive_closure = |num: u32| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    thread::spawn(move || println!("From expensive thread: {:?}", expensive_closure(2)))
        .join()
        .unwrap();
}
