//! Práctica 04: Iteradores
//!
//! En esta práctica aprenderás a:
//! - Usar adaptadores de iteradores (map, filter, etc.)
//! - Usar consumidores (collect, fold, sum, etc.)
//! - Crear iteradores personalizados
//! - Encadenar operaciones de forma eficiente

fn main() {
    println!("=== Práctica 04: Iteradores ===\n");

    // Ejercicio 1: Transformar con map y filter
    println!("Ejercicio 1: Map y Filter");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = cuadrados_pares(&numbers);
    println!("  Cuadrados de pares: {:?}\n", result);

    // Ejercicio 2: Usar fold para acumular
    println!("Ejercicio 2: Fold");
    let words = vec!["Hola", " ", "Rust", "!"];
    let concatenated = concatenar(&words);
    println!("  Concatenado: {}\n", concatenated);

    // Ejercicio 3: Encontrar elementos
    println!("Ejercicio 3: Find y Position");
    let data = vec![10, 25, 30, 45, 50];
    if let Some((pos, val)) = primer_mayor_que(&data, 28) {
        println!("  Primer mayor que 28: {} en posición {}\n", val, pos);
    }

    // Ejercicio 4: Zip y enumerate
    println!("Ejercicio 4: Zip");
    let names = vec!["Ana", "Bob", "Carlos"];
    let ages = vec![25, 30, 35];
    let people = combinar_datos(&names, &ages);
    println!("  Personas: {:?}\n", people);

    // Ejercicio 5: Implementar Iterator
    println!("Ejercicio 5: Iterator personalizado");
    let fib = Fibonacci::new();
    let first_10: Vec<u64> = fib.take(10).collect();
    println!("  Fibonacci: {:?}\n", first_10);

    println!("✅ Todos los ejercicios completados!");
}

// ============================================================
// EJERCICIO 1: Map y Filter
// ============================================================
// Retorna los cuadrados de los números pares.
// Entrada: [1, 2, 3, 4, 5, 6] -> Salida: [4, 16, 36]
//
// TODO: Usa filter para pares, map para cuadrados
// ============================================================

fn cuadrados_pares(numeros: &[i32]) -> Vec<i32> {
    // TODO: Implementa usando iteradores
    todo!("Implementa cuadrados_pares")
}

// ============================================================
// EJERCICIO 2: Fold para concatenar
// ============================================================
// Concatena todos los strings en uno solo.
//
// TODO: Usa fold con String::new() como acumulador
// ============================================================

fn concatenar(palabras: &[&str]) -> String {
    // TODO: Implementa usando fold
    todo!("Implementa concatenar")
}

// ============================================================
// EJERCICIO 3: Find con position
// ============================================================
// Encuentra el primer elemento mayor que `umbral`.
// Retorna Some((posición, valor)) o None.
//
// TODO: Usa enumerate y find
// ============================================================

fn primer_mayor_que(datos: &[i32], umbral: i32) -> Option<(usize, i32)> {
    // TODO: Implementa usando enumerate y find
    todo!("Implementa primer_mayor_que")
}

// ============================================================
// EJERCICIO 4: Zip para combinar
// ============================================================
// Combina nombres y edades en tuplas (nombre, edad).
//
// TODO: Usa zip y collect
// ============================================================

fn combinar_datos<'a>(names: &'a [&str], ages: &[i32]) -> Vec<(&'a str, i32)> {
    // TODO: Implementa usando zip
    todo!("Implementa combinar_datos")
}

// ============================================================
// EJERCICIO 5: Implementar Iterator - Fibonacci
// ============================================================
// Crea un iterador que genere la secuencia de Fibonacci.
// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...
//
// TODO: Implementa el trait Iterator para Fibonacci
// ============================================================

struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Implementa la lógica de Fibonacci
        // 1. Guarda el valor actual
        // 2. Calcula el siguiente (actual + siguiente)
        // 3. Actualiza actual y siguiente
        // 4. Retorna Some(valor_guardado)
        todo!("Implementa Fibonacci::next")
    }
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cuadrados_pares() {
        assert_eq!(cuadrados_pares(&[1, 2, 3, 4, 5, 6]), vec![4, 16, 36]);
        assert_eq!(cuadrados_pares(&[1, 3, 5]), Vec::<i32>::new());
        assert_eq!(cuadrados_pares(&[2]), vec![4]);
    }

    #[test]
    fn test_concatenar() {
        assert_eq!(concatenar(&["a", "b", "c"]), "abc");
        assert_eq!(concatenar(&["Hola", " ", "Mundo"]), "Hola Mundo");
        assert_eq!(concatenar(&[]), "");
    }

    #[test]
    fn test_primer_mayor_que() {
        assert_eq!(primer_mayor_que(&[1, 5, 10, 15], 7), Some((2, 10)));
        assert_eq!(primer_mayor_que(&[1, 2, 3], 10), None);
        assert_eq!(primer_mayor_que(&[100], 50), Some((0, 100)));
    }

    #[test]
    fn test_combinar_datos() {
        let nombres = vec!["A", "B"];
        let edades = vec![1, 2];
        assert_eq!(combinar_datos(&nombres, &edades), vec![("A", 1), ("B", 2)]);
    }

    #[test]
    fn test_fibonacci() {
        let fib: Vec<u64> = Fibonacci::new().take(10).collect();
        assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }
}
