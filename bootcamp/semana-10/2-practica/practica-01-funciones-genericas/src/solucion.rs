// ============================================
// SOLUCIONES - Práctica 01: Funciones Genéricas
// ============================================
// ⚠️  NO MIRAR HASTA INTENTAR RESOLVER LOS EJERCICIOS

#![allow(dead_code)]

// Ejercicio 1: Función Identidad
fn identidad<T>(valor: T) -> T {
    valor
}

// Ejercicio 2: Intercambiar Valores
fn intercambiar<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

// Ejercicio 3: Mayor de Dos
fn mayor<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// Ejercicio 4: Primer Elemento
fn primero<T: Clone>(elementos: &[T]) -> Option<T> {
    elementos.first().cloned()
    // Alternativa más explícita:
    // if elementos.is_empty() {
    //     None
    // } else {
    //     Some(elementos[0].clone())
    // }
}

// Ejercicio 5: Contar Elementos
fn contar<T>(elementos: &[T]) -> usize {
    elementos.len()
}
