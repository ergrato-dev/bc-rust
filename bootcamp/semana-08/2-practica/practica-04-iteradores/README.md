# Práctica 04: Iteradores

## 🎯 Objetivos

- Dominar los tres tipos de iteradores: `iter()`, `iter_mut()`, `into_iter()`
- Usar adaptadores: `map`, `filter`, `take`, `skip`, `enumerate`
- Aplicar consumidores: `collect`, `fold`, `reduce`, `sum`, `any`, `all`
- Crear cadenas de procesamiento eficientes

## 📋 Ejercicios

### Ejercicio 1: Tipos de Iteradores

Funciones que demuestran cuándo usar cada tipo de iterador.

### Ejercicio 2: Adaptadores

Transformaciones con `map`, `filter`, `enumerate`, `zip`, `flat_map`.

### Ejercicio 3: Consumidores

Operaciones terminales: `collect`, `fold`, `reduce`, `find`, `position`.

### Ejercicio 4: Procesador de Datos

Sistema completo de procesamiento de registros usando iteradores.

## 🚀 Ejecución

```bash
# Ejecutar
cargo run

# Tests
cargo test

# Tests con output
cargo test -- --nocapture
```

## 📚 Conceptos Clave

- **Lazy evaluation**: Los iteradores no procesan hasta que se consumen
- **Zero-cost abstractions**: El compilador optimiza las cadenas de iteradores
- **Ownership**: `iter()` vs `iter_mut()` vs `into_iter()` determinan propiedad

## 🔗 Recursos

- [Iterator Trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [The Rust Book - Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
