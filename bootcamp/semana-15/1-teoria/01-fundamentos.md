# Fundamentos de Programación Asíncrona

## 🎯 Objetivos

- Entender qué es la programación asíncrona
- Diferenciar entre síncrono, multi-thread y async
- Comprender el modelo de async en Rust

## 📚 Contenido

### ¿Qué es la Programación Asíncrona?

La programación asíncrona permite que un programa **no se bloquee** mientras espera operaciones lentas (I/O de red, disco, etc.).

```
SÍNCRONO                      ASÍNCRONO
─────────                     ──────────
Thread ────┐                  Thread ────┐
           │ Request 1                   │ Request 1
           ▼                             ├──────────► (esperando)
      [BLOQUEADO]                        │
           │                             │ Request 2
           ▼                             ├──────────► (esperando)
      Respuesta 1                        │
           │                             │ Request 3
           │ Request 2                   ├──────────► (esperando)
           ▼                             │
      [BLOQUEADO]                   Respuesta 1 ◄────┘
           │                             │
           ▼                        Respuesta 2 ◄────┘
      Respuesta 2                        │
                                    Respuesta 3 ◄────┘
```

### Comparativa de Modelos

| Modelo | Descripción | Uso |
|--------|-------------|-----|
| **Síncrono** | Un thread, bloquea en I/O | Scripts simples |
| **Multi-thread** | Un thread por tarea | CPU-bound, pocas tareas |
| **Async** | Un thread, múltiples tareas | I/O-bound, muchas tareas |

### ¿Por Qué Async?

#### El Problema con Threads

```rust
// 10,000 conexiones = 10,000 threads
// Cada thread ~ 8KB de stack
// Total: 80MB solo en stacks!

for _ in 0..10_000 {
    std::thread::spawn(|| {
        handle_connection();
    });
}
```

#### La Solución Async

```rust
// 10,000 conexiones = 10,000 futures
// Cada future ~ pocos bytes
// Total: ~1MB

for _ in 0..10_000 {
    tokio::spawn(async {
        handle_connection().await;
    });
}
```

---

## El Modelo Async de Rust

### async y await

```rust
// La palabra clave `async` convierte una función en un Future
async fn fetch_data(url: &str) -> String {
    // .await suspende la ejecución hasta que el Future esté listo
    let response = make_request(url).await;
    response.text().await
}
```

### ¿Qué es un Future?

Un **Future** representa un valor que estará disponible en el futuro.

```rust
pub trait Future {
    type Output;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),   // ¡Valor listo!
    Pending,    // Aún no, vuelve después
}
```

### Lazy Futures

Los futures en Rust son **lazy** - no hacen nada hasta que son polleados:

```rust
async fn saludar() {
    println!("¡Hola!");
}

fn main() {
    let futuro = saludar();  // No imprime nada
    // El future existe pero no se ejecutó
    
    // Necesitamos un runtime para ejecutarlo
}
```

---

## Anatomía de async/await

### Transformación del Compilador

El compilador transforma funciones async en **state machines**:

```rust
// Lo que escribes:
async fn ejemplo() -> i32 {
    let a = paso1().await;
    let b = paso2().await;
    a + b
}

// Lo que el compilador genera (conceptualmente):
enum EjemploFuture {
    Estado0,                    // Inicial
    Estado1 { a: i32 },        // Después de paso1
    Estado2 { a: i32, b: i32 }, // Después de paso2
    Terminado,
}

impl Future for EjemploFuture {
    type Output = i32;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<i32> {
        loop {
            match self.estado {
                Estado0 => {
                    match paso1_future.poll(cx) {
                        Poll::Ready(a) => self.estado = Estado1 { a },
                        Poll::Pending => return Poll::Pending,
                    }
                }
                Estado1 { a } => {
                    match paso2_future.poll(cx) {
                        Poll::Ready(b) => self.estado = Estado2 { a, b },
                        Poll::Pending => return Poll::Pending,
                    }
                }
                Estado2 { a, b } => {
                    return Poll::Ready(a + b);
                }
            }
        }
    }
}
```

