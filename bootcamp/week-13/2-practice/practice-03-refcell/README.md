# Práctica 03: RefCell - Interior Mutability

## 🎯 Objetivos

- Usar `RefCell<T>` para mutabilidad interior
- Combinar `Rc<RefCell<T>>` para datos mutables compartidos
- Manejar errores de borrowing en runtime

## 📋 Ejercicios

### Ejercicio 1: RefCell Básico

Practica `borrow()` y `borrow_mut()`:

```rust
let datos = RefCell::new(vec![1, 2, 3]);

// Lectura
let r = datos.borrow();

// Escritura
let mut r = datos.borrow_mut();
r.push(4);
```

### Ejercicio 2: Contador Compartido

Implementa un contador que múltiples referencias pueden modificar:

```rust
let contador = Contador::new("Principal");
let ref1 = Rc::clone(&contador);

Contador::incrementar(&ref1);
Contador::incrementar(&contador);
```

### Ejercicio 3: Cache con Interior Mutability

Implementa una calculadora con cache interno:

```rust
struct Calculadora {
    cache: RefCell<Option<i32>>,
    base: i32,
}

// Método &self que modifica cache internamente
fn calcular_costoso(&self) -> i32 { ... }
```

## 🔧 Ejecución

```bash
# Ejecutar
cargo run -p practica-03-refcell

# Ejecutar tests
cargo test -p practica-03-refcell
```

## ⚠️ Cuidado con Panic

RefCell puede causar panic en runtime:

```rust
let cell = RefCell::new(5);
let r1 = cell.borrow();
let r2 = cell.borrow_mut(); // PANIC!
```

Usa `try_borrow_mut()` para evitar panic:

```rust
match cell.try_borrow_mut() {
    Ok(mut r) => *r = 10,
    Err(e) => println!("Error: {}", e),
}
```

## 💡 Pistas

1. **borrow()** retorna `Ref<T>` (inmutable)
2. **borrow_mut()** retorna `RefMut<T>` (mutable)
3. Los borrows se liberan cuando salen de scope

## 📚 Recursos

- [RefCell en The Rust Book](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)
- [Teoría: RefCell](../../1-teoria/03-refcell.md)
