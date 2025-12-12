// src/calculos/perimetro.rs
// Cálculos de perímetro para las diferentes formas

use crate::formas::circulo::Circulo;
use crate::formas::rectangulo::Rectangulo;
use crate::formas::triangulo::Triangulo;

/// Calcula el perímetro (circunferencia) de un círculo: 2πr
pub fn perimetro_circulo(circulo: &Circulo) -> f64 {
    2.0 * super::PI * circulo.radio
}

/// Calcula el perímetro de un rectángulo: 2(ancho + alto)
pub fn perimetro_rectangulo(rectangulo: &Rectangulo) -> f64 {
    2.0 * (rectangulo.ancho + rectangulo.alto)
}

/// Calcula el perímetro de un triángulo: a + b + c
pub fn perimetro_triangulo(triangulo: &Triangulo) -> f64 {
    triangulo.lado_a + triangulo.lado_b + triangulo.lado_c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perimetro_circulo_radio_1() {
        let c = Circulo::nuevo(1.0);
        let perimetro = perimetro_circulo(&c);
        // 2π ≈ 6.2832
        assert!((perimetro - 2.0 * std::f64::consts::PI).abs() < 0.0001);
    }

    #[test]
    fn test_perimetro_circulo_radio_5() {
        let c = Circulo::nuevo(5.0);
        let perimetro = perimetro_circulo(&c);
        // 2π × 5 = 31.4159...
        assert!((perimetro - 31.4159).abs() < 0.001);
    }

    #[test]
    fn test_perimetro_rectangulo() {
        let r = Rectangulo::nuevo(4.0, 3.0);
        assert_eq!(perimetro_rectangulo(&r), 14.0);
    }

    #[test]
    fn test_perimetro_cuadrado() {
        let c = Rectangulo::cuadrado(5.0);
        assert_eq!(perimetro_rectangulo(&c), 20.0);
    }

    #[test]
    fn test_perimetro_triangulo() {
        let t = Triangulo::nuevo(3.0, 4.0, 5.0);
        assert_eq!(perimetro_triangulo(&t), 12.0);
    }

    #[test]
    fn test_perimetro_triangulo_equilatero() {
        let t = Triangulo::equilatero(5.0);
        assert_eq!(perimetro_triangulo(&t), 15.0);
    }
}
