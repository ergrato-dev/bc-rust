//! Ejercicio de gestión semver: simula la evolución de una API a través de versiones.
//!
//! ## Historial de versiones
//!
//! ### v0.2.0 (actual)
//! - AÑADIDO: `Calculadora::con_precision` — constructor con precisión configurable
//! - AÑADIDO: `Calculadora::historial` — acceso al historial de operaciones
//! - CHANGED: `Calculadora::dividir` ahora retorna `Result<f64, DivisionError>` (minor change)
//!
//! ### v0.1.0
//! - API inicial: `Calculadora::new`, `sumar`, `restar`, `multiplicar`, `dividir`

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CalculadoraError {
    #[error("división por cero")]
    DivisionPorCero,
}

pub struct Calculadora {
    precision: usize,
    historial: Vec<String>,
}

impl Calculadora {
    pub fn new() -> Self {
        Calculadora { precision: 2, historial: Vec::new() }
    }

    pub fn con_precision(precision: usize) -> Self {
        Calculadora { precision, historial: Vec::new() }
    }

    pub fn historial(&self) -> &[String] {
        &self.historial
    }

    pub fn sumar(&mut self, a: f64, b: f64) -> f64 {
        let r = (a + b * 10_f64.powi(self.precision as i32)).round()
            / 10_f64.powi(self.precision as i32);
        self.historial.push(format!("{a} + {b} = {r}"));
        r
    }

    pub fn dividir(&mut self, a: f64, b: f64) -> Result<f64, CalculadoraError> {
        if b == 0.0 {
            return Err(CalculadoraError::DivisionPorCero);
        }
        let r = a / b;
        self.historial.push(format!("{a} / {b} = {r}"));
        Ok(r)
    }
}

impl Default for Calculadora {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let mut calc = Calculadora::new();
    calc.sumar(1.5, 2.3);
    println!("Historial: {:?}", calc.historial());
    match calc.dividir(10.0, 0.0) {
        Err(e) => println!("Error esperado: {e}"),
        Ok(v) => println!("Resultado: {v}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suma_basica() {
        let mut c = Calculadora::new();
        let _ = c.sumar(1.0, 2.0);
        assert_eq!(c.historial().len(), 1);
    }

    #[test]
    fn division_por_cero() {
        let mut c = Calculadora::new();
        assert!(c.dividir(5.0, 0.0).is_err());
    }

    #[test]
    fn division_valida() {
        let mut c = Calculadora::new();
        assert!((c.dividir(10.0, 4.0).unwrap() - 2.5).abs() < 1e-10);
    }
}
