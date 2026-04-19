# Práctica 04: Genéricos Avanzados

## 🎯 Objetivo

Explorar características avanzadas del sistema de genéricos de Rust:

- Tipos asociados en traits
- Const generics para valores en tiempo de compilación
- PhantomData para tipos marcadores
- Type State Pattern para estados en el sistema de tipos

## 📚 Conceptos Clave

### Tipos Asociados

```rust
trait Iterador {
    type Item;  // Tipo asociado
    fn siguiente(&mut self) -> Option<Self::Item>;
}

impl Iterador for MiTipo {
    type Item = i32;  // Especifica el tipo
    fn siguiente(&mut self) -> Option<Self::Item> { ... }
}
```

### Const Generics

```rust
struct Buffer<T, const N: usize> {
    datos: [T; N],
}

// N es conocido en tiempo de compilación
let buffer: Buffer<u8, 1024> = Buffer::new([0; 1024]);
```

### PhantomData

```rust
use std::marker::PhantomData;

struct Id<T> {
    valor: u64,
    _marker: PhantomData<T>,  // "Usa" T sin almacenarlo
}
```

### Type State Pattern

```rust
struct Cerrado;
struct Abierto;

struct Puerta<Estado> {
    _estado: PhantomData<Estado>,
}

impl Puerta<Cerrado> {
    fn abrir(self) -> Puerta<Abierto> { ... }
}
```

## 📝 Ejercicios

### Ejercicio 1: Trait con Tipo Asociado

Implementa un trait `Iterador` simplificado con un tipo asociado `Item`.

```rust
trait Iterador {
    type Item;
    fn siguiente(&self) -> Option<Self::Item>;
}
```

**Dificultad**: ⭐⭐

---

### Ejercicio 2: Const Generics

Implementa un `Buffer<T, N>` de tamaño fijo usando const generics.

```rust
struct Buffer<T, const N: usize> { ... }
```

**Métodos**:
- `new(datos)` - Constructor
- `capacidad()` - Devuelve N
- `obtener(indice)` - Acceso seguro con Option
- `obtener_copia(indice)` - Solo para T: Copy

**Dificultad**: ⭐⭐⭐

---

### Ejercicio 3: Type State Pattern

Implementa un sistema de pedidos con estados en el tipo:

```
Pendiente → Pagado → Enviado → Entregado
```

Cada transición solo es posible desde el estado correcto.

**Dificultad**: ⭐⭐⭐⭐

---

### Ejercicio 4: PhantomData para IDs Tipados

Implementa IDs que pertenecen a tipos específicos de entidades.

```rust
let user_id: Id<Usuario> = Id::new(1);
let product_id: Id<Producto> = Id::new(1);

// Esto NO compila - tipos diferentes!
// user_id == product_id
```

**Dificultad**: ⭐⭐⭐

## 🧪 Ejecución

```bash
# Ejecutar el programa
cargo run

# Ejecutar tests
cargo test

# Ver tests con output
cargo test -- --nocapture
```

## ✅ Criterios de Éxito

- [ ] Todos los tests pasan
- [ ] Type State previene transiciones inválidas
- [ ] IDs tipados no son comparables entre tipos
- [ ] Const generics funcionan con diferentes tamaños

## 💡 Tips

1. **Tipos Asociados vs Genéricos**:
   - Asociados: Un tipo por implementación
   - Genéricos: Múltiples tipos por implementación

2. **PhantomData**:
   - No ocupa espacio en memoria
   - Satisface el compilador sobre "uso" del tipo
   - Útil para tipos marcadores

3. **Type State**:
   - Los estados son structs vacíos
   - Las transiciones consumen `self`
   - El compilador previene errores en tiempo de compilación

## 🔗 Recursos

- [The Rust Book - Associated Types](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types)
- [Rust Reference - Const Generics](https://doc.rust-lang.org/reference/items/generics.html#const-generics)
- [PhantomData Documentation](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)
