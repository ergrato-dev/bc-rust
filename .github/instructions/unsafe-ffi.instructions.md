---
applyTo: "**"
---

# Reglas de Unsafe y FFI — Bootcamp bc-rust

Estas reglas aplican a TODO el repositorio. Son especialmente relevantes en semanas 19-20.

## SAFETY comments — obligatorio en cualquier bloque unsafe

```rust
// ❌ NUNCA — unsafe sin justificación
unsafe { *ptr = 42; }

// ✅ SIEMPRE — justificar invariante antes del bloque
// SAFETY: `ptr` fue creado con `Box::into_raw` en esta misma función.
// La función toma ownership del puntero, garantizando uso único.
unsafe { *ptr = 42; }
```

## Reglas de raw pointers

1. **No desreferenciar punteros nulos**: verificar `!ptr.is_null()` antes de usar
2. **No crear referencias desde punteros potencialmente inválidos**
3. **No aliasing mutable**: nunca dos `&mut` al mismo dato simultáneamente
4. **Lifetime correcto**: el dato apuntado debe vivir más que el puntero

```rust
// ✅ Verificación antes de deref
fn safe_read(ptr: *const i32) -> Option<i32> {
    if ptr.is_null() {
        return None;
    }
    // SAFETY: verificamos que no es null arriba.
    Some(unsafe { *ptr })
}
```

## Reglas de FFI

### Tipos permitidos en la frontera ABI

```rust
// ✅ Tipos C-compatible
use std::os::raw::{c_int, c_char, c_void};

#[repr(C)]           // obligatorio para structs que cruzan ABI
pub struct Point {
    pub x: c_int,
    pub y: c_int,
}

// ✅ Función exportada a C
#[no_mangle]
pub extern "C" fn point_new(x: c_int, y: c_int) -> Point {
    Point { x, y }
}
```

### Strings en FFI

```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Recibir string desde C
pub extern "C" fn greet(name: *const c_char) {
    if name.is_null() { return; }
    // SAFETY: `name` no es null (verificado arriba) y C garantiza
    // que la cadena termina en null y es válida UTF-8 o ASCII.
    let s = unsafe { CStr::from_ptr(name) };
    if let Ok(s) = s.to_str() {
        println!("Hola, {}!", s);
    }
}
```

### Ownership en FFI

```rust
// ✅ Documentar claramente quién libera la memoria
/// Crea un Box en el heap de Rust y retorna un puntero opaco.
/// El caller DEBE liberar la memoria con `thing_free()`.
#[no_mangle]
pub extern "C" fn thing_new() -> *mut MyThing {
    Box::into_raw(Box::new(MyThing::default()))
}

/// Libera la memoria creada por `thing_new()`.
/// Llamar solo una vez. Comportamiento indefinido si se llama dos veces.
#[no_mangle]
pub extern "C" fn thing_free(ptr: *mut MyThing) {
    if ptr.is_null() { return; }
    // SAFETY: `ptr` proviene de `Box::into_raw` en `thing_new`.
    // El contrato de la API garantiza que se llama exactamente una vez.
    unsafe { drop(Box::from_raw(ptr)); }
}
```

## Checklist de revisión de código unsafe

Antes de mergear cualquier código con `unsafe`:

- [ ] Cada bloque tiene `// SAFETY:` con justificación completa
- [ ] Los invariantes están documentados en el rustdoc del tipo/función
- [ ] Existe al menos un test que ejercita el código unsafe
- [ ] El bloque unsafe es mínimo (no más código del necesario)
- [ ] Se probó con Miri si aplica: `cargo +nightly miri test`

## Send y Sync manuales

```rust
// Solo implementar Send/Sync manualmente cuando realmente es seguro
struct MyHandle(*mut libc::c_void);

// SAFETY: MyHandle es un wrapper sobre un handle opaco que puede
// transferirse entre threads según la documentación de la librería C.
unsafe impl Send for MyHandle {}
```

Documentar **siempre** por qué el tipo puede ser enviado/compartido entre threads.
