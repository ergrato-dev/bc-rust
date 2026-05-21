# Práctica 04 — Intro a SIMD y autovectorización

## 🎯 Objetivo
Implementar operaciones vectoriales y observar cómo el compilador autovectoriza usando criterio.

## 🛠️ Cómo ejecutar

```bash
cargo test
cargo bench

# Para ver assembly y verificar vectorización
RUSTFLAGS="-C target-cpu=native" cargo bench
```

## ✅ Criterios de Aceptación

- [ ] `suma_escalar` e `suma_iteradores` producen el mismo resultado
- [ ] `cargo bench` sin errores
- [ ] El benchmark muestra rendimientos similares (el compilador autovectoriza ambas)
