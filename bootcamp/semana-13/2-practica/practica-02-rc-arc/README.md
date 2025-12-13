# Práctica 02: Rc y Arc - Reference Counting

## 🎯 Objetivos

- Compartir datos entre múltiples propietarios con `Rc<T>`
- Usar `Arc<T>` para contextos multi-thread
- Entender `strong_count` y `weak_count`

## 📋 Ejercicios

### Ejercicio 1: Rc Básico

Practica crear y clonar `Rc`:

```rust
let datos = Rc::new(vec![1, 2, 3]);
let clone1 = Rc::clone(&datos);
println!("Count: {}", Rc::strong_count(&datos));
```

### Ejercicio 2: Grafo con Nodos Compartidos

Implementa un grafo donde múltiples nodos pueden apuntar al mismo nodo:

```rust
struct Nodo {
    nombre: String,
    conexiones: Vec<Rc<Nodo>>,
}
```

### Ejercicio 3: Arc en Multi-threading

Usa `Arc` para compartir datos entre threads:

```rust
let datos = Arc::new(vec![1, 2, 3]);

for i in 0..3 {
    let datos_clone = Arc::clone(&datos);
    thread::spawn(move || {
        // Usar datos_clone
    });
}
```

## 🔧 Ejecución

```bash
# Ejecutar
cargo run -p practica-02-rc-arc

# Ejecutar tests
cargo test -p practica-02-rc-arc
```

## ✅ Tests Esperados

- `test_rc_clone_increments_count`: Clonar incrementa el contador
- `test_rc_drop_decrements_count`: Drop decrementa el contador
- `test_arc_across_threads`: Arc funciona entre threads
- `test_nodo_creation`: Nodo se crea correctamente
- `test_shared_nodo`: Múltiples nodos comparten referencia

## 💡 Pistas

1. **Rc::clone** no clona los datos, solo incrementa el contador
2. **Arc** es como Rc pero thread-safe (más lento)
3. Usa `Rc::strong_count()` para ver cuántas referencias existen

## ⚠️ Errores Comunes

```rust
// ❌ Rc no es Send - no se puede usar en threads
let rc = Rc::new(5);
thread::spawn(move || println!("{}", rc)); // Error!

// ✅ Usar Arc para threads
let arc = Arc::new(5);
thread::spawn(move || println!("{}", arc)); // OK
```

## 📚 Recursos

- [Rc en The Rust Book](https://doc.rust-lang.org/book/ch15-04-rc.html)
- [Teoría: Rc y Arc](../../1-teoria/02-rc-arc.md)
