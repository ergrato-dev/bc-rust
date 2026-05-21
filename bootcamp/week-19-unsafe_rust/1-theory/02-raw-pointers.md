# 📖 Raw Pointers: `*const T` y `*mut T`

## ¿Qué son los Raw Pointers?

Los raw pointers son punteros sin ninguna garantía de seguridad: sin lifetime, sin garantía de
validez, sin protección contra aliasing. Son el equivalente directo de los punteros de C.

```
┌────────────────────────────────────────────────────────────────┐
│              COMPARACIÓN DE TIPOS DE PUNTERO                   │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  &T                      *const T                              │
│  ┌─────────────────┐     ┌──────────────────────────────────┐  │
│  │ • Lifetime OK   │     │ • Sin lifetime                   │  │
│  │ • Siempre válida│     │ • Puede ser nulo                 │  │
│  │ • No null       │     │ • Puede ser dangling             │  │
│  │ • Aliasing OK   │     │ • Aliasing sin verificar         │  │
│  │ • Solo lectura  │     │ • Solo lectura (por convención)  │  │
│  └─────────────────┘     └──────────────────────────────────┘  │
│                                                                │
│  &mut T                  *mut T                                │
│  ┌─────────────────┐     ┌──────────────────────────────────┐  │
│  │ • Exclusiva     │     │ • Sin exclusividad               │  │
│  │ • No null       │     │ • Puede ser nulo                 │  │
│  │ • Borrow check  │     │ • Sin borrow check               │  │
│  │ • Siempre válida│     │ • Sin garantía de validez        │  │
│  └─────────────────┘     └──────────────────────────────────┘  │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

## Crear Raw Pointers

Crear un raw pointer es **siempre seguro** — solo desreferenciarlo requiere `unsafe`:

```rust
let x = 42_i32;
let mut y = 99_i32;

// Crear desde referencias — SIEMPRE safe (no unsafe necesario)
let const_ptr: *const i32 = &x;             // de &T
let mut_ptr: *mut i32 = &mut y;             // de &mut T

// Desde puntero a array/slice
let v = vec![1_i32, 2, 3];
let slice_ptr: *const i32 = v.as_ptr();     // primer elemento

// Null pointer (tipo inferido por contexto)
let null: *const i32 = std::ptr::null();
let null_mut: *mut i32 = std::ptr::null_mut();

// NonNull — garantiza no-nulo; útil para APIs internas
use std::ptr::NonNull;
let nn: NonNull<i32> = NonNull::from(&x);
```

---

## Desreferenciar Raw Pointers

Desreferenciar un raw pointer requiere `unsafe` y un comentario `// SAFETY:`:

```rust
let x = 42_i32;
let ptr: *const i32 = &x;

// SAFETY: `ptr` apunta a `x`, que vive en el scope actual.
// Ninguna otra referencia mutable existe sobre `x` en este momento.
let val: i32 = unsafe { *ptr };
assert_eq!(val, 42);

// Modificar a través de *mut
let mut y = 0_i32;
let mut_ptr: *mut i32 = &mut y;

// SAFETY: `mut_ptr` apunta a `y`, que vive en este scope.
// Tenemos ownership exclusivo: no hay otras referencias a `y`.
unsafe { *mut_ptr = 100; }
assert_eq!(y, 100);
```

---

## Aritmética de Punteros

Los raw pointers soportan aritmética para moverse dentro de buffers contiguos:

```rust
let arr = [10_i32, 20, 30, 40, 50];
let base: *const i32 = arr.as_ptr();

// SAFETY: el índice 2 < arr.len() (5), por lo que base.add(2)
// apunta dentro del array `arr`, que está correctamente inicializado.
let tercero: i32 = unsafe { *base.add(2) };
assert_eq!(tercero, 30);

// Iterar manualmente (equivalente a slice iteration)
for i in 0..arr.len() {
    // SAFETY: `i < arr.len()`, por lo que `base.add(i)` está dentro de `arr`.
    let val = unsafe { *base.add(i) };
    println!("[{i}] = {val}");
}
```

### Diferencia entre `add` y `offset`

```rust
// `add` toma usize — más común y seguro de usar
ptr.add(n);    // ptr + n * size_of::<T>()

// `offset` toma isize — permite ir hacia atrás
ptr.offset(-1);  // ptr - size_of::<T>()

// `wrapping_add` nunca causa UB incluso si sobrepasa el buffer
// (pero el puntero resultante puede ser inválido)
ptr.wrapping_add(n);
```

---

## Verificación de Nulidad

Siempre verificar antes de desreferenciar un puntero potencialmente nulo:

```rust
/// Lee el valor en `ptr` si no es nulo.
///
/// # Safety
/// Si `ptr` no es null, debe apuntar a un `T` válido e inicializado.
pub fn safe_read<T: Copy>(ptr: *const T) -> Option<T> {
    if ptr.is_null() {
        return None;
    }
    // SAFETY: verificamos is_null() arriba; el caller garantiza validez
    // si el puntero no es null (ver doc de # Safety).
    Some(unsafe { *ptr })
}

fn main() {
    let x = 42_i32;
    let valid_ptr: *const i32 = &x;
    let null_ptr: *const i32 = std::ptr::null();

    assert_eq!(safe_read(valid_ptr), Some(42));
    assert_eq!(safe_read(null_ptr), None);
}
```

