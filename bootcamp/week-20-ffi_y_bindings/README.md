# 🦀 Semana 20: FFI y Language Bindings

## 📋 Información General

| Campo | Detalle |
|-------|---------|
| **Semana** | 20 de 25 |
| **Tema** | FFI: interop con C, Python (PyO3) y Node.js (napi-rs) |
| **Duración** | 4 horas |
| **Nivel** | Avanzado — Fase de Diseño de Librerías |
| **Requisitos** | Semana 19 (`unsafe` Rust), conocimiento básico de C |

---

## 🎯 Objetivos de Aprendizaje

1. **Llamar** a funciones C desde Rust usando `extern "C"` y `libc`
2. **Exportar** funciones Rust a C con `#[no_mangle]` y `extern "C"`
3. **Crear** una extensión Python usando PyO3 y `maturin`
4. **Crear** un addon Node.js usando `napi-rs` y `napi-build`
5. **Gestionar** ownership y lifetimes en la frontera ABI
6. **Usar** `#[repr(C)]` para structs interoperables

---

## 📚 Contenido

| Archivo | Tema |
|---------|------|
| [1-theory/README.md](1-theory/README.md) | Teoría completa de la semana |
| [2-practice/](2-practice/) | Prácticas y proyecto |
| [4-resources/RECURSOS.md](4-resources/RECURSOS.md) | Referencias y lecturas |
| [5-glossary/README.md](5-glossary/README.md) | Glosario de términos |

---

## 🚀 Setup Adicional

```bash
# Para PyO3 (Python bindings)
pip install maturin
maturin develop   # en el directorio de la práctica

# Para napi-rs (Node.js bindings)
npm install -g @napi-rs/cli
napi build --platform  # en el directorio de la práctica

# Para generar headers C desde Rust
cargo install cbindgen@0.27.0
cbindgen --config cbindgen.toml --output include/libreria.h
```

---

## ⚠️ Reglas FFI

- `#[repr(C)]` es **obligatorio** en toda struct que cruce la frontera ABI
- `#[no_mangle]` es **obligatorio** en toda función exportada a C
- Los strings en FFI usan `CStr`/`CString` — **nunca** `&str`/`String` directamente
- Documentar siempre quién es responsable de liberar la memoria
