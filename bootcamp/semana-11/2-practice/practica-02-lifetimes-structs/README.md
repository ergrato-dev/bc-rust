# 📝 Práctica 02: Lifetimes en Structs

## 🎯 Objetivo

Aprender a declarar e implementar structs que contienen referencias, entendiendo la relación entre el lifetime del struct y los datos que referencia.

## 📋 Ejercicios

### Ejercicio 1: `Excerpt` ⭐⭐
Un struct simple con una referencia a texto.

**Concepto:** El struct no puede vivir más que el dato referenciado.

### Ejercicio 2: `Config` ⭐⭐
Configuración con clave y valor como referencias.

**Pregunta:** ¿Un lifetime o dos?

### Ejercicio 3: `LineIterator` ⭐⭐⭐
Iterador manual sobre líneas de texto.

**Concepto:** Combina referencia (`content`) con valor (`position`).

### Ejercicio 4: `Split` ⭐⭐
Divide un string en dos partes.

**Concepto:** Ambas partes comparten el mismo lifetime.

### Ejercicio 5: `Article` ⭐⭐⭐
Mezcla datos owned (String) y referencias.

**Concepto:** Solo las referencias necesitan lifetime.

## 🏃 Ejecución

```bash
# Ejecutar ejercicios
cargo run

# Ejecutar tests
cargo test

# Ver solución
cargo run --bin solucion
```

## 💡 Conceptos Clave

### Structs con Lifetimes

```rust
struct MyStruct<'a> {
    reference: &'a str,  // Referencia - necesita 'a
    owned: String,       // Owned - no necesita lifetime
}
```

### impl con Lifetimes

```rust
impl<'a> MyStruct<'a> {
    fn new(reference: &'a str) -> MyStruct<'a> {
        // ...
    }
}
```

### La Regla de Oro

> Un struct con referencias NO puede vivir más que los datos que referencia.

## ✅ Criterios de Éxito

- [ ] Todos los structs compilan correctamente
- [ ] Los tests pasan
- [ ] Entiendes cuándo usar uno vs múltiples lifetimes
- [ ] Sabes diferenciar campos owned de referencias

## 📚 Recursos

- [Rust Book - Lifetimes in Structs](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-struct-definitions)
