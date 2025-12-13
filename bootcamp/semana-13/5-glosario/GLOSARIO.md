# 📖 Glosario - Semana 13: Smart Pointers

## A

### `Arc<T>` (Atomic Reference Counted)
Versión thread-safe de `Rc<T>`. Permite ownership compartido entre múltiples threads usando contadores atómicos.
```rust
use std::sync::Arc;
let data = Arc::new(42);
let clone = Arc::clone(&data);
```

### Allocator
Componente del sistema que gestiona la asignación y liberación de memoria en el heap.

---

## B

### `Box<T>`
Smart pointer que almacena datos en el heap en lugar del stack. Proporciona ownership único.
```rust
let b = Box::new(5);  // 5 está en el heap
```

### Borrow Checker
Sistema del compilador que verifica reglas de borrowing en tiempo de compilación.

---

## C

### `Cell<T>`
Tipo que permite interior mutability para tipos que implementan `Copy`. No requiere referencias.
```rust
use std::cell::Cell;
let c = Cell::new(5);
c.set(10);
```

### Ciclo de Referencias
Situación donde dos o más objetos se referencian mutuamente creando un ciclo que previene la liberación de memoria.
```rust
// A → B → A (ciclo, memory leak)
```

### Clone
Duplicar un valor. En `Rc`/`Arc`, incrementa el contador de referencias.

---

## D

### Deref Coercion
Conversión automática de `&T` donde `T` implementa `Deref<Target=U>` a `&U`.
```rust
fn takes_str(s: &str) {}
let boxed = Box::new(String::from("hello"));
takes_str(&boxed);  // Deref coercion
```

### `Deref` Trait
Trait que permite a un tipo comportarse como una referencia.
```rust
impl<T> Deref for Box<T> {
    type Target = T;
    fn deref(&self) -> &T { /* ... */ }
}
```

### `Drop` Trait
Trait que define qué hacer cuando un valor sale del scope.
```rust
impl Drop for MiTipo {
    fn drop(&mut self) {
        println!("Liberando recursos");
    }
}
```

### Dyn (Dynamic Dispatch)
Despacho dinámico de métodos en tiempo de ejecución usando vtables.
```rust
let animal: Box<dyn Animal> = Box::new(Perro);
```

---

## H

### Heap
Región de memoria para almacenamiento dinámico. Más lento que stack pero de tamaño flexible.

---

## I

### Interior Mutability
Patrón que permite mutar datos incluso cuando hay referencias inmutables. Se verifica en runtime.
```rust
let x = RefCell::new(42);
*x.borrow_mut() = 100;  // Muta a través de referencia inmutable
```

### Indirection
Acceder a datos a través de un puntero en lugar de directamente.

---

## M

### `Mutex<T>`
Mutual Exclusion. Permite acceso exclusivo a datos compartidos entre threads.
```rust
use std::sync::Mutex;
let m = Mutex::new(5);
let mut num = m.lock().unwrap();
```

### Memory Leak
Memoria que nunca se libera porque las referencias forman un ciclo.

---

## O

### Ownership Compartido
Múltiples variables pueden ser dueñas del mismo dato usando `Rc` o `Arc`.

---

## P

### Panic en Borrow
`RefCell` causa panic si se viola las reglas de borrowing en runtime.
```rust
let r = RefCell::new(5);
let a = r.borrow();
let b = r.borrow_mut();  // PANIC! Ya hay borrow inmutable
```

### Puntero
Variable que contiene una dirección de memoria.

---

## R

### `Rc<T>` (Reference Counted)
Smart pointer que permite ownership compartido mediante conteo de referencias.
```rust
use std::rc::Rc;
let a = Rc::new(5);
let b = Rc::clone(&a);  // count = 2
```

### `RefCell<T>`
Tipo que permite interior mutability verificada en runtime.
```rust
use std::cell::RefCell;
let r = RefCell::new(5);
*r.borrow_mut() = 10;
```

### Reference Counting
Técnica de gestión de memoria que cuenta cuántas referencias apuntan a un dato.

### `Ref<T>` / `RefMut<T>`
Tipos wrapper retornados por `RefCell::borrow()` y `RefCell::borrow_mut()`.

---

## S

### Smart Pointer
Estructura que actúa como puntero pero con capacidades adicionales (memoria automática, conteo de refs, etc).

### Stack
Región de memoria para variables locales. Rápido pero de tamaño fijo.

### Strong Count
Número de referencias `Rc`/`Arc` que mantienen vivo el dato.
```rust
Rc::strong_count(&mi_rc)  // Número de Rc activos
```

---

## T

### Trait Object
Valor que contiene cualquier tipo que implementa un trait específico.
```rust
let drawable: &dyn Draw = &circle;
```

---

## U

### `upgrade()`
Método de `Weak<T>` para obtener `Option<Rc<T>>`. Retorna `None` si el dato fue liberado.
```rust
let weak: Weak<i32> = Rc::downgrade(&rc);
if let Some(strong) = weak.upgrade() {
    println!("Dato aún existe: {}", strong);
}
```

---

## V

### Vtable
Tabla virtual de funciones usada para dispatch dinámico.

---

## W

### `Weak<T>`
Referencia débil que no incrementa el strong count. No previene la liberación.
```rust
let weak = Rc::downgrade(&rc);  // No incrementa count
```

### Weak Count
Número de referencias `Weak` a un dato.
```rust
Rc::weak_count(&mi_rc)
```

---

## Símbolos y Operadores

### `*` (Dereference)
Operador para acceder al valor apuntado por un puntero o smart pointer.
```rust
let b = Box::new(5);
println!("{}", *b);  // 5
```

### `&` y `&mut`
Operadores de referencia inmutable y mutable.

---

## Patrones de Memoria

| Patrón | Cuándo Usar |
|--------|-------------|
| `Box<T>` | Dato grande en heap, ownership único |
| `Rc<T>` | Ownership compartido, single thread |
| `Arc<T>` | Ownership compartido, multi-thread |
| `RefCell<T>` | Interior mutability, single thread |
| `Mutex<T>` | Interior mutability, multi-thread |
| `Weak<T>` | Romper ciclos de referencias |
| `Cell<T>` | Interior mutability para tipos Copy |

---

## Reglas de Combinación

```
Single-Thread              Multi-Thread
─────────────              ────────────
Rc<T>                      Arc<T>
Rc<RefCell<T>>             Arc<Mutex<T>>
Weak<RefCell<T>>           Weak<Mutex<T>> (con Arc)
```
