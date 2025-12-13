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
    let numero = identidad(42);
    let texto = identidad("Hola Rust");
    println!("Identidad número: {numero}");
    println!("Identidad texto: {texto}");

    // Ejercicio 2: Intercambiar valores
    let (a, b) = intercambiar(10, 20);
    println!("\nIntercambio: ({a}, {b})");

    // Ejercicio 3: Mayor de dos
    let mayor_num = mayor(15, 8);
    let mayor_char = mayor('z', 'a');
    println!("\nMayor número: {mayor_num}");
    println!("Mayor char: {mayor_char}");

    // Ejercicio 4: Primer elemento
    let numeros = vec![100, 200, 300];
    let letras = vec!['x', 'y', 'z'];
    println!("\nPrimer número: {:?}", primero(&numeros));
    println!("Primera letra: {:?}", primero(&letras));

    // Ejercicio 5: Contar elementos
    let elementos = vec![1, 2, 3, 4, 5];
    println!("\nCantidad de elementos: {}", contar(&elementos));

    println!("\n✅ ¡Práctica completada!");
}

// ============================================
// EJERCICIO 1: Función Identidad
// ============================================
// Implementa una función genérica que recibe un valor
// de cualquier tipo y lo devuelve sin modificar.
//
// Pista: No necesitas ningún trait bound

fn identidad<T>(valor: T) -> T {
    // TODO: Implementa esta función
    // Debe devolver el mismo valor que recibe
    todo!("Implementa identidad")
}

// ============================================
// EJERCICIO 2: Intercambiar Valores
// ============================================
// Implementa una función genérica que recibe dos valores
// del mismo tipo y los devuelve en orden inverso.
//
// Ejemplo: intercambiar(1, 2) -> (2, 1)

fn intercambiar<T>(a: T, b: T) -> (T, T) {
    // TODO: Implementa esta función
    // Debe devolver (b, a)
    todo!("Implementa intercambiar")
}

// ============================================
// EJERCICIO 3: Mayor de Dos
// ============================================
// Implementa una función genérica que recibe dos valores
// y devuelve el mayor de ellos.
//
// Pista: Necesitas el trait bound PartialOrd para comparar

fn mayor<T: PartialOrd>(a: T, b: T) -> T {
    // TODO: Implementa esta función
    // Usa el operador > para comparar
    todo!("Implementa mayor")
}

// ============================================
// EJERCICIO 4: Primer Elemento
// ============================================
// Implementa una función genérica que recibe una referencia
// a un slice y devuelve Option con el primer elemento clonado.
//
// Pista: Necesitas Clone para poder clonar el elemento

fn primero<T: Clone>(elementos: &[T]) -> Option<T> {
    // TODO: Implementa esta función
    // Si el slice está vacío, devuelve None
    // Si tiene elementos, devuelve Some con el primero clonado
    todo!("Implementa primero")
}

// ============================================
// EJERCICIO 5: Contar Elementos
// ============================================
// Implementa una función genérica que recibe una referencia
// a un slice de cualquier tipo y devuelve la cantidad de elementos.
//
// Pista: No necesitas ningún trait bound

fn contar<T>(elementos: &[T]) -> usize {
    // TODO: Implementa esta función
    // Usa el método len() del slice
    todo!("Implementa contar")
}

// ============================================
// TESTS
// ============================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identidad_numeros() {
        assert_eq!(identidad(42), 42);
        assert_eq!(identidad(3.14), 3.14);
        assert_eq!(identidad(-100), -100);
    }

    #[test]
    fn test_identidad_strings() {
        assert_eq!(identidad("hola"), "hola");
        assert_eq!(identidad(String::from("mundo")), String::from("mundo"));
    }

    #[test]
    fn test_intercambiar_numeros() {
        assert_eq!(intercambiar(1, 2), (2, 1));
        assert_eq!(intercambiar(100, 0), (0, 100));
    }

    #[test]
    fn test_intercambiar_strings() {
        assert_eq!(
            intercambiar(String::from("a"), String::from("b")),
            (String::from("b"), String::from("a"))
        );
    }

    #[test]
    fn test_mayor_numeros() {
        assert_eq!(mayor(10, 20), 20);
        assert_eq!(mayor(100, 50), 100);
        assert_eq!(mayor(5, 5), 5);
    }

    #[test]
    fn test_mayor_chars() {
        assert_eq!(mayor('a', 'z'), 'z');
        assert_eq!(mayor('Z', 'A'), 'Z');
    }

    #[test]
    fn test_mayor_strings() {
        assert_eq!(mayor("apple", "banana"), "banana");
        assert_eq!(mayor("zebra", "ant"), "zebra");
    }

    #[test]
    fn test_primero_con_elementos() {
        assert_eq!(primero(&[1, 2, 3]), Some(1));
        assert_eq!(primero(&['a', 'b', 'c']), Some('a'));
        assert_eq!(primero(&[String::from("hola")]), Some(String::from("hola")));
    }

    #[test]
    fn test_primero_vacio() {
        let vacio: &[i32] = &[];
        assert_eq!(primero(vacio), None);
    }

    #[test]
    fn test_contar_elementos() {
        assert_eq!(contar(&[1, 2, 3, 4, 5]), 5);
        assert_eq!(contar(&['a', 'b']), 2);
        assert_eq!(contar(&[String::from("uno")]), 1);
    }

    #[test]
    fn test_contar_vacio() {
        let vacio: &[i32] = &[];
        assert_eq!(contar(vacio), 0);
    }
}
