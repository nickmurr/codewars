use criterion::{black_box, criterion_group, BenchmarkId, Criterion, Fun, ParameterizedBenchmark};

fn comp(a: Vec<i64>, mut b: Vec<i64>) -> bool {
    if a.len() == 0 && b.len() == 0 {
        return true;
    }
    if a.len() != b.len() {
        return false;
    }
    for f in a.iter() {
        if !b.contains(&(f * f)) {
            return false;
        }
        let index = b.iter().position(|x| *x == f * f).unwrap();
        b.remove(index);
    }
    return true;
}

fn comp_ritiek(mut a: Vec<i64>, mut b: Vec<i64>) -> bool {
    a = a.iter().map(|x| x * x).collect();
    a.sort();
    b.sort();
    a == b
}

fn slow(c: &mut Criterion) {
    c.bench_function("compare fast`", |b| {
        b.iter(|| {
            comp(
                black_box(vec![
                    121, 144, 19, 161, 19, 144, 19, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                    23, 24, 25, 26, 27, 28, 29, 30,
                ]),
                black_box(vec![
                    11 * 11,
                    121 * 121,
                    144 * 144,
                    19 * 19,
                    161 * 161,
                    19 * 19,
                    144 * 144,
                    19 * 19,
                    11 * 11,
                    12 * 12,
                    13 * 13,
                    14 * 14,
                    15 * 15,
                    16 * 16,
                    17 * 17,
                    18 * 18,
                    19 * 19,
                    20 * 20,
                    21 * 21,
                    22 * 22,
                    23 * 23,
                    24 * 24,
                    25 * 25,
                    26 * 26,
                    27 * 27,
                    28 * 28,
                    29 * 29,
                    30 * 30,
                ]),
            )
        })
    });
}

fn fast(c: &mut Criterion) {
    c.bench_function("compare slow", |b| {
        b.iter(|| {
            comp_ritiek(
                black_box(vec![
                    121, 144, 19, 161, 19, 144, 19, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                    23, 24, 25, 26, 27, 28, 29, 30,
                ]),
                black_box(vec![
                    11 * 11,
                    121 * 121,
                    144 * 144,
                    19 * 19,
                    161 * 161,
                    19 * 19,
                    144 * 144,
                    19 * 19,
                    11 * 11,
                    12 * 12,
                    13 * 13,
                    14 * 14,
                    15 * 15,
                    16 * 16,
                    17 * 17,
                    18 * 18,
                    19 * 19,
                    20 * 20,
                    21 * 21,
                    22 * 22,
                    23 * 23,
                    24 * 24,
                    25 * 25,
                    26 * 26,
                    27 * 27,
                    28 * 28,
                    29 * 29,
                    30 * 30,
                ]),
            )
        })
    });
}

criterion_group!(same, slow, fast);
