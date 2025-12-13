# 📚 Trait Bounds en Generics

## ¿Qué son los Trait Bounds?

Los **trait bounds** son restricciones que especifican qué traits debe implementar un tipo genérico. Sin bounds, no puedes hacer casi nada con un tipo genérico porque el compilador no sabe qué operaciones están disponibles.

```rust
// ❌ Sin bound: ¿qué podemos hacer con T?
fn imprimir<T>(valor: T) {
    println!("{}", valor);  // Error: T no implementa Display
}

// ✅ Con bound: sabemos que T tiene Display
fn imprimir<T: std::fmt::Display>(valor: T) {
    println!("{}", valor);  // OK
}
```

## Sintaxis de Trait Bounds

### Sintaxis Inline

```rust
// Un bound
fn foo<T: Clone>(x: T) -> T {
    x.clone()
}

// Múltiples bounds con +
fn bar<T: Clone + Debug>(x: T) {
    println!("{:?}", x.clone());
}

// Múltiples parámetros con bounds
fn baz<T: Clone, U: Debug>(t: T, u: U) {
    let _ = t.clone();
    println!("{:?}", u);
}
```

### Sintaxis con `where`

La cláusula `where` es más legible para bounds complejos:

```rust
// Inline (difícil de leer)
fn procesar<T: Clone + Debug + Default, U: Display + PartialOrd>(t: T, u: U) { }

// Con where (más legible)
fn procesar<T, U>(t: T, u: U)
where
    T: Clone + Debug + Default,
    U: Display + PartialOrd,
{
    // ...
}
```

### Cuándo Usar `where`

| Situación | Recomendación |
|-----------|---------------|
| Un bound simple | Inline: `<T: Clone>` |
| Múltiples bounds en un tipo | `where` |
| Múltiples tipos con bounds | `where` |
| Bounds que dependen de otros tipos | `where` |

```rust
// where es necesario cuando el bound involucra el tipo de retorno
fn crear_default<T>() -> T
where
    T: Default,
{
    T::default()
}

// where permite bounds en tipos asociados
fn procesar<I>(iter: I)
where
    I: Iterator,
    I::Item: Display,
{
    for item in iter {
        println!("{}", item);
    }
}
```

## Bounds Comunes

### Display y Debug

```rust
use std::fmt::{Display, Debug};

fn mostrar_ambos<T: Display + Debug>(valor: T) {
    println!("Display: {}", valor);
    println!("Debug: {:?}", valor);
}
```

### Clone y Copy

```rust
fn duplicar<T: Clone>(valor: &T) -> (T, T) {
    (valor.clone(), valor.clone())
}

fn triplicar<T: Copy>(valor: T) -> (T, T, T) {
    (valor, valor, valor)  // Copy permite esto sin .clone()
}
```

### PartialEq y Eq

```rust
fn son_iguales<T: PartialEq>(a: &T, b: &T) -> bool {
    a == b
}

fn todos_iguales<T: Eq>(slice: &[T]) -> bool {
    if slice.is_empty() {
        return true;
    }
    let primero = &slice[0];
    slice.iter().all(|x| x == primero)
}
```

### PartialOrd y Ord

```rust
fn mayor<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn ordenar<T: Ord>(slice: &mut [T]) {
    slice.sort();
}
```

### Default

```rust
fn con_default<T: Default>() -> T {
    T::default()
}

fn o_default<T: Default>(opcion: Option<T>) -> T {
    opcion.unwrap_or_default()
}
```

### From e Into

```rust
fn convertir<T, U>(valor: T) -> U
where
    U: From<T>,
{
    U::from(valor)
}

fn aceptar_convertible<T>(valor: T)
where
    T: Into<String>,
{
    let s: String = valor.into();
    println!("{}", s);
}
```

## Bounds en Structs e Impl

### Bounds en la Definición del Struct

