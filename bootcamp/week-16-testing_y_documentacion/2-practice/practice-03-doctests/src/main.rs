//! # Biblioteca de Utilidades Matematicas
//!
//! Esta biblioteca proporciona funciones matematicas
//! con documentacion completa y ejemplos ejecutables.
//!
//! ## Ejemplo
//!
//! ```
//! use practice_03_doctests::{add, factorial};
//!
//! let result = add(2, 3);
//! assert_eq!(result, 5);
//! ```

/// Suma dos numeros enteros.
///
/// # Arguments
///
/// * `a` - Primer sumando
/// * `b` - Segundo sumando
///
/// # Returns
///
/// La suma de `a` y `b`.
///
/// # Example
///
/// ```
/// use practice_03_doctests::add;
///
/// assert_eq!(add(2, 3), 5);
/// assert_eq!(add(-1, 1), 0);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Calcula el factorial de un numero.
///
/// # Arguments
///
/// * `n` - Numero del cual calcular el factorial
///
/// # Returns
///
/// El factorial de `n`.
///
/// # Panics
///
/// Produce panic si `n` es mayor que 20 (overflow).
///
/// # Example
///
/// ```
/// use practice_03_doctests::factorial;
///
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(5), 120);
/// ```
///
/// ```should_panic
/// use practice_03_doctests::factorial;
///
/// factorial(21); // overflow!
/// ```
pub fn factorial(n: u64) -> u64 {
    if n > 20 {
        panic!("Factorial overflow para n > 20");
    }
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

/// Divide dos numeros de forma segura.
///
/// # Arguments
///
/// * `dividend` - El numero a dividir
/// * `divisor` - El numero por el cual dividir
///
/// # Returns
///
/// `Ok(quotient)` si la division es valida.
///
/// # Errors
///
/// Retorna `Err` si el divisor es cero.
///
/// # Example
///
/// ```
/// use practice_03_doctests::divide;
///
/// let result = divide(10, 2);
/// assert_eq!(result, Ok(5));
///
/// let error = divide(10, 0);
/// assert!(error.is_err());
/// ```
pub fn divide(dividend: i32, divisor: i32) -> Result<i32, &'static str> {
    if divisor == 0 {
        Err("Division por cero")
    } else {
        Ok(dividend / divisor)
    }
}

/// Verifica si un numero es primo.
///
/// # Arguments
///
/// * `n` - Numero a verificar
///
/// # Returns
///
/// `true` si `n` es primo, `false` en caso contrario.
///
/// # Example
///
/// ```
/// use practice_03_doctests::is_prime;
///
/// assert!(is_prime(2));
/// assert!(is_prime(17));
/// assert!(!is_prime(4));
/// assert!(!is_prime(1));
/// ```
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as u64;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// Representa un punto en 2D.
///
/// # Example
///
/// ```
/// use practice_03_doctests::Point;
///
/// let p = Point::new(3.0, 4.0);
/// assert_eq!(p.distance_to_origin(), 5.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    /// Coordenada X
    pub x: f64,
    /// Coordenada Y
    pub y: f64,
}

impl Point {
    /// Crea un nuevo punto.
    ///
    /// # Example
    ///
    /// ```
    /// use practice_03_doctests::Point;
    ///
    /// let origin = Point::new(0.0, 0.0);
    /// assert_eq!(origin.x, 0.0);
    /// ```
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    /// Calcula la distancia al origen.
    ///
    /// # Example
    ///
    /// ```
    /// use practice_03_doctests::Point;
    ///
    /// let p = Point::new(3.0, 4.0);
    /// assert_eq!(p.distance_to_origin(), 5.0);
    /// ```
    pub fn distance_to_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// Calcula la distancia a otro punto.
    ///
    /// # Example
    ///
    /// ```
    /// use practice_03_doctests::Point;
    ///
    /// let p1 = Point::new(0.0, 0.0);
    /// let p2 = Point::new(3.0, 4.0);
    /// assert_eq!(p1.distance_to(&p2), 5.0);
    /// ```
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

fn main() {
    println!("=== Practica 03: Doc Tests ===\n");
    println!("Ejecuta: cargo test --doc");
    println!("Ejecuta: cargo doc --open");
}
