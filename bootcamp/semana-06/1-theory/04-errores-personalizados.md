# 🎨 Errores Personalizados

> **Creando tipos de error expresivos y útiles**

![Errores Personalizados](../0-assets/04-errores-personalizados.svg)

---

## ¿Por Qué Crear Errores Custom?

Los errores de la biblioteca estándar son genéricos:

```rust
// io::Error no dice qué archivo falló
let contenido = std::fs::read_to_string("config.txt")?;

// ParseIntError no dice qué string era
let n: i32 = "abc".parse()?;
```

Errores personalizados agregan **contexto**.

---

## Enfoque 1: Struct Simple

```rust
#[derive(Debug)]
struct ConfigError {
    mensaje: String,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error de configuración: {}", self.mensaje)
    }
}

impl std::error::Error for ConfigError {}
```

---

## Enfoque 2: Enum con Variantes

Más flexible y expresivo:

```rust
#[derive(Debug)]
enum AppError {
    ArchivoNoEncontrado(String),
    ParseError { linea: usize, detalle: String },
    Validacion(Vec<String>),
    Conexion { host: String, puerto: u16 },
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::ArchivoNoEncontrado(ruta) => 
                write!(f, "Archivo no encontrado: {}", ruta),
            AppError::ParseError { linea, detalle } => 
                write!(f, "Error en línea {}: {}", linea, detalle),
            AppError::Validacion(errores) => 
                write!(f, "Validación fallida: {}", errores.join(", ")),
            AppError::Conexion { host, puerto } => 
                write!(f, "No se pudo conectar a {}:{}", host, puerto),
        }
    }
}

impl std::error::Error for AppError {}
```

---

## El Trait Error

```rust
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> { None }
}
```

Requisitos:
- Implementar `Debug` (usualmente con `#[derive(Debug)]`)
- Implementar `Display` (mensaje legible)
- Opcionalmente, `source()` para encadenar errores

---

## Encadenando Errores con source()

```rust
use std::error::Error;
use std::io;

#[derive(Debug)]
struct ConfigError {
    ruta: String,
    causa: io::Error,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error leyendo config desde {}", self.ruta)
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.causa)
    }
}
```

Permite mostrar la cadena completa de errores.

---

## Implementando From

Para usar `?` automáticamente:

```rust
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
enum DataError {
    Io(io::Error),
    Parse(ParseIntError),
}

impl From<io::Error> for DataError {
    fn from(err: io::Error) -> Self {
        DataError::Io(err)
    }
}

impl From<ParseIntError> for DataError {
    fn from(err: ParseIntError) -> Self {
        DataError::Parse(err)
    }
}

// Ahora esto funciona:
fn leer_numero(ruta: &str) -> Result<i32, DataError> {
    let s = std::fs::read_to_string(ruta)?;  // io::Error → DataError
    let n = s.trim().parse()?;                // ParseIntError → DataError
    Ok(n)
}
```

---

## Patrón Completo

```rust
use std::{error::Error, fmt, io};

#[derive(Debug)]
pub enum AppError {
    Io { 
        operacion: &'static str, 
        fuente: io::Error 
    },
    Config { 
        campo: String, 
        mensaje: String 
    },
    Validacion(Vec<String>),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io { operacion, .. } => 
                write!(f, "Error de I/O durante {}", operacion),
            AppError::Config { campo, mensaje } => 
                write!(f, "Config inválida - {}: {}", campo, mensaje),
            AppError::Validacion(errores) => {
                write!(f, "Errores de validación:\n")?;
                for e in errores {
                    write!(f, "  - {}\n", e)?;
                }
                Ok(())
            }
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io { fuente, .. } => Some(fuente),
            _ => None,
        }
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io { 
            operacion: "operación desconocida", 
            fuente: err 
        }
    }
}
```

---

## Constructores de Conveniencia

```rust
impl AppError {
    pub fn io_leer(fuente: io::Error) -> Self {
        AppError::Io { operacion: "leer archivo", fuente }
    }
    
    pub fn io_escribir(fuente: io::Error) -> Self {
        AppError::Io { operacion: "escribir archivo", fuente }
    }
    
    pub fn config(campo: &str, mensaje: &str) -> Self {
        AppError::Config {
            campo: campo.to_string(),
            mensaje: mensaje.to_string(),
        }
    }
}

// Uso:
fn cargar() -> Result<(), AppError> {
    std::fs::read_to_string("config.txt")
        .map_err(AppError::io_leer)?;
    Ok(())
}
```

---

## Result Type Alias

Patrón común en bibliotecas:

```rust
pub type Result<T> = std::result::Result<T, AppError>;

// Ahora las funciones son más limpias:
pub fn cargar_config() -> Result<Config> {
    // ...
}

pub fn guardar_config(config: &Config) -> Result<()> {
    // ...
}
```

---

## Crates Populares para Errores

### thiserror (más común)

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("Error de I/O: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Error de parseo en línea {linea}: {mensaje}")]
    Parse { linea: usize, mensaje: String },
    
    #[error("Valor fuera de rango: {0} (esperado {min}-{max})")]
    OutOfRange { valor: i32, min: i32, max: i32 },
}
```

### anyhow (para aplicaciones)

```rust
use anyhow::{Context, Result};

fn cargar_config() -> Result<Config> {
    let contenido = std::fs::read_to_string("config.toml")
        .context("No se pudo leer config.toml")?;
    
    let config: Config = toml::from_str(&contenido)
        .context("Config mal formateada")?;
    
    Ok(config)
}
```

---

## Cuándo Usar Cada Enfoque

| Situación | Enfoque |
|-----------|---------|
| Biblioteca pública | Tipos custom con `thiserror` |
| Aplicación CLI/servidor | `anyhow` para simplicidad |
| Proyecto pequeño | `Box<dyn Error>` |
| Código crítico | Tipos custom manuales |

---

## Resumen

```rust
// 1. Definir el tipo
#[derive(Debug)]
enum MiError { /* variantes */ }

// 2. Implementar Display
impl fmt::Display for MiError { /* ... */ }

// 3. Implementar Error
impl Error for MiError { /* opcional: source() */ }

// 4. Implementar From para conversión automática
impl From<io::Error> for MiError { /* ... */ }

// 5. (Opcional) Type alias
type Result<T> = std::result::Result<T, MiError>;
```

---

## 🧪 Ejercicio Mental

Diseña un tipo de error para un parser de JSON:

- Error de sintaxis (con línea y columna)
- Tipo inesperado (esperado vs encontrado)
- Campo faltante (nombre del campo)
- Valor fuera de rango

<details>
<summary>Ver respuesta</summary>

```rust
#[derive(Debug)]
enum JsonError {
    Syntax { linea: usize, columna: usize, mensaje: String },
    TipoInesperado { esperado: String, encontrado: String },
    CampoFaltante(String),
    FueraDeRango { valor: i64, min: i64, max: i64 },
}
```

</details>

---

## 📚 Siguiente

[Patrones y Buenas Prácticas →](05-patrones-practicas.md)
