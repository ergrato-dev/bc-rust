# 📖 `std::mem` — Herramientas de Manipulación de Memoria

## El Módulo `std::mem`

El módulo `std::mem` contiene funciones para inspeccionar y manipular la representación en
memoria de los tipos de Rust. Algunas son completamente seguras; otras requieren `unsafe`.

```
┌────────────────────────────────────────────────────────────────────┐
│                    FUNCIONES DE std::mem                           │
├────────────────────────────────────────────────────────────────────┤
│                                                                    │
│  SEGURAS (safe)                   UNSAFE                           │
│  ┌──────────────────────────┐    ┌──────────────────────────────┐  │
│  │ size_of::<T>()           │    │ transmute::<A, B>(val)       │  │
│  │ align_of::<T>()          │    │ transmute_copy::<A, B>(&val) │  │
│  │ size_of_val(&val)        │    │                              │  │
│  │ align_of_val(&val)       │    │ MaybeUninit::assume_init()   │  │
│  │ replace(&mut t, new)     │    │ (método de MaybeUninit)      │  │
│  │ swap(&mut a, &mut b)     │    └──────────────────────────────┘  │
│  │ take(&mut t)             │                                      │
│  │ drop(val)                │    USO CON CUIDADO                   │
│  │ forget(val)              │    ┌──────────────────────────────┐  │
│  │ needs_drop::<T>()        │    │ forget — leak intencional    │  │
│  │ discriminant(&enum)      │    │ ManuallyDrop — skip Drop     │  │
│  └──────────────────────────┘    └──────────────────────────────┘  │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
```

---

## `size_of` y `align_of` — Inspección de Layout

```rust
use std::mem;

// Tamaño en bytes de un tipo (en tiempo de compilación)
assert_eq!(mem::size_of::<u8>(),   1);
assert_eq!(mem::size_of::<u16>(),  2);
assert_eq!(mem::size_of::<u32>(),  4);
assert_eq!(mem::size_of::<u64>(),  8);
assert_eq!(mem::size_of::<f32>(),  4);
assert_eq!(mem::size_of::<f64>(),  8);
assert_eq!(mem::size_of::<bool>(), 1);
assert_eq!(mem::size_of::<char>(), 4);   // Unicode scalar = u32
assert_eq!(mem::size_of::<usize>(), 8);  // 8 en x86_64

// Alineación requerida (en bytes, siempre potencia de 2)
assert_eq!(mem::align_of::<u8>(),  1);
assert_eq!(mem::align_of::<u32>(), 4);
assert_eq!(mem::align_of::<u64>(), 8);

// Structs: Rust puede agregar padding para satisfacer alineaciones
#[repr(C)]
struct NoPadding { a: u32, b: u32 }    // 8 bytes, sin padding

#[repr(C)]
struct ConPadding { a: u8, b: u32 }    // a:1 + padding:3 + b:4 = 8 bytes

println!("NoPadding: {} bytes", mem::size_of::<NoPadding>());  // 8
println!("ConPadding: {} bytes", mem::size_of::<ConPadding>()); // 8
```

### `size_of_val` y `align_of_val` — Para Tipos Dinámicos

```rust
use std::mem;

let x = 42_i32;
let s: &str = "hola";
let v: &[i32] = &[1, 2, 3];

// size_of_val funciona con DSTs (dynamically sized types)
println!("{}", mem::size_of_val(&x));  // 4 (i32)
println!("{}", mem::size_of_val(s));   // 4 (longitud en bytes del str)
println!("{}", mem::size_of_val(v));   // 12 (3 * 4 bytes)
```

---

## `mem::replace` — Intercambiar sin Clonar

```rust
use std::mem;

// replace(&mut old, new) → retorna old, escribe new en su lugar
let mut name = String::from("Alice");
let old_name = mem::replace(&mut name, String::from("Bob"));

assert_eq!(old_name, "Alice");
assert_eq!(name, "Bob");

// Útil en implementaciones de Drop y manipulación de structs
struct Config {
    values: Vec<String>,
}

impl Config {
    fn take_values(&mut self) -> Vec<String> {
        // Toma el Vec y deja un Vec vacío en su lugar sin clonar
        mem::replace(&mut self.values, Vec::new())
        // equivalente moderno: std::mem::take(&mut self.values)
    }
}
```

---

## `mem::swap` — Intercambio In-Place

```rust
use std::mem;

let mut a = 10_i32;
let mut b = 20_i32;

mem::swap(&mut a, &mut b);
assert_eq!(a, 20);
assert_eq!(b, 10);

// También funciona con tipos no Copy
let mut s1 = String::from("alfa");
let mut s2 = String::from("beta");
mem::swap(&mut s1, &mut s2);
assert_eq!(s1, "beta");
assert_eq!(s2, "alfa");
```

---

## `mem::forget` — Leak Intencional

`forget` toma ownership de un valor **sin ejecutar su Drop**. Produce un memory leak
intencional y es seguro desde el punto de vista de Rust:

```rust
use std::mem;

// Caso de uso: pasar ownership a código C que libera la memoria
let v = vec![1, 2, 3];
let ptr = v.as_ptr();
let len = v.len();

// Transferimos la memoria a C: Rust no debe liberar el buffer
mem::forget(v);  // el Vec no llama a dealloc al salir del scope

// Ahora `ptr` sigue siendo válido; C liberará la memoria
// unsafe { libc::free(ptr as *mut _); }  // (hipotético)
```

### ⚠️ Cuándo es Útil vs Cuándo es Peligroso

