// ============================================
// SOLUCIONES - Práctica 03: Trait Bounds
// ============================================
// ⚠️  NO MIRAR HASTA INTENTAR RESOLVER LOS EJERCICIOS

#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

// Ejercicio 1: Imprimir con Debug
fn imprimir_debug<T: Debug>(valor: &T) {
    println!("{:?}", valor);
}

// Ejercicio 2: Comparar y Mostrar
fn mostrar_mayor<T: PartialOrd + Display>(a: T, b: T) {
    if a > b {
        println!("El mayor es: {}", a);
    } else {
        println!("El mayor es: {}", b);
    }
}

// Ejercicio 3: Clonar si es Mayor
fn clonar_si_mayor<T: PartialOrd + Clone>(a: &T, b: &T) -> Option<T> {
    if a > b {
        Some(a.clone())
    } else {
        None
    }
}

// Ejercicio 4: Contar Ocurrencias
fn contar_ocurrencias<T: Hash + Eq + Clone>(items: &[T]) -> HashMap<T, usize> {
    let mut conteo = HashMap::new();
    for item in items {
        *conteo.entry(item.clone()).or_insert(0) += 1;
    }
    conteo
}

// Ejercicio 5: Valor o Default
fn valor_o_default<T: Default>(opcion: Option<T>) -> T {
    opcion.unwrap_or_default()
    // Alternativa con match:
    // match opcion {
    //     Some(valor) => valor,
    //     None => T::default(),
    // }
}

// Ejercicio Bonus: Múltiples Bounds con Where
fn combinar_formatos<T, U>(t: T, u: U) -> String
where
    T: Display + Clone,
    U: Debug + Default,
{
    format!("{} - {:?}", t, u)
}
