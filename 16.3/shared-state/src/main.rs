use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc stands for Atomic reference counted.
    // Arc is thread safe compare to Rc. Rust will compile if we use Rc.
    // The reason we have to use reference counter type is because we pass counter to multiple
    // threads, and normal mutex will be borrowed by the first thread and cannot be used in a for
    // loop.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
