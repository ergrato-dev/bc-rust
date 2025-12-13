//! Práctica 02: Tokio Runtime
//!
//! Uso avanzado del runtime Tokio.

use std::future::Future;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinSet;
use tokio::time::{interval, sleep, timeout};

// =============================================================================
// EJERCICIO 1: Timer Interval
// =============================================================================

/// Imprime un contador cada segundo durante `n` segundos.
///
/// Ejemplo de salida:
/// ```text
/// Tick 1
/// Tick 2
/// Tick 3
/// ```
pub async fn interval_counter(n: u32) {
    // TODO: Implementar usando tokio::time::interval
    // 1. Crear un interval de 1 segundo
    // 2. En un loop de n iteraciones:
    //    - Esperar el tick
    //    - Imprimir "Tick {i}"
    todo!("Implementar interval_counter")
}

// =============================================================================
// EJERCICIO 2: Timeout con Reintentos
// =============================================================================

/// Ejecuta una operación con timeout y reintentos.
///
/// - Intenta ejecutar la operación
/// - Si excede el timeout, reintenta hasta `max_attempts`
/// - Retorna Some(resultado) si tiene éxito, None si todos los intentos fallan
pub async fn with_retries<T, F, Fut>(max_attempts: u32, timeout_ms: u64, operation: F) -> Option<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = T>,
{
    // TODO: Implementar
    // 1. Loop hasta max_attempts
    // 2. En cada intento, usar tokio::time::timeout
    // 3. Si timeout, imprimir "Intento {i} fallido, reintentando..."
    // 4. Si éxito, retornar Some(resultado)
    // 5. Si todos fallan, retornar None
    todo!("Implementar with_retries")
}

// =============================================================================
// EJERCICIO 3: Select con Múltiples Fuentes
// =============================================================================

/// Tipos de eventos posibles.
#[derive(Debug, Clone, PartialEq)]
pub enum Evento {
    Timer,
    Usuario(String),
    Shutdown,
}

/// Espera el primer evento de múltiples fuentes.
///
/// - Si llega un mensaje del usuario, retorna Evento::Usuario
/// - Si llega la señal de shutdown, retorna Evento::Shutdown
/// - Si expira el timeout, retorna Evento::Timer
pub async fn wait_for_event(
    rx_user: &mut mpsc::Receiver<String>,
    rx_shutdown: &mut oneshot::Receiver<()>,
    timeout_ms: u64,
) -> Evento {
    // TODO: Implementar usando tokio::select!
    // Pista:
    // tokio::select! {
    //     Some(msg) = rx_user.recv() => Evento::Usuario(msg),
    //     _ = rx_shutdown => Evento::Shutdown,
    //     _ = sleep(Duration::from_millis(timeout_ms)) => Evento::Timer,
    // }
    todo!("Implementar wait_for_event")
}

// =============================================================================
// EJERCICIO 4: JoinSet
// =============================================================================

/// Simula fetch de una URL (retorna después de un delay aleatorio).
async fn simulate_fetch(url: String) -> String {
    // Simular latencia variable basada en la longitud de la URL
    let delay_ms = (url.len() as u64 * 10) % 500 + 100;
    sleep(Duration::from_millis(delay_ms)).await;
    format!("Contenido de {}", url)
}

/// Ejecuta fetch de múltiples URLs y retorna los primeros `n` resultados.
///
/// Usa JoinSet para manejar las tareas dinámicamente.
pub async fn fetch_first(urls: Vec<String>, n: usize) -> Vec<String> {
    // TODO: Implementar usando tokio::task::JoinSet
    // 1. Crear un JoinSet
    // 2. Agregar una task por cada URL usando set.spawn()
    // 3. Esperar los primeros n resultados con set.join_next()
    // 4. Retornar los resultados
    todo!("Implementar fetch_first")
}

// =============================================================================
// MAIN
// =============================================================================

