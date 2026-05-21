# 📖 04 — Documentación, Doctests y `examples/`

## La documentación como parte del contrato

En Rust, la documentación no es opcional para una librería pública: es parte
del contrato de la API. `cargo doc` genera HTML navegable y `cargo test --doc`
ejecuta los ejemplos de código incrustados en ella.

```
┌─────────────────────────────────────────────────────────────────┐
│               TIPOS DE COMENTARIOS EN RUST                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  //  → comentario de implementación (no aparece en docs)        │
│  ///  → documenta el ÍTEM siguiente (fn, struct, enum, const)   │
│  //!  → documenta el MÓDULO que lo contiene (inner doc)         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## `///` — Documentar un ítem

```rust
/// Convierte una cadena a formato slug: minúsculas con guiones.
///
/// Los espacios múltiples se colapsan a un solo guión.
/// Los caracteres ya en minúscula se preservan.
///
/// # Examples
///
/// ```
/// use mi_crate::slugify;
///
/// assert_eq!(slugify("Hola Mundo"), "hola-mundo");
/// assert_eq!(slugify("  espacios  extra  "), "espacios-extra");
/// ```
///
/// # Panics
///
/// Esta función no hace panic.
///
/// # Errors
///
/// Esta función no retorna errores; si el input está vacío retorna `""`.
pub fn slugify(texto: &str) -> String {
    texto.split_whitespace()
         .map(str::to_lowercase)
         .collect::<Vec<_>>()
         .join("-")
}
```

---

## `//!` — Documentar el módulo o crate raíz

```rust
//! # mi-crate — Utilidades de texto
//!
//! Librería de utilidades de procesamiento de texto para Rust.
//!
//! ## Características
//!
//! - `slugify` — convierte texto a slug URL-safe
//! - `truncar` — corta texto con sufijo configurable
//! - `contar_palabras` — cuenta palabras en un texto
//!
//! ## Inicio rápido
//!
//! ```
//! use mi_crate::{slugify, truncar};
//!
//! let slug = slugify("Hola Mundo Rust");
//! assert_eq!(slug, "hola-mundo-rust");
//!
//! let truncado = truncar("texto muy largo", 5, "...");
//! assert_eq!(truncado, "texto...");
//! ```

// El código del módulo sigue aquí...
```

---

## Secciones canónicas de documentación

| Sección | Cuándo incluirla | Ejemplo |
|---------|-----------------|---------|
| `# Examples` | **Siempre** en funciones públicas | Código ejecutable |
| `# Errors` | Cuando la función retorna `Result<T, E>` | Variantes del error |
| `# Panics` | Cuando la función puede hacer `panic!` | Condición de panic |
| `# Safety` | Cuando la función es `unsafe` | Invariantes que el caller debe cumplir |

```rust
/// Divide `a` entre `b`.
///
/// # Examples
///
/// ```
/// use mi_crate::dividir;
///
/// assert_eq!(dividir(10, 2).unwrap(), 5);
/// assert!(dividir(10, 0).is_err());
/// ```
///
/// # Errors
///
/// Retorna [`DivisionError::PorCero`] si `b` es cero.
pub fn dividir(a: i64, b: i64) -> Result<i64, DivisionError> {
    if b == 0 { return Err(DivisionError::PorCero); }
    Ok(a / b)
}
```

---

## Doctests — ejemplos que son tests

Los bloques de código en `///` se ejecutan con `cargo test --doc`:

```rust
/// Calcula el factorial de `n`.
///
/// # Examples
///
/// ```
/// use mi_crate::factorial;
///
/// // Casos básicos
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(1), 1);
/// assert_eq!(factorial(5), 120);
/// ```
pub fn factorial(n: u64) -> u64 { (1..=n).product() }
```

### Directivas en doctests

```rust
/// ```
/// # use mi_crate::Foo;  // # oculta esta línea del output HTML pero la ejecuta
/// let f = Foo::new();
/// assert!(f.is_valid());
/// ```
///
/// ```should_panic
/// # use mi_crate::factorial;
/// factorial(21);  // panic — overflow
/// ```
///
/// ```compile_fail
/// // Este bloque documenta que algo NO compila
/// let x: i32 = "hola";  // error de tipo intencional
/// ```
///
/// ```no_run
/// // Se valida que compile pero no se ejecuta (útil para ejemplos de red)
/// let _ = reqwest::get("https://api.example.com");
/// ```
```

| Directiva | Efecto |
|-----------|--------|
| `# código` | Oculto en HTML, incluido en el test |
| `should_panic` | El bloque debe hacer panic |
| `compile_fail` | El bloque debe dar error de compilación |
| `no_run` | Se comprueba que compile pero no se ejecuta |
| `ignore` | El bloque se muestra pero no se ejecuta ni compila |

