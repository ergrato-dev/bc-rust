# 📖 03 — Exportar Rust a C

## Anatomía de una función exportada

Para que código C pueda llamar a una función Rust, se necesitan tres ingredientes:

```rust
// 1. #[no_mangle]   — deshabilita el name mangling de Rust
// 2. pub            — visibilidad pública
// 3. extern "C"     — ABI de C (convención de llamada)

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Sin `#[no_mangle]`, el símbolo en el binario sería algo como
`_ZN8mi_crate3add17h3f9a1b2c4d5e6f7gE`, imposible de llamar desde C.

---

## Tipos de crate para exportar

La clave está en el campo `crate-type` de `Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib"]   # librería dinámica (.so / .dll / .dylib)
# o
crate-type = ["staticlib"] # librería estática (.a / .lib)
# o ambos
crate-type = ["cdylib", "rlib"]  # dynamic + rlib para tests de Rust
```

```
┌──────────────────────────────────────────────────────────────────┐
│                 TIPOS DE LIBRERÍA                                │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  cdylib   → .so (Linux) / .dll (Windows) / .dylib (macOS)       │
│             Linkado dinámico — cargado en runtime                 │
│             ✅ Python extensions, Node.js addons                  │
│                                                                  │
│  staticlib → .a (Unix) / .lib (Windows)                          │
│             Linkado estático — incluido en el binario final       │
│             ✅ Embeber Rust en proyectos C/C++ grandes            │
│                                                                  │
│  rlib     → formato interno de Rust                              │
│             ✅ Dependencias entre crates de Rust                  │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

---

## `#[repr(C)]` en structs — obligatorio en la frontera ABI

Por defecto, Rust puede reordenar los campos de un struct para optimizar
el uso de memoria. Esto rompe cualquier código C que asuma un layout fijo.
`#[repr(C)]` garantiza que el orden y padding son idénticos a los de C.

```rust
// ❌ Sin #[repr(C)]: el compilador Rust puede reordenar campos
pub struct Punto {
    pub x: f32,  // podría quedar en posición y de C
    pub y: f32,
}

// ✅ Con #[repr(C)]: layout idéntico al struct C equivalente
#[repr(C)]
pub struct Punto {
    pub x: f32,  // siempre offset 0
    pub y: f32,  // siempre offset 4
}
```

```c
/* Equivalente en C */
typedef struct {
    float x;  /* offset 0 */
    float y;  /* offset 4 */
} Punto;
```

**Reglas para `#[repr(C)]`:**
- Toda struct pasada por valor a/desde C
- Toda struct accedida a través de puntero desde C
- Enums que se mapean a enteros C (`#[repr(C)]` o `#[repr(i32)]`)

---

## Patrón de ownership en FFI: `Box::into_raw` / `Box::from_raw`

Este es el patrón más importante para gestionar objetos Rust desde C.
El ciclo de vida es:

```
Rust crea el objeto     → Box::into_raw     → *mut T (C lo almacena)
C usa el puntero opaco  → funciones Rust     → acceden vía &mut *ptr
C ya no necesita el obj → función_free(ptr)  → Box::from_raw → drop
```

```rust
use std::os::raw::c_char;
use std::ffi::CStr;

pub struct Motor {
    velocidad: f64,
    nombre: String,
}

/// Crea un Motor. El caller DEBE liberarlo con `motor_free`.
///
/// # Safety (caller)
/// `nombre` debe apuntar a una cadena C válida terminada en null.
#[no_mangle]
pub unsafe extern "C" fn motor_new(nombre: *const c_char, vel: f64) -> *mut Motor {
    if nombre.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: verificamos null arriba. El caller garantiza cadena C válida.
    let nombre_str = match CStr::from_ptr(nombre).to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return std::ptr::null_mut(),
    };
    Box::into_raw(Box::new(Motor {
        velocidad: vel,
        nombre: nombre_str,
    }))
}

/// Retorna la velocidad del Motor.
/// Retorna -1.0 si el puntero es nulo.
#[no_mangle]
pub extern "C" fn motor_velocidad(ptr: *const Motor) -> f64 {
    if ptr.is_null() { return -1.0; }
    // SAFETY: verificamos null arriba. El caller garantiza puntero válido.
    unsafe { (*ptr).velocidad }
}

/// Libera el Motor. Llamar exactamente una vez.
///
/// # Safety (caller)
/// `ptr` debe haber sido creado por `motor_new` y no haberse liberado antes.
#[no_mangle]
pub extern "C" fn motor_free(ptr: *mut Motor) {
    if ptr.is_null() { return; }
    // SAFETY: `ptr` proviene de `Box::into_raw` en `motor_new`.
    // La API garantiza que se llama exactamente una vez.
    unsafe { drop(Box::from_raw(ptr)); }
}
```

