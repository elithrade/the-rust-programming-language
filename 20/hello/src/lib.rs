use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

/// Creates a new threal pool.
///
/// The size is the number of threads in the pool.
///
/// # Panics
///
/// The 'new' function will panic if the size is zero.
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // Each thread poll will create and hold on to the sender.
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // Each workder will hold onto the receiver.
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        // We need Send to transfer the closure from one thread to another and 'static because we
        // don't know how long the thread will take to execute.
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    // To share onwership across multiple threads and allow threads to mutate the value, we need to
    // use Arc<Mutex<T>>. The Arc type will allow multiple workers own the receiver, and Mutex will
    // ensure only one worker gets the job from the receiver at a time.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Workker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}
