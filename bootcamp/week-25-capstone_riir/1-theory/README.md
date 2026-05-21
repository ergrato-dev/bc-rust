# Semana 25 — Capstone: Librería Rust de Calidad de Producción

> Esta semana integra **todo** el bootcamp. El objetivo no es aprender conceptos nuevos
> sino construir un artefacto real que demuestre dominio del lenguaje y el ecosistema.

---

## 1. Las Cuatro Opciones RIIR

"Rewrite It In Rust" (RIIR) es el nombre informal para el proyecto de sustituir herramientas
existentes por implementaciones Rust más seguras, más rápidas o más ergonómicas.

| Opción | Proyecto | Stack principal | Salida |
|--------|----------|-----------------|--------|
| **A** | Parser/lexer expuesto a Python | `pyo3` + `maturin` | `.so` / `.pyd` importable desde Python |
| **B** | CLI que reemplaza `wc` | `clap` + `indicatif` | Binario nativo |
| **C** | Motor numérico en el navegador | `wasm-bindgen` + `wasm-pack` | Módulo `.wasm` + JS glue |
| **D** | Librería criptográfica con API C | `libc` + `cbindgen` | `.so` / `.a` + `.h` |

### Criterios de elección

- **Opción A** si tienes experiencia con Python y quieres ver el impacto de performance.
- **Opción B** si quieres el proyecto más portable y sin dependencias de runtime externo.
- **Opción C** si el objetivo es el ecosistema web / front-end.
- **Opción D** si quieres profundizar en `unsafe` y en la interoperabilidad a nivel de ABI.

---

## 2. Diseño de API Pública

Antes de escribir una sola línea de implementación, diseña la interfaz pública.
Las [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) resumen décadas
de experiencia. Los principios más importantes:

### 2.1 Nomenclatura idiomática

```rust
// Constructores: new() sin argumentos, from_xxx() con conversión
impl Config {
    pub fn new() -> Self { … }
    pub fn from_str(s: &str) -> Result<Self, ParseError> { … }
}

// Métodos de acceso: getters sin prefijo get_
// ✅ idiomático
pub fn nombre(&self) -> &str { … }
// ❌ no idiomático
pub fn get_nombre(&self) -> &str { … }

// Conversiones: as_xxx (borrow), to_xxx (alloc), into_xxx (consume)
pub fn as_bytes(&self) -> &[u8] { … }
pub fn to_string(&self) -> String { … }
pub fn into_vec(self) -> Vec<u8> { … }
```

### 2.2 Tipos de error bien diseñados

Los errores de una librería deben implementar `std::error::Error`, `Display` y `Debug`:

```rust
use std::fmt;

/// Error que puede ocurrir al parsear una expresión.
#[derive(Debug)]
pub enum ParseError {
    /// Carácter inesperado en la posición indicada.
    CaracterInvalido { pos: usize, c: char },
    /// La expresión está vacía.
    ExpresionVacia,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CaracterInvalido { pos, c } =>
                write!(f, "carácter inválido '{c}' en posición {pos}"),
            Self::ExpresionVacia => write!(f, "la expresión no puede estar vacía"),
        }
    }
}

impl std::error::Error for ParseError {}
```

### 2.3 Builder pattern para structs complejas

Cuando una struct tiene más de 3-4 campos opcionales, usa el patrón Builder:

```rust
/// Configuración del motor estadístico.
pub struct MotorConfig {
    precision: u32,
    usar_f64: bool,
    semilla: Option<u64>,
}

/// Builder de `MotorConfig`.
pub struct MotorConfigBuilder { inner: MotorConfig }

impl MotorConfigBuilder {
    pub fn precision(mut self, p: u32) -> Self {
        self.inner.precision = p; self
    }
    pub fn usar_f64(mut self, v: bool) -> Self {
        self.inner.usar_f64 = v; self
    }
    pub fn semilla(mut self, s: u64) -> Self {
        self.inner.semilla = Some(s); self
    }
    pub fn build(self) -> MotorConfig { self.inner }
}
```

---

## 3. Documentación con `#![deny(missing_docs)]`

Activar este lint convierte la falta de documentación en **error de compilación**:

```rust
// Al inicio de lib.rs
#![deny(missing_docs)]

//! # nombre-del-crate
//!
//! Descripción de una línea.
//!
//! ## Ejemplo rápido
//! ```
//! use nombre_del_crate::Procesador;
//! let p = Procesador::new();
//! assert_eq!(p.procesar("hola"), "HOLA");
//! ```
```

Cada `pub` item necesita `///`:

```rust
/// Procesa el texto según la configuración.
///
/// # Panics
/// No entra en pánico bajo ninguna condición normal.
///
/// # Errors
/// Retorna `Err(ParseError::ExpresionVacia)` si `texto` está vacío.
///
/// # Examples
/// ```
/// let p = Procesador::new();
/// assert_eq!(p.procesar("hola").unwrap(), "HOLA");
/// ```
pub fn procesar(&self, texto: &str) -> Result<String, ParseError> { … }
```

---

## 4. Estrategia de Testing

Una librería de producción necesita tres capas de tests:

```
src/
├── lib.rs              ← unit tests con #[cfg(test)] mod tests { … }
tests/
└── integracion.rs      ← integration tests (usan la API pública)
```

### Tests unitarios

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizar_suma_simple() {
        let tokens = tokenizar("1 + 2").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[1].tipo, TipoToken::Mas);
    }

    #[test]
    fn tokenizar_expresion_vacia_retorna_error() {
        assert!(tokenizar("").is_err());
    }
}
```

