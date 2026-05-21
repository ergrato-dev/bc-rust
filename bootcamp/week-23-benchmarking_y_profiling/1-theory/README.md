# Semana 23 — Teoría: Benchmarking y Profiling

## 1. Por qué el benchmark nativo de Rust es insuficiente

Rust incluye `#[bench]` en nightly, pero tiene limitaciones críticas:

- Solo disponible en **nightly** (inestable)
- No proporciona análisis estadístico: un solo run puede mentir
- El compilador puede **eliminar código que no tiene efectos observables** (dead code elimination)
- No detecta regresiones entre ejecuciones automáticamente

**Criterion** resuelve todos estos problemas en stable Rust.

---

## 2. Criterion: fundamentos estadísticos

Criterion ejecuta cada benchmark múltiples veces y aplica análisis estadístico:

- **Warmup**: descarta las primeras iteraciones (caché fría, JIT del SO)
- **Muestra**: ejecuta `N` iteraciones y registra tiempos
- **Media y desviación estándar**: cuantifica la variabilidad
- **Intervalos de confianza (95%)**: rango donde cae el valor real con alta probabilidad
- **Detección de regresión**: compara contra el baseline guardado

```toml
# Cargo.toml — agregar criterion como dev-dependency
[dev-dependencies]
criterion = { version = "0.5.1", features = ["html_reports"] }

[[bench]]
name    = "mi_bench"
harness = false   # deshabilitar el harness estándar de Rust
```

---

## 3. Anatomía de un benchmark con Criterion

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_mi_funcion(c: &mut Criterion) {
    // c es el contexto de benchmarking — gestiona muestras y tiempo
    c.bench_function("nombre_descriptivo", |b| {
        // b.iter() ejecuta el closure miles de veces
        b.iter(|| {
            // black_box: impide que el compilador elimine el cómputo
            mi_funcion(black_box(100))
        })
    });
}

// Registrar el grupo de benchmarks
criterion_group!(benches, bench_mi_funcion);
// Generar el main del harness
criterion_main!(benches);
```

Estructura de archivos:
```
mi-crate/
├── src/lib.rs          ← código a benchmarkear
└── benches/
    └── mi_bench.rs     ← benchmark (harness = false en Cargo.toml)
```

---

## 4. `black_box`: el escudo contra el compilador

Sin `black_box`, el compilador puede darse cuenta de que el resultado no se usa y eliminar
todo el cómputo, dando tiempos de ~0ns:

```rust
// ❌ El compilador puede eliminar fib_recursivo(20) si el resultado no se usa
b.iter(|| fib_recursivo(20));

// ✅ black_box oculta el valor del optimizador pero no inserta instrucciones reales
b.iter(|| fib_recursivo(black_box(20)));

// ✅ También aplicar a la salida si se quiere evitar que se elimine el retorno
b.iter(|| black_box(fib_recursivo(black_box(20))));
```

---

## 5. `BenchmarkGroup` y `BenchmarkId`: comparar variantes

```rust
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_sorting(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorting");

    // Iterar sobre tamaños de input distintos
    for size in [100usize, 1_000, 10_000] {
        let datos: Vec<i64> = (0..size as i64).rev().collect();

        group.bench_with_input(
            BenchmarkId::new("burbuja", size),  // nombre + parámetro
            &datos,
            |b, d| b.iter(|| { let mut v = black_box(d.clone()); burbuja(&mut v); }),
        );

        group.bench_with_input(
            BenchmarkId::new("stdlib", size),
            &datos,
            |b, d| b.iter(|| { let mut v = black_box(d.clone()); v.sort(); }),
        );
    }

    group.finish(); // importante: volcar resultados
}
```

Criterion genera una tabla comparativa y gráficos de violín en el reporte HTML.

---

## 6. Reportes HTML y cómo interpretarlos

```bash
cargo bench
# Abre el reporte en el navegador:
open target/criterion/report/index.html
```

El reporte contiene:
- **Gráfico de violín**: distribución de tiempos por variante
- **Gráfico de líneas**: tiempo vs tamaño de input (con `BenchmarkId`)
- **Tabla de resultados**: media, desviación estándar, throughput
- **Baseline comparison**: diferencia porcentual respecto al run anterior

```bash
# Guardar un baseline con nombre
cargo bench -- --save-baseline antes_optimizacion

# Comparar contra el baseline guardado
cargo bench -- --baseline antes_optimizacion
```

---

## 7. Flamegraph: instalación y generación

Un **flamegraph** es una visualización de _call stacks_ donde el eje X representa tiempo de CPU
y el eje Y la profundidad de llamadas. El ancho de cada rectángulo indica qué fracción del tiempo
consume esa función.

```bash
# Instalar flamegraph
cargo install flamegraph@0.6.5

