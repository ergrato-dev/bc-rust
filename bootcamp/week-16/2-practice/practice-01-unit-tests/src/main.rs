//! Practica 01: Tests Unitarios
//!
//! Implementa una calculadora con tests completos.

fn main() {
    println!("=== Practica 01: Tests Unitarios ===\n");

    println!("Suma: 5 + 3 = {}", suma(5, 3));
    println!("Resta: 10 - 4 = {}", resta(10, 4));
    println!("Multiplica: 6 * 7 = {}", multiplica(6, 7));
    println!("Divide: 20 / 4 = {}", dividir(20, 4));

    match dividir_safe(10, 2) {
        Ok(r) => println!("Divide safe: 10 / 2 = {}", r),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nEjecuta: cargo test");
}

// ============================================
// EJERCICIO 1: Operaciones basicas
// ============================================

/// Suma dos numeros.
pub fn suma(a: i32, b: i32) -> i32 {
    a + b
}

/// Resta dos numeros.
pub fn resta(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplica dos numeros.
pub fn multiplica(a: i32, b: i32) -> i32 {
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
pub fn dividir(a: i32, b: i32) -> i32 {
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
pub fn dividir_safe(a: i32, b: i32) -> Result<i32, String> {
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
    mod suma_tests {
        use super::*;

        #[test]
        fn test_suma_positivos() {
            assert_eq!(suma(2, 3), 5);
        }

        #[test]
        fn test_suma_negativos() {
            assert_eq!(suma(-2, -3), -5);
        }

        #[test]
        fn test_suma_mixtos() {
            assert_eq!(suma(-5, 10), 5);
        }

        #[test]
        fn test_suma_con_cero() {
            assert_eq!(suma(42, 0), 42);
        }
    }

    // ------------------------------------------
    // Tests de resta
    // ------------------------------------------
    mod resta_tests {
        use super::*;

        #[test]
        fn test_resta_basica() {
            assert_eq!(resta(10, 4), 6);
        }

        #[test]
        fn test_resta_resultado_negativo() {
            assert_eq!(resta(3, 10), -7);
        }
    }

    // ------------------------------------------
    // Tests de multiplicacion
    // ------------------------------------------
    mod multiplica_tests {
        use super::*;

        #[test]
        fn test_multiplica_positivos() {
            assert_eq!(multiplica(6, 7), 42);
        }

        #[test]
        fn test_multiplica_por_cero() {
            assert_eq!(multiplica(100, 0), 0);
        }

        #[test]
        fn test_multiplica_negativos() {
            assert_eq!(multiplica(-3, -4), 12);
        }
    }

    // ------------------------------------------
    // Tests de division
    // ------------------------------------------
    mod dividir_tests {
        use super::*;

        #[test]
        fn test_dividir_exacta() {
            assert_eq!(dividir(20, 4), 5);
        }

        #[test]
        fn test_dividir_con_resto() {
            assert_eq!(dividir(10, 3), 3);
        }

        #[test]
        #[should_panic(expected = "Division por cero")]
        fn test_dividir_por_cero() {
            dividir(10, 0);
        }
    }

    // ------------------------------------------
    // Tests de division safe
    // ------------------------------------------
    mod dividir_safe_tests {
        use super::*;

        #[test]
        fn test_dividir_safe_ok() {
            assert_eq!(dividir_safe(10, 2), Ok(5));
        }

        #[test]
        fn test_dividir_safe_error() {
            assert!(dividir_safe(10, 0).is_err());
        }

        #[test]
        fn test_dividir_safe_con_result() -> Result<(), String> {
            let resultado = dividir_safe(20, 4)?;
            assert_eq!(resultado, 5);
            Ok(())
        }
    }
}
