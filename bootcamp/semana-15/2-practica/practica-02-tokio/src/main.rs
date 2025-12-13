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
pub async fn contador_intervalo(n: u32) {
    // TODO: Implementar usando tokio::time::interval
    // 1. Crear un interval de 1 segundo
    // 2. En un loop de n iteraciones:
    //    - Esperar el tick
    //    - Imprimir "Tick {i}"
    todo!("Implementar contador_intervalo")
}

// =============================================================================
// EJERCICIO 2: Timeout con Reintentos
// =============================================================================

/// Ejecuta una operación con timeout y reintentos.
///
/// - Intenta ejecutar la operación
/// - Si excede el timeout, reintenta hasta `max_intentos`
/// - Retorna Some(resultado) si tiene éxito, None si todos los intentos fallan
pub async fn con_reintentos<T, F, Fut>(max_intentos: u32, timeout_ms: u64, operacion: F) -> Option<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = T>,
{
    // TODO: Implementar
    // 1. Loop hasta max_intentos
    // 2. En cada intento, usar tokio::time::timeout
    // 3. Si timeout, imprimir "Intento {i} fallido, reintentando..."
    // 4. Si éxito, retornar Some(resultado)
    // 5. Si todos fallan, retornar None
    todo!("Implementar con_reintentos")
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
pub async fn esperar_evento(
    rx_usuario: &mut mpsc::Receiver<String>,
    rx_shutdown: &mut oneshot::Receiver<()>,
    timeout_ms: u64,
) -> Evento {
    // TODO: Implementar usando tokio::select!
    // Pista:
    // tokio::select! {
    //     Some(msg) = rx_usuario.recv() => Evento::Usuario(msg),
    //     _ = rx_shutdown => Evento::Shutdown,
    //     _ = sleep(Duration::from_millis(timeout_ms)) => Evento::Timer,
    // }
    todo!("Implementar esperar_evento")
}

// =============================================================================
// EJERCICIO 4: JoinSet
// =============================================================================

/// Simula fetch de una URL (retorna después de un delay aleatorio).
async fn simular_fetch(url: String) -> String {
    // Simular latencia variable basada en la longitud de la URL
    let delay_ms = (url.len() as u64 * 10) % 500 + 100;
    sleep(Duration::from_millis(delay_ms)).await;
    format!("Contenido de {}", url)
}

/// Ejecuta fetch de múltiples URLs y retorna los primeros `n` resultados.
///
/// Usa JoinSet para manejar las tareas dinámicamente.
pub async fn fetch_primeros(urls: Vec<String>, n: usize) -> Vec<String> {
    // TODO: Implementar usando tokio::task::JoinSet
    // 1. Crear un JoinSet
    // 2. Agregar una task por cada URL usando set.spawn()
    // 3. Esperar los primeros n resultados con set.join_next()
    // 4. Retornar los resultados
    todo!("Implementar fetch_primeros")
}

// =============================================================================
// MAIN
// =============================================================================

#[tokio::main]
async fn main() {
    println!("=== Práctica 02: Tokio Runtime ===\n");

    // Ejercicio 1
    println!("--- Ejercicio 1: Contador Intervalo ---");
    contador_intervalo(3).await;
    println!();

    // Ejercicio 2
    println!("--- Ejercicio 2: Timeout con Reintentos ---");

    // Operación que siempre tarda más que el timeout
    let resultado_timeout = con_reintentos(3, 50, || async {
        sleep(Duration::from_millis(200)).await;
        "Resultado"
    })
    .await;
    println!("Resultado (timeout esperado): {:?}", resultado_timeout);

    // Operación que completa a tiempo
    let resultado_ok = con_reintentos(3, 200, || async {
        sleep(Duration::from_millis(50)).await;
        "Éxito!"
    })
    .await;
    println!("Resultado (éxito esperado): {:?}", resultado_ok);
    println!();

    // Ejercicio 3
    println!("--- Ejercicio 3: Select Múltiples Fuentes ---");
    let (tx_usuario, mut rx_usuario) = mpsc::channel::<String>(10);
    let (tx_shutdown, mut rx_shutdown) = oneshot::channel::<()>();

    // Enviar mensaje de usuario después de 100ms
    tokio::spawn(async move {
        sleep(Duration::from_millis(100)).await;
        tx_usuario.send("Hola!".to_string()).await.ok();
    });

    let evento = esperar_evento(&mut rx_usuario, &mut rx_shutdown, 500).await;
    println!("Evento recibido: {:?}", evento);

    // Probar timeout
    let (_tx_shutdown2, mut rx_shutdown2) = oneshot::channel::<()>();
    let evento_timeout = esperar_evento(&mut rx_usuario, &mut rx_shutdown2, 50).await;
    println!("Evento (timeout): {:?}", evento_timeout);
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

    let primeros = fetch_primeros(urls, 3).await;
    println!("Primeros 3 resultados:");
    for r in &primeros {
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
    async fn test_contador_intervalo_tiempo() {
        let inicio = Instant::now();
        contador_intervalo(2).await;
        let duracion = inicio.elapsed();

        // Debe tomar al menos 2 segundos
        assert!(duracion >= Duration::from_secs(2));
        // Pero no más de 2.5 segundos
        assert!(duracion < Duration::from_millis(2500));
    }

    #[tokio::test]
    async fn test_reintentos_exito() {
        let resultado = con_reintentos(3, 200, || async {
            sleep(Duration::from_millis(50)).await;
            42
        })
        .await;

        assert_eq!(resultado, Some(42));
    }

    #[tokio::test]
    async fn test_reintentos_fallo() {
        let resultado = con_reintentos(2, 50, || async {
            sleep(Duration::from_millis(200)).await;
            42
        })
        .await;

        assert_eq!(resultado, None);
    }

    #[tokio::test]
    async fn test_evento_usuario() {
        let (tx, mut rx) = mpsc::channel::<String>(1);
        let (_tx_shutdown, mut rx_shutdown) = oneshot::channel::<()>();

        tx.send("Test".to_string()).await.unwrap();

        let evento = esperar_evento(&mut rx, &mut rx_shutdown, 1000).await;
        assert_eq!(evento, Evento::Usuario("Test".to_string()));
    }

    #[tokio::test]
    async fn test_evento_shutdown() {
        let (_tx, mut rx) = mpsc::channel::<String>(1);
        let (tx_shutdown, mut rx_shutdown) = oneshot::channel::<()>();

        tx_shutdown.send(()).unwrap();

        let evento = esperar_evento(&mut rx, &mut rx_shutdown, 1000).await;
        assert_eq!(evento, Evento::Shutdown);
    }

    #[tokio::test]
    async fn test_evento_timer() {
        let (_tx, mut rx) = mpsc::channel::<String>(1);
        let (_tx_shutdown, mut rx_shutdown) = oneshot::channel::<()>();

        let evento = esperar_evento(&mut rx, &mut rx_shutdown, 50).await;
        assert_eq!(evento, Evento::Timer);
    }

    #[tokio::test]
    async fn test_fetch_primeros_cantidad() {
        let urls = vec![
            "a".to_string(),
            "bb".to_string(),
            "ccc".to_string(),
            "dddd".to_string(),
        ];

        let resultados = fetch_primeros(urls, 2).await;
        assert_eq!(resultados.len(), 2);
    }

    #[tokio::test]
    async fn test_fetch_primeros_contenido() {
        let urls = vec!["test".to_string()];

        let resultados = fetch_primeros(urls, 1).await;
        assert_eq!(resultados.len(), 1);
        assert!(resultados[0].contains("test"));
    }
}
