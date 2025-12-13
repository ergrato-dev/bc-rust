//! # Práctica 02: Comunicación con Channels
//!
//! En esta práctica aprenderás a:
//! - Crear channels con `std::sync::mpsc`
//! - Enviar mensajes entre threads
//! - Usar múltiples productores
//! - Iterar sobre mensajes recibidos

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Práctica 02: Channels ===\n");

    // Ejemplo 1: Channel básico
    example_basic_channel();

    // Ejemplo 2: Múltiples productores
    example_multiple_producers();

    // Ejemplo 3: Iterando mensajes
    example_iterating_messages();

    // Ejecuta los ejercicios
    println!("\n=== Ejercicios ===\n");

    // Descomenta para probar tus soluciones:
    // exercise_1_pipeline();
    // exercise_2_aggregator();
    // exercise_3_broadcast();
}

// ============================================================================
// EJEMPLOS
// ============================================================================

/// Ejemplo: Channel básico productor-consumidor
fn example_basic_channel() {
    println!("--- Ejemplo: Channel Básico ---");

    // Crear un channel
    let (tx, rx) = mpsc::channel();

    // Productor en otro thread
    let producer = thread::spawn(move || {
        let messages = vec!["Hola", "desde", "otro", "thread"];
        for msg in messages {
            tx.send(msg).expect("Error enviando");
            println!("  Enviado: {}", msg);
            thread::sleep(Duration::from_millis(50));
        }
        // tx se dropea aquí, cerrando el channel
    });

    // Consumidor en el thread principal
    println!("Esperando mensajes...");
    while let Ok(msg) = rx.recv() {
        println!("  Recibido: {}", msg);
    }
    println!("Channel cerrado");

    producer.join().unwrap();
    println!();
}

/// Ejemplo: Múltiples productores (MPSC)
fn example_multiple_producers() {
    println!("--- Ejemplo: Múltiples Productores ---");

    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    // Crear 3 productores
    for id in 0..3 {
        let tx_clone = tx.clone(); // Clonar el sender
        let handle = thread::spawn(move || {
            for i in 0..3 {
                let message = format!("P{}-M{}", id, i);
                tx_clone.send(message.clone()).unwrap();
                println!("  Productor {} envió: {}", id, message);
                thread::sleep(Duration::from_millis(30));
            }
        });
        handles.push(handle);
    }

    // Importante: dropear el tx original
    drop(tx);

    // Consumidor
    let mut count = 0;
    for msg in rx {
        println!("  Consumidor recibió: {}", msg);
        count += 1;
    }
    println!("Total mensajes: {}", count);

    for h in handles {
        h.join().unwrap();
    }
    println!();
}

/// Ejemplo: Iterando sobre mensajes con for
fn example_iterating_messages() {
    println!("--- Ejemplo: Iterando Mensajes ---");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i * 10).unwrap();
        }
    });

    // rx implementa Iterator
    let total: i32 = rx.iter().sum();
    println!("Suma de mensajes: {}\n", total);
}

// ============================================================================
// EJERCICIOS
// ============================================================================

/// # Ejercicio 1: Pipeline de Procesamiento
///
/// Implementa un pipeline de 3 etapas conectadas por channels:
/// 1. Generador: produce números del 1 al N
/// 2. Filtro: deja pasar solo los pares
/// 3. Transformador: multiplica por 10
///
/// ## Requisitos:
/// - Cada etapa corre en su propio thread
/// - Conectar etapas con channels
/// - Retornar el vector de resultados
///
/// ## Ejemplo:
/// ```
/// let resultado = pipeline(10);
/// assert_eq!(resultado, vec![20, 40, 60, 80, 100]);
/// ```
#[allow(dead_code)]
fn exercise_1_pipeline() {
    println!("--- Ejercicio 1: Pipeline ---");

    let result = pipeline(20);

    println!("Pipeline de 1 a 20:");
    println!("  Pares * 10 = {:?}", result);

    let expected: Vec<i32> = (1..=20).filter(|x| x % 2 == 0).map(|x| x * 10).collect();
    assert_eq!(result, expected);
    println!("✓ Ejercicio 1 completado!\n");
}

fn pipeline(n: i32) -> Vec<i32> {
    // TODO: Implementa el pipeline
    //
    // Estructura:
    // [Generador] --chan1--> [Filtro] --chan2--> [Transformador]
    //
    // 1. Crear dos channels: (tx1, rx1) y (tx2, rx2)
    // 2. Thread generador: envía 1..=n por tx1
    // 3. Thread filtro: recibe de rx1, si es par envía por tx2
    // 4. Thread transformador: recibe de rx2, multiplica por 10
    // 5. Recolectar resultados del transformador
    //
    // Hint: El transformador puede retornar Vec<i32> desde el thread

    let _ = n;
    todo!("Implementa pipeline")
}

