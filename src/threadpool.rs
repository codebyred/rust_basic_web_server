use std::{
    error, fmt::{self, Display, Formatter}, thread::{self, JoinHandle}
};

#[derive(Debug)]
pub struct ThreadPool{
    threads: Vec<Worker>
}

#[derive(Debug)]
pub struct PoolCreationErr;

#[derive(Debug)]
pub struct Worker {
    id: u32,
    handler: JoinHandle<()>
}

impl Display for PoolCreationErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Pool Creation Error")
    }
}

impl error::Error for PoolCreationErr {}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is number of threads in the pool
    ///
    /// # Panics
    /// 
    /// The new function will panic if size is 0
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationErr> {
        
        if size > 0 {
            return Err(PoolCreationErr);
        };
        
        let mut threads = Vec::with_capacity(size);
        
        for i in 0..size {
            threads.push(Worker::new(i as u32));
        }
        
        Ok(ThreadPool { threads })
        
    }
    
    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static
    {
        
    }
}

impl Worker {
    fn new(id: u32) -> Worker {
        Worker {
            id,
            handler: thread::spawn(|| {})
        }
    }
}