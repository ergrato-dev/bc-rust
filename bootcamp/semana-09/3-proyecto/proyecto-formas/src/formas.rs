//! # Implementación de Formas Geométricas
//!
//! Implementa diferentes formas con los traits definidos.

use std::f64::consts::PI;
use std::fmt;
use crate::traits::{Forma, Dibujable, Transformable, Posicionable};

// ============================================================================
// CÍRCULO
// ============================================================================

/// Representa un círculo
#[derive(Debug, Clone, PartialEq)]
pub struct Circulo {
    pub radio: f64,
    x: f64,
    y: f64,
}

impl Circulo {
    /// Crea un nuevo círculo con el radio dado
    pub fn new(radio: f64) -> Self {
        Self { radio: radio.abs(), x: 0.0, y: 0.0 }
    }
    
    /// Crea un círculo con posición
    pub fn con_posicion(radio: f64, x: f64, y: f64) -> Self {
        Self { radio: radio.abs(), x, y }
    }
    
    /// Obtiene el diámetro
    pub fn diametro(&self) -> f64 {
        self.radio * 2.0
    }
}

impl Default for Circulo {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl fmt::Display for Circulo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Círculo(r={:.2})", self.radio)
    }
}

impl Forma for Circulo {
    fn area(&self) -> f64 {
        PI * self.radio * self.radio
    }
    
    fn perimetro(&self) -> f64 {
        2.0 * PI * self.radio
    }
    
    fn nombre(&self) -> &str {
        "Círculo"
    }
}

impl Dibujable for Circulo {
    fn dibujar(&self) -> String {
        self.dibujar_con('*')
    }
    
    fn dibujar_con(&self, c: char) -> String {
        let size = (self.radio * 2.0).ceil() as i32;
        let mut resultado = String::new();
        
        for y in -size..=size {
            for x in -size..=size {
                let dist = ((x * x + y * y) as f64).sqrt();
                if dist <= self.radio {
                    resultado.push(c);
                } else {
                    resultado.push(' ');
                }
            }
            resultado.push('\n');
        }
        
        resultado
    }
}

impl Transformable for Circulo {
    fn escalar(&mut self, factor: f64) {
        self.radio *= factor.abs();
    }
}

impl Posicionable for Circulo {
    fn x(&self) -> f64 { self.x }
    fn y(&self) -> f64 { self.y }
    
    fn mover(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

// ============================================================================
// RECTÁNGULO
// ============================================================================

/// Representa un rectángulo
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangulo {
    pub ancho: f64,
    pub alto: f64,
    x: f64,
    y: f64,
}

impl Rectangulo {
    /// Crea un nuevo rectángulo
    pub fn new(ancho: f64, alto: f64) -> Self {
        Self { 
            ancho: ancho.abs(), 
            alto: alto.abs(),
            x: 0.0,
            y: 0.0,
        }
    }
    
    /// Crea un rectángulo con posición
    pub fn con_posicion(ancho: f64, alto: f64, x: f64, y: f64) -> Self {
        Self { 
            ancho: ancho.abs(), 
            alto: alto.abs(),
            x,
            y,
        }
    }
    
    /// Verifica si es un cuadrado
    pub fn es_cuadrado(&self) -> bool {
        (self.ancho - self.alto).abs() < f64::EPSILON
    }
}

impl Default for Rectangulo {
    fn default() -> Self {
        Self::new(1.0, 1.0)
    }
}

impl fmt::Display for Rectangulo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectángulo({}x{})", self.ancho, self.alto)
    }
}

impl Forma for Rectangulo {
    fn area(&self) -> f64 {
        self.ancho * self.alto
    }
    
    fn perimetro(&self) -> f64 {
        2.0 * (self.ancho + self.alto)
    }
    
    fn nombre(&self) -> &str {
        "Rectángulo"
    }
}

impl Dibujable for Rectangulo {
    fn dibujar(&self) -> String {
        self.dibujar_con('#')
    }
    
    fn dibujar_con(&self, c: char) -> String {
        let ancho = self.ancho.ceil() as usize;
        let alto = self.alto.ceil() as usize;
        let mut resultado = String::new();
        
        for _ in 0..alto {
            for _ in 0..ancho {
                resultado.push(c);
            }
            resultado.push('\n');
        }
        
        resultado
    }
}

impl Transformable for Rectangulo {
    fn escalar(&mut self, factor: f64) {
        let factor = factor.abs();
        self.ancho *= factor;
        self.alto *= factor;
    }
}

impl Posicionable for Rectangulo {
    fn x(&self) -> f64 { self.x }
    fn y(&self) -> f64 { self.y }
    
