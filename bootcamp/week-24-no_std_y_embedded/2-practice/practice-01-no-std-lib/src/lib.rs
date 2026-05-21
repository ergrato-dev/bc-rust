//! Librería `no_std` básica — solo `core::`.
//!
//! No hay acceso a heap, sistema de archivos, threads ni I/O estándar.
#![no_std]

use core::cmp::Ordering;

/// Busca el valor mínimo en un slice.
///
/// Retorna `None` si el slice está vacío.
pub fn minimo(datos: &[i32]) -> Option<i32> {
    datos.iter().copied().reduce(|a, b| if a <= b { a } else { b })
}

/// Busca el valor máximo en un slice.
pub fn maximo(datos: &[i32]) -> Option<i32> {
    datos.iter().copied().reduce(|a, b| if a >= b { a } else { b })
}

/// Ordena un slice in-place usando insertion sort (O(n²), sin allocación).
pub fn insertion_sort(datos: &mut [i32]) {
    for i in 1..datos.len() {
        let key = datos[i];
        let mut j = i;
        while j > 0 && datos[j - 1] > key {
            datos[j] = datos[j - 1];
            j -= 1;
        }
        datos[j] = key;
    }
}

/// Busca binaria en un slice ordenado. Retorna el índice si se encuentra.
pub fn busqueda_binaria(datos: &[i32], target: i32) -> Option<usize> {
    let mut lo = 0;
    let mut hi = datos.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match datos[mid].cmp(&target) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => lo = mid + 1,
            Ordering::Greater => hi = mid,
        }
    }
    None
}

/// Calcula el número de Fibonacci usando aritmética saturante.
pub fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 1..n {
        (a, b) = (b, a.saturating_add(b));
    }
    b
}

// Tests habilitados solo cuando std está disponible (test runner)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimo_correcto() {
        assert_eq!(minimo(&[5, 3, 8, 1, 9]), Some(1));
        assert_eq!(minimo(&[]), None);
    }

    #[test]
    fn insertion_sort_correcto() {
        let mut datos = [5i32, 3, 8, 1, 9, 2];
        insertion_sort(&mut datos);
        assert_eq!(datos, [1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn busqueda_binaria_correcta() {
        let datos = [1i32, 2, 3, 5, 8, 9];
        assert_eq!(busqueda_binaria(&datos, 5), Some(3));
        assert_eq!(busqueda_binaria(&datos, 7), None);
    }

    #[test]
    fn fibonacci_correcto() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(10), 55);
    }
}
