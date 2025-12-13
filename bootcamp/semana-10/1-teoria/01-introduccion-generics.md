# 📚 Introducción a Generics

## ¿Qué son los Generics?

Los **generics** (tipos genéricos) son una característica que permite escribir código que funciona con múltiples tipos sin duplicación. En lugar de escribir funciones o estructuras específicas para cada tipo, defines una "plantilla" que el compilador especializa para cada tipo concreto que uses.

```rust
// Sin generics: código duplicado
fn mayor_i32(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn mayor_f64(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

fn mayor_char(a: char, b: char) -> char {
    if a > b { a } else { b }
}

// Con generics: una sola función
fn mayor<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

## ¿Por qué usar Generics?

### 1. Eliminan la Duplicación de Código

```rust
// ❌ Sin generics: N funciones para N tipos
fn duplicar_i32(x: i32) -> i32 { x * 2 }
fn duplicar_i64(x: i64) -> i64 { x * 2 }
fn duplicar_f32(x: f32) -> f32 { x * 2.0 }
fn duplicar_f64(x: f64) -> f64 { x * 2.0 }

// ✅ Con generics: una función para todos
use std::ops::Mul;

fn duplicar<T>(x: T) -> T 
where 
    T: Mul<Output = T> + From<u8>
{
    x * T::from(2)
}
```

### 2. Garantizan Type Safety en Tiempo de Compilación

```rust
struct Contenedor<T> {
    valor: T,
}

let enteros = Contenedor { valor: 42 };
let textos = Contenedor { valor: "hola" };

// El compilador sabe exactamente qué tipo contiene cada uno
let n: i32 = enteros.valor;      // ✅ OK
let s: &str = textos.valor;      // ✅ OK
// let x: i32 = textos.valor;    // ❌ Error de compilación
```

### 3. Zero-Cost Abstraction

Los generics en Rust son **zero-cost** gracias a la **monomorphization**:

```rust
fn identidad<T>(x: T) -> T { x }

// Cuando usas:
let a = identidad(5);       // i32
let b = identidad("hola");  // &str
let c = identidad(3.14);    // f64

// El compilador genera:
fn identidad_i32(x: i32) -> i32 { x }
fn identidad_str(x: &str) -> &str { x }
fn identidad_f64(x: f64) -> f64 { x }
```

No hay overhead en runtime: el código genérico se convierte en código específico optimizado.

## Anatomía de un Tipo Genérico

```
fn mayor<T: PartialOrd>(a: T, b: T) -> T
   │    │  │           │      │      │
   │    │  │           │      │      └── Tipo de retorno (genérico)
   │    │  │           │      └── Segundo parámetro de tipo T
   │    │  │           └── Primer parámetro de tipo T
   │    │  └── Trait bound: T debe implementar PartialOrd
   │    └── Parámetro de tipo (convención: letra mayúscula)
   └── Nombre de la función
```

### Convenciones de Nombres

| Letra | Uso Común |
|-------|-----------|
| `T` | Type (tipo genérico principal) |
| `U`, `V` | Tipos adicionales |
| `E` | Error type |
| `K` | Key (clave) |
| `V` | Value (valor) |
| `R` | Return (retorno) |
| `S` | State (estado) |

```rust
// Ejemplos de la biblioteca estándar
enum Result<T, E> { Ok(T), Err(E) }
enum Option<T> { Some(T), None }
struct HashMap<K, V> { ... }
```

## Generics vs Otros Lenguajes

### Rust vs C++ Templates

| Aspecto | Rust Generics | C++ Templates |
|---------|---------------|---------------|
| Verificación | En definición | En instanciación |
| Errores | Claros, en el trait bound | Largos, en el uso |
| Concepto | Trait bounds | Concepts (C++20) |

```rust
// Rust: error en la DEFINICIÓN si falta bound
fn sumar<T>(a: T, b: T) -> T {
    a + b  // ❌ Error: T no implementa Add
}

// Rust: correcto con bound
fn sumar<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b  // ✅ OK
}
```

### Rust vs Java Generics

| Aspecto | Rust Generics | Java Generics |
|---------|---------------|---------------|
| Implementación | Monomorphization | Type erasure |
| Runtime | Tipos concretos | Object |
| Primitivos | Soportados | Solo wrappers |

```rust
// Rust: tipos primitivos directamente
let vec: Vec<i32> = vec![1, 2, 3];

// Java: necesita wrapper
// List<Integer> list = new ArrayList<>();
```

## Monomorphization

La **monomorphization** es el proceso donde el compilador genera código específico para cada combinación de tipos usada:

```rust
fn mostrar<T: std::fmt::Display>(valor: T) {
    println!("{}", valor);
}

fn main() {
    mostrar(42);        // Genera: mostrar_i32
    mostrar("hola");    // Genera: mostrar_str
    mostrar(3.14f64);   // Genera: mostrar_f64
}
```

### Ventajas

- **Performance**: Código optimizado para cada tipo
- **Inlining**: El compilador puede hacer inline del código
- **Sin overhead**: No hay indirección en runtime

### Desventajas

- **Tamaño del binario**: Más código generado
- **Tiempo de compilación**: Más trabajo para el compilador

## Cuándo Usar Generics

### ✅ Usar Generics Cuando:

1. **La lógica es idéntica** para múltiples tipos
2. **Necesitas type safety** en tiempo de compilación
3. **El rendimiento es crítico** (zero-cost)
4. **Quieres código reutilizable**

### ❌ No Usar Generics Cuando:

1. **Solo necesitas un tipo específico**
2. **El código varía significativamente** por tipo
3. **Prefieres dispatch dinámico** (trait objects)
4. **El tamaño del binario es crítico**

## Resumen

| Concepto | Descripción |
|----------|-------------|
| **Generic** | Código que funciona con múltiples tipos |
| **Parámetro de tipo** | `T`, `U`, etc. - placeholder para tipos concretos |
| **Trait bound** | Restricción sobre qué tipos son válidos |
| **Monomorphization** | Generación de código específico por tipo |
| **Zero-cost** | Sin overhead en runtime |

```rust
// Ejemplo completo
fn encontrar_mayor<T>(lista: &[T]) -> Option<&T>
where
    T: PartialOrd,
{
    if lista.is_empty() {
        return None;
    }
    
    let mut mayor = &lista[0];
    for item in lista {
        if item > mayor {
            mayor = item;
        }
    }
    Some(mayor)
}

fn main() {
    let numeros = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let chars = vec!['r', 'u', 's', 't'];
    
    println!("Mayor número: {:?}", encontrar_mayor(&numeros));
    println!("Mayor char: {:?}", encontrar_mayor(&chars));
}
```
