# Glosario — Semana 23: Benchmarking y Profiling

| Término | Definición |
|---------|-----------|
| **Microbenchmark** | Medición aislada del rendimiento de una función pequeña |
| **Criterion** | Crate de benchmarking estadístico para Rust |
| **`black_box`** | Función que impide que el compilador optimice el benchmark |
| **`BenchmarkGroup`** | Agrupación lógica de benchmarks relacionados en Criterion |
| **`BenchmarkId`** | Identificador único para un benchmark con parámetro variable |
| **Flamegraph** | Visualización de call stacks donde el ancho indica tiempo de CPU |
| **Hot path** | Ruta de código ejecutada con mayor frecuencia (crítica para performance) |
| **Autovectorización** | Conversión automática de loops escalares a instrucciones SIMD por el compilador |
| **SIMD** | Single Instruction Multiple Data — instrucciones que operan en paralelo |
| **`perf`** | Herramienta de profiling del kernel Linux basada en PMU |
| **PMU** | Performance Monitoring Unit — hardware que cuenta eventos de CPU |
| **Cache miss** | Acceso a memoria que no está en caché (lento) |
| **Zero-cost abstraction** | Abstracción Rust que no genera overhead en runtime |
| **`cargo bench`** | Comando que ejecuta todos los benchmarks del workspace |
| **Regresión de performance** | Degradación de rendimiento detectada por Criterion entre commits |
