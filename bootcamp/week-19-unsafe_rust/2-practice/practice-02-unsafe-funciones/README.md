# Práctica 02 — Funciones `unsafe`

## 🎯 Objetivo
Declarar y llamar funciones marcadas `unsafe fn`, documentar sus precondiciones e invariantes.

## 📋 Instrucciones

1. Implementar `unsafe fn swap_raw<T>(a: *mut T, b: *mut T)` que intercambie dos valores via raw pointers
2. Implementar `unsafe fn copy_nonoverlapping_manual<T>(src: *const T, dst: *mut T, count: usize)` sin usar `ptr::copy_nonoverlapping`
3. Crear una envoltura segura `fn swap<T>(a: &mut T, b: &mut T)` que llame a `swap_raw` internamente
4. Documentar cada función `unsafe` con sección `# Safety` en rustdoc y `// SAFETY:` en cada llamada

## ✅ Criterios de Aceptación

- [ ] Compila sin warnings
- [ ] `cargo test` pasa (mínimo 4 tests)
- [ ] Toda función `unsafe` tiene `# Safety` en su documentación
- [ ] Toda llamada `unsafe { ... }` tiene `// SAFETY:` previo
- [ ] `cargo clippy -- -D warnings` pasa limpio

## 💡 Pistas

- `std::ptr::read` y `std::ptr::write` son alternativas seguras a `*ptr`
- La sección `# Safety` en rustdoc documenta las precondiciones para el caller
