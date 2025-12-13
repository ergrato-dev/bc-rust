//! # Calculadora Testeada
//!
//! Una calculadora completa con tests exhaustivos.
//!
//! ## Ejemplo
//!
//! ```
//! use project_tested_calculator::Calculator;
//!
//! let mut calc = Calculator::new();
//! calc.add(10.0);
//! calc.multiply(2.0);
//! assert_eq!(calc.result(), 20.0);
//! ```

/// Errores posibles en la calculadora.
#[derive(Debug, Clone, PartialEq)]
pub enum CalculatorError {
    /// Division por cero
    DivisionByZero,
    /// Raiz de numero negativo
    NegativeRoot,
    /// Factorial de numero negativo
    NegativeFactorial,
    /// Overflow en operacion
    Overflow,
}

impl std::fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DivisionByZero => write!(f, "Division por cero"),
            Self::NegativeRoot => write!(f, "No se puede calcular raiz de numero negativo"),
            Self::NegativeFactorial => write!(f, "No se puede calcular factorial de numero negativo"),
            Self::Overflow => write!(f, "Overflow en operacion"),
        }
    }
}

impl std::error::Error for CalculatorError {}

/// Una calculadora con historial.
///
/// # Example
///
/// ```
/// use project_tested_calculator::Calculator;
///
/// let mut calc = Calculator::new();
/// calc.add(5.0);
/// calc.subtract(2.0);
/// assert_eq!(calc.result(), 3.0);
/// ```
#[derive(Debug)]
pub struct Calculator {
    value: f64,
    history: Vec<String>,
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

impl Calculator {
    /// Crea una calculadora con valor 0.
    ///
    /// # Example
    ///
    /// ```
    /// use project_tested_calculator::Calculator;
    ///
    /// let calc = Calculator::new();
    /// assert_eq!(calc.result(), 0.0);
    /// ```
    pub fn new() -> Self {
        Calculator {
            value: 0.0,
            history: Vec::new(),
        }
    }

    /// Crea una calculadora con un valor inicial.
    pub fn with_value(value: f64) -> Self {
        Calculator {
            value,
            history: vec![format!("Inicio: {}", value)],
        }
    }

    /// Retorna el resultado actual.
    pub fn result(&self) -> f64 {
        self.value
    }

    /// Suma un valor.
    ///
    /// # Example
    ///
    /// ```
    /// use project_tested_calculator::Calculator;
    ///
    /// let mut calc = Calculator::with_value(10.0);
    /// calc.add(5.0);
    /// assert_eq!(calc.result(), 15.0);
    /// ```
    pub fn add(&mut self, n: f64) {
        self.value += n;
        self.history.push(format!("+ {}", n));
    }

    /// Resta un valor.
    pub fn subtract(&mut self, n: f64) {
        self.value -= n;
        self.history.push(format!("- {}", n));
    }

    /// Multiplica por un valor.
    pub fn multiply(&mut self, n: f64) {
        self.value *= n;
        self.history.push(format!("* {}", n));
    }

    /// Divide por un valor.
    ///
    /// # Errors
    ///
    /// Retorna `CalculatorError::DivisionByZero` si n es 0.
    ///
    /// # Example
    ///
    /// ```
    /// use project_tested_calculator::Calculator;
    ///
    /// let mut calc = Calculator::with_value(20.0);
    /// calc.divide(4.0).unwrap();
    /// assert_eq!(calc.result(), 5.0);
    /// ```
    pub fn divide(&mut self, n: f64) -> Result<(), CalculatorError> {
        if n == 0.0 {
            return Err(CalculatorError::DivisionByZero);
        }
        self.value /= n;
        self.history.push(format!("/ {}", n));
        Ok(())
    }

    /// Eleva a una potencia.
    pub fn power(&mut self, exp: f64) {
        self.value = self.value.powf(exp);
        self.history.push(format!("^ {}", exp));
    }

    /// Calcula la raiz cuadrada.
    ///
    /// # Errors
    ///
    /// Retorna error si el valor es negativo.
    pub fn sqrt(&mut self) -> Result<(), CalculatorError> {
        if self.value < 0.0 {
            return Err(CalculatorError::NegativeRoot);
        }
        self.value = self.value.sqrt();
        self.history.push("sqrt".to_string());
        Ok(())
    }

    /// Reinicia la calculadora.
    pub fn clear(&mut self) {
        self.value = 0.0;
        self.history.clear();
    }

