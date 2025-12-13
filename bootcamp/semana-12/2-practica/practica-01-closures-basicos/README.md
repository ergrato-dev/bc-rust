# Práctica 01: Closures Básicos

## 🎯 Objetivos

- Crear closures con diferentes sintaxis
- Pasar closures como argumentos a funciones
- Retornar closures de funciones
- Usar `move` para capturar por valor

## 📋 Ejercicios

### Ejercicio 1: Crear Duplicador (3 puntos)

Implementa `crear_duplicador()` que retorne un closure que multiplique por 2.

```rust
fn crear_duplicador() -> impl Fn(i32) -> i32 {
    // Tu código aquí
}
```

### Ejercicio 2: Crear Sumador (3 puntos)

Implementa `crear_sumador()` que retorne un closure que sume dos números.

```rust
fn crear_sumador() -> impl Fn(i32, i32) -> i32 {
    // Tu código aquí
}
```

### Ejercicio 3: Transformador (4 puntos)

Implementa `crear_transformador()` que:
- Si el número es par, lo multiplica por 2
- Si el número es impar, le suma 1

```rust
fn crear_transformador() -> impl Fn(i32) -> i32 {
    // Tu código aquí
}
```

### Ejercicio 4: Filtro de Pares (2 puntos)

Implementa `crear_filtro_pares()` que retorne un predicado para filtrar números pares.

```rust
fn crear_filtro_pares() -> impl Fn(&&i32) -> bool {
    // Tu código aquí
}
```

### Ejercicio 5: Factory de Multiplicadores (3 puntos)

Implementa `crear_multiplicador(factor)` que capture el factor y retorne un closure multiplicador.

```rust
fn crear_multiplicador(factor: i32) -> impl Fn(i32) -> i32 {
    // Tu código aquí - usa 'move'
}
```

## 🧪 Tests

```bash
cargo test
```

## 💡 Pistas

1. **Sintaxis de closure**: `|args| expresion`
2. **Múltiples líneas**: `|args| { ... }`
3. **Captura por valor**: usa `move` antes de `|`
4. **`impl Fn`**: permite retornar closures

## ✅ Verificación

```bash
cargo run
cargo test
cargo clippy
```
