# Práctica 01 — Raw Pointers

## 🎯 Objetivo
Aprender a crear, verificar y desreferenciar raw pointers (`*const T`, `*mut T`) con invariantes documentados.

## 📋 Instrucciones

1. Crear un `i32` en stack y obtener un raw pointer `*const i32`
2. Desreferenciar el pointer en un bloque `unsafe` con `// SAFETY:` obligatorio
3. Obtener un `*mut i32` desde una referencia mutable y modificar el valor
4. Implementar la función `safe_read<T>(ptr: *const T) -> Option<T>` que verifique null antes de desreferenciar
5. Demostrar que crear un raw pointer a una variable que ya no existe en el stack es UB (solo documentar, no ejecutar)

## ✅ Criterios de Aceptación

- [ ] El programa compila sin warnings
- [ ] Pasa `cargo test`
- [ ] Cada `unsafe` tiene `// SAFETY:` con justificación
- [ ] `safe_read` comprueba `ptr.is_null()` antes de desreferenciar
- [ ] `cargo clippy -- -D warnings` pasa limpio

## 💡 Pistas

- `std::ptr::null()` crea un null pointer del tipo correcto
- `ptr.is_null()` es el método de verificación
- Para crear `*mut` desde `&mut`: `&mut x as *mut i32`
