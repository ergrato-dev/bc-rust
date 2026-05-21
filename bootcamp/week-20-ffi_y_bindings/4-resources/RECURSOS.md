# Recursos — Semana 20: FFI y Language Bindings

## 📖 Lectura Obligatoria

- [The Rustonomicon: FFI](https://doc.rust-lang.org/nomicon/ffi.html)
- [PyO3 User Guide](https://pyo3.rs/)
- [napi-rs Documentation](https://napi.rs/)
- [cbindgen: Generating C/C++ Headers](https://github.com/mozilla/cbindgen)

## 🛠️ Herramientas

```bash
# Generar bindings C desde código C existente
cargo install bindgen-cli

# Generar headers C desde Rust
cargo install cbindgen@0.27.0

# Compilar extensiones Python
pip install maturin

# Compilar addons Node.js
npm install -g @napi-rs/cli
```

## 📚 Artículos

- [Calling C from Rust](https://doc.rust-lang.org/std/ffi/index.html)
- [Rust FFI Guide](https://michael-f-bryan.github.io/rust-ffi-guide/)
- [PyO3: From Zero to Python Extension](https://pyo3.rs/v0.23.0/getting-started)
