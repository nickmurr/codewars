use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    // benchmarks::compare_functions::fibonaccis,
    benchmarks::compare_same::same,
}
