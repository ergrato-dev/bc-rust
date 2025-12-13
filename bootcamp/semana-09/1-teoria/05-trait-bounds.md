# 🎯 Trait Bounds

## ¿Qué son los Trait Bounds?

Los **trait bounds** son restricciones que especifican qué traits debe implementar un tipo genérico para poder usarse en una función, struct o impl.

```rust
// T debe implementar Display
fn imprimir<T: std::fmt::Display>(valor: T) {
    println!("{}", valor);
}
```

## Sintaxis Básica

### En Funciones

```rust
use std::fmt::Display;

// Sintaxis con dos puntos
fn mostrar<T: Display>(item: T) {
    println!("{}", item);
}

// Sintaxis con where (más legible)
fn mostrar_donde<T>(item: T) 
where 
    T: Display 
{
    println!("{}", item);
}

fn main() {
    mostrar(42);
    mostrar("Hola");
    mostrar(3.14);
}
```

### Múltiples Bounds

```rust
use std::fmt::{Debug, Display};

// Con + para combinar traits
fn imprimir_debug<T: Display + Debug>(item: T) {
    println!("Display: {}", item);
    println!("Debug: {:?}", item);
}

// Con where (preferido para múltiples bounds)
fn procesar<T, U>(t: T, u: U)
where
    T: Display + Clone,
    U: Debug + Default,
{
    println!("{}", t);
    println!("{:?}", u);
}
```

## impl Trait

`impl Trait` es syntactic sugar para trait bounds:

### Como Parámetro

```rust
use std::fmt::Display;

// Forma larga
fn imprimir_largo<T: Display>(item: T) {
    println!("{}", item);
}

// Forma corta con impl Trait
fn imprimir_corto(item: impl Display) {
    println!("{}", item);
}

fn main() {
    imprimir_largo("Hola");
    imprimir_corto("Mundo");
}
```

### Como Tipo de Retorno

```rust
// Retorna algo que implementa Iterator
fn crear_contador() -> impl Iterator<Item = i32> {
    (1..=5).into_iter()
}

fn main() {
    for n in crear_contador() {
        print!("{} ", n); // 1 2 3 4 5
    }
}
```

## Bounds en Structs

```rust
use std::fmt::Display;

// T debe ser Display para crear esta struct
struct Contenedor<T: Display> {
    valor: T,
}

impl<T: Display> Contenedor<T> {
    fn new(valor: T) -> Self {
        Contenedor { valor }
    }
    
    fn mostrar(&self) {
        println!("Contenido: {}", self.valor);
    }
}

fn main() {
    let c = Contenedor::new(42);
    c.mostrar(); // Contenido: 42
}
```

## Bounds Condicionales

Implementar métodos solo cuando el tipo cumple ciertas condiciones:

```rust
use std::fmt::Display;

struct Par<T> {
    primero: T,
    segundo: T,
}

// Métodos para cualquier T
impl<T> Par<T> {
    fn new(primero: T, segundo: T) -> Self {
        Par { primero, segundo }
    }
}

// Métodos solo si T es Display
impl<T: Display> Par<T> {
    fn mostrar(&self) {
        println!("({}, {})", self.primero, self.segundo);
    }
}

// Métodos solo si T es PartialOrd
impl<T: PartialOrd> Par<T> {
    fn mayor(&self) -> &T {
        if self.primero >= self.segundo {
            &self.primero
        } else {
            &self.segundo
        }
    }
}

// Métodos solo si T es PartialOrd + Display
impl<T: PartialOrd + Display> Par<T> {
    fn mostrar_mayor(&self) {
        println!("El mayor es: {}", self.mayor());
    }
}
```

## Blanket Implementations

Implementar un trait para todos los tipos que cumplan cierta condición:

```rust
use std::fmt::Display;

trait Imprimible {
    fn imprimir(&self);
}

// Implementación automática para cualquier T que sea Display
impl<T: Display> Imprimible for T {
    fn imprimir(&self) {
        println!("{}", self);
    }
}

fn main() {
    42.imprimir();      // 42
    "Hola".imprimir();  // Hola
    3.14.imprimir();    // 3.14
}
```

