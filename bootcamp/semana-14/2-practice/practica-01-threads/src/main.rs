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
    ejemplo_thread_basico();

    // Ejemplo 2: Múltiples threads
    ejemplo_multiples_threads();

    // Ejemplo 3: Move closure
    ejemplo_move_closure();

    // Ejecuta los ejercicios
    println!("\n=== Ejercicios ===\n");

    // Descomenta para probar tus soluciones:
    // ejercicio_1_suma_paralela();
    // ejercicio_2_busqueda_paralela();
    // ejercicio_3_procesador_datos();
}

// ============================================================================
// EJEMPLOS
// ============================================================================

/// Ejemplo: Crear un thread básico y esperar su resultado
fn ejemplo_thread_basico() {
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
    let resultado = handle.join().expect("El thread falló");
    println!("Thread terminó con resultado: {}\n", resultado);
}

/// Ejemplo: Crear múltiples threads
fn ejemplo_multiples_threads() {
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
    let resultados: Vec<i32> = handles
        .into_iter()
        .map(|h| h.join().expect("Thread falló"))
        .collect();

    println!("Resultados: {:?}\n", resultados);
}

/// Ejemplo: Mover datos al thread con `move`
fn ejemplo_move_closure() {
    println!("--- Ejemplo: Move Closure ---");

    let datos = vec![1, 2, 3, 4, 5];
    let nombre = String::from("Procesador");

    // `move` transfiere ownership de `datos` y `nombre` al thread
    let handle = thread::spawn(move || {
        println!("  {} procesando: {:?}", nombre, datos);
        let suma: i32 = datos.iter().sum();
        suma
    });

    // Aquí ya no podemos usar `datos` ni `nombre`
    // println!("{}", nombre); // ERROR: value moved

    let resultado = handle.join().expect("Thread falló");
    println!("Suma calculada: {}\n", resultado);
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
fn ejercicio_1_suma_paralela() {
    println!("--- Ejercicio 1: Suma Paralela ---");

    let numeros: Vec<i32> = (1..=100).collect();
    let num_threads = 4;

    let resultado = suma_paralela(&numeros, num_threads);

    println!("Suma de 1 a 100 con {} threads: {}", num_threads, resultado);
    assert_eq!(resultado, 5050, "La suma debería ser 5050");
    println!("✓ Ejercicio 1 completado!\n");
}

fn suma_paralela(numeros: &[i32], num_threads: usize) -> i32 {
    // TODO: Implementa la suma paralela
    //
    // Pasos sugeridos:
    // 1. Calcular el tamaño de cada chunk: numeros.len() / num_threads
    // 2. Usar chunks() para dividir el slice
    // 3. Para cada chunk, crear un thread que calcule la suma
    // 4. Recolectar los handles en un Vec
    // 5. Hacer join() de todos y sumar los resultados
    //
    // Hint: Necesitarás clonar los datos para moverlos al thread
    // let chunk_vec: Vec<i32> = chunk.to_vec();

    todo!("Implementa suma_paralela")
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
fn ejercicio_2_busqueda_paralela() {
    println!("--- Ejercicio 2: Búsqueda Paralela ---");

    let datos: Vec<i32> = (0..1000).collect();
    let objetivo = 777;
    let num_threads = 4;

    let resultado = busqueda_paralela(&datos, objetivo, num_threads);

    println!("Buscando {} con {} threads", objetivo, num_threads);
    match resultado {
        Some(idx) => println!("Encontrado en índice: {}", idx),
        None => println!("No encontrado"),
    }
    assert_eq!(resultado, Some(777));
    println!("✓ Ejercicio 2 completado!\n");
}

fn busqueda_paralela(datos: &[i32], objetivo: i32, num_threads: usize) -> Option<usize> {
    // TODO: Implementa la búsqueda paralela
    //
    // Pasos sugeridos:
    // 1. Calcular el tamaño de cada chunk
    // 2. Para cada chunk, crear un thread que busque el objetivo
    // 3. El thread debe retornar Option<usize> con el índice GLOBAL
    //    (no el índice dentro del chunk)
    // 4. Recolectar resultados y retornar el primero que sea Some
    //
    // Hint: Para calcular el índice global:
    // indice_global = indice_chunk * chunk_size + indice_local

    todo!("Implementa busqueda_paralela")
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
fn ejercicio_3_procesador_datos() {
    println!("--- Ejercicio 3: Procesador de Datos ---");

    let datos: Vec<i32> = (1..=20).collect();
    let num_threads = 4;

    // Calcular cuadrados en paralelo
    let cuadrados = procesar_paralelo(&datos, |x| x * x, num_threads);

    println!("Datos originales: {:?}", datos);
    println!("Cuadrados: {:?}", cuadrados);

    let esperado: Vec<i32> = datos.iter().map(|x| x * x).collect();
    assert_eq!(cuadrados, esperado);
    println!("✓ Ejercicio 3 completado!\n");
}

fn procesar_paralelo<F>(datos: &[i32], transformar: F, num_threads: usize) -> Vec<i32>
where
    F: Fn(i32) -> i32 + Send + Sync + Clone + 'static,
{
    // TODO: Implementa el procesamiento paralelo
    //
    // Pasos sugeridos:
    // 1. Dividir datos en chunks
    // 2. Para cada chunk, clonar la función transformar
    // 3. Cada thread aplica transformar a cada elemento de su chunk
    // 4. Recolectar resultados manteniendo el orden
    //
    // Hint: Usa enumerate() en chunks para mantener el orden
    // Los resultados deben estar en un Vec<(usize, Vec<i32>)>
    // Luego ordenar por el índice y aplanar

    let _ = (datos, transformar, num_threads);
    todo!("Implementa procesar_paralelo")
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suma_paralela_basico() {
        let numeros: Vec<i32> = (1..=10).collect();
        let resultado = suma_paralela(&numeros, 2);
        assert_eq!(resultado, 55);
    }

    #[test]
    fn test_suma_paralela_grande() {
        let numeros: Vec<i32> = (1..=1000).collect();
        let resultado = suma_paralela(&numeros, 4);
        assert_eq!(resultado, 500500);
    }

    #[test]
    fn test_suma_paralela_un_thread() {
        let numeros: Vec<i32> = (1..=100).collect();
        let resultado = suma_paralela(&numeros, 1);
        assert_eq!(resultado, 5050);
    }

    #[test]
    fn test_busqueda_paralela_encontrado() {
        let datos: Vec<i32> = (0..100).collect();
        let resultado = busqueda_paralela(&datos, 50, 4);
        assert_eq!(resultado, Some(50));
    }

    #[test]
    fn test_busqueda_paralela_no_encontrado() {
        let datos: Vec<i32> = (0..100).collect();
        let resultado = busqueda_paralela(&datos, 200, 4);
        assert_eq!(resultado, None);
    }

    #[test]
    fn test_busqueda_paralela_primero() {
        let datos: Vec<i32> = (0..100).collect();
        let resultado = busqueda_paralela(&datos, 0, 4);
        assert_eq!(resultado, Some(0));
    }

    #[test]
    fn test_busqueda_paralela_ultimo() {
        let datos: Vec<i32> = (0..100).collect();
        let resultado = busqueda_paralela(&datos, 99, 4);
        assert_eq!(resultado, Some(99));
    }

    #[test]
    fn test_procesar_paralelo_cuadrados() {
        let datos = vec![1, 2, 3, 4, 5];
        let resultado = procesar_paralelo(&datos, |x| x * x, 2);
        assert_eq!(resultado, vec![1, 4, 9, 16, 25]);
    }

    #[test]
    fn test_procesar_paralelo_doble() {
        let datos = vec![1, 2, 3, 4];
        let resultado = procesar_paralelo(&datos, |x| x * 2, 2);
        assert_eq!(resultado, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_procesar_paralelo_orden() {
        let datos: Vec<i32> = (1..=100).collect();
        let resultado = procesar_paralelo(&datos, |x| x, 4);
        assert_eq!(resultado, datos);
    }
}
