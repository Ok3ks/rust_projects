use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });
        Worker{ id, thread}

    //lock acquires the Mutex lock which ensures 
    //that only one worker thread is requesting a job, unwrap reveals any panic
    //from other threads(in any case there's poisoned threads)
    //recv blocks so if there's no job yet, the thread will wait 
    //until a job becomes available

    }
}

// struct Job;
//using a type alias for a trait object that holds the type of 
//closure that execute receives
type Job = Box<dyn FnOnce() + Send + 'static>;


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    //In prod, use std::thread::Builder and its spawn method which
    // returns Result and handles errors of insufficient resources.

    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);

            self.sender.send(job).unwrap();
        }

}



