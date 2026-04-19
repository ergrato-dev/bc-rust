# Práctica 04: Iteradores

## 🎯 Objetivos

- Dominar adaptadores: `map`, `filter`, `take`, `skip`
- Usar consumidores: `collect`, `fold`, `sum`, `find`
- Combinar con `zip`, `enumerate`, `chain`
- Implementar el trait `Iterator`

## 📋 Métodos Comunes

### Adaptadores (lazy)
```rust
.map(|x| ...)      // transformar
.filter(|x| ...)   // filtrar
.take(n)           // tomar n primeros
.skip(n)           // saltar n primeros
.enumerate()       // añadir índice
.zip(otro)         // combinar con otro iterador
```

### Consumidores (ejecutan)
```rust
.collect()         // a colección
.fold(init, |acc, x| ...)  // reducir
.sum()             // sumar
.find(|x| ...)     // buscar
.count()           // contar
```

## 📝 Ejercicios

### Ejercicio 1: Map y Filter (4 puntos)

Retorna los cuadrados de los números pares.

```rust
fn cuadrados_pares(numeros: &[i32]) -> Vec<i32> {
    // Usa .filter() y .map()
}
```

### Ejercicio 2: Fold (4 puntos)

Concatena strings usando fold.

```rust
fn concatenar(palabras: &[&str]) -> String {
    // Usa .fold(String::new(), ...)
}
```

### Ejercicio 3: Find con Enumerate (4 puntos)

Encuentra el primer elemento mayor que un umbral.

```rust
fn primer_mayor_que(datos: &[i32], umbral: i32) -> Option<(usize, i32)> {
    // Usa .enumerate() y .find()
}
```

### Ejercicio 4: Zip (4 puntos)

Combina dos slices en tuplas.

```rust
fn combinar_datos<'a>(nombres: &'a [&str], edades: &[i32]) -> Vec<(&'a str, i32)> {
    // Usa .zip()
}
```

### Ejercicio 5: Iterator Personalizado (4 puntos)

Implementa un iterador Fibonacci.

```rust
impl Iterator for Fibonacci {
    type Item = u64;
    
    fn next(&mut self) -> Option<Self::Item> {
        // Genera: 0, 1, 1, 2, 3, 5, 8, ...
    }
}
```

## 🧪 Tests

```bash
cargo test
```

## 💡 Pistas

1. **filter + map**: filtra primero, luego transforma
2. **fold**: `fold(inicial, |acumulador, elemento| nuevo_acumulador)`
3. **enumerate**: convierte `iter` en `(índice, valor)`
4. **Fibonacci**: guarda `actual` y `siguiente`, intercámbialos

## ✅ Verificación

```bash
cargo run
cargo test
cargo clippy
```
