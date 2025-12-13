# 📚 Funciones Genéricas

## Sintaxis Básica

Una función genérica declara uno o más **parámetros de tipo** entre `<>` después del nombre:

```rust
fn nombre<T>(parametro: T) -> T {
    // cuerpo
}
```

### Ejemplos Básicos

```rust
// Función identidad: retorna el mismo valor
fn identidad<T>(valor: T) -> T {
    valor
}

// Uso
let x = identidad(5);        // T = i32
let s = identidad("hola");   // T = &str
let v = identidad(vec![1]);  // T = Vec<i32>
```

## Múltiples Parámetros de Tipo

Puedes tener varios parámetros de tipo:

```rust
fn par<T, U>(primero: T, segundo: U) -> (T, U) {
    (primero, segundo)
}

fn main() {
    let p1 = par(1, "hola");        // (i32, &str)
    let p2 = par(3.14, true);       // (f64, bool)
    let p3 = par("a", vec![1, 2]);  // (&str, Vec<i32>)
}
```

## Funciones con Referencias Genéricas

```rust
// Referencia inmutable
fn primero<T>(slice: &[T]) -> Option<&T> {
    slice.first()
}

// Referencia mutable
fn intercambiar<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b);
}

fn main() {
    let nums = [1, 2, 3];
    println!("Primero: {:?}", primero(&nums));  // Some(&1)
    
    let mut x = 5;
    let mut y = 10;
    intercambiar(&mut x, &mut y);
    println!("x={}, y={}", x, y);  // x=10, y=5
}
```

## Trait Bounds en Funciones

Sin trait bounds, no puedes hacer mucho con un tipo genérico:

```rust
// ❌ Error: no sabemos qué operaciones tiene T
fn imprimir<T>(valor: T) {
    println!("{}", valor);  // Error: T no implementa Display
}

// ✅ Con trait bound
fn imprimir<T: std::fmt::Display>(valor: T) {
    println!("{}", valor);
}
```

### Sintaxis de Bounds

```rust
// Sintaxis 1: después del parámetro de tipo
fn foo<T: Trait>(x: T) { }

// Sintaxis 2: con where (más legible para bounds complejos)
fn foo<T>(x: T) 
where 
    T: Trait 
{ }
```

### Múltiples Bounds

```rust
use std::fmt::{Display, Debug};

// Con +
fn mostrar<T: Display + Debug>(valor: T) {
    println!("Display: {}", valor);
    println!("Debug: {:?}", valor);
}

// Con where (más legible)
fn mostrar_v2<T>(valor: T) 
where 
    T: Display + Debug 
{
    println!("Display: {}", valor);
    println!("Debug: {:?}", valor);
}
```

## Ejemplos Prácticos

### Encontrar el Mayor

```rust
fn mayor<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", mayor(10, 20));       // 20
    println!("{}", mayor(3.14, 2.71));   // 3.14
    println!("{}", mayor('a', 'z'));     // z
}
```

### Buscar en Slice

```rust
fn contiene<T: PartialEq>(slice: &[T], objetivo: &T) -> bool {
    for item in slice {
        if item == objetivo {
            return true;
        }
    }
    false
}

fn main() {
    let nums = [1, 2, 3, 4, 5];
    println!("{}", contiene(&nums, &3));  // true
    println!("{}", contiene(&nums, &9));  // false
    
    let palabras = ["hola", "mundo"];
    println!("{}", contiene(&palabras, &"mundo"));  // true
}
```

### Contar Ocurrencias

```rust
fn contar<T: PartialEq>(slice: &[T], objetivo: &T) -> usize {
    slice.iter().filter(|&x| x == objetivo).count()
}

fn main() {
    let nums = [1, 2, 2, 3, 2, 4];
    println!("Cantidad de 2s: {}", contar(&nums, &2));  // 3
}
```

### Clonar y Transformar

```rust
fn duplicar_todos<T: Clone>(slice: &[T]) -> Vec<T> {
    let mut resultado = Vec::with_capacity(slice.len() * 2);
    for item in slice {
        resultado.push(item.clone());
        resultado.push(item.clone());
    }
    resultado
}

fn main() {
    let nums = vec![1, 2, 3];
    let duplicados = duplicar_todos(&nums);
    println!("{:?}", duplicados);  // [1, 1, 2, 2, 3, 3]
}
```

## Inferencia de Tipos

El compilador de Rust es muy bueno infiriendo tipos:

```rust
fn crear_vec<T>() -> Vec<T> {
    Vec::new()
}

fn main() {
    // El compilador infiere T del contexto
    let v1: Vec<i32> = crear_vec();
    
    // O del uso posterior
    let mut v2 = crear_vec();
    v2.push(42);  // Ahora sabe que T = i32
    
    // Turbofish cuando no hay suficiente contexto
    let v3 = crear_vec::<String>();
}
```

## Turbofish `::<>`

Cuando el compilador no puede inferir el tipo, usa **turbofish**:

```rust
fn main() {
    // Parsear string a número
    let n = "42".parse::<i32>().unwrap();
    
    // Crear colección vacía de tipo específico
    let v = Vec::<f64>::new();
    
    // Especificar tipo en función genérica
    let resultado = std::mem::size_of::<u64>();
    println!("Tamaño de u64: {} bytes", resultado);
}
```

## Retornando Tipos Genéricos

```rust
// Retornar el mismo tipo que el input
fn identidad<T>(x: T) -> T {
    x
}

// Retornar un tipo diferente (construido)
fn en_vec<T>(x: T) -> Vec<T> {
    vec![x]
}

// Retornar Option
fn primero_si_no_vacio<T>(slice: &[T]) -> Option<&T> {
    if slice.is_empty() {
        None
    } else {
        Some(&slice[0])
    }
}
```

## Funciones Genéricas en Módulos

```rust
mod utilidades {
    pub fn intercambiar<T>(a: &mut T, b: &mut T) {
        std::mem::swap(a, b);
    }
    
    pub fn mayor<T: PartialOrd>(a: T, b: T) -> T {
        if a > b { a } else { b }
    }
}

fn main() {
    let mut x = 1;
    let mut y = 2;
    utilidades::intercambiar(&mut x, &mut y);
    
    let m = utilidades::mayor(10, 20);
}
```

## Comparación: Genérico vs Concreto

```rust
// Versión concreta: solo i32
fn sumar_i32(a: i32, b: i32) -> i32 {
    a + b
}

// Versión genérica: cualquier tipo que implemente Add
use std::ops::Add;

fn sumar<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    // Ambas funcionan para i32
    let r1 = sumar_i32(1, 2);
    let r2 = sumar(1, 2);
    
    // Solo la genérica funciona para otros tipos
    let r3 = sumar(1.5, 2.5);
    let r4 = sumar(String::from("Hola "), String::from("Mundo"));
}
```

## Resumen

| Concepto | Ejemplo |
|----------|---------|
| Función genérica simple | `fn foo<T>(x: T) -> T` |
| Múltiples parámetros | `fn foo<T, U>(x: T, y: U)` |
| Con trait bound | `fn foo<T: Clone>(x: T)` |
| Múltiples bounds | `fn foo<T: Clone + Debug>(x: T)` |
| Con where | `fn foo<T>(x: T) where T: Clone` |
| Turbofish | `foo::<i32>(42)` |
