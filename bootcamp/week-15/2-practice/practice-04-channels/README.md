# Práctica 04: Channels Async

## 🎯 Objetivo

Dominar los diferentes tipos de channels de Tokio para comunicación entre tasks.

## 📋 Ejercicios

### Ejercicio 1: mpsc - Work Queue

Implementa una cola de trabajo con múltiples workers:

```rust
/// Procesa items de una cola con múltiples workers.
async fn work_queue(items: Vec<i32>, num_workers: usize) -> Vec<i32> {
    // 1. Crear channel mpsc
    // 2. Spawn workers que procesan items
    // 3. Enviar items al channel
    // 4. Recolectar resultados
    todo!()
}
```

### Ejercicio 2: oneshot - Request/Response

Implementa un patrón request/response:

```rust
struct Request {
    data: String,
    respond_to: oneshot::Sender<String>,
}

/// Servidor que procesa requests.
async fn servidor(mut rx: mpsc::Receiver<Request>) {
    todo!()
}

/// Cliente que envía request y espera response.
async fn cliente(tx: mpsc::Sender<Request>, data: String) -> String {
    todo!()
}
```

### Ejercicio 3: broadcast - Pub/Sub

Implementa un sistema de notificaciones:

```rust
/// Publica mensajes a múltiples suscriptores.
async fn sistema_notificaciones(
    num_suscriptores: usize,
    mensajes: Vec<String>,
) -> Vec<Vec<String>> {
    todo!()
}
```

### Ejercicio 4: watch - Configuración Dinámica

Implementa configuración que se actualiza en runtime:

```rust
struct Config {
    timeout_ms: u64,
    max_retries: u32,
}

/// Worker que observa cambios de configuración.
async fn worker_con_config(
    mut config_rx: watch::Receiver<Config>,
    shutdown: oneshot::Receiver<()>,
) {
    todo!()
}
```

## 🧪 Tests

```bash
cargo test
```

## ✅ Criterios de Evaluación

| Criterio | Puntos |
|----------|--------|
| work_queue procesa todos los items | 25 |
| Request/Response funciona | 25 |
| broadcast entrega a todos | 25 |
| watch detecta cambios | 25 |
