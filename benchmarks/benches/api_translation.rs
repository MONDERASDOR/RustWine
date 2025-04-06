use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustwine_core::api_translation::ApiTranslator;
use std::path::PathBuf;

fn api_translation_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("api_translation");
    
    // Benchmark PE loading
    group.bench_function("load_pe", |b| {
        let mut translator = ApiTranslator::new().unwrap();
        let test_exe = PathBuf::from("test.exe");
        
        b.iter(|| {
            translator.load_executable(black_box(test_exe.to_str().unwrap())).unwrap();
        });
    });
    
    // Benchmark syscall handling
    group.bench_function("handle_syscall", |b| {
        let translator = ApiTranslator::new().unwrap();
        let syscall_number = 0x1234;
        let args = vec![0x1, 0x2, 0x3, 0x4];
        
        b.iter(|| {
            translator.handle_syscall(black_box(syscall_number), black_box(&args)).unwrap();
        });
    });
    
    // Benchmark DLL redirection
    group.bench_function("redirect_dll", |b| {
        let translator = ApiTranslator::new().unwrap();
        let dll_name = "kernel32.dll";
        
        b.iter(|| {
            translator.redirect_dll(black_box(dll_name)).unwrap();
        });
    });
    
    group.finish();
}

criterion_group!(benches, api_translation_benchmark);
criterion_main!(benches); 