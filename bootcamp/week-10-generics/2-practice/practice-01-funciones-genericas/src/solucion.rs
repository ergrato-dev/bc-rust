// ============================================
// SOLUCIONES - Práctica 01: Funciones Genéricas
// ============================================
// ⚠️  NO MIRAR HASTA INTENTAR RESOLVER LOS EJERCICIOS

#![allow(dead_code)]

// Ejercicio 1: Identity Function
fn identity<T>(value: T) -> T {
    value
}

// Ejercicio 2: Swap Values
fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

// Ejercicio 3: Larger of Two
fn larger<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// Ejercicio 4: First Element
fn first<T: Clone>(elements: &[T]) -> Option<T> {
    elements.first().cloned()
    // Alternativa más explícita:
    // if elements.is_empty() {
    //     None
    // } else {
    //     Some(elements[0].clone())
    // }
}

// Ejercicio 5: Count Elements
fn count<T>(elements: &[T]) -> usize {
    elements.len()
}
