# Practica 02: Tests de Integracion

## Objetivo

Crear tests de integracion para una biblioteca de usuarios.

## Instrucciones

1. Implementa el modulo `usuarios` en `src/lib.rs`
2. Crea tests de integracion en `tests/`
3. Comparte codigo con `tests/common/mod.rs`

## Estructura

```
practica-02-integration/
+-- Cargo.toml
+-- src/
|   +-- lib.rs
+-- tests/
    +-- common/
    |   +-- mod.rs
    +-- usuarios_test.rs
```

## Ejecutar

```bash
cargo test
cargo test --test usuarios_test
cargo test --lib
```
