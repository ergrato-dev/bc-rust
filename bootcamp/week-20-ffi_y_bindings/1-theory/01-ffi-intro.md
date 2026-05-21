# 📖 01 — ¿Qué es FFI y cuándo usarlo?

## Motivación: el ecosistema es políglota

Ningún lenguaje existe en aislamiento. Hay décadas de código C/C++ en el mundo
(OpenSSL, SQLite, libpng, BLAS...) que sería insensato reescribir. FFI
(**Foreign Function Interface**) permite que Rust hable con ese código sin copiar
la lógica ni sacrificar rendimiento.

```
┌─────────────────────────────────────────────────────────────────────┐
│                     EL MUNDO POLÍGLOTA                              │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│   Python / Node.js / C / C++                                        │
│         │                                                           │
│         │  FFI / bindings                                           │
│         ▼                                                           │
│   ┌───────────────┐     ┌─────────────────────────┐                │
│   │  Rust Library │────▶│  Alto rendimiento        │                │
│   │  (.so / .dll) │     │  Seguridad de memoria    │                │
│   └───────────────┘     │  Interfaz C estable      │                │
│                         └─────────────────────────┘                │
│                                                                     │
│  Rust puede ser:                                                    │
│    ● Caller  — llamar a librerías C existentes                      │
│    ● Callee  — exponer una API que C/Python/Node consumen           │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

---

## ¿Qué es el ABI de C?

El **ABI** (Application Binary Interface) es el contrato de bajo nivel que define:

- Cómo se pasan los argumentos a las funciones (registros vs. stack)
- Cómo se retornan los valores
- Quién limpia el stack después de la llamada
- El layout de memoria de los tipos de datos

El **ABI de C** es el estándar universal. Casi todos los lenguajes de sistema pueden
generar o consumir código con ABI C. Por eso lo usamos como "lingua franca" de FFI.

```
┌────────────────────────────────────────────────────────────────────┐
│                    CALLING CONVENTION (x86-64 Linux)               │
├────────────────────────────────────────────────────────────────────┤
│                                                                    │
│  Argumentos enteros:  rdi, rsi, rdx, rcx, r8, r9 → stack          │
│  Argumentos float:    xmm0-xmm7                                    │
│  Valor de retorno:    rax (entero), xmm0 (float)                   │
│                                                                    │
│  add(a: i32, b: i32) → i32                                         │
│  ┌────────┐   ┌────────┐     ┌────────┐                            │
│  │  rdi   │   │  rsi   │     │  rax   │                            │
│  │   a    │   │   b    │ →   │ a + b  │                            │
│  └────────┘   └────────┘     └────────┘                            │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
```

---

## Name Mangling: el problema que resuelve `extern "C"`

El compilador de Rust transforma los nombres de las funciones internamente
(name mangling) para incorporar información de módulo, tipos genéricos y
versiones. Por ejemplo:

```rust
// Nombre Rust (mangled): _ZN7my_crate3add17h4f3e1a9b2c7d5e8fE
pub fn add(a: i32, b: i32) -> i32 { a + b }

// Nombre C (sin mangling): add
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 { a + b }
```

| Atributo/keyword | Efecto |
|------------------|--------|
| `extern "C"` | Usar ABI de C (sin mangling de Rust) |
| `#[no_mangle]` | Preservar el nombre exacto en el binario |
| `#[repr(C)]` | Layout de memoria compatible con C |

---

## Cuándo usar FFI — árbol de decisiones

```
¿Existe una librería que necesito?
        │
        ├── Sí, está en Rust (crates.io)
        │         └── cargo add → no necesitas FFI
        │
        └── No está en Rust o está en C/C++
                  │
                  ├── ¿Quiero llamarla desde Rust?
                  │       └── extern "C" + libc/bindgen
                  │
                  └── ¿Quiero exponerla a Python/Node?
                            ├── Python → PyO3 + maturin
                            └── Node.js → napi-rs
```

---

## El coste de FFI

FFI no es gratuito. Cruzar la frontera ABI implica:

| Coste | Descripción | Magnitud |
|-------|-------------|----------|
| Conversión de tipos | `String` → `CString` → `*const c_char` | nanosegundos |
| No-inlining | El compilador no puede hacer inline cross-ABI | sin LTO |
| Overhead de llamada | Resolución de símbolo dinámico (cdylib) | nanosegundos |
| Unsafe explícito | El programador asume responsabilidad | cero coste en CPU |

**Regla práctica**: FFI es eficiente para llamadas que hacen trabajo real
(procesamiento de datos, I/O). Evitar llamadas FFI en loops muy calientes
donde el overhead supere el trabajo útil.

---

## Tipos de interop cubiertos en esta semana

| Semana 20 | Herramienta | Dirección |
|-----------|-------------|-----------|
| C → Rust | `extern "C"`, `libc` | Rust llama C |
| Rust → C | `#[no_mangle]`, `cbindgen` | C llama Rust |
| Python → Rust | `PyO3`, `maturin` | Python llama Rust |
| Node.js → Rust | `napi-rs` | Node llama Rust |

---

## Relación con la semana anterior (`unsafe`)

FFI implica `unsafe` casi siempre. Las funciones C:

- No garantizan que los punteros sean válidos
- No garantizan aliasing correcto
- No tienen borrow checker
- Pueden devolver punteros nulos

Por eso **cada bloque `unsafe` en código FFI debe tener `// SAFETY:`**
que documente qué invariantes asume Rust sobre el código C.

```rust
// SAFETY: `ptr` fue creado por `malloc` de C y tiene tamaño >= `len`.
// No hay otros alias mutables activos. El caller garantiza alineación.
let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
```

---

## Errores comunes en FFI

| Error | Consecuencia | Solución |
|-------|--------------|----------|
| Pasar `&str` a C directamente | C espera null terminator, `&str` no tiene | Usar `CString` |
| Olvidar `#[repr(C)]` en structs | Layout incorrecto → datos corruptos | Siempre `#[repr(C)]` |
| Olvidar `#[no_mangle]` | El símbolo no se encuentra → linker error | Siempre en exports |
| Double-free de un `Box` | Undefined behavior | Documentar ownership |
| `panic!` cruzando ABI | Undefined behavior en muchos casos | Usar `catch_unwind` |

---

## Comparación con otros lenguajes

| Lenguaje | Mecanismo FFI hacia C | Seguridad |
|----------|----------------------|-----------|
| **Rust** | `extern "C"` + `unsafe` | Alta (verificado en frontera) |
| Python | `ctypes`, `cffi` | Baja (todo es runtime) |
| Go | `cgo` | Media (GC complica ownership) |
| Java | JNI | Baja (JVM overhead + runtime checks) |
| C++ | Directo, `extern "C"` | Media (no enforces SAFETY comments) |

Rust es el único lenguaje donde el compilador te obliga a marcar cada cruce FFI
con `unsafe`, haciéndolo explícito y auditable.
