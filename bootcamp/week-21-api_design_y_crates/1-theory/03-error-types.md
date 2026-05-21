# 📖 03 — Tipos de Error Custom con `thiserror`

## El panorama del manejo de errores en Rust

Rust tiene dos enfoques principales para errores:

| Crate | Para qué | Cuándo usar |
|-------|----------|-------------|
| `thiserror` | Definir tipos de error de librería | Librerías públicas — el caller debe poder distinguir variantes |
| `anyhow` | Manejar errores en aplicaciones | Binarios y aplicaciones — solo importa el mensaje |
| Estándar (`std::error::Error`) | Base trait | Siempre implementada por ambos |

**Regla de oro**: librerías usan `thiserror`, aplicaciones usan `anyhow`.

---

## El trait `std::error::Error`

Para que un tipo sea "un error", debe implementar:

```rust
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> { None }
}
```

Hacerlo manualmente es verboso:

```rust
// ❌ Manual — mucho boilerplate
use std::fmt;

#[derive(Debug)]
pub enum MiError {
    NoEncontrado(String),
    Io(std::io::Error),
}

impl fmt::Display for MiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MiError::NoEncontrado(s) => write!(f, "no encontrado: {s}"),
            MiError::Io(e) => write!(f, "error de I/O: {e}"),
        }
    }
}

impl std::error::Error for MiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MiError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for MiError {
    fn from(e: std::io::Error) -> Self { MiError::Io(e) }
}
```

---

## `thiserror` — el derive que elimina el boilerplate

```rust
use thiserror::Error;

// ✅ Equivalente exacto al código manual de arriba
#[derive(Debug, Error)]
pub enum MiError {
    #[error("no encontrado: {0}")]
    NoEncontrado(String),

    #[error("error de I/O: {0}")]
    #[from]
    Io(#[source] std::io::Error),
}
```

### Atributos de `thiserror`

| Atributo | Efecto | Ejemplo |
|----------|--------|---------|
| `#[error("mensaje")]` | Implementa `Display` | `#[error("tiempo agotado")]` |
| `#[error("{0}")]` | Interpola primer campo | `#[error("error: {0}")]` |
| `#[error("{campo}")]` | Interpola campo nombrado | `#[error("inválido: {valor}")]` |
| `#[from]` | Genera `From<T>` automáticamente | Ver arriba |
| `#[source]` | Marca el campo como fuente del error | Habilita `.source()` |

---

## Diseño de jerarquías de error

### Errores simples — una sola variante

```rust
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("se esperaba un número, encontré '{0}'")]
    NumeroInvalido(String),

    #[error("el valor {0} está fuera del rango permitido [0, 100]")]
    FueraDeRango(i64),
}
```

### Errores compuestos — wrappear errores de dependencias

```rust
#[derive(Debug, Error)]
pub enum AppError {
    #[error("error de base de datos: {0}")]
    Db(#[from] sqlx::Error),

    #[error("error de I/O: {0}")]
    Io(#[from] std::io::Error),

    #[error("error de serialización: {0}")]
    Json(#[from] serde_json::Error),

    #[error("configuración inválida: {campo} — {motivo}")]
    ConfigInvalida { campo: String, motivo: String },
}
```

---

## Errores públicos vs. internos

Una librería bien diseñada separa:

```
mi_libreria/
├── src/
│   ├── lib.rs
│   ├── error.rs       ← Errores PÚBLICOS (parte de la API)
│   └── internal/
│       └── error.rs   ← Errores INTERNOS (no expuestos)
```

```rust
// error.rs — API pública
pub use crate::internal::error::InternalError;  // ❌ No exponer internos

// ✅ Convertir internos a públicos en la frontera
#[derive(Debug, Error)]
pub enum LibError {
    #[error("operación no soportada")]
    NotSupported,

    #[error("entrada inválida: {0}")]
    InvalidInput(String),
}

// Internamente: convertir el error interno al público
impl From<InternalParseError> for LibError {
    fn from(e: InternalParseError) -> Self {
        LibError::InvalidInput(e.to_string())
    }
}
```

