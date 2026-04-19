# Práctica 04: Weak - Rompiendo Ciclos

## 🎯 Objetivos

- Usar `Weak<T>` para referencias no-owning
- Romper ciclos de referencia
- Implementar estructuras padre-hijo

## 📋 Ejercicios

### Ejercicio 1: Weak Básico

Practica `Rc::downgrade()` y `Weak::upgrade()`:

```rust
let fuerte = Rc::new(42);
let debil = Rc::downgrade(&fuerte);

// upgrade() retorna Option<Rc<T>>
if let Some(valor) = debil.upgrade() {
    println!("Valor: {}", valor);
}
```

### Ejercicio 2: Árbol con Parent

Implementa un árbol donde los hijos tienen referencia al padre:

```rust
struct Nodo {
    valor: i32,
    parent: RefCell<Weak<Nodo>>,      // ← Weak!
    children: RefCell<Vec<Rc<Nodo>>>, // ← Strong
}
```

### Ejercicio 3: Observer Pattern

Implementa el patrón Observer con referencias débiles:

```rust
struct Publicador {
    observadores: RefCell<Vec<Weak<Observador>>>,
}
```

## 🔧 Ejecución

```bash
# Ejecutar
cargo run -p practica-04-weak

# Ejecutar tests
cargo test -p practica-04-weak
```

## 💡 Strong vs Weak

| Tipo   | Cuenta para Drop | upgrade() |
|--------|------------------|-----------|
| Strong | ✅ Sí            | N/A       |
| Weak   | ❌ No            | → Option  |

## ⚠️ Cuándo usar Weak

- ✅ Referencias al padre (evita ciclos)
- ✅ Caches que no deben prevenir cleanup
- ✅ Observadores que pueden desaparecer

## 📚 Recursos

- [Weak en The Rust Book](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)
- [Teoría: Patrones](../../1-teoria/05-patrones.md)
