# Práctica 02 — Benchmarking de Algoritmos de Ordenamiento

## 🎯 Objetivo
Comparar 4 algoritmos de ordenamiento con Criterion y `BenchmarkId` para múltiples tamaños de entrada.

## 🛠️ Cómo ejecutar

```bash
cargo bench
# Reportes HTML en target/criterion/sorting/report/index.html
```

## ✅ Criterios de Aceptación

- [ ] 4 algoritmos implementados y correctos (`cargo test`)
- [ ] Benchmark con 3 tamaños: 100, 1_000, 5_000
- [ ] `stdlib_sort` más rápido que `burbuja` en tamaños grandes
