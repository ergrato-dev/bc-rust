# Práctica 03 — Doctests y Ejemplos

## 🎯 Objetivo
Escribir documentación ejecutable (doctests) que sirva como especificación viva de la API.

## 📋 Instrucciones

1. Implementar `factorial`, `fibonacci`, `es_primo` con doctests completos en cada una
2. Añadir un doctest a nivel de módulo (en `//!` del lib.rs o main.rs)
3. Crear `examples/demo.rs` que use las tres funciones y se ejecute con `cargo run --example demo`
4. Documentar el caso de pánico de `factorial` con la sección `# Panics`
5. Añadir sección `# Errors` en alguna función que retorne `Result`

## ✅ Criterios de Aceptación

- [ ] `cargo test --doc` pasa todos los doctests
- [ ] `cargo run --example demo` funciona
- [ ] Cada función pública tiene `# Examples` en su doc
- [ ] `cargo clippy -- -D warnings` pasa limpio
