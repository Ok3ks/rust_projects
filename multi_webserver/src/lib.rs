use std::thread;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});
        Worker{ id, thread}
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }

    //In prod, use std::thread::Builder and its spawn method which
    // returns Result and handles errors of insufficient resources.
    pub fn spawn<F, T>(f: F) -> JoinHandle<T>
        where 
            F: FnOnce() -> T, //moving captured values out of the closures and fn Traits Fn, FnMut, FnOnce
            F: Send + 'static,
            T: Send + 'static,
    
    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static,
        {

        }

}

