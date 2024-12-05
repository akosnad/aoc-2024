use aoc_2024::day1;
use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    let input: Vec<_> = include_str!("../src/day1/input.txt")
        .split('\n')
        .map(String::from)
        .collect();
    c.bench_function("day1", |b| b.iter(|| day1::run(std::hint::black_box(input.clone()))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
