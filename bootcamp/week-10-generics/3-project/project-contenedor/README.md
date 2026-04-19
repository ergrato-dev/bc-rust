# Proyecto: Biblioteca de Contenedores Genéricos

## 📋 Descripción

En este proyecto crearás una biblioteca de contenedores genéricos reutilizables,
aplicando todos los conceptos de genéricos aprendidos en la semana.

Implementarás cuatro estructuras de datos fundamentales:

1. **Cola (Queue)** - FIFO (First In, First Out)
2. **Deque** - Cola de doble extremo
3. **Contenedor con Límite** - Capacidad máxima fija
4. **Caché LRU simplificado** - Least Recently Used

## 🎯 Objetivos de Aprendizaje

- Diseñar APIs genéricas idiomáticas
- Aplicar trait bounds apropiados
- Usar const generics para tamaños fijos
- Implementar traits estándar (Default, Debug, Clone, etc.)
- Escribir tests exhaustivos

## 📁 Estructura del Proyecto

```
proyecto-contenedor/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs          # Demo de uso
│   ├── lib.rs           # Módulo raíz
│   ├── cola.rs          # Cola genérica
│   ├── deque.rs         # Deque genérica
│   ├── limitado.rs      # Contenedor con límite
│   └── cache.rs         # Caché LRU
└── tests/
    └── integration.rs   # Tests de integración
```

## 📝 Requisitos por Componente

### 1. Cola<T> - FIFO Queue

```rust
pub struct Cola<T> { ... }

impl<T> Cola<T> {
    pub fn new() -> Self;
    pub fn encolar(&mut self, valor: T);
    pub fn desencolar(&mut self) -> Option<T>;
    pub fn frente(&self) -> Option<&T>;
    pub fn len(&self) -> usize;
    pub fn esta_vacia(&self) -> bool;
}
```

**Traits a implementar**: `Default`, `Debug` (donde T: Debug)

---

### 2. Deque<T> - Double-Ended Queue

```rust
pub struct Deque<T> { ... }

impl<T> Deque<T> {
    pub fn new() -> Self;
    pub fn push_frente(&mut self, valor: T);
    pub fn push_atras(&mut self, valor: T);
    pub fn pop_frente(&mut self) -> Option<T>;
    pub fn pop_atras(&mut self) -> Option<T>;
    pub fn frente(&self) -> Option<&T>;
    pub fn atras(&self) -> Option<&T>;
    pub fn len(&self) -> usize;
}
```

**Traits a implementar**: `Default`, `Debug` (donde T: Debug)

---

### 3. Limitado<T, N> - Contenedor con Capacidad Fija

```rust
pub struct Limitado<T, const N: usize> { ... }

impl<T, const N: usize> Limitado<T, N> {
    pub fn new() -> Self;
    pub fn insertar(&mut self, valor: T) -> Result<(), T>;
    pub fn remover(&mut self) -> Option<T>;
    pub fn len(&self) -> usize;
    pub fn capacidad(&self) -> usize;
    pub fn esta_lleno(&self) -> bool;
}
```

**Nota**: `insertar` devuelve `Err(valor)` si está lleno (devuelve el valor rechazado).

---

### 4. Cache<K, V> - Caché LRU Simplificado

```rust
pub struct Cache<K, V> { ... }

impl<K: Eq + Hash + Clone, V> Cache<K, V> {
    pub fn new(capacidad: usize) -> Self;
    pub fn insertar(&mut self, clave: K, valor: V);
    pub fn obtener(&mut self, clave: &K) -> Option<&V>;
    pub fn contiene(&self, clave: &K) -> bool;
    pub fn len(&self) -> usize;
}
```

**Comportamiento LRU**: Cuando la caché está llena, elimina el elemento menos recientemente usado.

## 🧪 Ejecución

```bash
# Ejecutar demo
cargo run

# Ejecutar todos los tests
cargo test

# Tests con output
cargo test -- --nocapture

# Solo tests de un módulo
cargo test cola::
cargo test deque::
```

## ✅ Criterios de Evaluación

| Criterio | Puntos |
|----------|--------|
| Cola<T> funcional con tests | 20 |
| Deque<T> funcional con tests | 20 |
| Limitado<T, N> con const generics | 25 |
| Cache<K, V> con trait bounds | 25 |
| Código limpio y documentado | 10 |
| **Total** | **100** |

## 💡 Tips de Implementación

### Para Cola y Deque
- Usa `VecDeque` internamente para eficiencia
- O implementa con `Vec` si prefieres más control

### Para Limitado
- `const N: usize` define la capacidad máxima
- Considera usar un `Vec` con verificación manual de tamaño

### Para Cache
- Usa `HashMap` para acceso O(1)
- Usa `Vec` o `VecDeque` para tracking de orden de uso
- Al acceder a un elemento, muévelo al "frente" (más reciente)

## 🔗 Recursos

- [VecDeque Documentation](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
- [HashMap Documentation](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [LRU Cache - Wikipedia](https://en.wikipedia.org/wiki/Cache_replacement_policies#Least_recently_used_(LRU))

## 📅 Entrega

- Todos los tests deben pasar
- Código formateado con `cargo fmt`
- Sin warnings de `cargo clippy`
- Documentación básica con `///` en funciones públicas
