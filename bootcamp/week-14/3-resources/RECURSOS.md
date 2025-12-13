# Recursos - Semana 14: Concurrencia

## 📚 Documentación Oficial

### The Rust Book
- [Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Using Threads](https://doc.rust-lang.org/book/ch16-01-threads.html)
- [Message Passing](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
- [Shared State](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [Send and Sync](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)

### Rust by Example
- [Threads](https://doc.rust-lang.org/rust-by-example/std_misc/threads.html)
- [Channels](https://doc.rust-lang.org/rust-by-example/std_misc/channels.html)

### Standard Library
- [std::thread](https://doc.rust-lang.org/std/thread/)
- [std::sync::mpsc](https://doc.rust-lang.org/std/sync/mpsc/)
- [std::sync::Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
- [std::sync::RwLock](https://doc.rust-lang.org/std/sync/struct.RwLock.html)
- [std::sync::Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)

---

## 🎓 Tutoriales y Artículos

### Conceptos Fundamentales
- [Rust Concurrency Explained](https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html) - Blog oficial
- [Send and Sync in Rust](https://doc.rust-lang.org/nomicon/send-and-sync.html) - Rustonomicon

### Patrones
- [Rust Book - Web Server (Thread Pool)](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
- [Parallel Programming in Rust](https://www.rayon-rs.org/)

### Avanzado
- [Atomics and Memory Ordering](https://doc.rust-lang.org/nomicon/atomics.html)
- [Interior Mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)

---

## 📦 Crates Útiles

### Paralelismo de Datos
```toml
[dependencies]
rayon = "1.10"
```
- [rayon](https://docs.rs/rayon/) - Paralelismo de datos fácil
- Iteradores paralelos: `par_iter()`, `par_chunks()`

### Async Runtime
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```
- [tokio](https://docs.rs/tokio/) - Runtime async más popular

### Channels Avanzados
```toml
[dependencies]
crossbeam-channel = "0.5"
```
- [crossbeam-channel](https://docs.rs/crossbeam-channel/) - Channels MPMC

### Primitivas de Sincronización
```toml
[dependencies]
parking_lot = "0.12"
```
- [parking_lot](https://docs.rs/parking_lot/) - Mutex/RwLock más rápidos

---

## 🛠️ Herramientas de Debugging

### ThreadSanitizer
```bash
RUSTFLAGS="-Z sanitizer=thread" cargo +nightly run
```

### Logging Concurrente
```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

```rust
use tracing::{info, span, Level};

let span = span!(Level::INFO, "worker", id = worker_id);
let _enter = span.enter();
info!("Procesando job");
```

---

## 📊 Comparación de Primitivas

| Primitiva | Uso | Overhead | Cuando Usar |
|-----------|-----|----------|-------------|
| `Mutex` | Exclusión mutua | Bajo | Escrituras frecuentes |
| `RwLock` | Múltiples lectores | Medio | Lecturas >> Escrituras |
| `Channel` | Comunicación | Medio | Pasar ownership |
| `Atomic` | Contadores simples | Muy bajo | Operaciones simples |
| `Arc` | Compartir ownership | Bajo | Datos inmutables compartidos |

---

## 🎯 Ejercicios Adicionales

### Exercism
- [exercism.org/tracks/rust](https://exercism.org/tracks/rust)
  - "Parallel Letter Frequency"
  - "Dot DSL"

### Rustlings
```bash
rustlings watch
```
- Ejercicios de threads y smart pointers

### Advent of Code
- Muchos problemas se benefician de paralelización
- [adventofcode.com](https://adventofcode.com/)

---

## 📖 Libros Recomendados

1. **Programming Rust** (O'Reilly)
   - Capítulo 19: Concurrency

2. **Rust in Action** (Manning)
   - Capítulo sobre sistemas concurrentes

3. **Hands-On Concurrency with Rust** (Packt)
   - Libro completo sobre concurrencia

---

## 🔗 Links Útiles

- [Are We Async Yet?](https://areweasyncyet.rs/) - Estado del ecosistema async
- [Rust Concurrency Cheat Sheet](https://upsuper.github.io/rust-cheatsheet/)
- [Rust Atomics and Locks](https://marabos.nl/atomics/) - Libro gratuito

---

## 💡 Tips de Performance

### Reducir Contención
```rust
// MAL: Lock durante toda la operación
let mut guard = mutex.lock().unwrap();
expensive_computation(&mut guard);

// BIEN: Lock mínimo
let data = {
    let guard = mutex.lock().unwrap();
    guard.clone()
};
expensive_computation(&data);
```

### Evitar False Sharing
```rust
// Usar padding entre datos accedidos por diferentes threads
#[repr(align(64))]
struct CacheLine<T>(T);
```

### Batch Processing
```rust
// Procesar múltiples items por lock
let mut guard = mutex.lock().unwrap();
for item in batch {
    process(&mut guard, item);
}
```
