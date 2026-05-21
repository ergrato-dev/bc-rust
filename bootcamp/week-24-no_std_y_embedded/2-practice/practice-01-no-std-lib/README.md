# Práctica 01 — Librería `no_std`

## 🎯 Objetivo
Crear una librería que funcione sin la librería estándar, usando solo `core::`.

## ⚠️ Restricciones

- `#![no_std]` — sin `Vec`, sin `String`, sin `println!`
- Solo `core::` disponible
- `panic = "abort"` en todos los perfiles

## 🛠️ Cómo ejecutar

```bash
# Tests (el runner usa std internamente pero el código no lo hace)
cargo test

# Verificar que compila para target bare metal
rustup target add thumbv7em-none-eabihf
cargo build --target thumbv7em-none-eabihf
```

## ✅ Criterios de Aceptación

- [ ] `#![no_std]` en lib.rs
- [ ] `cargo test` pasa
- [ ] Sin `use std::` en ningún lugar del código
