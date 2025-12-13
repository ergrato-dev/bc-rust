//! # Práctica 04: Patrones de Concurrencia
//!
//! En esta práctica aprenderás a implementar:
//! - Worker Pool (Thread Pool)
//! - Map-Reduce paralelo
//! - Pipeline de procesamiento

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Práctica 04: Patrones de Concurrencia ===\n");

    // Ejemplo: Fork-Join simple
    example_fork_join();

    // Ejecuta los ejercicios
    println!("\n=== Ejercicios ===\n");

    // Descomenta para probar tus soluciones:
    // exercise_1_worker_pool();
    // exercise_2_map_reduce();
    // exercise_3_processing_pipeline();
}

// ============================================================================
// EJEMPLOS
// ============================================================================

/// Ejemplo: Patrón Fork-Join
fn example_fork_join() {
    println!("--- Ejemplo: Fork-Join ---");

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let num_threads = 4;
    let chunk_size = (data.len() + num_threads - 1) / num_threads;

    // Fork: dividir trabajo
    let handles: Vec<_> = data
        .chunks(chunk_size)
        .map(|chunk| {
            let chunk = chunk.to_vec();
            thread::spawn(move || {
                let sum: i32 = chunk.iter().sum();
                println!("  Thread procesó {:?} = {}", chunk, sum);
                sum
            })
        })
        .collect();

    // Join: combinar resultados
    let total: i32 = handles.into_iter().map(|h| h.join().unwrap()).sum();

    println!("  Total: {}\n", total);
}

// ============================================================================
// EJERCICIOS
// ============================================================================

/// # Ejercicio 1: Worker Pool
///
/// Implementa un pool de workers que procesan jobs de una cola.
///
/// ## Requisitos:
/// - `new(num_workers)` crea el pool con N workers
/// - `execute(job)` envía un job al pool
/// - Los workers procesan jobs concurrentemente
/// - `shutdown()` espera a que terminen todos los jobs
#[allow(dead_code)]
fn exercise_1_worker_pool() {
    println!("--- Ejercicio 1: Worker Pool ---");

    let pool = WorkerPool::new(4);
    let results = Arc::new(Mutex::new(Vec::new()));

    // Enviar 10 jobs
    for i in 0..10 {
        let res = Arc::clone(&results);
        pool.execute(move || {
            thread::sleep(Duration::from_millis(50));
            let mut r = res.lock().unwrap();
            r.push(i * i);
            println!("  Job {} completado", i);
        });
    }

    // Esperar a que terminen
    pool.shutdown();

    let res = results.lock().unwrap();
    println!("  Resultados: {:?}", *res);
    assert_eq!(res.len(), 10);
    println!("✓ Ejercicio 1 completado!\n");
}

struct WorkerPool {
    // TODO: Agrega campos necesarios
    // - workers: Vec<thread::JoinHandle<()>>
    // - sender: Option<mpsc::Sender<Job>>
    workers: Vec<thread::JoinHandle<()>>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl WorkerPool {
    fn new(num_workers: usize) -> Self {
        // TODO: Implementa
        //
        // 1. Crear channel para jobs
        // 2. Envolver receiver en Arc<Mutex<>> para compartir
        // 3. Crear num_workers threads que:
        //    - Hacen loop infinito
        //    - Obtienen lock del receiver
        //    - recv() un job
        //    - Ejecutan el job
        //    - Si recv() retorna Err, salir del loop

        let _ = num_workers;
        todo!("Implementa new")
    }

    fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // TODO: Enviar el job por el channel
        let _ = job;
        todo!("Implementa execute")
    }

    fn shutdown(self) {
        // TODO: Implementa
        // 1. Dropear el sender (cerrar channel)
        // 2. Join todos los workers
        todo!("Implementa shutdown")
    }
}

/// # Ejercicio 2: Map-Reduce
///
/// Implementa el patrón Map-Reduce para procesar datos en paralelo.
///
/// ## Requisitos:
/// - `map_reduce(datos, map_fn, reduce_fn, num_threads)`
/// - Dividir datos entre threads
/// - Cada thread aplica map_fn a sus datos
/// - Combinar resultados con reduce_fn
#[allow(dead_code)]
fn exercise_2_map_reduce() {
    println!("--- Ejercicio 2: Map-Reduce ---");

    let words = vec![
        "hola", "mundo", "rust", "es", "genial", "hola", "rust", "rust",
    ];

    // Contar frecuencia de palabras
    let frequencies = map_reduce(
        &words,
        |word| vec![(word.to_string(), 1)],             // Map: emitir (palabra, 1)
        |counts| {                                       // Reduce: sumar counts
            let mut map = std::collections::HashMap::new();
            for (word, count) in counts {
                *map.entry(word).or_insert(0) += count;
            }
            map.into_iter().collect()
        },
        2,
    );

    println!("  Frecuencias: {:?}", frequencies);

    // Verificar
    let rust_count: i32 = frequencies
        .iter()
        .filter(|(p, _)| p == "rust")
        .map(|(_, c)| c)
        .sum();
    assert_eq!(rust_count, 3);
    println!("✓ Ejercicio 2 completado!\n");
}

