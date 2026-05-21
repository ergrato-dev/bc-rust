# Práctica 01 — `wasm-bindgen` Básico

## 🎯 Objetivo
Exponer funciones Rust a JavaScript usando `#[wasm_bindgen]` y compilar a WASM.

## 📋 Instrucciones

1. Implementar `greet(name: &str) -> String` con `#[wasm_bindgen]`
2. Implementar `suma(a: i32, b: i32) -> i32`
3. Implementar `invertir(s: &str) -> String`
4. Compilar con `wasm-pack build --target nodejs`
5. Probar desde Node.js

## 🛠️ Cómo compilar y probar

```bash
# Compilar para Node.js
wasm-pack build --target nodejs

# Probar desde Node.js
node -e "
const wasm = require('./pkg/practice_01_wasm_bindgen.js');
console.log(wasm.greet('Mundo'));     // ¡Hola, Mundo!
console.log(wasm.suma(2, 3));         // 5
console.log(wasm.invertir('Rust'));   // tsuR
"

# Ejecutar tests nativos (no WASM)
cargo test

# Ejecutar tests WASM
wasm-pack test --node
```

## ✅ Criterios de Aceptación

- [ ] `wasm-pack build --target nodejs` sin errores
- [ ] Las 3 funciones accesibles desde Node.js
- [ ] `cargo test` pasa
- [ ] `cargo clippy -- -D warnings` pasa limpio
