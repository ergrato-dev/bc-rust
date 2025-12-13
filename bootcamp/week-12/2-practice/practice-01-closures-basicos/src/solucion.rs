//! Soluciones - Práctica 01: Closures Básicos

fn main() {
    println!("=== Soluciones: Closures Básicos ===\n");

    // Ejercicio 1
    let double = crear_duplicador();
    println!("Duplicar 5: {}", double(5));

    // Ejercicio 2
    let result = aplicar_operacion(10, 20, crear_sumador());
    println!("Suma: {}", result);

    // Ejercicio 3
    let numbers = vec![1, 2, 3, 4, 5];
    let processed = procesar_lista(&numbers, crear_transformador());
    println!("Procesados: {:?}", processed);

    // Ejercicio 4
    let filtered: Vec<i32> = numbers.iter()
        .filter(crear_filtro_pares())
        .copied()
        .collect();
    println!("Pares: {:?}", filtered);

    // Ejercicio 5
    let multiplier = crear_multiplicador(3);
    println!("7 * 3 = {}", multiplier(7));;

    println!("\n✅ Todas las soluciones funcionan!");
}

// SOLUCIÓN Ejercicio 1
fn crear_duplicador() -> impl Fn(i32) -> i32 {
    |x| x * 2
}

// SOLUCIÓN Ejercicio 2
fn crear_sumador() -> impl Fn(i32, i32) -> i32 {
    |a, b| a + b
}

fn aplicar_operacion<F>(a: i32, b: i32, op: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    op(a, b)
}

// SOLUCIÓN Ejercicio 3
fn crear_transformador() -> impl Fn(i32) -> i32 {
    |x| {
        if x % 2 == 0 {
            x * 2
        } else {
            x + 1
        }
    }
}

fn procesar_lista<F>(lista: &[i32], f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    lista.iter().map(|&x| f(x)).collect()
}

// SOLUCIÓN Ejercicio 4
fn crear_filtro_pares() -> impl Fn(&&i32) -> bool {
    |x| **x % 2 == 0
}

// SOLUCIÓN Ejercicio 5
fn crear_multiplicador(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_solutions() {
        // Test duplicador
        let dup = crear_duplicador();
        assert_eq!(dup(5), 10);

        // Test sumador
        let suma = crear_sumador();
        assert_eq!(suma(2, 3), 5);

        // Test transformador
        let trans = crear_transformador();
        assert_eq!(trans(2), 4);
        assert_eq!(trans(3), 4);

        // Test filtro
        let filtro = crear_filtro_pares();
        assert!(filtro(&&2));
        assert!(!filtro(&&3));

        // Test multiplicador
        let por_3 = crear_multiplicador(3);
        assert_eq!(por_3(4), 12);
    }
}
