# 📖 Glosario - Semana 02

## Variables y Tipos de Datos

---

### B

**`bool`**
: Tipo booleano que representa `true` o `false`. Ocupa 1 byte en memoria.
```rust
let activo: bool = true;
```

**Binding (enlace)**
: La asociación entre un nombre de variable y un valor en memoria. En Rust, `let x = 5` crea un binding.

**Byte**
: Unidad de 8 bits. El tipo `u8` representa exactamente un byte (0-255).

---

### C

**`char`**
: Tipo que representa un carácter Unicode. Ocupa 4 bytes y usa comillas simples.
```rust
let letra: char = 'A';
let emoji: char = '🦀';
```

**`const`**
: Palabra clave para declarar constantes. El valor debe conocerse en tiempo de compilación.
```rust
const PI: f64 = 3.14159;
```

**Casting**
: Conversión explícita entre tipos usando `as`.
```rust
let x: i32 = 10;
let y: f64 = x as f64;
```

---

### E

**`edition`**
: Versión del lenguaje Rust (2015, 2018, 2021, 2024). Define características disponibles.

**Expresión**
: Código que produce un valor. En Rust, casi todo es una expresión.
```rust
let x = 5 + 3;  // 5 + 3 es una expresión
```

---

### F

**`f32`**
: Tipo flotante de 32 bits (precisión simple). Menos preciso que f64.
```rust
let x: f32 = 3.14;
```

**`f64`**
: Tipo flotante de 64 bits (precisión doble). Es el tipo flotante por defecto.
```rust
let x = 3.14;  // f64 por defecto
```

---

### I

**`i8`, `i16`, `i32`, `i64`, `i128`**
: Tipos enteros con signo de 8, 16, 32, 64 y 128 bits respectivamente.
```rust
let x: i32 = -42;  // i32 es el default
```

**Inmutabilidad**
: Propiedad de una variable que no puede cambiar su valor después de la asignación inicial. Es el comportamiento por defecto en Rust.
```rust
let x = 5;  // inmutable
```

**Inferencia de tipos**
: Capacidad del compilador para deducir el tipo de una variable sin anotación explícita.
```rust
let x = 5;  // El compilador infiere i32
```

**`isize`**
: Entero con signo cuyo tamaño depende de la arquitectura (32 o 64 bits).

---

### L

**`let`**
: Palabra clave para declarar variables.
```rust
let nombre = "Rust";
```

**Literal**
: Valor escrito directamente en el código fuente.
```rust
42        // literal entero
3.14      // literal flotante
"hola"    // literal string
'A'       // literal char
```

---

### M

**`mut`**
: Palabra clave que permite que una variable sea mutable (modificable).
```rust
let mut contador = 0;
contador += 1;  // ¡Ahora es válido!
```

**Mutabilidad**
: Capacidad de una variable para cambiar su valor. Requiere `mut` en Rust.

---

### O

**Overflow**
: Cuando un valor excede el rango de su tipo. En debug, Rust hace panic; en release, hace wrap-around.

---

### P

**Primitivo**
: Tipo de dato básico incorporado en el lenguaje: enteros, flotantes, bool, char.

---

### R

**Rango**
: Los valores mínimos y máximos que puede contener un tipo.
```rust
i8::MIN  // -128
i8::MAX  // 127
```

---

### S

**Shadowing**
: Re-declarar una variable con el mismo nombre, creando una nueva variable que "oculta" la anterior.
```rust
let x = 5;
let x = x + 1;  // Nueva variable, shadowing
```

**`static`**
: Palabra clave para variables con lifetime `'static` (toda la ejecución del programa).
```rust
static MENSAJE: &str = "Hola";
```

**String literal (`&str`)**
: Secuencia de caracteres inmutable almacenada en el binario.
```rust
let saludo: &str = "Hola";
```

**Sufijo de tipo**
: Indicador de tipo añadido a un literal.
```rust
let x = 42i64;   // i64
let y = 3.14f32; // f32
```

---

### T

**Tipo**
: Clasificación que determina qué valores puede tener una variable y qué operaciones se pueden realizar.

**Type annotation (anotación de tipo)**
: Especificación explícita del tipo de una variable.
```rust
let x: i32 = 5;
```

---

### U

**`u8`, `u16`, `u32`, `u64`, `u128`**
: Tipos enteros sin signo de 8, 16, 32, 64 y 128 bits respectivamente.
```rust
let byte: u8 = 255;
```

**`usize`**
: Entero sin signo cuyo tamaño depende de la arquitectura. Usado para índices.
```rust
let indice: usize = 0;
```

---

### V

**Variable**
: Nombre asociado a un valor almacenado en memoria.

---

## Símbolos y Operadores

| Símbolo | Nombre | Uso |
|---------|--------|-----|
| `=` | Asignación | `let x = 5` |
| `+` | Suma | `a + b` |
| `-` | Resta | `a - b` |
| `*` | Multiplicación | `a * b` |
| `/` | División | `a / b` |
| `%` | Módulo | `a % b` |
| `==` | Igualdad | `a == b` |
| `!=` | Desigualdad | `a != b` |
| `<` | Menor que | `a < b` |
| `>` | Mayor que | `a > b` |
| `<=` | Menor o igual | `a <= b` |
| `>=` | Mayor o igual | `a >= b` |
| `&&` | AND lógico | `a && b` |
| `\|\|` | OR lógico | `a \|\| b` |
| `!` | NOT lógico | `!a` |
| `:` | Anotación de tipo | `let x: i32` |
| `::` | Path separator | `i32::MAX` |

---

*Bootcamp Rust: Zero to Hero - Semana 02*
