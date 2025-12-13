//! # Práctica 01: Creación y Gestión de Threads
//!
//! En esta práctica aprenderás a:
//! - Crear threads con `std::thread::spawn`
//! - Esperar a que terminen con `join()`
//! - Mover ownership a threads con `move`
//! - Obtener información del thread actual

use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Práctica 01: Threads ===\n");

    // Ejemplo 1: Thread básico
    example_basic_thread();

    // Ejemplo 2: Múltiples threads
    example_multiple_threads();

    // Ejemplo 3: Move closure
    example_move_closure();

    // Ejecuta los ejercicios
    println!("\n=== Ejercicios ===\n");

    // Descomenta para probar tus soluciones:
    // exercise_1_parallel_sum();
    // exercise_2_parallel_search();
    // exercise_3_data_processor();
}

// ============================================================================
// EJEMPLOS
// ============================================================================

/// Ejemplo: Crear un thread básico y esperar su resultado
fn example_basic_thread() {
    println!("--- Ejemplo: Thread Básico ---");

    // Crear un thread
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  Thread hijo: iteración {}", i);
            thread::sleep(Duration::from_millis(100));
        }
        42 // Valor de retorno
    });

    // El thread principal continúa
    println!("Thread principal esperando...");

    // Esperar al thread y obtener su resultado
    let result = handle.join().expect("El thread falló");
    println!("Thread terminó con resultado: {}\n", result);
}

/// Ejemplo: Crear múltiples threads
fn example_multiple_threads() {
    println!("--- Ejemplo: Múltiples Threads ---");

    let mut handles = vec![];

    for id in 0..3 {
        let handle = thread::spawn(move || {
            println!("  Thread {} iniciado", id);
            thread::sleep(Duration::from_millis(50 * (id as u64 + 1)));
            println!("  Thread {} terminado", id);
            id * 10
        });
        handles.push(handle);
    }

    // Esperar a todos los threads
    let results: Vec<i32> = handles
        .into_iter()
        .map(|h| h.join().expect("Thread falló"))
        .collect();

    println!("Resultados: {:?}\n", results);
}

/// Ejemplo: Mover datos al thread con `move`
fn example_move_closure() {
    println!("--- Ejemplo: Move Closure ---");

    let data = vec![1, 2, 3, 4, 5];
    let name = String::from("Procesador");

    // `move` transfiere ownership de `data` y `name` al thread
    let handle = thread::spawn(move || {
        println!("  {} procesando: {:?}", name, data);
        let sum: i32 = data.iter().sum();
        sum
    });

    // Aquí ya no podemos usar `data` ni `name`
    // println!("{}", name); // ERROR: value moved

    let result = handle.join().expect("Thread falló");
    println!("Suma calculada: {}\n", result);
}

// ============================================================================
// EJERCICIOS
// ============================================================================

/// # Ejercicio 1: Suma Paralela
///
/// Implementa una función que divida un vector de números en N partes
/// y calcule la suma de cada parte en un thread separado.
///
/// ## Requisitos:
/// - Dividir el vector en `num_threads` partes aproximadamente iguales
/// - Cada thread calcula la suma de su parte
/// - El thread principal suma los resultados parciales
///
/// ## Ejemplo:
/// ```
/// let numeros = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let resultado = suma_paralela(&numeros, 2);
/// assert_eq!(resultado, 55);
/// ```
#[allow(dead_code)]
fn exercise_1_parallel_sum() {
    println!("--- Ejercicio 1: Suma Paralela ---");

    let numbers: Vec<i32> = (1..=100).collect();
    let num_threads = 4;

    let result = parallel_sum(&numbers, num_threads);

    println!("Suma de 1 a 100 con {} threads: {}", num_threads, result);
    assert_eq!(result, 5050, "La suma debería ser 5050");
    println!("✓ Ejercicio 1 completado!\n");
}

fn parallel_sum(numbers: &[i32], num_threads: usize) -> i32 {
    // TODO: Implementa la suma paralela
    //
    // Pasos sugeridos:
    // 1. Calcular el tamaño de cada chunk: numbers.len() / num_threads
    // 2. Usar chunks() para dividir el slice
    // 3. Para cada chunk, crear un thread que calcule la suma
    // 4. Recolectar los handles en un Vec
    // 5. Hacer join() de todos y sumar los resultados
    //
    // Hint: Necesitarás clonar los datos para moverlos al thread
    // let chunk_vec: Vec<i32> = chunk.to_vec();

    todo!("Implementa parallel_sum")
}

