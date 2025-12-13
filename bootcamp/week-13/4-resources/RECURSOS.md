# Recursos - Semana 13: Smart Pointers

## 📚 Documentación Oficial

### The Rust Book
- [Capítulo 15: Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
  - [15.1 Box<T>](https://doc.rust-lang.org/book/ch15-01-box.html)
  - [15.2 Deref Trait](https://doc.rust-lang.org/book/ch15-02-deref.html)
  - [15.3 Drop Trait](https://doc.rust-lang.org/book/ch15-03-drop.html)
  - [15.4 Rc<T>](https://doc.rust-lang.org/book/ch15-04-rc.html)
  - [15.5 RefCell<T>](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)
  - [15.6 Reference Cycles](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)

### Rust by Example
- [Box, stack and heap](https://doc.rust-lang.org/rust-by-example/std/box.html)
- [Rc](https://doc.rust-lang.org/rust-by-example/std/rc.html)
- [Arc](https://doc.rust-lang.org/rust-by-example/std/arc.html)

### API Reference
- [std::boxed::Box](https://doc.rust-lang.org/std/boxed/struct.Box.html)
- [std::rc::Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html)
- [std::rc::Weak](https://doc.rust-lang.org/std/rc/struct.Weak.html)
- [std::sync::Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)
- [std::cell::Cell](https://doc.rust-lang.org/std/cell/struct.Cell.html)
- [std::cell::RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html)

## 🎥 Videos

- [Let's Get Rusty - Smart Pointers](https://www.youtube.com/watch?v=CTTiaOo4cbY)
- [Jon Gjengset - Crust of Rust: Smart Pointers](https://www.youtube.com/watch?v=8O0Nt9qY_vo)

## 📝 Artículos

- [Interior Mutability in Rust](https://ricardomartins.cc/2016/06/08/interior-mutability)
- [Understanding Rust's Rc and RefCell](https://medium.com/@mrcjln/understanding-rusts-rc-and-refcell-9f7b321e8dfa)
- [Implementing a Simple LRU Cache in Rust](https://dev.to/antoniosbarotsis/implementing-an-lru-cache-in-rust-5bj7)

## 🔧 Herramientas

### Miri (Memory Safety Checker)
```bash
rustup +nightly component add miri
cargo +nightly miri run
```

### Clippy Lints para Smart Pointers
```bash
cargo clippy -- -W clippy::rc_buffer -W clippy::rc_clone_in_vec_init
```

## 📖 Libros Adicionales

- **Programming Rust, 2nd Edition** - Capítulo 4: Ownership and Moves
- **Rust for Rustaceans** - Capítulo 1: Foundations (Interior Mutability)

## 🏋️ Práctica Extra

### Ejercicios Recomendados
1. [Exercism - Simple Linked List](https://exercism.org/tracks/rust/exercises/simple-linked-list)
2. [LeetCode - LRU Cache](https://leetcode.com/problems/lru-cache/)
3. [Rustlings - smart_pointers](https://github.com/rust-lang/rustlings)

### Proyectos Sugeridos
- Implementar un árbol binario con parent pointers
- Cache con TTL usando `Instant`
- Sistema de eventos con observers

## 🔗 Links Útiles

- [Rust Playground](https://play.rust-lang.org/)
- [Rust Discord - #beginners](https://discord.gg/rust-lang)
- [r/rust](https://www.reddit.com/r/rust/)
