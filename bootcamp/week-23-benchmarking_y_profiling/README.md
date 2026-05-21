# 🦀 Semana 23: Benchmarking y Profiling

## 📋 Información General

| Campo | Detalle |
|-------|---------|
| **Semana** | 23 de 25 |
| **Tema** | Benchmarking con Criterion, Profiling, SIMD básico |
| **Duración** | 4 horas |
| **Nivel** | Avanzado — Fase de Sistemas y Performance |
| **Requisitos** | Semana 22, conocimiento de algoritmos básicos |

---

## 🎯 Objetivos de Aprendizaje

1. **Escribir** benchmarks estadísticamente rigurosos con `criterion`
2. **Generar** reportes HTML de benchmarks
3. **Usar** flamegraph y `perf` para profiling
4. **Identificar** cuellos de botella en código Rust
5. **Aplicar** optimizaciones guiadas por datos
6. **Introducción** a SIMD con `std::simd` (nightly) o manualmente

---

## 🛠️ Setup

```bash
# Instalar herramientas de profiling
cargo install flamegraph

# Ejecutar benchmarks
cargo bench

# Generar flamegraph
cargo flamegraph --bin mi-binario

# Ver reportes HTML de criterion
open target/criterion/report/index.html
```
