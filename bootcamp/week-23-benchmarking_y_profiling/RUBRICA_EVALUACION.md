# 📊 Rúbrica de Evaluación — Semana 23: Benchmarking y Profiling

## 📋 Distribución de Puntos

| Tipo | Peso | Puntos |
|------|------|--------|
| **Conocimiento** | 30% | 30 pts |
| **Desempeño** | 40% | 40 pts |
| **Producto** | 30% | 30 pts |

---

## 🧠 Conocimiento (30 pts)

1. ¿Qué problema resuelve Criterion vs el bench estándar de Rust? (5 pts)
2. ¿Qué es la varianza estadística en benchmarks y cómo afecta los resultados? (5 pts)
3. ¿Qué es un flamegraph y qué información proporciona? (5 pts)
4. ¿Cuándo es apropiado usar SIMD? (5 pts)
5. ¿Qué es "microbenchmark fallacy" y cómo evitarla? (5 pts)
6. ¿Qué hace `black_box` en Criterion? (5 pts)

---

## ⚙️ Desempeño (40 pts)

- [ ] Benchmark con `criterion::BenchmarkGroup` y múltiples configuraciones (20 pts)
- [ ] Análisis de resultados: identificar la implementación más rápida (20 pts)

---

## 🏗️ Producto (30 pts): `project-optimizacion`

| Criterio | Puntos |
|----------|--------|
| Al menos 3 benchmarks comparando implementaciones | 15 pts |
| Optimización demostrable entre v1 y v2 | 10 pts |
| `cargo bench` sin errores | 5 pts |
