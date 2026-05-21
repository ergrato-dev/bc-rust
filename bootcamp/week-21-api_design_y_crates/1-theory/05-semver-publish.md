# 📖 05 — Semver, CHANGELOG y `cargo publish`

## Semantic Versioning (semver)

Todos los crates de Rust siguen `semver`: **MAJOR.MINOR.PATCH**

```
     1  .  2  .  3
     │     │     │
     │     │     └── PATCH — corrección de bugs (compatible)
     │     └──────── MINOR — nueva funcionalidad (compatible)
     └────────────── MAJOR — cambio rompedor (incompatible)
```

### ¿Qué es un "breaking change"? (bump de MAJOR)

| Cambio | Breaking? |
|--------|-----------|
| Eliminar una función pública | ✅ Sí |
| Cambiar la firma de una función pública | ✅ Sí |
| Cambiar un enum público (añadir variante) | ✅ Sí (puede romper `match` exhaustivo) |
| Cambiar un campo público de struct | ✅ Sí |
| Añadir una función pública nueva | ❌ No |
| Añadir un campo con `Default` a un struct no-exhaustivo | ❌ No |
| Corregir un bug (cambio de comportamiento) | ⚠️ Depende |

### Versión 0.x — prerrelease especial

Cuando el crate es `0.y.z` (antes de la primera versión estable):
- `0.y` se comporta como MAJOR — `0.1 → 0.2` puede romper la API
- `0.y.z` se comporta como MINOR/PATCH

---

## Gestionar la versión en `Cargo.toml`

```toml
[package]
name    = "mi-crate"
version = "0.1.0"      # ← actualizar con cada release
edition = "2021"
```

La versión en `Cargo.toml` es la "fuente de verdad". Cambiarla a mano:

```bash
# Forma manual (editar Cargo.toml)
# version = "0.1.0"  →  version = "0.2.0"

# Con cargo-edit (más cómodo)
cargo set-version 0.2.0

# Ver la versión actual
cargo metadata --no-deps --format-version 1 | jq '.packages[0].version'
```

---

## El atributo `#[deprecated]`

Cuando una función o tipo deja de ser la forma recomendada pero no puedes
eliminarlo sin romper la API, márcalo como deprecado:

```rust
/// Suma dos números.
///
/// # Deprecated
///
/// Usa [`sumar_con_precision`] en su lugar, que evita pérdida de precisión
/// con números de punto flotante.
#[deprecated(since = "0.2.0", note = "Usa `sumar_con_precision` en su lugar")]
pub fn sumar(a: f64, b: f64) -> f64 {
    a + b
}

/// Suma dos números con precisión configurable.
pub fn sumar_con_precision(a: f64, b: f64, decimales: u32) -> f64 {
    let factor = 10_f64.powi(decimales as i32);
    (a * factor + b * factor).round() / factor
}
```

El caller verá un warning al usar `sumar`:
```
warning: use of deprecated function `mi_crate::sumar`: Usa `sumar_con_precision` en su lugar
```

Para silenciar en tests que aún cubren la función deprecada:
```rust
#[test]
#[allow(deprecated)]
fn test_sumar_legacy() {
    assert_eq!(sumar(1.0, 2.0), 3.0);
}
```

---

## CHANGELOG.md — formato Keep a Changelog

