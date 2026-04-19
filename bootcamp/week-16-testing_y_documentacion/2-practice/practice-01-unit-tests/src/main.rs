//! Practica 01: Tests Unitarios
//!
//! Implementa una calculadora con tests completos.

fn main() {
    println!("=== Practica 01: Tests Unitarios ===\n");

    println!("Suma: 5 + 3 = {}", add(5, 3));
    println!("Resta: 10 - 4 = {}", subtract(10, 4));
    println!("Multiplica: 6 * 7 = {}", multiply(6, 7));
    println!("Divide: 20 / 4 = {}", divide(20, 4));

    match divide_safe(10, 2) {
        Ok(r) => println!("Divide safe: 10 / 2 = {}", r),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nEjecuta: cargo test");
}

// ============================================
// EJERCICIO 1: Operaciones basicas
// ============================================

/// Suma dos numeros.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Resta dos numeros.
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplica dos numeros.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// ============================================
// EJERCICIO 2: Division con panic
// ============================================

/// Divide dos numeros.
///
/// # Panics
///
/// Produce panic si `b` es cero.
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division por cero");
    }
    a / b
}

// ============================================
// EJERCICIO 3: Division safe con Result
// ============================================

/// Divide dos numeros de forma segura.
///
/// # Errors
///
/// Retorna error si `b` es cero.
pub fn divide_safe(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division por cero".to_string())
    } else {
        Ok(a / b)
    }
}

// ============================================
// TESTS
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------
    // Tests de suma
    // ------------------------------------------
    mod add_tests {
        use super::*;

        #[test]
        fn test_add_positives() {
            assert_eq!(add(2, 3), 5);
        }

        #[test]
        fn test_add_negatives() {
            assert_eq!(add(-2, -3), -5);
        }

        #[test]
        fn test_add_mixed() {
            assert_eq!(add(-5, 10), 5);
        }

        #[test]
        fn test_add_with_zero() {
            assert_eq!(add(42, 0), 42);
        }
    }

    // ------------------------------------------
    // Tests de resta
    // ------------------------------------------
    mod subtract_tests {
        use super::*;

        #[test]
        fn test_subtract_basic() {
            assert_eq!(subtract(10, 4), 6);
        }

        #[test]
        fn test_subtract_negative_result() {
            assert_eq!(subtract(3, 10), -7);
        }
    }

    // ------------------------------------------
    // Tests de multiplicacion
    // ------------------------------------------
    mod multiply_tests {
        use super::*;

        #[test]
        fn test_multiply_positives() {
            assert_eq!(multiply(6, 7), 42);
        }

        #[test]
        fn test_multiply_by_zero() {
            assert_eq!(multiply(100, 0), 0);
        }

        #[test]
        fn test_multiply_negatives() {
            assert_eq!(multiply(-3, -4), 12);
        }
    }

    // ------------------------------------------
    // Tests de division
    // ------------------------------------------
    mod divide_tests {
        use super::*;

        #[test]
        fn test_divide_exact() {
            assert_eq!(divide(20, 4), 5);
        }

        #[test]
        fn test_divide_with_remainder() {
            assert_eq!(divide(10, 3), 3);
        }

        #[test]
        #[should_panic(expected = "Division por cero")]
        fn test_divide_by_zero() {
            divide(10, 0);
        }
    }

    // ------------------------------------------
    // Tests de division safe
    // ------------------------------------------
    mod divide_safe_tests {
        use super::*;

        #[test]
        fn test_divide_safe_ok() {
            assert_eq!(divide_safe(10, 2), Ok(5));
        }

        #[test]
        fn test_divide_safe_error() {
            assert!(divide_safe(10, 0).is_err());
        }

        #[test]
        fn test_divide_safe_with_result() -> Result<(), String> {
            let result = divide_safe(20, 4)?;
            assert_eq!(result, 5);
            Ok(())
        }
    }
}
