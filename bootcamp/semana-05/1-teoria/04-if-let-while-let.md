# 🔀 if let y while let

> **Sintaxis concisa para un solo patrón**

![if let vs match](../0-assets/04-if-let-vs-match.svg)

---

## El Problema con match

A veces solo te importa **un caso**:

```rust
let opcional: Option<i32> = Some(42);

// ❌ Verbose para un solo caso
match opcional {
    Some(valor) => println!("Valor: {}", valor),
    None => {},  // No hacer nada
}
```

---

## if let al Rescate

Sintaxis más concisa:

```rust
let opcional: Option<i32> = Some(42);

// ✅ Conciso y claro
if let Some(valor) = opcional {
    println!("Valor: {}", valor);
}
```

Lee como: "**si** `opcional` **coincide con** `Some(valor)`, ejecutar el bloque".

---

## if let con else

```rust
let config: Option<String> = None;

if let Some(valor) = config {
    println!("Configuración: {}", valor);
} else {
    println!("Usando configuración por defecto");
}
```

---

## if let con Enums Personalizados

```rust
enum Estado {
    Conectado { ip: String },
    Desconectado,
    Error(String),
}

fn mostrar_ip(estado: &Estado) {
    if let Estado::Conectado { ip } = estado {
        println!("IP: {}", ip);
    }
}

fn mostrar_error(estado: &Estado) {
    if let Estado::Error(msg) = estado {
        eprintln!("Error: {}", msg);
    }
}
```

---

## if let Encadenado

```rust
enum Respuesta {
    Exito(i32),
    Error(String),
    Pendiente,
}

fn procesar(resp: Respuesta) {
    if let Respuesta::Exito(valor) = resp {
        println!("Éxito: {}", valor);
    } else if let Respuesta::Error(msg) = resp {
        println!("Error: {}", msg);
    } else {
        println!("Pendiente...");
    }
}
```

---

## while let

Iterar mientras el patrón coincida:

```rust
let mut pila = vec![1, 2, 3, 4, 5];

while let Some(tope) = pila.pop() {
    println!("Valor: {}", tope);
}
// Imprime: 5, 4, 3, 2, 1
```

---

## while let con Iteradores

```rust
let mut iter = vec!["a", "b", "c"].into_iter();

while let Some(elemento) = iter.next() {
    println!("{}", elemento);
}
```

---

## while let vs Loop con Match

```rust
// ❌ Verbose
loop {
    match cola.pop() {
        Some(item) => procesar(item),
        None => break,
    }
}

// ✅ Conciso
while let Some(item) = cola.pop() {
    procesar(item);
}
```

---

## Cuándo Usar Cada Uno

| Situación | Usar |
|-----------|------|
| Manejar todas las variantes | `match` |
| Solo un caso te importa | `if let` |
| Iterar hasta que falle | `while let` |
| Necesitas exhaustividad | `match` |

---

## if let con Result

```rust
use std::fs::File;

fn abrir_config() {
    if let Ok(archivo) = File::open("config.txt") {
        println!("Archivo abierto: {:?}", archivo);
    } else {
        println!("No se pudo abrir config.txt");
    }
}
```

---

## let else (Rust 1.65+)

Para el caso inverso - salir si NO coincide:

```rust
fn procesar(valor: Option<i32>) -> i32 {
    let Some(n) = valor else {
        println!("No hay valor");
        return 0;
    };
    
    // n está disponible aquí
    n * 2
}
```

---

## Comparación let else vs if let

```rust
// Con if let - el valor queda en el bloque
fn ejemplo1(opt: Option<i32>) -> i32 {
    if let Some(n) = opt {
        return n * 2;
    }
    0
}

// Con let else - el valor queda fuera
fn ejemplo2(opt: Option<i32>) -> i32 {
    let Some(n) = opt else {
        return 0;
    };
    n * 2  // n disponible aquí
}
```

---

## Patrones Complejos

```rust
enum Mensaje {
    Datos { id: u32, payload: Vec<u8> },
    Control(String),
    Vacio,
}

fn procesar_datos(msg: &Mensaje) {
    // Destructuring complejo en if let
    if let Mensaje::Datos { id, payload } = msg {
        if !payload.is_empty() {
            println!("Mensaje {}: {} bytes", id, payload.len());
        }
    }
}
```

---

## Anti-Patrones

```rust
// ❌ No uses if let para comparar con literales
if let 42 = x {
    // ...
}

// ✅ Usa if normal
if x == 42 {
    // ...
}

// ❌ No anides demasiados if let
if let Some(a) = opt_a {
    if let Some(b) = opt_b {
        if let Some(c) = opt_c {
            // Difícil de leer
        }
    }
}

// ✅ Usa match o combina Options
if let (Some(a), Some(b), Some(c)) = (opt_a, opt_b, opt_c) {
    // Mejor
}
```

---

## Resumen

| Sintaxis | Propósito |
|----------|-----------|
| `if let Pat = expr { }` | Un patrón, ejecutar si coincide |
| `if let Pat = expr { } else { }` | Con fallback |
| `while let Pat = expr { }` | Iterar mientras coincida |
| `let Pat = expr else { return }` | Continuar o salir |

---

## 🧪 Ejercicio Mental

Simplifica este código usando `if let`:

```rust
match usuario.email {
    Some(email) => enviar_correo(&email),
    None => {},
}
```

<details>
<summary>Ver respuesta</summary>

```rust
if let Some(email) = &usuario.email {
    enviar_correo(email);
}
```

</details>

---

## 📚 Siguiente

[Option y Result →](05-option-result.md)
