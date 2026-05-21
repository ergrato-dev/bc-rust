# Glosario — Semana 22: WebAssembly

| Término | Definición |
|---------|-----------|
| **WebAssembly (WASM)** | Formato de código binario portátil que ejecuta en navegadores y entornos server-side |
| **wasm32-unknown-unknown** | Target de compilación Rust para WASM sin sistema operativo |
| **wasm32-wasi** | Target WASM con acceso al sistema de ficheros via WASI |
| **`wasm-bindgen`** | Crate y herramienta que genera bindings JS/WASM automáticamente |
| **`#[wasm_bindgen]`** | Atributo que marca funciones/structs para exportar a JavaScript |
| **`wasm-pack`** | Herramienta de build que compila, empaqueta y publica crates WASM |
| **`wasm-pack build`** | Comando que compila a WASM y genera el paquete npm |
| **WAT** | WebAssembly Text Format — representación textual legible de WASM |
| **JsValue** | Tipo en Rust que representa cualquier valor JavaScript |
| **`js-sys`** | Crate con bindings a la librería estándar de JavaScript |
| **`web-sys`** | Crate con bindings a las APIs del navegador (DOM, fetch, etc.) |
| **Wasmtime** | Runtime WASM embebible en aplicaciones Rust, C, Python, etc. |
| **Engine** | Componente de Wasmtime que compila módulos WASM |
| **Store** | Almacén de estado para instancias WASM en Wasmtime |
| **`wasm-bindgen-test`** | Crate para tests de código WASM (en navegador o Node.js) |
| **`wasm-pack test`** | Comando para ejecutar tests WASM con wasm-bindgen-test |
| **AOT / JIT** | Estrategias de compilación WASM: Ahead-of-Time vs Just-in-Time |
