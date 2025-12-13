# Proyecto Final: Cache LRU con Smart Pointers

## 🎯 Objetivo

Implementar un **Cache LRU (Least Recently Used)** usando smart pointers:

- `Rc<RefCell<T>>` para nodos mutables compartidos
- `Weak<T>` para referencias bidireccionales (prev)
- `HashMap` para acceso O(1)

## 📋 Descripción

Un cache LRU mantiene los elementos más recientemente usados y elimina automáticamente los menos usados cuando se alcanza la capacidad.

```
Operaciones:
- get(key)  → O(1) - Retorna valor y mueve al frente
- put(k, v) → O(1) - Inserta/actualiza y mueve al frente

Estructura:
HEAD ←→ Nodo ←→ Nodo ←→ Nodo ←→ TAIL
(MRU)                           (LRU)
```

## 🏗️ Estructura

```rust
struct Nodo<K, V> {
    key: K,
    value: V,
    prev: RefCell<Weak<Nodo<K, V>>>,      // ← Weak!
    next: RefCell<Option<Rc<Nodo<K, V>>>>, // ← Strong
}

struct LruCache<K, V> {
    capacidad: usize,
    mapa: RefCell<HashMap<K, Rc<Nodo<K, V>>>>,
    head: RefCell<Option<Rc<Nodo<K, V>>>>,
    tail: RefCell<Weak<Nodo<K, V>>>,
}
```

## 🔧 Ejecución

```bash
# Ejecutar
cargo run -p proyecto-cache

# Ejecutar tests
cargo test -p proyecto-cache
```

## 📊 Ejemplo de Uso

```rust
let cache = LruCache::new(3);

cache.put("a", 1);
cache.put("b", 2);
cache.put("c", 3);
// Cache: [c, b, a]

cache.get(&"a");     // Mueve 'a' al frente
// Cache: [a, c, b]

cache.put("d", 4);   // Elimina 'b' (LRU)
// Cache: [d, a, c]
```

## ✅ Criterios de Evaluación

| Criterio | Puntos |
|----------|--------|
| Compila sin warnings | 20% |
| Tests pasan | 30% |
| Implementación correcta de LRU | 30% |
| Uso correcto de Weak | 20% |

## 💡 Conceptos Aplicados

- **Rc<RefCell<T>>**: Nodos mutables con múltiples owners
- **Weak<T>**: Referencias prev sin ciclos
- **Interior Mutability**: Modificar lista con &self
- **HashMap**: Acceso O(1) a nodos

## 🎓 Extensiones Opcionales

1. Agregar TTL (Time To Live) a las entradas
2. Implementar `Iterator` para el cache
3. Versión thread-safe con `Arc<Mutex<T>>`

## 📚 Recursos

- [Teoría: Patrones](../../1-teoria/05-patrones.md)
- [LRU Cache - LeetCode](https://leetcode.com/problems/lru-cache/)
