# Semana 13: Smart Pointers

## 🎯 Objetivos de Aprendizaje

Al finalizar esta semana, serás capaz de:

- Comprender qué son los smart pointers y cuándo usarlos
- Usar `Box<T>` para datos en el heap y tipos recursivos
- Implementar conteo de referencias con `Rc<T>` y `Arc<T>`
- Aplicar mutabilidad interior con `RefCell<T>` y `Mutex<T>`
- Evitar ciclos de referencia con `Weak<T>`
- Combinar smart pointers para patrones avanzados

## 📚 Contenido

### Teoría

| Archivo | Tema | Duración |
|---------|------|----------|
| [01-box.md](1-teoria/01-box.md) | Box y Heap Allocation | 30 min |
| [02-rc-arc.md](1-teoria/02-rc-arc.md) | Reference Counting | 35 min |
| [03-refcell.md](1-teoria/03-refcell.md) | RefCell y Borrowing Runtime | 30 min |
| [04-interior-mutability.md](1-teoria/04-interior-mutability.md) | Interior Mutability | 25 min |
| [05-patrones.md](1-teoria/05-patrones.md) | Patrones con Smart Pointers | 30 min |

### Práctica

| Ejercicio | Tema | Dificultad |
|-----------|------|------------|
| [practica-01-box](2-practica/practica-01-box/) | Box y tipos recursivos | ⭐⭐ |
| [practica-02-rc-arc](2-practica/practica-02-rc-arc/) | Rc y Arc | ⭐⭐⭐ |
| [practica-03-refcell](2-practica/practica-03-refcell/) | RefCell | ⭐⭐⭐ |
| [practica-04-weak](2-practica/practica-04-weak/) | Weak y ciclos | ⭐⭐⭐⭐ |

### Proyecto

| Proyecto | Descripción |
|----------|-------------|
| [proyecto-cache](3-proyecto/proyecto-cache/) | Sistema de caché con smart pointers |

## 🧠 Conceptos Clave

### ¿Qué es un Smart Pointer?

Un **smart pointer** es un tipo que actúa como un puntero pero tiene metadatos y capacidades adicionales:

```rust
// Box<T> - Datos en el heap
let b = Box::new(5);

// Rc<T> - Reference Counting (single-threaded)
let rc = Rc::new(String::from("compartido"));
let rc2 = Rc::clone(&rc);

// Arc<T> - Atomic Reference Counting (multi-threaded)
let arc = Arc::new(vec![1, 2, 3]);

// RefCell<T> - Interior mutability
let cell = RefCell::new(42);
*cell.borrow_mut() += 1;
```

### Cuándo Usar Cada Uno

| Smart Pointer | Caso de Uso |
|---------------|-------------|
| `Box<T>` | Tipos recursivos, datos grandes en heap |
| `Rc<T>` | Múltiples propietarios (single-thread) |
| `Arc<T>` | Múltiples propietarios (multi-thread) |
| `RefCell<T>` | Mutabilidad interior (single-thread) |
| `Mutex<T>` | Mutabilidad interior (multi-thread) |
| `Weak<T>` | Referencias no-propietarias, romper ciclos |

### Los Traits Deref y Drop

```rust
use std::ops::Deref;

// Deref permite usar * y auto-dereferencing
impl<T> Deref for Box<T> {
    type Target = T;
    fn deref(&self) -> &T { ... }
}

// Drop se ejecuta cuando el valor sale de scope
impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        // Libera memoria del heap
    }
}
```

## 📊 Diagrama de Smart Pointers

```
┌─────────────────────────────────────────────────────────────────┐
│                     SMART POINTERS EN RUST                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐      │
│  │  Box<T> │    │  Rc<T>  │    │ Arc<T>  │    │RefCell<T│      │
│  └────┬────┘    └────┬────┘    └────┬────┘    └────┬────┘      │
│       │              │              │              │            │
│       ▼              ▼              ▼              ▼            │
│  ┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐      │
│  │  Heap   │    │ Shared  │    │ Thread  │    │ Runtime │      │
│  │  Data   │    │  Refs   │    │  Safe   │    │ Borrow  │      │
│  └─────────┘    └─────────┘    └─────────┘    └─────────┘      │
│                                                                  │
│  Ownership:    Ownership:     Ownership:     Mutability:        │
│  Único         Compartido     Compartido     Interior           │
│  (1 owner)     (N owners)     (N owners)     (runtime check)    │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## ⚠️ Errores Comunes

### 1. Ciclos de Referencia con Rc

```rust
// ❌ CICLO: memoria nunca se libera
use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    next: Option<Rc<RefCell<Node>>>,
}

let a = Rc::new(RefCell::new(Node { next: None }));
let b = Rc::new(RefCell::new(Node { next: Some(Rc::clone(&a)) }));
a.borrow_mut().next = Some(Rc::clone(&b)); // ¡Ciclo!

// ✅ Usar Weak para romper el ciclo
use std::rc::Weak;

struct SafeNode {
    next: Option<Rc<RefCell<SafeNode>>>,
    prev: Option<Weak<RefCell<SafeNode>>>, // Weak no cuenta como owner
}
```

### 2. Panic en RefCell

```rust
// ❌ Dos borrows mutables = panic en runtime
let cell = RefCell::new(5);
let borrow1 = cell.borrow_mut();
let borrow2 = cell.borrow_mut(); // PANIC!

// ✅ Liberar el borrow antes
let cell = RefCell::new(5);
{
    let mut borrow = cell.borrow_mut();
    *borrow += 1;
} // borrow se libera aquí
let value = cell.borrow(); // OK
```

### 3. Arc sin Mutex

```rust
// ❌ Arc solo no permite mutación
let data = Arc::new(vec![1, 2, 3]);
// data.push(4); // Error: cannot borrow as mutable

// ✅ Combinar Arc con Mutex
let data = Arc::new(Mutex::new(vec![1, 2, 3]));
data.lock().unwrap().push(4); // OK
```

## 🔗 Recursos

- [Teoría completa](1-teoria/)
- [Ejercicios prácticos](2-practica/)
- [Proyecto de la semana](3-proyecto/)
- [Recursos adicionales](4-recursos/RECURSOS.md)
- [Glosario](5-glosario/GLOSARIO.md)

## 📋 Evaluación

Ver [RUBRICA_EVALUACION.md](RUBRICA_EVALUACION.md) para los criterios de evaluación.

## ⏱️ Tiempo Estimado

| Actividad | Tiempo |
|-----------|--------|
| Teoría | 2.5 horas |
| Práctica | 1 hora |
| Proyecto | 30 min |
| **Total** | **4 horas** |

---

**Semana anterior:** [Semana 12 - Closures e Iteradores](../semana-12/)  
**Siguiente semana:** [Semana 14 - Concurrencia](../semana-14/)
