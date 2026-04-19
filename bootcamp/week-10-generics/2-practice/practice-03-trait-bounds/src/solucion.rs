// ============================================
// SOLUCIONES - Práctica 03: Trait Bounds
// ============================================
// ⚠️  NO MIRAR HASTA INTENTAR RESOLVER LOS EJERCICIOS

#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

// Ejercicio 1: Print with Debug
fn print_debug<T: Debug>(value: &T) {
    println!("{:?}", value);
}

// Ejercicio 2: Compare and Show
fn show_larger<T: PartialOrd + Display>(a: T, b: T) {
    if a > b {
        println!("The larger is: {}", a);
    } else {
        println!("The larger is: {}", b);
    }
}

// Ejercicio 3: Clone if Larger
fn clone_if_larger<T: PartialOrd + Clone>(a: &T, b: &T) -> Option<T> {
    if a > b {
        Some(a.clone())
    } else {
        None
    }
}

// Ejercicio 4: Count Occurrences
fn count_occurrences<T: Hash + Eq + Clone>(items: &[T]) -> HashMap<T, usize> {
    let mut counts = HashMap::new();
    for item in items {
        *counts.entry(item.clone()).or_insert(0) += 1;
    }
    counts
}

// Ejercicio 5: Value or Default
fn value_or_default<T: Default>(option: Option<T>) -> T {
    option.unwrap_or_default()
    // Alternativa con match:
    // match option {
    //     Some(value) => value,
    //     None => T::default(),
    // }
}

// Ejercicio Bonus: Multiple Bounds with Where
fn combine_formats<T, U>(t: T, u: U) -> String
where
    T: Display + Clone,
    U: Debug + Default,
{
    format!("{} - {:?}", t, u)
}
