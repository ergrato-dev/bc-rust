//! Práctica 04: Channels Async
//!
//! Comunicación entre tasks con channels de Tokio.

use std::time::Duration;
use tokio::sync::{broadcast, mpsc, oneshot, watch};
use tokio::time::sleep;

// =============================================================================
// EJERCICIO 1: mpsc - Work Queue
// =============================================================================

/// Procesa items con múltiples workers usando mpsc.
///
/// Cada worker:
/// 1. Recibe items del channel
/// 2. Procesa (multiplica por 2)
/// 3. Envía resultado al channel de resultados
pub async fn work_queue(items: Vec<i32>, num_workers: usize) -> Vec<i32> {
    // TODO: Implementar
    // 1. Crear channel para items (mpsc)
    // 2. Crear channel para resultados (mpsc)
    // 3. Spawn `num_workers` workers
    // 4. Cada worker loop: recv item, procesar, send resultado
    // 5. Enviar todos los items
    // 6. Cerrar channel de items (drop tx)
    // 7. Esperar workers y recolectar resultados
    todo!("Implementar work_queue")
}

// =============================================================================
// EJERCICIO 2: oneshot - Request/Response
// =============================================================================

/// Request con canal para la respuesta.
pub struct Request {
    pub data: String,
    pub respond_to: oneshot::Sender<String>,
}

/// Servidor que procesa requests y envía responses.
pub async fn server(mut rx: mpsc::Receiver<Request>) {
    // TODO: Implementar
    // Loop mientras haya requests:
    // 1. Recibir request
    // 2. Procesar (por ejemplo: data.to_uppercase())
    // 3. Enviar respuesta por respond_to
    todo!("Implementar server")
}

/// Cliente que envía un request y espera la respuesta.
pub async fn client(tx: mpsc::Sender<Request>, data: String) -> String {
    // TODO: Implementar
    // 1. Crear channel oneshot
    // 2. Crear Request con data y sender
    // 3. Enviar request
    // 4. Esperar y retornar respuesta
    todo!("Implementar client")
}

// =============================================================================
// EJERCICIO 3: broadcast - Pub/Sub
// =============================================================================

/// Sistema de notificaciones con múltiples suscriptores.
///
/// - Crea `num_subscribers` suscriptores
/// - Publica cada mensaje a todos
/// - Retorna los mensajes recibidos por cada suscriptor
pub async fn notification_system(
    num_subscribers: usize,
    messages: Vec<String>,
) -> Vec<Vec<String>> {
    // TODO: Implementar
    // 1. Crear broadcast channel
    // 2. Spawn tasks para cada suscriptor
    // 3. Cada suscriptor guarda mensajes recibidos
    // 4. Publicar todos los mensajes
    // 5. Esperar y retornar mensajes de cada suscriptor
    todo!("Implementar notification_system")
}

// =============================================================================
// EJERCICIO 4: watch - Configuración Dinámica
// =============================================================================

/// Configuración de la aplicación.
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub timeout_ms: u64,
    pub max_retries: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            timeout_ms: 1000,
            max_retries: 3,
        }
    }
}

/// Worker que reacciona a cambios de configuración.
///
/// - Observa cambios en la configuración
/// - Imprime cuando detecta un cambio
/// - Termina cuando recibe señal de shutdown
pub async fn worker_with_config(
    mut config_rx: watch::Receiver<Config>,
    mut shutdown: oneshot::Receiver<()>,
) -> Vec<Config> {
    // TODO: Implementar
    // 1. Vector para guardar configs vistas
    // 2. Loop con select!:
    //    - Si config cambia (config_rx.changed()), guardar nueva config
    //    - Si shutdown, retornar configs
    todo!("Implementar worker_with_config")
}

// =============================================================================
// MAIN
// =============================================================================

