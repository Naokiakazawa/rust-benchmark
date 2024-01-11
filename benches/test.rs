use criterion::{criterion_group, criterion_main, Criterion};

fn bench_for_loop() -> u64 {
    let mut sum = 0;
    for i in 0..1000000 {
        sum += i;
    }
    sum
}

/*
fn bench_fold() -> u64 {
    (0..1000000).fold(0, |sum, i| sum + i)
}
*/

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("for_loop", |b| b.iter(|| bench_for_loop()));
    // c.bench_function("fold", |b| b.iter(|| bench_fold()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
