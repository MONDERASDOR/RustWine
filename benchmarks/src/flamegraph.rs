use flamegraph::start_flamegraph;
use rustwine_core::memory::MemoryManager;
use rustwine_core::threading::ThreadManager;
use std::thread;
use std::time::Duration;

pub fn profile_memory_operations() {
    let memory = MemoryManager::new();
    start_flamegraph("memory_operations", || {
        for _ in 0..1000 {
            let _ = memory.allocate(1024);
            thread::sleep(Duration::from_millis(1));
        }
    });
}

pub fn profile_thread_operations() {
    let thread_manager = ThreadManager::new();
    start_flamegraph("thread_operations", || {
        for _ in 0..100 {
            let _ = thread_manager.create_thread(|| {
                thread::sleep(Duration::from_millis(10));
            });
        }
    });
}

pub fn main() {
    profile_memory_operations();
    profile_thread_operations();
} 