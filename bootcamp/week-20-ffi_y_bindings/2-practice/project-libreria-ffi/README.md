# Proyecto — `libreria-ffi`: Motor de Estadísticas con API C

## 🎯 Objetivo
Implementar una librería Rust completa con API C pública: crear/destruir objetos opacos, operar sobre ellos y manejar errores sin usar `panic!`.

## 📋 Instrucciones

1. Implementar `dataset_new`, `dataset_free`, `dataset_push`, `dataset_mean`, `dataset_len`
2. Todos los tipos exportados tienen `#[repr(C)]`
3. Todas las funciones usan `#[no_mangle] pub extern "C" fn`
4. Errores retornados via `FfiError` enum (sin panic)
5. Documentar invariantes con `// SAFETY:` y `# Safety` en rustdoc
6. (Opcional) Generar `include/libreria_ffi.h` con `cbindgen`

## ✅ Criterios de Aceptación

- [ ] `cargo test` pasa (≥ 5 tests)
- [ ] Punteros nulos manejados sin panic en todas las funciones
- [ ] `// SAFETY:` en todos los bloques unsafe
- [ ] `cargo clippy -- -D warnings` pasa limpio
- [ ] No hay memory leaks (verificar con Valgrind o Miri)
