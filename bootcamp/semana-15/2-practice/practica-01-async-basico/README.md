# Práctica 01: Async Básico

## 🎯 Objetivo

Familiarizarse con los fundamentos de async/await en Rust usando Tokio.

## 📋 Ejercicios

### Ejercicio 1: Primera Función Async

Crea una función async que simule una operación lenta.

```rust
// TODO: Implementar
async fn operacion_lenta(nombre: &str, segundos: u64) -> String {
    // 1. Imprimir "Iniciando {nombre}..."
    // 2. Esperar `segundos` usando tokio::time::sleep
    // 3. Imprimir "{nombre} completada"
    // 4. Retornar "{nombre} OK"
    todo!()
}
```

### Ejercicio 2: Ejecución Secuencial vs Concurrente

Implementa dos funciones que ejecuten múltiples operaciones:

```rust
// Ejecuta operaciones una después de otra
async fn secuencial() -> Duration {
    todo!()
}

// Ejecuta operaciones concurrentemente con join!
async fn concurrente() -> Duration {
    todo!()
}
```

### Ejercicio 3: Spawn de Tasks

Crea tasks independientes que se ejecuten en paralelo:

```rust
async fn spawn_tasks(n: u32) -> Vec<String> {
    // 1. Crear n tasks con tokio::spawn
    // 2. Cada task debe retornar un mensaje
    // 3. Esperar todos los JoinHandles
    // 4. Retornar los resultados
    todo!()
}
```

### Ejercicio 4: Select - Carrera de Futures

Implementa un timeout usando select!:

```rust
async fn con_timeout<T>(
    future: impl Future<Output = T>,
    timeout_ms: u64,
) -> Option<T> {
    // Usar select! para implementar timeout
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
| operacion_lenta funciona correctamente | 25 |
| Diferencia medible entre secuencial y concurrente | 25 |
| spawn_tasks crea y espera todas las tasks | 25 |
| con_timeout retorna None en timeout | 25 |

## 💡 Pistas

- Usa `tokio::time::sleep` para simular trabajo
- `Instant::now()` y `elapsed()` para medir tiempo
- `tokio::join!` para concurrencia estructurada
- `tokio::select!` para carreras de futures
