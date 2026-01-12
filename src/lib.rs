use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool
    /// The size is the number of threads in the pool
    /// # Panics
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // create a new channel for communication
        let (sender, receiver) = mpsc::channel();

        // using Arc<Mutex<T>> to bump ref count so that worker instances can share ownership of receiver
        let receiver = Arc::new(Mutex::new(receiver));

        // vector to store threads
        let mut workers = Vec::with_capacity(size);

        // create some threads and store them in the vector
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static, 
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

// each worker stores a single instance of JoinHandle<()>
// it will take closure of a code to run and send it to already running thread
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    // loop forever asking the receiving end of channel for a job and running it when it gets one
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {id} got a job; executing...");
                job();
            }
        });

        Worker { id, thread }
    }
}