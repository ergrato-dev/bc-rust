# Futures y el trait Future

## 🎯 Objetivos

- Entender el trait Future en profundidad
- Comprender Poll, Pin y Context
- Saber cómo se ejecutan los futures

## 📚 Contenido

### El trait Future

```rust
pub trait Future {
    type Output;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

| Componente | Propósito |
|------------|-----------|
| `Output` | Tipo del valor resultante |
| `poll()` | Avanza la ejecución del future |
| `Pin<&mut Self>` | Garantiza que el future no se mueva |
| `Context` | Contiene el Waker para notificaciones |
| `Poll` | Indica si está listo o pendiente |

### Poll: Ready vs Pending

```rust
pub enum Poll<T> {
    Ready(T),   // El future terminó, aquí está el valor
    Pending,    // Aún no está listo, vuelve después
}
```

```rust
// Ejemplo conceptual de cómo funciona poll
fn ejecutar_future<F: Future>(mut future: F) -> F::Output {
    loop {
        match future.poll() {
            Poll::Ready(valor) => return valor,
            Poll::Pending => {
                // Esperar notificación y reintentar
                wait_for_wakeup();
            }
        }
    }
}
```

---

## Pin y Por Qué Es Necesario

### El Problema: Self-References

Los futures pueden contener referencias a sí mismos:

```rust
async fn ejemplo() {
    let data = vec![1, 2, 3];
    let referencia = &data[0];  // referencia apunta a data
    
    alguna_operacion().await;   // Future puede moverse aquí
    
    println!("{}", referencia); // ¡referencia sería inválida si se movió!
}
```

### La Solución: Pin

`Pin` garantiza que un valor no se moverá en memoria:

```rust
use std::pin::Pin;

// Pin<&mut T> = referencia mutable que garantiza que T no se moverá
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
```

### Unpin

Tipos que pueden moverse libremente implementan `Unpin`:

```rust
// La mayoría de tipos son Unpin
impl Unpin for i32 {}
impl Unpin for String {}
impl<T: Unpin> Unpin for Vec<T> {}

// Los futures generados por async NO son Unpin
// porque pueden tener auto-referencias
```

### Crear un Pin

```rust
use std::pin::pin;

// Método 1: Macro pin! (stack)
let future = pin!(mi_future());

// Método 2: Box::pin (heap)
let future: Pin<Box<dyn Future<Output = ()>>> = Box::pin(mi_future());
```

---

## Context y Waker

### ¿Qué es el Waker?

El **Waker** es el mecanismo para notificar al runtime que un future puede progresar:

```rust
pub struct Context<'a> {
    waker: &'a Waker,
}

impl Context<'_> {
    pub fn waker(&self) -> &Waker {
        self.waker
    }
}
```

### Flujo de Ejecución

```
1. Executor llama poll() en el future
2. Future retorna Pending, guarda una copia del Waker
3. Executor pone el future a dormir
4. Cuando I/O está listo, el reactor llama waker.wake()
5. Executor despierta y vuelve a llamar poll()
6. Future retorna Ready(valor)
```

```
┌──────────┐         ┌──────────┐         ┌──────────┐
│ Executor │ poll()  │  Future  │ espera  │ Reactor  │
│          │────────►│          │────────►│  (I/O)   │
│          │◄────────│ Pending  │         │          │
│ (duerme) │         │          │         │          │
│          │         │          │◄────────│ wake()   │
│          │ poll()  │          │         │          │
│          │────────►│          │         │          │
│          │◄────────│ Ready(v) │         │          │
└──────────┘         └──────────┘         └──────────┘
```

---

## Implementando un Future Manual

### Future Simple: Timer

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct Timer {
    when: Instant,
}

impl Timer {
    fn new(duration: Duration) -> Self {
        Timer {
            when: Instant::now() + duration,
        }
    }
}

impl Future for Timer {
    type Output = ();
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            // Programar despertar
            // (En realidad necesitarías un timer del runtime)
            let waker = cx.waker().clone();
            let when = self.when;
            
            std::thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    std::thread::sleep(when - now);
                }
                waker.wake();
            });
            
            Poll::Pending
        }
    }
}
```

### Future que Retorna un Valor

```rust
struct ReadyFuture<T>(Option<T>);

impl<T> Future for ReadyFuture<T> {
    type Output = T;
    
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        match self.0.take() {
            Some(value) => Poll::Ready(value),
            None => panic!("Future polled after completion"),
        }
    }
}

// Uso
async fn ejemplo() {
    let valor = ReadyFuture(Some(42)).await;
    println!("Valor: {}", valor);
}
```

