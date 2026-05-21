use wasm_bindgen::prelude::*;

/// Calculadora con estado persistente accesible desde JavaScript.
#[wasm_bindgen]
pub struct Calculadora {
    acumulador: f64,
    historial: Vec<String>,
}

#[wasm_bindgen]
impl Calculadora {
    /// Crea una nueva calculadora con el acumulador en 0.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Calculadora {
        Calculadora {
            acumulador: 0.0,
            historial: Vec::new(),
        }
    }

    /// Retorna el valor actual del acumulador.
    pub fn valor(&self) -> f64 {
        self.acumulador
    }

    /// Suma al acumulador.
    pub fn sumar(&mut self, n: f64) -> f64 {
        self.historial.push(format!("+ {n}"));
        self.acumulador += n;
        self.acumulador
    }

    /// Resta al acumulador.
    pub fn restar(&mut self, n: f64) -> f64 {
        self.historial.push(format!("- {n}"));
        self.acumulador -= n;
        self.acumulador
    }

    /// Multiplica el acumulador.
    pub fn multiplicar(&mut self, n: f64) -> f64 {
        self.historial.push(format!("* {n}"));
        self.acumulador *= n;
        self.acumulador
    }

    /// Divide el acumulador. Retorna `NaN` si el divisor es 0.
    pub fn dividir(&mut self, n: f64) -> f64 {
        if n == 0.0 {
            return f64::NAN;
        }
        self.historial.push(format!("/ {n}"));
        self.acumulador /= n;
        self.acumulador
    }

    /// Resetea el acumulador y el historial a 0.
    pub fn resetear(&mut self) {
        self.acumulador = 0.0;
        self.historial.clear();
    }

    /// Retorna el número de operaciones realizadas.
    pub fn num_operaciones(&self) -> u32 {
        self.historial.len() as u32
    }
}

impl Default for Calculadora {
    fn default() -> Self {
        Self::new()
    }
}

/// Calcula la raíz cuadrada de un número.
#[wasm_bindgen]
pub fn raiz_cuadrada(n: f64) -> f64 {
    n.sqrt()
}

/// Calcula n elevado a la potencia p.
#[wasm_bindgen]
pub fn potencia(base: f64, exponente: f64) -> f64 {
    base.powf(exponente)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculadora_suma_y_resta() {
        let mut c = Calculadora::new();
        c.sumar(10.0);
        c.restar(3.0);
        assert_eq!(c.valor(), 7.0);
        assert_eq!(c.num_operaciones(), 2);
    }

    #[test]
    fn calculadora_multiplica_divide() {
        let mut c = Calculadora::new();
        c.sumar(10.0);
        c.multiplicar(2.0);
        c.dividir(4.0);
        assert_eq!(c.valor(), 5.0);
    }

    #[test]
    fn division_por_cero_retorna_nan() {
        let mut c = Calculadora::new();
        let r = c.dividir(0.0);
        assert!(r.is_nan());
    }

    #[test]
    fn resetear_limpia_estado() {
        let mut c = Calculadora::new();
        c.sumar(100.0);
        c.resetear();
        assert_eq!(c.valor(), 0.0);
        assert_eq!(c.num_operaciones(), 0);
    }

    #[test]
    fn raiz_y_potencia() {
        assert!((raiz_cuadrada(9.0) - 3.0).abs() < f64::EPSILON);
        assert!((potencia(2.0, 10.0) - 1024.0).abs() < f64::EPSILON);
    }
}