El proyecto [keepachangelog.com](https://keepachangelog.com/) define una
convención estándar para documentar cambios entre versiones:

```markdown
# Changelog

Todos los cambios notables se documentan en este archivo.
Formato basado en [Keep a Changelog](https://keepachangelog.com/es/1.0.0/).
Este proyecto usa [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Nueva función `procesar_lote` para procesar múltiples elementos

## [0.2.0] — 2026-05-21
### Added
- `Calculadora::con_precision(usize)` — constructor con decimales configurables
- `Calculadora::historial()` — acceso al historial de operaciones

### Changed
- `Calculadora::dividir` ahora retorna `Result<f64, CalculadoraError>` en
  lugar de `f64` (retornaba NaN en división por cero)

## [0.1.0] — 2026-04-15
### Added
- API inicial: `Calculadora::new`, `sumar`, `restar`, `multiplicar`, `dividir`

[Unreleased]: https://github.com/usuario/mi-crate/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/usuario/mi-crate/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/usuario/mi-crate/releases/tag/v0.1.0
```

### Categorías del CHANGELOG

| Categoría | Qué incluye |
|-----------|-------------|
| `Added` | Nuevas funciones, tipos, módulos |
| `Changed` | Cambios en funcionalidad existente |
| `Deprecated` | Funciones marcadas con `#[deprecated]` |
| `Removed` | Funciones o tipos eliminados |
| `Fixed` | Correcciones de bugs |
| `Security` | Correcciones de vulnerabilidades |

---

## Preparar el `Cargo.toml` para publicación

`crates.io` requiere cierta metadata. Ejemplo completo:

```toml
[package]
name        = "mi-crate"
version     = "0.1.0"
edition     = "2021"
description = "Utilidades de procesamiento de texto para Rust"  # obligatorio
license     = "MIT OR Apache-2.0"          # obligatorio (SPDX identifier)
authors     = ["Tu Nombre <tu@email.com>"] # recomendado
repository  = "https://github.com/usuario/mi-crate"  # recomendado
homepage    = "https://mi-crate.rs"        # opcional
documentation = "https://docs.rs/mi-crate" # opcional (auto-generado)
readme      = "README.md"                  # recomendado
keywords    = ["text", "utils", "string"]  # máx. 5
categories  = ["text-processing"]         # de la lista de crates.io
exclude     = ["tests/fixtures/**", ".github/**"]  # no incluir en el crate

[dependencies]
thiserror = "2.0.12"
```

### Licencias válidas en `crates.io`

```toml
# Una licencia
license = "MIT"

# Doble licencia (lo más común en Rust)
license = "MIT OR Apache-2.0"

# Licencia en archivo separado
license-file = "LICENSE.txt"
```

---

## `cargo publish` — el flujo completo

```bash
# 1. Crear cuenta en crates.io y obtener API token
#    → https://crates.io/settings/tokens

# 2. Autenticarse localmente
cargo login <API_TOKEN>

# 3. Simular la publicación (no sube nada)
cargo publish --dry-run

# 4. Ver qué archivos se incluirán
cargo package --list

# 5. Si todo está bien, publicar
cargo publish

# Publicar una versión específica
cargo publish --package mi-crate
```

### ¿Qué incluye el paquete publicado?

Por defecto, Cargo incluye:
- Todo lo rastreado por git
- `Cargo.toml` y `Cargo.lock` (si existe)
- El `src/` del crate

Se excluye con `exclude` en `Cargo.toml` o con `.cargo_vcs_info.json`.

---

## Reglas de `crates.io` — no olvidar

1. **Los crates son permanentes**: una vez publicado, no puedes eliminar una versión
   (solo "yank" — marcarlo como no recomendado, pero sigue descargable)
2. **El nombre es primero en llegar**: si `mi-crate` ya existe, no puedes usarlo
3. **La versión solo sube**: `0.1.0 → 0.1.1` pero nunca `0.1.1 → 0.1.0`
4. **Máximo 10 MB** por publicación

```bash
# Si publicas accidentalmente una versión con bugs:
cargo yank --version 0.1.0 --package mi-crate

# Para des-yankar
cargo yank --undo --version 0.1.0 --package mi-crate
```

---

## Verificar antes de publicar — checklist

```bash
# Compilación limpia
cargo build --release

# Tests pasan (incluidos doctests)
cargo test
cargo test --doc

# Clippy sin warnings
cargo clippy -- -D warnings

# Formato correcto
cargo fmt --check

# Documentación generada sin warnings
cargo doc --no-deps 2>&1 | grep -i warning

# Simulación de publicación
cargo publish --dry-run

# Lista de archivos que se incluirán
cargo package --list
```

---

## Comparación con otros ecosistemas

| Aspecto | Rust (crates.io) | npm | PyPI |
|---------|-----------------|-----|------|
| Versiones permanentes | Sí (solo yank) | No (unpublish posible) | Sí |
| Semver obligatorio | Convención fuerte | Convención | No |
| Docs automáticas | docs.rs | No | Read the Docs (manual) |
| Auditoría CVE | `cargo audit` | `npm audit` | `pip-audit` |
| Namespace | Global (sin scope) | `@scope/paquete` | Global |
| Autenticación | Token de API | Token de API | Usuario+contraseña o token |
