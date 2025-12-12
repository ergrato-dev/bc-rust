// src/formas/rectangulo.rs
// Implementación de la forma Rectángulo

use super::Forma;

/// Representa un rectángulo definido por ancho y alto
#[derive(Debug, Clone, Copy)]
pub struct Rectangulo {
    pub ancho: f64,
    pub alto: f64,
}

impl Rectangulo {
    /// Crea un nuevo rectángulo con las dimensiones especificadas
    pub fn nuevo(ancho: f64, alto: f64) -> Self {
        Self { ancho, alto }
    }

    /// Crea un cuadrado (ancho = alto)
    pub fn cuadrado(lado: f64) -> Self {
        Self {
            ancho: lado,
            alto: lado,
        }
    }

    /// Verifica si es un cuadrado
    pub fn es_cuadrado(&self) -> bool {
        (self.ancho - self.alto).abs() < f64::EPSILON
    }

    /// Retorna la diagonal del rectángulo
    pub fn diagonal(&self) -> f64 {
        (self.ancho.powi(2) + self.alto.powi(2)).sqrt()
    }
}

impl Forma for Rectangulo {
    fn nombre(&self) -> &str {
        if self.es_cuadrado() {
            "Cuadrado"
        } else {
            "Rectángulo"
        }
    }

    fn es_valida(&self) -> bool {
        self.ancho > 0.0 && self.alto > 0.0 && self.ancho.is_finite() && self.alto.is_finite()
    }
}

impl Default for Rectangulo {
    fn default() -> Self {
        Self {
            ancho: 1.0,
            alto: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nuevo_rectangulo() {
        let r = Rectangulo::nuevo(4.0, 3.0);
        assert_eq!(r.ancho, 4.0);
        assert_eq!(r.alto, 3.0);
    }

    #[test]
    fn test_cuadrado() {
        let c = Rectangulo::cuadrado(5.0);
        assert_eq!(c.ancho, 5.0);
        assert_eq!(c.alto, 5.0);
        assert!(c.es_cuadrado());
    }

    #[test]
    fn test_no_es_cuadrado() {
        let r = Rectangulo::nuevo(4.0, 3.0);
        assert!(!r.es_cuadrado());
    }

    #[test]
    fn test_diagonal() {
        let r = Rectangulo::nuevo(3.0, 4.0);
        assert!((r.diagonal() - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_es_valida() {
        let r = Rectangulo::nuevo(4.0, 3.0);
        assert!(r.es_valida());
    }

    #[test]
    fn test_no_es_valida_cero() {
        let r = Rectangulo::nuevo(0.0, 3.0);
        assert!(!r.es_valida());
    }

    #[test]
    fn test_nombre_rectangulo() {
        let r = Rectangulo::nuevo(4.0, 3.0);
        assert_eq!(r.nombre(), "Rectángulo");
    }

    #[test]
    fn test_nombre_cuadrado() {
        let c = Rectangulo::cuadrado(5.0);
        assert_eq!(c.nombre(), "Cuadrado");
    }
}
