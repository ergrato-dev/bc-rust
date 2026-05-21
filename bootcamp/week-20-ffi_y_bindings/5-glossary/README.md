# Glosario — Semana 20: FFI y Language Bindings

| Término | Definición |
|---------|-----------|
| **FFI** | Foreign Function Interface — mecanismo para llamar código de otro lenguaje |
| **ABI** | Application Binary Interface — contrato de bajo nivel sobre cómo se llaman funciones |
| **`extern "C"`** | Indica que la función usa el ABI de C (sin name mangling de Rust) |
| **`#[no_mangle]`** | Evita que Rust cambie el nombre de la función al compilar |
| **`#[repr(C)]`** | Garantiza que el struct tiene el layout de memoria de C |
| **`CStr`** | Vista de una cadena C terminada en null (no owned) |
| **`CString`** | Cadena C terminada en null con ownership (owned) |
| **`c_int`, `c_char`** | Tipos C-compatible de `std::os::raw` / `libc` |
| **`Box::into_raw`** | Convierte un `Box<T>` en `*mut T` transfiriendo ownership al caller |
| **`Box::from_raw`** | Recupera el `Box<T>` de un `*mut T` — libera memoria al hacer drop |
| **`bindgen`** | Herramienta que genera bindings Rust desde headers C/C++ |
| **`cbindgen`** | Herramienta que genera headers C/C++ desde código Rust |
| **PyO3** | Crate para crear extensiones Python nativas en Rust |
| **maturin** | Build tool para compilar y publicar extensiones Python/Rust |
| **napi-rs** | Framework para crear addons Node.js nativos en Rust |
| **`cdylib`** | Tipo de crate que genera una librería dinámica (`.so`, `.dll`, `.dylib`) |
| **Name mangling** | Transformación del nombre de una función por el compilador (C++ y Rust lo hacen) |
| **Opaque type** | Tipo cuya estructura interna es oculta al caller de la API |