---

## Combinadores de Futures

### futures crate

```toml
[dependencies]
futures = "0.3"
```

### join! - Ejecutar en Paralelo

```rust
use futures::join;

async fn ejemplo() {
    let (a, b, c) = join!(
        fetch_a(),
        fetch_b(),
        fetch_c(),
    );
    // Las tres se ejecutan concurrentemente
}
```

### select! - Carrera de Futures

```rust
use futures::select;

async fn ejemplo() {
    select! {
        resultado = tarea_rapida() => {
            println!("Rápida ganó: {}", resultado);
        }
        resultado = tarea_lenta() => {
            println!("Lenta ganó: {}", resultado);
        }
    }
    // Solo el primero en completar se ejecuta
}
```

### try_join! - Con Manejo de Errores

```rust
use futures::try_join;

async fn ejemplo() -> Result<(), Error> {
    let (a, b) = try_join!(
        operacion_a(),  // -> Result<A, Error>
        operacion_b(),  // -> Result<B, Error>
    )?;
    // Si alguno falla, retorna el error inmediatamente
    Ok(())
}
```

---

## FutureExt - Métodos de Extensión

```rust
use futures::future::FutureExt;

async fn ejemplo() {
    // map - Transformar resultado
    let doubled = async { 21 }.map(|x| x * 2).await;
    
    // then - Encadenar futures
    let result = async { 5 }
        .then(|x| async move { x + 10 })
        .await;
    
    // boxed - Convertir a Box<dyn Future>
    let boxed: Pin<Box<dyn Future<Output = i32>>> = 
        async { 42 }.boxed();
    
    // fuse - Hacer safe para poll múltiple
    let mut fused = async { 42 }.fuse();
}
```

---

## Async Blocks

Además de `async fn`, puedes crear futures inline:

```rust
async fn ejemplo() {
    // async block
    let future = async {
        println!("Ejecutando...");
        42
    };
    
    let resultado = future.await;
    
    // async move block (captura ownership)
    let datos = vec![1, 2, 3];
    let future = async move {
        // datos se mueve aquí
        datos.iter().sum::<i32>()
    };
    
    // datos ya no está disponible aquí
}
```

---

## Errores Comunes

### 1. Future No Es Send

```rust
// MAL: Rc no es Send
async fn malo() {
    let rc = std::rc::Rc::new(5);
    alguna_operacion().await;  // Error: future no es Send
    println!("{}", rc);
}

// BIEN: Usar Arc
async fn bueno() {
    let arc = std::sync::Arc::new(5);
    alguna_operacion().await;
    println!("{}", arc);
}
```

### 2. Mutex std en Async

```rust
// MAL: std::sync::Mutex bloquea
async fn malo() {
    let mutex = std::sync::Mutex::new(0);
    let guard = mutex.lock().unwrap();
    operacion_async().await;  // Mantiene lock durante await!
}

// BIEN: Usar tokio::sync::Mutex o soltar antes de await
async fn bueno() {
    let mutex = tokio::sync::Mutex::new(0);
    let guard = mutex.lock().await;
    // O con std::sync::Mutex, soltar antes de await
}
```

### 3. Olvidar .await

```rust
// MAL
async fn malo() {
    let future = fetch_data();  // Esto es un Future, no datos!
    // El fetch nunca ocurre
}

// BIEN
async fn bueno() {
    let datos = fetch_data().await;  // Ahora sí ejecuta
}
```

---

## Resumen

| Concepto | Descripción |
|----------|-------------|
| `Future` | Trait para valores asíncronos |
| `poll()` | Método que avanza el future |
| `Poll::Ready(T)` | Future completado con valor |
| `Poll::Pending` | Future aún en progreso |
| `Pin` | Garantiza que el valor no se moverá |
| `Waker` | Mecanismo de notificación |
| `join!` | Ejecutar futures concurrentemente |
| `select!` | Carrera de futures |

---

## 🔗 Recursos

- [Async Book - Under the Hood](https://rust-lang.github.io/async-book/02_execution/01_chapter.html)
- [Pin Documentation](https://doc.rust-lang.org/std/pin/)
- [futures crate](https://docs.rs/futures/)

---

**Siguiente:** [03 - Runtime Tokio](03-tokio.md)
