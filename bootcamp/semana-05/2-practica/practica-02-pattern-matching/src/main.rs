// Práctica 02: Pattern Matching
// Semana 05 - Enums y Pattern Matching

use std::f64::consts::PI;

// ============================================
// Ejercicio 1: Match Básico con Monedas
// ============================================

#[derive(Debug, Clone, Copy)]
enum Moneda {
    Peso,
    Dolar,
    Euro,
    Libra,
}

// TODO: Implementa la conversión a pesos
// Tasas: Peso = 1, Dolar = 850, Euro = 920, Libra = 1080
fn a_pesos(moneda: Moneda, cantidad: f64) -> f64 {
    todo!("Implementar con match")
}

// ============================================
// Ejercicio 2: Enums con Datos - Figuras
// ============================================

#[derive(Debug)]
enum Figura {
    Circulo(f64),                        // radio
    Rectangulo { ancho: f64, alto: f64 },
    Triangulo(f64, f64),                 // base, altura
}

// TODO: Calcula el área de cualquier figura
fn calcular_area(figura: &Figura) -> f64 {
    todo!("Implementar extrayendo datos con match")
}

// TODO: Retorna el nombre de la figura
fn nombre_figura(figura: &Figura) -> &str {
    todo!("Implementar con match")
}

// ============================================
// Ejercicio 3: Patrones Avanzados
// ============================================

// TODO: Usa guards y rangos para clasificar
fn clasificar_numero(n: i32) -> &'static str {
    todo!("Implementar con match, guards y rangos")
}

// ============================================
// Ejercicio 4: Destructuring de Tuplas
// ============================================

// TODO: Clasifica un punto (x, y)
fn clasificar_punto(punto: (i32, i32)) -> &'static str {
    // Retorna:
    // - "origen" si (0, 0)
    // - "eje x" si y == 0
    // - "eje y" si x == 0
    // - "cuadrante I" si x > 0 && y > 0
    // - "cuadrante II" si x < 0 && y > 0
    // - "cuadrante III" si x < 0 && y < 0
    // - "cuadrante IV" si x > 0 && y < 0
    todo!("Implementar con match y destructuring")
}

fn main() {
    // Prueba monedas
    println!("100 dólares = {} pesos", a_pesos(Moneda::Dolar, 100.0));
    
    // Prueba figuras
    let circulo = Figura::Circulo(5.0);
    let rectangulo = Figura::Rectangulo { ancho: 10.0, alto: 5.0 };
    
    println!("{}: área = {:.2}", nombre_figura(&circulo), calcular_area(&circulo));
    println!("{}: área = {:.2}", nombre_figura(&rectangulo), calcular_area(&rectangulo));
    
    // Prueba clasificación
    println!("50 es: {}", clasificar_numero(50));
    println!("-5 es: {}", clasificar_numero(-5));
    
    // Prueba puntos
    println!("(3, 4) está en: {}", clasificar_punto((3, 4)));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests Monedas
    #[test]
    fn test_peso_a_pesos() {
        assert_eq!(a_pesos(Moneda::Peso, 100.0), 100.0);
    }

    #[test]
    fn test_dolar_a_pesos() {
        assert_eq!(a_pesos(Moneda::Dolar, 1.0), 850.0);
    }

    // Tests Figuras
    #[test]
    fn test_area_circulo() {
        let c = Figura::Circulo(1.0);
        assert!((calcular_area(&c) - PI).abs() < 0.001);
    }

    #[test]
    fn test_area_rectangulo() {
        let r = Figura::Rectangulo { ancho: 4.0, alto: 5.0 };
        assert_eq!(calcular_area(&r), 20.0);
    }

    #[test]
    fn test_area_triangulo() {
        let t = Figura::Triangulo(6.0, 4.0);
        assert_eq!(calcular_area(&t), 12.0);
    }

    // Tests Clasificación
    #[test]
    fn test_clasificar_cero() {
        assert_eq!(clasificar_numero(0), "cero");
    }

    #[test]
    fn test_clasificar_pequeno() {
        assert_eq!(clasificar_numero(5), "pequeño");
    }

    #[test]
    fn test_clasificar_mediano() {
        assert_eq!(clasificar_numero(50), "mediano");
    }

    #[test]
    fn test_clasificar_grande() {
        assert_eq!(clasificar_numero(200), "grande");
    }

    #[test]
    fn test_clasificar_negativo() {
        assert_eq!(clasificar_numero(-10), "negativo");
    }

    // Tests Puntos
    #[test]
    fn test_punto_origen() {
        assert_eq!(clasificar_punto((0, 0)), "origen");
    }

    #[test]
    fn test_punto_cuadrante_i() {
        assert_eq!(clasificar_punto((3, 4)), "cuadrante I");
    }

    #[test]
    fn test_punto_eje_x() {
        assert_eq!(clasificar_punto((5, 0)), "eje x");
    }
}
