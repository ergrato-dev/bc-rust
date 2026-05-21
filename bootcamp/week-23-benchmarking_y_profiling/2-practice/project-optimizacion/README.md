# Proyecto — Optimización Guiada por Benchmarks

## 🎯 Objetivo
Demostrar técnicas de optimización en Rust midiendo el impacto real con Criterion.

## 📋 Optimizaciones demostradas

| Benchmark | v1 (lento) | v2 (optimizado) | Técnica |
|-----------|-----------|----------------|---------|
| `maximo` | loop manual | `reduce` iterator | Autovectorización |
| `suma_cuadrados` | `collect` intermedio | iterator directo | Evitar allocación |
| `concatenar` | operador `+` | `with_capacity` + `push_str` | Pre-allocar |

## 🛠️ Cómo ejecutar

```bash
cargo test
cargo bench
open target/criterion/report/index.html
```

## ✅ Criterios de Aceptación

- [ ] 3 pares de benchmarks (v1 vs v2) para 3 patrones distintos
- [ ] `cargo test` — todas las versiones producen el mismo resultado
- [ ] `cargo bench` sin errores
- [ ] Al menos una optimización muestra mejora medible
