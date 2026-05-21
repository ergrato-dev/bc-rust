# 📖 `unsafe` Traits: `Send` y `Sync`

## ¿Qué son los Unsafe Traits?

Un **unsafe trait** es un trait cuya implementación requiere que el programador garantice
invariantes que el compilador no puede verificar. Implementarlo con `unsafe impl` es una
promesa al compilador: *"sé lo que hago y esta implementación es correcta"*.

```
┌────────────────────────────────────────────────────────────────┐
│                  SEND y SYNC EN RUST                           │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  T: Send                          T: Sync                      │
│  ┌──────────────────────────┐    ┌──────────────────────────┐  │
│  │ T puede MOVERSE          │    │ &T puede COMPARTIRSE     │  │
│  │ a otro thread            │    │ entre threads            │  │
│  │                          │    │                          │  │
│  │ spawn(move || { T })     │    │ Arc::new(T)              │  │
│  │ channel.send(T)          │    │ Mutex::new(T) → Sync     │  │
│  └──────────────────────────┘    └──────────────────────────┘  │
│                                                                │
│  Regla: T: Sync  ⟺  &T: Send                                   │
│                                                                │
│  Implementación automática (auto traits):                      │
│  Si todos los campos son Send → T es Send                      │
│  Si todos los campos son Sync → T es Sync                      │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

## `Send`: Transferir entre Threads

Un tipo `T: Send` puede transferirse de un thread a otro. Rust implementa `Send`
automáticamente si todos los campos son `Send`.

```rust
// ✅ Estos tipos son Send automáticamente
let x: i32 = 42;               // i32: Send
let s: String = "hola".into(); // String: Send
let v: Vec<i32> = vec![1,2,3]; // Vec<i32>: Send

// ❌ Estos tipos NO son Send
use std::rc::Rc;
let rc: Rc<i32> = Rc::new(1);  // Rc<T>: !Send (no es thread-safe)

use std::cell::Cell;
let cell: Cell<i32> = Cell::new(0);  // Cell<T>: !Send (interior mutability sin Mutex)

// ❌ *mut T tampoco es Send por defecto
let ptr: *mut i32 = std::ptr::null_mut();  // *mut T: !Send
```

### Por qué `Rc<T>` no es `Send`

```rust
// Rc usa un contador de referencias simple (no atómico)
let rc1 = Rc::new(42);
let rc2 = Rc::clone(&rc1);
// Si rc1 y rc2 estuvieran en threads distintos, el contador de referencias
// podría corromperse por data race → por eso Rc<T>: !Send
```

---

## `Sync`: Compartir Referencias entre Threads

Un tipo `T: Sync` significa que `&T` es seguro de compartir entre threads concurrentemente.

```rust
use std::sync::{Arc, Mutex};

// ✅ Arc<T> permite compartir T entre threads si T: Send + Sync
let arc = Arc::new(42_i32);
let arc2 = Arc::clone(&arc);

std::thread::spawn(move || {
    println!("{}", *arc2);  // Acceso compartido seguro
}).join().unwrap();

// ✅ Mutex<T> implementa Sync incluso si T: !Sync
// porque el Mutex garantiza acceso exclusivo
let shared = Arc::new(Mutex::new(vec![1, 2, 3]));
let shared2 = Arc::clone(&shared);

std::thread::spawn(move || {
    shared2.lock().unwrap().push(4);
}).join().unwrap();
```

---

## Implementar `Send` y `Sync` Manualmente

Solo es necesario cuando el tipo contiene raw pointers o tipos `!Send`/`!Sync`,
pero semánticamente SÍ es seguro para threading:

```rust
use std::sync::Arc;

/// Wrapper sobre un handle opaco de una librería C.
///
/// La documentación de la librería garantiza que:
/// - El handle puede transferirse entre threads (thread-ownership transfer OK)
/// - El acceso concurrente está permitido si se sincroniza externamente
pub struct CHandle {
    ptr: *mut u8,
}

impl CHandle {
    pub fn new(ptr: *mut u8) -> Self {
        CHandle { ptr }
    }

    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
}

// SAFETY: La API C garantiza que el handle puede transferirse entre threads.
// La responsabilidad de sincronización es del caller (usar con Arc<Mutex<CHandle>>).
unsafe impl Send for CHandle {}