/// # Ejercicio 2: Agregador de Resultados
///
/// Implementa un sistema donde múltiples workers procesan tareas
/// y envían resultados a un agregador central.
///
/// ## Requisitos:
/// - N workers procesan tareas (calcular cuadrado)
/// - Cada worker envía su resultado por un channel compartido
/// - El agregador suma todos los resultados
///
/// ## Ejemplo:
/// ```
/// let tareas = vec![1, 2, 3, 4, 5];
/// let resultado = agregador(&tareas, 2);
/// assert_eq!(resultado, 55); // 1 + 4 + 9 + 16 + 25
/// ```
#[allow(dead_code)]
fn exercise_2_aggregator() {
    println!("--- Ejercicio 2: Agregador ---");

    let tasks: Vec<i32> = (1..=10).collect();
    let num_workers = 3;

    let result = aggregator(&tasks, num_workers);

    println!("Tareas: {:?}", tasks);
    println!("Workers: {}", num_workers);
    println!("Suma de cuadrados: {}", result);

    let expected: i32 = tasks.iter().map(|x| x * x).sum();
    assert_eq!(result, expected);
    println!("✓ Ejercicio 2 completado!\n");
}

fn aggregator(tasks: &[i32], num_workers: usize) -> i32 {
    // TODO: Implementa el agregador
    //
    // Estrategia:
    // 1. Crear un channel para resultados (tx, rx)
    // 2. Dividir tasks entre workers (chunks)
    // 3. Cada worker:
    //    - Recibe su chunk de tareas
    //    - Calcula el cuadrado de cada una
    //    - Envía los resultados por tx (clonado)
    // 4. Dropear tx original
    // 5. El thread principal suma todos los mensajes de rx
    //
    // Hint: Puedes enviar cada resultado individual o la suma parcial

    let _ = (tasks, num_workers);
    todo!("Implementa aggregator")
}

/// # Ejercicio 3: Sistema de Broadcast
///
/// Implementa un sistema donde un productor envía mensajes
/// a múltiples consumidores (cada uno recibe todos los mensajes).
///
/// ## Requisitos:
/// - Un productor genera N mensajes
/// - M consumidores, cada uno con su propio channel
/// - Cada consumidor recibe TODOS los mensajes
/// - Retornar cuántos mensajes recibió cada consumidor
///
/// ## Ejemplo:
/// ```
/// let resultado = broadcast(5, 3); // 5 mensajes, 3 consumidores
/// assert_eq!(resultado, vec![5, 5, 5]);
/// ```
#[allow(dead_code)]
fn exercise_3_broadcast() {
    println!("--- Ejercicio 3: Broadcast ---");

    let num_messages = 10;
    let num_consumers = 4;

    let result = broadcast(num_messages, num_consumers);

    println!("Mensajes: {}", num_messages);
    println!("Consumidores: {}", num_consumers);
    println!("Mensajes recibidos por cada uno: {:?}", result);

    assert_eq!(result.len(), num_consumers);
    assert!(result.iter().all(|&c| c == num_messages));
    println!("✓ Ejercicio 3 completado!\n");
}

fn broadcast(num_messages: usize, num_consumers: usize) -> Vec<usize> {
    // TODO: Implementa el broadcast
    //
    // Estrategia:
    // 1. Crear un Vec de senders, uno por consumidor
    // 2. Para cada consumidor:
    //    - Crear channel (tx, rx)
    //    - Guardar tx en el Vec
    //    - Spawn thread que cuenta mensajes de rx
    // 3. El productor envía cada mensaje a TODOS los senders
    // 4. Dropear todos los senders
    // 5. Join todos los consumidores y recolectar conteos
    //
    // Hint: El mensaje debe ser Clone para enviarlo múltiples veces

    let _ = (num_messages, num_consumers);
    todo!("Implementa broadcast")
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_10() {
        let result = pipeline(10);
        assert_eq!(result, vec![20, 40, 60, 80, 100]);
    }

    #[test]
    fn test_pipeline_5() {
        let result = pipeline(5);
        assert_eq!(result, vec![20, 40]);
    }

    #[test]
    fn test_pipeline_1() {
        let result = pipeline(1);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_aggregator_simple() {
        let tasks = vec![1, 2, 3];
        let result = aggregator(&tasks, 2);
        assert_eq!(result, 14); // 1 + 4 + 9
    }

    #[test]
    fn test_aggregator_large() {
        let tasks: Vec<i32> = (1..=100).collect();
        let result = aggregator(&tasks, 4);
        let expected: i32 = (1..=100).map(|x| x * x).sum();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_aggregator_one_worker() {
        let tasks = vec![1, 2, 3, 4, 5];
        let result = aggregator(&tasks, 1);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_broadcast_basic() {
        let result = broadcast(5, 3);
        assert_eq!(result, vec![5, 5, 5]);
    }

    #[test]
    fn test_broadcast_many_consumers() {
        let result = broadcast(10, 5);
        assert_eq!(result.len(), 5);
        assert!(result.iter().all(|&c| c == 10));
    }

    #[test]
    fn test_broadcast_one_consumer() {
        let result = broadcast(7, 1);
        assert_eq!(result, vec![7]);
    }
}
