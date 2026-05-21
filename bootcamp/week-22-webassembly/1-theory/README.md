# Semana 22 — Teoría: WebAssembly

## 1. ¿Qué es WebAssembly?

**WebAssembly (WASM)** es un formato de instrucciones binario, portátil y de bajo nivel diseñado
como objetivo de compilación para lenguajes de alto nivel. Ejecuta en navegadores web y en runtimes
server-side a velocidades cercanas al nativo.

| Característica | JavaScript | WebAssembly |
|----------------|-----------|-------------|
| Tipado | Dinámico | Estático (tipos del guest) |
| Parsing | Texto JS | Binario compacto |
| Velocidad | JIT variable | Compilación AOT predecible |
| Casos de uso | Lógica UI | Cómputo intensivo, librerías |
| Memoria | GC automático | Memoria lineal explícita |

Rust es uno de los lenguajes mejor preparados para compilar a WASM: sin runtime ni GC, control
preciso de memoria y toolchain oficial (`wasm-pack`).

---

## 2. Targets de compilación WASM en Rust

Rust ofrece dos targets principales:

```bash
# Sin sistema operativo (navegador y runtimes embebidos)
rustup target add wasm32-unknown-unknown

# Con WASI (WebAssembly System Interface): acceso a archivos, stdio
rustup target add wasm32-wasip1
```

| Target | Dónde ejecuta | `std::fs` | `std::thread` |
|--------|---------------|-----------|---------------|
| `wasm32-unknown-unknown` | Navegador, Node.js, Wasmtime | ❌ | ❌ |
| `wasm32-wasip1` | Wasmtime, Wasmer, WASI hosts | ✅ | Limitado |

---

## 3. `wasm-bindgen`: el puente Rust ↔ JavaScript

`wasm-bindgen` genera automáticamente el "glue code" JavaScript que traduce tipos entre el
mundo JS y la memoria lineal de WASM.

```rust
use wasm_bindgen::prelude::*;

/// Función exportada a JavaScript.
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("¡Hola, {name}!")
}
```

Al compilar, `wasm-bindgen` produce:
- El binario `.wasm`
- Un fichero `.js` con el glue code de conversiones
- Opcionalmente, tipos TypeScript (`.d.ts`)

### Cómo funciona la conversión de strings

Las strings no se pasan directamente porque JS y WASM tienen espacios de memoria separados.
`wasm-bindgen` copia los bytes UTF-8 a la memoria lineal del módulo WASM y pasa un par
`(ptr, len)` al código Rust.

```
JS  ──── ptr+len ────►  memoria WASM  ──── &str ────►  código Rust
JS  ◄─── ptr+len ────  copia de bytes  ◄──  String  ──  código Rust
```

---

## 4. Tipos en la frontera WASM/JS

| Tipo Rust | Tipo JS generado | Notas |
|-----------|-----------------|-------|
| `i32`, `u32`, `f64` | `number` | Sin coste de conversión |
| `bool` | `boolean` | |
| `&str` | `string` | Copia en memoria WASM |
| `String` | `string` | Copia y ownership Rust |
| `Vec<f64>` | `Float64Array` | Copia el buffer |
| `Vec<String>` | `string[]` | |
| `JsValue` | `any` | Tipo opaco de JS |
| Struct con `#[wasm_bindgen]` | Clase JS | Referencia por puntero |

```rust
use wasm_bindgen::prelude::*;

// Vec<f64> → Float64Array en JavaScript
#[wasm_bindgen]
pub fn cuadrados(valores: &[f64]) -> Vec<f64> {
    valores.iter().map(|x| x * x).collect()
}
```

---

## 5. Structs con `#[wasm_bindgen]`

Los structs Rust se convierten en **clases JavaScript** con estado en la memoria del módulo WASM.

```rust
#[wasm_bindgen]
pub struct Contador {
    valor: i32,
}

#[wasm_bindgen]
impl Contador {
    #[wasm_bindgen(constructor)]  // permite `new Contador(0)` en JS
    pub fn new(inicial: i32) -> Contador {
        Contador { valor: inicial }
    }

    pub fn valor(&self) -> i32 { self.valor }
    pub fn incrementar(&mut self) { self.valor += 1; }
}
```

Desde JavaScript:

```javascript
const c = new Contador(0);
c.incrementar();
console.log(c.valor()); // 1
c.free();               // liberar memoria (importante en loops)
```

---

## 6. `wasm-pack`: flujo de build y targets

`wasm-pack` orquesta `cargo build`, `wasm-bindgen-cli` y empaqueta un módulo npm listo para usar.

