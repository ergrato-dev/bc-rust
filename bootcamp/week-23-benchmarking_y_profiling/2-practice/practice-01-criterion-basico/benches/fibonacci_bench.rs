use criterion::{black_box, criterion_group, criterion_main, Criterion};
use practice_01_criterion_basico::{fib_iterativo, fib_recursivo};

fn bench_fibonacci(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");

    group.bench_function("recursivo_n20", |b| {
        b.iter(|| fib_recursivo(black_box(20)));
    });

    group.bench_function("iterativo_n20", |b| {
        b.iter(|| fib_iterativo(black_box(20)));
    });

    group.bench_function("iterativo_n1000", |b| {
        b.iter(|| fib_iterativo(black_box(1000)));
    });

    group.finish();
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);
