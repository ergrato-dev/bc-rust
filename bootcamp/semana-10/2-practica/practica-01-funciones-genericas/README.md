# Práctica 01: Funciones Genéricas

## 🎯 Objetivo

Dominar la sintaxis y uso de funciones genéricas en Rust, incluyendo:

- Definición de parámetros de tipo genérico
- Uso de trait bounds básicos
- Inferencia de tipos

## 📚 Conceptos Clave

### Sintaxis Básica

```rust
fn nombre<T>(parametro: T) -> T {
    parametro
}
```

### Con Trait Bounds

```rust
fn comparar<T: PartialOrd>(a: T, b: T) -> bool {
    a > b
}
```

## 📝 Ejercicios

### Ejercicio 1: Función Identidad

Implementa una función que devuelve el mismo valor que recibe.

```rust
fn identidad<T>(valor: T) -> T
```

**Dificultad**: ⭐

---

### Ejercicio 2: Intercambiar Valores

Implementa una función que intercambia dos valores.

```rust
fn intercambiar<T>(a: T, b: T) -> (T, T)
```

**Dificultad**: ⭐

---

### Ejercicio 3: Mayor de Dos

Implementa una función que devuelve el mayor de dos valores.

```rust
fn mayor<T: PartialOrd>(a: T, b: T) -> T
```

**Pista**: Usa el operador `>` para comparar.

**Dificultad**: ⭐⭐

---

### Ejercicio 4: Primer Elemento

Implementa una función que devuelve el primer elemento de un slice.

```rust
fn primero<T: Clone>(elementos: &[T]) -> Option<T>
```

**Pista**: Usa `.first().cloned()` o maneja el caso vacío manualmente.

**Dificultad**: ⭐⭐

---

### Ejercicio 5: Contar Elementos

Implementa una función que cuenta los elementos de un slice.

```rust
fn contar<T>(elementos: &[T]) -> usize
```

**Dificultad**: ⭐

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
- [ ] No hay warnings de compilación
- [ ] El código usa genéricos correctamente
- [ ] Se entiende cuándo usar trait bounds

## 💡 Tips

1. **Sin bounds**: Si solo necesitas pasar el valor, no necesitas traits
2. **Clone**: Necesario cuando quieres duplicar un valor
3. **PartialOrd**: Necesario para comparaciones (<, >, <=, >=)
4. **Copy vs Clone**: `Copy` es implícito, `Clone` es explícito

## 🔗 Recursos

- [The Rust Book - Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Rust by Example - Generics](https://doc.rust-lang.org/rust-by-example/generics.html)
