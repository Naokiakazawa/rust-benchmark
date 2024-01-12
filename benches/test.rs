use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Debug, Copy, Clone)]
enum FunctionType {
    For,
    Fold,
}

fn bench_loop(n: u64, function: FunctionType) -> u64 {
    match function {
        FunctionType::For => for_loop(n),
        FunctionType::Fold => fold_method(n),
    }
}

fn for_loop(n: u64) -> u64 {
    let mut sum = 0;
    for i in 0..n {
        sum += i;
    }
    sum
}

fn fold_method(n: u64) -> u64 {
    (0..n).fold(0, |sum, i| sum + i)
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("loop_bench");
    for ft in [FunctionType::For, FunctionType::Fold].iter() {
        group.bench_function(format!("{:?}", ft), |b| {
            b.iter(|| bench_loop(black_box(1000000), *ft))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
