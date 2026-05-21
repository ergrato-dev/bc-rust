# 📖 05 — Bindings Python con PyO3 y Node.js con napi-rs

## PyO3: Rust como extensión Python

PyO3 permite escribir módulos Python nativos en Rust. El módulo compilado
es un archivo `.so` (Linux) que Python importa directamente — sin overhead
de IPC ni subprocesos, con rendimiento idéntico al código C nativo.

```
┌──────────────────────────────────────────────────────────────────┐
│                   ARQUITECTURA PyO3                              │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Python script                                                   │
│  ┌─────────────────────────────────┐                            │
│  │  import mi_modulo                │                            │
│  │  resultado = mi_modulo.calcular()│                            │
│  └────────────────┬────────────────┘                            │
│                   │  CPython C API                               │
│                   ▼                                              │
│  ┌─────────────────────────────────┐                            │
│  │  mi_modulo.so (cdylib)           │                            │
│  │  ┌───────────────────────────┐  │                            │
│  │  │  Rust + PyO3              │  │                            │
│  │  │  #[pyfunction] calcular() │  │                            │
│  │  └───────────────────────────┘  │                            │
│  └─────────────────────────────────┘                            │
│                                                                  │
│  Mismo proceso, mismo heap — cero overhead de serialización      │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

---

## Configuración inicial con maturin

```bash
# Instalar maturin
pip install maturin

# Crear nuevo proyecto PyO3 desde cero
maturin init --bindings pyo3

# Compilar e instalar en el virtualenv activo (desarrollo)
maturin develop

# Compilar wheel de distribución
maturin build --release
```

`Cargo.toml` mínimo:

```toml
[package]
name    = "mi-extension"
version = "0.1.0"
edition = "2021"

[lib]
name       = "mi_extension"   # nombre del módulo Python
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.23.4", features = ["extension-module"] }
```

---

## `#[pyfunction]` — funciones Python

```rust
use pyo3::prelude::*;

/// Suma dos números — docstring visible desde Python como `help(suma)`.
#[pyfunction]
fn suma(a: f64, b: f64) -> f64 {
    a + b
}

/// Función que puede fallar → retorna PyResult
#[pyfunction]
fn dividir(a: f64, b: f64) -> PyResult<f64> {
    if b == 0.0 {
        // Lanzar excepción Python ZeroDivisionError
        return Err(pyo3::exceptions::PyZeroDivisionError::new_err(
            "división por cero"
        ));
    }
    Ok(a / b)
}

/// Acepta tipos Python nativos
#[pyfunction]
fn contar_palabras(texto: &str) -> usize {
    texto.split_whitespace().count()
}

/// Trabaja con listas Python
#[pyfunction]
fn suma_lista(numeros: Vec<f64>) -> f64 {
    numeros.iter().sum()
}
```

---

## `#[pyclass]` — clases Python

```rust
use pyo3::prelude::*;

/// Clase expuesta a Python como `mi_extension.Pila`
#[pyclass]
pub struct Pila {
    datos: Vec<i64>,
}

#[pymethods]
impl Pila {
    /// Constructor: `Pila()` en Python
    #[new]
    fn new() -> Self {
        Pila { datos: Vec::new() }
    }

    /// `pila.push(valor)`
    fn push(&mut self, valor: i64) {
        self.datos.push(valor);
    }

    /// `pila.pop()` — retorna None si está vacía
    fn pop(&mut self) -> Option<i64> {
        self.datos.pop()
    }

    /// `len(pila)` — protocolo Python
    fn __len__(&self) -> usize {
        self.datos.len()
    }

    /// `repr(pila)` — representación legible
    fn __repr__(&self) -> String {
        format!("Pila({:?})", self.datos)
    }

    /// Propiedad: `pila.vacia`
    #[getter]
    fn vacia(&self) -> bool {
        self.datos.is_empty()
    }
}
```

---

## `#[pymodule]` — registrar el módulo

```rust
use pyo3::prelude::*;

// Funciones y clases definidas arriba...

/// El nombre de la función DEBE coincidir con el `name` en Cargo.toml [lib].
#[pymodule]
fn mi_extension(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(suma, m)?)?;
    m.add_function(wrap_pyfunction!(dividir, m)?)?;
    m.add_function(wrap_pyfunction!(contar_palabras, m)?)?;
    m.add_function(wrap_pyfunction!(suma_lista, m)?)?;
    m.add_class::<Pila>()?;
    // Constantes
    m.add("VERSION", "1.0.0")?;
    Ok(())
}
```

Uso desde Python:

```python
import mi_extension

print(mi_extension.suma(3.0, 4.0))     # 7.0
print(mi_extension.dividir(10.0, 2.0)) # 5.0

pila = mi_extension.Pila()
pila.push(42)
pila.push(99)
print(len(pila))   # 2
print(pila.pop())  # 99
```

---

## napi-rs: Rust como addon Node.js

`napi-rs` es el framework oficial para addons nativos de Node.js/Deno
escritos en Rust. Usa la API N-API (ahora llamada Node-API) de Node.js,
que garantiza compatibilidad entre versiones.

