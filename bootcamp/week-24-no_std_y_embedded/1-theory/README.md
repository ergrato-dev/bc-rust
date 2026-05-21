# Semana 24 — Teoría: `no_std` e Embedded

## 1. El ecosistema: `core`, `alloc`, `std`

La librería de Rust se divide en tres capas que se apilan una sobre la otra:

```
┌─────────────────────────────────────────┐
│   std  (sistema operativo requerido)    │
│   Vec, String, File, Thread, Mutex…     │
├─────────────────────────────────────────┤
│   alloc  (allocator requerido)          │
│   Box, Rc, Arc, Vec, String…            │
├─────────────────────────────────────────┤
│   core   (siempre disponible)           │
│   Option, Result, Iterator, fmt, mem…   │
└─────────────────────────────────────────┘
```

| Capa | Requiere | Disponible en |
|------|----------|---------------|
| `core` | Nada | Todo entorno |
| `alloc` | Allocator global | Embedded con heap |
| `std` | OS + allocator | Entornos con OS |

`#![no_std]` desactiva `std` automáticamente. La capa `core` siempre
está disponible. Si el hardware tiene heap y registras un allocator, puedes
añadir `alloc` con `extern crate alloc;`.

---

## 2. `#![no_std]` — qué se pierde y qué queda

El atributo `#![no_std]` va al inicio del crate raíz (`lib.rs` o `main.rs`):

```rust
#![no_std]

// core:: sigue disponible sin ningún import adicional
use core::fmt::Write;
use core::mem;
```

**Qué se pierde con `#![no_std]`:**

| Eliminado | Alternativa `no_std` |
|-----------|----------------------|
| `println!` / `print!` | `core::fmt::Write` + UART |
| `Vec<T>`, `String` | `heapless::Vec`, `heapless::String` |
| `HashMap`, `BTreeMap` | `heapless::FnvIndexMap` |
| `std::sync::Mutex` | `core::sync::atomic`, RTIC |
| `std::thread` | Interrupciones + RTIC |
| `Box<T>`, `Rc<T>` | Solo si hay `alloc` |
| `std::error::Error` | Trait custom o tipos concretos |
| `std::time` | HAL timers / `core::time::Duration` |

**Qué queda disponible:**

- `core::option::Option`, `core::result::Result`
- `core::iter` — iteradores completos
- `core::cmp`, `core::ops`, `core::convert`
- `core::fmt` — `Display`, `Debug`, `Write`
- `core::mem` — `size_of`, `align_of`, `swap`, `replace`
- `core::ptr` — `copy_nonoverlapping`, `write`, `read`
- `core::sync::atomic` — `AtomicBool`, `AtomicU32`, etc.

---

## 3. `core::fmt`, `core::mem`, `core::ptr`

### `core::fmt` — formateo sin heap

```rust
use core::fmt::Write;

// Implementar Write sobre un buffer fijo para simular println!
struct UartWriter;

impl Write for UartWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // Enviar s byte a byte por UART hardware
        Ok(())
    }
}

let mut uart = UartWriter;
core::write!(uart, "temp={:.2}", 36.6_f32).ok();
```

### `core::mem` — tamaños y alineación en compile-time

```rust
const U32_SIZE: usize = core::mem::size_of::<u32>();   // 4
const F64_ALIGN: usize = core::mem::align_of::<f64>(); // 8

// Intercambiar sin heap
let (mut a, mut b) = (10_i32, 20_i32);
core::mem::swap(&mut a, &mut b); // a=20, b=10
```

### `core::ptr` — raw pointers seguros

```rust
// SAFETY: src y dst son válidos, n bytes disponibles, sin solapamiento.
unsafe {
    core::ptr::copy_nonoverlapping(src, dst, n);
}
```

---

## 4. Panic handlers: `#[panic_handler]` y `panic = "abort"`

En `no_std`, Rust no sabe qué hacer al hacer `panic!`. Debes proveer
un handler manualmente:

