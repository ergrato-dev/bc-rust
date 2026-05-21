# Semana 20 — Teoría: FFI y Language Bindings

> ⚠️ **TODO**: Completar con teoría completa (~180 líneas).

## Secciones pendientes

1. ¿Qué es FFI y cuándo usarlo?
2. ABI de C y `extern "C"`
3. Tipos C-compatibles: `#[repr(C)]`, `c_int`, `c_char`, `c_void`
4. Llamar a C desde Rust: `libc`, `bindgen`
5. Exportar Rust a C: `#[no_mangle]`, headers con `cbindgen`
6. Strings en FFI: `CStr`, `CString`, `*const c_char`
7. Ownership en la frontera: `Box::into_raw` / `Box::from_raw`
8. Manejo de errores en FFI (sin panic)
9. Python bindings con PyO3 y maturin
10. Node.js bindings con napi-rs
