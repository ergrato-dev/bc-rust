//! # Biblioteca de Utilidades Matematicas
//!
//! Esta biblioteca proporciona funciones matematicas
//! con documentacion completa y ejemplos ejecutables.
//!
//! ## Ejemplo
//!
//! ```
//! use practica_03_doctests::{suma, factorial};
//!
//! let resultado = suma(2, 3);
//! assert_eq!(resultado, 5);
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
/// use practica_03_doctests::suma;
///
/// assert_eq!(suma(2, 3), 5);
/// assert_eq!(suma(-1, 1), 0);
/// ```
pub fn suma(a: i32, b: i32) -> i32 {
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
/// use practica_03_doctests::factorial;
///
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(5), 120);
/// ```
///
/// ```should_panic
/// use practica_03_doctests::factorial;
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
/// * `dividendo` - El numero a dividir
/// * `divisor` - El numero por el cual dividir
///
/// # Returns
///
/// `Ok(cociente)` si la division es valida.
///
/// # Errors
///
/// Retorna `Err` si el divisor es cero.
///
/// # Example
///
/// ```
/// use practica_03_doctests::dividir;
///
/// let resultado = dividir(10, 2);
/// assert_eq!(resultado, Ok(5));
///
/// let error = dividir(10, 0);
/// assert!(error.is_err());
/// ```
pub fn dividir(dividendo: i32, divisor: i32) -> Result<i32, &'static str> {
    if divisor == 0 {
        Err("Division por cero")
    } else {
        Ok(dividendo / divisor)
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
/// use practica_03_doctests::es_primo;
///
/// assert!(es_primo(2));
/// assert!(es_primo(17));
/// assert!(!es_primo(4));
/// assert!(!es_primo(1));
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
    let limite = (n as f64).sqrt() as u64;
    for i in (3..=limite).step_by(2) {
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
/// use practica_03_doctests::Punto;
///
/// let p = Punto::new(3.0, 4.0);
/// assert_eq!(p.distancia_origen(), 5.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Punto {
    /// Coordenada X
    pub x: f64,
    /// Coordenada Y
    pub y: f64,
}

impl Punto {
    /// Crea un nuevo punto.
    ///
    /// # Example
    ///
    /// ```
    /// use practica_03_doctests::Punto;
    ///
    /// let origen = Punto::new(0.0, 0.0);
    /// assert_eq!(origen.x, 0.0);
    /// ```
    pub fn new(x: f64, y: f64) -> Self {
        Punto { x, y }
    }

    /// Calcula la distancia al origen.
    ///
    /// # Example
    ///
    /// ```
    /// use practica_03_doctests::Punto;
    ///
    /// let p = Punto::new(3.0, 4.0);
    /// assert_eq!(p.distancia_origen(), 5.0);
    /// ```
    pub fn distancia_origen(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// Calcula la distancia a otro punto.
    ///
    /// # Example
    ///
    /// ```
    /// use practica_03_doctests::Punto;
    ///
    /// let p1 = Punto::new(0.0, 0.0);
    /// let p2 = Punto::new(3.0, 4.0);
    /// assert_eq!(p1.distancia_a(&p2), 5.0);
    /// ```
    pub fn distancia_a(&self, otro: &Punto) -> f64 {
        let dx = self.x - otro.x;
        let dy = self.y - otro.y;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

fn main() {
    println!("=== Practica 03: Doc Tests ===\n");
    println!("Ejecuta: cargo test --doc");
    println!("Ejecuta: cargo doc --open");
}
