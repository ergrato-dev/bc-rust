# Práctica 04 — Node.js Bindings con napi-rs

## 🎯 Objetivo
Crear un addon nativo Node.js en Rust usando `napi-rs`.

## 📋 Instrucciones

1. Implementar `#[napi] pub fn suma(a: f64, b: f64) -> f64`
2. Implementar `#[napi] pub fn a_mayusculas(s: String) -> String`
3. Implementar `#[napi] pub fn factorial(n: u32) -> u64`
4. Compilar con `napi build --platform` y probar desde Node.js

## 🛠️ Cómo compilar y probar

```bash
# Instalar CLI de napi-rs
npm install -g @napi-rs/cli

# Compilar el addon nativo
napi build --platform

# Probar desde Node.js
node -e "
const addon = require('./practice-04-napi-basico.node');
console.log(addon.suma(2, 3));          // 5
console.log(addon.aMayusculas('hola')); // HOLA
console.log(addon.factorial(10));       // 3628800
"
```

## ✅ Criterios de Aceptación

- [ ] `cargo check` pasa sin errores de compilación
- [ ] `napi build --platform` genera el `.node` correctamente
- [ ] Las 3 funciones son accesibles desde Node.js
- [ ] `cargo clippy -- -D warnings` pasa limpio

## 💡 Pistas

- `napi-build` en `build.rs` configura el linker automáticamente
- El macro `#[napi]` genera automáticamente los bindings de tipos
- Los tipos se convierten automáticamente: `i32` → `number`, `String` → `string`, etc.
