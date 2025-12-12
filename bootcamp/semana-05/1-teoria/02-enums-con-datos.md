# 📦 Enums con Datos Asociados

> **El verdadero poder de los enums de Rust**

---

## Variantes con Datos

En Rust, cada variante puede contener datos diferentes:

```rust
enum Mensaje {
    Salir,                       // Sin datos
    Mover { x: i32, y: i32 },   // Struct-like
    Escribir(String),            // Tuple-like
    Color(u8, u8, u8),          // Múltiples valores
}
```

Esto es **mucho más poderoso** que los enums de C/Java.

---

## Tres Estilos de Variantes

### 1. Sin datos (Unit-like)

```rust
enum Comando {
    Pausar,
    Continuar,
    Detener,
}
```

### 2. Tipo Tupla (Tuple-like)

```rust
enum Evento {
    Click(i32, i32),        // Coordenadas
    Tecla(char),            // Carácter
    Scroll(f64),            // Delta
}
```

### 3. Tipo Struct (Struct-like)

```rust
enum Accion {
    Mover { destino: String, velocidad: f64 },
    Atacar { objetivo: String, daño: u32 },
    Curar { cantidad: u32 },
}
```

---

## Acceder a los Datos con Match

```rust
enum Figura {
    Circulo(f64),                    // radio
    Rectangulo { ancho: f64, alto: f64 },
    Triangulo(f64, f64),             // base, altura
}

fn area(figura: &Figura) -> f64 {
    match figura {
        Figura::Circulo(radio) => 
            std::f64::consts::PI * radio * radio,
            
        Figura::Rectangulo { ancho, alto } => 
            ancho * alto,
            
        Figura::Triangulo(base, altura) => 
            (base * altura) / 2.0,
    }
}
```

---

## Comparación con Structs Separados

Sin enums tendrías que usar múltiples structs:

```rust
// ❌ Sin enums - verbose y propenso a errores
struct Circulo { radio: f64 }
struct Rectangulo { ancho: f64, alto: f64 }
struct Triangulo { base: f64, altura: f64 }

// ¿Cómo almacenar "cualquier figura"?
// Necesitarías trait objects o generics...
```

Con enums:

```rust
// ✅ Con enums - simple y seguro
enum Figura {
    Circulo(f64),
    Rectangulo { ancho: f64, alto: f64 },
    Triangulo(f64, f64),
}

// Un Vec puede contener cualquier figura
let figuras: Vec<Figura> = vec![
    Figura::Circulo(5.0),
    Figura::Rectangulo { ancho: 10.0, alto: 20.0 },
];
```

---

## El Enum Option<T>

El enum más usado en Rust:

```rust
// Definido en la biblioteca estándar
enum Option<T> {
    Some(T),  // Hay un valor de tipo T
    None,     // No hay valor
}
```

Uso:

```rust
fn dividir(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    match dividir(10.0, 2.0) {
        Some(resultado) => println!("Resultado: {}", resultado),
        None => println!("No se puede dividir por cero"),
    }
}
```

---

## El Enum Result<T, E>

Para operaciones que pueden fallar:

```rust
// Definido en la biblioteca estándar
enum Result<T, E> {
    Ok(T),   // Operación exitosa con valor T
    Err(E),  // Error de tipo E
}
```

Uso:

```rust
fn parsear_numero(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(format!("'{}' no es un número válido", s)),
    }
}
```

---

## Enums Recursivos con Box

Para estructuras recursivas:

```rust
enum Lista {
    Vacia,
    Nodo(i32, Box<Lista>),  // Box para tamaño conocido
}

fn main() {
    let lista = Lista::Nodo(1, 
        Box::new(Lista::Nodo(2, 
            Box::new(Lista::Nodo(3, 
                Box::new(Lista::Vacia)
            ))
        ))
    );
}
```

---

## Ejemplo Práctico: Mensajes de Red

```rust
enum MensajeRed {
    Ping,
    Pong,
    Datos { 
        origen: String, 
        destino: String, 
        payload: Vec<u8> 
    },
    Error { 
        codigo: u16, 
        mensaje: String 
    },
    Desconectar { razon: Option<String> },
}

fn procesar(msg: MensajeRed) {
    match msg {
        MensajeRed::Ping => enviar(MensajeRed::Pong),
        
        MensajeRed::Datos { origen, destino, payload } => {
            println!("{} -> {}: {} bytes", origen, destino, payload.len());
        }
        
        MensajeRed::Error { codigo, mensaje } => {
            eprintln!("Error {}: {}", codigo, mensaje);
        }
        
        MensajeRed::Desconectar { razon } => {
            if let Some(r) = razon {
                println!("Desconectado: {}", r);
            }
        }
        
        MensajeRed::Pong => {}
    }
}
```

---

## Resumen

| Estilo | Sintaxis | Uso |
|--------|----------|-----|
| Unit | `Variante` | Estados simples |
| Tuple | `Variante(T, U)` | Datos simples |
| Struct | `Variante { campo: T }` | Datos con nombres |

---

## 🧪 Ejercicio Mental

Define un enum para representar el resultado de una operación de archivo:

- Éxito con contenido (String)
- Archivo no encontrado
- Permiso denegado con usuario (String)
- Error desconocido con código (u32)

<details>
<summary>Ver respuesta</summary>

```rust
enum ResultadoArchivo {
    Exito(String),
    NoEncontrado,
    PermisoDenegado { usuario: String },
    ErrorDesconocido(u32),
}
```

</details>

---

## 📚 Siguiente

[Pattern Matching →](03-pattern-matching.md)
