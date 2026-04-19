# Patrones con Smart Pointers

## 🎯 Objetivos

- Usar `Weak<T>` para romper ciclos de referencia
- Implementar estructuras con referencias padre-hijo
- Combinar smart pointers efectivamente
- Aplicar patrones comunes en Rust

![Patrones con Smart Pointers](../0-assets/05-patrones.svg)

## 📚 El Problema de los Ciclos

### Memory Leak con Rc

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
    
    // Crear ciclo: a -> b -> a
    a.borrow_mut().next = Some(Rc::clone(&b));
    
    println!("a strong: {}", Rc::strong_count(&a)); // 2
    println!("b strong: {}", Rc::strong_count(&b)); // 2
    
    // Al salir de scope:
    // - drop(b): b strong = 1 (a todavía tiene referencia)
    // - drop(a): a strong = 1 (b todavía tiene referencia)
    // ¡Ninguno llega a 0! MEMORY LEAK
}
```

## 🔧 Weak<T> - La Solución

`Weak<T>` es una referencia que no cuenta como owner:

```
┌─────────────────────────────────────────────────────────────┐
│                    Rc vs Weak                                │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   Rc (strong reference)        Weak (weak reference)        │
│   ┌─────────────────┐         ┌─────────────────┐          │
│   │ Cuenta como     │         │ NO cuenta como  │          │
│   │ propietario     │         │ propietario     │          │
│   │ Mantiene vivo   │         │ No mantiene     │          │
│   │ el dato         │         │ vivo el dato    │          │
│   └─────────────────┘         └─────────────────┘          │
│                                                             │
│   strong_count > 0             Puede ser inválido          │
│   = dato existe                upgrade() -> Option<Rc<T>>  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Uso de Weak

```rust
use std::rc::{Rc, Weak};

fn main() {
    let strong = Rc::new(5);
    println!("Strong count: {}", Rc::strong_count(&strong)); // 1
    
    // Crear weak reference
    let weak: Weak<i32> = Rc::downgrade(&strong);
    println!("Weak count: {}", Rc::weak_count(&strong)); // 1
    println!("Strong count: {}", Rc::strong_count(&strong)); // 1 (no cambió)
    
    // Acceder a través de weak
    if let Some(rc) = weak.upgrade() {
        println!("Valor: {}", rc);
    }
    
    // Drop el strong
    drop(strong);
    
    // Weak ya no puede acceder al dato
    assert!(weak.upgrade().is_none());
    println!("Weak upgrade: {:?}", weak.upgrade()); // None
}
```

## 📖 Árbol con Referencias Padre

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct TreeNode {
    value: i32,
    parent: RefCell<Weak<TreeNode>>,      // Weak para evitar ciclo
    children: RefCell<Vec<Rc<TreeNode>>>, // Strong para hijos
}

impl TreeNode {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(TreeNode {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }
    
    fn add_child(parent: &Rc<Self>, child: &Rc<Self>) {
        // Establecer padre (weak)
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        // Agregar hijo (strong)
        parent.children.borrow_mut().push(Rc::clone(child));
    }
}

fn main() {
    let root = TreeNode::new(1);
    let child1 = TreeNode::new(2);
    let child2 = TreeNode::new(3);
    
    TreeNode::add_child(&root, &child1);
    TreeNode::add_child(&root, &child2);
    
    // Verificar estructura
    println!("Root children: {}", root.children.borrow().len());
    
    // Acceder al padre desde el hijo
    if let Some(parent) = child1.parent.borrow().upgrade() {
        println!("Child1's parent value: {}", parent.value);
    }
    
    // Counts
    println!("Root strong: {}", Rc::strong_count(&root));   // 1
    println!("Child1 strong: {}", Rc::strong_count(&child1)); // 2
}
```

## 🔄 Lista Doblemente Enlazada

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

type Link<T> = Option<Rc<RefCell<DNode<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<DNode<T>>>>;

#[derive(Debug)]
struct DNode<T> {
    value: T,
    next: Link<T>,     // Strong hacia adelante
    prev: WeakLink<T>, // Weak hacia atrás
}

struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: WeakLink<T>,
}

impl<T> DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }
    
    fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(DNode {
            value,
            next: None,
            prev: self.tail.clone(),
        }));
        
        match self.tail.take() {
            Some(old_tail) => {
                if let Some(old_tail_rc) = old_tail.upgrade() {
                    old_tail_rc.borrow_mut().next = Some(Rc::clone(&new_node));
                }
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
            }
        }
        
        self.tail = Some(Rc::downgrade(&new_node));
    }
}
```

## 📊 Patrones Comunes

### 1. Observer Pattern

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

trait Observer {
    fn update(&self, value: i32);
}

struct Subject {
    observers: RefCell<Vec<Weak<dyn Observer>>>,
    value: RefCell<i32>,
}

impl Subject {
    fn new() -> Self {
        Subject {
            observers: RefCell::new(vec![]),
            value: RefCell::new(0),
        }
    }
    
