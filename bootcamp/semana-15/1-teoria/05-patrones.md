# Patrones de Programación Asíncrona

## 🎯 Objetivos

- Aplicar patrones comunes en código async
- Manejar errores correctamente
- Implementar cancelación y rate limiting
- Usar channels async para comunicación

## 📚 Contenido

### Channels Async con Tokio

Tokio proporciona varios tipos de channels para comunicación entre tasks.

### mpsc - Multiple Producer, Single Consumer

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // Channel con buffer de 32 mensajes
    let (tx, mut rx) = mpsc::channel::<String>(32);
    
    // Múltiples productores
    for i in 0..3 {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            tx_clone.send(format!("Mensaje de {}", i)).await.unwrap();
        });
    }
    
    // Importante: soltar el tx original
    drop(tx);
    
    // Consumidor único
    while let Some(msg) = rx.recv().await {
        println!("Recibido: {}", msg);
    }
}
```

### oneshot - Un Solo Mensaje

```rust
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel::<String>();
    
    tokio::spawn(async move {
        // Hacer algún trabajo
        let resultado = "Trabajo completado".to_string();
        tx.send(resultado).unwrap();
    });
    
    // Esperar el resultado
    let resultado = rx.await.unwrap();
    println!("Resultado: {}", resultado);
}
```

### broadcast - Múltiples Consumidores

```rust
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel::<String>(16);
    
    // Múltiples suscriptores
    let mut rx1 = tx.subscribe();
    let mut rx2 = tx.subscribe();
    
    tx.send("Mensaje broadcast".to_string()).unwrap();
    
    // Todos reciben el mismo mensaje
    println!("rx1: {}", rx1.recv().await.unwrap());
    println!("rx2: {}", rx2.recv().await.unwrap());
}
```

### watch - Último Valor

```rust
use tokio::sync::watch;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = watch::channel("inicial");
    
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        tx.send("actualizado").unwrap();
    });
    
    // changed() espera hasta que haya un nuevo valor
    rx.changed().await.unwrap();
    println!("Nuevo valor: {}", *rx.borrow());
}
```

---

## Graceful Shutdown

### Patrón con Channel

```rust
use tokio::sync::mpsc;
use tokio::signal;

#[tokio::main]
async fn main() {
    let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);
    
    // Task que necesita shutdown graceful
    let worker = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    println!("Recibida señal de shutdown");
                    break;
                }
                _ = hacer_trabajo() => {
                    println!("Trabajo completado");
                }
            }
        }
        println!("Worker terminado limpiamente");
    });
    
    // Esperar Ctrl+C
    signal::ctrl_c().await.unwrap();
    println!("Ctrl+C recibido, iniciando shutdown...");
    
    drop(shutdown_tx);  // Notificar shutdown
    worker.await.unwrap();
}

async fn hacer_trabajo() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
```

### Con CancellationToken

```rust
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    
    let token_clone = token.clone();
    let worker = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = token_clone.cancelled() => {
                    println!("Cancelado!");
                    break;
                }
                _ = tokio::time::sleep(std::time::Duration::from_millis(500)) => {
                    println!("Trabajando...");
                }
            }
        }
    });
    
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    token.cancel();  // Cancelar todos los que tienen el token
    
    worker.await.unwrap();
}
```

---

## Rate Limiting

### Semáforo para Limitar Concurrencia

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Máximo 3 operaciones concurrentes
    let semaforo = Arc::new(Semaphore::new(3));
    let mut handles = vec![];
    
    for i in 0..10 {
        let sem = semaforo.clone();
        handles.push(tokio::spawn(async move {
            // Adquirir permiso (bloquea si no hay disponibles)
            let _permit = sem.acquire().await.unwrap();
            
            println!("Tarea {} iniciando", i);
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            println!("Tarea {} terminando", i);
            
            // Permiso se libera automáticamente al salir del scope
        }));
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
}
```

### Rate Limiter Simple

```rust
use tokio::time::{interval, Duration};

struct RateLimiter {
    interval: tokio::time::Interval,
}

impl RateLimiter {
    fn new(rate: Duration) -> Self {
        Self {
            interval: interval(rate),
        }
    }
    
    async fn wait(&mut self) {
        self.interval.tick().await;
    }
}

#[tokio::main]
async fn main() {
    let mut limiter = RateLimiter::new(Duration::from_millis(100));
    
    for i in 0..10 {
        limiter.wait().await;
        println!("Request {}", i);
    }
}
```

---

## Retry Pattern

```rust
use tokio::time::{sleep, Duration};

async fn con_retry<T, E, F, Fut>(
    max_intentos: u32,
    delay_base: Duration,
    mut operacion: F,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut intentos = 0;
    
    loop {
        match operacion().await {
            Ok(valor) => return Ok(valor),
            Err(e) if intentos < max_intentos => {
                intentos += 1;
                let delay = delay_base * 2u32.pow(intentos - 1);
                println!("Intento {} fallido, reintentando en {:?}", intentos, delay);
                sleep(delay).await;
            }
            Err(e) => return Err(e),
        }
    }
}

// Uso
async fn operacion_fallible() -> Result<String, &'static str> {
    static mut COUNTER: u32 = 0;
    unsafe {
        COUNTER += 1;
        if COUNTER < 3 {
            Err("Error temporal")
        } else {
            Ok("Éxito!".to_string())
        }
    }
}

#[tokio::main]
async fn main() {
    let resultado = con_retry(
        5,
        Duration::from_millis(100),
        operacion_fallible,
    ).await;
    
    println!("Resultado: {:?}", resultado);
}
```