```rust
use core::panic::PanicInfo;

#[panic_handler]
fn mi_panic(_info: &PanicInfo) -> ! {
    // En hardware real: resetear, encender LED de error, etc.
    loop {}  // Bucle infinito — detiene la ejecución
}
```

El tipo de retorno `!` indica que la función nunca retorna.

**Estrategias de panic para embedded:**

| Estrategia | `Cargo.toml` | Comportamiento |
|------------|-------------|----------------|
| `unwind` (defecto) | `panic = "unwind"` | Limpia el stack — requiere unwinding runtime |
| `abort` | `panic = "abort"` | Llama a `abort()` del OS — más pequeño |
| Custom handler | `#[panic_handler]` | Control total (loop, reset, LED…) |

En `no_std`, siempre se requiere `panic = "abort"` en `Cargo.toml` o
un `#[panic_handler]` propio:

```toml
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```

---

## 5. `heapless` — colecciones sin allocación

`heapless` provee estructuras de datos donde la capacidad máxima es un
parámetro genérico de tipo `const` resuelto en **tiempo de compilación**:

```rust
use heapless::{Vec, String};
use heapless::spsc::Queue;

// Vec con capacidad máxima 8 — vive en stack
let mut v: Vec<u8, 8> = Vec::new();
v.push(1).ok();  // Retorna Err si está lleno

// String con máximo 32 bytes
let mut s: String<32> = String::new();
s.push_str("hola").ok();

// Cola SPSC (Single-Producer Single-Consumer) — safe entre ISR y main
static mut COLA: Queue<u32, 16> = Queue::new();
```

**Estructuras disponibles en `heapless`:**

| Tipo | Análogo `std` | Parámetro |
|------|--------------|-----------|
| `Vec<T, N>` | `Vec<T>` | capacidad máxima |
| `String<N>` | `String` | bytes máximos |
| `FnvIndexMap<K,V,N>` | `HashMap<K,V>` | pares máximos |
| `spsc::Queue<T, N>` | `mpsc::channel` | slots |
| `Deque<T, N>` | `VecDeque<T>` | elementos máximos |

---

## 6. `#[global_allocator]` — usar `alloc` sin `std`

Si el hardware tiene SRAM suficiente, puedes registrar un allocator global
para tener `Box`, `Vec` y `String` sin OS:

```rust
#![no_std]
extern crate alloc;

use alloc::vec::Vec;
use alloc::boxed::Box;

// Registrar un allocator (ej. buddy_system_allocator)
use buddy_system_allocator::LockedHeap;

#[global_allocator]
static HEAP: LockedHeap<32> = LockedHeap::empty();

// Inicializar en el arranque del firmware
unsafe {
    let heap_start = 0x2000_0000 as *mut u8;
    let heap_size = 32 * 1024; // 32 KiB
    // SAFETY: dirección y tamaño son válidos para este MCU.
    HEAP.lock().init(heap_start, heap_size);
}
```

Esto separa la librería estándar (`std`) del allocator (`alloc`), permitiendo
usar colecciones dinámicas incluso en microcontroladores con RTOS mínimo.

---

## 7. HAL — Hardware Abstraction Layer

El ecosistema embedded Rust se organiza en tres capas:

```
┌──────────────────────────────────┐
│  Código de aplicación (tu crate) │
├──────────────────────────────────┤
│  HAL crate (stm32f4xx-hal, etc.) │
│  Implementa embedded-hal traits  │
├──────────────────────────────────┤
│  PAC — Peripheral Access Crate   │
│  Wrappers seguros sobre registros│
├──────────────────────────────────┤
│  svd2rust — generado desde SVD   │
├──────────────────────────────────┤
│  Hardware (registros del MCU)    │
└──────────────────────────────────┘
```

**PAC (Peripheral Access Crate):** generado con `svd2rust` desde el SVD
(System View Description) del fabricante. Provee acceso tipado a registros.

**HAL crate:** implementa los traits de `embedded-hal` sobre el PAC,
convirtiendo operaciones de bajo nivel en APIs idiomáticas de Rust.

---

## 8. `embedded-hal` traits