#[tokio::main]
async fn main() {
    println!("=== Práctica 04: Channels Async ===\n");

    // Ejercicio 1: Work Queue
    println!("--- Ejercicio 1: Work Queue ---");
    let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let results = work_queue(items.clone(), 3).await;
    println!("Items originales: {:?}", items);
    println!("Resultados: {:?}", results);
    println!();

    // Ejercicio 2: Request/Response
    println!("--- Ejercicio 2: Request/Response ---");
    let (tx, rx) = mpsc::channel::<Request>(10);

    // Iniciar servidor
    let server_handle = tokio::spawn(server(rx));

    // Enviar requests
    let resp1 = client(tx.clone(), "hola".to_string()).await;
    let resp2 = client(tx.clone(), "rust".to_string()).await;

    println!("Respuesta 1: {}", resp1);
    println!("Respuesta 2: {}", resp2);

    drop(tx); // Cerrar channel para que servidor termine
    server_handle.await.ok();
    println!();

    // Ejercicio 3: Broadcast
    println!("--- Ejercicio 3: Broadcast ---");
    let messages = vec![
        "Mensaje 1".to_string(),
        "Mensaje 2".to_string(),
        "Mensaje 3".to_string(),
    ];

    let received = notification_system(3, messages).await;
    for (i, msgs) in received.iter().enumerate() {
        println!("Suscriptor {}: {:?}", i, msgs);
    }
    println!();

    // Ejercicio 4: Watch
    println!("--- Ejercicio 4: Watch ---");
    let (config_tx, config_rx) = watch::channel(Config::default());
    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    // Iniciar worker
    let worker_handle = tokio::spawn(worker_with_config(config_rx, shutdown_rx));

    // Cambiar configuración
    sleep(Duration::from_millis(50)).await;
    config_tx
        .send(Config {
            timeout_ms: 2000,
            max_retries: 5,
        })
        .ok();

    sleep(Duration::from_millis(50)).await;
    config_tx
        .send(Config {
            timeout_ms: 500,
            max_retries: 1,
        })
        .ok();

    sleep(Duration::from_millis(50)).await;
    shutdown_tx.send(()).ok();

    let configs_seen = worker_handle.await.unwrap();
    println!("Configuraciones vistas:");
    for cfg in configs_seen {
        println!("  {:?}", cfg);
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_work_queue_processes_all() {
        let items = vec![1, 2, 3, 4, 5];
        let mut results = work_queue(items.clone(), 2).await;
        results.sort();

        // Cada item multiplicado por 2
        assert_eq!(results, vec![2, 4, 6, 8, 10]);
    }

    #[tokio::test]
    async fn test_work_queue_multiple_workers() {
        let items: Vec<i32> = (1..=100).collect();
        let results = work_queue(items.clone(), 5).await;

        assert_eq!(results.len(), 100);
    }

    #[tokio::test]
    async fn test_request_response() {
        let (tx, rx) = mpsc::channel::<Request>(10);

        tokio::spawn(server(rx));

        let resp = client(tx.clone(), "test".to_string()).await;
        assert_eq!(resp, "TEST");

        let resp2 = client(tx, "hello".to_string()).await;
        assert_eq!(resp2, "HELLO");
    }

    #[tokio::test]
    async fn test_broadcast_all_receive() {
        let messages = vec!["A".to_string(), "B".to_string()];
        let received = notification_system(3, messages.clone()).await;

        assert_eq!(received.len(), 3);
        for msgs in received {
            assert_eq!(msgs, messages);
        }
    }

    #[tokio::test]
    async fn test_watch_detects_changes() {
        let (tx, rx) = watch::channel(Config::default());
        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        let handle = tokio::spawn(worker_with_config(rx, shutdown_rx));

        // Cambiar config
        sleep(Duration::from_millis(10)).await;
        tx.send(Config {
            timeout_ms: 500,
            max_retries: 1,
        })
        .ok();

        sleep(Duration::from_millis(10)).await;
        shutdown_tx.send(()).ok();

        let configs = handle.await.unwrap();
        // Debe tener al menos la config inicial y el cambio
        assert!(configs.len() >= 2);
    }
}
