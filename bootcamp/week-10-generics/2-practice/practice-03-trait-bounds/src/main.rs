// ============================================
// PRÁCTICA 03: Trait Bounds
// ============================================
// Objetivo: Dominar la restricción de tipos genéricos con traits
//
// Instrucciones:
// 1. Implementa cada función marcada con TODO
// 2. Presta atención a los trait bounds requeridos
// 3. Ejecuta `cargo test` para verificar tu implementación

use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::collections::HashMap;

fn main() {
    println!("=== Práctica 03: Trait Bounds ===\n");

    // Ejercicio 1: Imprimir con Debug
    let numbers = vec![1, 2, 3];
    let point = (10, 20);
    print_debug(&numbers);
    print_debug(&point);

    // Ejercicio 2: Comparar y mostrar
    println!();
    show_larger(10, 5);
    show_larger("zebra", "apple");

    // Ejercicio 3: Clonar si es mayor
    println!();
    let result = clone_if_larger(&100, &50);
    println!("Cloned if larger: {:?}", result);

    // Ejercicio 4: Contar ocurrencias
    println!();
    let items = vec!["a", "b", "a", "c", "a", "b"];
    let counts = count_occurrences(&items);
    println!("Counts: {:?}", counts);

    // Ejercicio 5: Valor por defecto si None
    println!();
    let some_val: Option<i32> = Some(42);
    let none_val: Option<i32> = None;
    println!("With value: {}", value_or_default(some_val));
    println!("Without value: {}", value_or_default(none_val));

    println!("\n✅ ¡Práctica completada!");
}

// ============================================
// EJERCICIO 1: Print with Debug
// ============================================
// Implementa una función que imprime cualquier valor
// que implemente Debug.
//
// Trait bound requerido: Debug
// Formato: "{:?}"

fn print_debug<T>(_value: &T) {
    // TODO: Añade el trait bound correcto y usa println! con {:?}
    todo!("Implementa print_debug")
}

// ============================================
// EJERCICIO 2: Compare and Show
// ============================================
// Implementa una función que compara dos valores
// y muestra cuál es mayor.
//
// Trait bounds requeridos: PartialOrd + Display
// Debe imprimir: "The larger is: X"

fn show_larger<T>(_a: T, _b: T) {
    // TODO: Añade los trait bounds correctos
    // Compara a y b, imprime el mayor
    todo!("Implementa show_larger")
}

// ============================================
// EJERCICIO 3: Clone if Larger
// ============================================
// Implementa una función que devuelve Some con el clone
// del primer valor si es mayor que el segundo,
// o None si no lo es.
//
// Trait bounds requeridos: PartialOrd + Clone

fn clone_if_larger<T>(_a: &T, _b: &T) -> Option<T> {
    // TODO: Añade los trait bounds correctos
    // Si a > b, devuelve Some(a.clone())
    // Si no, devuelve None
    todo!("Implementa clone_if_larger")
}

// ============================================
// EJERCICIO 4: Count Occurrences
// ============================================
// Implementa una función que cuenta cuántas veces
// aparece cada elemento en un slice.
//
// Trait bounds requeridos: Hash + Eq + Clone
// Devuelve: HashMap<T, usize>

fn count_occurrences<T>(_items: &[T]) -> HashMap<T, usize> {
    // TODO: Añade los trait bounds correctos
    // Itera sobre items y cuenta cada uno
    todo!("Implementa count_occurrences")
}

// ============================================
// EJERCICIO 5: Value or Default
// ============================================
// Implementa una función que devuelve el valor
// de un Option, o el valor por defecto del tipo si es None.
//
// Trait bound requerido: Default

fn value_or_default<T>(_option: Option<T>) -> T {
    // TODO: Añade el trait bound correcto
    // Usa match o unwrap_or_default()
    todo!("Implementa value_or_default")
}

// ============================================
// EJERCICIO BONUS: Multiple Bounds with Where
// ============================================
// Implementa una función que recibe dos tipos diferentes,
// los formatea y los combina en un String.
//
// T debe implementar: Display + Clone
// U debe implementar: Debug + Default

#[allow(dead_code)]
fn combine_formats<T, U>(_t: T, _u: U) -> String {
    // TODO: Usa cláusula where para los bounds
    // Devuelve format!("{} - {:?}", t, u)
    todo!("Implementa combine_formats")
}

// ============================================
// TESTS
// ============================================
#[cfg(test)]
mod tests {
    use super::*;

    // Nota: print_debug no tiene test de valor de retorno
    // Solo verificamos que compila con tipos Debug

    #[test]
    fn test_print_debug_compiles() {
        // Si compila, el bound está correcto
        print_debug(&42);
        print_debug(&"hola");
        print_debug(&vec![1, 2, 3]);
    }

    #[test]
    fn test_show_larger_compiles() {
        // Verificamos que compila con tipos PartialOrd + Display
        show_larger(10, 5);
        show_larger(1.5, 2.5);
        show_larger("a", "b");
    }

    #[test]
    fn test_clone_if_larger_some() {
        assert_eq!(clone_if_larger(&10, &5), Some(10));
        assert_eq!(clone_if_larger(&"z", &"a"), Some("z"));
    }

    #[test]
    fn test_clone_if_larger_none() {
        assert_eq!(clone_if_larger(&5, &10), None);
        assert_eq!(clone_if_larger(&5, &5), None); // No es mayor, es igual
    }

    #[test]
    fn test_count_occurrences() {
        let items = vec!["a", "b", "a", "c", "a"];
        let counts = count_occurrences(&items);

        assert_eq!(counts.get("a"), Some(&3));
        assert_eq!(counts.get("b"), Some(&1));
        assert_eq!(counts.get("c"), Some(&1));
    }

    #[test]
    fn test_count_occurrences_numbers() {
        let items = vec![1, 2, 2, 3, 3, 3];
        let counts = count_occurrences(&items);

        assert_eq!(counts.get(&1), Some(&1));
        assert_eq!(counts.get(&2), Some(&2));
        assert_eq!(counts.get(&3), Some(&3));
    }

    #[test]
    fn test_value_or_default_some() {
        assert_eq!(value_or_default(Some(42)), 42);
        assert_eq!(value_or_default(Some(String::from("hola"))), "hola");
    }

    #[test]
    fn test_value_or_default_none() {
        let none_i32: Option<i32> = None;
        let none_string: Option<String> = None;

        assert_eq!(value_or_default(none_i32), 0); // Default de i32
        assert_eq!(value_or_default(none_string), ""); // Default de String
    }

    #[test]
    fn test_combine_formats() {
        let result = combine_formats("texto", 42);
        assert_eq!(result, "texto - 42");
    }

    #[test]
    fn test_combine_formats_with_default() {
        let result = combine_formats(100, String::from("debug"));
        assert_eq!(result, "100 - \"debug\"");
    }
}