```rust
use std::mem;
use std::ptr::NonNull;
use std::alloc::{self, Layout};

// ÚTIL: Box::into_raw — se usa forget internamente
let boxed = Box::new(42_i32);
let raw = Box::into_raw(boxed);    // Box::into_raw usa forget internamente
// ahora somos responsables de liberar `raw`
// SAFETY: `raw` proviene de Box::into_raw, size y align correctos para i32.
let _ = unsafe { Box::from_raw(raw) };  // restaura el Drop

// PELIGROSO: olvidar un tipo que contiene recursos del SO (file handles, etc.)
// mem::forget(File::open("importante.log").unwrap());
// ← El archivo podría no cerrarse correctamente en todos los OS
```

---

## `mem::transmute` — Reinterpretación de Bits

`transmute` es la función **más peligrosa** de Rust: reinterpreta los bits de un tipo
como otro tipo. Requiere que ambos tipos tengan el mismo tamaño.

```rust
use std::mem;

// Convertir f32 a su representación IEEE 754 como u32
let f: f32 = 1.0_f32;
// SAFETY: f32 y u32 tienen el mismo tamaño (4 bytes) y la misma alineación.
// El patrón de bits de 1.0f32 es un u32 válido (0x3F800000).
let bits: u32 = unsafe { mem::transmute(f) };
assert_eq!(bits, 0x3F80_0000);

// Convertir enum a su discriminante
#[repr(u8)]
enum Color { Rojo = 0, Verde = 1, Azul = 2 }
let c = Color::Verde;
// SAFETY: Color tiene #[repr(u8)], por lo que su representación en memoria
// es exactamente un byte. El valor Verde=1 es un u8 válido.
let disc: u8 = unsafe { mem::transmute(c) };
assert_eq!(disc, 1);
```

### Alternativas más Seguras a `transmute`

```rust
// En vez de transmute para primitivos con mismo tamaño:
// ✅ Usar métodos de conversión tipados
let bits = f32::to_bits(1.0_f32);  // → u32, sin unsafe
let f = f32::from_bits(0x3F80_0000_u32);  // → f32, sin unsafe

// ✅ Para reinterpretar bytes → usar bytemuck (crate)
// bytemuck::cast::<f32, u32>(1.0)  // sin unsafe

// ✅ Para enums → usar as con #[repr(u8)]
#[repr(u8)]
enum Status { Ok = 0, Error = 1 }
let s = Status::Ok as u8;  // → 0, sin unsafe
```

---

## `MaybeUninit<T>` — Inicialización Diferida

`MaybeUninit<T>` es la forma correcta de declarar valores no inicializados (en lugar
de `transmute` de memoria basura):

```rust
use std::mem::MaybeUninit;

// Inicializar un array de i32 de forma diferida
let mut arr: [MaybeUninit<i32>; 5] = unsafe {
    // SAFETY: MaybeUninit no requiere inicialización;
    // assume_uninit_array crea un array de MaybeUninit no inicializados.
    MaybeUninit::uninit().assume_init()
};

// Inicializar elemento a elemento
for (i, elem) in arr.iter_mut().enumerate() {
    elem.write(i as i32 * 10);
}

// SAFETY: Inicializamos todos los elementos del array en el bucle anterior.
let arr: [i32; 5] = unsafe {
    // Reinterpretamos el array de MaybeUninit como array de i32 inicializados
    std::ptr::read(&arr as *const _ as *const [i32; 5])
};
assert_eq!(arr, [0, 10, 20, 30, 40]);
```

---

## `mem::discriminant` — Comparar Variantes de Enum

```rust
use std::mem;

#[derive(Debug)]
enum Mensaje {
    Texto(String),
    Numero(i32),
    Vacio,
}

let m1 = Mensaje::Texto("hola".to_string());
let m2 = Mensaje::Texto("mundo".to_string());
let m3 = Mensaje::Numero(42);

// Comparar la variante sin comparar el contenido
assert_eq!(mem::discriminant(&m1), mem::discriminant(&m2));  // misma variante
assert_ne!(mem::discriminant(&m1), mem::discriminant(&m3));  // variantes distintas
```

---

## `mem::needs_drop` — Optimización de Colecciones

```rust
use std::mem;

// Saber si un tipo tiene destructor (Drop impl) en tiempo de compilación
println!("{}", mem::needs_drop::<i32>());     // false — i32 no tiene Drop
println!("{}", mem::needs_drop::<String>());  // true — String libera memoria
println!("{}", mem::needs_drop::<Vec<i32>>()); // true — Vec libera memoria

// Útil para optimizar colecciones: si T: !needs_drop, podemos omitir
// el bucle de destrucción al limpiar un buffer
```

---

## Resumen de Funciones por Seguridad

| Función | `unsafe`? | Descripción breve |
|---------|-----------|-------------------|
| `size_of::<T>()` | No | Tamaño del tipo en bytes |
| `align_of::<T>()` | No | Alineación del tipo en bytes |
| `size_of_val(&v)` | No | Tamaño de un valor concreto |
| `replace(&mut a, b)` | No | Intercambia y retorna el viejo |
| `swap(&mut a, &mut b)` | No | Intercambia dos valores |
| `take(&mut a)` | No | Reemplaza con Default, retorna viejo |
| `forget(val)` | No | Evita Drop (puede causar leak) |
| `drop(val)` | No | Llama a Drop explícitamente |
| `discriminant(&e)` | No | Discriminante de enum |
| `needs_drop::<T>()` | No | ¿Tiene destructor? |
| `transmute(val)` | **Sí** | Reinterpreta bits — muy peligroso |
| `transmute_copy(&val)` | **Sí** | Copia bits como otro tipo |
| `MaybeUninit::assume_init()` | **Sí** | Asume que el valor está inicializado |
