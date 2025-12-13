# RefCell<T> y Borrowing en Runtime

## 🎯 Objetivos

- Entender el borrowing en tiempo de ejecución
- Usar `borrow()` y `borrow_mut()` correctamente
- Combinar `Rc<RefCell<T>>` para mutabilidad compartida
- Manejar errores de borrowing

## 📚 Conceptos

### El Problema

Rust verifica borrowing en tiempo de compilación, pero a veces necesitamos más flexibilidad:

```rust
use std::rc::Rc;

let data = Rc::new(5);
// *data = 10;  // ❌ Error: Rc no permite mutación

// ¿Cómo mutamos datos compartidos?
```

### RefCell: Borrowing en Runtime

`RefCell<T>` mueve las reglas de borrowing a tiempo de ejecución:

```
┌─────────────────────────────────────────────────────────────┐
│                    BORROWING RULES                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   COMPILE TIME (normal)         RUNTIME (RefCell)           │
│   ┌─────────────────┐          ┌─────────────────┐         │
│   │ &T  - N refs    │          │ borrow() - N    │         │
│   │ &mut T - 1 ref  │    vs    │ borrow_mut() - 1│         │
│   │ Error = compile │          │ Error = panic!  │         │
│   └─────────────────┘          └─────────────────┘         │
│                                                             │
│   let mut x = 5;               let x = RefCell::new(5);    │
│   let r = &mut x;              let r = x.borrow_mut();     │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## 🔧 Uso Básico

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    
    // Borrow inmutable
    {
        let r = data.borrow();
        println!("Valor: {}", *r);
    } // r se libera aquí
    
    // Borrow mutable
    {
        let mut r = data.borrow_mut();
        *r += 10;
    } // r se libera aquí
    
    println!("Nuevo valor: {}", data.borrow());
}
```

### Métodos de RefCell

| Método | Retorna | Descripción |
|--------|---------|-------------|
| `borrow()` | `Ref<T>` | Referencia inmutable |
| `borrow_mut()` | `RefMut<T>` | Referencia mutable |
| `try_borrow()` | `Result<Ref<T>, BorrowError>` | Sin panic |
| `try_borrow_mut()` | `Result<RefMut<T>, BorrowMutError>` | Sin panic |

## ⚠️ Panic en Runtime

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    
    let r1 = data.borrow();
    let r2 = data.borrow();  // ✅ OK: múltiples borrows inmutables
    
    // ❌ PANIC: ya hay borrows inmutables activos
    // let r3 = data.borrow_mut();
    
    drop(r1);
    drop(r2);
    
    let r3 = data.borrow_mut();  // ✅ OK: no hay otros borrows
}
```

### Usar try_borrow para Evitar Panic

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    let _r1 = data.borrow();
    
    // En lugar de panic, obtener Result
    match data.try_borrow_mut() {
        Ok(mut r) => *r += 1,
        Err(e) => println!("Error: {}", e),
    }
}
```

## 📖 Patrón: Rc<RefCell<T>>

Combinar `Rc` y `RefCell` para datos mutables compartidos:

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Counter {
    value: i32,
}

fn main() {
    // Crear dato compartido y mutable
    let counter = Rc::new(RefCell::new(Counter { value: 0 }));
    
    // Múltiples referencias al mismo contador
    let c1 = Rc::clone(&counter);
    let c2 = Rc::clone(&counter);
    
    // Cada referencia puede mutar
    c1.borrow_mut().value += 10;
    c2.borrow_mut().value += 20;
    counter.borrow_mut().value += 5;
    
    println!("Final: {:?}", counter.borrow()); // Counter { value: 35 }
}
```

## 🔄 Ejemplo: Lista con Nodos Mutables

```rust
use std::rc::Rc;
use std::cell::RefCell;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

#[derive(Debug)]
struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }
    
    fn push(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: self.head.take(),
        }));
        self.head = Some(new_node);
    }
    
    fn modify_head<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        if let Some(ref node) = self.head {
            f(&mut node.borrow_mut().value);
        }
    }
}

fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    
    // Modificar el primer elemento
    list.modify_head(|v| *v *= 10);
    
    println!("{:?}", list); // head: 30 -> 2 -> 1
}
```

## 📊 Comparación de Patterns

```rust
use std::cell::RefCell;
use std::rc::Rc;

// Solo lectura compartida
let shared_read: Rc<String> = Rc::new(String::from("hello"));

// Lectura/escritura con único owner
let single_mut: RefCell<String> = RefCell::new(String::from("hello"));

// Lectura/escritura compartida (single-thread)
let shared_mut: Rc<RefCell<String>> = Rc::new(RefCell::new(String::from("hello")));
```

## 🎯 Cuándo Usar RefCell

| Situación | Solución |
|-----------|----------|
| Mutar campo en struct inmutable | RefCell en el campo |
| Mock objects en tests | RefCell para tracking |
| Datos compartidos mutables | Rc<RefCell<T>> |
| Caches internos | RefCell |

### Ejemplo: Caché Interno

```rust
use std::cell::RefCell;
use std::collections::HashMap;

struct ExpensiveComputer {
    cache: RefCell<HashMap<i32, i32>>,
}

impl ExpensiveComputer {
    fn new() -> Self {
        ExpensiveComputer {
            cache: RefCell::new(HashMap::new()),
        }
    }
    
    // Método &self que modifica el cache interno
    fn compute(&self, input: i32) -> i32 {
        if let Some(&result) = self.cache.borrow().get(&input) {
            return result;
        }
        
        // Cálculo costoso
        let result = input * input;
        
        // Mutar cache aunque self es inmutable
        self.cache.borrow_mut().insert(input, result);
        result
    }
}

fn main() {
    let computer = ExpensiveComputer::new();
    
    println!("{}", computer.compute(5));  // Calcula
    println!("{}", computer.compute(5));  // Usa cache
    println!("{}", computer.compute(10)); // Calcula
}
```

## 📊 Diagrama Visual

![RefCell](../0-assets/03-refcell.svg)

## 🎯 Resumen

| Aspecto | RefCell<T> |
|---------|------------|
| Verificación | Runtime |
| Error | Panic (o Result con try_*) |
| Thread-safe | ❌ No |
| Combinación común | Rc<RefCell<T>> |
| Costo | Pequeño overhead de tracking |

## 🔗 Siguiente

[04 - Interior Mutability](04-interior-mutability.md)
