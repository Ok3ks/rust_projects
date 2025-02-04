use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
    //Changing to Option to allow .join consume the thread 
    //and replace Some with None
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });
        Worker{ id, thread: Some(thread)}

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
    sender: Option<mpsc::Sender<Job>>,
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

        ThreadPool { workers, sender:Some(sender) }
    }

    //In prod, use std::thread::Builder and its spawn method which
    // returns Result and handles errors of insufficient resources.

    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);

            self.sender.as_ref().send(job).unwrap();
        }

    }

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}