## Ejemplos Prácticos

### Función de Comparación Genérica

```rust
use std::cmp::PartialOrd;

fn encontrar_mayor<T: PartialOrd>(lista: &[T]) -> Option<&T> {
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
    let palabras = vec!["manzana", "banana", "cereza"];
    
    println!("Mayor número: {:?}", encontrar_mayor(&numeros));
    println!("Mayor palabra: {:?}", encontrar_mayor(&palabras));
}
```

### Struct con Múltiples Bounds

```rust
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::collections::HashMap;

struct Cache<K, V>
where
    K: Eq + Hash + Clone + Display,
    V: Clone + Debug,
{
    datos: HashMap<K, V>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone + Display,
    V: Clone + Debug,
{
    fn new() -> Self {
        Cache { datos: HashMap::new() }
    }
    
    fn insertar(&mut self, clave: K, valor: V) {
        println!("Insertando clave: {}", clave);
        self.datos.insert(clave, valor);
    }
    
    fn obtener(&self, clave: &K) -> Option<V> {
        self.datos.get(clave).cloned()
    }
}
```

### Trait con Métodos Genéricos

```rust
trait Convertible {
    fn convertir<T: From<Self>>(&self) -> T
    where
        Self: Sized + Clone,
    {
        T::from(self.clone())
    }
}

impl Convertible for i32 {}
impl Convertible for String {}
```

## Comparación de Sintaxis

```rust
use std::fmt::{Debug, Display};
use std::clone::Clone;

// Todas estas funciones son equivalentes:

// 1. Bounds inline
fn forma1<T: Display + Debug + Clone>(x: T) {}

// 2. Where clause
fn forma2<T>(x: T) 
where 
    T: Display + Debug + Clone 
{}

// 3. impl Trait (solo para parámetros)
fn forma3(x: impl Display + Debug + Clone) {}
```

## Cuándo Usar Cada Forma

| Forma | Cuándo Usar |
|-------|-------------|
| `T: Bound` | Bounds simples, un solo parámetro |
| `where` | Múltiples parámetros, bounds complejos |
| `impl Trait` param | Funciones simples, código conciso |
| `impl Trait` return | Ocultar tipo concreto de retorno |

## Errores Comunes

### Error 1: Bound Faltante

```rust
fn imprimir<T>(valor: T) {
    // ❌ Error: T no implementa Display
    println!("{}", valor);
}

// ✅ Correcto
fn imprimir<T: std::fmt::Display>(valor: T) {
    println!("{}", valor);
}
```

### Error 2: Bound Inconsistente

```rust
struct Caja<T> {
    valor: T,
}

// ❌ Error: bounds deben coincidir
impl<T: Clone> Caja<T> {
    fn clonar(&self) -> T {
        self.valor.clone()
    }
}

impl<T> Caja<T> {  // Sin Clone bound
    fn clonar_error(&self) -> T {
        self.valor.clone() // ❌ Error!
    }
}
```

## Resumen

| Concepto | Sintaxis | Ejemplo |
|----------|----------|---------|
| Bound simple | `T: Trait` | `fn f<T: Clone>(x: T)` |
| Múltiples bounds | `T: A + B` | `T: Clone + Debug` |
| Where clause | `where T: Trait` | `where T: Clone` |
| impl Trait | `impl Trait` | `fn f(x: impl Clone)` |
| Blanket impl | `impl<T: X> Y for T` | Para todos los T |

---

## 🔗 Navegación

| ⬅️ Anterior | 🏠 Índice | ➡️ Siguiente |
|:------------|:--------:|-------------:|
| [Traits Estándar](04-traits-estandar.md) | [Semana 09](../README.md) | [Práctica 01](../2-practica/practica-01-definir-traits/README.md) |
