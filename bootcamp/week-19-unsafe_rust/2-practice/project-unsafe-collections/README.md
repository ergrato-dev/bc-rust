# Proyecto — `RawVec<T>`: Vector con Gestión Manual de Memoria

## 🎯 Objetivo
Implementar un vector dinámico (`RawVec<T>`) que gestione su propia memoria usando el allocator global de Rust, demostrando el interior de `std::vec::Vec`.

## 📋 Instrucciones

1. Implementar `RawVec<T>` con campos `ptr: NonNull<T>`, `len: usize`, `cap: usize`
2. Implementar `push`, `pop`, `len`, `capacity`, `is_empty`
3. Implementar `grow` privado que doble la capacidad usando `alloc::realloc`
4. Implementar `Drop` que libere todos los elementos y la memoria
5. Agregar `get(&self, index: usize) -> Option<&T>`
6. Escribir mínimo 8 tests que cubran casos normales y límites

## ✅ Criterios de Aceptación

- [ ] Compila sin warnings
- [ ] `cargo test` pasa (≥ 8 tests)
- [ ] Cada `unsafe` tiene `// SAFETY:` con justificación
- [ ] `Drop` libera memoria sin double-free ni leaks
- [ ] `cargo clippy -- -D warnings` pasa limpio

## 💡 Pistas

- `NonNull::dangling()` es seguro para capacidad cero
- `std::alloc::Layout::array::<T>(n)` calcula el layout correcto
- `ptr.write(value)` inicializa sin leer el valor previo (importante para `MaybeUninit`)
- Valida con Miri si está disponible: `cargo +nightly miri test`
