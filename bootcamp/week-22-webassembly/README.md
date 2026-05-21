# 🦀 Semana 22: WebAssembly

## 📋 Información General

| Campo | Detalle |
|-------|---------|
| **Semana** | 22 de 25 |
| **Tema** | WebAssembly: `wasm-bindgen`, `wasm-pack`, Wasmtime |
| **Duración** | 4 horas |
| **Nivel** | Avanzado — Fase de Sistemas y Performance |
| **Requisitos** | Semana 21, conocimiento básico de JavaScript/TypeScript |

---

## 🎯 Objetivos de Aprendizaje

1. **Compilar** código Rust a WebAssembly (`.wasm`)
2. **Usar** `wasm-bindgen` para interoperar con JavaScript
3. **Empaquetar** con `wasm-pack` para npm
4. **Ejecutar** WASM desde el lado del host con Wasmtime
5. **Evitar** `std::thread` en WASM (usar `wasm-bindgen-futures` para async)
6. **Publicar** un paquete npm con `wasm-pack publish`

---

## 🛠️ Setup del entorno WASM

```bash
# Instalar target WASM
rustup target add wasm32-unknown-unknown

# Instalar wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Compilar un proyecto wasm
wasm-pack build --target web

# Compilar para Node.js
wasm-pack build --target nodejs
```

---

## ⚠️ Restricciones en WASM

- **`std::thread`** — no disponible en WASM; usar `wasm-bindgen-futures`
- **`println!`** — redirigir con `web_sys::console::log_1`
- **`std::time::Instant`** — usar `web_sys::Performance` en browser
- **`std::fs`** — no hay filesystem; usar APIs del host