    /// Retorna el historial de operaciones.
    pub fn history(&self) -> &[String] {
        &self.history
/// # Arguments
///
/// * `n` - Numero entero no negativo
///
/// # Errors
///
/// Retorna error si n es negativo o causa overflow.
///
/// # Example
///
/// ```
/// use proyecto_calculadora_testeada::factorial;
///
/// assert_eq!(factorial(5).unwrap(), 120);
/// assert_eq!(factorial(0).unwrap(), 1);
/// ```
pub fn factorial(n: i64) -> Result<u64, CalculadoraError> {
    if n < 0 {
        return Err(CalculadoraError::FactorialNegativo);
    }
    if n > 20 {
        return Err(CalculadoraError::Overflow);
    }

    let mut resultado: u64 = 1;
    for i in 2..=n as u64 {
        resultado *= i;
    }
    Ok(resultado)
}

/// Calcula el maximo comun divisor.
///
/// # Example
///
/// ```
/// use project_tested_calculator::gcd;
///
/// assert_eq!(gcd(12, 8), 4);
/// assert_eq!(gcd(17, 13), 1);
/// ```
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Calcula el minimo comun multiplo.
///
/// # Example
///
/// ```
/// use project_tested_calculator::lcm;
///
/// assert_eq!(lcm(4, 6), 12);
/// ```
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

// ============================================
// TESTS UNITARIOS
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    mod calculator_basic {
        use super::*;

        #[test]
        fn test_new() {
            let calc = Calculator::new();
            assert_eq!(calc.result(), 0.0);
        }

        #[test]
        fn test_with_value() {
            let calc = Calculator::with_value(42.0);
            assert_eq!(calc.result(), 42.0);
        }

        #[test]
        fn test_add() {
            let mut calc = Calculator::new();
            calc.add(5.0);
            assert_eq!(calc.result(), 5.0);
        }

        #[test]
        fn test_subtract() {
            let mut calc = Calculator::with_value(10.0);
            calc.subtract(3.0);
            assert_eq!(calc.result(), 7.0);
        }

        #[test]
        fn test_multiply() {
            let mut calc = Calculator::with_value(6.0);
            calc.multiply(7.0);
            assert_eq!(calc.result(), 42.0);
        }

        #[test]
        fn test_divide() {
            let mut calc = Calculator::with_value(20.0);
            calc.divide(4.0).unwrap();
            assert_eq!(calc.result(), 5.0);
        }

        #[test]
        fn test_divide_by_zero() {
            let mut calc = Calculator::with_value(10.0);
            let result = calc.divide(0.0);
            assert_eq!(result, Err(CalculatorError::DivisionByZero));
        }
    }

    mod calculator_advanced {
        use super::*;

        #[test]
        fn test_power() {
            let mut calc = Calculator::with_value(2.0);
            calc.power(10.0);
            assert_eq!(calc.result(), 1024.0);
        }

        #[test]
        fn test_sqrt() {
            let mut calc = Calculator::with_value(16.0);
            calc.sqrt().unwrap();
            assert_eq!(calc.result(), 4.0);
        }

        #[test]
        fn test_sqrt_negative() {
            let mut calc = Calculator::with_value(-4.0);
            let result = calc.sqrt();
            assert_eq!(result, Err(CalculatorError::NegativeRoot));
        }

        #[test]
        fn test_clear() {
            let mut calc = Calculator::with_value(100.0);
            calc.add(50.0);
            calc.clear();
            assert_eq!(calc.result(), 0.0);
            assert!(calc.history().is_empty());
        }
    }

    mod history_tests {
        use super::*;

        #[test]
        fn test_history_empty() {
            let calc = Calculator::new();
            assert!(calc.history().is_empty());
        }

        #[test]
        fn test_history_operations() {
            let mut calc = Calculator::new();
            calc.add(5.0);
            calc.multiply(2.0);
            assert_eq!(calc.history().len(), 2);
        }
    }

    mod math_functions {
        use super::*;

        #[test]
        fn test_factorial_zero() {
            assert_eq!(factorial(0).unwrap(), 1);
        }

        #[test]
        fn test_factorial_five() {
            assert_eq!(factorial(5).unwrap(), 120);
        }

        #[test]
        fn test_factorial_negative() {
            assert_eq!(factorial(-1), Err(CalculatorError::NegativeFactorial));
        }

        #[test]
        fn test_factorial_overflow() {
            assert_eq!(factorial(21), Err(CalculatorError::Overflow));
        }

        #[test]
        fn test_gcd() {
            assert_eq!(gcd(12, 8), 4);
            assert_eq!(gcd(17, 13), 1);
            assert_eq!(gcd(100, 25), 25);
        }

        #[test]
        fn test_lcm() {
            assert_eq!(lcm(4, 6), 12);
            assert_eq!(lcm(3, 5), 15);
        }
    }
}
