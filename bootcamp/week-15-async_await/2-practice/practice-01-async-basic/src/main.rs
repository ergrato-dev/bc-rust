//! Práctica 01: Async Básico
//!
//! Fundamentos de programación asíncrona con Tokio.

use std::future::Future;
use std::time::{Duration, Instant};
use tokio::time::sleep;

// =============================================================================
// EJERCICIO 1: Primera Función Async
// =============================================================================

/// Simula una operación lenta que toma `seconds` segundos.
///
/// # Comportamiento esperado
/// 1. Imprime "Iniciando {name}..."
/// 2. Espera `seconds` segundos
/// 3. Imprime "{name} completada"
/// 4. Retorna "{name} OK"
pub async fn slow_operation(name: &str, seconds: u64) -> String {
    // TODO: Implementar
    // Pista: usa tokio::time::sleep(Duration::from_secs(seconds)).await
    todo!("Implementar slow_operation")
}

// =============================================================================
// EJERCICIO 2: Secuencial vs Concurrente
// =============================================================================

/// Ejecuta tres operaciones de forma secuencial (una después de otra).
/// Retorna el tiempo total de ejecución.
pub async fn sequential() -> Duration {
    let start = Instant::now();

    // TODO: Ejecutar secuencialmente:
    // - slow_operation("Tarea A", 1)
    // - slow_operation("Tarea B", 1)
    // - slow_operation("Tarea C", 1)
    // Tiempo esperado: ~3 segundos
    todo!("Implementar ejecución secuencial")

    // start.elapsed()
}

/// Ejecuta tres operaciones de forma concurrente.
/// Retorna el tiempo total de ejecución.
pub async fn concurrent() -> Duration {
    let start = Instant::now();

    // TODO: Ejecutar concurrentemente con tokio::join!:
    // - slow_operation("Tarea A", 1)
    // - slow_operation("Tarea B", 1)
    // - slow_operation("Tarea C", 1)
    // Tiempo esperado: ~1 segundo
    todo!("Implementar ejecución concurrente")

    // start.elapsed()
}

// =============================================================================
// EJERCICIO 3: Spawn de Tasks
// =============================================================================

/// Crea `n` tasks independientes y espera sus resultados.
///
/// Cada task debe:
/// 1. Esperar 100ms
/// 2. Retornar "Task {i} completada"
pub async fn spawn_tasks(n: u32) -> Vec<String> {
    // TODO: Implementar
    // 1. Crear un Vec<JoinHandle<String>>
    // 2. Usar un loop para crear n tasks con tokio::spawn
    // 3. Cada task: sleep 100ms y retornar mensaje
    // 4. Esperar todos los handles y recolectar resultados
    todo!("Implementar spawn_tasks")
}

// =============================================================================
// EJERCICIO 4: Select - Timeout
// =============================================================================

/// Ejecuta un future con timeout.
///
/// - Si el future completa antes del timeout, retorna Some(resultado)
/// - Si el timeout expira primero, retorna None
pub async fn with_timeout<T>(future: impl Future<Output = T>, timeout_ms: u64) -> Option<T> {
    // TODO: Implementar usando tokio::select!
    // Pista:
    // tokio::select! {
    //     result = future => Some(result),
    //     _ = sleep(Duration::from_millis(timeout_ms)) => None,
    // }
    todo!("Implementar with_timeout")
}

// =============================================================================
// MAIN
// =============================================================================

#[tokio::main]
async fn main() {
    println!("=== Práctica 01: Async Básico ===\n");

    // Ejercicio 1
    println!("--- Ejercicio 1: Operación Lenta ---");
    let result = slow_operation("MiTarea", 1).await;
    println!("Resultado: {}\n", result);

    // Ejercicio 2
    println!("--- Ejercicio 2: Secuencial vs Concurrente ---");
    let time_seq = sequential().await;
    println!("Tiempo secuencial: {:?}", time_seq);

    let time_con = concurrent().await;
    println!("Tiempo concurrente: {:?}", time_con);
    println!(
        "Speedup: {:.2}x\n",
        time_seq.as_secs_f64() / time_con.as_secs_f64()
    );

    // Ejercicio 3
    println!("--- Ejercicio 3: Spawn Tasks ---");
    let results = spawn_tasks(5).await;
    for r in &results {
        println!("  {}", r);
    }
    println!();

    // Ejercicio 4
    println!("--- Ejercicio 4: Timeout ---");
    let fast = with_timeout(
        async {
            sleep(Duration::from_millis(50)).await;
            "Rápido!"
        },
        100,
    )
    .await;
    println!("Operación rápida: {:?}", fast);

    let slow = with_timeout(
        async {
            sleep(Duration::from_millis(200)).await;
            "Lento!"
        },
        100,
    )
    .await;
    println!("Operación lenta: {:?}", slow);
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_slow_operation() {
        let start = Instant::now();
        let result = slow_operation("Test", 1).await;
        let duration = start.elapsed();

        assert!(result.contains("Test"));
        assert!(result.contains("OK"));
        assert!(duration >= Duration::from_millis(900)); // Al menos 0.9 segundos
    }

    #[tokio::test]
    async fn test_sequential_is_slow() {
        let duration = sequential().await;
        // Secuencial debe tomar al menos 2.5 segundos
        assert!(duration >= Duration::from_millis(2500));
    }

    #[tokio::test]
    async fn test_concurrent_is_fast() {
        let duration = concurrent().await;
        // Concurrente debe tomar menos de 1.5 segundos
        assert!(duration < Duration::from_millis(1500));
    }

    #[tokio::test]
    async fn test_spawn_tasks_count() {
        let results = spawn_tasks(10).await;
        assert_eq!(results.len(), 10);
    }

    #[tokio::test]
    async fn test_spawn_tasks_content() {
        let results = spawn_tasks(3).await;
        // Verificar que cada resultado contiene "completada"
        for r in results {
            assert!(r.contains("completada"));
        }
    }

    #[tokio::test]
    async fn test_timeout_completes() {
        let result = with_timeout(
            async {
                sleep(Duration::from_millis(10)).await;
                42
            },
            100,
        )
        .await;

        assert_eq!(result, Some(42));
    }

    #[tokio::test]
    async fn test_timeout_expires() {
        let result = with_timeout(
            async {
                sleep(Duration::from_millis(200)).await;
                42
            },
            50,
        )
        .await;

        assert_eq!(result, None);
    }
}
