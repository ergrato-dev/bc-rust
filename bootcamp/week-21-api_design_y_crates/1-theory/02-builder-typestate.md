# 📖 02 — Patrón Builder: Clásico y Typestate

## El problema que resuelve Builder

Los constructores con muchos argumentos son frágiles:

```rust
// ❌ 6 parámetros: difícil de leer, fácil de confundir orden
fn crear_servidor(host: String, puerto: u16, workers: usize,
                  timeout_ms: u64, reintentos: u32, tls: bool) -> Servidor { ... }

// Llamada confusa — ¿qué era el 5? ¿y el 6?
crear_servidor("0.0.0.0".into(), 8080, 4, 5000, 3, true);
```

El **patrón Builder** resuelve esto:

```rust
// ✅ Fluent interface — cada llamada documenta su intención
let servidor = ServidorBuilder::new()
    .host("0.0.0.0")
    .puerto(8080)
    .workers(4)
    .timeout_ms(5000)
    .reintentos(3)
    .tls(true)
    .build()?;
```

---

## Builder Clásico — implementación paso a paso

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServidorError {
    #[error("el host no puede estar vacío")]
    HostVacio,
    #[error("el puerto {0} está reservado (< 1024), usa sudo o elige otro")]
    PuertoReservado(u16),
}

#[derive(Debug)]
pub struct Servidor {
    pub host: String,
    pub puerto: u16,
    pub workers: usize,
    pub timeout_ms: u64,
    pub tls: bool,
}

/// Builder para `Servidor`.
///
/// # Examples
///
/// ```
/// # use mi_modulo::ServidorBuilder;
/// let srv = ServidorBuilder::new()
///     .host("127.0.0.1")
///     .puerto(8080)
///     .build()
///     .unwrap();
/// assert_eq!(srv.puerto, 8080);
/// ```
#[derive(Default)]
pub struct ServidorBuilder {
    host:       Option<String>,
    puerto:     Option<u16>,
    workers:    usize,
    timeout_ms: u64,
    tls:        bool,
}

impl ServidorBuilder {
    pub fn new() -> Self {
        ServidorBuilder {
            workers:    4,
            timeout_ms: 5000,
            ..Default::default()
        }
    }

    // impl Into<String> — acepta &str y String sin conversión explícita
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    pub fn puerto(mut self, puerto: u16) -> Self {
        self.puerto = Some(puerto);
        self
    }

    pub fn workers(mut self, workers: usize) -> Self {
        self.workers = workers;
        self
    }

    pub fn timeout_ms(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }

    pub fn tls(mut self, tls: bool) -> Self {
        self.tls = tls;
        self
    }

    /// Valida los campos y construye el `Servidor`.
    pub fn build(self) -> Result<Servidor, ServidorError> {
        let host = self.host.unwrap_or_default();
        if host.is_empty() {
            return Err(ServidorError::HostVacio);
        }
        let puerto = self.puerto.unwrap_or(8080);
        if puerto < 1024 {
            return Err(ServidorError::PuertoReservado(puerto));
        }
        Ok(Servidor { host, puerto, workers: self.workers,
                       timeout_ms: self.timeout_ms, tls: self.tls })
    }
}
```

### Diagrama de flujo del Builder clásico

```
ServidorBuilder::new()
        │
        ▼  .host("0.0.0.0")
  { host: Some("0.0.0.0"), puerto: None, ... }
        │
        ▼  .puerto(8080)
  { host: Some(...), puerto: Some(8080), ... }
        │
        ▼  .build()
   ┌────────────┐
   │ Validación │──── Err(HostVacio) si host vacío
   │            │──── Err(PuertoReservado) si puerto < 1024
   └─────┬──────┘
         │ Ok
         ▼
   Servidor { host, puerto, ... }
```

---

## Builder con Typestate — garantías en tiempo de compilación

El Builder clásico valida en runtime. El **typestate** lo hace en **compile-time**:
si olvidas especificar un campo obligatorio, tu programa no compila.

```rust
use std::marker::PhantomData;

// Estados como tipos vacíos (zero-cost abstraction)
pub struct SinUrl;
pub struct ConUrl(String);
pub struct SinMetodo;
pub struct ConMetodo(String);
```

### Implementación completa

```rust
pub struct RequestBuilder<U, M> {
    url:     U,
    metodo:  M,
    headers: Vec<(String, String)>,
    cuerpo:  Option<String>,
    _marker: PhantomData<(U, M)>,
}

