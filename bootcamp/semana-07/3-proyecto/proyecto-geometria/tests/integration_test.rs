// tests/integration_test.rs
// Tests de integración para la biblioteca de geometría

use proyecto_geometria::calculos::{area, perimetro};
use proyecto_geometria::formato;
use proyecto_geometria::{Circulo, Forma, Rectangulo, Triangulo};

#[test]
fn test_circulo_completo() {
    let circulo = Circulo::nuevo(5.0);

    assert!(circulo.es_valida());
    assert_eq!(circulo.nombre(), "Círculo");

    let area_calc = area::area_circulo(&circulo);
    let perim_calc = perimetro::perimetro_circulo(&circulo);

    // π × 5² ≈ 78.54
    assert!((area_calc - 78.5398).abs() < 0.001);
    // 2π × 5 ≈ 31.42
    assert!((perim_calc - 31.4159).abs() < 0.001);
}

#[test]
fn test_rectangulo_completo() {
    let rect = Rectangulo::nuevo(4.0, 3.0);

    assert!(rect.es_valida());
    assert_eq!(rect.nombre(), "Rectángulo");

    assert_eq!(area::area_rectangulo(&rect), 12.0);
    assert_eq!(perimetro::perimetro_rectangulo(&rect), 14.0);
}

#[test]
fn test_cuadrado_es_rectangulo() {
    let cuadrado = Rectangulo::cuadrado(5.0);

    assert!(cuadrado.es_valida());
    assert!(cuadrado.es_cuadrado());
    assert_eq!(cuadrado.nombre(), "Cuadrado");

    assert_eq!(area::area_rectangulo(&cuadrado), 25.0);
    assert_eq!(perimetro::perimetro_rectangulo(&cuadrado), 20.0);
}

#[test]
fn test_triangulo_completo() {
    let triangulo = Triangulo::nuevo(3.0, 4.0, 5.0);

    assert!(triangulo.es_valida());
    assert_eq!(triangulo.nombre(), "Triángulo Escaleno");

    let area_calc = area::area_triangulo(&triangulo);
    assert!((area_calc - 6.0).abs() < 0.0001);

    assert_eq!(perimetro::perimetro_triangulo(&triangulo), 12.0);
}

#[test]
fn test_triangulo_equilatero() {
    let tri = Triangulo::equilatero(6.0);

    assert!(tri.es_valida());
    assert!(tri.es_equilatero());
    assert_eq!(tri.nombre(), "Triángulo Equilátero");

    assert_eq!(perimetro::perimetro_triangulo(&tri), 18.0);
}

#[test]
fn test_triangulo_invalido_desigualdad() {
    let tri = Triangulo::nuevo(1.0, 2.0, 10.0);
    assert!(!tri.es_valida());
}

#[test]
fn test_circulo_invalido_negativo() {
    let circulo = Circulo::nuevo(-5.0);
    assert!(!circulo.es_valida());
}

#[test]
fn test_rectangulo_invalido_cero() {
    let rect = Rectangulo::nuevo(0.0, 5.0);
    assert!(!rect.es_valida());
}

#[test]
fn test_formato_resultado() {
    let resultado = formato::formatear_resultado("Test", 10.0, 20.0);
    assert!(resultado.contains("Test"));
    assert!(resultado.contains("10.0"));
    assert!(resultado.contains("20.0"));
}

#[test]
fn test_formato_tabla() {
    let datos = vec![("Círculo", 78.54, 31.42), ("Rectángulo", 12.0, 14.0)];

    let tabla = formato::formatear_tabla(&datos);

    assert!(tabla.contains("Círculo"));
    assert!(tabla.contains("Rectángulo"));
    assert!(tabla.contains("Área"));
    assert!(tabla.contains("Perímetro"));
}

#[test]
fn test_multiples_formas() {
    let formas: Vec<Box<dyn Forma>> = vec![
        Box::new(Circulo::nuevo(1.0)),
        Box::new(Rectangulo::nuevo(2.0, 3.0)),
        Box::new(Triangulo::equilatero(4.0)),
    ];

    for forma in &formas {
        assert!(forma.es_valida());
        assert!(!forma.nombre().is_empty());
    }
}
