use criterion::{criterion_group, BenchmarkId, Criterion, Fun, ParameterizedBenchmark};

fn fibonacci_slow(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fibonacci_slow(n - 1) + fibonacci_slow(n - 2),
    }
}

fn fibonacci_fast(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

fn slow(c: &mut Criterion) {
    c.bench_function("fib 20 slow", |b| b.iter(|| fibonacci_slow(20)));
}

fn fast(c: &mut Criterion) {
    c.bench_function("fib 20 fast", |b| b.iter(|| fibonacci_fast(20)));
}

criterion_group!(fibonaccis, slow, fast);
