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
    ejemplo_channel_basico();

    // Ejemplo 2: Múltiples productores
    ejemplo_multiples_productores();

    // Ejemplo 3: Iterando mensajes
    ejemplo_iterando_mensajes();

    // Ejecuta los ejercicios
    println!("\n=== Ejercicios ===\n");

    // Descomenta para probar tus soluciones:
    // ejercicio_1_pipeline();
    // ejercicio_2_agregador();
    // ejercicio_3_broadcast();
}

// ============================================================================
// EJEMPLOS
// ============================================================================

/// Ejemplo: Channel básico productor-consumidor
fn ejemplo_channel_basico() {
    println!("--- Ejemplo: Channel Básico ---");

    // Crear un channel
    let (tx, rx) = mpsc::channel();

    // Productor en otro thread
    let productor = thread::spawn(move || {
        let mensajes = vec!["Hola", "desde", "otro", "thread"];
        for msg in mensajes {
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

    productor.join().unwrap();
    println!();
}

/// Ejemplo: Múltiples productores (MPSC)
fn ejemplo_multiples_productores() {
    println!("--- Ejemplo: Múltiples Productores ---");

    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    // Crear 3 productores
    for id in 0..3 {
        let tx_clone = tx.clone(); // Clonar el sender
        let handle = thread::spawn(move || {
            for i in 0..3 {
                let mensaje = format!("P{}-M{}", id, i);
                tx_clone.send(mensaje.clone()).unwrap();
                println!("  Productor {} envió: {}", id, mensaje);
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
fn ejemplo_iterando_mensajes() {
    println!("--- Ejemplo: Iterando Mensajes ---");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i * 10).unwrap();
        }
    });

    // rx implementa Iterator
    let suma: i32 = rx.iter().sum();
    println!("Suma de mensajes: {}\n", suma);
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
fn ejercicio_1_pipeline() {
    println!("--- Ejercicio 1: Pipeline ---");

    let resultado = pipeline(20);

    println!("Pipeline de 1 a 20:");
    println!("  Pares * 10 = {:?}", resultado);

    let esperado: Vec<i32> = (1..=20).filter(|x| x % 2 == 0).map(|x| x * 10).collect();
    assert_eq!(resultado, esperado);
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
fn ejercicio_2_agregador() {
    println!("--- Ejercicio 2: Agregador ---");

    let tareas: Vec<i32> = (1..=10).collect();
    let num_workers = 3;

    let resultado = agregador(&tareas, num_workers);

    println!("Tareas: {:?}", tareas);
    println!("Workers: {}", num_workers);
    println!("Suma de cuadrados: {}", resultado);

    let esperado: i32 = tareas.iter().map(|x| x * x).sum();
    assert_eq!(resultado, esperado);
    println!("✓ Ejercicio 2 completado!\n");
}

fn agregador(tareas: &[i32], num_workers: usize) -> i32 {
    // TODO: Implementa el agregador
    //
    // Estrategia:
    // 1. Crear un channel para resultados (tx, rx)
    // 2. Dividir tareas entre workers (chunks)
    // 3. Cada worker:
    //    - Recibe su chunk de tareas
    //    - Calcula el cuadrado de cada una
    //    - Envía los resultados por tx (clonado)
    // 4. Dropear tx original
    // 5. El thread principal suma todos los mensajes de rx
    //
    // Hint: Puedes enviar cada resultado individual o la suma parcial

    let _ = (tareas, num_workers);
    todo!("Implementa agregador")
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
fn ejercicio_3_broadcast() {
    println!("--- Ejercicio 3: Broadcast ---");

    let num_mensajes = 10;
    let num_consumidores = 4;

    let resultado = broadcast(num_mensajes, num_consumidores);

    println!("Mensajes: {}", num_mensajes);
    println!("Consumidores: {}", num_consumidores);
    println!("Mensajes recibidos por cada uno: {:?}", resultado);

    assert_eq!(resultado.len(), num_consumidores);
    assert!(resultado.iter().all(|&c| c == num_mensajes));
    println!("✓ Ejercicio 3 completado!\n");
}

fn broadcast(num_mensajes: usize, num_consumidores: usize) -> Vec<usize> {
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

    let _ = (num_mensajes, num_consumidores);
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
        let resultado = pipeline(10);
        assert_eq!(resultado, vec![20, 40, 60, 80, 100]);
    }

    #[test]
    fn test_pipeline_5() {
        let resultado = pipeline(5);
        assert_eq!(resultado, vec![20, 40]);
    }

    #[test]
    fn test_pipeline_1() {
        let resultado = pipeline(1);
        assert_eq!(resultado, vec![]);
    }

    #[test]
    fn test_agregador_simple() {
        let tareas = vec![1, 2, 3];
        let resultado = agregador(&tareas, 2);
        assert_eq!(resultado, 14); // 1 + 4 + 9
    }

    #[test]
    fn test_agregador_grande() {
        let tareas: Vec<i32> = (1..=100).collect();
        let resultado = agregador(&tareas, 4);
        let esperado: i32 = (1..=100).map(|x| x * x).sum();
        assert_eq!(resultado, esperado);
    }

    #[test]
    fn test_agregador_un_worker() {
        let tareas = vec![1, 2, 3, 4, 5];
        let resultado = agregador(&tareas, 1);
        assert_eq!(resultado, 55);
    }

    #[test]
    fn test_broadcast_basico() {
        let resultado = broadcast(5, 3);
        assert_eq!(resultado, vec![5, 5, 5]);
    }

    #[test]
    fn test_broadcast_muchos_consumidores() {
        let resultado = broadcast(10, 5);
        assert_eq!(resultado.len(), 5);
        assert!(resultado.iter().all(|&c| c == 10));
    }

    #[test]
    fn test_broadcast_un_consumidor() {
        let resultado = broadcast(7, 1);
        assert_eq!(resultado, vec![7]);
    }
}
