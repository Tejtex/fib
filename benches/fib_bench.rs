use std::collections::VecDeque;
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use num_bigint::BigInt;
use fib::generate;
fn f(v: &VecDeque<BigInt>, _n: usize) -> BigInt {
    v.iter().sum()
}
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| generate(black_box(20), black_box(vec![BigInt::from(1), BigInt::from(1)]), black_box(2), black_box(f))));
    c.bench_function("fib 100", |b| b.iter(|| generate(black_box(100), black_box(vec![BigInt::from(1), BigInt::from(1)]), black_box(2), black_box(f))));
    c.bench_function("fib 1000", |b| b.iter(|| generate(black_box(1000), black_box(vec![BigInt::from(1), BigInt::from(1)]), black_box(2), black_box(f))));
    c.bench_function("fib 10000", |b| b.iter(|| generate(black_box(10000), black_box(vec![BigInt::from(1), BigInt::from(1)]), black_box(2), black_box(f))));
    c.bench_function("fib 100000", |b| b.iter(|| generate(black_box(100000), black_box(vec![BigInt::from(1), BigInt::from(1)]), black_box(2), black_box(f))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);