```rust
// ⚠️ Generalmente NO recomendado
struct Contenedor<T: Clone> {
    valor: T,
}

// ✅ Mejor: sin bounds en el struct
struct Contenedor<T> {
    valor: T,
}

// Agregar bounds solo donde se necesitan
impl<T: Clone> Contenedor<T> {
    fn clonar_valor(&self) -> T {
        self.valor.clone()
    }
}
```

### Bounds en impl Blocks Separados

```rust
struct Datos<T> {
    items: Vec<T>,
}

// Métodos disponibles para TODOS los T
impl<T> Datos<T> {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
    
    fn agregar(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
}

// Solo para T: Clone
impl<T: Clone> Datos<T> {
    fn duplicar_primero(&self) -> Option<T> {
        self.items.first().cloned()
    }
}

// Solo para T: Ord
impl<T: Ord> Datos<T> {
    fn ordenar(&mut self) {
        self.items.sort();
    }
    
    fn mayor(&self) -> Option<&T> {
        self.items.iter().max()
    }
}

// Solo para T: Display
impl<T: std::fmt::Display> Datos<T> {
    fn imprimir_todos(&self) {
        for item in &self.items {
            println!("{}", item);
        }
    }
}
```

## Bounds con Tipos Asociados

```rust
// Iterator tiene un tipo asociado Item
fn sumar_iterador<I>(iter: I) -> i32
where
    I: Iterator<Item = i32>,
{
    iter.sum()
}

// Bound en el tipo asociado
fn mostrar_items<I>(iter: I)
where
    I: Iterator,
    I::Item: Display,
{
    for item in iter {
        println!("{}", item);
    }
}
```

## Bounds Condicionales

```rust
use std::fmt::Debug;

struct Wrapper<T> {
    valor: T,
}

// Debug solo si T: Debug
impl<T: Debug> Debug for Wrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Wrapper")
            .field("valor", &self.valor)
            .finish()
    }
}

// Clone solo si T: Clone
impl<T: Clone> Clone for Wrapper<T> {
    fn clone(&self) -> Self {
        Self {
            valor: self.valor.clone(),
        }
    }
}
```

## Supertraits

Un trait puede requerir otro trait:

```rust
// Comparable requiere que el tipo implemente PartialEq
trait Comparable: PartialEq {
    fn es_igual(&self, otro: &Self) -> bool {
        self == otro
    }
}

// Imprimible requiere Display
trait Imprimible: std::fmt::Display {
    fn imprimir(&self) {
        println!("{}", self);
    }
}
```

## Bounds Negativos (No Estables)

Rust no tiene bounds negativos estables, pero puedes lograr efectos similares:

```rust
// Usando marker traits
use std::marker::PhantomData;

struct SoloSend<T: Send> {
    valor: T,
}

struct NoSync<T> {
    valor: T,
    _marker: PhantomData<*const ()>,  // *const () no es Sync
}
```

## Errores Comunes

### Bound Faltante

```rust
// ❌ Error
fn clonar<T>(x: &T) -> T {
    x.clone()  // T no implementa Clone
}

// ✅ Correcto
fn clonar<T: Clone>(x: &T) -> T {
    x.clone()
}
```

### Bound Innecesario

```rust
// ❌ Bounds excesivos
fn largo<T: Clone + Debug + Display>(slice: &[T]) -> usize {
    slice.len()  // No usa Clone, Debug, ni Display
}

// ✅ Sin bounds innecesarios
fn largo<T>(slice: &[T]) -> usize {
    slice.len()
}
```

### Bound en Lugar Incorrecto

```rust
// ❌ Bound en el struct limita todo uso
struct Datos<T: Clone> {
    valor: T,
}

// ✅ Bound solo donde se necesita
struct Datos<T> {
    valor: T,
}

impl<T: Clone> Datos<T> {
    fn clonar(&self) -> T {
        self.valor.clone()
    }
}
```

## Resumen

| Sintaxis | Uso |
|----------|-----|
| `<T: Trait>` | Bound inline simple |
| `<T: A + B>` | Múltiples bounds |
| `where T: Trait` | Bounds complejos |
| `where I::Item: Trait` | Bound en tipo asociado |
| `impl<T: Trait>` | Bound en implementación |
