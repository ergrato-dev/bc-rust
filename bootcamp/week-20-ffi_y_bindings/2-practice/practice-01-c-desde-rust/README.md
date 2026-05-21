# Práctica 01 — Llamar a C desde Rust

## 🎯 Objetivo
Declarar e invocar funciones de la librería estándar de C desde Rust usando `extern "C"` y el crate `libc`.

## 📋 Instrucciones

1. Declarar `extern "C" { fn strlen(...) }` y `fn abs(...)` manualmente
2. Crear wrappers seguros (`fn strlen_safe`, `fn abs_safe`) con `// SAFETY:`
3. Usar `CString::new` para convertir `&str` a un puntero C
4. Llamar a `libc::malloc` y `libc::free` manualmente para asignar un buffer
5. Implementar `fn to_upper_c(s: &str) -> String` que llame a `toupper` en cada byte

## ✅ Criterios de Aceptación

- [ ] Compila sin warnings
- [ ] `cargo test` pasa (≥ 4 tests)
- [ ] Cada `unsafe` tiene `// SAFETY:`
- [ ] `CString` se usa para pasar strings a C (no `&str` raw)
- [ ] `cargo clippy -- -D warnings` pasa limpio

## 💡 Pistas

- `libc::c_char` es el tipo correcto para `*const char` de C
- `CString::as_ptr()` retorna `*const c_char` — válido mientras el `CString` esté vivo
- El crate `libc` re-exporta todas las funciones estándar de C
