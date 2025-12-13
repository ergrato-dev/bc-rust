# Glosario - Semana 13: Smart Pointers

## Términos Clave

### Box<T>
Smart pointer que almacena datos en el heap. Proporciona ownership único y se desaloca automáticamente cuando sale de scope.

```rust
let b = Box::new(5); // 5 está en el heap
```

### Rc<T> (Reference Counted)
Smart pointer que permite múltiples owners del mismo dato mediante conteo de referencias. Solo para uso single-threaded.

```rust
let a = Rc::new(5);
let b = Rc::clone(&a); // Incrementa el contador
```

### Arc<T> (Atomic Reference Counted)
Versión thread-safe de `Rc<T>`. Usa operaciones atómicas para el conteo de referencias.

```rust
let a = Arc::new(5);
let b = Arc::clone(&a); // Thread-safe
```

### RefCell<T>
Proporciona interior mutability con verificación de borrowing en runtime.

```rust
let cell = RefCell::new(5);
*cell.borrow_mut() = 10; // Borrow mutable en runtime
```

### Cell<T>
Interior mutability para tipos `Copy`. No tiene borrowing, solo get/set.

```rust
let cell = Cell::new(5);
cell.set(10);
```

### Weak<T>
Referencia no-owning a datos manejados por `Rc<T>`. No incrementa el strong count.

```rust
let weak = Rc::downgrade(&rc);
if let Some(val) = weak.upgrade() { ... }
```

### Interior Mutability
Patrón que permite mutar datos incluso con referencias inmutables, moviendo la verificación al runtime.

### Strong Count
Número de referencias `Rc<T>` activas. El dato se libera cuando llega a 0.

### Weak Count
Número de referencias `Weak<T>` activas. No previene la liberación del dato.

### Deref Coercion
Conversión automática de referencias a smart pointers en referencias al tipo interno.

```rust
let b: Box<String> = Box::new("hello".to_string());
let s: &str = &b; // Deref coercion
```

### Drop Trait
Define el código a ejecutar cuando un valor sale de scope. Los smart pointers implementan `Drop` para liberar recursos.

### Ciclo de Referencias
Situación donde dos o más `Rc<T>` se referencian mutuamente, causando memory leak.

### LRU (Least Recently Used)
Algoritmo de cache que elimina los elementos menos recientemente usados cuando se alcanza la capacidad.

## Comparación Rápida

| Tipo | Thread-Safe | Multiple Owners | Mutability |
|------|-------------|-----------------|------------|
| `Box<T>` | ✅ | ❌ | Normal |
| `Rc<T>` | ❌ | ✅ | Inmutable |
| `Arc<T>` | ✅ | ✅ | Inmutable |
| `Cell<T>` | ❌ | ❌ | Interior (Copy) |
| `RefCell<T>` | ❌ | ❌ | Interior (runtime) |
| `Mutex<T>` | ✅ | - | Interior (blocking) |
