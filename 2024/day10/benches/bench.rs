use criterion::{criterion_group, criterion_main, Criterion};
use day10::doit;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day10 main", |b| b.iter(|| doit()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
