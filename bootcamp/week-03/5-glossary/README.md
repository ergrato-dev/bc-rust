# 📖 Glosario - Semana 03

## Structs y Métodos

---

### B

**Bloque impl**
: Bloque donde se definen métodos y funciones asociadas para un tipo.
```rust
impl MiStruct {
    fn metodo(&self) { }
}
```

---

### C

**Campo (field)**
: Cada uno de los valores almacenados en un struct.
```rust
struct Usuario {
    nombre: String,  // campo
    edad: u32,       // campo
}
```

**Constructor**
: Función asociada que crea una nueva instancia de un struct. Por convención se llama `new()`.
```rust
fn new(nombre: String) -> Self {
    Self { nombre, ... }
}
```

---

### D

**Desestructuración**
: Extraer valores de un struct en variables separadas.
```rust
let Punto(x, y) = punto;
let Usuario { nombre, edad } = usuario;
```

---

### F

**Field init shorthand**
: Sintaxis abreviada cuando el nombre de variable coincide con el campo.
```rust
let nombre = String::from("Ana");
let u = Usuario { nombre };  // equivale a nombre: nombre
```

**Función asociada**
: Función dentro de `impl` que NO tiene `self`. Se llama con `Tipo::funcion()`.
```rust
Usuario::new(...)
String::from(...)
```

---

### I

**Instancia**
: Un valor concreto creado a partir de un struct.
```rust
let usuario = Usuario { ... };  // instancia
```

**impl**
: Palabra clave para implementar métodos y funciones asociadas.

---

### M

**Método**
: Función dentro de `impl` que tiene `self` como primer parámetro.
```rust
fn area(&self) -> u32 { ... }
```

**mut self**
: Método que toma ownership mutable (raro).
```rust
fn consumir_y_modificar(mut self) -> Self { ... }
```

---

### N

**Named struct**
: Struct con campos nombrados (el tipo más común).
```rust
struct Usuario {
    nombre: String,
    edad: u32,
}
```

**new()**
: Nombre convencional para el constructor principal.

**Newtype pattern**
: Envolver un tipo en un tuple struct para crear un tipo distinto.
```rust
struct UserId(u64);
struct ProductId(u64);
```

---

### S

**Self**
: Alias del tipo dentro de un bloque `impl`.
```rust
impl Usuario {
    fn new() -> Self { ... }  // Self = Usuario
}
```

**self**
: Referencia a la instancia actual en un método.

**&self**
: Referencia inmutable a la instancia. Para métodos de lectura.
```rust
fn nombre(&self) -> &str { &self.nombre }
```

**&mut self**
: Referencia mutable a la instancia. Para métodos de modificación.
```rust
fn set_edad(&mut self, edad: u32) { self.edad = edad; }
```

**Struct**
: Tipo de dato personalizado que agrupa valores relacionados.

**Struct update syntax**
: Crear un struct basado en otro existente.
```rust
let u2 = Usuario { email: nuevo, ..u1 };
```

---

### T

**Tuple struct**
: Struct con campos sin nombre, accesibles por índice.
```rust
struct Color(u8, u8, u8);
let c = Color(255, 0, 0);
println!("{}", c.0);  // 255
```

---

### U

**Unit struct**
: Struct sin campos.
```rust
struct Marcador;
```

---

## Símbolos

| Símbolo | Uso | Ejemplo |
|---------|-----|---------|
| `::` | Llamar función asociada | `Usuario::new()` |
| `.` | Llamar método o acceder campo | `usuario.nombre` |
| `{}` | Cuerpo del struct | `struct S { }` |
| `()` | Tuple struct | `struct S(T)` |
| `;` | Unit struct | `struct S;` |

---

*Bootcamp Rust: Zero to Hero - Semana 03*
