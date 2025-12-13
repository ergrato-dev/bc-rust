# Proyecto Final: Thread Pool Completo

## 🎯 Objetivo

Implementar un Thread Pool robusto y completo que demuestre dominio de:
- Threads y sincronización
- Channels (MPSC)
- Mutex y Arc
- Patrones de concurrencia

## 📋 Características

### Básicas (Implementadas)
- ✅ Pool de N workers configurables
- ✅ Cola de jobs con channel
- ✅ Shutdown graceful
- ✅ Jobs con resultados
- ✅ Estadísticas de ejecución

### Avanzadas (Ejercicios)
- ⬜ Timeout para jobs
- ⬜ Prioridad de jobs
- ⬜ Pool dinámico

## 🏗️ Arquitectura

```
                    ┌─────────────────┐
                    │   ThreadPool    │
                    │  ┌───────────┐  │
  execute(job) ───► │  │  Sender   │  │
                    │  └─────┬─────┘  │
                    │        │        │
                    │        ▼        │
                    │ ┌─────────────┐ │
                    │ │   Channel   │ │
                    │ │   (Queue)   │ │
                    │ └──────┬──────┘ │
                    │        │        │
          ┌─────────┼────────┼────────┼─────────┐
          │         │        │        │         │
          ▼         ▼        ▼        ▼         ▼
     ┌────────┐┌────────┐┌────────┐┌────────┐
     │Worker 0││Worker 1││Worker 2││Worker 3│
     └────────┘└────────┘└────────┘└────────┘
```

## 📝 Uso

### Básico

```rust
let pool = ThreadPool::new(4);

pool.execute(|| {
    println!("Ejecutando en worker");
});

pool.shutdown();
```

### Con Resultados

```rust
let pool = ThreadPool::new(4);

let rx = pool.execute_with_result(|| {
    expensive_computation()
});

let result = rx.recv().unwrap();
pool.shutdown();
```

### Con Estadísticas

```rust
let pool = ThreadPool::with_stats(4);

// ... ejecutar jobs ...

let stats = pool.stats();
println!("Completados: {}", stats.jobs_completed);
```

## 🔧 Implementación

### Message Enum

```rust
enum Message {
    Job(Job),      // Job para ejecutar
    Terminate,     // Señal de shutdown
}
```

### Worker Loop

```rust
loop {
    let message = {
        let rx = receiver.lock().unwrap();
        rx.recv()
    };
    
    match message {
        Ok(Message::Job(job)) => job(),
        Ok(Message::Terminate) | Err(_) => break,
    }
}
```

### Shutdown Graceful

```rust
fn shutdown(mut self) {
    // 1. Enviar Terminate a cada worker
    for _ in &self.workers {
        sender.send(Message::Terminate);
    }
    
    // 2. Join todos los workers
    for worker in self.workers.drain(..) {
        worker.thread.join();
    }
}
```

## ▶️ Ejecución

```bash
# Ejecutar demos
cargo run

# Ejecutar tests
cargo test

# Tests con output
cargo test -- --nocapture

# Release build (más rápido)
cargo run --release
```

## ✅ Criterios de Evaluación

| Criterio | Puntos |
|----------|--------|
| Pool funcional con N workers | 20 |
| Execute y shutdown correctos | 20 |
| Execute with result | 15 |
| Estadísticas | 15 |
| Tests pasan | 15 |
| Código limpio y documentado | 15 |

## 🎓 Ejercicios Avanzados

### 1. Timeout para Jobs

```rust
impl ThreadPool {
    fn execute_with_timeout<F>(&self, f: F, timeout: Duration) -> bool
    where F: FnOnce() + Send + 'static
}
```

### 2. Prioridad de Jobs

```rust
enum Priority { High, Medium, Low }

impl ThreadPool {
    fn execute_priority<F>(&self, f: F, priority: Priority)
}
```

### 3. Pool Dinámico

```rust
impl DynamicThreadPool {
    fn new(min: usize, max: usize) -> Self
    // Ajusta workers según carga
}
```

## 🔗 Recursos

- [Rust Book - Web Server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
- [threadpool crate](https://docs.rs/threadpool/)
- [rayon crate](https://docs.rs/rayon/)
