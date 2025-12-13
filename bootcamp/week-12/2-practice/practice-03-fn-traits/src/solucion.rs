//! Soluciones - Práctica 03: Fn Traits

fn main() {
    println!("=== Soluciones: Fn Traits ===\n");

    // Ejercicio 1
    let x = 10;
    let add = |y| x + y;
    let result = aplicar_fn(5, add);
    println!("Ejercicio 1 - Fn: {}", result);

    // Ejercicio 2
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        counter
    };
    let results = aplicar_fn_mut_veces(&mut increment, 3);
    println!("Ejercicio 2 - FnMut: {:?}", results);

    // Ejercicio 3
    let data = vec![1, 2, 3, 4, 5];
    let consume = move || data.iter().sum();
    let total = aplicar_fn_once(consume);
    println!("Ejercicio 3 - FnOnce: {}", total);

    // Ejercicio 4
    let nums = vec![1, 2, 3, 4, 5];
    let evens = filtrar(&nums, |x| x % 2 == 0);
    println!("Ejercicio 4 - Filtrar pares: {:?}", evens);

    // Ejercicio 5
    let mult = crear_multiplicador_boxed(5);
    println!("Ejercicio 5 - Box<dyn Fn>: 3*5={}", mult(3));

    println!("\n✅ Todas las soluciones funcionan!");
}

// SOLUCIÓN Ejercicio 1
fn aplicar_fn<F>(valor: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(valor)
}

// SOLUCIÓN Ejercicio 2
fn aplicar_fn_mut_veces<F>(f: &mut F, veces: usize) -> Vec<i32>
where
    F: FnMut() -> i32,
{
    let mut resultados = Vec::with_capacity(veces);
    for _ in 0..veces {
        resultados.push(f());
    }
    resultados
}

// SOLUCIÓN Ejercicio 3
fn aplicar_fn_once<F>(f: F) -> i32
where
    F: FnOnce() -> i32,
{
    f()
}

// SOLUCIÓN Ejercicio 4
fn filtrar<F>(lista: &[i32], predicado: F) -> Vec<i32>
where
    F: Fn(&i32) -> bool,
{
    lista.iter().filter(|x| predicado(x)).copied().collect()
}

// SOLUCIÓN Ejercicio 5
fn crear_multiplicador_boxed(factor: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x * factor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_solutions() {
        // Test Fn
        assert_eq!(aplicar_fn(5, |x| x * 2), 10);

        // Test FnMut
        let mut n = 0;
        let mut generador = || { n += 1; n };
        assert_eq!(aplicar_fn_mut_veces(&mut generador, 3), vec![1, 2, 3]);

        // Test FnOnce
        let v = vec![1, 2, 3];
        assert_eq!(aplicar_fn_once(move || v.iter().sum()), 6);

        // Test filtrar
        assert_eq!(filtrar(&[1, 2, 3, 4], |x| x % 2 == 0), vec![2, 4]);

        // Test Box<dyn Fn>
        let m = crear_multiplicador_boxed(3);
        assert_eq!(m(4), 12);
    }
}
