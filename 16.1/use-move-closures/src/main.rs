use std::thread;

fn main() {
    let v = [1, 2, 3];

    // Add move would move v into closure's environment.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // Rust cannot guarantee if v is still valid in the spawned thread.
    // drop(v); // oh no...

    handle.join().unwrap();
}
