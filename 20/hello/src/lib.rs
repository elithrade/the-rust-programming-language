use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

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

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        // We need Send to transfer the closure from one thread to another and 'static because we
        // don't know how long the thread will take to execute.
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}
