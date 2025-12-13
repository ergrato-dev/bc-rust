# 📚 Recursos - Semana 10: Genéricos

## 📖 Documentación Oficial

### The Rust Book
- [Chapter 10: Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [10.1 Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [10.2 Traits: Defining Shared Behavior](https://doc.rust-lang.org/book/ch10-02-traits.html)

### Rust Reference
- [Generic Parameters](https://doc.rust-lang.org/reference/items/generics.html)
- [Const Generics](https://doc.rust-lang.org/reference/items/generics.html#const-generics)
- [Where Clauses](https://doc.rust-lang.org/reference/items/generics.html#where-clauses)

### Rust by Example
- [Generics](https://doc.rust-lang.org/rust-by-example/generics.html)
- [Bounds](https://doc.rust-lang.org/rust-by-example/generics/bounds.html)
- [Multiple Bounds](https://doc.rust-lang.org/rust-by-example/generics/multi_bounds.html)
- [Where Clauses](https://doc.rust-lang.org/rust-by-example/generics/where.html)
- [Associated Types](https://doc.rust-lang.org/rust-by-example/generics/assoc_items/types.html)
- [PhantomData](https://doc.rust-lang.org/rust-by-example/generics/phantom.html)

## 🔧 API Documentation

### Traits Comunes
- [Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html)
- [Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html)
- [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
- [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html)
- [Default](https://doc.rust-lang.org/std/default/trait.Default.html)
- [PartialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)
- [Eq](https://doc.rust-lang.org/std/cmp/trait.Eq.html)
- [PartialOrd](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)
- [Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html)
- [Hash](https://doc.rust-lang.org/std/hash/trait.Hash.html)

### Tipos Especiales
- [PhantomData](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)
- [Option<T>](https://doc.rust-lang.org/std/option/enum.Option.html)
- [Result<T, E>](https://doc.rust-lang.org/std/result/enum.Result.html)

## 📝 Artículos y Blogs

### Conceptos Fundamentales
- [Generics in Rust - A Deep Dive](https://blog.logrocket.com/understanding-rust-generics/)
- [Rust Generics Tutorial](https://www.koderhq.com/tutorial/rust/generics/)
- [Zero-Cost Abstractions in Rust](https://boats.gitlab.io/blog/post/zero-cost-abstractions/)

### Temas Avanzados
- [Const Generics MVP](https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html)
- [Type-State Pattern in Rust](https://cliffle.com/blog/rust-typestate/)
- [PhantomData and Variance](https://doc.rust-lang.org/nomicon/phantom-data.html)
- [Generic Associated Types (GATs)](https://blog.rust-lang.org/2022/10/28/gats-stabilization.html)

## 🎥 Videos

### YouTube
- [Rust Generics Explained](https://www.youtube.com/watch?v=nvur2Ast8hE) - Let's Get Rusty
- [Traits and Generics](https://www.youtube.com/watch?v=T0Xfltu4h3A) - Jon Gjengset
- [Const Generics in Rust](https://www.youtube.com/watch?v=yTQZahHST7k)

## 🏋️ Práctica Adicional

### Exercism
- [Rust Track - Generics](https://exercism.org/tracks/rust/concepts/generics)
- [Ejercicio: Generic Container](https://exercism.org/tracks/rust/exercises/simple-linked-list)

### Rustlings
- [Generics exercises](https://github.com/rust-lang/rustlings) - Sección `generics`

### Codewars
- [Kata con Generics en Rust](https://www.codewars.com/kata/search/rust?q=generic)

## 📊 Comparación con Otros Lenguajes

| Concepto | Rust | C++ | Java | TypeScript |
|----------|------|-----|------|------------|
| Genéricos | `fn foo<T>()` | `template<typename T>` | `<T>` | `<T>` |
| Bounds | `T: Trait` | `concept` (C++20) | `T extends X` | `T extends X` |
| Monomorphization | ✅ Siempre | ✅ Templates | ❌ Type erasure | ❌ Type erasure |
| Const generics | `const N: usize` | Template params | ❌ No | ❌ No |
| Tipos asociados | `type Item` | `typedef` | ❌ No | ❌ No |

## 🔍 Errores Comunes

### Error: "the trait bound is not satisfied"
```rust
// ❌ Error: T no implementa Clone
fn duplicar<T>(x: T) -> (T, T) {
    (x.clone(), x)
}

// ✅ Solución: añadir bound
fn duplicar<T: Clone>(x: T) -> (T, T) {
    (x.clone(), x.clone())
}
```

### Error: "cannot infer type"
```rust
// ❌ Error: tipo ambiguo
let v = Vec::new();

// ✅ Solución: especificar tipo
let v: Vec<i32> = Vec::new();
// o con turbofish
let v = Vec::<i32>::new();
```

### Error: "unused type parameter"
```rust
// ❌ Error: T no se usa
struct Id<T> {
    value: u64,
}

// ✅ Solución: usar PhantomData
use std::marker::PhantomData;
struct Id<T> {
    value: u64,
    _marker: PhantomData<T>,
}
```

## 🧪 Herramientas

### Cargo Expand
Ver el código generado por monomorphization:
```bash
cargo install cargo-expand
cargo expand
```

### Rust Analyzer
- Hover sobre tipos para ver tipos inferidos
- Inlay hints para parámetros de tipo

## 📌 Cheat Sheet

```rust
// Función genérica básica
fn foo<T>(x: T) -> T { x }

// Con trait bound inline
fn foo<T: Clone>(x: T) -> T { x.clone() }

// Múltiples bounds
fn foo<T: Clone + Debug>(x: T) { }

// Con where clause
fn foo<T, U>(t: T, u: U)
where
    T: Clone + Send,
    U: Debug + Default,
{ }

// Struct genérico
struct Wrapper<T> { value: T }

// Impl genérico
impl<T> Wrapper<T> { }

// Impl con bounds
impl<T: Display> Wrapper<T> { }

// Impl especializado
impl Wrapper<i32> { }

// Const generic
struct Array<T, const N: usize> {
    data: [T; N]
}

// Tipo asociado
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```
