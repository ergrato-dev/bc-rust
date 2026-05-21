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

| Archivo | Tema | Diagrama |
|---------|------|----------|
| [1-theory/01-ffi-intro.md](1-theory/01-ffi-intro.md) | ¿Qué es FFI? ABI de C, name mangling | [SVG](0-assets/01-ffi-intro.svg) |
| [1-theory/02-c-desde-rust.md](1-theory/02-c-desde-rust.md) | `extern "C"`, `libc`, `bindgen`, wrappers | [SVG](0-assets/02-c-desde-rust.svg) |
| [1-theory/03-rust-hacia-c.md](1-theory/03-rust-hacia-c.md) | `#[no_mangle]`, `#[repr(C)]`, `cbindgen` | [SVG](0-assets/03-rust-hacia-c.svg) |
| [1-theory/04-strings-ffi.md](1-theory/04-strings-ffi.md) | `CStr`, `CString`, peligros de strings | [SVG](0-assets/04-strings-ffi.svg) |
| [1-theory/05-pyo3-napi.md](1-theory/05-pyo3-napi.md) | PyO3 + maturin, napi-rs, async | [SVG](0-assets/05-pyo3-napi.svg) |
| [2-practice/](2-practice/) | Prácticas y proyecto integrador | — |
| [4-resources/RECURSOS.md](4-resources/RECURSOS.md) | Referencias y lecturas | — |
| [5-glossary/README.md](5-glossary/README.md) | Glosario de términos | — |

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

## 🧪 Prácticas

| # | Práctica | Descripción | Tiempo |
|---|----------|-------------|--------|
| 1 | [practice-01-c-desde-rust](2-practice/practice-01-c-desde-rust/) | Llamar `strlen`, `abs`, `malloc` desde Rust | 30 min |
| 2 | [practice-02-rust-hacia-c](2-practice/practice-02-rust-hacia-c/) | Exportar `Point` con `#[no_mangle]` y `#[repr(C)]` | 30 min |
| 3 | [practice-03-pyo3-basico](2-practice/practice-03-pyo3-basico/) | `#[pyfunction]`, `#[pyclass]`, `#[pymodule]` | 30 min |
| 4 | [practice-04-napi-basico](2-practice/practice-04-napi-basico/) | `#[napi]` functions y clase para Node.js | 30 min |
| P | [project-libreria-ffi](2-practice/project-libreria-ffi/) | Mini motor de estadísticas con API C completa | 90 min |

---

## 🚀 Cómo Ejecutar

```bash
# Prácticas C puras (no requieren setup adicional)
cargo test -p practice-01-c-desde-rust
cargo test -p practice-02-rust-hacia-c
cargo test -p project-libreria-ffi

# Práctica PyO3 (requiere Python + maturin)
cd 2-practice/practice-03-pyo3-basico
maturin develop
python3 -c "import practice_03_pyo3_basico as m; print(m.suma(3, 4))"

# Práctica napi-rs (requiere Node.js)
cd 2-practice/practice-04-napi-basico
napi build --platform
node -e "const m = require('./index.node'); console.log(m.suma(3, 4))"
```

---

## ⚠️ Reglas FFI

- `#[repr(C)]` es **obligatorio** en toda struct que cruce la frontera ABI
- `#[no_mangle]` es **obligatorio** en toda función exportada a C
- Los strings en FFI usan `CStr`/`CString` — **nunca** `&str`/`String` directamente
- Documentar siempre quién es responsable de liberar la memoria
