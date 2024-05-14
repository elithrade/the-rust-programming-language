pub struct ThreadPool;

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
        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        // We need Send to transfer the closure from one thread to another and 'static because we
        // don't know how long the thread will take to execute.
        F: FnOnce() + Send + 'static,
    {
    }
}
