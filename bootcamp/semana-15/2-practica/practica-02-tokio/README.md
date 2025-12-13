# Práctica 02: Tokio Runtime

## 🎯 Objetivo

Dominar el uso del runtime Tokio: spawn, join!, select!, y timers.

## 📋 Ejercicios

### Ejercicio 1: Timer Interval

Implementa un contador que imprima cada segundo:

```rust
/// Imprime un contador cada segundo durante `n` segundos.
async fn contador_intervalo(n: u32) {
    // Usar tokio::time::interval
    todo!()
}
```

### Ejercicio 2: Timeout Robusto

Implementa una función que ejecute una operación con timeout y reintentos:

```rust
/// Ejecuta `operacion` con timeout. Si falla, reintenta hasta `max_intentos`.
async fn con_reintentos<T, F, Fut>(
    max_intentos: u32,
    timeout_ms: u64,
    operacion: F,
) -> Option<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = T>,
{
    todo!()
}
```

### Ejercicio 3: Select con Múltiples Fuentes

Implementa un manejador que responda al primer evento:

```rust
enum Evento {
    Timer,
    Usuario(String),
    Shutdown,
}

/// Espera múltiples fuentes de eventos y retorna el primero.
async fn esperar_evento(
    rx_usuario: &mut mpsc::Receiver<String>,
    rx_shutdown: &mut oneshot::Receiver<()>,
    timeout_ms: u64,
) -> Evento {
    todo!()
}
```

### Ejercicio 4: JoinSet

Usa JoinSet para ejecutar y recolectar tareas dinámicamente:

```rust
/// Ejecuta `urls` concurrentemente (simulado) y retorna los primeros `n` resultados.
async fn fetch_primeros(urls: Vec<String>, n: usize) -> Vec<String> {
    // Usar tokio::task::JoinSet
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
| contador_intervalo funciona correctamente | 25 |
| con_reintentos maneja timeouts y retries | 25 |
| esperar_evento responde al primer evento | 25 |
| fetch_primeros usa JoinSet correctamente | 25 |
