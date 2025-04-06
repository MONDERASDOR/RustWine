use anyhow::Result;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

pub struct ThreadManager {
    threads: Mutex<HashMap<u32, thread::JoinHandle<()>>>,
    next_thread_id: Mutex<u32>,
}

impl ThreadManager {
    pub fn new() -> Self {
        Self {
            threads: Mutex::new(HashMap::new()),
            next_thread_id: Mutex::new(1),
        }
    }

    pub fn create_thread<F>(&self, f: F) -> Result<u32>
    where
        F: FnOnce() + Send + 'static,
    {
        let mut next_id = self.next_thread_id.lock();
        let thread_id = *next_id;
        *next_id += 1;

        let handle = thread::spawn(f);
        self.threads.lock().insert(thread_id, handle);
        Ok(thread_id)
    }

    pub fn join_thread(&self, thread_id: u32) -> Result<()> {
        if let Some(handle) = self.threads.lock().remove(&thread_id) {
            handle.join().map_err(|_| anyhow::anyhow!("Thread join failed"))?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Invalid thread ID"))
        }
    }

    pub fn thread_count(&self) -> usize {
        self.threads.lock().len()
    }
} 