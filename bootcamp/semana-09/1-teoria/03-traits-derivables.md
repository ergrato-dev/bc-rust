# 🎁 Traits Derivables

## ¿Qué es `#[derive()]`?

El atributo `#[derive()]` le pide al compilador que genere automáticamente la implementación de ciertos traits para tu tipo.

```rust
#[derive(Debug, Clone, PartialEq)]
struct Punto {
    x: i32,
    y: i32,
}

// El compilador genera automáticamente:
// - impl Debug for Punto
// - impl Clone for Punto  
// - impl PartialEq for Punto
```

## Traits Derivables Comunes

| Trait | Propósito | Uso |
|-------|-----------|-----|
| `Debug` | Formato de depuración | `{:?}`, `{:#?}` |
| `Clone` | Clonar valores | `.clone()` |
| `Copy` | Copiar implícitamente | Asignación |
| `PartialEq` | Comparar igualdad | `==`, `!=` |
| `Eq` | Igualdad total | Requisito para HashMap keys |
| `PartialOrd` | Ordenamiento parcial | `<`, `>`, `<=`, `>=` |
| `Ord` | Ordenamiento total | `.sort()` |
| `Hash` | Calcular hash | HashMap, HashSet |
| `Default` | Valor por defecto | `Default::default()` |

## Debug

Permite imprimir el tipo con formato de depuración.

```rust
#[derive(Debug)]
struct Usuario {
    nombre: String,
    edad: u32,
    activo: bool,
}

fn main() {
    let usuario = Usuario {
        nombre: String::from("Ana"),
        edad: 25,
        activo: true,
    };
    
    // Formato compacto
    println!("{:?}", usuario);
    // Usuario { nombre: "Ana", edad: 25, activo: true }
    
    // Formato bonito (pretty-print)
    println!("{:#?}", usuario);
    // Usuario {
    //     nombre: "Ana",
    //     edad: 25,
    //     activo: true,
    // }
}
```

## Clone

Permite crear copias profundas del valor.

```rust
#[derive(Debug, Clone)]
struct Configuracion {
    nombre: String,
    valores: Vec<i32>,
}

fn main() {
    let config1 = Configuracion {
        nombre: String::from("default"),
        valores: vec![1, 2, 3],
    };
    
    // Clonación explícita
    let config2 = config1.clone();
    
    // Ambos son independientes
    println!("{:?}", config1);
    println!("{:?}", config2);
}
```

## Copy

Permite copiar valores automáticamente (sin `.clone()`).

> ⚠️ **Importante**: Solo tipos que pueden copiarse bit a bit pueden derivar `Copy`. Requiere `Clone`.

```rust
#[derive(Debug, Clone, Copy)]
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Punto { x: 1, y: 2 };
    let p2 = p1; // Copia automática, NO move
    
    // Ambos son válidos
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
}

// ❌ No puede derivar Copy (String no es Copy)
// #[derive(Copy)]
// struct Usuario {
//     nombre: String, // Error!
// }
```

### ¿Cuándo Usar Copy vs Clone?

| Copy | Clone |
|------|-------|
| Tipos pequeños, simples | Cualquier tipo |
| Copia implícita | Copia explícita |
| Sin efectos secundarios | Puede ser costoso |
| `i32`, `f64`, `bool`, `char` | `String`, `Vec`, structs complejos |

## PartialEq y Eq

Permiten comparar valores por igualdad.

```rust
#[derive(Debug, PartialEq)]
struct Producto {
    id: u32,
    nombre: String,
    precio: f64,
}

fn main() {
    let p1 = Producto { id: 1, nombre: String::from("A"), precio: 10.0 };
    let p2 = Producto { id: 1, nombre: String::from("A"), precio: 10.0 };
    let p3 = Producto { id: 2, nombre: String::from("B"), precio: 20.0 };
    
    println!("p1 == p2: {}", p1 == p2); // true
    println!("p1 == p3: {}", p1 == p3); // false
    println!("p1 != p3: {}", p1 != p3); // true
}
```

### Diferencia: PartialEq vs Eq