---

## El operador `?` con tipos de error custom

Con `#[from]`, el operador `?` convierte automáticamente:

```rust
use std::fs;

pub fn leer_config(path: &str) -> Result<Config, AppError> {
    // std::io::Error → AppError::Io gracias a #[from]
    let contenido = fs::read_to_string(path)?;

    // serde_json::Error → AppError::Json gracias a #[from]
    let config: Config = serde_json::from_str(&contenido)?;

    Ok(config)
}
```

Sin `#[from]`, necesitarías `.map_err(AppError::Io)?` en cada lugar.

---

## `anyhow` — para aplicaciones y binarios

```rust
// Cargo.toml: anyhow = "1.0.97"
use anyhow::{Context, Result};

fn main() -> Result<()> {
    // anyhow::Result = Result<T, anyhow::Error>
    // anyhow::Error puede wrappear CUALQUIER error

    let config = leer_config("config.toml")
        .context("al leer la configuración")?;  // agrega contexto

    ejecutar(config)
        .context("al ejecutar la aplicación")?;

    Ok(())
}
```

`anyhow` construye cadenas de contexto:
```
Error: al ejecutar la aplicación

Caused by:
    0: al leer la configuración
    1: No such file or directory (os error 2)
```

---

## Definir errores específicos del dominio

Errores que modelan situaciones del negocio (no solo técnicas):

```rust
#[derive(Debug, Error, PartialEq)]
pub enum PagoError {
    #[error("saldo insuficiente: disponible {disponible:.2}, requerido {requerido:.2}")]
    SaldoInsuficiente { disponible: f64, requerido: f64 },

    #[error("tarjeta expirada desde {mes:02}/{anio}")]
    TarjetaExpirada { mes: u32, anio: u32 },

    #[error("límite diario de {limite:.2} alcanzado")]
    LimiteDiarioAlcanzado { limite: f64 },

    #[error("transacción duplicada: id={id}")]
    TransaccionDuplicada { id: String },
}
```

Estos errores son parte de la API pública y el caller puede hacer `match`
para tratarlos de forma específica:

```rust
match procesar_pago(monto, tarjeta) {
    Ok(confirmacion) => confirmar_al_usuario(&confirmacion),
    Err(PagoError::SaldoInsuficiente { disponible, .. }) => {
        mostrar_saldo_insuficiente(disponible)
    }
    Err(PagoError::TarjetaExpirada { mes, anio }) => {
        redirigir_renovacion(mes, anio)
    }
    Err(e) => log_error_generico(&e),
}
```

---

## Tests de errores

Los tipos de error con `PartialEq` se pueden verificar directamente:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_saldo_insuficiente() {
        let res = procesar_pago(1000.0, saldo_disponible: 50.0);
        assert_eq!(
            res.unwrap_err(),
            PagoError::SaldoInsuficiente { disponible: 50.0, requerido: 1000.0 }
        );
    }

    #[test]
    fn error_display_es_descriptivo() {
        let e = PagoError::SaldoInsuficiente { disponible: 50.0, requerido: 100.0 };
        assert!(e.to_string().contains("50.00"));
        assert!(e.to_string().contains("100.00"));
    }
}
```

---

## Checklist de errores en una librería

Antes de publicar tu crate, verifica:

- [ ] Todos los errores públicos implementan `Error + Debug + Display`
- [ ] Los mensajes de `#[error("...")]` son descriptivos (incluyen el valor)
- [ ] Los errores son `PartialEq` cuando es posible (facilita tests)
- [ ] Los errores internos no se exponen en la API pública
- [ ] Hay tests para cada variante de error
- [ ] Los errores de terceros se convierten a errores propios con `#[from]`