// SAFETY: La API C garantiza que las lecturas concurrentes son seguras.
// Toda escritura debe sincronizarse mediante Mutex en el código Rust.
unsafe impl Sync for CHandle {}
```

### Uso correcto con `Arc<Mutex<T>>`

```rust
fn usar_handle() {
    let handle = Arc::new(Mutex::new(CHandle::new(std::ptr::null_mut())));
    let handle2 = Arc::clone(&handle);

    let t = std::thread::spawn(move || {
        let locked = handle2.lock().unwrap();
        let _ = locked.as_ptr();
    });

    t.join().unwrap();
}
```

---

## Marcadores de Negatividad: `!Send` y `!Sync`

Para tipos que **explícitamente NO deben ser** `Send` o `Sync`:

```rust
use std::marker::PhantomData;

/// Tipo que solo puede usarse en el thread que lo creó.
pub struct ThreadLocal<T> {
    value: T,
    // PhantomData<!Send> marca este tipo como !Send
    _not_send: PhantomData<*mut ()>,
}

// *mut () es !Send + !Sync, lo que hace que ThreadLocal<T> sea !Send + !Sync
// sin necesidad de declararlo explícitamente

impl<T> ThreadLocal<T> {
    pub fn new(value: T) -> Self {
        ThreadLocal {
            value,
            _not_send: PhantomData,
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }
}
```

---

## Tabla de Tipos Comunes y su Send/Sync

| Tipo | `Send` | `Sync` | Razón |
|------|--------|--------|-------|
| `i32`, `f64`, primitivos | ✅ | ✅ | Tipos copiables, sin estado compartido |
| `String`, `Vec<T>` | ✅ | ✅ | Ownership único |
| `Arc<T>` (T: Send+Sync) | ✅ | ✅ | Contador atómico |
| `Rc<T>` | ❌ | ❌ | Contador no atómico |
| `Mutex<T>` (T: Send) | ✅ | ✅ | Garantiza exclusión mutua |
| `RefCell<T>` | ✅ | ❌ | Borrow checking en runtime, no atómico |
| `Cell<T>` | ✅ | ❌ | Interior mutability sin sincronización |
| `*mut T`, `*const T` | ❌ | ❌ | Sin garantías de threading |
| `MutexGuard<T>` | ❌ | ✅ | No se puede enviar el guard a otro thread |

---

## Declarar un Tipo como `!Send` Explícitamente

Desde Rust 1.0, se puede negar un auto trait con `impl !Send`:

```rust
// Nota: requiere nightly para impl !Send explícito.
// En stable, usar PhantomData<*mut ()> (ver sección anterior)

// En nightly:
#![feature(negative_impls)]

struct MiTipo {
    data: i32,
}

// Aunque i32 sea Send, este tipo NO puede cruzar threads
impl !Send for MiTipo {}
```

---

## Errores Comunes

### Error 1: Implementar `Send` para tipos que comparten estado sin sincronización

```rust
use std::cell::RefCell;

struct BadShared {
    data: RefCell<Vec<i32>>,
}

// ❌ INCORRECTO: RefCell<T> no es Sync — usar con múltiples threads
//    causaría data races en el borrow counter
// unsafe impl Sync for BadShared {}  // UB potencial

// ✅ CORRECTO: usar Mutex en lugar de RefCell para datos compartidos
use std::sync::Mutex;
struct GoodShared {
    data: Mutex<Vec<i32>>,
}
// GoodShared implementa Sync automáticamente si T: Send
```

### Error 2: Implementar `Sync` cuando el tipo muta su estado sin sincronización

```rust
struct Counter {
    count: *mut u64,  // puntero a contador global sin Mutex
}

// ❌ INCORRECTO: incrementar count desde múltiples threads es una data race
// unsafe impl Sync for Counter {}

// ✅ CORRECTO: usar operaciones atómicas
use std::sync::atomic::{AtomicU64, Ordering};
struct AtomicCounter {
    count: AtomicU64,
}
// AtomicCounter implementa Sync automáticamente
```

---

## Verificar Send/Sync en Tests

Una técnica útil es usar `static_assertions` o test de trait bounds:

```rust
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}
fn assert_send_sync<T: Send + Sync>() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_es_send_y_sync() {
        // Si este test compila, el tipo es Send+Sync.
        // Si no compila, tenemos un error de trait bounds.
        assert_send::<CHandle>();
        assert_sync::<CHandle>();
    }
}
```
