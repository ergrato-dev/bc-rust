---
applyTo: "bootcamp/week-18-*/**,bootcamp/week-19-*/**,bootcamp/week-20-*/**,bootcamp/week-21-*/**,bootcamp/week-22-*/**,bootcamp/week-23-*/**,bootcamp/week-24-*/**,bootcamp/week-25-*/**"
---

# Fases Avanzadas (Semanas 18-25) — Reglas Específicas

## Semana 18: Macros

### Declarativas (`macro_rules!`)
```rust
// Documentar qué patrón acepta el macro y qué expande
/// Crea un `HashMap` con pares clave-valor en una sola expresión.
///
/// # Examples
/// ```
/// let m = map!{ "a" => 1, "b" => 2 };
/// ```
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m = std::collections::HashMap::new();
        $(m.insert($k, $v);)*
        m
    }};
}
```

### Procedurales (`proc-macro`)
- Crear un crate separado `mi-crate-derive` para macros `#[derive]`
- Estructura del workspace:
  ```
  mi-crate/
  mi-crate-derive/   ← proc-macro = true en Cargo.toml
  ```
- Usar `syn` + `quote` para parsing y generación de código
- Probar macros con `cargo expand` (requiere `cargo-expand`)

## Semana 19: `unsafe` Rust

### Regla de oro
Todo bloque `unsafe` **debe** tener un comentario `// SAFETY:` explicando por qué es correcto:

```rust
// SAFETY: `ptr` fue obtenido de `Box::into_raw` en esta misma función
// y el caller garantiza que no hay otros accesos concurrentes.
let value = unsafe { Box::from_raw(ptr) };
```

### Checklist antes de escribir `unsafe`
- [ ] ¿Existe una alternativa segura? Si existe, úsala.
- [ ] ¿Están documentadas las invariantes que el código debe mantener?
- [ ] ¿El bloque `unsafe` es lo más pequeño posible?
- [ ] ¿Hay tests que verifica el comportamiento en casos límite?

### Tipos de `unsafe` cubiertos
| Operación | Cuándo es necesario |
|-----------|---------------------|
| Raw pointers `*const T` / `*mut T` | FFI, estructuras de datos internas |
| `unsafe fn` | Funciones que requieren invariantes del caller |
| `unsafe impl` | Implementar `Send`/`Sync` manualmente |
| Deref de raw pointer | Leer/escribir a través de un puntero crudo |

## Semana 20: FFI y Language Bindings

### Interop con C (`cbindgen`)
```rust
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
- Siempre `#[no_mangle]` para funciones exportadas a C
- `extern "C"` ABI para compatibilidad
- Tipos solo de `std::os::raw` o `libc` para cruzar la frontera ABI
- Generar cabecera con `cbindgen --config cbindgen.toml --crate mi-crate --output mi-crate.h`

### Bindings Python (`PyO3`)
```rust
use pyo3::prelude::*;

#[pyfunction]
fn suma(a: i64, b: i64) -> i64 {
    a + b
}

#[pymodule]
fn mi_modulo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(suma, m)?)?;
    Ok(())
}
```
- Build con `maturin develop` (desarrollo) o `maturin build --release` (distribución)
- Anotar tipos de error con `PyResult<T>` siempre

### Bindings Node.js (`napi-rs`)
```rust
#[napi]
pub fn suma(a: i32, b: i32) -> i32 {
    a + b
}
```
- Build con `npm run build` (usa `napi build` internamente)

## Semana 21: API Design y `crates.io`

### Reglas de diseño de API pública
- Seguir las [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- `#![deny(missing_docs)]` en el crate root
- Todos los tipos `pub` deben tener `///` doc comment
- Tipos de error deben implementar `std::error::Error` + `Display` + `Debug`
- Builder pattern para structs con muchos campos opcionales

### Semver
| Cambio | Versión bump |
|--------|-------------|
| Nueva funcionalidad, compatible | minor (`0.x.0`) |
| Bugfix | patch (`0.0.x`) |
| Cambio breaking | major (`x.0.0`) |

### Checklist publicación `crates.io`
- [ ] `Cargo.toml` con `description`, `license`, `repository`, `keywords`, `categories`
- [ ] `README.md` en el crate (se muestra en crates.io)
- [ ] `CHANGELOG.md` actualizado
- [ ] `cargo test --all-features` pasa
- [ ] `cargo publish --dry-run` sin errores

## Semana 22: WebAssembly

### Restricciones WASM
- ❌ No usar `std::thread` (WASM single-threaded salvo WASI threads)
- ❌ No usar `std::fs` directamente
- ✅ Usar `wasm-bindgen-futures` para código async
- ✅ `console_error_panic_hook` para ver panics en DevTools

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hola, {}!", name)
}
```
Build: `wasm-pack build --target web`

## Semana 23: Benchmarking y Profiling

### Criterion
```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_mi_funcion(c: &mut Criterion) {
    c.bench_function("mi_funcion 100", |b| {
        b.iter(|| mi_funcion(criterion::black_box(100)))
    });
}

criterion_group!(benches, bench_mi_funcion);
criterion_main!(benches);
```
- Siempre usar `criterion::black_box()` para evitar optimizaciones del compilador
- Crear grupo de benchmark por módulo / función
- Guardar resultados baseline con `--save-baseline`

## Semana 24: `no_std`

```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```
- Usar `core::` en lugar de `std::`
- Usar `alloc::` para heap allocation (requiere `extern crate alloc`)
- Prohibido `println!` — usar UART o semihosting
- Prohibido `Vec`, `String` sin `alloc` feature

## Semana 25: Capstone

- El crate debe tener `#![deny(missing_docs)]`
- Tests de integración en `tests/` usando la API pública
- `README.md` completo con: badge de CI, instalación, uso, API docs
- El proyecto debe compilar y pasar `cargo clippy -- -D warnings`
- Incluir benchmark de la funcionalidad principal con `criterion`
