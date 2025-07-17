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

fn display_benchmark(c: &mut Criterion) {
    use std::io::Write as _;

    let mut group = c.benchmark_group("display/simple");

    for (name, value) in TEST_CASES {
        group.bench_function(
            BenchmarkId::new("PrettyDecimal", format!("plain-{}", name)),
            |b| {
                b.iter(|| {
                    let mut buf = arrayvec::ArrayVec::<u8, 128>::new();
                    write!(&mut buf, "{}", PrettyDecimal::plain(*value)).unwrap();
                    black_box(buf)
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("PrettyDecimal", format!("comma-{}", name)),
            |b| {
                b.iter(|| {
                    let mut buf = arrayvec::ArrayVec::<u8, 128>::new();
                    write!(&mut buf, "{}", PrettyDecimal::comma3dot(*value)).unwrap();
                    black_box(buf)
                })
            },
        );
    }
    group.finish();

    let mut group = c.benchmark_group("display/padding");
    for (name, value) in TEST_CASES {
        group.bench_function(
            BenchmarkId::new("PrettyDecimal", format!("plain-{}", name)),
            |b| {
                b.iter(|| {
                    let mut buf = arrayvec::ArrayVec::<u8, 128>::new();
                    write!(&mut buf, "{:-^20}", PrettyDecimal::plain(*value)).unwrap();
                    black_box(buf)
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("PrettyDecimal", format!("comma-{}", name)),
            |b| {
                b.iter(|| {
                    let mut buf = arrayvec::ArrayVec::<u8, 128>::new();
                    write!(&mut buf, "{:-^20}", PrettyDecimal::comma3dot(*value)).unwrap();
                    black_box(buf)
                })
            },
        );
    }
    group.finish();

    let mut group = c.benchmark_group("display/precision");
    for (name, value) in TEST_CASES {
        group.bench_function(
            BenchmarkId::new("PrettyDecimal", format!("plain-{}", name)),
            |b| {
                b.iter(|| {
                    let mut buf = arrayvec::ArrayVec::<u8, 128>::new();
                    write!(&mut buf, "{:.5}", PrettyDecimal::plain(*value)).unwrap();
                    black_box(buf)
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("PrettyDecimal", format!("comma-{}", name)),
            |b| {
                b.iter(|| {
                    let mut buf = arrayvec::ArrayVec::<u8, 128>::new();
                    write!(&mut buf, "{:.5}", PrettyDecimal::comma3dot(*value)).unwrap();
                    black_box(buf)
                })
            },
        );
    }
    group.finish();

    let mut group = c.benchmark_group("display/sign");
    for (name, value) in TEST_CASES {
        group.bench_function(
            BenchmarkId::new("PrettyDecimal", format!("plain-{}", name)),
            |b| {
                b.iter(|| {
                    let mut buf = arrayvec::ArrayVec::<u8, 128>::new();
                    write!(&mut buf, "{:+}", PrettyDecimal::plain(*value)).unwrap();
                    black_box(buf)
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("PrettyDecimal", format!("comma-{}", name)),
            |b| {
                b.iter(|| {
                    let mut buf = arrayvec::ArrayVec::<u8, 128>::new();
                    write!(&mut buf, "{:+}", PrettyDecimal::comma3dot(*value)).unwrap();
                    black_box(buf)
                })
            },
        );
    }
    group.finish();

    let mut group = c.benchmark_group("display/zero-sign-precision");
    for (name, value) in TEST_CASES {
        group.bench_function(
            BenchmarkId::new("PrettyDecimal", format!("plain-{}", name)),
            |b| {
                b.iter(|| {
                    let mut buf = arrayvec::ArrayVec::<u8, 128>::new();
                    write!(&mut buf, "{:+08.5}", PrettyDecimal::plain(*value)).unwrap();
                    black_box(buf)
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("PrettyDecimal", format!("comma-{}", name)),
            |b| {
                b.iter(|| {
                    let mut buf = arrayvec::ArrayVec::<u8, 128>::new();
                    write!(&mut buf, "{:+08.5}", PrettyDecimal::comma3dot(*value)).unwrap();
                    black_box(buf)
                })
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    from_str_benchmark,
    to_string_benchmark,
    display_benchmark
);
criterion_main!(benches);
