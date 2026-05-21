use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use project_optimizacion::{
    concatenar_v1, concatenar_v2, maximo_v1, maximo_v2, suma_cuadrados_v1, suma_cuadrados_v2,
};

fn bench_maximo(c: &mut Criterion) {
    let mut group = c.benchmark_group("maximo");
    let datos: Vec<i64> = (0..100_000).collect();

    group.bench_with_input(BenchmarkId::new("v1_loop", "100k"), &datos, |b, d| {
        b.iter(|| maximo_v1(black_box(d)));
    });
    group.bench_with_input(BenchmarkId::new("v2_iterator", "100k"), &datos, |b, d| {
        b.iter(|| maximo_v2(black_box(d)));
    });
    group.finish();
}

fn bench_suma_cuadrados(c: &mut Criterion) {
    let mut group = c.benchmark_group("suma_cuadrados");
    let datos: Vec<i64> = (0..10_000).collect();

    group.bench_with_input(BenchmarkId::new("v1_collect", "10k"), &datos, |b, d| {
        b.iter(|| suma_cuadrados_v1(black_box(d)));
    });
    group.bench_with_input(BenchmarkId::new("v2_noalloc", "10k"), &datos, |b, d| {
        b.iter(|| suma_cuadrados_v2(black_box(d)));
    });
    group.finish();
}

fn bench_concatenar(c: &mut Criterion) {
    let mut group = c.benchmark_group("concatenar");
    let palabras: Vec<String> = (0..500).map(|i| format!("palabra{i} ")).collect();
    let refs: Vec<&str> = palabras.iter().map(|s| s.as_str()).collect();

    group.bench_with_input(BenchmarkId::new("v1_plus", "500"), &refs, |b, r| {
        b.iter(|| concatenar_v1(black_box(r)));
    });
    group.bench_with_input(BenchmarkId::new("v2_capacity", "500"), &refs, |b, r| {
        b.iter(|| concatenar_v2(black_box(r)));
    });
    group.finish();
}

criterion_group!(benches, bench_maximo, bench_suma_cuadrados, bench_concatenar);
criterion_main!(benches);
