# 📦 Tipos de Structs

![Tipos de Structs](../0-assets/04-tipos-structs.svg)

## Tres Tipos de Structs en Rust

| Tipo | Sintaxis | Uso |
|------|----------|-----|
| Named Struct | `struct S { campo: Tipo }` | Datos con nombres claros |
| Tuple Struct | `struct S(Tipo1, Tipo2)` | Pocos campos, semántica por posición |
| Unit Struct | `struct S;` | Marcadores, sin datos |

---

## 1. Named Structs (Structs con Campos Nombrados)

El tipo más común, campos accesibles por nombre:

```rust
struct Usuario {
    nombre: String,
    email: String,
    edad: u32,
}

fn main() {
    let user = Usuario {
        nombre: String::from("Ana"),
        email: String::from("ana@email.com"),
        edad: 28,
    };

    println!("Nombre: {}", user.nombre);
}
```

### Cuándo usar
- Structs con muchos campos
- Cuando los nombres de campos son importantes
- APIs públicas donde la claridad es clave

---

## 2. Tuple Structs

Structs sin nombres de campos, acceso por índice:

```rust
struct Color(u8, u8, u8);        // RGB
struct Punto(f64, f64);          // coordenadas x, y
struct Punto3D(f64, f64, f64);   // coordenadas x, y, z

fn main() {
    let rojo = Color(255, 0, 0);
    let origen = Punto(0.0, 0.0);

    // Acceso por índice
    println!("R: {}, G: {}, B: {}", rojo.0, rojo.1, rojo.2);
    println!("Punto: ({}, {})", origen.0, origen.1);
}
```

### Métodos en Tuple Structs

```rust
struct Punto(f64, f64);

impl Punto {
    fn new(x: f64, y: f64) -> Self {
        Self(x, y)
    }

    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn distancia_al_origen(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
}
```

### Cuándo usar
- Pocos campos (2-3)
- El significado es obvio por contexto
- Crear tipos nuevos sobre primitivos (newtype pattern)

---

## 3. Newtype Pattern

Crear un tipo nuevo que envuelve otro tipo:

```rust
// Tipos distintos aunque internamente son u64
struct UserId(u64);
struct ProductId(u64);

fn obtener_usuario(id: UserId) {
    // Solo acepta UserId, no ProductId
}

fn main() {
    let user_id = UserId(12345);
    let product_id = ProductId(67890);

    obtener_usuario(user_id);      // ✅ OK
    // obtener_usuario(product_id); // ❌ Error de tipos
}
```

### Beneficios del Newtype
- Seguridad de tipos en tiempo de compilación
- No se pueden mezclar IDs de diferentes entidades
- Cero costo en runtime

---

## 4. Unit Structs

Structs sin campos, ocupan cero bytes:

```rust
struct Marcador;
struct Metros;
struct Segundos;

fn main() {
    let _m = Marcador;
}
```

### Usos Comunes

```rust
// Como marcadores de tipo
struct Validado;
struct NoValidado;

struct Formulario<Estado> {
    datos: String,
    _estado: std::marker::PhantomData<Estado>,
}

// Para implementar traits sin datos
struct MiLogger;

// impl algún trait for MiLogger { ... }
```

### Cuándo usar
- Implementar traits que no necesitan datos
- Marcadores de estado (type state pattern)
- Distinguir tipos sin añadir overhead

---

## Comparación Visual

```rust
// Named Struct - campos con nombre
struct Persona {
    nombre: String,
    edad: u32,
}
let p = Persona { nombre: String::from("Ana"), edad: 25 };
println!("{}", p.nombre);  // Acceso por nombre

// Tuple Struct - campos por posición
struct Punto(f64, f64);
let punto = Punto(3.0, 4.0);
println!("{}", punto.0);   // Acceso por índice

// Unit Struct - sin campos
struct Vacio;
let v = Vacio;
```

---

## Structs Anidados

```rust
struct Direccion {
    calle: String,
    ciudad: String,
    codigo_postal: String,
}

struct Persona {
    nombre: String,
    edad: u32,
    direccion: Direccion,  // Struct anidado
}

fn main() {
    let persona = Persona {
        nombre: String::from("Carlos"),
        edad: 30,
        direccion: Direccion {
            calle: String::from("Calle Principal 123"),
            ciudad: String::from("Madrid"),
            codigo_postal: String::from("28001"),
        },
    };

    // Acceso a campos anidados
    println!("{} vive en {}", persona.nombre, persona.direccion.ciudad);
}
```

---

## Desestructuración

```rust
struct Punto(f64, f64);

struct Persona {
    nombre: String,
    edad: u32,
}

fn main() {
    // Desestructurar tuple struct
    let punto = Punto(3.0, 4.0);
    let Punto(x, y) = punto;
    println!("x: {}, y: {}", x, y);

    // Desestructurar named struct
    let persona = Persona {
        nombre: String::from("Ana"),
        edad: 25,
    };
    let Persona { nombre, edad } = persona;
    println!("{} tiene {} años", nombre, edad);
}
```

---

## 📝 Resumen

| Tipo | Sintaxis | Acceso | Uso Principal |
|------|----------|--------|---------------|
| Named | `struct S { a: T }` | `s.a` | Datos complejos |
| Tuple | `struct S(T)` | `s.0` | Newtype, pocos campos |
| Unit | `struct S;` | - | Marcadores, traits |

---

*Bootcamp Rust: Zero to Hero - Semana 03*
