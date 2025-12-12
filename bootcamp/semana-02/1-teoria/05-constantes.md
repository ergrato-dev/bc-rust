# 📖 Constantes

## 🎯 Objetivo de Aprendizaje

Entender las diferencias entre `const`, `static` y variables `let`, y cuándo usar cada uno.

---

## 📚 Contenido

### Constantes con `const`

Las **constantes** son valores que nunca cambian y se evalúan en tiempo de compilación:

```rust
const PI: f64 = 3.14159265359;
const MAX_PUNTOS: u32 = 100_000;
const NOMBRE_APP: &str = "Rust Bootcamp";

fn main() {
    println!("Pi: {}", PI);
    println!("Máximo: {}", MAX_PUNTOS);
    println!("App: {}", NOMBRE_APP);
}
```

![Diagrama Constantes](../0-assets/05-constantes.svg)

### Reglas de `const`

| Regla | Descripción |
|-------|-------------|
| Tipo obligatorio | Siempre debes declarar el tipo |
| SCREAMING_SNAKE_CASE | Convención de nombres |
| Valor constante | Debe conocerse en compilación |
| Inmutable siempre | No se puede usar `mut` |
| Scope global o local | Puede declararse en cualquier scope |

```rust
// ✅ Correcto
const MAX_SIZE: usize = 1024;

// ❌ Incorrecto - sin tipo
// const MAX_SIZE = 1024;

// ❌ Incorrecto - no es valor constante
// const RANDOM: i32 = rand::random();
```

---

## 🔒 Variables Estáticas con `static`

Las variables `static` tienen una ubicación fija en memoria durante toda la ejecución:

```rust
static MENSAJE: &str = "Hola desde static";
static CONTADOR: i32 = 0;

fn main() {
    println!("{}", MENSAJE);
}
```

### static mut (¡Peligroso!)

```rust
static mut CONTADOR_GLOBAL: i32 = 0;

fn main() {
    unsafe {
        CONTADOR_GLOBAL += 1;
        println!("Contador: {}", CONTADOR_GLOBAL);
    }
}
```

> ⚠️ `static mut` requiere `unsafe` porque puede causar data races. Evítalo cuando sea posible.

---

## 📊 const vs static vs let

| Característica | `const` | `static` | `let` |
|----------------|---------|----------|-------|
| Momento de evaluación | Compilación | Compilación | Ejecución |
| Ubicación en memoria | Inline (copiado) | Fija | Stack |
| Mutabilidad | ❌ Nunca | ⚠️ Con unsafe | ✅ Con mut |
| Tipo requerido | ✅ Obligatorio | ✅ Obligatorio | ❌ Inferible |
| Scope global | ✅ Sí | ✅ Sí | ❌ No |
| Convención nombre | SCREAMING_CASE | SCREAMING_CASE | snake_case |

### Ejemplo Comparativo

```rust
// Constante: se "copia" donde se usa
const MAX: i32 = 100;

// Static: una sola ubicación en memoria
static MIN: i32 = 0;

fn main() {
    // Variable: vive en el stack de esta función
    let valor: i32 = 50;
    
    println!("MAX: {}, MIN: {}, valor: {}", MAX, MIN, valor);
}
```

---

## 🎯 ¿Cuándo Usar Cada Uno?

### Usa `const` para:

1. **Valores matemáticos**
```rust
const PI: f64 = 3.14159265359;
const E: f64 = 2.718281828;
const GOLDEN_RATIO: f64 = 1.618033988749;
```

2. **Configuraciones fijas**
```rust
const MAX_CONEXIONES: u32 = 100;
const TIMEOUT_MS: u64 = 5000;
const VERSION: &str = "1.0.0";
```

3. **Valores usados en múltiples lugares**
```rust
const TAMANIO_BUFFER: usize = 1024;

fn crear_buffer() -> [u8; TAMANIO_BUFFER] {
    [0; TAMANIO_BUFFER]
}
```

### Usa `static` para:

1. **Datos con referencia global**
```rust
static DATOS: &[u8] = include_bytes!("data.bin");
```

2. **Interoperabilidad con C**
```rust
#[no_mangle]
static EXPORTED_VALUE: i32 = 42;
```

### Usa `let` para:

1. **Valores calculados en runtime**
```rust
let ahora = std::time::Instant::now();
let entrada_usuario = leer_input();
```

2. **Valores que podrían cambiar**
```rust
let mut contador = 0;
```

---

## 🔢 Constantes en Expresiones

Las constantes pueden usarse en contextos que requieren valores constantes:

```rust
const TAMANIO: usize = 5;

fn main() {
    // En tamaño de array (requiere constante)
    let array: [i32; TAMANIO] = [1, 2, 3, 4, 5];
    
    // En patrones de match
    const LIMITE: i32 = 10;
    let x = 5;
    
    match x {
        0..=LIMITE => println!("Dentro del límite"),
        _ => println!("Fuera del límite"),
    }
    
    println!("Array: {:?}", array);
}
```

---

## 💡 Buenas Prácticas

### 1. Agrupa Constantes Relacionadas

```rust
mod config {
    pub const DB_HOST: &str = "localhost";
    pub const DB_PORT: u16 = 5432;
    pub const DB_NAME: &str = "bootcamp";
}

fn main() {
    println!("Conectando a {}:{}", config::DB_HOST, config::DB_PORT);
}
```

### 2. Usa Nombres Descriptivos

```rust
// ❌ Poco claro
const X: u32 = 60;
const Y: u32 = 3600;

// ✅ Descriptivo
const SEGUNDOS_POR_MINUTO: u32 = 60;
const SEGUNDOS_POR_HORA: u32 = 3600;
```

### 3. Documenta las Constantes

```rust
/// Número máximo de intentos de reconexión
const MAX_REINTENTOS: u8 = 3;

/// Tiempo de espera entre reintentos en milisegundos
const ESPERA_REINTENTO_MS: u64 = 1000;
```

---

## 🧪 Ejercicio

Crea constantes para una aplicación de quiz:

1. Número máximo de preguntas
2. Puntos por respuesta correcta
3. Tiempo límite en segundos
4. Nombre de la aplicación

<details>
<summary>Ver solución</summary>

```rust
/// Número máximo de preguntas por quiz
const MAX_PREGUNTAS: u8 = 10;

/// Puntos otorgados por cada respuesta correcta
const PUNTOS_CORRECTA: u32 = 100;

/// Tiempo límite para responder (en segundos)
const TIEMPO_LIMITE_SEG: u32 = 30;

/// Nombre de la aplicación
const NOMBRE_APP: &str = "Rust Quiz";

fn main() {
    println!("=== {} ===", NOMBRE_APP);
    println!("Preguntas: {}", MAX_PREGUNTAS);
    println!("Puntos por correcta: {}", PUNTOS_CORRECTA);
    println!("Tiempo por pregunta: {} segundos", TIEMPO_LIMITE_SEG);
    
    let puntuacion_maxima = MAX_PREGUNTAS as u32 * PUNTOS_CORRECTA;
    println!("Puntuación máxima posible: {}", puntuacion_maxima);
}
```

</details>

---

## 📌 Resumen

| Concepto | Sintaxis | Uso Principal |
|----------|----------|---------------|
| `const` | `const NOMBRE: Tipo = valor;` | Valores fijos, configuración |
| `static` | `static NOMBRE: Tipo = valor;` | Datos con ubicación fija |
| `let` | `let nombre = valor;` | Variables normales |

---

## 🔗 Navegación

[← Shadowing](./04-shadowing.md) | [Prácticas →](../2-practica/)
