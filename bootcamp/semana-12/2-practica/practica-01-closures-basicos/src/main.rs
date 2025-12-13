//! Práctica 01: Closures Básicos
//!
//! En esta práctica aprenderás a:
//! - Crear closures con diferentes sintaxis
//! - Pasar closures como argumentos
//! - Retornar closures de funciones

fn main() {
    println!("=== Práctica 01: Closures Básicos ===\n");

    // Ejercicio 1: Crear closures simples
    let duplicar = crear_duplicador();
    println!("Ejercicio 1 - Duplicar 5: {}", duplicar(5));

    // Ejercicio 2: Closure que suma
    let resultado = aplicar_operacion(10, 20, crear_sumador());
    println!("Ejercicio 2 - Suma: {}", resultado);

    // Ejercicio 3: Closure con múltiples operaciones
    let numeros = vec![1, 2, 3, 4, 5];
    let procesados = procesar_lista(&numeros, crear_transformador());
    println!("Ejercicio 3 - Procesados: {:?}", procesados);

    // Ejercicio 4: Closure como filtro
    let filtrados: Vec<i32> = numeros.iter()
        .filter(crear_filtro_pares())
        .copied()
        .collect();
    println!("Ejercicio 4 - Pares: {:?}", filtrados);

    // Ejercicio 5: Closure que retorna closure
    let multiplicador = crear_multiplicador(3);
    println!("Ejercicio 5 - 7 * 3 = {}", multiplicador(7));

    println!("\n✅ Todos los ejercicios completados!");
}

// ============================================================
// EJERCICIO 1: Crear un closure que duplica
// ============================================================
// Implementa un closure que multiplique por 2
//
// TODO: Retorna un closure que duplique el valor
// ============================================================

fn crear_duplicador() -> impl Fn(i32) -> i32 {
    // TODO: Retorna un closure que multiplique x por 2
    // ↓ Cambia esto por: |x| x * 2
    |x| x
}

// ============================================================
// EJERCICIO 2: Closure que suma dos números
// ============================================================
// Crea un closure que sume sus dos argumentos
//
// TODO: Retorna un closure que sume a + b
// ============================================================

fn crear_sumador() -> impl Fn(i32, i32) -> i32 {
    // TODO: Retorna un closure que sume dos números
    // ↓ Cambia esto por: |a, b| a + b
    |a, _b| a
}

/// Aplica una operación a dos números
fn aplicar_operacion<F>(a: i32, b: i32, op: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    op(a, b)
}

// ============================================================
// EJERCICIO 3: Transformador de lista
// ============================================================
// Crea un closure que transforme cada elemento:
// - Si es par, multiplica por 2
// - Si es impar, suma 1
//
// TODO: Implementa la lógica de transformación
// ============================================================

fn crear_transformador() -> impl Fn(i32) -> i32 {
    // TODO: Retorna closure que transforma según la regla
    // ↓ Cambia esto por: |x| if x % 2 == 0 { x * 2 } else { x + 1 }
    |x| x
}

/// Procesa una lista aplicando una función a cada elemento
fn procesar_lista<F>(lista: &[i32], f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    lista.iter().map(|&x| f(x)).collect()
}

// ============================================================
// EJERCICIO 4: Closure como predicado de filtro
// ============================================================
// Crea un closure que retorne true si el número es par
//
// TODO: Retorna un closure que filtre pares
// ============================================================

fn crear_filtro_pares() -> impl Fn(&&i32) -> bool {
    // TODO: Retorna closure que retorne true si es par
    // ↓ Cambia esto por: |x| *x % 2 == 0
    |_x| true
}

// ============================================================
// EJERCICIO 5: Factory de closures
// ============================================================
// Crea una función que retorne un closure multiplicador
//
// El closure captura el factor y lo usa para multiplicar
// TODO: Usa 'move' para capturar el factor
// ============================================================

fn crear_multiplicador(factor: i32) -> impl Fn(i32) -> i32 {
    // TODO: Retorna closure que multiplique por 'factor'
    // Pista: Necesitas usar 'move'
    // ↓ Cambia esto por: move |x| x * factor
    let _ = factor;
    move |x| x
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicador() {
        let dup = crear_duplicador();
        assert_eq!(dup(5), 10);
        assert_eq!(dup(0), 0);
        assert_eq!(dup(-3), -6);
    }

    #[test]
    fn test_sumador() {
        let suma = crear_sumador();
        assert_eq!(suma(2, 3), 5);
        assert_eq!(suma(-1, 1), 0);
    }

    #[test]
    fn test_transformador() {
        let trans = crear_transformador();
        assert_eq!(trans(2), 4);  // par * 2
        assert_eq!(trans(3), 4);  // impar + 1
        assert_eq!(trans(4), 8);  // par * 2
        assert_eq!(trans(5), 6);  // impar + 1
    }

    #[test]
    fn test_filtro_pares() {
        let filtro = crear_filtro_pares();
        assert!(filtro(&&2));
        assert!(!filtro(&&3));
        assert!(filtro(&&0));
    }

    #[test]
    fn test_multiplicador() {
        let por_3 = crear_multiplicador(3);
        let por_5 = crear_multiplicador(5);
        
        assert_eq!(por_3(4), 12);
        assert_eq!(por_5(4), 20);
    }
}
