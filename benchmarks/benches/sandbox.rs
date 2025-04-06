use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustwine_seccomp::sandbox::Sandbox;
use libseccomp::ScmpAction;

fn sandbox_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sandbox");
    
    // Benchmark sandbox initialization
    group.bench_function("init_sandbox", |b| {
        b.iter(|| {
            Sandbox::new().unwrap();
        });
    });
    
    // Benchmark rule addition
    group.bench_function("add_rule", |b| {
        let mut sandbox = Sandbox::new().unwrap();
        
        b.iter(|| {
            sandbox.add_custom_rule(black_box("read"), black_box(ScmpAction::Allow)).unwrap();
        });
    });
    
    // Benchmark filter loading
    group.bench_function("load_filter", |b| {
        let sandbox = Sandbox::new().unwrap();
        
        b.iter(|| {
            sandbox.load_filter().unwrap();
        });
    });
    
    // Benchmark anti-cheat rules
    group.bench_function("add_anti_cheat_rules", |b| {
        let mut sandbox = Sandbox::new().unwrap();
        
        b.iter(|| {
            sandbox.add_anti_cheat_rules().unwrap();
        });
    });
    
    group.finish();
}

criterion_group!(benches, sandbox_benchmark);
criterion_main!(benches); 