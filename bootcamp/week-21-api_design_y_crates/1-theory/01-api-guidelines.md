# 📖 01 — Rust API Guidelines y Ergonomía

## ¿Por qué importa el diseño de API?

Una API bien diseñada es aquella que:
- Es **difícil de usar mal** (los errores no compilan)
- Es **fácil de usar correctamente** (el caso común requiere poco código)
- Es **predecible** (los nombres sugieren el comportamiento)
- Es **flexible** sin sacrificar seguridad

Rust tiene un documento oficial: las **Rust API Guidelines** (`rust-lang.github.io/api-guidelines/`).
Esta semana las aplicamos en la práctica.

---

## Naming Conventions — coherencia ante todo

| Elemento | Convención | Ejemplos |
|----------|------------|---------|
| Funciones y métodos | `snake_case` | `to_string`, `as_bytes`, `into_inner` |
| Tipos y traits | `PascalCase` | `HashMap`, `IntoIterator`, `Display` |
| Constantes y statics | `SCREAMING_SNAKE_CASE` | `MAX_SIZE`, `DEFAULT_TIMEOUT` |
| Módulos | `snake_case` | `std::collections`, `std::io` |
| Variables de tipo genérico | `PascalCase`, cortas | `T`, `K`, `V`, `E`, `Err` |

### Prefijos de conversión estándar

Rust tiene convenciones para métodos de conversión:

```rust
// as_*  — conversión barata, sin coste, retorna referencia
impl String {
    pub fn as_str(&self) -> &str { self.as_ref() }
    pub fn as_bytes(&self) -> &[u8] { ... }
}

// to_*  — conversión costosa (puede allocar), toma referencia
impl str {
    pub fn to_string(&self) -> String { ... }
    pub fn to_uppercase(&self) -> String { ... }
}

// into_* — conversión consumidora, transfiere ownership
impl String {
    pub fn into_bytes(self) -> Vec<u8> { ... }
}

// from / into — conversiones de traits estándar
let s = String::from("hola");
let s: String = "hola".into();
```

---

## El patrón `impl Into<T>` — flexibilidad en parámetros

Una API que acepta `String` obliga al caller a pasar exactamente un `String`:

```rust
// ❌ Poco ergonómico — fuerza a hacer .to_string()
fn saludar(nombre: String) { println!("Hola, {nombre}"); }

saludar("Ana".to_string());  // caller debe convertir
saludar(nombre.clone());     // si nombre es &String, debe clonar
```

La solución idiomática: `impl Into<String>`.

```rust
// ✅ Ergonómico — acepta &str, String, &String, Cow<str>...
fn saludar(nombre: impl Into<String>) {
    let nombre = nombre.into();
    println!("Hola, {nombre}");
}

saludar("Ana");            // &str — sin conversión explícita
saludar(String::from("Ana")); // String — directo
saludar(&nombre_variable);    // &String — funciona también
```

Variantes similares:
- `impl AsRef<str>` — para operaciones de solo lectura (sin necesitar `String`)
- `impl AsRef<Path>` — para funciones que aceptan rutas
- `impl Iterator<Item = T>` — para parámetros de iteradores

---

## Newtype Pattern — semántica sobre primitivos

Un **newtype** es un struct de un solo campo que envuelve otro tipo para darle
identidad semántica. Resuelve la "primitive obsession":

```rust
// ❌ Sin newtype — fácil confundir argumentos
fn registrar_usuario(nombre: String, email: String, edad: u32) { ... }
registrar_usuario(email, nombre, 25);  // ¡compiló pero es incorrecto!

// ✅ Con newtypes — el compilador te protege
pub struct Nombre(String);
pub struct Email(String);
pub struct Edad(u32);

fn registrar_usuario(nombre: Nombre, email: Email, edad: Edad) { ... }
registrar_usuario(email, nombre, Edad(25));  // error de compilación
```

### Implementar newtypes ergonómicamente

```rust
pub struct Metros(f64);

impl Metros {
    pub fn new(valor: f64) -> Self {
        Metros(valor)
    }

    pub fn valor(&self) -> f64 {
        self.0
    }
}

// Conversiones estándar
impl From<f64> for Metros {
    fn from(v: f64) -> Self { Metros(v) }
}

impl std::fmt::Display for Metros {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m", self.0)
    }
}

// Uso
let distancia: Metros = 42.5.into();
println!("{distancia}");  // "42.5m"
```

