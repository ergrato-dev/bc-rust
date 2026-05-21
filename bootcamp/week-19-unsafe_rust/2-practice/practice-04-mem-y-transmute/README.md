# Práctica 04 — `std::mem`: `transmute`, `size_of`, `forget`

## 🎯 Objetivo
Usar las herramientas de `std::mem` para inspeccionar y manipular la representación en memoria de los tipos.

## 📋 Instrucciones

1. Usar `mem::size_of::<T>()` y `mem::align_of::<T>()` para varios tipos primitivos y structs
2. Usar `mem::transmute` para reinterpretar los bits de un `u32` como `f32` (IEEE 754)
3. Usar `mem::replace` para intercambiar un `String` sin clonar
4. Demostrar `mem::forget` y explicar en un comentario por qué produce un memory leak
5. Explorar `mem::swap` y compararlo con el patrón `let tmp = ...`

## ✅ Criterios de Aceptación

- [ ] Compila sin warnings
- [ ] `cargo test` pasa (mínimo 4 tests)
- [ ] Cada `transmute` tiene `// SAFETY:` explicando que los tipos tienen el mismo tamaño/alineación
- [ ] Hay un comentario explicando por qué `forget` puede usarse en destrucción manual

## 💡 Pistas

- `mem::transmute::<A, B>` requiere que `size_of::<A>() == size_of::<B>()`
- Prefiere `bytemuck::cast` sobre `transmute` en código de producción cuando sea posible
- `mem::MaybeUninit<T>` es la forma idiomática de inicialización diferida en Rust moderno
