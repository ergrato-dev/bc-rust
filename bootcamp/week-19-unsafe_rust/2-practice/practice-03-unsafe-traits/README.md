# Práctica 03 — `unsafe` Traits: `Send` y `Sync`

## 🎯 Objetivo
Comprender cuándo y cómo implementar manualmente `Send` y `Sync`, documentando las garantías de seguridad.

## 📋 Instrucciones

1. Crear `struct AtomicHandle(*mut u8)` — un wrapper sobre puntero opaco
2. Implementar `unsafe impl Send for AtomicHandle` con justificación semántica
3. Implementar `unsafe impl Sync for AtomicHandle` con justificación semántica
4. Demostrar su uso transfiriendo el handle entre threads con `Arc<Mutex<AtomicHandle>>`
5. Crear `struct NotSend(std::rc::Rc<i32>)` y verificar que **no** puede implementar `Send`

## ✅ Criterios de Aceptación

- [ ] Compila sin warnings
- [ ] `cargo test` pasa
- [ ] Cada `unsafe impl` tiene comentario `// SAFETY:` que justifica la decisión
- [ ] El código demuestra que `Rc<T>` no es `Send` (test de compilación negativa o comentario)
- [ ] `cargo clippy -- -D warnings` pasa limpio

## 💡 Pistas

- `unsafe impl Send for T {}` solo se justifica cuando sabemos que T puede cruzar threads
- Si contiene `*mut T`, Rust automáticamente marca el tipo como `!Send`
- Usa `#[allow(dead_code)]` temporalmente si necesitas silenciar warnings de campos no usados
