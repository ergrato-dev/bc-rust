# 📖 04 — Strings en FFI: `CStr`, `CString` y `*const c_char`

## El problema: Rust y C tienen modelos de strings incompatibles

```
┌────────────────────────────────────────────────────────────────────┐
│             COMPARACIÓN DE MODELOS DE STRING                       │
├────────────────────────────────────────────────────────────────────┤
│                                                                    │
│  RUST &str / String                    C  const char *             │
│  ──────────────────                    ────────────────────────    │
│  Longitud explícita (len field)        Terminado en null ('\0')    │
│  UTF-8 garantizado                     Bytes arbitrarios (no UTF-8)│
│  NO tiene null terminator              DEBE tener null terminator  │
│  Fat pointer (ptr + len)               Thin pointer solo           │
│                                                                    │
│  "hello" en Rust:  68 65 6C 6C 6F                                  │
│  "hello" en C:     68 65 6C 6C 6F 00  ← null terminator           │
│                                                                    │
│  ❌ Pasar &str.as_ptr() a C como si fuera const char* es UB        │
│     (C podría leer más allá del buffer buscando el null)           │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
```

---

## Los cuatro tipos del ecosistema FFI de strings

| Tipo | Módulo | Owned | Null term. | Uso |
|------|--------|-------|------------|-----|
| `&str` | core | No | No | Strings Rust normales |
| `String` | std | Sí | No | Strings Rust con heap |
| `&CStr` | `std::ffi` | No | Sí | Vista sobre string C |
| `CString` | `std::ffi` | Sí | Sí | String C con ownership |
| `*const c_char` | `std::os::raw` | No | Depende del caller | Frontera ABI |

---

## `CString` — crear strings para enviar a C

`CString` añade el byte nulo al final y garantiza que no hay bytes nulos en medio.

```rust
use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn puts(s: *const c_char) -> libc::c_int;
}

fn imprimir_en_c(mensaje: &str) {
    // CString::new falla si `mensaje` contiene bytes nulos internos.
    let c_string = CString::new(mensaje)
        .expect("el mensaje no debe contener bytes nulos");

    // ⚠️ CRÍTICO: el CString debe estar vivo mientras se use el puntero.
    // Guardar el puntero y dejar que CString droppee = dangling pointer.
    let ptr: *const c_char = c_string.as_ptr();

    // SAFETY: `ptr` apunta a una cadena C válida (terminada en null).
    // `c_string` sigue vivo en este scope, garantizando que el puntero
    // no es dangling durante la llamada a `puts`.
    unsafe { puts(ptr) };

    // c_string droppea aquí — después de que ya no usamos el puntero
}
```

### Métodos útiles de `CString`

```rust
let cs = CString::new("hola mundo").unwrap();

cs.as_ptr()          // *const c_char — válido mientras cs vive
cs.as_bytes()        // &[u8] — SIN el null terminator
cs.as_bytes_with_nul() // &[u8] — CON el null terminator
cs.into_raw()        // *mut c_char — transfiere ownership (requiere from_raw!)
cs.to_str()          // Result<&str, Utf8Error>
```

---

## `CStr` — interpretar strings recibidos de C

`CStr` es la vista (sin ownership) sobre una cadena C. Se usa cuando:
- C nos pasa un puntero a una cadena que C gestiona
- Queremos leer el contenido sin copiar

```rust
use std::ffi::CStr;
use std::os::raw::c_char;

/// Función exportada que recibe una cadena desde C.
///
/// # Safety (caller)
/// `name` debe ser un puntero no nulo a una cadena C válida (terminada en null)
/// que viva al menos durante la duración de esta llamada.
#[no_mangle]
pub unsafe extern "C" fn saludar(name: *const c_char) {
    if name.is_null() {
        eprintln!("Error: name es null");
        return;
    }

    // SAFETY: verificamos null arriba. El caller garantiza que `name`
    // apunta a una cadena C válida con null terminator.
    let c_str: &CStr = unsafe { CStr::from_ptr(name) };

    // to_str() falla si hay bytes no-UTF-8 (retorna Err)
    match c_str.to_str() {
        Ok(s) => println!("Hola, {}!", s),
        Err(_) => eprintln!("Error: la cadena no es UTF-8 válido"),
    }

    // También podemos usar to_string_lossy() que reemplaza bytes inválidos
    let s = c_str.to_string_lossy();
    println!("(lossy) Hola, {}!", s);
}
```

### `to_str()` vs `to_string_lossy()`

```rust
let c_str: &CStr = ...;

// to_str() — falla si hay bytes no-UTF-8
match c_str.to_str() {
    Ok(s)  => { /* &str válido */ },
    Err(e) => { /* manejo de error */ },
}

// to_string_lossy() — nunca falla, reemplaza bytes inválidos con U+FFFD (?)
let s: Cow<str> = c_str.to_string_lossy();
// Si la cadena era UTF-8 válido: Cow::Borrowed(&str) — sin copia
// Si no era UTF-8 válido:        Cow::Owned(String)  — con copia y reemplazo
```

