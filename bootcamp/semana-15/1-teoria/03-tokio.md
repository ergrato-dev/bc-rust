# Runtime Tokio

## 🎯 Objetivos

- Configurar y usar el runtime Tokio
- Crear y manejar tasks
- Usar join!, select! y spawn
- Entender el modelo multi-thread de Tokio

## 📚 Contenido

### ¿Qué es Tokio?

Tokio es el runtime async más popular para Rust. Proporciona:

- **Executor** multi-thread para ejecutar futures
- **Reactor** basado en epoll/kqueue/IOCP
- **I/O asíncrono** (TCP, UDP, archivos)
- **Timers** y utilidades de tiempo
- **Channels** para comunicación entre tasks
- **Primitivas de sincronización** async

### Instalación

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }

# O features específicas:
tokio = { version = "1", features = ["rt", "macros", "time"] }
```

| Feature | Descripción |
|---------|-------------|
| `rt` | Runtime básico |
| `rt-multi-thread` | Runtime multi-thread |
| `macros` | Macros como `#[tokio::main]` |
| `time` | Timers y delays |
| `io-util` | Utilidades de I/O |
| `fs` | Sistema de archivos async |
| `net` | Networking async |
| `sync` | Primitivas de sincronización |
| `full` | Todas las features |

---

## Configuración del Runtime

### Método 1: Macro #[tokio::main]

```rust
#[tokio::main]
async fn main() {
    println!("¡Hola desde Tokio!");
    mi_funcion_async().await;
}
```

### Método 2: Macro con Configuración

```rust
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // Runtime con 4 workers
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Runtime single-thread (más ligero)
}
```

### Método 3: Runtime Manual

```rust
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    rt.block_on(async {
        println!("Ejecutando en runtime");
    });
}

// O con Builder para más control
fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();
    
    rt.block_on(mi_app());
}
```

---

## Tasks con tokio::spawn

### Crear Tasks

```rust
use tokio;

#[tokio::main]
async fn main() {
    // Spawn crea una task que se ejecuta concurrentemente
    let handle = tokio::spawn(async {
        println!("Task ejecutándose");
        42
    });
    
    // La task corre en background
    println!("Main continúa...");
    
    // Esperar resultado
    let resultado = handle.await.unwrap();
    println!("Resultado: {}", resultado);
}
```

### JoinHandle

```rust
use tokio::task::JoinHandle;

async fn crear_muchas_tasks() {
    let mut handles: Vec<JoinHandle<i32>> = vec![];
    
    for i in 0..10 {
        let handle = tokio::spawn(async move {
            // Cada task tiene su propio i
            i * 2
        });
        handles.push(handle);
    }
    
    // Esperar todas
    for handle in handles {
        let resultado = handle.await.unwrap();
        println!("Resultado: {}", resultado);
    }
}
```

### Manejo de Errores en Tasks

```rust
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // Si hay panic, JoinError lo captura
        panic!("¡Algo salió mal!");
    });
    
    match handle.await {
        Ok(valor) => println!("Éxito: {:?}", valor),
        Err(e) if e.is_panic() => {
            println!("Task panicked: {:?}", e.into_panic());
        }
        Err(e) if e.is_cancelled() => {
            println!("Task fue cancelada");
        }
        Err(e) => println!("Otro error: {:?}", e),
    }
}
```

---

## join! - Concurrencia Estructurada

### Básico

```rust
use tokio;

async fn fetch_usuario() -> String {
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    "Usuario".to_string()
}

async fn fetch_datos() -> Vec<i32> {
    tokio::time::sleep(std::time::Duration::from_millis(150)).await;
    vec![1, 2, 3]
}

#[tokio::main]
async fn main() {
    // Ambas se ejecutan concurrentemente
    let (usuario, datos) = tokio::join!(
        fetch_usuario(),
        fetch_datos(),
    );
    
    println!("Usuario: {}, Datos: {:?}", usuario, datos);
    // Tiempo total: ~150ms (no 250ms)
}
```

### try_join! con Errores

```rust
use tokio;

async fn operacion_a() -> Result<i32, &'static str> {
    Ok(1)
}

async fn operacion_b() -> Result<i32, &'static str> {
    Err("Error en B")
}

#[tokio::main]
async fn main() {
    let resultado = tokio::try_join!(
        operacion_a(),
        operacion_b(),
    );
    
    match resultado {
        Ok((a, b)) => println!("Éxito: {} y {}", a, b),
        Err(e) => println!("Error: {}", e),
    }
}
```

---

## select! - Carrera de Futures

### Básico

```rust
use tokio::{select, time::{sleep, Duration}};

#[tokio::main]
async fn main() {
    select! {
        _ = sleep(Duration::from_secs(1)) => {
            println!("Timer 1 ganó");
        }
        _ = sleep(Duration::from_millis(500)) => {
            println!("Timer 2 ganó");  // Este gana
        }
    }
}
```

### Con Valores

```rust
use tokio::select;

async fn tarea_a() -> i32 { 1 }
async fn tarea_b() -> i32 { 2 }

#[tokio::main]
async fn main() {
    let resultado = select! {
        a = tarea_a() => a * 10,
        b = tarea_b() => b * 10,
    };
    
    println!("Resultado: {}", resultado);
}
```

