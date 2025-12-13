//! # Calculadora Testeada
//!
//! Una calculadora completa con tests exhaustivos.
//!
//! ## Ejemplo
//!
//! ```
//! use proyecto_calculadora_testeada::Calculadora;
//!
//! let mut calc = Calculadora::new();
//! calc.sumar(10.0);
//! calc.multiplicar(2.0);
//! assert_eq!(calc.resultado(), 20.0);
//! ```

/// Errores posibles en la calculadora.
#[derive(Debug, Clone, PartialEq)]
pub enum CalculadoraError {
    /// Division por cero
    DivisionPorCero,
    /// Raiz de numero negativo
    RaizNegativa,
    /// Factorial de numero negativo
    FactorialNegativo,
    /// Overflow en operacion
    Overflow,
}

impl std::fmt::Display for CalculadoraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DivisionPorCero => write!(f, "Division por cero"),
            Self::RaizNegativa => write!(f, "No se puede calcular raiz de numero negativo"),
            Self::FactorialNegativo => write!(f, "No se puede calcular factorial de numero negativo"),
            Self::Overflow => write!(f, "Overflow en operacion"),
        }
    }
}

impl std::error::Error for CalculadoraError {}

/// Una calculadora con historial.
///
/// # Example
///
/// ```
/// use proyecto_calculadora_testeada::Calculadora;
///
/// let mut calc = Calculadora::new();
/// calc.sumar(5.0);
/// calc.restar(2.0);
/// assert_eq!(calc.resultado(), 3.0);
/// ```
#[derive(Debug)]
pub struct Calculadora {
    valor: f64,
    historial: Vec<String>,
}

impl Default for Calculadora {
    fn default() -> Self {
        Self::new()
    }
}

impl Calculadora {
    /// Crea una calculadora con valor 0.
    ///
    /// # Example
    ///
    /// ```
    /// use proyecto_calculadora_testeada::Calculadora;
    ///
    /// let calc = Calculadora::new();
    /// assert_eq!(calc.resultado(), 0.0);
    /// ```
    pub fn new() -> Self {
        Calculadora {
            valor: 0.0,
            historial: Vec::new(),
        }
    }

    /// Crea una calculadora con un valor inicial.
    pub fn con_valor(valor: f64) -> Self {
        Calculadora {
            valor,
            historial: vec![format!("Inicio: {}", valor)],
        }
    }

    /// Retorna el resultado actual.
    pub fn resultado(&self) -> f64 {
        self.valor
    }

    /// Suma un valor.
    ///
    /// # Example
    ///
    /// ```
    /// use proyecto_calculadora_testeada::Calculadora;
    ///
    /// let mut calc = Calculadora::con_valor(10.0);
    /// calc.sumar(5.0);
    /// assert_eq!(calc.resultado(), 15.0);
    /// ```
    pub fn sumar(&mut self, n: f64) {
        self.valor += n;
        self.historial.push(format!("+ {}", n));
    }

    /// Resta un valor.
    pub fn restar(&mut self, n: f64) {
        self.valor -= n;
        self.historial.push(format!("- {}", n));
    }

    /// Multiplica por un valor.
    pub fn multiplicar(&mut self, n: f64) {
        self.valor *= n;
        self.historial.push(format!("* {}", n));
    }

    /// Divide por un valor.
    ///
    /// # Errors
    ///
    /// Retorna `CalculadoraError::DivisionPorCero` si n es 0.
    ///
    /// # Example
    ///
    /// ```
    /// use proyecto_calculadora_testeada::Calculadora;
    ///
    /// let mut calc = Calculadora::con_valor(20.0);
    /// calc.dividir(4.0).unwrap();
    /// assert_eq!(calc.resultado(), 5.0);
    /// ```
    pub fn dividir(&mut self, n: f64) -> Result<(), CalculadoraError> {
        if n == 0.0 {
            return Err(CalculadoraError::DivisionPorCero);
        }
        self.valor /= n;
        self.historial.push(format!("/ {}", n));
        Ok(())
    }

    /// Eleva a una potencia.
    pub fn potencia(&mut self, exp: f64) {
        self.valor = self.valor.powf(exp);
        self.historial.push(format!("^ {}", exp));
    }

    /// Calcula la raiz cuadrada.
    ///
    /// # Errors
    ///
    /// Retorna error si el valor es negativo.
    pub fn raiz(&mut self) -> Result<(), CalculadoraError> {
        if self.valor < 0.0 {
            return Err(CalculadoraError::RaizNegativa);
        }
        self.valor = self.valor.sqrt();
        self.historial.push("sqrt".to_string());
        Ok(())
    }