```
┌──────────────────────────────────────────────────────────────────┐
│                  ARQUITECTURA napi-rs                            │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  JavaScript / TypeScript                                         │
│  ┌──────────────────────────────────────┐                       │
│  │  const { suma } = require('./index') │                       │
│  │  suma(3, 4)  // 7                    │                       │
│  └───────────────────┬──────────────────┘                       │
│                      │  Node-API (ABI estable)                   │
│                      ▼                                           │
│  ┌──────────────────────────────────────┐                       │
│  │  index.node (cdylib)                 │                       │
│  │  ┌────────────────────────────────┐  │                       │
│  │  │  Rust + napi-rs                │  │                       │
│  │  │  #[napi] pub fn suma(...)      │  │                       │
│  │  └────────────────────────────────┘  │                       │
│  └──────────────────────────────────────┘                       │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

---

## Configuración napi-rs

```toml
# Cargo.toml
[package]
name    = "mi-addon"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
napi        = "2.16.14"
napi-derive = "2.16.14"

[build-dependencies]
napi-build = "2.1.4"
```

```rust
// build.rs — obligatorio para napi-rs
fn main() {
    napi_build::setup();
}
```

---

## `#[napi]` — funciones y clases para Node.js

```rust
#![deny(clippy::all)]
use napi_derive::napi;

/// Función simple: automáticamente genera la type definition TypeScript.
#[napi]
pub fn suma(a: f64, b: f64) -> f64 {
    a + b
}

/// Función asíncrona — Node.js la llama como Promise.
#[napi]
pub async fn calcular_async(n: u32) -> u64 {
    // En la realidad aquí iría I/O asíncrono con tokio
    (1..=n as u64).sum()
}

/// Clase para Node.js
#[napi]
pub struct Contador {
    valor: i64,
}

#[napi]
impl Contador {
    /// Constructor `new Contador(inicial)`
    #[napi(constructor)]
    pub fn new(inicial: i64) -> Self {
        Contador { valor: inicial }
    }

    /// Método `contador.incrementar()`
    #[napi]
    pub fn incrementar(&mut self) {
        self.valor += 1;
    }

    /// Getter `contador.valor`
    #[napi(getter)]
    pub fn valor(&self) -> i64 {
        self.valor
    }
}
```

---

## Compilar y usar napi-rs

```bash
# Compilar para la plataforma actual
napi build --platform

# Compilar en modo release
napi build --platform --release

# El resultado es index.node (+ index.d.ts con tipos TypeScript)
```

Uso desde JavaScript/TypeScript:

```typescript
import { suma, Contador } from './index'

console.log(suma(3, 4))      // 7

const c = new Contador(0)
c.incrementar()
c.incrementar()
console.log(c.valor)          // 2
```

---

## Comparación PyO3 vs napi-rs vs cbindgen

| Aspecto | PyO3 | napi-rs | cbindgen (C) |
|---------|------|---------|--------------|
| Lenguaje destino | Python | Node.js / Deno | C / C++ / cualquiera |
| Build tool | maturin | napi CLI | cbindgen |
| Tipos automáticos | No (.pyi manual) | Sí (.d.ts) | Sí (.h) |
| Async | `#[pyfunction]` + asyncio | `async fn` + Promise | No (manual) |
| Clases/objetos | `#[pyclass]` | `#[napi]` struct | Opaque pointer |
| Error handling | `PyResult<T>` | `napi::Result<T>` | Código de retorno |
| Madurez | Alta | Alta | Muy alta |

---

## Anti-patrones comunes con PyO3 y napi-rs

```rust
// ❌ PyO3: GIL innecesario
#[pyfunction]
fn calcular(py: Python) -> PyResult<i64> {
    // Adquirir el GIL y no soltarlo durante cómputo pesado
    Ok(heavy_computation()) // Bloquea todo el intérprete Python
}

// ✅ PyO3: liberar el GIL durante cómputo intensivo
#[pyfunction]
fn calcular(py: Python) -> PyResult<i64> {
    py.allow_threads(|| {
        Ok(heavy_computation()) // Python puede ejecutar otros hilos
    })
}

// ❌ napi-rs: bloquear el event loop de Node
#[napi]
pub fn operacion_lenta() -> String {
    std::thread::sleep(std::time::Duration::from_secs(5)); // Bloquea Node!
    "listo".to_string()
}

// ✅ napi-rs: usar async para operaciones lentas
#[napi]
pub async fn operacion_lenta() -> String {
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    "listo".to_string()
}
```

---

## Distribución: maturin publish

```bash
# Publicar en PyPI (requiere cuenta)
maturin publish

# Construir wheels para múltiples plataformas (en CI)
# usa cibuildwheel o maturin action en GitHub Actions

# napi-rs: publicar en npm
napi prepublish
npm publish
```

`maturin` genera wheels que incluyen el binario nativo compilado —
el usuario final instala con `pip install mi-paquete` sin necesidad
de tener Rust instalado.