```bash
# Instalar
cargo install wasm-pack@0.13.1

# Targets disponibles
wasm-pack build --target web       # ESModule para navegador
wasm-pack build --target bundler   # Para webpack/vite (import estático)
wasm-pack build --target nodejs    # CommonJS para Node.js
wasm-pack build --target no-modules  # <script> sin bundler

# Build de release (binario optimizado)
wasm-pack build --release --target web
```

El directorio `pkg/` generado contiene:
- `mi_crate_bg.wasm` — el binario WASM
- `mi_crate.js` — glue code JavaScript
- `mi_crate.d.ts` — tipos TypeScript
- `package.json` — metadatos npm

---

## 7. `web-sys` y `js-sys`

- **`js-sys`**: bindings a objetos estándar de JavaScript (`Array`, `Math`, `Date`, `Promise`...)
- **`web-sys`**: bindings a APIs del navegador (`Document`, `Window`, `fetch`, `console`...)

```rust
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn log_mensaje(msg: &str) {
    console::log_1(&msg.into());
}
```

Cada API de `web-sys` es un **feature** que hay que activar en `Cargo.toml`:

```toml
[dependencies]
web-sys = { version = "0.3.77", features = ["console", "Window", "Document"] }
```

---

## 8. Restricciones de WASM

```
❌  std::thread::spawn      →  WASM es single-threaded (salvo SharedArrayBuffer)
❌  std::fs::read_to_string →  no filesystem en wasm32-unknown-unknown
❌  println!                →  no stdout; usar web_sys::console::log_1
❌  std::time::Instant      →  usar web_sys::Performance en browser
✅  wasm-bindgen-futures     →  async con Promises JS
✅  console_error_panic_hook →  ver panics de Rust en DevTools
```

Para ver panics en el navegador:

```rust
// lib.rs
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
```

---

## 9. Wasmtime: ejecutar WASM desde el server-side

**Wasmtime** es un runtime WASM embebible en aplicaciones Rust (y otros lenguajes). Permite
ejecutar módulos WASM fuera del navegador, ideal para plugins, funciones serverless y sandboxing.

```rust
use wasmtime::*;

fn main() -> anyhow::Result<()> {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    // Compilar desde WAT (WebAssembly Text Format)
    let module = Module::new(&engine, r#"
        (module
          (func (export "suma") (param i32 i32) (result i32)
            local.get 0
            local.get 1
            i32.add))
    "#)?;

    let instance = Instance::new(&mut store, &module, &[])?;
    let suma = instance.get_typed_func::<(i32, i32), i32>(&mut store, "suma")?;

    println!("suma(10, 32) = {}", suma.call(&mut store, (10, 32))?);
    Ok(())
}
```

Conceptos clave de Wasmtime:

| Tipo | Rol |
|------|-----|
| `Engine` | Compila y optimiza módulos (AOT/JIT) |
| `Store<T>` | Almacén de estado + recursos del host |
| `Module` | Módulo WASM compilado (reutilizable) |
| `Instance` | Módulo instanciado con su memoria propia |
| `TypedFunc` | Función WASM con tipos Rust verificados |

---

## 10. Testing con `wasm-bindgen-test`

```rust
use wasm_bindgen_test::*;

// Ejecutar en Node.js (no necesita navegador)
wasm_bindgen_test_configure!(run_in_node_experimental);

#[wasm_bindgen_test]
fn suma_correcta() {
    assert_eq!(super::suma(2, 3), 5);
}
```

```bash
# Ejecutar tests en Node.js
wasm-pack test --node

# Ejecutar tests en un navegador headless
wasm-pack test --headless --firefox
```

---

## 11. Errores comunes

| Error | Causa | Solución |
|-------|-------|----------|
| `can't use std::thread` | WASM sin threads | Usar `wasm-bindgen-futures` |
| `JsValue` en tipo de retorno | Tipo no soportado | Usar tipos primitivos o `serde` |
| Struct sin `#[wasm_bindgen]` | Struct privada al ABI | Añadir el atributo |
| `free()` olvidado en JS | Memoria WASM sin liberar | Usar bloque `try/finally` en JS |
| Panic sin mensaje en DevTools | Sin panic hook | Añadir `console_error_panic_hook` |

---

## 12. Comparación con otros lenguajes

| Lenguaje | Toolchain WASM | Madurez | Tamaño binario |
|----------|---------------|---------|----------------|
| **Rust** | `wasm-pack`, `wasm-bindgen` | Excelente | Pequeño (sin GC) |
| C/C++ | `emscripten` | Muy buena | Variable |
| Go | `GOARCH=wasm` | Buena | Grande (runtime Go) |
| Python | `pyodide` | Experimental | Muy grande |
| AssemblyScript | `asc` | Buena | Pequeño |

Rust destaca por su binario compacto (sin GC ni runtime), su seguridad de memoria y el excelente
soporte de `wasm-pack` para publicar en npm.