---

## El peligro del dangling pointer con `CString::into_raw`

En ocasiones, necesitamos transferir ownership de una `CString` a C
(que la liberará con `free`). Para esto existe `into_raw` / `from_raw`:

```rust
use std::ffi::CString;
use std::os::raw::c_char;

/// Retorna una cadena que C debe liberar con `rust_free_string`.
#[no_mangle]
pub extern "C" fn get_version() -> *mut c_char {
    let version = CString::new("1.0.0").unwrap();
    version.into_raw() // Transfiere ownership — Rust no libera la memoria
}

/// DEBE llamarse exactamente una vez con un puntero de `get_version`.
///
/// # Safety (caller)
/// `ptr` debe haber sido retornado por `get_version` y no haberse liberado.
#[no_mangle]
pub unsafe extern "C" fn rust_free_string(ptr: *mut c_char) {
    if ptr.is_null() { return; }
    // SAFETY: `ptr` proviene de `CString::into_raw` en `get_version`.
    // Reconstruimos el CString para que droppee y libere la memoria.
    unsafe { drop(CString::from_raw(ptr)); }
}
```

> ⚠️ **Nunca** usar `libc::free` para liberar una cadena creada con `into_raw`.
> Siempre usar la función `rust_free_string` correspondiente.

---

## Strings de longitud conocida: `from_raw_parts` + `str::from_utf8`

Cuando C nos pasa un puntero + longitud (no null-terminated):

```rust
use std::os::raw::{c_char, c_ulong};

/// Procesa un buffer de bytes con longitud explícita.
///
/// # Safety (caller)
/// `data` debe apuntar a un buffer válido de al menos `len` bytes.
/// El buffer debe vivir durante toda la llamada.
#[no_mangle]
pub unsafe extern "C" fn procesar_buffer(
    data: *const c_char,
    len: c_ulong,
) -> c_ulong {
    if data.is_null() || len == 0 {
        return 0;
    }
    // SAFETY: el caller garantiza que `data` apunta a `len` bytes válidos.
    let bytes = unsafe { std::slice::from_raw_parts(data as *const u8, len as usize) };

    // Intentar interpretar como UTF-8
    match std::str::from_utf8(bytes) {
        Ok(s) => s.chars().count() as c_ulong,
        Err(_) => 0,
    }
}
```

---

## Tabla de conversiones: `&str` ↔ `*const c_char`

| Desde | Hacia | Código |
|-------|-------|--------|
| `&str` | `*const c_char` | `CString::new(s)?.as_ptr()` |
| `*const c_char` | `&str` | `CStr::from_ptr(ptr)?.to_str()?` |
| `String` | `*mut c_char` | `CString::new(s)?.into_raw()` |
| `*mut c_char` | `String` | `CString::from_raw(ptr).into_string()?` |
| `&[u8]` | `*const c_char` | Verificar ausencia de nulos, añadir `\0` |
| `*const c_char` + len | `&[u8]` | `slice::from_raw_parts(ptr, len)` |

---

## Errores frecuentes con strings en FFI

```rust
// ❌ ERROR 1: dangling pointer — CString dropeado antes de usarse
let ptr = {
    let cs = CString::new("hola").unwrap();
    cs.as_ptr() // cs droppea al salir del bloque → ptr es dangling!
};
unsafe { strlen(ptr) }; // UB

// ❌ ERROR 2: olvidar from_raw → memory leak
let raw = CString::new("hola").unwrap().into_raw();
// Si nunca llamamos CString::from_raw(raw), la memoria se pierde

// ❌ ERROR 3: as_ptr en una CString temporaria
unsafe { strlen(CString::new("hola").unwrap().as_ptr()) };
// La CString temporaria puede droppear antes de que strlen termine

// ✅ CORRECTO: variable con lifetime explícito
let cs = CString::new("hola").unwrap();
let len = unsafe { strlen(cs.as_ptr()) };
// cs droppea aquí, después de que ya no se usa el puntero
println!("len = {}", len);
```

---

## Interoperabilidad con Python: strings Unicode

PyO3 maneja automáticamente la conversión `str` ↔ Python str:

```rust
use pyo3::prelude::*;

#[pyfunction]
fn invertir(s: &str) -> String {
    // PyO3 convierte Python str → &str automáticamente
    // Python str es Unicode (UTF-16/32 interno, UTF-8 en la interfaz)
    s.chars().rev().collect()
}
// La función retorna String → PyO3 la convierte a Python str automáticamente
```

Para `bytes` de Python (bytes object):

```rust
#[pyfunction]
fn hash_bytes(data: &[u8]) -> u64 {
    // PyO3 convierte Python bytes → &[u8] automáticamente
    data.iter().fold(0u64, |acc, &b| acc.wrapping_add(b as u64))
}
```