    fn mover(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

// ============================================================================
// CUADRADO
// ============================================================================

/// Representa un cuadrado (caso especial de rectángulo)
#[derive(Debug, Clone, PartialEq)]
pub struct Cuadrado {
    pub lado: f64,
    x: f64,
    y: f64,
}

impl Cuadrado {
    /// Crea un nuevo cuadrado
    pub fn new(lado: f64) -> Self {
        Self { lado: lado.abs(), x: 0.0, y: 0.0 }
    }
    
    /// Crea un cuadrado con posición
    pub fn con_posicion(lado: f64, x: f64, y: f64) -> Self {
        Self { lado: lado.abs(), x, y }
    }
    
    /// Convierte a rectángulo
    pub fn a_rectangulo(&self) -> Rectangulo {
        Rectangulo::con_posicion(self.lado, self.lado, self.x, self.y)
    }
}

impl Default for Cuadrado {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl fmt::Display for Cuadrado {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cuadrado(lado={})", self.lado)
    }
}

impl Forma for Cuadrado {
    fn area(&self) -> f64 {
        self.lado * self.lado
    }
    
    fn perimetro(&self) -> f64 {
        4.0 * self.lado
    }
    
    fn nombre(&self) -> &str {
        "Cuadrado"
    }
}

impl Dibujable for Cuadrado {
    fn dibujar(&self) -> String {
        self.dibujar_con('□')
    }
    
    fn dibujar_con(&self, c: char) -> String {
        let lado = self.lado.ceil() as usize;
        let mut resultado = String::new();
        
        for _ in 0..lado {
            for _ in 0..lado {
                resultado.push(c);
            }
            resultado.push('\n');
        }
        
        resultado
    }
}

impl Transformable for Cuadrado {
    fn escalar(&mut self, factor: f64) {
        self.lado *= factor.abs();
    }
}

impl Posicionable for Cuadrado {
    fn x(&self) -> f64 { self.x }
    fn y(&self) -> f64 { self.y }
    
    fn mover(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

// ============================================================================
// TRIÁNGULO
// ============================================================================

/// Representa un triángulo por sus tres lados
#[derive(Debug, Clone, PartialEq)]
pub struct Triangulo {
    pub lado_a: f64,
    pub lado_b: f64,
    pub lado_c: f64,
    x: f64,
    y: f64,
}

impl Triangulo {
    /// Crea un nuevo triángulo si los lados son válidos
    pub fn new(a: f64, b: f64, c: f64) -> Option<Self> {
        let (a, b, c) = (a.abs(), b.abs(), c.abs());
        
        // Verificar desigualdad triangular
        if a + b > c && b + c > a && a + c > b {
            Some(Self { 
                lado_a: a, 
                lado_b: b, 
                lado_c: c,
                x: 0.0,
                y: 0.0,
            })
        } else {
            None
        }
    }
    
    /// Crea un triángulo equilátero
    pub fn equilatero(lado: f64) -> Self {
        Self { 
            lado_a: lado.abs(), 
            lado_b: lado.abs(), 
            lado_c: lado.abs(),
            x: 0.0,
            y: 0.0,
        }
    }
    
    /// Crea un triángulo isósceles
    pub fn isosceles(base: f64, lado_igual: f64) -> Option<Self> {
        Self::new(base, lado_igual, lado_igual)
    }
    
    /// Crea un triángulo rectángulo
    pub fn rectangulo(cateto_a: f64, cateto_b: f64) -> Self {
        let hipotenusa = (cateto_a * cateto_a + cateto_b * cateto_b).sqrt();
        Self { 
            lado_a: cateto_a.abs(), 
            lado_b: cateto_b.abs(), 
            lado_c: hipotenusa,
            x: 0.0,
            y: 0.0,
        }
    }
    
    /// Verifica si es equilátero
    pub fn es_equilatero(&self) -> bool {
        let eps = f64::EPSILON;
        (self.lado_a - self.lado_b).abs() < eps && 
        (self.lado_b - self.lado_c).abs() < eps
    }
    
    /// Verifica si es isósceles
    pub fn es_isosceles(&self) -> bool {
        let eps = 1e-10;
        (self.lado_a - self.lado_b).abs() < eps ||
        (self.lado_b - self.lado_c).abs() < eps ||
        (self.lado_a - self.lado_c).abs() < eps
    }
    
    /// Verifica si es rectángulo (teorema de Pitágoras)
    pub fn es_rectangulo(&self) -> bool {
        let mut lados = [self.lado_a, self.lado_b, self.lado_c];
        lados.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let eps = 1e-10;
        (lados[0] * lados[0] + lados[1] * lados[1] - lados[2] * lados[2]).abs() < eps
    }
}

impl Default for Triangulo {
    fn default() -> Self {
        Self::equilatero(1.0)
    }
}

impl fmt::Display for Triangulo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Triángulo({:.2}, {:.2}, {:.2})", 
            self.lado_a, self.lado_b, self.lado_c)
    }
}

impl Forma for Triangulo {
    fn area(&self) -> f64 {
        // Fórmula de Herón
        let s = self.perimetro() / 2.0;
        (s * (s - self.lado_a) * (s - self.lado_b) * (s - self.lado_c)).sqrt()
    }
    