    fn attach(&self, observer: &Rc<dyn Observer>) {
        self.observers.borrow_mut().push(Rc::downgrade(observer));
    }
    
    fn set_value(&self, value: i32) {
        *self.value.borrow_mut() = value;
        self.notify();
    }
    
    fn notify(&self) {
        let value = *self.value.borrow();
        self.observers.borrow_mut().retain(|weak| {
            if let Some(observer) = weak.upgrade() {
                observer.update(value);
                true
            } else {
                false // Remove dead observers
            }
        });
    }
}
```

### 2. Cache con Weak References

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;

struct Cache<K, V> {
    data: RefCell<HashMap<K, Weak<V>>>,
}

impl<K: std::hash::Hash + Eq, V> Cache<K, V> {
    fn new() -> Self {
        Cache {
            data: RefCell::new(HashMap::new()),
        }
    }
    
    fn get(&self, key: &K) -> Option<Rc<V>> {
        self.data.borrow().get(key)?.upgrade()
    }
    
    fn insert(&self, key: K, value: &Rc<V>) {
        self.data.borrow_mut().insert(key, Rc::downgrade(value));
    }
    
    fn cleanup(&self) {
        self.data.borrow_mut().retain(|_, weak| weak.upgrade().is_some());
    }
}
```

### 3. Shared State con Interior Mutability

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct SharedState {
    counter: Rc<RefCell<i32>>,
}

impl SharedState {
    fn new() -> Self {
        SharedState {
            counter: Rc::new(RefCell::new(0)),
        }
    }
    
    fn clone_counter(&self) -> Rc<RefCell<i32>> {
        Rc::clone(&self.counter)
    }
    
    fn increment(&self) {
        *self.counter.borrow_mut() += 1;
    }
    
    fn get(&self) -> i32 {
        *self.counter.borrow()
    }
}

fn main() {
    let state = SharedState::new();
    let counter_ref = state.clone_counter();
    
    state.increment();
    *counter_ref.borrow_mut() += 10;
    
    println!("Counter: {}", state.get()); // 11
}
```

## 📊 Diagrama Visual

![Patrones Smart Pointers](../0-assets/05-patrones.svg)

## 🎯 Resumen de Combinaciones

| Patrón | Tipos | Uso |
|--------|-------|-----|
| Ownership único en heap | `Box<T>` | Tipos recursivos |
| Sharing inmutable | `Rc<T>` | Múltiples lectores |
| Sharing mutable | `Rc<RefCell<T>>` | Mutación compartida |
| Referencia no-owner | `Weak<T>` | Evitar ciclos |
| Thread-safe sharing | `Arc<Mutex<T>>` | Multi-threading |

## ⚠️ Checklist Anti-Ciclos

- [ ] ¿Hay referencias bidireccionales?
- [ ] ¿El padre tiene `Rc` a hijos?
- [ ] ¿Los hijos tienen `Weak` al padre?
- [ ] ¿Se usa `upgrade()` y se maneja `None`?
- [ ] ¿Se limpian las `Weak` references inválidas?

## 🔗 Recursos

- [Anterior: Interior Mutability](04-interior-mutability.md)
- [Práctica: Ejercicios](../2-practica/)
- [Proyecto: Sistema de Caché](../3-proyecto/)
