//! Librería de utilidades matemáticas con doctests completos.
//!
//! # Ejemplos
//!
//! ```
//! use practice_03_doctests_ejemplos::{factorial, fibonacci, es_primo};
//!
//! assert_eq!(factorial(5), 120);
//! assert_eq!(fibonacci(10), 55);
//! assert!(es_primo(17));
//! ```

/// Calcula el factorial de `n`.
///
/// # Examples
///
/// ```
/// use practice_03_doctests_ejemplos::factorial;
///
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(1), 1);
/// assert_eq!(factorial(5), 120);
/// assert_eq!(factorial(10), 3628800);
/// ```
///
/// # Panics
///
/// Panics si `n > 20` (overflow de `u64`).
pub fn factorial(n: u64) -> u64 {
    assert!(n <= 20, "factorial: n={n} causaría overflow de u64");
    (1..=n).product()
}

/// Calcula el n-ésimo número de Fibonacci (0-indexed).
///
/// # Examples
///
/// ```
/// use practice_03_doctests_ejemplos::fibonacci;
///
/// assert_eq!(fibonacci(0), 0);
/// assert_eq!(fibonacci(1), 1);
/// assert_eq!(fibonacci(10), 55);
/// ```
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut a, mut b) = (0u64, 1u64);
            for _ in 2..=n {
                (a, b) = (b, a + b);
            }
            b
        }
    }
}

/// Determina si `n` es primo.
///
/// # Examples
///
/// ```
/// use practice_03_doctests_ejemplos::es_primo;
///
/// assert!(!es_primo(0));
/// assert!(!es_primo(1));
/// assert!(es_primo(2));
/// assert!(es_primo(17));
/// assert!(!es_primo(100));
/// ```
pub fn es_primo(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as u64 + 1;
    (3..=limit).step_by(2).all(|i| n % i != 0)
}

fn main() {
    println!("factorial(10) = {}", factorial(10));
    println!("fibonacci(10) = {}", fibonacci(10));
    println!("es_primo(17)  = {}", es_primo(17));
}
