# Practica 01: Tests Unitarios

## Objetivo

Escribir tests unitarios para una calculadora simple.

## Instrucciones

1. Implementa las funciones de la calculadora
2. Escribe tests para cada operacion
3. Usa las macros `assert!`, `assert_eq!`, `assert_ne!`
4. Implementa tests con `#[should_panic]`

## Ejercicios

### Ejercicio 1: Tests basicos
Implementa tests para `suma`, `resta`, `multiplica`.

### Ejercicio 2: Tests con panic
Implementa `dividir` que hace panic si el divisor es 0.
Escribe un test con `#[should_panic]`.

### Ejercicio 3: Tests con Result
Implementa `dividir_safe` que retorna `Result`.
Escribe tests que usen `?`.

### Ejercicio 4: Organizacion
Agrupa los tests en submodulos por operacion.

## Ejecutar

```bash
cargo test
cargo test suma
cargo test -- --show-output
```
