# Práctica 03 — Structs con `#[wasm_bindgen]`

## 🎯 Objetivo
Exponer structs Rust con estado a JavaScript usando `#[wasm_bindgen]`.

## 📋 Instrucciones

1. Implementar `struct Contador` con `#[wasm_bindgen]`
2. Agregar `#[wasm_bindgen(constructor)]` para `new`
3. Métodos `valor`, `incrementar`, `decrementar`, `resetear`, `sumar`
4. Compilar y verificar que desde JS se crea con `new Contador(0)`

## 🛠️ Cómo probar

```bash
wasm-pack build --target nodejs
cargo test
```

## ✅ Criterios de Aceptación

- [ ] Struct accesible desde JS con `new`
- [ ] Estado persiste entre llamadas a métodos
- [ ] `cargo test` pasa