/// # Ejercicio 2: Búsqueda Paralela
///
/// Implementa una función que busque un elemento en un vector
/// usando múltiples threads en paralelo.
///
/// ## Requisitos:
/// - Dividir el vector en partes
/// - Cada thread busca en su parte
/// - Retornar el índice si se encuentra, None si no
///
/// ## Ejemplo:
/// ```
/// let datos = vec![5, 2, 8, 1, 9, 3, 7, 4, 6];
/// let resultado = busqueda_paralela(&datos, 7, 3);
/// assert_eq!(resultado, Some(6));
/// ```
#[allow(dead_code)]
fn exercise_2_parallel_search() {
    println!("--- Ejercicio 2: Búsqueda Paralela ---");

    let data: Vec<i32> = (0..1000).collect();
    let target = 777;
    let num_threads = 4;

    let result = parallel_search(&data, target, num_threads);

    println!("Buscando {} con {} threads", target, num_threads);
    match result {
        Some(idx) => println!("Encontrado en índice: {}", idx),
        None => println!("No encontrado"),
    }
    assert_eq!(result, Some(777));
    println!("✓ Ejercicio 2 completado!\n");
}

fn parallel_search(data: &[i32], target: i32, num_threads: usize) -> Option<usize> {
    // TODO: Implementa la búsqueda paralela
    //
    // Pasos sugeridos:
    // 1. Calcular el tamaño de cada chunk
    // 2. Para cada chunk, crear un thread que busque el target
    // 3. El thread debe retornar Option<usize> con el índice GLOBAL
    //    (no el índice dentro del chunk)
    // 4. Recolectar resultados y retornar el primero que sea Some
    //
    // Hint: Para calcular el índice global:
    // indice_global = indice_chunk * chunk_size + indice_local

    todo!("Implementa parallel_search")
}

/// # Ejercicio 3: Procesador de Datos
///
/// Implementa un procesador que aplique una transformación a cada
/// elemento de un vector en paralelo.
///
/// ## Requisitos:
/// - Dividir el trabajo entre threads
/// - Cada thread transforma su parte
/// - Reconstruir el vector con los resultados en orden
///
/// ## Ejemplo:
/// ```
/// let datos = vec![1, 2, 3, 4];
/// let resultado = procesar_paralelo(&datos, |x| x * x, 2);
/// assert_eq!(resultado, vec![1, 4, 9, 16]);
/// ```
#[allow(dead_code)]
fn exercise_3_data_processor() {
    println!("--- Ejercicio 3: Procesador de Datos ---");

    let data: Vec<i32> = (1..=20).collect();
    let num_threads = 4;

    // Calcular cuadrados en paralelo
    let squares = process_parallel(&data, |x| x * x, num_threads);

    println!("Datos originales: {:?}", data);
    println!("Cuadrados: {:?}", squares);

    let expected: Vec<i32> = data.iter().map(|x| x * x).collect();
    assert_eq!(squares, expected);
    println!("✓ Ejercicio 3 completado!\n");
}

fn process_parallel<F>(data: &[i32], transform: F, num_threads: usize) -> Vec<i32>
where
    F: Fn(i32) -> i32 + Send + Sync + Clone + 'static,
{
    // TODO: Implementa el procesamiento paralelo
    //
    // Pasos sugeridos:
    // 1. Dividir datos en chunks
    // 2. Para cada chunk, clonar la función transform
    // 3. Cada thread aplica transform a cada elemento de su chunk
    // 4. Recolectar resultados manteniendo el orden
    //
    // Hint: Usa enumerate() en chunks para mantener el orden
    // Los resultados deben estar en un Vec<(usize, Vec<i32>)>
    // Luego ordenar por el índice y aplanar

    let _ = (data, transform, num_threads);
    todo!("Implementa process_parallel")
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_sum_basic() {
        let numbers: Vec<i32> = (1..=10).collect();
        let result = parallel_sum(&numbers, 2);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_parallel_sum_large() {
        let numbers: Vec<i32> = (1..=1000).collect();
        let result = parallel_sum(&numbers, 4);
        assert_eq!(result, 500500);
    }

    #[test]
    fn test_parallel_sum_one_thread() {
        let numbers: Vec<i32> = (1..=100).collect();
        let result = parallel_sum(&numbers, 1);
        assert_eq!(result, 5050);
    }

    #[test]
    fn test_parallel_search_found() {
        let data: Vec<i32> = (0..100).collect();
        let result = parallel_search(&data, 50, 4);
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_parallel_search_not_found() {
        let data: Vec<i32> = (0..100).collect();
        let result = parallel_search(&data, 200, 4);
        assert_eq!(result, None);
    }

    #[test]
    fn test_parallel_search_first() {
        let data: Vec<i32> = (0..100).collect();
        let result = parallel_search(&data, 0, 4);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_parallel_search_last() {
        let data: Vec<i32> = (0..100).collect();
        let result = parallel_search(&data, 99, 4);
        assert_eq!(result, Some(99));
    }

    #[test]
    fn test_process_parallel_squares() {
        let data = vec![1, 2, 3, 4, 5];
        let result = process_parallel(&data, |x| x * x, 2);
        assert_eq!(result, vec![1, 4, 9, 16, 25]);
    }

    #[test]
    fn test_process_parallel_double() {
        let data = vec![1, 2, 3, 4];
        let result = process_parallel(&data, |x| x * 2, 2);
        assert_eq!(result, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_process_parallel_order() {
        let data: Vec<i32> = (1..=100).collect();
        let result = process_parallel(&data, |x| x, 4);
        assert_eq!(result, data);
    }
}
