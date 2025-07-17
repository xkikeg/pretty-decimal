use std::hint::black_box;
use std::str::FromStr;

use pretty_decimal::PrettyDecimal;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

static TEST_CASES: &[(&'static str, Decimal)] = &[
    ("regular", dec!(12_345_678.90)),
    ("negative", dec!(-12_345_678.90)),
    ("small", dec!(123)),
    ("tiny", dec!(0.000000001)),
];

fn from_str_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_str");

    for (name, value) in TEST_CASES {
        let s = value.to_string();
        group.bench_function(
            BenchmarkId::new("PrettyDecimal", format!("plain-{}", name)),
            |b| b.iter(|| black_box(PrettyDecimal::from_str(&s).unwrap())),
        );
        let s = PrettyDecimal::comma3dot(*value).to_string();
        group.bench_function(
            BenchmarkId::new("PrettyDecimal", format!("comma-{}", name)),
            |b| b.iter(|| black_box(PrettyDecimal::from_str(&s).unwrap())),
        );
    }

    group.finish();
}

fn to_string_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_string");

    for (name, value) in TEST_CASES {
        group.bench_function(
            BenchmarkId::new("PrettyDecimal", format!("plain-{}", name)),
            |b| b.iter(|| black_box(PrettyDecimal::plain(*value).to_string())),
        );
        group.bench_function(
            BenchmarkId::new("PrettyDecimal", format!("comma-{}", name)),
            |b| b.iter(|| black_box(PrettyDecimal::comma3dot(*value).to_string())),
        );
    }
}

criterion_group!(benches, from_str_benchmark, to_string_benchmark);
criterion_main!(benches);
