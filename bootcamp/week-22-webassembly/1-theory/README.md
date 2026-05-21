# Semana 22 — Teoría: WebAssembly

> ⚠️ **TODO**: Completar con teoría completa (~180 líneas).

## Secciones pendientes

1. ¿Qué es WebAssembly? Diferencias con JavaScript
2. El target `wasm32-unknown-unknown`
3. `wasm-bindgen`: anotaciones y generación de bindings JS
4. Tipos en la frontera WASM/JS: primitivos, strings, `JsValue`
5. `wasm-pack`: flujo de build y targets (web, bundler, nodejs, no-modules)
6. Structs con `#[wasm_bindgen]` y estado
7. `web-sys` y `js-sys`: API del navegador desde Rust
8. Restricciones de WASM: sin threads, sin filesystem, sin panic-unwind
9. Wasmtime: ejecutar WASM desde el server-side
10. `wasm-bindgen-test` y testing del módulo WASM