---

## `#[deny(missing_docs)]` — API completamente documentada

Añadir este atributo al crate root fuerza documentar toda la API pública:

```rust
//! Documentación del crate

#![deny(missing_docs)]

pub struct MiStruct {
    // ❌ Error de compilación — campo público sin doc
    pub campo: String,
}

/// Struct bien documentado.
pub struct OtroStruct {
    /// El nombre del elemento.
    pub nombre: String,
}
```

Cuando `cargo build` o `cargo doc` encuentra un ítem público sin `///`,
se convierte en un **error** de compilación (no un warning).

---

## La carpeta `examples/`

Los archivos en `examples/` son binarios independientes que se ejecutan con
`cargo run --example nombre`:

```
mi_crate/
├── src/
│   └── lib.rs
└── examples/
    ├── basico.rs      → cargo run --example basico
    ├── avanzado.rs    → cargo run --example avanzado
    └── benchmarks.rs  → cargo run --example benchmarks
```

```rust
// examples/basico.rs
use mi_crate::{slugify, truncar, contar_palabras};

fn main() {
    let textos = vec![
        "Hola Mundo Rust",
        "  múltiples   espacios  ",
        "Un texto bastante largo que necesita truncado",
    ];

    for texto in &textos {
        println!("Original:  {texto}");
        println!("Slug:      {}", slugify(texto));
        println!("Truncado:  {}", truncar(texto, 10, "..."));
        println!("Palabras:  {}", contar_palabras(texto));
        println!();
    }
}
```

Los ejemplos también se pueden incluir en la documentación con
`[`basico.rs`](../examples/basico.rs)`.

---

## Generar y publicar documentación

```bash
# Ver docs localmente
cargo doc --open

# Solo doctests (más rápido que cargo test completo)
cargo test --doc

# Ver qué tests se ejecutarán
cargo test --doc -- --list

# Generar docs con features privadas visibles
cargo doc --document-private-items

# Publicar en docs.rs (automático al publicar en crates.io)
cargo publish  # docs.rs genera docs automáticamente
```

---

## Enlazar a otros ítems en la documentación

Rust soporta **intra-doc links** (enlaces a otros ítems del mismo crate):

```rust
use thiserror::Error;

/// Procesa un archivo y retorna un [`Reporte`].
///
/// # Errors
///
/// Retorna [`ProcesadorError::ArchivoNoEncontrado`] si la ruta no existe.
/// Retorna [`ProcesadorError::FormatoInvalido`] si el archivo no es CSV.
pub fn procesar(ruta: &str) -> Result<Reporte, ProcesadorError> { ... }

/// Resultado del procesamiento. Véase también [`procesar`].
pub struct Reporte { ... }

#[derive(Debug, Error)]
pub enum ProcesadorError {
    /// El archivo no existe en la ruta indicada.
    #[error("archivo no encontrado: {0}")]
    ArchivoNoEncontrado(String),
    ...
}
```

Los links entre `[`` ``ProcesadorError::ArchivoNoEncontrado``]` se resuelven
en tiempo de compilación de docs — si el ítem no existe, es un warning.

---

## Comparación con otros lenguajes

| Lenguaje | Sistema de docs | Tests en docs |
|----------|----------------|---------------|
| **Rust** | `///` → HTML via `rustdoc` | Sí — `cargo test --doc` |
| Python | Docstrings → Sphinx | Sí — `doctest` |
| Java | Javadoc | No directamente |
| Go | Comentarios `//` | Sí — `go test` |
| JavaScript | JSDoc | No estándar |

La característica única de Rust es que los doctests son **primeros ciudadanos**:
son tests reales que fallan el CI si el ejemplo está desactualizado.

---

## Errores comunes en documentación

| Error | Consecuencia | Solución |
|-------|--------------|----------|
| Olvidar `use` en el doctest | Falla en `cargo test --doc` | Añadir `# use mi_crate::Tipo;` |
| Ejemplo inconsistente con la implementación | Falla en CI | Los doctests lo detectan |
| Doc de `pub use` sin doc propia | Warning con `missing_docs` | Añadir `///` al re-export |
| Sección `# Safety` ausente en `unsafe fn` | Riesgo de uso incorrecto | Documentar invariantes |
