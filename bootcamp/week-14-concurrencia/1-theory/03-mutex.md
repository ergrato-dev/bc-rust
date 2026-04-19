# Mutex y RwLock

## 🎯 Objetivos

- Proteger datos compartidos con `Mutex<T>`
- Usar `Arc<Mutex<T>>` para múltiples threads
- Conocer `RwLock<T>` para múltiples lectores
- Prevenir y detectar deadlocks

![Mutex](../0-assets/03-mutex.svg)

## 📚 Conceptos

### ¿Qué es un Mutex?

**Mut**ual **Ex**clusion: Solo un thread puede acceder a los datos a la vez.

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        // Adquirir lock
        let mut num = m.lock().unwrap();
        *num = 10;
        // Lock se libera al salir del scope
    }

    println!("Valor: {:?}", m);
}
```

### Mutex con Múltiples Threads

Para compartir un `Mutex` entre threads, necesitamos `Arc`:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

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

### Anatomía del Lock

```
┌─────────────────────────────────────────────────────────────┐
│                    MUTEX LIFECYCLE                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   1. lock()                                                 │
│      ┌─────────────────────────────────────┐               │
│      │ Thread A quiere acceso              │               │
│      │ ↓                                   │               │
│      │ ┌─────────┐    ┌─────────┐         │               │
│      │ │ LOCKED  │───►│  DATA   │         │               │
│      │ └─────────┘    └─────────┘         │               │
│      │      │              ↑              │               │
│      │      └──────────────┘              │               │
│      │      MutexGuard<T>                 │               │
│      └─────────────────────────────────────┘               │
│                                                             │
│   2. Mientras tanto...                                      │
│      ┌─────────────────────────────────────┐               │
│      │ Thread B intenta lock()             │               │
│      │ ↓                                   │               │
│      │ ████████ BLOQUEADO ████████        │               │
│      │ (esperando que A libere)            │               │
│      └─────────────────────────────────────┘               │
│                                                             │
│   3. drop(guard) o salir de scope                          │
│      ┌─────────────────────────────────────┐               │
│      │ Lock liberado → Thread B continúa   │               │
│      └─────────────────────────────────────┘               │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## 🔧 API de Mutex

### Adquirir Lock

```rust
use std::sync::Mutex;

let m = Mutex::new(5);

// lock() - Bloquea hasta obtener acceso
let guard = m.lock().unwrap();

// try_lock() - No bloquea
match m.try_lock() {
    Ok(guard) => println!("Lock adquirido: {}", *guard),
    Err(_) => println!("Lock ocupado"),
}
```

### MutexGuard

```rust
let m = Mutex::new(vec![1, 2, 3]);

{
    let mut guard = m.lock().unwrap();
    
    // MutexGuard implementa Deref y DerefMut
    guard.push(4);           // Como si fuera &mut Vec
    println!("{:?}", *guard);
    
} // guard se dropea aquí, liberando el lock

// Ahora otros pueden adquirir el lock
```

### Manejo de Errores (Poisoning)

Si un thread entra en panic mientras tiene el lock, el Mutex queda "envenenado":

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let data_clone = Arc::clone(&data);

    let _ = thread::spawn(move || {
        let mut guard = data_clone.lock().unwrap();
        guard.push(4);
        panic!("Oops!"); // Mutex queda envenenado
    }).join();

    // lock() retorna Err si está envenenado
    match data.lock() {
        Ok(guard) => println!("Datos: {:?}", *guard),
        Err(poisoned) => {
            // Podemos recuperar los datos
            let guard = poisoned.into_inner();
            println!("Recuperado: {:?}", *guard);
        }
    }
}
```

## 📖 RwLock - Múltiples Lectores

`RwLock` permite múltiples lectores O un escritor:

```rust
use std::sync::RwLock;

let lock = RwLock::new(5);