// Estado inicial: sin URL y sin método
impl RequestBuilder<SinUrl, SinMetodo> {
    pub fn new() -> Self {
        RequestBuilder {
            url:     SinUrl,
            metodo:  SinMetodo,
            headers: Vec::new(),
            cuerpo:  None,
            _marker: PhantomData,
        }
    }
}

impl Default for RequestBuilder<SinUrl, SinMetodo> {
    fn default() -> Self { Self::new() }
}

// Transición: agregar URL (cualquier estado de método)
impl<M> RequestBuilder<SinUrl, M> {
    pub fn url(self, url: impl Into<String>) -> RequestBuilder<ConUrl, M> {
        RequestBuilder {
            url:     ConUrl(url.into()),
            metodo:  self.metodo,
            headers: self.headers,
            cuerpo:  self.cuerpo,
            _marker: PhantomData,
        }
    }
}

// Transición: agregar método (cualquier estado de URL)
impl<U> RequestBuilder<U, SinMetodo> {
    pub fn metodo(self, m: impl Into<String>) -> RequestBuilder<U, ConMetodo> {
        RequestBuilder {
            url:     self.url,
            metodo:  ConMetodo(m.into()),
            headers: self.headers,
            cuerpo:  self.cuerpo,
            _marker: PhantomData,
        }
    }
}

// Headers y cuerpo: disponibles en cualquier estado (no cambian el typestate)
impl<U, M> RequestBuilder<U, M> {
    pub fn header(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.headers.push((k.into(), v.into()));
        self
    }

    pub fn cuerpo(mut self, c: impl Into<String>) -> Self {
        self.cuerpo = Some(c.into());
        self
    }
}

// build() SOLO disponible cuando URL y Método están presentes
impl RequestBuilder<ConUrl, ConMetodo> {
    pub fn build(self) -> Request {
        Request {
            url:     self.url.0,
            metodo:  self.metodo.0,
            headers: self.headers,
            cuerpo:  self.cuerpo,
        }
    }
}
```

### El typestate en acción

```rust
// ✅ Compila — tiene URL y método
let req = RequestBuilder::new()
    .url("https://api.example.com")
    .metodo("GET")
    .build();

// ❌ Error de compilación — falta URL
// RequestBuilder::new().metodo("GET").build();
//                                      ^^^^ método `build` no existe en
//                                      `RequestBuilder<SinUrl, ConMetodo>`
```

---

## Comparación: Builder Clásico vs. Typestate

| Característica | Builder Clásico | Builder Typestate |
|----------------|-----------------|-------------------|
| Detección de errores | Runtime (`build()`) | Compile-time |
| Complejidad de implementación | Baja | Media-Alta |
| Mensajes de error | Descriptivos en runtime | Mensajes de tipo crípticos |
| Campos opcionales | Natural con `Option<T>` | Natural (no cambian tipos) |
| Campos obligatorios | Validados en `build()` | Imposible olvidar |
| Número de tipos generados | 1 struct | N structs (uno por estado) |

**¿Cuándo usar cada uno?**
- **Clásico**: API pública de librería, muchos campos opcionales, errores descriptivos
- **Typestate**: Cuando olvidar un campo es un error grave, APIs de máquinas de estado

---

## Fluent Interface — por qué `self` (no `&mut self`)

Los métodos del builder retornan `Self` (ownership completo):

```rust
// ✅ Correcto — encadenamiento funciona
pub fn host(mut self, host: impl Into<String>) -> Self {
    self.host = Some(host.into());
    self  // retorna ownership
}

// ❌ No permite encadenamiento
pub fn host(&mut self, host: impl Into<String>) {
    self.host = Some(host.into());
    // retorna () — no se puede encadenar
}
```

El patrón `self → Self` es **zero-cost**: el compilador elimina las copias
intermedias mediante NRVO (Named Return Value Optimization).

---

## Ejercicio mental: ¿Qué compila?

```rust
let b = ServidorBuilder::new();
let b2 = b.host("localhost");
// ¿Puedo usar `b` aquí? NO — fue movido a `b2`

// La forma correcta es encadenar:
let srv = ServidorBuilder::new()
    .host("localhost")
    .puerto(8080)
    .build()?;
```

Esta restricción de ownership hace que el builder sea **memory-safe**: nunca
puedes tener dos builders parciales apuntando al mismo dato.