```rust
// PartialEq: Igualdad que puede no ser reflexiva
// Ejemplo: f64 (NaN != NaN)
let nan = f64::NAN;
println!("{}", nan == nan); // false

// Eq: Igualdad total (a == a siempre es true)
// Requiere PartialEq
#[derive(PartialEq, Eq)]
struct Id(u32);
```

## PartialOrd y Ord

Permiten ordenar valores.

```rust
#[derive(Debug, PartialEq, PartialOrd)]
struct Puntaje {
    valor: i32,
}

fn main() {
    let a = Puntaje { valor: 100 };
    let b = Puntaje { valor: 200 };
    
    println!("a < b: {}", a < b);   // true
    println!("a > b: {}", a > b);   // false
    println!("a <= b: {}", a <= b); // true
}
```

### Para Ordenamiento Completo (sort)

```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Estudiante {
    nota: i32,
    nombre: String,
}

fn main() {
    let mut estudiantes = vec![
        Estudiante { nota: 85, nombre: String::from("Ana") },
        Estudiante { nota: 92, nombre: String::from("Bob") },
        Estudiante { nota: 78, nombre: String::from("Carlos") },
    ];
    
    estudiantes.sort(); // Ordena por nota, luego por nombre
    
    for e in &estudiantes {
        println!("{}: {}", e.nombre, e.nota);
    }
}
```

## Hash

Permite usar el tipo como clave en HashMap/HashSet.

```rust
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coordenada {
    x: i32,
    y: i32,
}

fn main() {
    let mut mapa: HashMap<Coordenada, String> = HashMap::new();
    
    mapa.insert(Coordenada { x: 0, y: 0 }, String::from("Origen"));
    mapa.insert(Coordenada { x: 1, y: 1 }, String::from("Punto A"));
    
    let coord = Coordenada { x: 0, y: 0 };
    println!("{:?}", mapa.get(&coord)); // Some("Origen")
}
```

## Default

Proporciona un valor por defecto.

```rust
#[derive(Debug, Default)]
struct Configuracion {
    debug: bool,
    max_conexiones: u32,
    nombre: String,
}

fn main() {
    // Crea con valores por defecto
    let config: Configuracion = Default::default();
    println!("{:?}", config);
    // Configuracion { debug: false, max_conexiones: 0, nombre: "" }
    
    // Sintaxis alternativa con actualización
    let config2 = Configuracion {
        debug: true,
        ..Default::default()
    };
    println!("{:?}", config2);
    // Configuracion { debug: true, max_conexiones: 0, nombre: "" }
}
```

## Derivar para Enums

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    Rojo,
    Verde,
    Azul,
}

#[derive(Debug, Clone, PartialEq)]
enum Resultado {
    Exito(String),
    Error(i32),
}

fn main() {
    let c1 = Color::Rojo;
    let c2 = c1; // Copy
    println!("{:?} == {:?}: {}", c1, c2, c1 == c2);
    
    let r1 = Resultado::Exito(String::from("ok"));
    let r2 = r1.clone(); // Clone (String no es Copy)
    println!("{:?}", r2);
}
```

## Combinaciones Comunes

```rust
// Tipo simple, copiable
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Id(u32);

// Tipo con String, no copiable
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Nombre(String);

// Tipo para ordenar
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Puntaje(i32);

// Tipo completo para colecciones
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Usuario {
    id: u32,
    nombre: String,
}
```

## Resumen

| Trait | Requisito | Genera |
|-------|-----------|--------|
| `Debug` | Todos los campos Debug | `{:?}` formatting |
| `Clone` | Todos los campos Clone | `.clone()` |
| `Copy` | Clone + campos Copy | Copia implícita |
| `PartialEq` | Todos los campos PartialEq | `==`, `!=` |
| `Eq` | PartialEq | Marca igualdad total |
| `Hash` | Todos los campos Hash | Hashing |
| `Default` | Todos los campos Default | Valores por defecto |

---

## 🔗 Navegación

| ⬅️ Anterior | 🏠 Índice | ➡️ Siguiente |
|:------------|:--------:|-------------:|
| [Implementación](02-implementacion-traits.md) | [Semana 09](../README.md) | [Traits Estándar](04-traits-estandar.md) |
