# Práctica 02 — Exportar Rust a C

## 🎯 Objetivo
Crear funciones Rust que puedan ser llamadas desde código C, usando `#[no_mangle]`, `#[repr(C)]` y gestión explícita de ownership.

## 📋 Instrucciones

1. Definir `#[repr(C)] pub struct Point { x: f64, y: f64 }`
2. Implementar `point_new`, `point_free`, `point_distance` con `#[no_mangle]`
3. Usar el patrón `Box::into_raw` / `Box::from_raw` para gestionar el heap
4. Manejar punteros nulos devolviendo valores de error (sin `panic!`)
5. (Opcional) Generar el header C con `cbindgen`

## ✅ Criterios de Aceptación

- [ ] `#[repr(C)]` en toda struct exportada
- [ ] `#[no_mangle] pub extern "C" fn` en toda función exportada
- [ ] Punteros nulos manejados con retorno de error
- [ ] `// SAFETY:` en todos los bloques unsafe
- [ ] `cargo test` pasa (≥ 3 tests)
- [ ] `cargo clippy -- -D warnings` pasa limpio

## 💡 Pistas

- `Box::into_raw(Box::new(val))` retorna `*mut T` — la forma idiomática de heap allocation para FFI
- `Box::from_raw(ptr)` recupera el Box y lo dropea al salir de scope
- `cbindgen`: `cargo install cbindgen@0.27.0 && cbindgen --config cbindgen.toml -o include/lib.h`
