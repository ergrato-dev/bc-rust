# 📖 Funciones y Bloques `unsafe`

## Anatomía de `unsafe` en Rust

Hay tres formas de usar `unsafe` en Rust, cada una con semántica diferente:

```
┌──────────────────────────────────────────────────────────────────┐
│                  FORMAS DE unsafe EN RUST                        │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  1. Bloque unsafe                                                │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  // SAFETY: <justificación>                                │  │
│  │  unsafe { operacion_peligrosa(); }                         │  │
│  │  ↑ Contiene: las 5 operaciones unsafe                      │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
│  2. Función unsafe                                               │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  /// # Safety                                              │  │
│  │  /// <precondiciones para el caller>                       │  │
│  │  unsafe fn operacion(ptr: *mut T) { ... }                  │  │
│  │  ↑ El CALLER debe garantizar las precondiciones            │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
│  3. Bloque unsafe dentro de función unsafe                       │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  unsafe fn outer() {                                       │  │
│  │      // SAFETY: estamos en función unsafe, las             │  │
│  │      // precondiciones ya fueron garantizadas por el caller│  │
│  │      unsafe { ptr::write(dst, val); }                      │  │
│  │  }                                                         │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

---

## Funciones `unsafe fn`

Una función marcada `unsafe fn` tiene **precondiciones** que el compilador no puede verificar.
El caller se hace responsable de cumplirlas:

```rust
use std::ptr;

/// Intercambia dos valores en memoria usando raw pointers.
///
/// # Safety
/// El caller debe garantizar:
/// - `a` y `b` apuntan a valores inicializados del tipo `T`
/// - `a` y `b` son distintos (no se solapan en memoria)
/// - Ambos punteros son válidos para lectura y escritura
unsafe fn swap_raw<T>(a: *mut T, b: *mut T) {
    // SAFETY: El caller garantizó que ambos son válidos y distintos.
    // Usamos ptr::read para copiar sin invocar Drop del valor leído.
    let tmp: T = ptr::read(a);
    ptr::copy_nonoverlapping(b, a, 1);
    ptr::write(b, tmp);
}

fn main() {
    let mut x = 10_i32;
    let mut y = 20_i32;

    // SAFETY: `x` e `y` son variables distintas, válidas e inicializadas.
    unsafe { swap_raw(&mut x, &mut y); }

    assert_eq!(x, 20);
    assert_eq!(y, 10);
}
```

---

## Wrapper Seguro (Safe Abstraction)

El patrón más importante con funciones `unsafe` es crear una **safe abstraction**:
la función pública tiene interfaz segura pero usa `unsafe` internamente.

```rust
use std::ptr;

// Función pública SEGURA — interfaz sin unsafe para el usuario
pub fn swap<T>(a: &mut T, b: &mut T) {
    // Las referencias &mut garantizan exclusividad y validez.
    // SAFETY: Las referencias &mut garantizan que:
    // - ambos punteros son válidos y están inicializados
    // - son distintos porque el borrow checker impide alias de &mut
    unsafe { swap_raw(a as *mut T, b as *mut T); }
}

// Función interna unsafe con precondiciones manuales
unsafe fn swap_raw<T>(a: *mut T, b: *mut T) {
    // SAFETY: precondiciones documentadas y garantizadas por `swap`.
    let tmp: T = ptr::read(a);
    ptr::copy_nonoverlapping(b, a, 1);
    ptr::write(b, tmp);
}
```

Este patrón es el que usa la biblioteca estándar: `Vec`, `HashMap`, `String` y `Box`
usan `unsafe` internamente pero exponen interfaces completamente seguras.

---

## Documentar Precondiciones con `# Safety`

El contrato de una `unsafe fn` debe documentarse con la sección **`# Safety`** en rustdoc:

```rust
/// Crea una referencia a partir de un raw pointer.
///
/// # Safety
///
/// El caller debe garantizar:
/// - `ptr` no es nulo
/// - `ptr` está correctamente alineado para el tipo `T`
/// - `ptr` apunta a un valor inicializado y válido de tipo `T`
/// - La referencia devuelta no sobrevive al dato apuntado
/// - Durante la vida de la referencia devuelta, no existe ningún `&mut T`
///   al mismo dato (sin aliasing mutable)
pub unsafe fn ptr_to_ref<'a, T>(ptr: *const T) -> &'a T {
    // SAFETY: el caller garantizó todas las precondiciones arriba.
    &*ptr
}
```

---

## `extern "C"` — Funciones FFI son `unsafe`

Las funciones importadas desde otros lenguajes son siempre `unsafe fn`:

```rust
// Declarar función de la biblioteca C estándar
extern "C" {
    fn strlen(s: *const std::os::raw::c_char) -> usize;
    fn malloc(size: usize) -> *mut std::os::raw::c_void;
    fn free(ptr: *mut std::os::raw::c_void);
}

fn main() {
    let s = b"hola\0";  // C string con null terminator

    // SAFETY: `s.as_ptr()` apunta a un C string válido terminado en null.
    // La longitud retornada no incluye el null terminator.
    let len = unsafe { strlen(s.as_ptr() as *const std::os::raw::c_char) };
    println!("strlen = {len}");  // 4
}
```

