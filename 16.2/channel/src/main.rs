use std::{sync::mpsc, thread, time::Duration};

fn main() {
    // A channel can have multiple sending ends that produce values but only one receiving end that
    // consumes those values.
    // tx is the sending end, the transmitter.
    // rx is the consuming end, the receiver.
    let (tx, rx) = mpsc::channel();

    // Create a thread to move tx into the closure so the thread owns tx.
    // The spawned thread needs to own tx in order to send through the channel.
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // Using val is not allowed.
        // The thread's modificationo could cause errors or unexpected results due to inconsistent
        // or nonexistent data.
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let (tx, rx) = mpsc::channel();

    // Sending multiple values.
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Treating rx as iterator.
    for received in rx {
        println!("Got {}", received);
    }

    let (tx, rx) = mpsc::channel();

    // Creating multiple producers by cloning the transmitter.
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
