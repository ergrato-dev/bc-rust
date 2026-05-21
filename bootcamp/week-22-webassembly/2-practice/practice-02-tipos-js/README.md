# Práctica 02 — Tipos JS en WASM

## 🎯 Objetivo
Pasar arrays, strings y tipos complejos entre JavaScript y Rust/WASM.

## 📋 Instrucciones

1. Implementar funciones que operen sobre `&[f64]` y `Vec<f64>`
2. Retornar `bool` desde WASM
3. Trabajar con `Vec<String>` en la frontera WASM/JS
4. Probar cómo JS ve los tipos retornados

## 🛠️ Cómo probar

```bash
wasm-pack build --target nodejs
cargo test
```

## ✅ Criterios de Aceptación

- [ ] Arrays JS (`Float64Array`) se pasan a Rust correctamente
- [ ] Rust retorna arrays a JS
- [ ] `cargo test` pasa
