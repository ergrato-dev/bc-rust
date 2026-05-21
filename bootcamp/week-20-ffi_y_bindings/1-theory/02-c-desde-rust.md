# 📖 02 — Llamar a C desde Rust

## El bloque `extern`

Para llamar a una función de C desde Rust, primero hay que declararla en un
bloque `extern "C"`. Rust confía ciegamente en esta declaración — si el tipo
no coincide con la realidad, el comportamiento es indefinido.

```rust
extern "C" {
    // Declaramos las funciones C que queremos usar.
    // Los tipos deben coincidir exactamente con el header de C.
    fn strlen(s: *const libc::c_char) -> libc::size_t;
    fn abs(n: libc::c_int) -> libc::c_int;
    fn malloc(size: libc::size_t) -> *mut libc::c_void;
    fn free(ptr: *mut libc::c_void);
}
```

Toda llamada a una función en un bloque `extern` es `unsafe` porque el
compilador no puede verificar las precondiciones de la función C.

---

## El crate `libc` — tipos portables

El crate `libc` expone los tipos C en forma portátil para cada plataforma:

```rust
use libc::{
    c_int,     // int  de C  (32-bit en todas las plataformas modernas)
    c_char,    // char de C  (i8 en la mayoría)
    c_void,    // void de C  (para punteros opacos)
    size_t,    // size_t de C (usize en la mayoría)
    c_double,  // double de C (f64)
    c_float,   // float  de C (f32)
};
```

**¿Por qué no usar `i32` directamente?**

```
┌──────────────────────────────────────────────────────────────────┐
│              TIPOS C vs TIPOS RUST                               │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Tipo C    │  Rust portátil │  Rust directo │  Problema          │
│  ──────────┼────────────────┼───────────────┼──────────────────  │
│  int       │  c_int         │  i32          │  puede ser 16 bits │
│  long      │  c_long        │  i64/i32      │  platform-depend.  │
│  size_t    │  size_t        │  usize        │  tamaño varía      │
│  char      │  c_char        │  i8 o u8      │  signedness varía  │
│                                                                  │
│  ✅ Usar libc::c_int garantiza compatibilidad en cualquier       │
│     plataforma donde el crate libc esté disponible.             │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

---

## Patrón: wrapper seguro

El código `unsafe` debe encapsularse en funciones con API segura.
El módulo interno usa `unsafe`; el módulo público expone una API
que cumple las precondiciones antes de llamar al código C.

```rust
use std::ffi::CString;
use libc::{c_char, size_t};

extern "C" {
    fn strlen(s: *const c_char) -> size_t;
}

/// Calcula la longitud de una cadena UTF-8.
///
/// # Errores
/// Retorna `None` si la cadena contiene bytes nulos.
pub fn strlen_safe(s: &str) -> Option<usize> {
    // CString::new falla si hay bytes nulos — cumplimos la precondición de strlen.
    let c_string = CString::new(s).ok()?;
    // SAFETY: `c_string.as_ptr()` es un puntero válido a una cadena C terminada
    // en null. `c_string` vive hasta el fin de esta función, por lo que el
    // puntero es válido durante la llamada a `strlen`.
    let len = unsafe { strlen(c_string.as_ptr()) };
    Some(len)
}
```

**Regla de diseño**: la cantidad de código `unsafe` debe ser la mínima posible.
El wrapper seguro verifica las precondiciones para que el caller de Rust
no necesite pensar en ellas.

---

## Herramienta `bindgen` — generar bindings automáticamente

Para headers C grandes, escribir cada declaración a mano es tedioso y propenso
a errores. `bindgen` lee un header `.h` y genera los bloques `extern` de Rust:

```bash
# Instalar
cargo install bindgen-cli@0.71.1

# Generar bindings desde un header
bindgen include/mi_libreria.h -o src/bindings.rs

