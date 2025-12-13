# Proyecto: Calculadora Testeada

## Objetivo

Implementar una calculadora completa con:
- Tests unitarios
- Tests de integracion
- Documentacion con doc tests
- Cobertura de errores

## Funcionalidades

1. Operaciones basicas (+, -, *, /)
2. Operaciones avanzadas (potencia, raiz, factorial)
3. Historial de operaciones
4. Manejo de errores

## Estructura

```
proyecto-calculadora-testeada/
+-- Cargo.toml
+-- src/
|   +-- lib.rs
|   +-- main.rs
+-- tests/
    +-- integration_test.rs
```

## Requisitos

- Minimo 20 tests unitarios
- Minimo 5 tests de integracion
- Todas las funciones documentadas
- Doc tests para funciones principales

## Ejecutar

```bash
cargo test
cargo test --doc
cargo doc --open
```
