# Semana 20 — Teoría: FFI y Language Bindings

## Archivos de Teoría

| # | Archivo | Tema | Diagrama |
|---|---------|------|----------|
| 01 | [01-ffi-intro.md](01-ffi-intro.md) | ¿Qué es FFI? ABI de C, name mangling, cuándo usar | [SVG](../0-assets/01-ffi-intro.svg) |
| 02 | [02-c-desde-rust.md](02-c-desde-rust.md) | `extern "C"`, `libc`, `bindgen`, wrappers seguros | [SVG](../0-assets/02-c-desde-rust.svg) |
| 03 | [03-rust-hacia-c.md](03-rust-hacia-c.md) | `#[no_mangle]`, `#[repr(C)]`, `cbindgen`, ownership | [SVG](../0-assets/03-rust-hacia-c.svg) |
| 04 | [04-strings-ffi.md](04-strings-ffi.md) | `CStr`, `CString`, `*const c_char`, peligros | [SVG](../0-assets/04-strings-ffi.svg) |
| 05 | [05-pyo3-napi.md](05-pyo3-napi.md) | PyO3 + maturin, napi-rs, clases, async | [SVG](../0-assets/05-pyo3-napi.svg) |

---

## Mapa Conceptual

```
FFI y Language Bindings
│
├── ¿Qué es FFI?
│   ├── ABI de C — contrato de bajo nivel
│   ├── Name mangling — por qué #[no_mangle]
│   └── Cuándo usar FFI vs reescribir en Rust
│
├── Rust llama a C
│   ├── extern "C" { fn ... }
│   ├── libc — tipos portables (c_int, c_char, size_t)
│   ├── bindgen — generar bindings desde headers
│   └── Wrappers seguros — encapsular unsafe
│
├── C llama a Rust
│   ├── #[no_mangle] + pub extern "C" fn
│   ├── #[repr(C)] — layout de structs
│   ├── Box::into_raw / Box::from_raw — ownership
│   ├── cbindgen — generar headers
│   └── catch_unwind — no panic! en la frontera
│
├── Strings en FFI
│   ├── CString — Rust crea string para C
│   ├── CStr — Rust interpreta string de C
│   ├── into_raw / from_raw — transferir ownership
│   └── Peligros: dangling pointer, double-free
│
└── Language Bindings de Alto Nivel
    ├── PyO3 — #[pyfunction], #[pyclass], #[pymodule]
    ├── maturin — build y publish para Python
    ├── napi-rs — #[napi], async, clases para Node.js
    └── Comparativa: PyO3 vs napi-rs vs cbindgen
```

---

## Prerequisitos

- Semana 19: `unsafe` Rust, raw pointers, `// SAFETY:` comments
- Conceptos básicos de C (punteros, structs, compilación)
- Python o Node.js instalado para probar las prácticas de bindings
