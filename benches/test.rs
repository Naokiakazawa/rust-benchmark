use std::time::Instant;

fn bench_for_loop() -> u64 {
    let mut sum = 0;
    for i in 0..1000000 {
        sum += i;
    }
    sum
}

fn bench_fold() -> u64 {
    (0..1000000).fold(0, |sum, i| sum + i)
}

fn main() {
    let start = Instant::now();
    let for_loop_result = bench_for_loop();
    let for_loop_duration = start.elapsed();

    let start = Instant::now();
    let fold_result = bench_fold();
    let fold_duration = start.elapsed();

    println!("For loop result: {}, Time: {:?}", for_loop_result, for_loop_duration);
    println!("Fold result: {}, Time: {:?}", fold_result, fold_duration);
}
