//! Práctica 03: Traits Fn, FnMut, FnOnce
//!
//! En esta práctica aprenderás a:
//! - Distinguir entre Fn, FnMut y FnOnce
//! - Usar bounds de traits de función
//! - Elegir el trait correcto para cada caso

fn main() {
    println!("=== Práctica 03: Fn Traits ===\n");

    // Ejercicio 1: Usar Fn
    println!("Ejercicio 1: Fn");
    let x = 10;
    let add = |y| x + y;
    let result = aplicar_fn(5, add);
    println!("  Resultado: {}\n", result);

    // Ejercicio 2: Usar FnMut
    println!("Ejercicio 2: FnMut");
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        counter
    };
    let results = aplicar_fn_mut_veces(&mut increment, 3);
    println!("  Resultados: {:?}\n", results);

    // Ejercicio 3: Usar FnOnce
    println!("Ejercicio 3: FnOnce");
    let data = vec![1, 2, 3, 4, 5];
    let consume = move || {
        let sum: i32 = data.iter().sum();
        sum
    };
    let total = aplicar_fn_once(consume);
    println!("  Total: {}\n", total);

    // Ejercicio 4: Función que acepta cualquier closure
    println!("Ejercicio 4: Genérico con Fn");
    let nums = vec![1, 2, 3, 4, 5];
    let evens = filtrar(&nums, |x| x % 2 == 0);
    let greater_than_3 = filtrar(&nums, |x| *x > 3);
    println!("  Pares: {:?}", evens);
    println!("  Mayores a 3: {:?}\n", greater_than_3);

    // Ejercicio 5: Retornar closure con Box<dyn Fn>
    println!("Ejercicio 5: Box<dyn Fn>");
    let multiplier = crear_multiplicador_boxed(5);
    println!("  3 * 5 = {}", multiplier(3));
    println!("  7 * 5 = {}", multiplier(7));

    println!("\n✅ Todos los ejercicios completados!");
}

// ============================================================
// EJERCICIO 1: Función que acepta Fn
// ============================================================
// Implementa una función que tome un valor y un closure Fn.
// El closure debe poder llamarse múltiples veces.
//
// TODO: Implementa aplicar_fn
// ============================================================

fn aplicar_fn<F>(valor: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    // TODO: Llama a f con el valor y retorna el resultado
    todo!("Implementa aplicar_fn")
}

// ============================================================
// EJERCICIO 2: Función que acepta FnMut
// ============================================================
// Implementa una función que ejecute un closure FnMut N veces.
// Colecciona los resultados en un Vec.
//
// TODO: Implementa aplicar_fn_mut_veces
// ============================================================

fn aplicar_fn_mut_veces<F>(f: &mut F, veces: usize) -> Vec<i32>
where
    F: FnMut() -> i32,
{
    // TODO: Llama a f 'veces' veces y colecciona resultados
    todo!("Implementa aplicar_fn_mut_veces")
}

// ============================================================
// EJERCICIO 3: Función que acepta FnOnce
// ============================================================
// Implementa una función que ejecute un closure FnOnce una vez.
// El closure consume sus capturas.
//
// TODO: Implementa aplicar_fn_once
// ============================================================

fn aplicar_fn_once<F>(f: F) -> i32
where
    F: FnOnce() -> i32,
{
    // TODO: Llama a f una vez y retorna el resultado
    todo!("Implementa aplicar_fn_once")
}

// ============================================================
// EJERCICIO 4: Filtrar con closure genérico
// ============================================================
// Implementa una función filtrar que use un predicado Fn.
// Debe retornar los elementos que cumplan el predicado.
//
// TODO: Implementa filtrar
// ============================================================

fn filtrar<F>(lista: &[i32], predicado: F) -> Vec<i32>
where
    F: Fn(&i32) -> bool,
{
    // TODO: Filtra los elementos usando el predicado
    todo!("Implementa filtrar")
}

// ============================================================
// EJERCICIO 5: Retornar closure con Box<dyn Fn>
// ============================================================
// Crea una función factory que retorne un closure boxeado.
// Útil cuando necesitas retornar diferentes closures.
//
// TODO: Implementa crear_multiplicador_boxed
// ============================================================

fn crear_multiplicador_boxed(factor: i32) -> Box<dyn Fn(i32) -> i32> {
    // TODO: Retorna un Box con un closure que multiplique por factor
    todo!("Implementa crear_multiplicador_boxed")
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aplicar_fn() {
        let doble = |x| x * 2;
        assert_eq!(aplicar_fn(5, doble), 10);
        assert_eq!(aplicar_fn(5, doble), 10); // Puede llamarse múltiples veces
    }

    #[test]
    fn test_aplicar_fn_mut_veces() {
        let mut n = 0;
        let mut generador = || {
            n += 10;
            n
        };
        let resultados = aplicar_fn_mut_veces(&mut generador, 3);
        assert_eq!(resultados, vec![10, 20, 30]);
    }

    #[test]
    fn test_aplicar_fn_once() {
        let v = vec![1, 2, 3];
        let consumir = move || v.into_iter().sum();
        assert_eq!(aplicar_fn_once(consumir), 6);
    }

    #[test]
    fn test_filtrar() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(filtrar(&nums, |x| x % 2 == 0), vec![2, 4, 6]);
        assert_eq!(filtrar(&nums, |x| *x > 4), vec![5, 6]);
    }

    #[test]
    fn test_multiplicador_boxed() {
        let por_3 = crear_multiplicador_boxed(3);
        let por_7 = crear_multiplicador_boxed(7);
        
        assert_eq!(por_3(4), 12);
        assert_eq!(por_7(4), 28);
    }
}