# Usar en build.rs (recomendado para librerías publicadas)
```

```rust
// build.rs — genera bindings en tiempo de compilación
fn main() {
    let bindings = bindgen::Builder::default()
        .header("include/mi_libreria.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out.join("bindings.rs")).unwrap();
    println!("cargo:rerun-if-changed=include/mi_libreria.h");
}
```

---

## Llamar a funciones con punteros de salida (out-params)

En C es común el patrón de "parámetro de salida" (out-param):

```c
// C: la función escribe el resultado en *out
int compute(int input, double *out);
```

```rust
extern "C" {
    fn compute(input: libc::c_int, out: *mut libc::c_double) -> libc::c_int;
}

pub fn compute_safe(input: i32) -> Result<f64, i32> {
    let mut result: f64 = 0.0;
    // SAFETY: `&mut result` es un puntero válido, alineado y exclusivo.
    // `compute` es una función C que sigue la convención de out-param documentada.
    let ret = unsafe { compute(input, &mut result as *mut f64) };
    if ret == 0 {
        Ok(result)
    } else {
        Err(ret)
    }
}
```

---

## Llamar a funciones con callbacks (punteros a función)

```c
// C: acepta un puntero a función como callback
void set_log_handler(void (*handler)(const char *message));
```

```rust
extern "C" {
    fn set_log_handler(handler: Option<extern "C" fn(*const libc::c_char)>);
}

// El callback debe ser `extern "C"` para que tenga el ABI correcto.
extern "C" fn my_log_handler(msg: *const libc::c_char) {
    if msg.is_null() { return; }
    // SAFETY: `msg` es una cadena C válida pasada por la librería C.
    let s = unsafe { std::ffi::CStr::from_ptr(msg) };
    eprintln!("[C LOG] {}", s.to_string_lossy());
}

pub fn init_logging() {
    // SAFETY: `my_log_handler` tiene el ABI C correcto y no captura nada.
    unsafe { set_log_handler(Some(my_log_handler)) };
}
```

---

## Variadic functions (`...`)

Rust puede llamar a funciones C variádicas (como `printf`):

```rust
extern "C" {
    // `...` en el bloque extern
    fn printf(fmt: *const libc::c_char, ...) -> libc::c_int;
}

pub fn c_print(s: &str) {
    // Preferir `puts` sobre `printf` para strings simples (sin %s)
    let msg = std::ffi::CString::new(s).expect("null byte");
    // SAFETY: llamamos printf con formato "%s\n" y un argumento c_str válido.
    unsafe { printf(b"%s\n\0".as_ptr() as *const libc::c_char, msg.as_ptr()) };
}
```

> ⚠️ Las funciones variádicas son inherentemente `unsafe`. Preferir wrappers
> específicos cuando sea posible.

---

## Gestión de memoria: `malloc`/`free` vs Box

```rust
use libc::{malloc, free, c_void, size_t};
use std::mem;

/// Asigna un buffer con malloc de C y lo llena con ceros.
pub fn c_alloc_zeros(len: usize) -> *mut u8 {
    let size = len * mem::size_of::<u8>();
    // SAFETY: `malloc` retorna un puntero alineado al tipo más estricto.
    // Verificamos que no sea nulo antes de usarlo.
    let ptr = unsafe { malloc(size as size_t) } as *mut u8;
    assert!(!ptr.is_null(), "malloc devolvió null (OOM)");
    // SAFETY: ptr es válido y tiene tamaño `len`. Escribir ceros es seguro.
    unsafe { std::ptr::write_bytes(ptr, 0, len); }
    ptr
}

/// SAFETY: `ptr` debe haber sido creado con `c_alloc_zeros` y no liberado antes.
pub unsafe fn c_free(ptr: *mut u8) {
    free(ptr as *mut c_void);
}
```

**Regla de oro**: nunca mezclar `malloc`/`free` de C con `Box` de Rust.
Si C asigna memoria, C debe liberarla. Si Rust la asigna, Rust la libera.

---

## Errores comunes al llamar C desde Rust

```rust
// ❌ ERROR: lifetime del CString demasiado corto
let ptr = CString::new("hola").unwrap().as_ptr(); // CString dropeado aquí!
unsafe { strlen(ptr) }; // UB: dangling pointer

// ✅ CORRECTO: CString vive mientras se usa el puntero
let c_str = CString::new("hola").unwrap();
let len = unsafe { strlen(c_str.as_ptr()) };

// ❌ ERROR: pasar &str directamente (no tiene null terminator)
// strlen(texto.as_ptr() as *const c_char) — puede leer más allá del buffer!

// ✅ CORRECTO: siempre convertir con CString
let c_str = CString::new(texto).unwrap();
let len = unsafe { strlen(c_str.as_ptr()) };
```

---

## Ejemplo completo: wrapper sobre `qsort` de C

```rust
use libc::{c_int, c_void, qsort};

/// Ordena un slice de i32 usando qsort de C.
pub fn sort_c(data: &mut [i32]) {
    extern "C" fn comparar(a: *const c_void, b: *const c_void) -> c_int {
        // SAFETY: qsort garantiza que a y b apuntan a elementos del array.
        let (a, b) = unsafe { (*(a as *const i32), *(b as *const i32)) };
        a.cmp(&b) as c_int
    }

    if data.is_empty() { return; }
    // SAFETY: `data.as_mut_ptr()` es válido, `data.len()` es correcto,
    // `size_of::<i32>()` es el tamaño de cada elemento, `comparar` tiene ABI C.
    unsafe {
        qsort(
            data.as_mut_ptr() as *mut c_void,
            data.len(),
            std::mem::size_of::<i32>(),
            Some(comparar),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordena_ints() {
        let mut v = vec![5, 3, 1, 4, 2];
        sort_c(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5]);
    }
}
```
