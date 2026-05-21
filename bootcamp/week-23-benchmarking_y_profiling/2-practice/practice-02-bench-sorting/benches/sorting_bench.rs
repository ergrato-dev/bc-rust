use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use practice_02_bench_sorting::{burbuja, insercion, mergesort, stdlib_sort};

fn generar_datos(n: usize) -> Vec<i64> {
    // Datos pseudoaleatorios reproducibles (lcg simple)
    let mut v = Vec::with_capacity(n);
    let mut x = 12345u64;
    for _ in 0..n {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        v.push((x >> 33) as i64);
    }
    v
}

fn bench_sorting(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorting");

    for size in [100usize, 1_000, 5_000] {
        let datos = generar_datos(size);

        group.bench_with_input(BenchmarkId::new("burbuja", size), &datos, |b, d| {
            b.iter(|| {
                let mut v = black_box(d.clone());
                burbuja(&mut v);
                v
            });
        });

        group.bench_with_input(BenchmarkId::new("insercion", size), &datos, |b, d| {
            b.iter(|| {
                let mut v = black_box(d.clone());
                insercion(&mut v);
                v
            });
        });

        group.bench_with_input(BenchmarkId::new("mergesort", size), &datos, |b, d| {
            b.iter(|| {
                let mut v = black_box(d.clone());
                mergesort(&mut v);
                v
            });
        });

        group.bench_with_input(BenchmarkId::new("stdlib", size), &datos, |b, d| {
            b.iter(|| {
                let mut v = black_box(d.clone());
                stdlib_sort(&mut v);
                v
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_sorting);
criterion_main!(benches);