    /// Reinicia la calculadora.
    pub fn limpiar(&mut self) {
        self.valor = 0.0;
        self.historial.clear();
    }

    /// Retorna el historial de operaciones.
    pub fn historial(&self) -> &[String] {
        &self.historial
    }
}

/// Calcula el factorial de un numero.
///
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
/// use proyecto_calculadora_testeada::mcd;
///
/// assert_eq!(mcd(12, 8), 4);
/// assert_eq!(mcd(17, 13), 1);
/// ```
pub fn mcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        mcd(b, a % b)
    }
}

/// Calcula el minimo comun multiplo.
///
/// # Example
///
/// ```
/// use proyecto_calculadora_testeada::mcm;
///
/// assert_eq!(mcm(4, 6), 12);
/// ```
pub fn mcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / mcd(a, b)
    }
}

// ============================================
// TESTS UNITARIOS
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    mod calculadora_basica {
        use super::*;

        #[test]
        fn test_nueva() {
            let calc = Calculadora::new();
            assert_eq!(calc.resultado(), 0.0);
        }

        #[test]
        fn test_con_valor() {
            let calc = Calculadora::con_valor(42.0);
            assert_eq!(calc.resultado(), 42.0);
        }

        #[test]
        fn test_sumar() {
            let mut calc = Calculadora::new();
            calc.sumar(5.0);
            assert_eq!(calc.resultado(), 5.0);
        }

        #[test]
        fn test_restar() {
            let mut calc = Calculadora::con_valor(10.0);
            calc.restar(3.0);
            assert_eq!(calc.resultado(), 7.0);
        }

        #[test]
        fn test_multiplicar() {
            let mut calc = Calculadora::con_valor(6.0);
            calc.multiplicar(7.0);
            assert_eq!(calc.resultado(), 42.0);
        }

        #[test]
        fn test_dividir() {
            let mut calc = Calculadora::con_valor(20.0);
            calc.dividir(4.0).unwrap();
            assert_eq!(calc.resultado(), 5.0);
        }

        #[test]
        fn test_dividir_por_cero() {
            let mut calc = Calculadora::con_valor(10.0);
            let result = calc.dividir(0.0);
            assert_eq!(result, Err(CalculadoraError::DivisionPorCero));
        }
    }

    mod calculadora_avanzada {
        use super::*;

        #[test]
        fn test_potencia() {
            let mut calc = Calculadora::con_valor(2.0);
            calc.potencia(10.0);
            assert_eq!(calc.resultado(), 1024.0);
        }

        #[test]
        fn test_raiz() {
            let mut calc = Calculadora::con_valor(16.0);
            calc.raiz().unwrap();
            assert_eq!(calc.resultado(), 4.0);
        }

        #[test]
        fn test_raiz_negativa() {
            let mut calc = Calculadora::con_valor(-4.0);
            let result = calc.raiz();
            assert_eq!(result, Err(CalculadoraError::RaizNegativa));
        }

        #[test]
        fn test_limpiar() {
            let mut calc = Calculadora::con_valor(100.0);
            calc.sumar(50.0);
            calc.limpiar();
            assert_eq!(calc.resultado(), 0.0);
            assert!(calc.historial().is_empty());
        }
    }

    mod historial {
        use super::*;

        #[test]
        fn test_historial_vacio() {
            let calc = Calculadora::new();
            assert!(calc.historial().is_empty());
        }

        #[test]
        fn test_historial_operaciones() {
            let mut calc = Calculadora::new();
            calc.sumar(5.0);
            calc.multiplicar(2.0);
            assert_eq!(calc.historial().len(), 2);
        }
    }

    mod funciones_matematicas {
        use super::*;

        #[test]
        fn test_factorial_cero() {
            assert_eq!(factorial(0).unwrap(), 1);
        }

        #[test]
        fn test_factorial_cinco() {
            assert_eq!(factorial(5).unwrap(), 120);
        }

        #[test]
        fn test_factorial_negativo() {
            assert_eq!(factorial(-1), Err(CalculadoraError::FactorialNegativo));
        }

        #[test]
        fn test_factorial_overflow() {
            assert_eq!(factorial(21), Err(CalculadoraError::Overflow));
        }

        #[test]
        fn test_mcd() {
            assert_eq!(mcd(12, 8), 4);
            assert_eq!(mcd(17, 13), 1);
            assert_eq!(mcd(100, 25), 25);
        }

        #[test]
        fn test_mcm() {
            assert_eq!(mcm(4, 6), 12);
            assert_eq!(mcm(3, 5), 15);
        }
    }
}