### Tests de integración (`tests/`)

```rust
// tests/integracion.rs — accede SOLO a la API pública
use nombre_del_crate::{Procesador, ParseError};

#[test]
fn pipeline_completo() {
    let p = Procesador::new();
    let resultado = p.procesar("(3 + 4) * 2").unwrap();
    assert!((resultado - 14.0).abs() < f64::EPSILON);
}
```

### Doctests

Los ejemplos en `///` se ejecutan con `cargo test` automáticamente. Son la primera
documentación que ve el usuario: deben compilar y pasar siempre.

---

## 5. Semver y `Cargo.toml` para publicación

```toml
[package]
name        = "nombre-del-crate"
version     = "0.1.0"          # MAJOR.MINOR.PATCH
edition     = "2021"
description = "Descripción de una línea (obligatoria para crates.io)"
license     = "MIT OR Apache-2.0"   # estándar dual-license en Rust
repository  = "https://github.com/usuario/repo"
keywords    = ["parser", "math", "lexer"]  # máx 5
categories  = ["parsing"]                  # de la lista oficial de crates.io
readme      = "README.md"
```

| Cambio | Bump | Ejemplo |
|--------|------|---------|
| Añadir función compatible | `MINOR` | `0.1.0 → 0.2.0` |
| Bugfix sin romper API | `PATCH` | `0.1.0 → 0.1.1` |
| Cambio de firma / eliminar pub item | `MAJOR` | `0.1.0 → 1.0.0` |

---

## 6. Integración por Opción

### Opción A — PyO3 + maturin

```toml
# Cargo.toml
[lib]
crate-type = ["cdylib"]       # obligatorio para módulos Python

[dependencies]
pyo3 = { version = "0.23.4", features = ["extension-module"] }
```

```bash
# Flujo de desarrollo
pip install maturin
maturin develop          # instala en el entorno Python activo
python -c "import nombre_del_crate; print(nombre_del_crate.funcion())"

# Distribución
maturin build --release  # genera .whl en target/wheels/
```

### Opción B — clap

```bash
cargo run -- --help           # mostrar ayuda
cargo run -- archivo.txt      # procesar archivo
cargo install --path .        # instalar binario globalmente
```

### Opción C — wasm-bindgen

```bash
# wasm-pack genera pkg/ con .wasm + JS glue + TypeScript types
wasm-pack build --target web      # para navegador
wasm-pack build --target nodejs   # para Node.js
wasm-pack test --headless --firefox
```

### Opción D — cbindgen

```bash
# build.rs genera el header automáticamente
cargo build                    # ejecuta build.rs → genera include/header.h
gcc demo.c -L target/debug -lnombre -o demo
./demo
```

---

## 7. Benchmarking con Criterion

Toda librería de producción necesita medir su rendimiento:

```toml
[dev-dependencies]
criterion = { version = "0.5.1", features = ["html_reports"] }

[[bench]]
name    = "benchmarks"
harness = false
```

```rust
// benches/benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nombre_del_crate::mi_funcion;

fn bench_mi_funcion(c: &mut Criterion) {
    c.bench_function("mi_funcion input-100", |b| {
        b.iter(|| mi_funcion(black_box("input de 100 chars")))
    });
}

criterion_group!(benches, bench_mi_funcion);
criterion_main!(benches);
```

```bash
cargo bench                         # ejecutar benchmarks
cargo bench -- --save-baseline v1   # guardar baseline
cargo bench -- --baseline v1        # comparar contra baseline
```

---

## 8. Checklist Final de Calidad

Antes de declarar el proyecto terminado, verificar:

```bash
cargo fmt --check              # código formateado
cargo clippy -- -D warnings    # cero warnings
cargo test                     # todos los tests pasan (unit + integration + doctests)
cargo audit                    # cero CVEs
cargo doc --open               # documentación genera sin errores
cargo publish --dry-run        # simular publicación
```

### Errores comunes

| Error | Causa | Solución |
|-------|-------|----------|
| `missing_docs` en type alias | `pub type Foo = …` sin `///` | Añadir doc comment |
| Doctest falla en CI | Import incompleto en ejemplo | Añadir `use crate::…` en el ejemplo |
| `clippy::needless_pass_by_ref` | `&String` en firma pública | Cambiar a `&str` |
| `unused_results` en PyO3 | `push()` retorna `Result` ignorado | Usar `.ok()` o manejar el error |
| ABI mismatch en FFI | Tipo Rust sin `#[repr(C)]` | Añadir `#[repr(C)]` en structs exportadas |

---

## 9. Comparación con Otros Lenguajes

| Aspecto | Rust | Python (biblioteca) | C++ (biblioteca) |
|---------|------|---------------------|-----------------|
| Distribución | `cargo publish` | `pip publish` | manual / vcpkg |
| ABI estable | No (rustc interna) | No (CPython) | Parcial (C-ABI sí) |
| Docs autogeneradas | `cargo doc` (rustdoc) | Sphinx / pdoc | Doxygen |
| Tests integrados | `cargo test` | pytest | CTest / gtest |
| Versionado | SemVer estricto | SemVer (convención) | Sin estándar |
| Bindings externos | PyO3 / napi-rs / cbindgen | ctypes / cffi | pybind11 / swig |
