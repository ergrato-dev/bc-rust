# Semana 14: Concurrencia

## 🎯 Objetivos de la Semana

Al finalizar esta semana, serás capaz de:

- Crear y gestionar threads con `std::thread`
- Comunicar threads mediante channels (`mpsc`)
- Proteger datos compartidos con `Mutex<T>` y `RwLock<T>`
- Entender los traits `Send` y `Sync`
- Aplicar patrones de concurrencia seguros

## 📚 Contenido

### Teoría

| Archivo | Tema | Duración |
|---------|------|----------|
| [01-threads.md](1-teoria/01-threads.md) | Threads y `std::thread` | 45 min |
| [02-channels.md](1-teoria/02-channels.md) | Channels y `mpsc` | 45 min |
| [03-mutex.md](1-teoria/03-mutex.md) | Mutex y RwLock | 45 min |
| [04-send-sync.md](1-teoria/04-send-sync.md) | Traits Send y Sync | 30 min |
| [05-patrones.md](1-teoria/05-patrones.md) | Patrones de Concurrencia | 45 min |

### Práctica

| Ejercicio | Tema | Dificultad |
|-----------|------|------------|
| [practica-01-threads](2-practica/practica-01-threads) | Crear y gestionar threads | ⭐ |
| [practica-02-channels](2-practica/practica-02-channels) | Comunicación con channels | ⭐⭐ |
| [practica-03-mutex](2-practica/practica-03-mutex) | Datos compartidos con Mutex | ⭐⭐ |
| [practica-04-patrones](2-practica/practica-04-patrones) | Patrones concurrentes | ⭐⭐⭐ |
| [proyecto-workers](2-practica/proyecto-workers) | Thread Pool | ⭐⭐⭐ |

## 🔑 Conceptos Clave

### Modelo de Concurrencia de Rust

```
┌─────────────────────────────────────────────────────────────┐
│              CONCURRENCIA SEGURA EN RUST                    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   "Fearless Concurrency" - El compilador previene:          │
│   • Data races                                              │
│   • Use-after-free                                          │
│   • Dangling pointers                                       │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   COMUNICACIÓN                    COMPARTIR DATOS           │
│   ┌─────────────┐                ┌─────────────┐           │
│   │  Channels   │                │   Mutex<T>  │           │
│   │   (mpsc)    │                │  RwLock<T>  │           │
│   │             │                │   Arc<T>    │           │
│   └─────────────┘                └─────────────┘           │
│   "Don't communicate            "Share memory              │
│    by sharing memory"            by communicating"         │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Comparación de Primitivas

| Primitiva | Uso | Thread-Safe | Bloquea |
|-----------|-----|-------------|---------|
| `thread::spawn` | Crear threads | ✅ | No |
| `mpsc::channel` | Comunicación | ✅ | Receiver sí |
| `Mutex<T>` | Exclusión mutua | ✅ | Sí |
| `RwLock<T>` | Múltiples lectores | ✅ | Parcial |
| `Arc<T>` | Ownership compartido | ✅ | No |

### Send y Sync

```rust
// Send: Puede transferirse entre threads
// Sync: Puede compartirse entre threads (via &T)

// ✅ La mayoría de tipos son Send + Sync
Arc<Mutex<T>>  // T: Send

// ❌ No son Send ni Sync
Rc<T>          // No thread-safe
RefCell<T>     // No thread-safe
*mut T         // Raw pointers
```

## 📋 Ejercicio Rápido

```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let contador = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let contador = Arc::clone(&contador);
        let handle = thread::spawn(move || {
            let mut num = contador.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Resultado: {}", *contador.lock().unwrap());
}
```

## ⚠️ Errores Comunes

### 1. Olvidar `move` en closures

```rust
// ❌ Error: closure may outlive the current function
let data = vec![1, 2, 3];
thread::spawn(|| println!("{:?}", data));

// ✅ Correcto
thread::spawn(move || println!("{:?}", data));
```

### 2. Usar Rc en lugar de Arc

```rust
// ❌ Error: Rc<T> cannot be sent between threads safely
let rc = Rc::new(5);
thread::spawn(move || println!("{}", rc));

// ✅ Correcto
let arc = Arc::new(5);
thread::spawn(move || println!("{}", arc));
```

### 3. Deadlock con múltiples Mutex

```rust
// ❌ Posible deadlock
let a = Arc::new(Mutex::new(1));
let b = Arc::new(Mutex::new(2));

// Thread 1: lock(a), lock(b)
// Thread 2: lock(b), lock(a)  // DEADLOCK!

// ✅ Siempre adquirir locks en el mismo orden
```

## 📖 Recursos

- [Teoría completa](1-teoria/)
- [Glosario](3-recursos/GLOSARIO.md)
- [Recursos adicionales](3-recursos/RECURSOS.md)

## ✅ Checklist de Progreso

- [ ] Leer teoría de threads
- [ ] Leer teoría de channels
- [ ] Leer teoría de Mutex
- [ ] Leer teoría de Send/Sync
- [ ] Completar práctica 01 (threads)
- [ ] Completar práctica 02 (channels)
- [ ] Completar práctica 03 (mutex)
- [ ] Completar práctica 04 (patrones)
- [ ] Completar proyecto (workers)
