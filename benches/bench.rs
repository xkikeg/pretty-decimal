use std::hint::black_box;
use std::str::FromStr;

use pretty_decimal::PrettyDecimal;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rust_decimal_macros::dec;

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("pretty-decimal");

    group.bench_function(BenchmarkId::new("from_str", "plain"), |b| {
        b.iter(|| black_box(PrettyDecimal::from_str("12345.678").unwrap()))
    });
    group.bench_function(BenchmarkId::new("from_str", "comma"), |b| {
        b.iter(|| black_box(PrettyDecimal::from_str("12,345.678").unwrap()))
    });

    group.bench_function(BenchmarkId::new("to_string", "plain"), |b| {
        b.iter(|| black_box(PrettyDecimal::plain(dec!(12_345_678.90)).to_string()))
    });
    group.bench_function(BenchmarkId::new("to_string", "comma"), |b| {
        b.iter(|| black_box(PrettyDecimal::comma3dot(dec!(12_345_678.90)).to_string()))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
