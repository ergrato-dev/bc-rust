use criterion::{black_box, criterion_group, criterion_main, Criterion};
use practice_04_simd_intro::{dot_product, suma_escalar, suma_iteradores};

fn generar_f32(n: usize) -> Vec<f32> {
    (0..n).map(|i| (i as f32) * 0.001).collect()
}

fn bench_suma(c: &mut Criterion) {
    let n = 10_000;
    let a = generar_f32(n);
    let b = generar_f32(n);

    let mut group = c.benchmark_group("suma_vectores");

    group.bench_function("escalar", |bench| {
        let mut out = vec![0.0f32; n];
        bench.iter(|| suma_escalar(black_box(&a), black_box(&b), &mut out));
    });

    group.bench_function("iteradores", |bench| {
        let mut out = vec![0.0f32; n];
        bench.iter(|| suma_iteradores(black_box(&a), black_box(&b), &mut out));
    });

    group.finish();

    let mut dot_group = c.benchmark_group("dot_product");
    dot_group.bench_function("n10000", |bench| {
        bench.iter(|| dot_product(black_box(&a), black_box(&b)));
    });
    dot_group.finish();
}

criterion_group!(benches, bench_suma);
criterion_main!(benches);
