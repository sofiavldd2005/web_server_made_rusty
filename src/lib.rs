use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

#[derive(Debug)]
pub enum PoolCreationError {
    SizeIsZero,
    NotANumber, // Usually handled during string-to-int parsing
}

///Type alias for a trait objetc that holds the type of closure that execute receives
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool with 'size' threads.
    ///
    /// Instead of panicking, this returns a Result for the caller to handle.
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        //usize cuz it dont make sense negative threads
        if size == 0 {
            return Err(PoolCreationError::SizeIsZero);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        // 1. Initialize the vector first
        let mut workers = Vec::with_capacity(size);

        // 2. Run the logic to populate it
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // 3. Return the struct
        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, //the closure will be executed only one time
                                      // Sent to transfer the close from one thread to another and 'static cuz
                                      // we dk how much time the thread will take to execute
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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}
