# 📦 Práctica 01: Box<T>

## 🎯 Objetivos

- Usar `Box<T>` para crear tipos recursivos
- Implementar una lista enlazada
- Usar trait objects con `Box<dyn Trait>`

---

## 📋 Ejercicios

### Ejercicio 1: Lista Enlazada

Implementa una lista enlazada simple usando `Box`:

```rust
pub struct Lista<T> {
    head: Option<Box<Nodo<T>>>,
}

struct Nodo<T> {
    valor: T,
    siguiente: Option<Box<Nodo<T>>>,
}
```

**Métodos a implementar:**
- `new()` - Crear lista vacía
- `push(valor)` - Agregar al inicio
- `pop()` - Remover y retornar del inicio
- `peek()` - Ver primer elemento sin remover
- `len()` - Contar elementos

### Ejercicio 2: Árbol Binario de Búsqueda

Implementa un árbol binario:

```rust
pub struct ArbolBinario<T> {
    valor: T,
    izquierda: Option<Box<ArbolBinario<T>>>,
    derecha: Option<Box<ArbolBinario<T>>>,
}
```

**Métodos a implementar:**
- `new(valor)` - Crear árbol con raíz
- `insertar(valor)` - Insertar manteniendo orden
- `contiene(valor)` - Buscar un valor

### Ejercicio 3: Trait Objects

Implementa figuras geométricas usando `Box<dyn Trait>`:

```rust
trait Figura {
    fn area(&self) -> f64;
    fn nombre(&self) -> &str;
}
```

**A implementar:**
- `Circulo` con radio
- `Rectangulo` con ancho y alto
- Factory function `crear_figura(tipo, dimension)`

---

## 🧪 Tests

```bash
cargo test
```

---

## 💡 Pistas

### Lista Enlazada

```rust
// push: el nuevo nodo apunta al head actual
let nuevo = Box::new(Nodo {
    valor,
    siguiente: self.head.take(),  // take() obtiene ownership
});
self.head = Some(nuevo);

// pop: extraer head y actualizar
self.head.take().map(|nodo| {
    self.head = nodo.siguiente;
    nodo.valor
})
```

### Árbol Binario

```rust
// Inserción recursiva
if valor < self.valor {
    match &mut self.izquierda {
        Some(izq) => izq.insertar(valor),
        None => self.izquierda = Some(Box::new(Self::new(valor))),
    }
}
```

---

## ✅ Criterios de Éxito

- [ ] Lista enlazada funciona correctamente
- [ ] Árbol mantiene orden BST
- [ ] Trait objects permiten polimorfismo
- [ ] Todos los tests pasan
