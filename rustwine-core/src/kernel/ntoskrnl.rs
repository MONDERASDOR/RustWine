use anyhow::Result;
use parking_lot::Mutex;
use std::collections::HashMap;

pub struct Ntoskrnl {
    system_calls: Mutex<HashMap<u32, fn() -> Result<()>>>,
}

impl Ntoskrnl {
    pub fn new() -> Self {
        Self {
            system_calls: Mutex::new(HashMap::new()),
        }
    }

    pub fn register_syscall(&self, number: u32, handler: fn() -> Result<()>) {
        self.system_calls.lock().insert(number, handler);
    }

    pub fn handle_syscall(&self, number: u32) -> Result<()> {
        if let Some(handler) = self.system_calls.lock().get(&number) {
            handler()
        } else {
            Err(anyhow::anyhow!("Unknown system call: {}", number))
        }
    }
} 