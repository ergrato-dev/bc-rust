# 📖 Introducción a `unsafe` Rust — Los 5 Superpoderes

## ¿Qué es `unsafe` en Rust?

Rust garantiza **seguridad de memoria** en tiempo de compilación. Sin embargo, existen operaciones
que el compilador no puede verificar estáticamente pero que son correctas si el programador
cumple ciertos invariantes. La palabra clave `unsafe` le dice al compilador:
*"yo me hago responsable de esta operación"*.

```
┌─────────────────────────────────────────────────────────────────┐
│                  SAFE RUST vs UNSAFE RUST                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Safe Rust                        Unsafe Rust                   │
│  ┌─────────────────────────┐     ┌─────────────────────────┐   │
│  │  El compilador verifica │     │  El programador certifica│   │
│  │  cada invariante        │     │  los invariantes         │   │
│  │                         │     │                          │   │
│  │  Borrow checker         │     │  // SAFETY: comentario   │   │
│  │  Lifetime analysis      │     │  unsafe { ... }          │   │
│  │  Aliasing rules         │     │                          │   │
│  │  Null-safety            │     │  5 operaciones extra     │   │
│  └─────────────────────────┘     └─────────────────────────┘   │
│                                                                 │
│  ❌ Usar unsafe no desactiva el borrow checker                   │
│  ✅ Unsafe solo habilita 5 capacidades adicionales               │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Los 5 Superpoderes de `unsafe`

`unsafe` en Rust habilita exactamente **5 operaciones** que safe Rust prohíbe:

| # | Superpoder | Descripción |
|---|-----------|-------------|
| 1 | **Desreferenciar raw pointers** | `*const T` y `*mut T` |
| 2 | **Llamar funciones `unsafe`** | funciones con precondiciones manuales |
| 3 | **Acceder/modificar variables estáticas mutables** | `static mut` |
| 4 | **Implementar traits `unsafe`** | `unsafe impl Send for T {}` |
| 5 | **Acceder a campos de unions** | `union` C-compatible |

```rust
// Superpoder 1: desreferenciar raw pointer
let x = 42_i32;
let raw = &x as *const i32;
// SAFETY: `raw` apunta a `x`, que vive en este scope.
let val = unsafe { *raw };

// Superpoder 2: llamar función unsafe
// SAFETY: los punteros apuntan a slices disjuntos de longitud correcta.
unsafe { std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), n) };

// Superpoder 3: static mut (evitar en código nuevo — preferir Mutex)
static mut CONTADOR: u32 = 0;
// SAFETY: ejecutado en single-thread durante inicialización.
unsafe { CONTADOR += 1; }

// Superpoder 4: unsafe trait (ver tema 04)
unsafe impl Send for MiTipo {}

// Superpoder 5: union
union Bits { i: i32, b: [u8; 4] }
let u = Bits { i: 0x0102_0304 };
// SAFETY: accedemos al campo `b`, cuya representación es válida para [u8; 4].
let bytes = unsafe { u.b };
```

---

## Qué NO Cambia con `unsafe`

Es un error común creer que `unsafe` desactiva todas las verificaciones:

```rust
let mut v = vec![1, 2, 3];
let r = &v[0];      // referencia inmutable

unsafe {
    // ❌ Esto SIGUE siendo error de compilación —
    //    el borrow checker NO se desactiva con unsafe
    v.push(4);      // error: cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("{r}");
}
```

El borrow checker, el lifetime analysis y el type system siguen activos dentro de bloques `unsafe`.

---

## La Regla del SAFETY Comment

Todo bloque o función `unsafe` **debe** documentar por qué es correcto:

```rust
// ❌ NUNCA — unsafe sin justificación
unsafe { *ptr = 42; }