---

## Generación automática de headers con `cbindgen`

`cbindgen` lee el código Rust y genera el header C correspondiente:

```bash
# Instalar
cargo install cbindgen@0.27.0

# cbindgen.toml (en la raíz del crate)
cat > cbindgen.toml << 'EOF'
language = "C"
include_guard = "MI_LIBRERIA_H"

[parse]
parse_deps = false
EOF

# Generar
cbindgen --config cbindgen.toml --crate mi-libreria --output include/mi_libreria.h
```

Resultado generado:

```c
/* Generado automáticamente por cbindgen */
#ifndef MI_LIBRERIA_H
#define MI_LIBRERIA_H

#include <stdint.h>

typedef struct Motor Motor;

Motor *motor_new(const char *nombre, double vel);
double motor_velocidad(const Motor *ptr);
void motor_free(Motor *ptr);

#endif /* MI_LIBRERIA_H */
```

---

## Manejo de errores: sin `panic!` en funciones exportadas

Un `panic!` que cruza la frontera FFI produce **comportamiento indefinido** (UB).
La solución es capturar el panic con `std::panic::catch_unwind`:

```rust
use std::panic;

/// Versión segura ante panics para exportar a C.
#[no_mangle]
pub extern "C" fn operacion_segura(input: i32) -> i32 {
    match panic::catch_unwind(|| {
        // Aquí puede haber código que potencialmente paniquea
        calcular(input)
    }) {
        Ok(resultado) => resultado,
        Err(_) => -1, // Código de error que C puede interpretar
    }
}
```

**Alternativa preferida**: diseñar funciones que no puedan paniquear.
Validar los inputs al principio y retornar códigos de error.

---

## Códigos de error en FFI: el patrón C

```rust
/// Códigos de error que C puede interpretar.
#[repr(C)]
pub enum FfiResult {
    Ok          = 0,
    NullPointer = 1,
    InvalidArg  = 2,
    OutOfMemory = 3,
}

#[no_mangle]
pub extern "C" fn procesar(ptr: *mut Motor, factor: f64) -> FfiResult {
    if ptr.is_null() {
        return FfiResult::NullPointer;
    }
    if factor <= 0.0 {
        return FfiResult::InvalidArg;
    }
    // SAFETY: verificamos null y validez del factor arriba.
    unsafe { (*ptr).velocidad *= factor; }
    FfiResult::Ok
}
```

---

## Variables estáticas y thread safety en FFI

Evitar `static mut` — preferir `Mutex` o `OnceLock`:

```rust
use std::sync::OnceLock;

static CONFIGURACION: OnceLock<String> = OnceLock::new();

#[no_mangle]
pub extern "C" fn init_config(valor: *const std::os::raw::c_char) -> bool {
    if valor.is_null() { return false; }
    // SAFETY: verificamos null arriba.
    let s = unsafe { std::ffi::CStr::from_ptr(valor) };
    let texto = match s.to_str() {
        Ok(t) => t.to_string(),
        Err(_) => return false,
    };
    CONFIGURACION.set(texto).is_ok()
}
```

---

## Comparación: exportar Rust vs exportar C++

| Aspecto | Rust | C++ |
|---------|------|-----|
| ABI estable | `extern "C"` forzado | `extern "C"` opcional |
| Name mangling | deshabilitado con `#[no_mangle]` | deshabilitado con `extern "C"` |
| Excepciones | `catch_unwind` explícito | pueden cruzar ABI (UB) |
| Tipos genéricos en ABI | No permitido | No permitido |
| Documentación de safety | `// SAFETY:` explícito | No enforced |

---

## Ejemplo de Makefile para compilar y enlazar

```makefile
# Compilar la librería Rust como .so
.PHONY: build
build:
	cargo build --release
	cp target/release/libmi_libreria.so .

# Compilar el programa C que la usa
test_c: main.c libmi_libreria.so
	gcc -o test_c main.c -L. -lmi_libreria -Iinclude -Wl,-rpath,.

# Ejecutar
run: test_c
	./test_c
```