    fn perimetro(&self) -> f64 {
        self.lado_a + self.lado_b + self.lado_c
    }
    
    fn nombre(&self) -> &str {
        "Triángulo"
    }
}

impl Dibujable for Triangulo {
    fn dibujar(&self) -> String {
        self.dibujar_con('^')
    }
    
    fn dibujar_con(&self, c: char) -> String {
        let altura = ((self.lado_a + self.lado_b + self.lado_c) / 3.0).ceil() as usize;
        let altura = altura.max(3);
        let mut resultado = String::new();
        
        for i in 0..altura {
            let espacios = altura - i - 1;
            let chars = 2 * i + 1;
            
            resultado.push_str(&" ".repeat(espacios));
            resultado.push_str(&c.to_string().repeat(chars));
            resultado.push('\n');
        }
        
        resultado
    }
}

impl Transformable for Triangulo {
    fn escalar(&mut self, factor: f64) {
        let factor = factor.abs();
        self.lado_a *= factor;
        self.lado_b *= factor;
        self.lado_c *= factor;
    }
}

impl Posicionable for Triangulo {
    fn x(&self) -> f64 { self.x }
    fn y(&self) -> f64 { self.y }
    
    fn mover(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

// ============================================================================
// CONVERSIONES: From/Into
// ============================================================================

/// Convertir Cuadrado a Rectángulo
impl From<Cuadrado> for Rectangulo {
    fn from(cuadrado: Cuadrado) -> Self {
        Rectangulo::con_posicion(cuadrado.lado, cuadrado.lado, cuadrado.x, cuadrado.y)
    }
}

/// Convertir f64 (radio) a Círculo
impl From<f64> for Circulo {
    fn from(radio: f64) -> Self {
        Circulo::new(radio)
    }
}

/// Convertir tupla (ancho, alto) a Rectángulo
impl From<(f64, f64)> for Rectangulo {
    fn from((ancho, alto): (f64, f64)) -> Self {
        Rectangulo::new(ancho, alto)
    }
}

/// Convertir f64 (lado) a Cuadrado
impl From<f64> for Cuadrado {
    fn from(lado: f64) -> Self {
        Cuadrado::new(lado)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    // --- Tests Círculo ---
    
    #[test]
    fn test_circulo_new() {
        let c = Circulo::new(5.0);
        assert!((c.radio - 5.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_circulo_area() {
        let c = Circulo::new(2.0);
        let area_esperada = PI * 4.0;
        assert!((c.area() - area_esperada).abs() < 1e-10);
    }
    
    #[test]
    fn test_circulo_perimetro() {
        let c = Circulo::new(1.0);
        let perimetro_esperado = 2.0 * PI;
        assert!((c.perimetro() - perimetro_esperado).abs() < 1e-10);
    }
    
    #[test]
    fn test_circulo_display() {
        let c = Circulo::new(3.0);
        assert!(c.to_string().contains("Círculo"));
    }
    
    #[test]
    fn test_circulo_default() {
        let c = Circulo::default();
        assert!((c.radio - 1.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_circulo_escalar() {
        let mut c = Circulo::new(2.0);
        c.escalar(2.0);
        assert!((c.radio - 4.0).abs() < f64::EPSILON);
    }
    
    // --- Tests Rectángulo ---
    
    #[test]
    fn test_rectangulo_new() {
        let r = Rectangulo::new(10.0, 5.0);
        assert!((r.ancho - 10.0).abs() < f64::EPSILON);
        assert!((r.alto - 5.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_rectangulo_area() {
        let r = Rectangulo::new(4.0, 3.0);
        assert!((r.area() - 12.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_rectangulo_perimetro() {
        let r = Rectangulo::new(4.0, 3.0);
        assert!((r.perimetro() - 14.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_rectangulo_es_cuadrado() {
        let cuadrado = Rectangulo::new(5.0, 5.0);
        let rectangulo = Rectangulo::new(5.0, 3.0);
        
        assert!(cuadrado.es_cuadrado());
        assert!(!rectangulo.es_cuadrado());
    }
    
    #[test]
    fn test_rectangulo_default() {
        let r = Rectangulo::default();
        assert!((r.ancho - 1.0).abs() < f64::EPSILON);
        assert!((r.alto - 1.0).abs() < f64::EPSILON);
    }
    
    // --- Tests Cuadrado ---
    
    #[test]
    fn test_cuadrado_new() {
        let c = Cuadrado::new(5.0);
        assert!((c.lado - 5.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_cuadrado_area() {
        let c = Cuadrado::new(4.0);
        assert!((c.area() - 16.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_cuadrado_perimetro() {
        let c = Cuadrado::new(4.0);
        assert!((c.perimetro() - 16.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_cuadrado_a_rectangulo() {
        let c = Cuadrado::new(5.0);
        let r = c.a_rectangulo();
        assert!(r.es_cuadrado());
        assert!((r.ancho - 5.0).abs() < f64::EPSILON);
    }
    
    // --- Tests Triángulo ---
    
    #[test]
    fn test_triangulo_new_valido() {
        let t = Triangulo::new(3.0, 4.0, 5.0);
        assert!(t.is_some());
    }
    
    #[test]
    fn test_triangulo_new_invalido() {
        let t = Triangulo::new(1.0, 2.0, 10.0);
        assert!(t.is_none());
    }
    
    #[test]
    fn test_triangulo_equilatero() {
        let t = Triangulo::equilatero(5.0);
        assert!(t.es_equilatero());
    }
    
    #[test]
    fn test_triangulo_rectangulo() {
        let t = Triangulo::rectangulo(3.0, 4.0);
        assert!(t.es_rectangulo());
        assert!((t.lado_c - 5.0).abs() < 1e-10); // Hipotenusa 3-4-5
    }
    
    #[test]
    fn test_triangulo_area_heron() {
        let t = Triangulo::new(3.0, 4.0, 5.0).unwrap();
        // Triángulo 3-4-5 tiene área 6
        assert!((t.area() - 6.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_triangulo_perimetro() {
        let t = Triangulo::new(3.0, 4.0, 5.0).unwrap();
        assert!((t.perimetro() - 12.0).abs() < f64::EPSILON);
    }
    
    // --- Tests Traits ---
    
    #[test]
    fn test_trait_forma_nombre() {
        let c = Circulo::new(1.0);
        let r = Rectangulo::new(1.0, 1.0);
        let t = Triangulo::equilatero(1.0);
        
        assert_eq!(c.nombre(), "Círculo");
        assert_eq!(r.nombre(), "Rectángulo");
        assert_eq!(t.nombre(), "Triángulo");
    }
    
    #[test]
    fn test_trait_area_mayor_que() {
        let c = Circulo::new(10.0);
        assert!(c.area_mayor_que(100.0));
        assert!(!c.area_mayor_que(1000.0));
    }
    
    #[test]
    fn test_trait_es_mas_grande_que() {
        let grande = Circulo::new(10.0);
        let pequeno = Circulo::new(1.0);
        
        assert!(grande.es_mas_grande_que(&pequeno));
        assert!(!pequeno.es_mas_grande_que(&grande));
    }
    
    #[test]
    fn test_trait_dibujable() {
        let c = Cuadrado::new(3.0);
        let dibujo = c.dibujar();
        assert!(!dibujo.is_empty());
    }
    
    #[test]
    fn test_trait_transformable_escalada() {
        let c = Circulo::new(5.0);
        let c2 = c.escalada(2.0);
        
        assert!((c.radio - 5.0).abs() < f64::EPSILON); // Original sin cambiar
        assert!((c2.radio - 10.0).abs() < f64::EPSILON); // Copia escalada
    }
    
    #[test]
    fn test_trait_posicionable() {
        let mut c = Circulo::con_posicion(5.0, 10.0, 20.0);
        assert_eq!(c.posicion(), (10.0, 20.0));
        
        c.mover(5.0, -10.0);
        assert_eq!(c.posicion(), (15.0, 10.0));
    }
    
    // --- Tests Conversiones ---
    
    #[test]
    fn test_from_cuadrado_a_rectangulo() {
        let c = Cuadrado::new(5.0);
        let r: Rectangulo = c.into();
        assert!((r.ancho - 5.0).abs() < f64::EPSILON);
        assert!((r.alto - 5.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_from_f64_a_circulo() {
        let c: Circulo = 3.0.into();
        assert!((c.radio - 3.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_from_tupla_a_rectangulo() {
        let r: Rectangulo = (10.0, 5.0).into();
        assert!((r.ancho - 10.0).abs() < f64::EPSILON);
        assert!((r.alto - 5.0).abs() < f64::EPSILON);
    }
    
    // --- Tests Clone y PartialEq ---
    
    #[test]
    fn test_clone() {
        let c1 = Circulo::new(5.0);
        let c2 = c1.clone();
        assert_eq!(c1, c2);
    }
    
    #[test]
    fn test_partial_eq() {
        let c1 = Circulo::new(5.0);
        let c2 = Circulo::new(5.0);
        let c3 = Circulo::new(3.0);
        
        assert_eq!(c1, c2);
        assert_ne!(c1, c3);
    }
}
