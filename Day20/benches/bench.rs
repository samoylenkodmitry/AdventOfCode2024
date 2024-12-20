use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day::{f1, f2};
use std::fs::read_to_string;

fn criterion_benchmark(c: &mut Criterion) {
    let input = read_to_string("./input.txt").expect("Could not read input file");
    
    c.bench_function("f1", |b| b.iter(|| f1(black_box(&input))));
    c.bench_function("f2", |b| b.iter(|| f2(black_box(&input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);