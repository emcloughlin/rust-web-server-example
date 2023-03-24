use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

static MIN_POOL_SIZE: usize = 1;
static MAX_POOL_SIZE: usize = 10;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

#[derive(Debug)]
pub enum PoolCreationError {
    InvalidPoolSize,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if (size < MIN_POOL_SIZE) || (size > MAX_POOL_SIZE) {
            Err(PoolCreationError::InvalidPoolSize)
        } else {
            let (sender, receiver) = mpsc::channel();

            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)))
            }

            Ok(ThreadPool { workers, sender })
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread: Some(thread) }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down workers...");
        for worker in &mut self.workers {
            worker.thread.take().unwrap().join().unwrap();
        }
    }
}