fn map_reduce<T, K, V, M, R>(
    data: &[T],
    map_fn: M,
    reduce_fn: R,
    num_threads: usize,
) -> Vec<(K, V)>
where
    T: Clone + Send + Sync + 'static,
    K: Clone + Send + 'static,
    V: Clone + Send + 'static,
    M: Fn(&T) -> Vec<(K, V)> + Send + Sync + Clone + 'static,
    R: Fn(Vec<(K, V)>) -> Vec<(K, V)>,
{
    // TODO: Implementa Map-Reduce
    //
    // 1. Dividir data en chunks
    // 2. Cada thread aplica map_fn a cada elemento de su chunk
    // 3. Recolectar todos los (K, V) de todos los threads
    // 4. Aplicar reduce_fn al resultado combinado
    //
    // Hint: Cada thread retorna Vec<(K, V)>

    let _ = (data, map_fn, reduce_fn, num_threads);
    todo!("Implementa map_reduce")
}

/// # Ejercicio 3: Pipeline de Procesamiento de Imágenes (Simulado)
///
/// Implementa un pipeline de procesamiento con etapas:
/// 1. Generador: produce "imágenes" (números)
/// 2. Filtro: descarta imágenes inválidas (negativos)
/// 3. Transformador: aplica procesamiento (duplicar)
/// 4. Guardador: guarda resultados
///
/// ## Requisitos:
/// - Cada etapa corre en su propio thread
/// - Conectadas por channels
/// - Retornar las imágenes procesadas
#[allow(dead_code)]
fn exercise_3_processing_pipeline() {
    println!("--- Ejercicio 3: Pipeline de Procesamiento ---");

    let input_images = vec![1, -2, 3, -4, 5, 6, -7, 8, 9, 10];
    let result = image_pipeline(input_images.clone());

    println!("  Entrada: {:?}", input_images);
    println!("  Salida:  {:?}", result);

    // Solo positivos, duplicados
    let expected: Vec<i32> = input_images
        .iter()
        .filter(|&&x| x > 0)
        .map(|&x| x * 2)
        .collect();

    assert_eq!(result, expected);
    println!("✓ Ejercicio 3 completado!\n");
}

fn image_pipeline(images: Vec<i32>) -> Vec<i32> {
    // TODO: Implementa el pipeline
    //
    // Etapa 1 - Generador:
    //   - Recibe images
    //   - Envía cada una por chan1
    //
    // Etapa 2 - Filtro:
    //   - Recibe de chan1
    //   - Si > 0, envía por chan2
    //
    // Etapa 3 - Transformador:
    //   - Recibe de chan2
    //   - Multiplica por 2
    //   - Envía por chan3
    //
    // Etapa 4 - Guardador:
    //   - Recibe de chan3
    //   - Guarda en Vec
    //   - Retorna Vec al final
    //
    // Estructura:
    // [Gen] --chan1--> [Filtro] --chan2--> [Transform] --chan3--> [Guardador]

    let _ = images;
    todo!("Implementa image_pipeline")
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_pool_basic() {
        let pool = WorkerPool::new(2);
        let counter = Arc::new(Mutex::new(0));

        for _ in 0..5 {
            let c = Arc::clone(&counter);
            pool.execute(move || {
                let mut num = c.lock().unwrap();
                *num += 1;
            });
        }

        pool.shutdown();
        assert_eq!(*counter.lock().unwrap(), 5);
    }

    #[test]
    fn test_worker_pool_order() {
        let pool = WorkerPool::new(4);
        let results = Arc::new(Mutex::new(Vec::new()));

        for i in 0..10 {
            let r = Arc::clone(&results);
            pool.execute(move || {
                let mut v = r.lock().unwrap();
                v.push(i);
            });
        }

        pool.shutdown();
        assert_eq!(results.lock().unwrap().len(), 10);
    }

    #[test]
    fn test_map_reduce_sum() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let result = map_reduce(
            &numbers,
            |&n| vec![("sum".to_string(), n)],
            |pairs| {
                let sum: i32 = pairs.iter().map(|(_, v)| v).sum();
                vec![("total".to_string(), sum)]
            },
            2,
        );

        assert_eq!(result, vec![("total".to_string(), 55)]);
    }

    #[test]
    fn test_pipeline_only_positives() {
        let input = vec![1, -1, 2, -2, 3];
        let output = image_pipeline(input);
        assert_eq!(output, vec![2, 4, 6]);
    }

    #[test]
    fn test_pipeline_all_positives() {
        let input = vec![1, 2, 3, 4, 5];
        let output = image_pipeline(input);
        assert_eq!(output, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_pipeline_all_negatives() {
        let input = vec![-1, -2, -3];
        let output = image_pipeline(input);
        assert!(output.is_empty());
    }

    #[test]
    fn test_pipeline_empty() {
        let input: Vec<i32> = vec![];
        let output = image_pipeline(input);
        assert!(output.is_empty());
    }
}
