// ============================================
// PRÁCTICA 01: Funciones Genéricas
// ============================================
// Objetivo: Dominar la sintaxis y uso de funciones genéricas
//
// Instrucciones:
// 1. Implementa cada función marcada con TODO
// 2. Ejecuta `cargo test` para verificar tu implementación
// 3. Todos los tests deben pasar

fn main() {
    println!("=== Práctica 01: Funciones Genéricas ===\n");

    // Ejercicio 1: Función identidad
    let number = identity(42);
    let text = identity("Hola Rust");
    println!("Identity número: {number}");
    println!("Identity texto: {text}");

    // Ejercicio 2: Intercambiar valores
    let (a, b) = swap(10, 20);
    println!("\nSwap: ({a}, {b})");

    // Ejercicio 3: Mayor de dos
    let larger_num = larger(15, 8);
    let larger_char = larger('z', 'a');
    println!("\nLarger número: {larger_num}");
    println!("Larger char: {larger_char}");

    // Ejercicio 4: Primer elemento
    let numbers = vec![100, 200, 300];
    let letters = vec!['x', 'y', 'z'];
    println!("\nFirst número: {:?}", first(&numbers));
    println!("First letra: {:?}", first(&letters));

    // Ejercicio 5: Contar elementos
    let elements = vec![1, 2, 3, 4, 5];
    println!("\nCount elementos: {}", count(&elements));

    println!("\n✅ ¡Práctica completada!");
}

// ============================================
// EJERCICIO 1: Función Identity
// ============================================
// Implementa una función genérica que recibe un valor
// de cualquier tipo y lo devuelve sin modificar.
//
// Pista: No necesitas ningún trait bound

fn identity<T>(value: T) -> T {
    // TODO: Implementa esta función
    // Debe devolver el mismo valor que recibe
    todo!("Implementa identity")
}

// ============================================
// EJERCICIO 2: Swap Values
// ============================================
// Implementa una función genérica que recibe dos valores
// del mismo tipo y los devuelve en orden inverso.
//
// Ejemplo: swap(1, 2) -> (2, 1)

fn swap<T>(a: T, b: T) -> (T, T) {
    // TODO: Implementa esta función
    // Debe devolver (b, a)
    todo!("Implementa swap")
}

// ============================================
// EJERCICIO 3: Larger of Two
// ============================================
// Implementa una función genérica que recibe dos valores
// y devuelve el mayor de ellos.
//
// Pista: Necesitas el trait bound PartialOrd para comparar

fn larger<T: PartialOrd>(a: T, b: T) -> T {
    // TODO: Implementa esta función
    // Usa el operador > para comparar
    todo!("Implementa larger")
}

// ============================================
// EJERCICIO 4: First Element
// ============================================
// Implementa una función genérica que recibe una referencia
// a un slice y devuelve Option con el primer elemento clonado.
//
// Pista: Necesitas Clone para poder clonar el elemento

fn first<T: Clone>(elements: &[T]) -> Option<T> {
    // TODO: Implementa esta función
    // Si el slice está vacío, devuelve None
    // Si tiene elementos, devuelve Some con el primero clonado
    todo!("Implementa first")
}

// ============================================
// EJERCICIO 5: Count Elements
// ============================================
// Implementa una función genérica que recibe una referencia
// a un slice de cualquier tipo y devuelve la cantidad de elementos.
//
// Pista: No necesitas ningún trait bound

fn count<T>(elements: &[T]) -> usize {
    // TODO: Implementa esta función
    // Usa el método len() del slice
    todo!("Implementa count")
}

// ============================================
// TESTS
// ============================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_numbers() {
        assert_eq!(identity(42), 42);
        assert_eq!(identity(3.14), 3.14);
        assert_eq!(identity(-100), -100);
    }

    #[test]
    fn test_identity_strings() {
        assert_eq!(identity("hola"), "hola");
        assert_eq!(identity(String::from("mundo")), String::from("mundo"));
    }

    #[test]
    fn test_swap_numbers() {
        assert_eq!(swap(1, 2), (2, 1));
        assert_eq!(swap(100, 0), (0, 100));
    }

    #[test]
    fn test_swap_strings() {
        assert_eq!(
            swap(String::from("a"), String::from("b")),
            (String::from("b"), String::from("a"))
        );
    }

    #[test]
    fn test_larger_numbers() {
        assert_eq!(larger(10, 20), 20);
        assert_eq!(larger(100, 50), 100);
        assert_eq!(larger(5, 5), 5);
    }

    #[test]
    fn test_larger_chars() {
        assert_eq!(larger('a', 'z'), 'z');
        assert_eq!(larger('Z', 'A'), 'Z');
    }

    #[test]
    fn test_larger_strings() {
        assert_eq!(larger("apple", "banana"), "banana");
        assert_eq!(larger("zebra", "ant"), "zebra");
    }

    #[test]
    fn test_first_with_elements() {
        assert_eq!(first(&[1, 2, 3]), Some(1));
        assert_eq!(first(&['a', 'b', 'c']), Some('a'));
        assert_eq!(first(&[String::from("hola")]), Some(String::from("hola")));
    }

    #[test]
    fn test_first_empty() {
        let empty: &[i32] = &[];
        assert_eq!(first(empty), None);
    }

    #[test]
    fn test_count_elements() {
        assert_eq!(count(&[1, 2, 3, 4, 5]), 5);
        assert_eq!(count(&['a', 'b']), 2);
        assert_eq!(count(&[String::from("uno")]), 1);
    }

    #[test]
    fn test_count_empty() {
        let empty: &[i32] = &[];
        assert_eq!(count(empty), 0);
    }
}
