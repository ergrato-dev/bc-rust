# Práctica 03: Estado Compartido con Mutex

## 🎯 Objetivos

- Usar `Mutex<T>` para exclusión mutua
- Combinar `Arc<Mutex<T>>` para compartir entre threads
- Usar `RwLock<T>` para múltiples lectores
- Evitar deadlocks

## 📚 Conceptos Clave

### Mutex Básico

```rust
use std::sync::Mutex;

let datos = Mutex::new(0);

{
    let mut guard = datos.lock().unwrap();
    *guard += 1;
} // lock liberado aquí
```

### Arc + Mutex

```rust
use std::sync::{Arc, Mutex};

let contador = Arc::new(Mutex::new(0));
let contador_clone = Arc::clone(&contador);

thread::spawn(move || {
    let mut num = contador_clone.lock().unwrap();
    *num += 1;
});
```

### RwLock

```rust
use std::sync::RwLock;

let datos = RwLock::new(vec![1, 2, 3]);

// Múltiples lectores
let r1 = datos.read().unwrap();
let r2 = datos.read().unwrap(); // OK simultáneo

// Un escritor (exclusivo)
let mut w = datos.write().unwrap();
w.push(4);
```

## 📝 Ejercicios

### Ejercicio 1: Contador Concurrente

Implementa `ContadorConcurrente` con:

```rust
impl ContadorConcurrente {
    fn new(inicial: i32) -> Self
    fn increment(&self)
    fn decrement(&self)
    fn get(&self) -> i32
}
```

### Ejercicio 2: Cache Concurrente

Implementa `CacheConcurrente<K, V>` usando RwLock:

```rust
impl<K, V> CacheConcurrente<K, V> {
    fn new() -> Self
    fn get(&self, key: &K) -> Option<V>
    fn set(&self, key: K, value: V)
    fn contains(&self, key: &K) -> bool
}
```

### Ejercicio 3: Sistema Bancario

Implementa `Banco` con transferencias atómicas:

```rust
impl Banco {
    fn new() -> Self
    fn crear_cuenta(&self, nombre: &str, saldo: i32)
    fn depositar(&self, nombre: &str, cantidad: i32) -> bool
    fn retirar(&self, nombre: &str, cantidad: i32) -> bool
    fn transferir(&self, desde: &str, hacia: &str, cantidad: i32) -> bool
    fn saldo(&self, nombre: &str) -> Option<i32>
}
```

## ⚠️ Evitar Deadlocks

```rust
// MAL: Dos locks en orden diferente
let a = mutex_a.lock();
let b = mutex_b.lock();

// En otro thread:
let b = mutex_b.lock(); // Deadlock!
let a = mutex_a.lock();

// BIEN: Un solo lock o siempre mismo orden
let mut cuentas = self.cuentas.lock().unwrap();
// Operar sobre ambas cuentas con el mismo lock
```

## ▶️ Ejecución

```bash
cargo run
cargo test
cargo test -- --nocapture
```

## ✅ Criterios de Evaluación

| Criterio | Puntos |
|----------|--------|
| ContadorConcurrente thread-safe | 25 |
| CacheConcurrente con RwLock | 30 |
| Banco con transferencias atómicas | 35 |
| Sin deadlocks | 10 |

## 🔗 Recursos

- [Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
- [RwLock](https://doc.rust-lang.org/std/sync/struct.RwLock.html)
- [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)
