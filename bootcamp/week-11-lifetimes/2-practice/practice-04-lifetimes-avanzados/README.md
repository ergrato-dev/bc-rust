# 📝 Práctica 04: Lifetimes Avanzados

## 🎯 Objetivo

Dominar conceptos avanzados de lifetimes: `'static`, lifetime bounds, y patrones con traits genéricos.

## 📋 Ejercicios

### Ejercicio 1: `'static` ⭐⭐
Funciones que retornan datos con lifetime `'static`.

**Concepto:** `'static` significa que puede vivir durante toda la ejecución.

### Ejercicio 2: Lifetime Bounds ⭐⭐⭐
Struct genérico con `T: 'a`.

**Concepto:** `T: 'a` = "T vive al menos tanto como 'a"

### Ejercicio 3: Múltiples Lifetimes ⭐⭐⭐
Struct con dos lifetimes diferentes.

**Concepto:** A veces los datos tienen diferentes tiempos de vida.

### Ejercicio 4: Trait con Lifetime ⭐⭐⭐
Implementar un trait que tiene parámetro de lifetime.

**Concepto:** Los traits pueden ser genéricos sobre lifetimes.

### Ejercicio 5: Genéricos + Lifetimes ⭐⭐⭐⭐
Combinar tipos genéricos con lifetime bounds.

## 🏃 Ejecución

```bash
cargo run
cargo test
```

## 💡 Conceptos Clave

### 'static

```rust
// String literals son 'static
let s: &'static str = "hola";

// Constantes también
const VER: &str = "1.0";
```

### Lifetime Bounds

```rust
// T debe vivir al menos tanto como 'a
struct Holder<'a, T: 'a> {
    value: &'a T,
}

// 'a debe vivir al menos tanto como 'b
fn f<'a, 'b>(x: &'a str) -> &'b str
where 'a: 'b
{
    x
}
```

### Trait con Lifetime

```rust
trait Parser<'a> {
    fn parse(&self, input: &'a str) -> Vec<&'a str>;
}
```

## ✅ Criterios de Éxito

- [ ] Entiendes cuándo usar `'static`
- [ ] Sabes aplicar bounds como `T: 'a`
- [ ] Puedes implementar traits con lifetimes
- [ ] Combinas genéricos y lifetimes correctamente

## 📚 Recursos Adicionales

- [Rust Book - Advanced Lifetimes](https://doc.rust-lang.org/book/ch19-02-advanced-lifetimes.html)
- [Rustonomicon - Lifetimes](https://doc.rust-lang.org/nomicon/lifetimes.html)
