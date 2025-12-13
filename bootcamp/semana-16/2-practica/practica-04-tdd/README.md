# Practica 04: Test-Driven Development

## Objetivo

Implementar una Pila (Stack) usando TDD.

## Metodologia

1. **RED**: Escribe un test que falle
2. **GREEN**: Implementa el codigo minimo
3. **REFACTOR**: Mejora sin romper tests

## Ejercicios

Implementa una pila generica con:

1. `new()` - Crear pila vacia
2. `push(item)` - Agregar elemento
3. `pop()` - Quitar y retornar el tope
4. `peek()` - Ver el tope sin quitar
5. `is_empty()` - Verificar si esta vacia
6. `len()` - Cantidad de elementos

## Pasos TDD

### Test 1: Pila vacia
```rust
#[test]
fn test_pila_vacia() {
    let pila: Pila<i32> = Pila::new();
    assert!(pila.is_empty());
}
```

### Test 2: Push y Pop
Escribe el test, luego implementa.

### Test 3: Peek
Escribe el test, luego implementa.

## Ejecutar

```bash
cargo test
```
