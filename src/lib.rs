//! # Web Server Made Rusty
//!
//! This library provides a thread pool implementation designed for
//! handling concurrent TCP connections, specifically tailored for
//! learning the HTTP/1.1 protocol .
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

#[derive(Debug)]
pub enum PoolCreationError {
    SizeIsZero,
    NotANumber, // Usually handled during string-to-int parsing
}

///The Worker picks up code that needs to be run and runs the code in its thread.
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
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
        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }
    ///Worker structs that we just created to fetch the code to run
    ///from a queue held in the ThreadPool and send that code to its thread to run.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, //the closure will be executed only one time
                                      // Sent to transfer the close from one thread to another and 'static cuz
                                      // we dk how much time the thread will take to execute
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
impl Worker {
    ///The Arc type will let multiple Worker instances own the receiver,
    ///and Mutex will ensure that only one Worker gets a job from the receiver at a time
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing");
                    job();
                }
                Err(_) => {
                    println!("Worker{id} disconected; shutting down.");
                    break;
                }
            }
        });

        Worker { id, thread }
    }
}