### Puntos de Suspensión

Cada `.await` es un **punto de suspensión** donde la ejecución puede pausarse:

```rust
async fn proceso() {
    println!("Inicio");          // Ejecuta inmediatamente
    operacion1().await;          // ⏸️ Puede pausar aquí
    println!("Después de op1");  // Ejecuta cuando op1 termine
    operacion2().await;          // ⏸️ Puede pausar aquí
    println!("Fin");             // Ejecuta cuando op2 termine
}
```

---

## El Runtime

### ¿Qué es un Runtime?

El runtime es el **ejecutor** de futures. Rust no incluye uno por defecto.

```
┌─────────────────────────────────────┐
│            RUNTIME                   │
│  ┌─────────────────────────────────┐│
│  │          EXECUTOR               ││
│  │   Ejecuta futures, llama poll   ││
│  └──────────────┬──────────────────┘│
│                 │                    │
│  ┌──────────────▼──────────────────┐│
│  │          REACTOR                ││
│  │   Escucha eventos de I/O        ││
│  │   Despierta futures cuando      ││
│  │   hay datos disponibles         ││
│  └─────────────────────────────────┘│
└─────────────────────────────────────┘
```

### Runtimes Populares

| Runtime | Uso | Características |
|---------|-----|-----------------|
| **Tokio** | Producción | Completo, multi-thread |
| **async-std** | General | API similar a std |
| **smol** | Embebido | Mínimo, ligero |

### Ejemplo con Tokio

```rust
// Cargo.toml
// [dependencies]
// tokio = { version = "1", features = ["full"] }

use tokio;

#[tokio::main]  // Macro que crea el runtime
async fn main() {
    println!("¡Async funcionando!");
    
    let resultado = mi_funcion_async().await;
    println!("Resultado: {}", resultado);
}

async fn mi_funcion_async() -> i32 {
    // Simular trabajo async
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    42
}
```

---

## Cuándo Usar Async

### ✅ Usar Async Para:

- **Servidores web** con muchas conexiones
- **Clientes HTTP** con múltiples requests
- **Aplicaciones de red** (chat, websockets)
- **I/O de archivos** cuando hay muchas operaciones

### ❌ NO Usar Async Para:

- **Cálculos pesados** (CPU-bound) - usar threads
- **Scripts simples** - añade complejidad innecesaria
- **Pocas operaciones** - overhead no justificado

### Regla General

```
Si esperas I/O más del 50% del tiempo → Async
Si calculas más del 50% del tiempo → Threads
```

---

## Ejemplo Completo

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Iniciando...");
    
    // Ejecutar dos tareas "en paralelo" (concurrentemente)
    let (resultado1, resultado2) = tokio::join!(
        tarea_asincrona("Tarea 1", 2),
        tarea_asincrona("Tarea 2", 1),
    );
    
    println!("Resultados: {} y {}", resultado1, resultado2);
}

async fn tarea_asincrona(nombre: &str, segundos: u64) -> String {
    println!("{}: Iniciando", nombre);
    
    // Simula I/O (no bloquea el thread)
    sleep(Duration::from_secs(segundos)).await;
    
    println!("{}: Completada", nombre);
    format!("{} OK", nombre)
}
```

**Output:**
```
Iniciando...
Tarea 1: Iniciando
Tarea 2: Iniciando
Tarea 2: Completada    // Tarea 2 termina primero (1 seg)
Tarea 1: Completada    // Tarea 1 termina después (2 seg)
Resultados: Tarea 1 OK y Tarea 2 OK
```

---

## Resumen

| Concepto | Descripción |
|----------|-------------|
| **async fn** | Función que retorna un Future |
| **.await** | Suspende hasta que el Future esté listo |
| **Future** | Valor que estará disponible después |
| **Runtime** | Ejecutor de futures (Tokio, async-std) |
| **Poll** | Mecanismo interno de progreso |

---

## 🔗 Recursos

- [Async Book - Getting Started](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

---

**Siguiente:** [02 - Futures y el trait Future](02-futures.md)
