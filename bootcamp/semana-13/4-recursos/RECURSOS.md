# 📚 Recursos - Semana 13: Smart Pointers

## 📖 Documentación Oficial

### The Rust Book
- [Chapter 15: Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [15.1 Box<T>](https://doc.rust-lang.org/book/ch15-01-box.html)
- [15.2 Deref Trait](https://doc.rust-lang.org/book/ch15-02-deref.html)
- [15.3 Drop Trait](https://doc.rust-lang.org/book/ch15-03-drop.html)
- [15.4 Rc<T>](https://doc.rust-lang.org/book/ch15-04-rc.html)
- [15.5 RefCell<T>](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)
- [15.6 Reference Cycles](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)

### API Reference
- [std::boxed::Box](https://doc.rust-lang.org/std/boxed/struct.Box.html)
- [std::rc::Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html)
- [std::rc::Weak](https://doc.rust-lang.org/std/rc/struct.Weak.html)
- [std::sync::Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)
- [std::cell::RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
- [std::cell::Cell](https://doc.rust-lang.org/std/cell/struct.Cell.html)

---

## 🎓 Rust by Example

- [Box, stack and heap](https://doc.rust-lang.org/rust-by-example/std/box.html)
- [Rc](https://doc.rust-lang.org/rust-by-example/std/rc.html)
- [Arc](https://doc.rust-lang.org/rust-by-example/std/arc.html)

---

## 📺 Videos Recomendados

### Let's Get Rusty
- [Smart Pointers in Rust](https://www.youtube.com/watch?v=CTTiaOo4cbY)
- [Interior Mutability](https://www.youtube.com/watch?v=HwupNf9iCJk)

### Jon Gjengset
- [Crust of Rust: Smart Pointers](https://www.youtube.com/watch?v=8O0Nt9qY_vo)

### Ryan Levick
- [Understanding Rust's Box](https://www.youtube.com/watch?v=IPmRDS0OSxM)

---

## 📝 Artículos y Blogs

### Interior Mutability
- [Interior Mutability in Rust](https://ricardomartins.cc/2016/06/08/interior-mutability)
- [Interior mutability patterns](https://pitdicker.github.io/Interior-mutability-patterns/)

### Reference Counting
- [Rc and Weak explained](https://medium.com/@pvinchon/rust-reference-counting-explained-with-examples-42b81f5df0dd)

### Smart Pointers Comparisons
- [When to use Box, Rc, or RefCell](https://users.rust-lang.org/t/when-to-use-box-rc-refcell/8851)

---

## 🛠️ Herramientas

### Miri (Memory Checker)
```bash
# Instalar
rustup +nightly component add miri

# Ejecutar
cargo +nightly miri test
```

### Valgrind
```bash
# En Linux, verificar memory leaks
valgrind --leak-check=full ./target/debug/programa
```

---

## 📊 Comparación de Smart Pointers

| Tipo | Heap | Compartido | Mutable | Thread-Safe |
|------|------|------------|---------|-------------|
| `Box<T>` | ✅ | ❌ | ✅ (único) | ✅ |
| `Rc<T>` | ✅ | ✅ | ❌ | ❌ |
| `Arc<T>` | ✅ | ✅ | ❌ | ✅ |
| `RefCell<T>` | ❌ | ❌ | ✅ (runtime) | ❌ |
| `Cell<T>` | ❌ | ❌ | ✅ (Copy) | ❌ |
| `Mutex<T>` | ❌ | ❌ | ✅ (runtime) | ✅ |

---

## 🎯 Patrones Comunes

### Estructura de Árbol
```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    parent: Weak<RefCell<Node>>,
    children: Vec<Rc<RefCell<Node>>>,
}
```

### Cache con Interior Mutability
```rust
use std::cell::RefCell;
use std::collections::HashMap;

struct Cache {
    data: RefCell<HashMap<String, String>>,
}
```

### Shared State Multi-thread
```rust
use std::sync::{Arc, Mutex};

let data = Arc::new(Mutex::new(vec![]));
let data_clone = Arc::clone(&data);

std::thread::spawn(move || {
    data_clone.lock().unwrap().push(1);
});
```

---

## 🔗 Enlaces Adicionales

- [Rust Design Patterns - Smart Pointers](https://rust-unofficial.github.io/patterns/)
- [Nomicon - Working with Unsafe](https://doc.rust-lang.org/nomicon/)
- [Too Many Lists](https://rust-unofficial.github.io/too-many-lists/) - Tutorial sobre listas enlazadas
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

---

## 📅 Semanas Relacionadas

| Semana | Tema | Relación |
|--------|------|----------|
| 02 | Ownership | Base para entender smart pointers |
| 10 | Lifetimes | Cómo los smart pointers manejan lifetimes |
| 14 | Concurrencia | Arc y Mutex para threads |