---

## `NonNull<T>` — La Alternativa Preferida

`NonNull<T>` es un raw pointer con garantía de no-nulidad. Preferirlo en APIs internas:

```rust
use std::ptr::NonNull;

struct Node<T> {
    value: T,
    next: Option<NonNull<Node<T>>>,  // nulidad modelada con Option, no raw null
}

impl<T> Node<T> {
    fn new(value: T) -> Box<Self> {
        Box::new(Node { value, next: None })
    }
}

// NonNull::new retorna Option<NonNull<T>>
let mut x = 42_i32;
let nn: Option<NonNull<i32>> = NonNull::new(&mut x as *mut i32);
assert!(nn.is_some());

let null_nn = NonNull::<i32>::new(std::ptr::null_mut());
assert!(null_nn.is_none());
```

### Ventajas de `NonNull<T>` sobre `*mut T`

| Característica | `*mut T` | `NonNull<T>` |
|----------------|----------|--------------|
| Puede ser nulo | Sí | No (garantía) |
| Covariance | Invariante | Covariant en `T` |
| Tamaño | 8 bytes | 8 bytes (igual) |
| `Option<NonNull<T>>` | — | Mismo tamaño que `*mut T` |

---

## `ptr::read` y `ptr::write` vs Desreferenciación Directa

Para operaciones en memoria no inicializada, usar las funciones del módulo `ptr`:

```rust
use std::ptr;

let src = 42_i32;
let mut dst: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();

// ptr::write: escribe sin leer el valor previo (correcto para MaybeUninit)
// SAFETY: `dst.as_mut_ptr()` apunta a memoria de tamaño correcto para i32.
// No estamos leyendo el valor no-inicializado previo.
unsafe { ptr::write(dst.as_mut_ptr(), src); }

// SAFETY: acabamos de inicializar `dst` con ptr::write arriba.
let val = unsafe { dst.assume_init() };
assert_eq!(val, 42);

// ptr::read: copia el valor SIN mover ownership (puede duplicar valores Drop)
let x = 100_i32;
// SAFETY: `&x` es una referencia válida; reading i32 (Copy) es seguro.
let copy: i32 = unsafe { ptr::read(&x) };
assert_eq!(copy, 100);
```

---

## `ptr::copy_nonoverlapping` — El `memcpy` de Rust

```rust
use std::ptr;

let src = [1_i32, 2, 3, 4, 5];
let mut dst = [0_i32; 5];

// SAFETY: `src` y `dst` tienen exactamente 5 elementos.
// Sus rangos en memoria no se solapan (arrays distintos en el stack).
unsafe {
    ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), 5);
}
assert_eq!(dst, [1, 2, 3, 4, 5]);

// Para rangos que SÍ se solapan, usar ptr::copy (equivale a memmove)
let mut buf = [1_i32, 2, 3, 4, 5];
let p = buf.as_mut_ptr();
// SAFETY: copiamos del índice 1 al índice 0, solapamiento permitido con ptr::copy.
unsafe { ptr::copy(p.add(1), p, 4); }
// buf = [2, 3, 4, 5, 5]
```

---

## Errores Comunes con Raw Pointers

```rust
// ❌ 1. Dangling pointer — el dato ya no existe
let raw;
{
    let x = 42_i32;
    raw = &x as *const i32;  // OK aquí
}   // x dropped aquí
// unsafe { *raw }  // ← UB: dangling pointer

// ❌ 2. Aliasing mutable — violar exclusividad
let mut x = 0_i32;
let a = &mut x as *mut i32;
let b = &mut x as *mut i32;
// unsafe { *a = 1; *b = 2; }  // UB: dos *mut al mismo dato

// ❌ 3. Puntero fuera del array
let arr = [1_i32, 2, 3];
let ptr = arr.as_ptr();
// unsafe { *ptr.add(5) }  // UB: out of bounds

// ❌ 4. Alineación incorrecta
let bytes = [0u8; 8];
let ptr = bytes.as_ptr() as *const u64;
// Puede ser UB en arquitecturas que requieren alineación de 8 bytes
// unsafe { *ptr }

// ✅ Correcto: verificar alineación con align_of
assert_eq!(ptr as usize % std::mem::align_of::<u64>(), 0);
```

---

## Cuándo Usar Raw Pointers

Raw pointers son necesarios en estos contextos:

1. **FFI**: interoperar con código C que pasa punteros
2. **Estructuras de datos de bajo nivel**: listas enlazadas, árboles con parent pointers
3. **Allocación manual**: implementar `Vec`, `Box`, arena allocators
4. **Rendimiento extremo**: evitar overhead de referencias en hot paths (con benchmarks que lo justifiquen)
5. **Código `no_std`**: donde las abstracciones seguras no están disponibles

En todos los demás casos, las referencias y smart pointers son la opción correcta.
