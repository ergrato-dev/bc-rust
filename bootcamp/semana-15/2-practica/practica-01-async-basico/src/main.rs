//! Práctica 01: Async Básico
//!
//! Fundamentos de programación asíncrona con Tokio.

use std::future::Future;
use std::time::{Duration, Instant};
use tokio::time::sleep;

// =============================================================================
// EJERCICIO 1: Primera Función Async
// =============================================================================

/// Simula una operación lenta que toma `segundos` segundos.
///
/// # Comportamiento esperado
/// 1. Imprime "Iniciando {nombre}..."
/// 2. Espera `segundos` segundos
/// 3. Imprime "{nombre} completada"
/// 4. Retorna "{nombre} OK"
pub async fn operacion_lenta(nombre: &str, segundos: u64) -> String {
    // TODO: Implementar
    // Pista: usa tokio::time::sleep(Duration::from_secs(segundos)).await
    todo!("Implementar operacion_lenta")
}

// =============================================================================
// EJERCICIO 2: Secuencial vs Concurrente
// =============================================================================

/// Ejecuta tres operaciones de forma secuencial (una después de otra).
/// Retorna el tiempo total de ejecución.
pub async fn secuencial() -> Duration {
    let inicio = Instant::now();

    // TODO: Ejecutar secuencialmente:
    // - operacion_lenta("Tarea A", 1)
    // - operacion_lenta("Tarea B", 1)
    // - operacion_lenta("Tarea C", 1)
    // Tiempo esperado: ~3 segundos
    todo!("Implementar ejecución secuencial")

    // inicio.elapsed()
}

/// Ejecuta tres operaciones de forma concurrente.
/// Retorna el tiempo total de ejecución.
pub async fn concurrente() -> Duration {
    let inicio = Instant::now();

    // TODO: Ejecutar concurrentemente con tokio::join!:
    // - operacion_lenta("Tarea A", 1)
    // - operacion_lenta("Tarea B", 1)
    // - operacion_lenta("Tarea C", 1)
    // Tiempo esperado: ~1 segundo
    todo!("Implementar ejecución concurrente")

    // inicio.elapsed()
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
pub async fn con_timeout<T>(future: impl Future<Output = T>, timeout_ms: u64) -> Option<T> {
    // TODO: Implementar usando tokio::select!
    // Pista:
    // tokio::select! {
    //     resultado = future => Some(resultado),
    //     _ = sleep(Duration::from_millis(timeout_ms)) => None,
    // }
    todo!("Implementar con_timeout")
}

// =============================================================================
// MAIN
// =============================================================================

#[tokio::main]
async fn main() {
    println!("=== Práctica 01: Async Básico ===\n");

    // Ejercicio 1
    println!("--- Ejercicio 1: Operación Lenta ---");
    let resultado = operacion_lenta("MiTarea", 1).await;
    println!("Resultado: {}\n", resultado);

    // Ejercicio 2
    println!("--- Ejercicio 2: Secuencial vs Concurrente ---");
    let tiempo_sec = secuencial().await;
    println!("Tiempo secuencial: {:?}", tiempo_sec);

    let tiempo_con = concurrente().await;
    println!("Tiempo concurrente: {:?}", tiempo_con);
    println!(
        "Speedup: {:.2}x\n",
        tiempo_sec.as_secs_f64() / tiempo_con.as_secs_f64()
    );

    // Ejercicio 3
    println!("--- Ejercicio 3: Spawn Tasks ---");
    let resultados = spawn_tasks(5).await;
    for r in &resultados {
        println!("  {}", r);
    }
    println!();

    // Ejercicio 4
    println!("--- Ejercicio 4: Timeout ---");
    let rapido = con_timeout(
        async {
            sleep(Duration::from_millis(50)).await;
            "Rápido!"
        },
        100,
    )
    .await;
    println!("Operación rápida: {:?}", rapido);

    let lento = con_timeout(
        async {
            sleep(Duration::from_millis(200)).await;
            "Lento!"
        },
        100,
    )
    .await;
    println!("Operación lenta: {:?}", lento);
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_operacion_lenta() {
        let inicio = Instant::now();
        let resultado = operacion_lenta("Test", 1).await;
        let duracion = inicio.elapsed();

        assert!(resultado.contains("Test"));
        assert!(resultado.contains("OK"));
        assert!(duracion >= Duration::from_millis(900)); // Al menos 0.9 segundos
    }

    #[tokio::test]
    async fn test_secuencial_es_lento() {
        let duracion = secuencial().await;
        // Secuencial debe tomar al menos 2.5 segundos
        assert!(duracion >= Duration::from_millis(2500));
    }

    #[tokio::test]
    async fn test_concurrente_es_rapido() {
        let duracion = concurrente().await;
        // Concurrente debe tomar menos de 1.5 segundos
        assert!(duracion < Duration::from_millis(1500));
    }

    #[tokio::test]
    async fn test_spawn_tasks_cantidad() {
        let resultados = spawn_tasks(10).await;
        assert_eq!(resultados.len(), 10);
    }

    #[tokio::test]
    async fn test_spawn_tasks_contenido() {
        let resultados = spawn_tasks(3).await;
        // Verificar que cada resultado contiene "completada"
        for r in resultados {
            assert!(r.contains("completada"));
        }
    }

    #[tokio::test]
    async fn test_timeout_completa() {
        let resultado = con_timeout(
            async {
                sleep(Duration::from_millis(10)).await;
                42
            },
            100,
        )
        .await;

        assert_eq!(resultado, Some(42));
    }

    #[tokio::test]
    async fn test_timeout_expira() {
        let resultado = con_timeout(
            async {
                sleep(Duration::from_millis(200)).await;
                42
            },
            50,
        )
        .await;

        assert_eq!(resultado, None);
    }
}
