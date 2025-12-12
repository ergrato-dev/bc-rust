# Práctica 03: Borrow Checker

## 🎯 Objetivo

Aprender a leer y resolver errores del borrow checker.

## 📋 Ejercicios

### Ejercicio 1: Corregir Data Race

Este código intenta modificar un vector mientras hay una referencia activa:

```rust
fn main() {
    let mut numeros = vec![1, 2, 3, 4, 5];
    let primero = &numeros[0];
    
    numeros.push(6);  // ERROR: modifica mientras hay préstamo
    
    println!("Primero: {}", primero);
}
```

**Corrige de dos formas diferentes.**

### Ejercicio 2: Préstamos en Conflicto

Corrige este código manteniendo la funcionalidad:

```rust
fn main() {
    let mut texto = String::from("Hola");
    
    let r1 = &texto;
    let r2 = &texto;
    let r3 = &mut texto;  // ERROR
    
    println!("{}, {}", r1, r2);
    r3.push_str(" mundo");
    println!("{}", r3);
}
```

### Ejercicio 3: Referencia a Variable Local

¿Por qué no compila? Corrígelo:

```rust
fn crear_mensaje() -> &String {
    let s = String::from("Hola desde la función");
    &s  // ERROR: devuelve referencia a local
}
```

### Ejercicio 4: Análisis de Lifetimes

Identifica cuál es el **último uso** de cada referencia:

```rust
fn main() {
    let mut s = String::from("hola");
    
    let r1 = &s;           // Línea A
    println!("{}", r1);     // Línea B
    
    let r2 = &s;           // Línea C
    let r3 = &s;           // Línea D
    println!("{} {}", r2, r3);  // Línea E
    
    let r4 = &mut s;       // Línea F
    r4.push_str("!");       // Línea G
    println!("{}", r4);     // Línea H
}
```

**Preguntas:**
1. ¿Dónde termina el préstamo de r1?
2. ¿Dónde terminan los préstamos de r2 y r3?
3. ¿Por qué r4 puede existir después de r2 y r3?

---

## ✅ Criterios de Éxito

- [ ] Puedes leer mensajes de error del borrow checker
- [ ] Identificas el conflicto entre referencias
- [ ] Conoces las técnicas para resolver errores
- [ ] Entiendes Non-Lexical Lifetimes (NLL)

---

## 💡 Técnicas de Solución

1. **Reorganizar código**: Mover el uso de refs antes de modificar
2. **Scope interno**: Usar `{}` para limitar el préstamo
3. **Clonar**: Cuando necesitas dos valores independientes
4. **Cambiar firma**: Retornar ownership en lugar de referencia