`embedded-hal` define traits estándar para abstraer periféricos, permitiendo
escribir código portable entre diferentes microcontroladores:

```rust
use embedded_hal::digital::OutputPin;
use embedded_hal::serial::{Read, Write};

// Función genérica — funciona con cualquier HAL
fn parpadear<P: OutputPin>(pin: &mut P, n: u32) {
    for _ in 0..n {
        pin.set_high().ok();
        // delay...
        pin.set_low().ok();
    }
}
```

**Traits principales:**

| Trait | Periférico | Métodos clave |
|-------|-----------|---------------|
| `OutputPin` | GPIO salida | `set_high()`, `set_low()` |
| `InputPin` | GPIO entrada | `is_high()`, `is_low()` |
| `serial::Write` | UART TX | `write()`, `flush()` |
| `serial::Read` | UART RX | `read()` |
| `spi::Transfer` | SPI | `transfer()` |
| `i2c::Write` | I2C | `write()` |
| `DelayMs` | Timer | `delay_ms()` |

---

## 9. RTIC — Real-Time Interrupt-driven Concurrency

RTIC es un framework que gestiona la concurrencia mediante interrupciones,
sin necesitar un RTOS completo:

```rust
#[rtic::app(device = stm32f4xx_hal::pac)]
mod app {
    #[shared]
    struct Shared {
        contador: u32,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        (Shared { contador: 0 }, Local {}, init::Monotonics())
    }

    // Tarea activada por interrupción EXTI0 (prioridad 1)
    #[task(binds = EXTI0, shared = [contador], priority = 1)]
    fn boton_presionado(mut cx: boton_presionado::Context) {
        cx.shared.contador.lock(|c| *c += 1);
    }
}
```

RTIC garantiza en tiempo de compilación que no hay data races entre tareas
de diferente prioridad, usando el modelo Stack Resource Policy.

---

## 10. Depuración: probe-rs, defmt, RTT

### `probe-rs` — flashear y depurar

```bash
# Flashear firmware
cargo embed --chip STM32F411CEUx

# Con probe-rs CLI
probe-rs download --chip STM32F411 target/thumbv7em-none-eabihf/release/firmware
```

### `defmt` — logging eficiente en embedded

`defmt` (deferred formatting) formatea los mensajes en el host, no en el MCU,
reduciendo el overhead a enviar solo IDs numéricos por RTT:

```rust
use defmt::info;

info!("temperatura={}", 36.6_f32);
// En el MCU: envía solo el ID del string + el float
// En el host: reconstruye el mensaje completo
```

### RTT — Real-Time Transfer

RTT (Real-Time Transfer) usa la memoria de debug del MCU para enviar logs
sin interrumpir la ejecución del firmware. Es la alternativa más eficiente
a UART para depuración.

---

## 11. Errores comunes en `no_std`

| Error | Causa | Solución |
|-------|-------|----------|
| `can't find crate for std` | Falta `#![no_std]` en la raíz | Añadir `#![no_std]` al inicio |
| `language item required... panic_impl` | Sin `#[panic_handler]` | Añadir handler o `panic-halt` crate |
| `use of `std::...`` | Importar de `std` en lugar de `core` | Reemplazar `std::` por `core::` |
| `heapless::Vec push failed` | Buffer lleno en runtime | Verificar `is_full()` o manejar `Err` |
| `alloc` types unavailable | Sin `extern crate alloc` | Añadir el extern y un `#[global_allocator]` |

---

## 12. Comparación con otros lenguajes

| Aspecto | Rust `no_std` | C embedded | MicroPython |
|---------|---------------|-----------|-------------|
| Seguridad de memoria | ✅ Garantizada | ❌ Manual | ✅ GC |
| Tamaño binario | ✅ ~2-10 KB | ✅ ~1-5 KB | ❌ ~200 KB |
| Abstracciones | ✅ Cero costo | ⚠️ Macros C | ❌ Overhead Python |
| Tipos en compile-time | ✅ Capacidades fijas | ❌ No | ❌ No |
| Concurrencia segura | ✅ RTIC | ❌ Manual | ❌ GIL |
