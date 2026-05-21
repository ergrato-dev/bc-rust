# Práctica 03 — Profiling con Flamegraph

## 🎯 Objetivo
Generar un flamegraph de un programa Rust con múltiples "hot paths" para practicar análisis de profiling.

## 🛠️ Cómo generar el flamegraph

```bash
# Compilar en release (necesario para profiling útil)
cargo build --release

# Generar flamegraph (requiere cargo-flamegraph instalado)
cargo flamegraph --bin carga-cpu

# Abrir el flamegraph
open flamegraph.svg
```

## ✅ Criterios de Aceptación

- [ ] `cargo run --release` termina correctamente
- [ ] Flamegraph generado muestra `calcular_primos` como función dominante
