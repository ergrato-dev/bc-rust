// src/formas/triangulo.rs
// Implementación de la forma Triángulo

use super::Forma;

/// Representa un triángulo definido por sus tres lados
#[derive(Debug, Clone, Copy)]
pub struct Triangulo {
    pub lado_a: f64,
    pub lado_b: f64,
    pub lado_c: f64,
}

impl Triangulo {
    /// Crea un nuevo triángulo con los lados especificados
    pub fn nuevo(lado_a: f64, lado_b: f64, lado_c: f64) -> Self {
        Self {
            lado_a,
            lado_b,
            lado_c,
        }
    }

    /// Crea un triángulo equilátero
    pub fn equilatero(lado: f64) -> Self {
        Self {
            lado_a: lado,
            lado_b: lado,
            lado_c: lado,
        }
    }

    /// Crea un triángulo isósceles
    pub fn isosceles(base: f64, lado_igual: f64) -> Self {
        Self {
            lado_a: base,
            lado_b: lado_igual,
            lado_c: lado_igual,
        }
    }

    /// Verifica si es equilátero
    pub fn es_equilatero(&self) -> bool {
        let epsilon = 0.0001;
        (self.lado_a - self.lado_b).abs() < epsilon && (self.lado_b - self.lado_c).abs() < epsilon
    }

    /// Verifica si es isósceles
    pub fn es_isosceles(&self) -> bool {
        let epsilon = 0.0001;
        (self.lado_a - self.lado_b).abs() < epsilon
            || (self.lado_b - self.lado_c).abs() < epsilon
            || (self.lado_a - self.lado_c).abs() < epsilon
    }

    /// Verifica si es escaleno (todos los lados diferentes)
    pub fn es_escaleno(&self) -> bool {
        !self.es_isosceles()
    }

    /// Verifica la desigualdad triangular
    fn cumple_desigualdad_triangular(&self) -> bool {
        self.lado_a + self.lado_b > self.lado_c
            && self.lado_b + self.lado_c > self.lado_a
            && self.lado_a + self.lado_c > self.lado_b
    }

    /// Calcula el semiperímetro (útil para fórmula de Herón)
    pub fn semiperimetro(&self) -> f64 {
        (self.lado_a + self.lado_b + self.lado_c) / 2.0
    }
}

impl Forma for Triangulo {
    fn nombre(&self) -> &str {
        if self.es_equilatero() {
            "Triángulo Equilátero"
        } else if self.es_isosceles() {
            "Triángulo Isósceles"
        } else {
            "Triángulo Escaleno"
        }
    }

    fn es_valida(&self) -> bool {
        // Todos los lados deben ser positivos y finitos
        let lados_positivos = self.lado_a > 0.0 && self.lado_b > 0.0 && self.lado_c > 0.0;

        let lados_finitos =
            self.lado_a.is_finite() && self.lado_b.is_finite() && self.lado_c.is_finite();

        lados_positivos && lados_finitos && self.cumple_desigualdad_triangular()
    }
}

impl Default for Triangulo {
    fn default() -> Self {
        // Triángulo equilátero unitario
        Self {
            lado_a: 1.0,
            lado_b: 1.0,
            lado_c: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nuevo_triangulo() {
        let t = Triangulo::nuevo(3.0, 4.0, 5.0);
        assert_eq!(t.lado_a, 3.0);
        assert_eq!(t.lado_b, 4.0);
        assert_eq!(t.lado_c, 5.0);
    }

    #[test]
    fn test_equilatero() {
        let t = Triangulo::equilatero(5.0);
        assert!(t.es_equilatero());
        assert!(t.es_isosceles()); // Un equilátero también es isósceles
    }

    #[test]
    fn test_isosceles() {
        let t = Triangulo::isosceles(4.0, 5.0);
        assert!(t.es_isosceles());
        assert!(!t.es_equilatero());
    }

    #[test]
    fn test_escaleno() {
        let t = Triangulo::nuevo(3.0, 4.0, 5.0);
        assert!(t.es_escaleno());
        assert!(!t.es_isosceles());
    }

    #[test]
    fn test_triangulo_valido() {
        let t = Triangulo::nuevo(3.0, 4.0, 5.0);
        assert!(t.es_valida());
    }

    #[test]
    fn test_triangulo_invalido_desigualdad() {
        // 1 + 2 = 3, no es mayor que 10
        let t = Triangulo::nuevo(1.0, 2.0, 10.0);
        assert!(!t.es_valida());
    }

    #[test]
    fn test_triangulo_invalido_negativo() {
        let t = Triangulo::nuevo(-1.0, 2.0, 3.0);
        assert!(!t.es_valida());
    }

    #[test]
    fn test_semiperimetro() {
        let t = Triangulo::nuevo(3.0, 4.0, 5.0);
        assert_eq!(t.semiperimetro(), 6.0);
    }

    #[test]
    fn test_nombre_equilatero() {
        let t = Triangulo::equilatero(5.0);
        assert_eq!(t.nombre(), "Triángulo Equilátero");
    }

    #[test]
    fn test_nombre_isosceles() {
        let t = Triangulo::isosceles(4.0, 5.0);
        assert_eq!(t.nombre(), "Triángulo Isósceles");
    }

    #[test]
    fn test_nombre_escaleno() {
        let t = Triangulo::nuevo(3.0, 4.0, 5.0);
        assert_eq!(t.nombre(), "Triángulo Escaleno");
    }
}
