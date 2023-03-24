pub struct ThreadPool;
static MIN_POOL_SIZE: usize = 1;
static MAX_POOL_SIZE: usize = 10;

#[derive(Debug)]
pub enum PoolCreationError {
    InvalidPoolSize,

}

impl ThreadPool {
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size < MIN_POOL_SIZE || size > MAX_POOL_SIZE {
            Err(PoolCreationError::InvalidPoolSize)
        } else {
            Ok(ThreadPool)
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