### Timeout con select!

```rust
use tokio::{select, time::{sleep, Duration}};

async fn operacion_lenta() -> String {
    sleep(Duration::from_secs(10)).await;
    "Completado".to_string()
}

#[tokio::main]
async fn main() {
    select! {
        resultado = operacion_lenta() => {
            println!("Operación completó: {}", resultado);
        }
        _ = sleep(Duration::from_secs(2)) => {
            println!("Timeout!");
        }
    }
}
```

### Loop con select!

```rust
use tokio::{select, sync::mpsc};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<i32>(10);
    
    // Productor
    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });
    
    // Consumidor con timeout
    loop {
        select! {
            Some(msg) = rx.recv() => {
                println!("Recibido: {}", msg);
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {
                println!("Timeout, saliendo");
                break;
            }
        }
    }
}
```

---

## Timers y Delays

### sleep

```rust
use tokio::time::{sleep, Duration};

async fn ejemplo() {
    println!("Antes del sleep");
    sleep(Duration::from_secs(1)).await;
    println!("Después del sleep");
}
```

### timeout

```rust
use tokio::time::{timeout, Duration};

async fn operacion_lenta() -> i32 {
    tokio::time::sleep(Duration::from_secs(10)).await;
    42
}

#[tokio::main]
async fn main() {
    match timeout(Duration::from_secs(2), operacion_lenta()).await {
        Ok(valor) => println!("Éxito: {}", valor),
        Err(_) => println!("Timeout!"),
    }
}
```

### interval

```rust
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() {
    let mut intervalo = interval(Duration::from_millis(500));
    
    for _ in 0..5 {
        intervalo.tick().await;
        println!("Tick!");
    }
}
```

---

## Cancellation

### Abortar Tasks

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        loop {
            println!("Trabajando...");
            sleep(Duration::from_millis(500)).await;
        }
    });
    
    // Esperar un poco
    sleep(Duration::from_secs(2)).await;
    
    // Cancelar la task
    handle.abort();
    
    // Verificar resultado
    match handle.await {
        Ok(_) => println!("Task completó"),
        Err(e) if e.is_cancelled() => println!("Task cancelada"),
        Err(e) => println!("Error: {:?}", e),
    }
}
```

### AbortHandle

```rust
use tokio::task::AbortHandle;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        42
    });
    
    let abort_handle: AbortHandle = handle.abort_handle();
    
    // Desde cualquier lugar
    abort_handle.abort();
}
```

---

## spawn_blocking

Para código que bloquea (sync), usar `spawn_blocking`:

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    // MAL: Esto bloquea el runtime
    // let resultado = std::fs::read_to_string("archivo.txt");
    
    // BIEN: Ejecutar en thread pool de bloqueo
    let resultado = task::spawn_blocking(|| {
        std::fs::read_to_string("archivo.txt")
    }).await.unwrap();
    
    println!("Contenido: {:?}", resultado);
}
```

---

## Modelo Multi-Thread

```
┌─────────────────────────────────────────────────────────┐
│                    TOKIO RUNTIME                         │
│                                                          │
│  ┌─────────────────────────────────────────────────────┐│
│  │              WORKER THREADS (N)                      ││
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐   ││
│  │  │Worker 0 │ │Worker 1 │ │Worker 2 │ │Worker 3 │   ││
│  │  │ ┌─────┐ │ │ ┌─────┐ │ │ ┌─────┐ │ │ ┌─────┐ │   ││
│  │  │ │Task │ │ │ │Task │ │ │ │Task │ │ │ │Task │ │   ││
│  │  │ │Task │ │ │ │Task │ │ │ │Task │ │ │ │Task │ │   ││
│  │  │ └─────┘ │ │ └─────┘ │ │ └─────┘ │ │ └─────┘ │   ││
│  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘   ││
│  └─────────────────────────────────────────────────────┘│
│                                                          │
│  ┌─────────────────────────────────────────────────────┐│
│  │            BLOCKING THREAD POOL                      ││
│  │     (para spawn_blocking y operaciones sync)        ││
│  └─────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────┘
```

---

## Ejemplo Completo

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Iniciando aplicación...");
    
    // Múltiples tasks concurrentes
    let task1 = tokio::spawn(async {
        for i in 0..3 {
            println!("Task 1: {}", i);
            sleep(Duration::from_millis(100)).await;
        }
        "Task 1 completada"
    });
    
    let task2 = tokio::spawn(async {
        sleep(Duration::from_millis(150)).await;
        "Task 2 completada"
    });
    
    // Esperar ambas
    let (r1, r2) = tokio::join!(task1, task2);
    
    println!("Resultados: {:?}, {:?}", r1, r2);
}
```

---

## Resumen

| Concepto | Uso |
|----------|-----|
| `#[tokio::main]` | Inicializar runtime |
| `tokio::spawn` | Crear task concurrente |
| `tokio::join!` | Ejecutar futures en paralelo |
| `tokio::select!` | Carrera de futures |
| `tokio::time::sleep` | Delay async |
| `tokio::time::timeout` | Límite de tiempo |
| `spawn_blocking` | Código bloqueante |
| `handle.abort()` | Cancelar task |

---

**Siguiente:** [04 - I/O Asíncrono](04-io-async.md)
