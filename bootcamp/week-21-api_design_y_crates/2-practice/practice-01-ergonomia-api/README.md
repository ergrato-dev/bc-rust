# Práctica 01 — Ergonomía de API: Builder + Errores

## 🎯 Objetivo
Diseñar una API ergonómica usando el patrón Builder y tipos de error custom con `thiserror`.

## 📋 Instrucciones

1. Implementar `UserBuilder` con el patrón Builder (métodos encadenables)
2. Usar `impl Into<String>` para los parámetros string (más ergonómico que `&str` o `String`)
3. Validar en `build()` y retornar `Result<User, UserError>`
4. Definir `enum UserError` con `thiserror`: `EmptyName`, `InvalidAge`, `InvalidEmail`
5. Escribir doctests en `UserBuilder` que funcionen con `cargo test`

## ✅ Criterios de Aceptación

- [ ] Métodos del builder son encadenables (fluent interface)
- [ ] `impl Into<String>` en parámetros string
- [ ] `build()` retorna `Result<User, UserError>`
- [ ] Doctests en la documentación de `UserBuilder::build`
- [ ] `cargo test` pasa (≥ 4 tests)
- [ ] `cargo clippy -- -D warnings` pasa limpio
