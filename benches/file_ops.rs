use criterion::{black_box, criterion_group, criterion_main, Criterion};
use agent_laboratory::utils::FileOps;
use std::path::PathBuf;

fn file_ops_benchmark(c: &mut Criterion) {
    let file_ops = FileOps::new(false);
    let test_content = "Hello, World!".as_bytes();
    let test_path = PathBuf::from("test.txt");

    c.bench_function("safe_save", |b| {
        b.iter(|| file_ops.safe_save(black_box(test_content), black_box(&test_path)))
    });

    c.bench_function("safe_load", |b| {
        b.iter(|| file_ops.safe_load(black_box(&test_path)))
    });
}

criterion_group!(benches, file_ops_benchmark);
criterion_main!(benches);