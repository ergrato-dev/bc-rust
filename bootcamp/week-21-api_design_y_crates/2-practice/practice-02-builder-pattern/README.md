# Práctica 02 — Patrón Builder con Typestate

## 🎯 Objetivo
Implementar el patrón Builder avanzado usando phantom types (typestate) para garantizar en tiempo de compilación que los campos obligatorios estén presentes.

## 📋 Instrucciones

1. Crear `HttpRequestBuilder<U>` donde `U` es un phantom type (`NoUrl` / `WithUrl`)
2. El método `.url()` transforma `Builder<NoUrl>` → `Builder<WithUrl>`
3. El método `.build()` solo existe en `Builder<WithUrl>`
4. Implementar `.method()`, `.header()`, `.body()` disponibles en cualquier estado

## ✅ Criterios de Aceptación

- [ ] Intentar llamar `.build()` sin `.url()` da error de compilación
- [ ] `cargo test` pasa (≥ 3 tests)
- [ ] `cargo clippy -- -D warnings` pasa limpio
- [ ] Doctest funcional en la documentación del Builder
