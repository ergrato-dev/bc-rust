# Práctica 04 — Panic Handler y Error Codes

## 🎯 Objetivo
Implementar manejo de errores sin `std::error::Error` y definir un `#[panic_handler]` condicional.

## 🛠️ Cómo ejecutar

```bash
cargo test
```

## ✅ Criterios de Aceptación

- [ ] `#[panic_handler]` compilado solo cuando `#[cfg(not(test))]`
- [ ] `ErrorCode` con `#[repr(u8)]`
- [ ] `EventLog` con array fijo de capacidad 8
- [ ] `cargo test` pasa