---

## Consistencia en constructores

Rust no tiene constructores en el sentido de C++ o Java. La convención es:

| Método | Uso | Ejemplo |
|--------|-----|---------|
| `Type::new(...)` | Constructor principal | `Vec::new()`, `String::new()` |
| `Type::default()` | Valor por defecto (trait `Default`) | `HashMap::default()` |
| `Type::from(x)` / `x.into()` | Conversión desde otro tipo | `String::from("hola")` |
| `Type::with_capacity(n)` | Constructor con configuración | `Vec::with_capacity(100)` |
| `Type::builder()` | Inicio de un Builder | `Request::builder()` |

```rust
// ✅ Seguir las convenciones hace la API predecible
#[derive(Default)]
pub struct Config {
    pub timeout_ms: u64,
    pub retries: u32,
    pub base_url: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            timeout_ms: 5000,
            retries: 3,
            base_url: "https://api.example.com".to_string(),
        }
    }
}
```

---

## Getters — regla de naming

A diferencia de Java, Rust no usa el prefijo `get_`:

```rust
pub struct Punto {
    x: f64,
    y: f64,
}

impl Punto {
    // ✅ Sin prefijo get_
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }

    // ✅ Para Option<T>, se acepta is_* o has_*
    pub fn is_valid(&self) -> bool { ... }

    // ❌ No hacer esto
    pub fn get_x(&self) -> f64 { self.x }
}
```

---

## Tipos de retorno — ser explícito con los errores

```rust
// ❌ Oculta qué puede fallar
pub fn leer_config(path: &str) -> Option<Config> { ... }

// ✅ Documenta el tipo de error
pub fn leer_config(path: &str) -> Result<Config, ConfigError> { ... }
```

Las APIs públicas de una librería **nunca** deben usar `unwrap()` o `expect()`.
Todo error propagable debe retornarse como `Result<T, E>`.

---

## Trait `Default` — cuándo implementarlo

Implementa `Default` cuando el tipo tiene un valor "vacío" o "inicial" razonable:

```rust
#[derive(Default)]
pub struct ContadorEstadisticas {
    pub total: u64,
    pub errores: u64,
    pub latencia_ms: f64,
}

// El derive genera:
// impl Default for ContadorEstadisticas {
//     fn default() -> Self {
//         ContadorEstadisticas { total: 0, errores: 0, latencia_ms: 0.0 }
//     }
// }
```

Beneficio: los callers pueden usar `..Default::default()` para
crear instancias con algunos campos personalizados:

```rust
let stats = ContadorEstadisticas {
    total: 100,
    ..Default::default()  // errores = 0, latencia_ms = 0.0
};
```

---

## Comparación con otros lenguajes

| Aspecto | Rust | Java | Python |
|---------|------|------|--------|
| Constructores | `Type::new()` convención | `new Type()` obligatorio | `__init__` |
| Getters | Sin prefijo `get_` | `getX()` convención | Property `@property` |
| Errores | `Result<T, E>` checked | Excepciones unchecked | Excepciones |
| Conversiones | `From`/`Into` traits | `.toString()`, casting | Duck typing |
| Null | Imposible — usar `Option<T>` | `null` peligroso | `None` sin tipos |

---

## Errores comunes en diseño de API

| Error | Ejemplo incorrecto | Solución |
|-------|-------------------|----------|
| Parámetros primitivos sin semántica | `fn move(x: f64, y: f64)` | Newtype `Coord`, `Metros` |
| API que retorna `bool` en lugar de `Result` | `fn guardar() -> bool` | `fn guardar() -> Result<(), IoError>` |
| Usar `String` cuando `&str` es suficiente | `fn mostrar(s: String)` | `fn mostrar(s: &str)` |
| Clonar innecesariamente | `fn nombre(&self) -> String` | `fn nombre(&self) -> &str` |
| Método que muta y retorna bool de éxito | `fn insertar(&mut self, v: T) -> bool` | `fn insertar(&mut self, v: T) -> Result<(), E>` |
