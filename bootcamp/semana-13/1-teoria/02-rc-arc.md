# Rc<T> y Arc<T> - Reference Counting

## 🎯 Objetivos

- Entender el conteo de referencias
- Usar `Rc<T>` para ownership compartido (single-threaded)
- Usar `Arc<T>` para ownership compartido (multi-threaded)
- Conocer strong_count y weak_count

## 📚 Conceptos

### ¿Por Qué Reference Counting?

En Rust, cada valor tiene un único propietario. Pero a veces necesitamos múltiples propietarios:

```rust
// ❌ No compila: solo puede haber un owner
let s = String::from("hello");
let a = s;
let b = s;  // Error: value moved

// ✅ Con Rc: múltiples owners
use std::rc::Rc;
let s = Rc::new(String::from("hello"));
let a = Rc::clone(&s);
let b = Rc::clone(&s);
// s, a y b comparten el mismo String
```

### Cómo Funciona Rc

```
┌─────────────────────────────────────────────────────────────┐
│                    REFERENCE COUNTING                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   Variables              Rc Metadata           Datos        │
│   ┌───┐                  ┌──────────┐         ┌──────────┐ │
│   │ a ├─────────────────►│ strong: 3│────────►│ "hello"  │ │
│   └───┘                  │ weak: 0  │         └──────────┘ │
│   ┌───┐                  └──────────┘                      │
│   │ b ├──────────────────────┘ ▲                           │
│   └───┘                        │                           │
│   ┌───┐                        │                           │
│   │ c ├────────────────────────┘                           │
│   └───┘                                                    │
│                                                             │
│   Cuando strong_count llega a 0, los datos se liberan      │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## 🔧 Uso de Rc<T>

```rust
use std::rc::Rc;

fn main() {
    // Crear un Rc
    let original = Rc::new(vec![1, 2, 3]);
    println!("Creado. Count: {}", Rc::strong_count(&original));
    
    // Clonar incrementa el contador (no clona los datos)
    let clone1 = Rc::clone(&original);
    println!("Clone 1. Count: {}", Rc::strong_count(&original));
    
    let clone2 = Rc::clone(&original);
    println!("Clone 2. Count: {}", Rc::strong_count(&original));
    
    // Usar los datos
    println!("Datos: {:?}", original);
    println!("Mismo dato: {:?}", clone1);
    
    // Al salir de scope, el contador decrementa
    drop(clone2);
    println!("Después de drop. Count: {}", Rc::strong_count(&original));
}

// Output:
// Creado. Count: 1
// Clone 1. Count: 2
// Clone 2. Count: 3
// Datos: [1, 2, 3]
// Mismo dato: [1, 2, 3]
// Después de drop. Count: 2
```

### Rc::clone vs .clone()

```rust
use std::rc::Rc;

let data = Rc::new(String::from("hello"));

// ✅ Preferido: clarifica que solo incrementa el contador
let a = Rc::clone(&data);

// ⚠️ Funciona pero confuso: parece que clona los datos
let b = data.clone();

// Ambos hacen lo mismo, pero Rc::clone es más claro
```

## 📖 Ejemplo: Grafo con Nodos Compartidos

```rust
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<Node>>,
}

fn main() {
    // Nodo compartido por múltiples padres
    let shared = Rc::new(Node {
        value: 3,
        children: vec![],
    });
    
    let parent1 = Rc::new(Node {
        value: 1,
        children: vec![Rc::clone(&shared)],
    });
    
    let parent2 = Rc::new(Node {
        value: 2,
        children: vec![Rc::clone(&shared)],
    });
    
    println!("shared count: {}", Rc::strong_count(&shared)); // 3
    println!("Parent 1: {:?}", parent1);
    println!("Parent 2: {:?}", parent2);
}
```

## 🔒 Arc<T> - Atomic Reference Counting

`Arc` es como `Rc` pero thread-safe (usa operaciones atómicas):

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];
    
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("Thread {}: {:?}", i, data_clone);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final count: {}", Arc::strong_count(&data));
}
```

## 📊 Rc vs Arc

| Característica | Rc<T> | Arc<T> |
|----------------|-------|--------|
| Thread-safe | ❌ No | ✅ Sí |
| Performance | Más rápido | Más lento (atomic ops) |
| Traits | !Send, !Sync | Send + Sync |
| Uso | Single-thread | Multi-thread |

```rust
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

// ❌ Rc no es Send
let rc = Rc::new(5);
// thread::spawn(move || println!("{}", rc));  // Error!

// ✅ Arc es Send
let arc = Arc::new(5);
thread::spawn(move || println!("{}", arc));
```

## ⚠️ Limitación: Rc/Arc Son Inmutables

```rust
use std::rc::Rc;

let data = Rc::new(5);
// *data = 10;  // ❌ Error: cannot assign to data in an `Rc`

// Para mutabilidad, combinar con RefCell (siguiente tema)
use std::cell::RefCell;

let data = Rc::new(RefCell::new(5));
*data.borrow_mut() = 10;  // ✅ OK
```

## 🔄 Ciclos de Referencia

⚠️ **PELIGRO**: Rc puede crear memory leaks con ciclos:

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

fn main() {
    let a = Rc::new(RefCell::new(Node { value: 1, next: None }));
    let b = Rc::new(RefCell::new(Node { value: 2, next: Some(Rc::clone(&a)) }));
    
    // ¡Crear ciclo!
    a.borrow_mut().next = Some(Rc::clone(&b));
    
    // a -> b -> a -> b -> ... (ciclo infinito)
    // strong_count nunca llega a 0
    // MEMORY LEAK!
    
    println!("a count: {}", Rc::strong_count(&a)); // 2
    println!("b count: {}", Rc::strong_count(&b)); // 2
}
```

**Solución**: Usar `Weak<T>` (se verá en tema posterior)

## 📊 Diagrama Visual

![Rc y Arc](../0-assets/02-rc-arc.svg)

## 🎯 Resumen

| Método | Descripción |
|--------|-------------|
| `Rc::new(v)` | Crear nuevo Rc |
| `Rc::clone(&rc)` | Incrementar contador |
| `Rc::strong_count(&rc)` | Obtener contador |
| `Rc::weak_count(&rc)` | Obtener weak refs |
| `Arc::new(v)` | Crear Arc thread-safe |

## 🔗 Siguiente

[03 - RefCell y Borrowing Runtime](03-refcell.md)
