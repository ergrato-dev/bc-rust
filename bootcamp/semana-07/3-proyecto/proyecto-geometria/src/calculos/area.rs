// src/calculos/area.rs
// Cálculos de área para las diferentes formas

use crate::formas::circulo::Circulo;
use crate::formas::rectangulo::Rectangulo;
use crate::formas::triangulo::Triangulo;

/// Calcula el área de un círculo: π × r²
pub fn area_circulo(circulo: &Circulo) -> f64 {
    super::PI * circulo.radio.powi(2)
}

/// Calcula el área de un rectángulo: ancho × alto
pub fn area_rectangulo(rectangulo: &Rectangulo) -> f64 {
    rectangulo.ancho * rectangulo.alto
}

/// Calcula el área de un triángulo usando la fórmula de Herón
/// A = √(s(s-a)(s-b)(s-c)) donde s es el semiperímetro
pub fn area_triangulo(triangulo: &Triangulo) -> f64 {
    let s = triangulo.semiperimetro();
    let a = triangulo.lado_a;
    let b = triangulo.lado_b;
    let c = triangulo.lado_c;

    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_circulo_radio_1() {
        let c = Circulo::nuevo(1.0);
        let area = area_circulo(&c);
        assert!((area - std::f64::consts::PI).abs() < 0.0001);
    }

    #[test]
    fn test_area_circulo_radio_5() {
        let c = Circulo::nuevo(5.0);
        let area = area_circulo(&c);
        // π × 5² = 78.5398...
        assert!((area - 78.5398).abs() < 0.001);
    }

    #[test]
    fn test_area_rectangulo() {
        let r = Rectangulo::nuevo(4.0, 3.0);
        assert_eq!(area_rectangulo(&r), 12.0);
    }

    #[test]
    fn test_area_cuadrado() {
        let c = Rectangulo::cuadrado(5.0);
        assert_eq!(area_rectangulo(&c), 25.0);
    }

    #[test]
    fn test_area_triangulo_3_4_5() {
        // Triángulo rectángulo 3-4-5 tiene área = (3×4)/2 = 6
        let t = Triangulo::nuevo(3.0, 4.0, 5.0);
        let area = area_triangulo(&t);
        assert!((area - 6.0).abs() < 0.0001);
    }

    #[test]
    fn test_area_triangulo_equilatero() {
        // Área de equilátero con lado a = (√3/4) × a²
        let t = Triangulo::equilatero(2.0);
        let area = area_triangulo(&t);
        let esperada = (3.0_f64.sqrt() / 4.0) * 4.0; // √3 ≈ 1.732
        assert!((area - esperada).abs() < 0.0001);
    }
}
