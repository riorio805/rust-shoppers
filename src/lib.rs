use std::{
    thread,
    sync::{mpsc, Arc, Mutex},
};
use std::sync::mpsc::{
    Receiver, Sender
};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: Sender<Job>,
}

pub struct PoolCreationError {
    error_message: &'static str
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.\
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    /// This function will panic if the size is zero or less.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender}
    }

    /// Create a new ThreadPool.\
    /// The size is the number of threads in the pool.
    ///
    /// # Errors
    /// This function will return an error if the size is zero or less.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError { error_message: "aaa" })
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    /// Executes passed function once using an available thread.
    /// If no threads are available, the function will be stored until a thread is available.
    pub fn execute<F>(&self, f: F)
    where
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
    pub fn new(id:usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        });

        Worker { id, thread }
    }
}
