# Práctica 02: Structs y Enums Genéricos

## 🎯 Objetivo

Crear estructuras de datos genéricas reutilizables, incluyendo:

- Structs con parámetros de tipo
- Enums genéricos
- Bloques `impl` genéricos
- Implementaciones especializadas

## 📚 Conceptos Clave

### Struct Genérico

```rust
struct Par<T> {
    primero: T,
    segundo: T,
}

impl<T> Par<T> {
    fn new(primero: T, segundo: T) -> Self {
        Par { primero, segundo }
    }
}
```

### Enum Genérico

```rust
enum Resultado<T, E> {
    Exito(T),
    Fallo(E),
}
```

### Implementación Especializada

```rust
// Solo para Punto<f64>
impl Punto<f64> {
    fn distancia(&self) -> f64 {
        // Solo f64 tiene sqrt()
    }
}
```

## 📝 Ejercicios

### Ejercicio 1: Par Genérico

Implementa un struct que almacena dos valores del mismo tipo.

```rust
struct Par<T> { ... }
```

**Métodos**:
- `new(primero, segundo)` - Constructor
- `primero(&self)` - Referencia al primer elemento
- `segundo(&self)` - Referencia al segundo elemento
- `invertir(self)` - Nuevo Par con elementos intercambiados

**Dificultad**: ⭐⭐

---

### Ejercicio 2: Caja Genérica

Implementa un contenedor simple para cualquier valor.

```rust
struct Caja<T> { ... }
```

**Métodos**:
- `new(valor)` - Constructor
- `valor(&self)` - Referencia al contenido
- `desenvolver(self)` - Consume y devuelve el valor
- `map<U, F>(self, f)` - Transforma el contenido

**Dificultad**: ⭐⭐⭐

---

### Ejercicio 3: Punto Genérico

Implementa un punto 2D con métodos generales y especializados.

```rust
struct Punto<T> { ... }
```

**Métodos generales** (para cualquier T):
- `new(x, y)` - Constructor
- `x(&self)` - Referencia a x
- `y(&self)` - Referencia a y

**Métodos especializados** (solo para f64):
- `distancia_origen(&self)` - Calcula √(x² + y²)

**Dificultad**: ⭐⭐⭐

---

### Ejercicio 4: Resultado Simplificado

Implementa un enum similar a `Result`.

```rust
enum Resultado<T, E> { ... }
```

**Métodos**:
- `exito(valor)` - Crea variante exitosa
- `fallo(error)` - Crea variante fallida
- `es_exito(&self)` - Verifica si es éxito
- `es_fallo(&self)` - Verifica si es fallo
- `obtener_valor(self)` - Extrae valor si es éxito

**Dificultad**: ⭐⭐⭐

---

### Ejercicio 5: Pila Genérica

Implementa una pila (stack) LIFO usando `Vec`.

```rust
struct Pila<T> { ... }
```

**Métodos**:
- `new()` - Crea pila vacía
- `push(&mut self, valor)` - Agrega al tope
- `pop(&mut self)` - Remueve del tope
- `peek(&self)` - Mira el tope sin remover
- `len(&self)` - Cantidad de elementos
- `esta_vacia(&self)` - Verifica si está vacía

**Dificultad**: ⭐⭐

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
- [ ] Structs tienen los campos correctos
- [ ] Enums tienen las variantes correctas
- [ ] Métodos implementados correctamente
- [ ] Implementaciones especializadas funcionan

## 💡 Tips

1. **Self** se refiere al tipo actual en `impl`
2. **&self** para métodos que solo leen
3. **&mut self** para métodos que modifican
4. **self** (sin &) para métodos que consumen
5. Usa `matches!` para verificar variantes de enum

## 🔗 Recursos

- [The Rust Book - Generic Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Rust by Example - Structures](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)
