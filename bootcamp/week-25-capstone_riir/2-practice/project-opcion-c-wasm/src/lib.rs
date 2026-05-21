//! # capstone-opcion-c: Motor de estadísticas compilado a WASM
//!
//! Librería de cálculo estadístico implementada en Rust y compilada a WASM
//! para uso en navegadores y Node.js.
//!
//! ## Uso desde JavaScript
//!
//! ```javascript
//! const { Estadisticas } = await import('./pkg/capstone_opcion_c.js');
//! const stats = new Estadisticas([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
//! console.log(stats.media());    // 5.5
//! console.log(stats.mediana());  // 5.5
//! console.log(stats.desviacion_std()); // ~3.03
//! ```
#![deny(missing_docs)]

use wasm_bindgen::prelude::*;

/// Motor de cálculo estadístico accesible desde JavaScript.
#[wasm_bindgen]
pub struct Estadisticas {
    datos: Vec<f64>,
}

#[wasm_bindgen]
impl Estadisticas {
    /// Crea un motor estadístico con los datos proporcionados.
    ///
    /// Los datos se copian internamente y se ordenan para el cálculo de mediana.
    #[wasm_bindgen(constructor)]
    pub fn new(datos: Vec<f64>) -> Estadisticas {
        Estadisticas { datos }
    }

    /// Retorna el número de elementos.
    pub fn n(&self) -> usize {
        self.datos.len()
    }

    /// Calcula la media aritmética.
    pub fn media(&self) -> f64 {
        if self.datos.is_empty() {
            return f64::NAN;
        }
        self.datos.iter().sum::<f64>() / self.datos.len() as f64
    }

    /// Calcula la mediana.
    pub fn mediana(&self) -> f64 {
        if self.datos.is_empty() {
            return f64::NAN;
        }
        let mut ordenados = self.datos.clone();
        ordenados.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let n = ordenados.len();
        if n % 2 == 0 {
            (ordenados[n / 2 - 1] + ordenados[n / 2]) / 2.0
        } else {
            ordenados[n / 2]
        }
    }

    /// Calcula la varianza poblacional.
    pub fn varianza(&self) -> f64 {
        if self.datos.is_empty() {
            return f64::NAN;
        }
        let media = self.media();
        self.datos.iter().map(|x| (x - media).powi(2)).sum::<f64>() / self.datos.len() as f64
    }

    /// Calcula la desviación estándar poblacional.
    pub fn desviacion_std(&self) -> f64 {
        self.varianza().sqrt()
    }

    /// Retorna el valor mínimo.
    pub fn minimo(&self) -> f64 {
        self.datos.iter().cloned().fold(f64::INFINITY, f64::min)
    }

    /// Retorna el valor máximo.
    pub fn maximo(&self) -> f64 {
        self.datos.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    }

    /// Retorna el rango (máximo − mínimo).
    pub fn rango(&self) -> f64 {
        self.maximo() - self.minimo()
    }

    /// Agrega un nuevo dato al conjunto.
    pub fn agregar(&mut self, valor: f64) {
        self.datos.push(valor);
    }
}

/// Calcula el coeficiente de correlación de Pearson entre dos conjuntos de datos.
///
/// Retorna `NaN` si los conjuntos tienen diferente longitud o están vacíos.
#[wasm_bindgen]
pub fn correlacion_pearson(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.is_empty() {
        return f64::NAN;
    }
    let n = x.len() as f64;
    let media_x = x.iter().sum::<f64>() / n;
    let media_y = y.iter().sum::<f64>() / n;
    let cov: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| (xi - media_x) * (yi - media_y)).sum::<f64>() / n;
    let std_x = (x.iter().map(|xi| (xi - media_x).powi(2)).sum::<f64>() / n).sqrt();
    let std_y = (y.iter().map(|yi| (yi - media_y).powi(2)).sum::<f64>() / n).sqrt();
    if std_x == 0.0 || std_y == 0.0 { f64::NAN } else { cov / (std_x * std_y) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn media_correcta() {
        let s = Estadisticas::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert!((s.media() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn mediana_par() {
        let s = Estadisticas::new(vec![1.0, 2.0, 3.0, 4.0]);
        assert!((s.mediana() - 2.5).abs() < 1e-10);
    }

    #[test]
    fn desviacion_std_conocida() {
        let s = Estadisticas::new(vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]);
        assert!((s.desviacion_std() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn correlacion_perfecta() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        let y = [2.0, 4.0, 6.0, 8.0, 10.0];
        assert!((correlacion_pearson(&x, &y) - 1.0).abs() < 1e-10);
    }
}