#[tokio::main]
async fn main() {
    println!("=== Práctica 02: Tokio Runtime ===\n");

    // Ejercicio 1
    println!("--- Ejercicio 1: Contador Intervalo ---");
    interval_counter(3).await;
    println!();

    // Ejercicio 2
    println!("--- Ejercicio 2: Timeout con Reintentos ---");

    // Operación que siempre tarda más que el timeout
    let result_timeout = with_retries(3, 50, || async {
        sleep(Duration::from_millis(200)).await;
        "Resultado"
    })
    .await;
    println!("Resultado (timeout esperado): {:?}", result_timeout);

    // Operación que completa a tiempo
    let result_ok = with_retries(3, 200, || async {
        sleep(Duration::from_millis(50)).await;
        "Éxito!"
    })
    .await;
    println!("Resultado (éxito esperado): {:?}", result_ok);
    println!();

    // Ejercicio 3
    println!("--- Ejercicio 3: Select Múltiples Fuentes ---");
    let (tx_user, mut rx_user) = mpsc::channel::<String>(10);
    let (tx_shutdown, mut rx_shutdown) = oneshot::channel::<()>();

    // Enviar mensaje de usuario después de 100ms
    tokio::spawn(async move {
        sleep(Duration::from_millis(100)).await;
        tx_user.send("Hola!".to_string()).await.ok();
    });

    let event = wait_for_event(&mut rx_user, &mut rx_shutdown, 500).await;
    println!("Evento recibido: {:?}", event);

    // Probar timeout
    let (_tx_shutdown2, mut rx_shutdown2) = oneshot::channel::<()>();
    let event_timeout = wait_for_event(&mut rx_user, &mut rx_shutdown2, 50).await;
    println!("Evento (timeout): {:?}", event_timeout);
    println!();

    // Ejercicio 4
    println!("--- Ejercicio 4: JoinSet ---");
    let urls = vec![
        "https://example.com".to_string(),
        "https://rust-lang.org".to_string(),
        "https://tokio.rs".to_string(),
        "https://crates.io".to_string(),
        "https://docs.rs".to_string(),
    ];

    let first_results = fetch_first(urls, 3).await;
    println!("Primeros 3 resultados:");
    for r in &first_results {
        println!("  {}", r);
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_interval_counter_time() {
        let start = Instant::now();
        interval_counter(2).await;
        let duration = start.elapsed();

        // Debe tomar al menos 2 segundos
        assert!(duration >= Duration::from_secs(2));
        // Pero no más de 2.5 segundos
        assert!(duration < Duration::from_millis(2500));
    }

    #[tokio::test]
    async fn test_retries_success() {
        let result = with_retries(3, 200, || async {
            sleep(Duration::from_millis(50)).await;
            42
        })
        .await;

        assert_eq!(result, Some(42));
    }

    #[tokio::test]
    async fn test_retries_failure() {
        let result = with_retries(2, 50, || async {
            sleep(Duration::from_millis(200)).await;
            42
        })
        .await;

        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_event_user() {
        let (tx, mut rx) = mpsc::channel::<String>(1);
        let (_tx_shutdown, mut rx_shutdown) = oneshot::channel::<()>();

        tx.send("Test".to_string()).await.unwrap();

        let event = wait_for_event(&mut rx, &mut rx_shutdown, 1000).await;
        assert_eq!(event, Evento::Usuario("Test".to_string()));
    }

    #[tokio::test]
    async fn test_event_shutdown() {
        let (_tx, mut rx) = mpsc::channel::<String>(1);
        let (tx_shutdown, mut rx_shutdown) = oneshot::channel::<()>();

        tx_shutdown.send(()).unwrap();

        let event = wait_for_event(&mut rx, &mut rx_shutdown, 1000).await;
        assert_eq!(event, Evento::Shutdown);
    }

    #[tokio::test]
    async fn test_event_timer() {
        let (_tx, mut rx) = mpsc::channel::<String>(1);
        let (_tx_shutdown, mut rx_shutdown) = oneshot::channel::<()>();

        let event = wait_for_event(&mut rx, &mut rx_shutdown, 50).await;
        assert_eq!(event, Evento::Timer);
    }

    #[tokio::test]
    async fn test_fetch_first_count() {
        let urls = vec![
            "a".to_string(),
            "bb".to_string(),
            "ccc".to_string(),
            "dddd".to_string(),
        ];

        let results = fetch_first(urls, 2).await;
        assert_eq!(results.len(), 2);
    }

    #[tokio::test]
    async fn test_fetch_first_content() {
        let urls = vec!["test".to_string()];

        let results = fetch_first(urls, 1).await;
        assert_eq!(results.len(), 1);
        assert!(results[0].contains("test"));
    }
}
