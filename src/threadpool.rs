use std::{
    error, fmt::{self, Display, Formatter}, sync::{mpsc, Arc, Mutex}, thread
};

type Job = Box<dyn FnOnce() + 'static + Send>;

#[derive(Debug)]
pub struct PoolCreationErr;

impl Display for PoolCreationErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Pool Creation Error")
    }
}

impl error::Error for PoolCreationErr {}

#[derive(Debug)]
struct Worker {
    id: u32,
    handler: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: u32, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        Worker {
            id,
            handler: thread::spawn(move || loop {
                
                let job = receiver
                    .lock()
                    .unwrap()
                    .recv()
                    .unwrap();
                
                job();
            
            })
        }
    }
}

#[derive(Debug)]
pub struct ThreadPool{
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is number of threads in the pool
    ///
    /// # Panics
    /// 
    /// The new function will panic if size is 0
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationErr> {
        
        if size == 0 {
            return Err(PoolCreationErr);
        };
        
        let mut threads = Vec::with_capacity(size);
        
        let (tx, rx) = mpsc::channel();
        
        let receiver = Arc::new(Mutex::new(rx));
        
        for i in 0..size {
            threads.push(Worker::new(i as u32, Arc::clone(&receiver)));
        }
        
        Ok(ThreadPool { threads , sender:tx})
        
    }
    
    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}


