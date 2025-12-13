# 📖 Glosario - Semana 07: Módulos y Crates

## Términos Fundamentales

### Crate

**Definición**: La unidad de compilación más pequeña en Rust. Puede ser una biblioteca (library) o un ejecutable (binary).

```rust
// Un crate binario tiene main.rs
// Un crate biblioteca tiene lib.rs
```

**Tipos**:
- **Binary crate**: Produce un ejecutable (`src/main.rs`)
- **Library crate**: Produce una biblioteca reutilizable (`src/lib.rs`)

---

### Module (Módulo)

**Definición**: Una unidad de organización de código que agrupa funciones, structs, enums, traits y otros módulos relacionados.

```rust
mod animales {
    pub mod perros {
        pub fn ladrar() { println!("¡Guau!"); }
    }
}
```

---

### Package (Paquete)

**Definición**: Un conjunto de uno o más crates que proveen funcionalidad relacionada. Definido por `Cargo.toml`.

**Reglas**:
- Puede contener múltiples binary crates
- Solo puede contener un library crate
- Debe contener al menos un crate

---

### Path (Ruta)

**Definición**: La forma de referirse a un item (función, struct, módulo) dentro del árbol de módulos.

```rust
// Path absoluto
crate::modulo::funcion()

// Path relativo
super::funcion_padre()
self::funcion_local()
```

---

## Palabras Clave

### `mod`

**Definición**: Declara un nuevo módulo o referencia a un archivo de módulo externo.

```rust
// Módulo inline
mod interno {
    pub fn funcion() {}
}

// Referencia a archivo externo
mod externo; // busca externo.rs o externo/mod.rs
```

---

### `pub`

**Definición**: Modificador de visibilidad que hace público un item.

```rust
pub fn publica() {}          // Público total
pub(crate) fn crate_() {}    // Visible en el crate
pub(super) fn padre() {}     // Visible en módulo padre
pub(in crate::ruta) fn() {}  // Visible en ruta específica
fn privada() {}              // Privado (default)
```

---

### `use`

**Definición**: Trae items al scope actual para usarlos sin el path completo.

```rust
use std::collections::HashMap;
use std::io::{self, Read, Write};
use crate::modulo::Item as MiItem;  // Renombrar
```

---

### `as`

**Definición**: Renombra un item al importarlo, útil para evitar conflictos de nombres.

```rust
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;
```

---

### `crate`

**Definición**: Referencia a la raíz del crate actual. Base para paths absolutos.

```rust
use crate::config::Config;
crate::utilidades::helper()
```

---

### `self`

**Definición**: Referencia al módulo actual.

```rust
use self::submodulo::Funcion;
self::funcion_local()
```

---

### `super`

**Definición**: Referencia al módulo padre del actual.

```rust
use super::Tipo;
super::funcion_padre()
```

---

## Archivos Especiales

### `main.rs`

**Definición**: Punto de entrada de un binary crate. Contiene la función `main()`.

---

### `lib.rs`

**Definición**: Raíz de un library crate. Define la API pública de la biblioteca.

---

### `mod.rs`

**Definición**: (Estilo antiguo) Archivo que representa un módulo en una carpeta. Equivalente al nombre de la carpeta.

```
src/
├── animales/
│   └── mod.rs    ← representa mod animales
```

---

### `Cargo.toml`

**Definición**: Manifiesto del proyecto. Define metadata, dependencias y configuración.

```toml
[package]
name = "mi-proyecto"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = "1.0"
```

---

### `Cargo.lock`

**Definición**: Archivo generado automáticamente que registra las versiones exactas de todas las dependencias. Debe incluirse en control de versiones para binarios.

---

## Conceptos de Visibilidad

### Privado (Default)

**Definición**: Sin modificador. Solo visible dentro del mismo módulo.

---

### `pub` (Público)

**Definición**: Visible desde cualquier lugar que pueda acceder al módulo padre.

---

### `pub(crate)`

**Definición**: Visible en cualquier parte del crate actual, pero no externamente.

---

### `pub(super)`

**Definición**: Visible solo para el módulo padre inmediato.

---

### `pub(in path)`

**Definición**: Visible solo dentro del path especificado.

```rust
pub(in crate::modulo) fn visible_en_modulo() {}
```

---

## Términos de Cargo

### Dependency (Dependencia)

**Definición**: Un crate externo requerido por el proyecto.

```toml
[dependencies]
tokio = "1.0"
```

---

### Dev Dependency

**Definición**: Dependencia solo necesaria durante desarrollo/testing.

```toml
[dev-dependencies]
criterion = "0.5"
```

---

### Feature

**Definición**: Compilación condicional opcional de funcionalidad.

```toml
[features]
default = ["json"]
json = ["serde_json"]
```

---

### Workspace

**Definición**: Colección de paquetes relacionados que comparten `Cargo.lock` y directorio `target/`.

```toml
[workspace]
members = ["core", "cli", "server"]
```

---

### Edition

**Definición**: Versión del lenguaje Rust (2015, 2018, 2021, 2024). Afecta comportamiento de módulos.

---

## Patrones Comunes

### Re-export (Re-exportación)

**Definición**: Hacer público un item de un submódulo en un nivel superior.

```rust
// En lib.rs
pub use internal::PublicType;
```

---

### Prelude

**Definición**: Conjunto de items comúnmente usados re-exportados para import conveniente.

```rust
// biblioteca/prelude.rs
pub use crate::{TipoA, TipoB, trait_comun::*};

// Usuario
use biblioteca::prelude::*;
```

---

### Glob Import

**Definición**: Importar todos los items públicos de un módulo con `*`.

```rust
use std::collections::*;  // ¡Usar con precaución!
```

---

## Comandos Cargo Relacionados

| Comando | Descripción |
|---------|-------------|
| `cargo new` | Crear nuevo proyecto |
| `cargo add` | Agregar dependencia |
| `cargo tree` | Ver árbol de dependencias |
| `cargo doc` | Generar documentación |
| `cargo publish` | Publicar a crates.io |
| `cargo update` | Actualizar dependencias |
