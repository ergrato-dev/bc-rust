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
    let numeros = vec![1, 2, 3];
    let punto = (10, 20);
    imprimir_debug(&numeros);
    imprimir_debug(&punto);

    // Ejercicio 2: Comparar y mostrar
    println!();
    mostrar_mayor(10, 5);
    mostrar_mayor("zebra", "apple");

    // Ejercicio 3: Clonar si es mayor
    println!();
    let resultado = clonar_si_mayor(&100, &50);
    println!("Clonado si mayor: {:?}", resultado);

    // Ejercicio 4: Contar ocurrencias
    println!();
    let items = vec!["a", "b", "a", "c", "a", "b"];
    let conteo = contar_ocurrencias(&items);
    println!("Conteo: {:?}", conteo);

    // Ejercicio 5: Valor por defecto si None
    println!();
    let some_val: Option<i32> = Some(42);
    let none_val: Option<i32> = None;
    println!("Con valor: {}", valor_o_default(some_val));
    println!("Sin valor: {}", valor_o_default(none_val));

    println!("\n✅ ¡Práctica completada!");
}

// ============================================
// EJERCICIO 1: Imprimir con Debug
// ============================================
// Implementa una función que imprime cualquier valor
// que implemente Debug.
//
// Trait bound requerido: Debug
// Formato: "{:?}"

fn imprimir_debug<T>(_valor: &T) {
    // TODO: Añade el trait bound correcto y usa println! con {:?}
    todo!("Implementa imprimir_debug")
}

// ============================================
// EJERCICIO 2: Comparar y Mostrar
// ============================================
// Implementa una función que compara dos valores
// y muestra cuál es mayor.
//
// Trait bounds requeridos: PartialOrd + Display
// Debe imprimir: "El mayor es: X"

fn mostrar_mayor<T>(_a: T, _b: T) {
    // TODO: Añade los trait bounds correctos
    // Compara a y b, imprime el mayor
    todo!("Implementa mostrar_mayor")
}

// ============================================
// EJERCICIO 3: Clonar si es Mayor
// ============================================
// Implementa una función que devuelve Some con el clone
// del primer valor si es mayor que el segundo,
// o None si no lo es.
//
// Trait bounds requeridos: PartialOrd + Clone

fn clonar_si_mayor<T>(_a: &T, _b: &T) -> Option<T> {
    // TODO: Añade los trait bounds correctos
    // Si a > b, devuelve Some(a.clone())
    // Si no, devuelve None
    todo!("Implementa clonar_si_mayor")
}

// ============================================
// EJERCICIO 4: Contar Ocurrencias
// ============================================
// Implementa una función que cuenta cuántas veces
// aparece cada elemento en un slice.
//
// Trait bounds requeridos: Hash + Eq + Clone
// Devuelve: HashMap<T, usize>

fn contar_ocurrencias<T>(_items: &[T]) -> HashMap<T, usize> {
    // TODO: Añade los trait bounds correctos
    // Itera sobre items y cuenta cada uno
    todo!("Implementa contar_ocurrencias")
}

// ============================================
// EJERCICIO 5: Valor o Default
// ============================================
// Implementa una función que devuelve el valor
// de un Option, o el valor por defecto del tipo si es None.
//
// Trait bound requerido: Default

fn valor_o_default<T>(_opcion: Option<T>) -> T {
    // TODO: Añade el trait bound correcto
    // Usa match o unwrap_or_default()
    todo!("Implementa valor_o_default")
}

// ============================================
// EJERCICIO BONUS: Múltiples Bounds con Where
// ============================================
// Implementa una función que recibe dos tipos diferentes,
// los formatea y los combina en un String.
//
// T debe implementar: Display + Clone
// U debe implementar: Debug + Default

#[allow(dead_code)]
fn combinar_formatos<T, U>(_t: T, _u: U) -> String {
    // TODO: Usa cláusula where para los bounds
    // Devuelve format!("{} - {:?}", t, u)
    todo!("Implementa combinar_formatos")
}

// ============================================
// TESTS
// ============================================
#[cfg(test)]
mod tests {
    use super::*;

    // Nota: imprimir_debug no tiene test de valor de retorno
    // Solo verificamos que compila con tipos Debug

    #[test]
    fn test_imprimir_debug_compila() {
        // Si compila, el bound está correcto
        imprimir_debug(&42);
        imprimir_debug(&"hola");
        imprimir_debug(&vec![1, 2, 3]);
    }

    #[test]
    fn test_mostrar_mayor_compila() {
        // Verificamos que compila con tipos PartialOrd + Display
        mostrar_mayor(10, 5);
        mostrar_mayor(1.5, 2.5);
        mostrar_mayor("a", "b");
    }

    #[test]
    fn test_clonar_si_mayor_some() {
        assert_eq!(clonar_si_mayor(&10, &5), Some(10));
        assert_eq!(clonar_si_mayor(&"z", &"a"), Some("z"));
    }

    #[test]
    fn test_clonar_si_mayor_none() {
        assert_eq!(clonar_si_mayor(&5, &10), None);
        assert_eq!(clonar_si_mayor(&5, &5), None); // No es mayor, es igual
    }

    #[test]
    fn test_contar_ocurrencias() {
        let items = vec!["a", "b", "a", "c", "a"];
        let conteo = contar_ocurrencias(&items);

        assert_eq!(conteo.get("a"), Some(&3));
        assert_eq!(conteo.get("b"), Some(&1));
        assert_eq!(conteo.get("c"), Some(&1));
    }

    #[test]
    fn test_contar_ocurrencias_numeros() {
        let items = vec![1, 2, 2, 3, 3, 3];
        let conteo = contar_ocurrencias(&items);

        assert_eq!(conteo.get(&1), Some(&1));
        assert_eq!(conteo.get(&2), Some(&2));
        assert_eq!(conteo.get(&3), Some(&3));
    }

    #[test]
    fn test_valor_o_default_some() {
        assert_eq!(valor_o_default(Some(42)), 42);
        assert_eq!(valor_o_default(Some(String::from("hola"))), "hola");
    }

    #[test]
    fn test_valor_o_default_none() {
        let none_i32: Option<i32> = None;
        let none_string: Option<String> = None;

        assert_eq!(valor_o_default(none_i32), 0); // Default de i32
        assert_eq!(valor_o_default(none_string), ""); // Default de String
    }

    #[test]
    fn test_combinar_formatos() {
        let resultado = combinar_formatos("texto", 42);
        assert_eq!(resultado, "texto - 42");
    }

    #[test]
    fn test_combinar_formatos_con_default() {
        let resultado = combinar_formatos(100, String::from("debug"));
        assert_eq!(resultado, "100 - \"debug\"");
    }
}