// ✅ SIEMPRE — comentario SAFETY: antes del bloque
// SAFETY: `ptr` fue creado con `Box::into_raw` en esta función.
// Tenemos ownership exclusivo; nadie más tiene acceso al puntero.
// La función se llama solo una vez por objeto.
unsafe { *ptr = 42; }
```

El comentario debe responder: **¿qué invariante garantiza que esto no es UB?**

---

## Comportamiento Indefinido (UB)

**Undefined Behavior** significa que el compilador puede generar **cualquier cosa**: crash, datos
corruptos, código que funcione el 99% del tiempo, o incluso código que compile y parezca
correcto pero falle en producción.

### Causas comunes de UB en Rust

```rust
// 1. Desreferenciar puntero nulo o colgante (dangling pointer)
let ptr: *const i32 = std::ptr::null();
unsafe { let _ = *ptr; }    // UB: null dereference

// 2. Aliasing mutable — dos &mut al mismo dato
let mut x = 0_i32;
let a: *mut i32 = &mut x;
let b: *mut i32 = &mut x;
unsafe {
    *a = 1;
    *b = 2;   // UB: aliasing mutable viola las reglas de Rust
}

// 3. Lectura de memoria no inicializada
let mut uninit: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
// INCORRECTO: leer sin inicializar es UB
// let val = unsafe { uninit.assume_init() };

// 4. transmute con tamaños distintos
// unsafe { std::mem::transmute::<u32, u64>(0) }  // UB: tamaños distintos

// 5. Romper las reglas de aliasing con punteros
```

### La Regla de Aliasing de Rust (Stacked Borrows)

En todo momento:
- Puede existir **cualquier número** de `&T` (referencias compartidas)
- O puede existir **exactamente una** `&mut T` (referencia exclusiva)
- Pero **nunca ambas al mismo tiempo**

Violar esta regla es UB aunque sea en código `unsafe`.

---

## Minimizar el Código `unsafe`

La estrategia correcta es **aislar** el unsafe en abstracciones seguras:

```rust
/// Versión segura pública — el usuario no necesita saber que hay unsafe adentro.
pub fn split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    assert!(mid <= len);
    let ptr = slice.as_mut_ptr();

    // SAFETY: `mid <= len` verificado arriba. Los dos rangos no se solapan:
    // [0, mid) y [mid, len) son disjuntos. Ambos apuntan a memoria válida
    // dentro del slice original.
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

**Principio**: hacer el bloque `unsafe` lo más pequeño posible y proveer una interfaz safe al exterior.

---

## Errores Comunes y Cómo Evitarlos

| Error | Causa | Solución |
|-------|-------|----------|
| Dangling pointer | Variable dropped antes del puntero | Verificar lifetimes explícitamente |
| Double-free | `Box::from_raw` dos veces | Un solo responsable del ownership |
| Aliasing mutable | Dos `*mut` al mismo dato | Usar `NonNull` + documentar exclusividad |
| UB en `transmute` | Tamaños distintos o valor inválido | Verificar con `assert_eq!(size_of, size_of)` |
| Olvidar null-check | Asumir puntero válido | Usar `NonNull<T>` o verificar `.is_null()` |

---

## Herramientas de Detección de UB

```bash
# Miri: detecta UB en tiempo de ejecución
cargo +nightly miri test

# AddressSanitizer: detecta accesos fuera de rango, leaks
RUSTFLAGS="-Z sanitizer=address" cargo +nightly test --target x86_64-unknown-linux-gnu

# ThreadSanitizer: detecta data races
RUSTFLAGS="-Z sanitizer=thread" cargo +nightly test --target x86_64-unknown-linux-gnu
```

Miri es la herramienta más importante: ejecuta el código en un intérprete que
verifica cada acceso a memoria contra el modelo de Stacked Borrows.

---

## Comparación con C/C++

| Aspecto | C/C++ | Rust safe | Rust unsafe |
|---------|-------|-----------|-------------|
| Buffer overflow | Posible en todo momento | Imposible | Posible con `*ptr.add(n)` |
| Null dereference | Posible en todo momento | Imposible | Posible sin `is_null()` |
| Double-free | Posible | Imposible | Posible sin discipline |
| Data race | Posible | Imposible | Posible sin `Mutex` |
| Alcance del riesgo | Todo el código | Ninguno | Solo bloques `unsafe` |

La ventaja de Rust es que **el riesgo está acotado**: solo en los bloques `unsafe`, que son
visibles en la revisión de código y auditables.
