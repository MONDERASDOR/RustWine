use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustwine_core::memory::MemoryManager;
use rustwine_core::threading::ThreadManager;

pub fn memory_benchmark(c: &mut Criterion) {
    let memory = MemoryManager::new();
    
    c.bench_function("memory allocation", |b| {
        b.iter(|| {
            let _ = black_box(memory.allocate(1024));
        })
    });
}

pub fn threading_benchmark(c: &mut Criterion) {
    let thread_manager = ThreadManager::new();
    
    c.bench_function("thread creation", |b| {
        b.iter(|| {
            let _ = black_box(thread_manager.create_thread(|| {}));
        })
    });
}

criterion_group!(benches, memory_benchmark, threading_benchmark);
criterion_main!(benches); 