use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // The main thread will stop the spawned threads prematurely due to the main thread ending.
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // To fix the prematurely ending spawned threads, the returned type of spawn returns a
    // JoinHandle. Calling join method to wait for its threads to finish.
    // Where to call join matters, it affects whether or not your threads run at the same time.
    handle.join().unwrap();
}