// Múltiples lectores simultáneos
{
    let r1 = lock.read().unwrap();
    let r2 = lock.read().unwrap();  // OK!
    println!("r1: {}, r2: {}", *r1, *r2);
}

// Solo un escritor
{
    let mut w = lock.write().unwrap();
    *w += 1;
}
```

### RwLock con Threads

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];

    // 5 lectores
    for i in 0..5 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let read = data.read().unwrap();
            println!("Lector {}: {:?}", i, *read);
        }));
    }

    // 1 escritor
    {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let mut write = data.write().unwrap();
            write.push(4);
            println!("Escritor agregó 4");
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}
```

## ⚠️ Deadlocks

### Ejemplo de Deadlock

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let a = Arc::new(Mutex::new(1));
    let b = Arc::new(Mutex::new(2));

    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);
    let t1 = thread::spawn(move || {
        let _a = a1.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(100));
        let _b = b1.lock().unwrap();  // Espera b
    });

    let a2 = Arc::clone(&a);
    let b2 = Arc::clone(&b);
    let t2 = thread::spawn(move || {
        let _b = b2.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(100));
        let _a = a2.lock().unwrap();  // Espera a - DEADLOCK!
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
```

### Prevención de Deadlocks

```rust
// 1. Siempre adquirir locks en el mismo orden
let _a = a.lock().unwrap();
let _b = b.lock().unwrap();

// 2. Usar try_lock con timeout
use std::time::Duration;
loop {
    if let Ok(guard) = mutex.try_lock() {
        // Usar guard
        break;
    }
    thread::sleep(Duration::from_millis(10));
}

// 3. Minimizar el scope del lock
{
    let mut data = mutex.lock().unwrap();
    *data += 1;
} // Lock liberado inmediatamente
// Continuar sin el lock
```

## 📊 Mutex vs RwLock

| Aspecto | Mutex | RwLock |
|---------|-------|--------|
| Lectores simultáneos | 1 | Múltiples |
| Escritores simultáneos | 1 | 1 |
| Overhead | Menor | Mayor |
| Caso de uso | Escrituras frecuentes | Lecturas frecuentes |
| Riesgo starvation | No | Escritores pueden esperar |

## 🎯 Patrones Comunes

### Contador Thread-Safe

```rust
use std::sync::{Arc, Mutex};
use std::thread;

struct Contador {
    valor: Mutex<i32>,
}

impl Contador {
    fn new() -> Self {
        Contador { valor: Mutex::new(0) }
    }

    fn incrementar(&self) {
        *self.valor.lock().unwrap() += 1;
    }

    fn obtener(&self) -> i32 {
        *self.valor.lock().unwrap()
    }
}

fn main() {
    let contador = Arc::new(Contador::new());
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&contador);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                c.incrementar();
            }
        }));
    }

    for h in handles { h.join().unwrap(); }
    println!("Total: {}", contador.obtener()); // 1000
}
```

### Cache Thread-Safe

```rust
use std::sync::RwLock;
use std::collections::HashMap;

struct Cache {
    data: RwLock<HashMap<String, String>>,
}

impl Cache {
    fn new() -> Self {
        Cache { data: RwLock::new(HashMap::new()) }
    }

    fn get(&self, key: &str) -> Option<String> {
        self.data.read().unwrap().get(key).cloned()
    }

    fn set(&self, key: String, value: String) {
        self.data.write().unwrap().insert(key, value);
    }
}
```

## 🎯 Resumen

```rust
use std::sync::{Arc, Mutex, RwLock};

// Mutex: exclusión mutua
let m = Arc::new(Mutex::new(0));
{
    let mut guard = m.lock().unwrap();
    *guard += 1;
}

// RwLock: múltiples lectores
let rw = Arc::new(RwLock::new(vec![]));
let r = rw.read().unwrap();   // Lectura
let w = rw.write().unwrap();  // Escritura
```

## 📖 Siguiente

[04-send-sync.md](04-send-sync.md) - Traits de seguridad
