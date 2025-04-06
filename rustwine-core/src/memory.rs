use anyhow::Result;
use parking_lot::Mutex;
use std::collections::HashMap;
use memmap2::{MmapMut, MmapOptions};

pub struct MemoryManager {
    mappings: Mutex<HashMap<u64, MmapMut>>,
    next_address: Mutex<u64>,
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            mappings: Mutex::new(HashMap::new()),
            next_address: Mutex::new(0x100000), // Start at 1MB
        }
    }

    pub fn allocate(&self, size: usize) -> Result<u64> {
        let mut next_addr = self.next_address.lock();
        let addr = *next_addr;
        *next_addr += size as u64;

        let mut mmap = MmapOptions::new()
            .len(size)
            .map_anon()?;

        self.mappings.lock().insert(addr, mmap);
        Ok(addr)
    }

    pub fn deallocate(&self, addr: u64) -> Result<()> {
        if self.mappings.lock().remove(&addr).is_some() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Invalid memory address"))
        }
    }

    pub fn read(&self, addr: u64, buf: &mut [u8]) -> Result<()> {
        if let Some(mmap) = self.mappings.lock().get(&addr) {
            buf.copy_from_slice(&mmap[..buf.len()]);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Invalid memory address"))
        }
    }

    pub fn write(&self, addr: u64, buf: &[u8]) -> Result<()> {
        if let Some(mmap) = self.mappings.lock().get_mut(&addr) {
            mmap[..buf.len()].copy_from_slice(buf);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Invalid memory address"))
        }
    }
} 