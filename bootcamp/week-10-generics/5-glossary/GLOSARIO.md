# 📖 Glosario - Semana 10: Genéricos

## A

### Associated Type (Tipo Asociado)
Tipo definido dentro de un trait que se especifica en la implementación. A diferencia de los parámetros genéricos, solo puede haber una implementación por tipo.
```rust
trait Iterator {
    type Item;  // Tipo asociado
    fn next(&mut self) -> Option<Self::Item>;
}
```

## B

### Bound (Restricción)
Requisito que un tipo genérico debe cumplir, típicamente implementar ciertos traits.
```rust
fn print<T: Display>(x: T) { }  // T debe implementar Display
```

## C

### Const Generic
Parámetro genérico que es un valor constante (no un tipo) conocido en tiempo de compilación.
```rust
struct Array<T, const N: usize> {
    data: [T; N]
}
```

### Constraint (ver Bound)
Sinónimo de trait bound. Restricción sobre un tipo genérico.

## D

### Dynamic Dispatch
Resolución de métodos en tiempo de ejecución usando una vtable. Se usa con `dyn Trait`.
```rust
fn process(x: &dyn Display) { }  // Dispatch dinámico
```

## G

### Generic (Genérico)
Código que puede operar sobre múltiples tipos diferentes. En Rust, los genéricos se resuelven en tiempo de compilación.
```rust
fn identity<T>(x: T) -> T { x }
```

### Generic Parameter (Parámetro Genérico)
Placeholder para un tipo que se especifica al usar la función, struct o trait.
```rust
struct Pair<T, U> {  // T y U son parámetros genéricos
    first: T,
    second: U,
}
```

## H

### Higher-Ranked Trait Bound (HRTB)
Bound que funciona para cualquier lifetime, no solo uno específico.
```rust
fn foo<F>(f: F)
where
    F: for<'a> Fn(&'a str) -> &'a str
{ }
```

## I

### Impl Trait
Sintaxis para especificar que un tipo implementa un trait sin nombrar el tipo concreto.
```rust
fn make_iter() -> impl Iterator<Item = i32> {
    vec![1, 2, 3].into_iter()
}
```

### Inference (Inferencia de Tipos)
Capacidad del compilador de deducir tipos automáticamente.
```rust
let x = identity(42);  // T inferido como i32
```

## M

### Monomorphization
Proceso donde el compilador genera código especializado para cada tipo concreto usado con un genérico.
```rust
// Rust genera versiones separadas:
identity::<i32>(42);    // Código para i32
identity::<String>(s);  // Código para String
```

### Multiple Bounds (Múltiples Restricciones)
Requerir que un tipo implemente varios traits.
```rust
fn foo<T: Clone + Debug>(x: T) { }  // T debe ser Clone Y Debug
```

## P

### Phantom Type (Tipo Fantasma)
Tipo usado para marcar información en el sistema de tipos sin almacenar datos.
```rust
use std::marker::PhantomData;

struct Id<Entity> {
    value: u64,
    _marker: PhantomData<Entity>,
}
```

### PhantomData
Struct de tamaño cero que "usa" un tipo genérico sin almacenarlo.
```rust
use std::marker::PhantomData;
struct Tagged<T>(PhantomData<T>);
```

## S

### Specialization (Especialización)
Capacidad de proporcionar implementaciones más específicas para ciertos tipos. (Feature inestable en Rust)
```rust
// Impl general
impl<T> Trait for T { }
// Impl especializada
impl Trait for i32 { }  // Más específica
```

### Static Dispatch
Resolución de métodos en tiempo de compilación. Los genéricos usan static dispatch.
```rust
fn print<T: Display>(x: T) { }  // Static dispatch
```

### Supertrait
Trait que otro trait requiere como prerequisito.
```rust
trait Printable: Display {  // Display es supertrait
    fn print(&self) {
        println!("{}", self);
    }
}
```

## T

### Trait Bound (ver Bound)
Restricción que especifica qué traits debe implementar un tipo genérico.

### Turbofish
Sintaxis `::< >` para especificar tipos explícitamente.
```rust
let x = "42".parse::<i32>();
let v = Vec::<i32>::new();
```

### Type Erasure
Técnica donde la información de tipo se pierde en tiempo de ejecución. Rust lo evita con monomorphization pero lo usa con `dyn Trait`.

### Type Parameter (ver Generic Parameter)
Parámetro que representa un tipo en una definición genérica.

### Type State
Patrón donde los estados de un objeto se representan como tipos diferentes.
```rust
struct Door<State> { _state: PhantomData<State> }
struct Open;
struct Closed;

impl Door<Closed> {
    fn open(self) -> Door<Open> { ... }
}
```

## V

### Variance (Varianza)
Describe cómo los tipos genéricos se relacionan con sus subtipos. En Rust: covariant, contravariant, invariant.

### Vtable
Tabla de punteros a funciones usada para dynamic dispatch con trait objects.

## W

### Where Clause
Sintaxis alternativa para especificar trait bounds, más legible para bounds complejos.
```rust
fn process<T, U>(t: T, u: U)
where
    T: Clone + Send + Sync,
    U: Iterator<Item = T>,
{ }
```

## Z

### Zero-Cost Abstraction
Principio de Rust donde las abstracciones (como genéricos) no tienen costo en tiempo de ejecución comparado con código escrito a mano.

---

## Símbolos y Sintaxis

| Símbolo | Nombre | Uso |
|---------|--------|-----|
| `<T>` | Parámetro genérico | Declarar tipo genérico |
| `T: Trait` | Trait bound | Restringir tipo |
| `T: A + B` | Múltiples bounds | T debe implementar A y B |
| `where` | Cláusula where | Bounds complejos |
| `::<>` | Turbofish | Especificar tipo explícito |
| `impl Trait` | Impl trait | Tipo opaco que implementa trait |
| `dyn Trait` | Trait object | Dynamic dispatch |
| `const N: T` | Const generic | Valor constante genérico |
| `type Item` | Tipo asociado | Tipo definido en trait |
| `Self::Item` | Acceso tipo asociado | Usar tipo asociado |
| `PhantomData<T>` | Phantom data | Marcar uso de tipo |
