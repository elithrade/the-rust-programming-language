use std::{thread, time::Duration};

fn main() {
    thread::spawn(|| {
        for i in 1..100 {
            println!("hi number {} from the spawned thread!", i);
            // Stops execution of main thread for short duration allowing a DIFFERENT thread to
            // run.
            thread::sleep(Duration::from_millis(1));
        }
    });

    // The threads probably will take turns but this is not guaranteed, it depends how OS schedules
    // the threads.
    // The main thread will stop the spawned threads prematurely due to the main thread ending.
    for i in 1..50 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