---

## Fan-Out / Fan-In

### Procesar Items en Paralelo

```rust
use futures::stream::{self, StreamExt};

async fn procesar_item(item: i32) -> i32 {
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    item * 2
}

#[tokio::main]
async fn main() {
    let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Procesar con concurrencia limitada
    let resultados: Vec<i32> = stream::iter(items)
        .map(|item| procesar_item(item))
        .buffer_unordered(3)  // Máximo 3 concurrentes
        .collect()
        .await;
    
    println!("Resultados: {:?}", resultados);
}
```

### Con spawn y JoinSet

```rust
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    
    for i in 0..10 {
        set.spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            i * 2
        });
    }
    
    // Recolectar resultados a medida que terminan
    let mut resultados = vec![];
    while let Some(res) = set.join_next().await {
        resultados.push(res.unwrap());
    }
    
    println!("Resultados: {:?}", resultados);
}
```

---

## Manejo de Errores Async

### Propagación con ?

```rust
use std::io;

async fn leer_config() -> io::Result<String> {
    let contenido = tokio::fs::read_to_string("config.txt").await?;
    Ok(contenido)
}

async fn procesar() -> io::Result<()> {
    let config = leer_config().await?;
    println!("Config: {}", config);
    Ok(())
}
```

### Error Types Personalizados

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("Error de IO: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Error de red: {0}")]
    Network(String),
    
    #[error("Timeout")]
    Timeout,
}

async fn operacion() -> Result<String, AppError> {
    let datos = tokio::fs::read_to_string("archivo.txt").await?;
    Ok(datos)
}
```

### Errores en Tasks

```rust
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // Task puede fallar
        let resultado: Result<i32, &str> = Err("Error en task");
        resultado
    });
    
    match handle.await {
        Ok(Ok(valor)) => println!("Éxito: {}", valor),
        Ok(Err(e)) => println!("Error en lógica: {}", e),
        Err(e) => println!("Error de task (panic/cancel): {}", e),
    }
}
```

---

## Patrón Actor Simplificado

```rust
use tokio::sync::mpsc;

// Mensajes que el actor puede recibir
enum Message {
    Get { respond_to: tokio::sync::oneshot::Sender<i32> },
    Set { value: i32 },
    Increment,
}

// El actor
struct CounterActor {
    receiver: mpsc::Receiver<Message>,
    value: i32,
}

impl CounterActor {
    fn new(receiver: mpsc::Receiver<Message>) -> Self {
        Self { receiver, value: 0 }
    }
    
    async fn run(mut self) {
        while let Some(msg) = self.receiver.recv().await {
            match msg {
                Message::Get { respond_to } => {
                    let _ = respond_to.send(self.value);
                }
                Message::Set { value } => {
                    self.value = value;
                }
                Message::Increment => {
                    self.value += 1;
                }
            }
        }
    }
}

// Handle para comunicarse con el actor
#[derive(Clone)]
struct CounterHandle {
    sender: mpsc::Sender<Message>,
}

impl CounterHandle {
    fn new() -> Self {
        let (sender, receiver) = mpsc::channel(32);
        let actor = CounterActor::new(receiver);
        tokio::spawn(actor.run());
        Self { sender }
    }
    
    async fn get(&self) -> i32 {
        let (send, recv) = tokio::sync::oneshot::channel();
        self.sender.send(Message::Get { respond_to: send }).await.unwrap();
        recv.await.unwrap()
    }
    
    async fn set(&self, value: i32) {
        self.sender.send(Message::Set { value }).await.unwrap();
    }
    
    async fn increment(&self) {
        self.sender.send(Message::Increment).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let counter = CounterHandle::new();
    
    counter.set(10).await;
    counter.increment().await;
    counter.increment().await;
    
    println!("Valor: {}", counter.get().await); // 12
}
```

---

## Mutex Async vs Sync

### Cuándo Usar Cada Uno

| Situación | Usar |
|-----------|------|
| Lock corto, sin await dentro | `std::sync::Mutex` |
| Lock largo o await dentro | `tokio::sync::Mutex` |
| Muchos lectores, pocos escritores | `tokio::sync::RwLock` |

```rust
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(vec![]));
    
    let data_clone = data.clone();
    tokio::spawn(async move {
        let mut guard = data_clone.lock().await;
        // Puedes hacer await aquí sin problemas
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        guard.push(1);
    });
    
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    println!("Data: {:?}", *data.lock().await);
}
```

---

## Resumen de Patrones

| Patrón | Uso |
|--------|-----|
| **mpsc** | Múltiples productores, un consumidor |
| **oneshot** | Respuesta única a una request |
| **broadcast** | Enviar a múltiples suscriptores |
| **watch** | Último valor para configuración |
| **Semaphore** | Limitar concurrencia |
| **CancellationToken** | Graceful shutdown |
| **Retry** | Reintentar operaciones fallidas |
| **Actor** | Encapsular estado mutable |

---

## 🔗 Recursos

- [Tokio Tutorial - Channels](https://tokio.rs/tokio/tutorial/channels)
- [Async Book - Patterns](https://rust-lang.github.io/async-book/06_multiple_futures/01_chapter.html)

---

**Volver a:** [README - Semana 15](../README.md)
