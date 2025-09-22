use std::collections::VecDeque;
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use num_bigint::BigInt;
use num_traits::One;
use fib::generate;
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| generate(black_box(20), black_box(vec![BigInt::one(), BigInt::one()]), black_box(2), black_box(&[1, 1]), black_box(None))));
    c.bench_function("fib 100", |b| b.iter(|| generate(black_box(100), black_box(vec![BigInt::one(), BigInt::one()]), black_box(2), black_box(&[1, 1]), black_box(None))));
    c.bench_function("fib 1000", |b| b.iter(|| generate(black_box(1000), black_box(vec![BigInt::one(), BigInt::one()]), black_box(2), black_box(&[1, 1]), black_box(None))));
    c.bench_function("fib 10000", |b| b.iter(|| generate(black_box(10000), black_box(vec![BigInt::one(), BigInt::one()]), black_box(2), black_box(&[1, 1]), black_box(None))));
    c.bench_function("fib 100000", |b| b.iter(|| generate(black_box(100000), black_box(vec![BigInt::one(), BigInt::one()]), black_box(2), black_box(&[1, 1]), black_box(None))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);