# En Linux (requiere perf)
cargo flamegraph --bin mi-binario

# Salida: flamegraph.svg en el directorio actual
# Abrir en navegador — los frames son interactivos (click para zoom)
```

**Cómo leer el flamegraph:**
- Las funciones en la **base** son el punto de entrada (main)
- Las funciones en la **cima** son las que están "on CPU"
- Los rectángulos **más anchos** = más tiempo de CPU = hot path
- Buscar "mesetas" planas en la cima: candidatas a optimizar

---

## 8. `perf` en Linux: eventos de CPU

```bash
# Estadísticas generales
perf stat ./target/release/mi-binario

# Eventos de caché
perf stat -e cache-misses,cache-references ./target/release/mi-binario

# Profiling por función
perf record -g ./target/release/mi-binario
perf report
```

Eventos clave a monitorear:

| Evento | Qué indica |
|--------|-----------|
| `cache-misses` | Accesos a RAM (lento) en lugar de caché |
| `branch-misses` | Predicciones de salto fallidas |
| `instructions` | Instrucciones ejecutadas total |
| `cycles` | Ciclos de CPU consumidos |

---

## 9. Patrones de optimización comunes en Rust

### Evitar allocaciones en hot paths

```rust
// ❌ Crea un Vec en cada llamada
fn suma_cuadrados_v1(datos: &[i64]) -> i64 {
    let cuadrados: Vec<i64> = datos.iter().map(|x| x * x).collect();
    cuadrados.iter().sum()
}

// ✅ Sin allocación intermedia
fn suma_cuadrados_v2(datos: &[i64]) -> i64 {
    datos.iter().map(|x| x * x).sum()
}
```

### Pre-reservar capacidad

```rust
// ❌ Múltiples reallocaciones durante el push
let mut v = Vec::new();

// ✅ Una sola allocación
let mut v = Vec::with_capacity(n);
```

### Preferir iteradores sobre índices

Los iteradores permiten al compilador autovectorizar (SIMD automático):

```rust
// ✅ El compilador puede emitir instrucciones AVX/SSE
out.iter_mut()
    .zip(a.iter().zip(b.iter()))
    .for_each(|(o, (x, y))| *o = x + y);
```

### String concatenation

```rust
// ❌ O(n²) — crea un nuevo String en cada iteración
let mut s = String::new();
for p in palabras { s = s + p; }

// ✅ O(n) — una sola allocación
let mut s = String::with_capacity(total_len);
for p in palabras { s.push_str(p); }
```

---

## 10. Introducción a SIMD y autovectorización

**SIMD** (Single Instruction Multiple Data) permite operar sobre múltiples datos en paralelo
con una sola instrucción de CPU:

```
Escalar:  [a0] + [b0] = [c0]                      (1 operación)
SIMD x4:  [a0,a1,a2,a3] + [b0,b1,b2,b3] = [c0,c1,c2,c3]  (4 en paralelo)
```

Rust puede **autovectorizar** código simple si:
1. Los datos son contiguos en memoria (slices, no listas enlazadas)
2. El loop no tiene dependencias entre iteraciones
3. Se compila con `--release` (`-O3` equivalente)

Para forzar autovectorización en release:

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto        = true
codegen-units = 1  # más tiempo de compilación, mejor optimización
```

Verificar que se generaron instrucciones SIMD:

```bash
# Ver el ensamblado generado (buscar instrucciones ymm/xmm)
cargo rustc --release -- --emit asm
grep -i "ymm\|xmm\|vaddps\|vmulps" target/release/deps/*.s
```

---

## 11. Errores comunes en benchmarking

| Error | Problema | Solución |
|-------|---------|----------|
| Sin `black_box` | Compilador elimina el código | Envolver input y output |
| Medir incluido setup | El setup infla los tiempos | Mover setup fuera de `b.iter()` |
| Solo un run | Alta varianza sin contexto | Usar Criterion (múltiples runs) |
| Release desactivado | Debug es 10-100x más lento | `cargo bench` usa release por defecto |
| Estado entre iteraciones | Resultado afecta siguiente run | Clonar input dentro de `b.iter()` |

---

## 12. Comparación de herramientas

| Herramienta | Qué mide | Nivel |
|-------------|---------|-------|
| `criterion` | Tiempo de funciones individuales | Microbenchmark |
| `flamegraph` | Distribución de tiempo entre funciones | Profiling |
| `perf stat` | Eventos de hardware (caché, ciclos) | Profiling bajo nivel |
| `heaptrack` | Allocaciones de heap | Memory profiling |
| `twiggy` | Tamaño de secciones en binarios | WASM / binario |
| `cargo-llvm-lines` | Líneas de LLVM IR generadas | Análisis de code bloat |
