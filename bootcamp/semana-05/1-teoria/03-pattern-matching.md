# 🎯 Pattern Matching

> **match** - Control de flujo potente y seguro

---

## La Expresión match

`match` compara un valor contra patrones:

```rust
enum Moneda {
    Peso,
    Dolar,
    Euro,
}

fn valor_en_centavos(moneda: Moneda) -> u32 {
    match moneda {
        Moneda::Peso => 100,
        Moneda::Dolar => 85000,
        Moneda::Euro => 92000,
    }
}
```

---

## Exhaustividad

El compilador **garantiza** que cubras todos los casos:

```rust
enum Direccion {
    Norte,
    Sur,
    Este,
    Oeste,
}

fn mover(dir: Direccion) {
    match dir {
        Direccion::Norte => println!("↑"),
        Direccion::Sur => println!("↓"),
        // ❌ ERROR: non-exhaustive patterns
        // Falta Este y Oeste
    }
}
```

---

## El Patrón Comodín `_`

Para "todo lo demás":

```rust
fn mover(dir: Direccion) {
    match dir {
        Direccion::Norte => println!("↑"),
        Direccion::Sur => println!("↓"),
        _ => println!("→ o ←"),  // Este u Oeste
    }
}
```

⚠️ **Cuidado**: `_` puede ocultar nuevas variantes agregadas después.

---

## Extraer Datos con Patrones

```rust
enum Mensaje {
    Texto(String),
    Numero(i32),
    Posicion { x: i32, y: i32 },
}

fn procesar(msg: Mensaje) {
    match msg {
        Mensaje::Texto(contenido) => {
            println!("Texto: {}", contenido);
        }
        
        Mensaje::Numero(n) => {
            println!("Número: {}", n);
        }
        
        Mensaje::Posicion { x, y } => {
            println!("Posición: ({}, {})", x, y);
        }
    }
}
```

---

## Patrones con Guardas (Guards)

Condiciones adicionales con `if`:

```rust
fn clasificar(n: i32) {
    match n {
        0 => println!("Cero"),
        n if n > 0 => println!("Positivo: {}", n),
        n if n < 0 => println!("Negativo: {}", n),
        _ => unreachable!(),
    }
}
```

---

## Patrones con Rangos

```rust
fn clasificar_edad(edad: u32) {
    match edad {
        0..=2 => println!("Bebé"),
        3..=12 => println!("Niño"),
        13..=19 => println!("Adolescente"),
        20..=64 => println!("Adulto"),
        65.. => println!("Senior"),
    }
}
```

---

## Múltiples Patrones con `|`

```rust
fn es_vocal(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' |
        'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => false,
    }
}
```

---

## Binding con `@`

Capturar el valor mientras se hace match:

```rust
enum Evento {
    Click { x: i32, y: i32 },
    Tecla(char),
}

fn manejar(evento: Evento) {
    match evento {
        Evento::Click { x: 0..=100, y: 0..=100 } => {
            println!("Click en zona superior izquierda");
        }
        
        // Capturar las coordenadas con @
        Evento::Click { x: x_pos @ 101..=200, y } => {
            println!("Click en x={} (zona media), y={}", x_pos, y);
        }
        
        Evento::Click { x, y } => {
            println!("Click en ({}, {})", x, y);
        }
        
        Evento::Tecla(c @ 'a'..='z') => {
            println!("Letra minúscula: {}", c);
        }
        
        Evento::Tecla(c) => {
            println!("Otra tecla: {}", c);
        }
    }
}
```

---

## match es una Expresión

Retorna un valor:

```rust
let descripcion = match moneda {
    Moneda::Peso => "Peso argentino",
    Moneda::Dolar => "Dólar estadounidense",
    Moneda::Euro => "Euro",
};
```

---

## Destructuring en Tuplas

```rust
let punto = (3, 4);

match punto {
    (0, 0) => println!("Origen"),
    (x, 0) => println!("En eje X: {}", x),
    (0, y) => println!("En eje Y: {}", y),
    (x, y) => println!("Punto: ({}, {})", x, y),
}
```

---

## Destructuring en Structs

```rust
struct Punto {
    x: i32,
    y: i32,
}

fn clasificar(p: Punto) {
    match p {
        Punto { x: 0, y: 0 } => println!("Origen"),
        Punto { x, y: 0 } => println!("Eje X: {}", x),
        Punto { x: 0, y } => println!("Eje Y: {}", y),
        Punto { x, y } => println!("({}, {})", x, y),
    }
}
```

---

## Patrones Anidados

```rust
enum Contenedor {
    Vacio,
    ConValor(Option<i32>),
}

fn extraer(c: Contenedor) -> i32 {
    match c {
        Contenedor::Vacio => 0,
        Contenedor::ConValor(None) => 0,
        Contenedor::ConValor(Some(n)) => n,
    }
}
```

---

## Ignorar Partes con `..`

```rust
struct Config {
    debug: bool,
    verbose: bool,
    timeout: u32,
    max_connections: u32,
}

fn es_debug(config: &Config) -> bool {
    match config {
        Config { debug: true, .. } => true,
        _ => false,
    }
}
```

---

## Resumen de Patrones

| Patrón | Ejemplo | Uso |
|--------|---------|-----|
| Literal | `42`, `"hola"` | Valor exacto |
| Variable | `x`, `nombre` | Capturar valor |
| Wildcard | `_` | Ignorar valor |
| Rango | `1..=10` | Rango de valores |
| OR | `A \| B` | Múltiples opciones |
| Guard | `x if x > 0` | Condición extra |
| Binding | `x @ 1..=10` | Capturar y verificar |
| Struct | `Point { x, y }` | Destructurar |
| Tuple | `(a, b, _)` | Destructurar |
| Rest | `..` | Ignorar resto |

---

## 🧪 Ejercicio Mental

Escribe un match para:

```rust
enum Resultado {
    Ok { valor: i32, tiempo: u64 },
    Error { codigo: u32, mensaje: String },
    Timeout,
}
```

Que imprima información apropiada para cada caso.

<details>
<summary>Ver respuesta</summary>

```rust
match resultado {
    Resultado::Ok { valor, tiempo } => {
        println!("Éxito: {} en {}ms", valor, tiempo);
    }
    Resultado::Error { codigo, mensaje } => {
        println!("Error {}: {}", codigo, mensaje);
    }
    Resultado::Timeout => {
        println!("Tiempo agotado");
    }
}
```

</details>

---

## 📚 Siguiente

[if let y while let →](04-if-let-while-let.md)