---

## Funciones `unsafe` en Traits

Un trait puede declarar métodos que son `unsafe fn`. Implementar el método obliga
al implementador a respetar los invariantes:

```rust
/// Un tipo que puede ser copiado bitwise entre posiciones de memoria.
///
/// # Safety
/// Solo implementar si el tipo tiene inicializado todos sus bytes
/// (sin padding con bits indefinidos) y la copia bitwise es semánticamente válida.
pub unsafe trait BitwiseCopy: Sized {
    /// Copia `n` elementos desde `src` a `dst`.
    ///
    /// # Safety
    /// - `src` y `dst` apuntan a al menos `n` elementos del tipo
    /// - Los rangos no se solapan
    unsafe fn copy_to(src: *const Self, dst: *mut Self, n: usize) {
        std::ptr::copy_nonoverlapping(src, dst, n);
    }
}

// i32 es safe para copiar bitwise — todos sus bytes están definidos
// SAFETY: i32 no tiene padding y la copia bitwise es semánticamente correcta.
unsafe impl BitwiseCopy for i32 {}
```

---

## Errores Comunes al Escribir Funciones `unsafe`

### Error 1: Precondiciones incompletas

```rust
// ❌ Documentación insuficiente — ¿qué garantías necesita `ptr`?
/// # Safety
/// El puntero debe ser válido.
unsafe fn read_val(ptr: *const i32) -> i32 { *ptr }

// ✅ Precondiciones explícitas y completas
/// # Safety
/// - `ptr` no es nulo
/// - `ptr` está alineado a 4 bytes
/// - `ptr` apunta a un `i32` inicializado
/// - El `i32` apuntado no es modificado concurrentemente
unsafe fn read_val(ptr: *const i32) -> i32 { *ptr }
```

### Error 2: Unsafe demasiado grande

```rust
// ❌ Bloque unsafe engloba código safe innecesariamente
unsafe {
    let len = data.len();         // ← no necesita unsafe
    let sum: i32 = data.iter()    // ← no necesita unsafe
        .sum();
    *result_ptr = sum;            // ← este sí necesita unsafe
}

// ✅ Bloque unsafe mínimo — solo lo que realmente lo necesita
let sum: i32 = data.iter().sum();
// SAFETY: `result_ptr` fue obtenido del caller con garantía de validez.
unsafe { *result_ptr = sum; }
```

### Error 3: Propagar `unsafe` sin necesidad

```rust
// ❌ Hacer `unsafe fn` toda la función cuando solo una línea lo necesita
unsafe fn procesar(data: &[i32], ptr: *mut i32) {
    let sum: i32 = data.iter().sum();
    *ptr = sum;
}

// ✅ Función safe, bloque unsafe localizado
fn procesar(data: &[i32], ptr: *mut i32) {
    let sum: i32 = data.iter().sum();
    // SAFETY: el contrato de la función documentado arriba garantiza que
    // `ptr` es válido para escritura mientras esta función se ejecuta.
    unsafe { *ptr = sum; }
}
```

---

## Funciones de la stdlib que son `unsafe fn`

Conocer las funciones unsafe de la biblioteca estándar más usadas:

```rust
use std::{slice, str, ptr};

// Crear referencias desde punteros — requiere SAFETY muy específico
let raw = b"hello" as *const [u8; 5];
// SAFETY: `raw` apunta a un array de 5 bytes UTF-8 válido en este scope.
let s: &str = unsafe { str::from_utf8_unchecked(&*raw) };

// Crear slice desde puntero + longitud
let arr = [1_i32, 2, 3];
// SAFETY: `arr.as_ptr()` + len=3 cubre exactamente el array `arr`.
let sl: &[i32] = unsafe { slice::from_raw_parts(arr.as_ptr(), 3) };

// Inicializar MaybeUninit
let mut val: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
unsafe { val.as_mut_ptr().write(42); }
// SAFETY: acabamos de inicializar `val` con write() arriba.
let x: i32 = unsafe { val.assume_init() };
assert_eq!(x, 42);
```

---

## Checklist de Revisión de Código `unsafe`

Antes de hacer merge de cualquier bloque `unsafe`:

- [ ] Cada bloque tiene `// SAFETY:` con justificación completa
- [ ] Cada `unsafe fn` tiene sección `# Safety` en rustdoc
- [ ] Los invariantes cubren: validez, alineación, inicialización, aliasing, lifetime
- [ ] El bloque `unsafe` es lo más pequeño posible
- [ ] Existe al menos un test que ejercita el código `unsafe`
- [ ] Se probó con Miri: `cargo +nightly miri test`
- [ ] No hay `unwrap()` en código que podría fallar con UB
