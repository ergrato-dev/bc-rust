# Práctica 01 — Criterion Básico

## 🎯 Objetivo
Configurar y ejecutar benchmarks con Criterion, comparando dos implementaciones de Fibonacci.

## 🛠️ Cómo ejecutar

```bash
# Ejecutar benchmarks
cargo bench

# Ver reporte HTML
open target/criterion/fibonacci/report/index.html
```

## ✅ Criterios de Aceptación

- [ ] `cargo test` pasa
- [ ] `cargo bench` genera reportes HTML
- [ ] El benchmark muestra que `fib_iterativo` es significativamente más rápido
