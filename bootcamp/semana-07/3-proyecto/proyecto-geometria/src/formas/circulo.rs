// src/formas/circulo.rs
// Implementación de la forma Círculo

use super::Forma;

/// Representa un círculo definido por su radio
#[derive(Debug, Clone, Copy)]
pub struct Circulo {
    pub radio: f64,
}

impl Circulo {
    /// Crea un nuevo círculo con el radio especificado
    pub fn nuevo(radio: f64) -> Self {
        Self { radio }
    }

    /// Retorna el diámetro del círculo
    pub fn diametro(&self) -> f64 {
        self.radio * 2.0
    }
}

impl Forma for Circulo {
    fn nombre(&self) -> &str {
        "Círculo"
    }

    fn es_valida(&self) -> bool {
        self.radio > 0.0 && self.radio.is_finite()
    }
}

impl Default for Circulo {
    fn default() -> Self {
        Self { radio: 1.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nuevo_circulo() {
        let c = Circulo::nuevo(5.0);
        assert_eq!(c.radio, 5.0);
    }

    #[test]
    fn test_diametro() {
        let c = Circulo::nuevo(3.0);
        assert_eq!(c.diametro(), 6.0);
    }

    #[test]
    fn test_es_valida_positivo() {
        let c = Circulo::nuevo(5.0);
        assert!(c.es_valida());
    }

    #[test]
    fn test_es_valida_cero() {
        let c = Circulo::nuevo(0.0);
        assert!(!c.es_valida());
    }

    #[test]
    fn test_es_valida_negativo() {
        let c = Circulo::nuevo(-1.0);
        assert!(!c.es_valida());
    }

    #[test]
    fn test_nombre() {
        let c = Circulo::nuevo(1.0);
        assert_eq!(c.nombre(), "Círculo");
    }

    #[test]
    fn test_default() {
        let c = Circulo::default();
        assert_eq!(c.radio, 1.0);
    }
}
