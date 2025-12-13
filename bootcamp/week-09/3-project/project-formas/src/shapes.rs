//! # Implementación de Formas Geométricas
//!
//! Implementa diferentes formas con los traits definidos.

use std::f64::consts::PI;
use std::fmt;
use crate::traits::{Shape, Drawable, Transformable, Positionable, ComparableShape};

// ============================================================================
// CÍRCULO
// ============================================================================

/// Representa un círculo
#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub radius: f64,
    x: f64,
    y: f64,
}

impl Circle {
    /// Crea un nuevo círculo con el radio dado
    pub fn new(radius: f64) -> Self {
        Self { radius: radius.abs(), x: 0.0, y: 0.0 }
    }
    
    /// Crea un círculo con posición
    pub fn with_position(radius: f64, x: f64, y: f64) -> Self {
        Self { radius: radius.abs(), x, y }
    }
    
    /// Obtiene el diámetro
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Círculo(r={:.2})", self.radius)
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
    
    fn name(&self) -> &str {
        "Círculo"
    }
}

impl Drawable for Circle {
    fn draw(&self) -> String {
        self.draw_with('*')
    }
    
    fn draw_with(&self, c: char) -> String {
        let size = (self.radius * 2.0).ceil() as i32;
        let mut result = String::new();
        
        for y in -size..=size {
            for x in -size..=size {
                let dist = ((x * x + y * y) as f64).sqrt();
                if dist <= self.radius {
                    result.push(c);
                } else {
                    result.push(' ');
                }
            }
            result.push('\n');
        }
        
        result
    }
}

impl Transformable for Circle {
    fn scale(&mut self, factor: f64) {
        self.radius *= factor.abs();
    }
}

impl Positionable for Circle {
    fn x(&self) -> f64 { self.x }
    fn y(&self) -> f64 { self.y }
    
    fn move_by(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

impl ComparableShape for Circle {}

// ============================================================================
// RECTÁNGULO
// ============================================================================

/// Representa un rectángulo
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    x: f64,
    y: f64,
}

impl Rectangle {
    /// Crea un nuevo rectángulo
    pub fn new(width: f64, height: f64) -> Self {
        Self { 
            width: width.abs(), 
            height: height.abs(),
            x: 0.0,
            y: 0.0,
        }
    }
    
    /// Crea un rectángulo con posición
    pub fn with_position(width: f64, height: f64, x: f64, y: f64) -> Self {
        Self { 
            width: width.abs(), 
            height: height.abs(),
            x,
            y,
        }
    }
    
    /// Verifica si es un cuadrado
    pub fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self::new(1.0, 1.0)
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectángulo({}x{})", self.width, self.height)
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    
    fn name(&self) -> &str {
        "Rectángulo"
    }
}

impl Drawable for Rectangle {
    fn draw(&self) -> String {
        self.draw_with('#')
    }
    
    fn draw_with(&self, c: char) -> String {
        let width = self.width.ceil() as usize;
        let height = self.height.ceil() as usize;
        let mut result = String::new();
        
        for _ in 0..height {
            for _ in 0..width {
                result.push(c);
            }
            result.push('\n');
        }
        
        result
    }
}

impl Transformable for Rectangle {
    fn scale(&mut self, factor: f64) {
        let factor = factor.abs();
        self.width *= factor;
        self.height *= factor;
    }
}

impl Positionable for Rectangle {
    fn x(&self) -> f64 { self.x }
    fn y(&self) -> f64 { self.y }
    
    fn move_by(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

impl ComparableShape for Rectangle {}

// ============================================================================
// CUADRADO
// ============================================================================

/// Representa un cuadrado (caso especial de rectángulo)
#[derive(Debug, Clone, PartialEq)]
pub struct Square {
    pub side: f64,
    x: f64,
    y: f64,
}

impl Square {
    /// Crea un nuevo cuadrado
    pub fn new(side: f64) -> Self {
        Self { side: side.abs(), x: 0.0, y: 0.0 }
    }
    
    /// Crea un cuadrado con posición
    pub fn with_position(side: f64, x: f64, y: f64) -> Self {
        Self { side: side.abs(), x, y }
    }
    
    /// Convierte a rectángulo
    pub fn to_rectangle(&self) -> Rectangle {
        Rectangle::with_position(self.side, self.side, self.x, self.y)
    }
}

impl Default for Square {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cuadrado(lado={})", self.side)
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
    
    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }
    
    fn name(&self) -> &str {
        "Cuadrado"
    }
}

impl Drawable for Square {
    fn draw(&self) -> String {
        self.draw_with('□')
    }
    
    fn draw_with(&self, c: char) -> String {
        let side = self.side.ceil() as usize;
        let mut result = String::new();
        
        for _ in 0..side {
            for _ in 0..side {
                result.push(c);
            }
            result.push('\n');
        }
        
        result
    }
}

impl Transformable for Square {
    fn scale(&mut self, factor: f64) {
        self.side *= factor.abs();
    }
}

impl Positionable for Square {
    fn x(&self) -> f64 { self.x }
    fn y(&self) -> f64 { self.y }
    
    fn move_by(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

impl ComparableShape for Square {}

// ============================================================================
// TRIÁNGULO
// ============================================================================

/// Representa un triángulo por sus tres lados
#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub side_a: f64,
    pub side_b: f64,
    pub side_c: f64,
    x: f64,
    y: f64,
}

impl Triangle {
    /// Crea un nuevo triángulo si los lados son válidos
    pub fn new(a: f64, b: f64, c: f64) -> Option<Self> {
        let (a, b, c) = (a.abs(), b.abs(), c.abs());
        
        // Verificar desigualdad triangular
        if a + b > c && b + c > a && a + c > b {
            Some(Self { 
                side_a: a, 
                side_b: b, 
                side_c: c,
                x: 0.0,
                y: 0.0,
            })
        } else {
            None
        }
    }
    
    /// Crea un triángulo equilátero
    pub fn equilateral(side: f64) -> Self {
        Self { 
            side_a: side.abs(), 
            side_b: side.abs(), 
            side_c: side.abs(),
            x: 0.0,
            y: 0.0,
        }
    }
    
    /// Crea un triángulo isósceles
    pub fn isosceles(base: f64, equal_side: f64) -> Option<Self> {
        Self::new(base, equal_side, equal_side)
    }
    
    /// Crea un triángulo rectángulo
    pub fn right_triangle(leg_a: f64, leg_b: f64) -> Self {
        let hypotenuse = (leg_a * leg_a + leg_b * leg_b).sqrt();
        Self { 
            side_a: leg_a.abs(), 
            side_b: leg_b.abs(), 
            side_c: hypotenuse,
            x: 0.0,
            y: 0.0,
        }
    }
    
    /// Verifica si es equilátero
    pub fn is_equilateral(&self) -> bool {
        let eps = f64::EPSILON;
        (self.side_a - self.side_b).abs() < eps && 
        (self.side_b - self.side_c).abs() < eps
    }
    
    /// Verifica si es isósceles
    pub fn is_isosceles(&self) -> bool {
        let eps = 1e-10;
        (self.side_a - self.side_b).abs() < eps ||
        (self.side_b - self.side_c).abs() < eps ||
        (self.side_a - self.side_c).abs() < eps
    }
    
    /// Verifica si es rectángulo (teorema de Pitágoras)
    pub fn is_right_triangle(&self) -> bool {
        let mut sides = [self.side_a, self.side_b, self.side_c];
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let eps = 1e-10;
        (sides[0] * sides[0] + sides[1] * sides[1] - sides[2] * sides[2]).abs() < eps
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self::equilateral(1.0)
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Triángulo({:.2}, {:.2}, {:.2})", 
            self.side_a, self.side_b, self.side_c)
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // Fórmula de Herón
        let s = self.perimeter() / 2.0;
        (s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c)).sqrt()
    }
    
    fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }
    
    fn name(&self) -> &str {
        "Triángulo"
    }
}

impl Drawable for Triangle {
    fn draw(&self) -> String {
        self.draw_with('^')
    }
    
    fn draw_with(&self, c: char) -> String {
        let height = ((self.side_a + self.side_b + self.side_c) / 3.0).ceil() as usize;
        let height = height.max(3);
        let mut result = String::new();
        
        for i in 0..height {
            let spaces = height - i - 1;
            let chars = 2 * i + 1;
            
            result.push_str(&" ".repeat(spaces));
            result.push_str(&c.to_string().repeat(chars));
            result.push('\n');
        }
        
        result
    }
}

impl Transformable for Triangle {
    fn scale(&mut self, factor: f64) {
        let factor = factor.abs();
        self.side_a *= factor;
        self.side_b *= factor;
        self.side_c *= factor;
    }
}

impl Positionable for Triangle {
    fn x(&self) -> f64 { self.x }
    fn y(&self) -> f64 { self.y }
    
    fn move_by(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

impl ComparableShape for Triangle {}

// ============================================================================
// CONVERSIONES: From/Into
// ============================================================================

/// Convertir Square a Rectangle
impl From<Square> for Rectangle {
    fn from(square: Square) -> Self {
        Rectangle::with_position(square.side, square.side, square.x, square.y)
    }
}

/// Convertir f64 (radius) a Circle
impl From<f64> for Circle {
    fn from(radius: f64) -> Self {
        Circle::new(radius)
    }
}

/// Convertir tupla (width, height) a Rectangle
impl From<(f64, f64)> for Rectangle {
    fn from((width, height): (f64, f64)) -> Self {
        Rectangle::new(width, height)
    }
}

/// Convertir f64 (side) a Square
impl From<f64> for Square {
    fn from(side: f64) -> Self {
        Square::new(side)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    // --- Tests Circle ---
    
    #[test]
    fn test_circle_new() {
        let c = Circle::new(5.0);
        assert!((c.radius - 5.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_circle_area() {
        let c = Circle::new(2.0);
        let expected_area = PI * 4.0;
        assert!((c.area() - expected_area).abs() < 1e-10);
    }
    
    #[test]
    fn test_circle_perimeter() {
        let c = Circle::new(1.0);
        let expected_perimeter = 2.0 * PI;
        assert!((c.perimeter() - expected_perimeter).abs() < 1e-10);
    }
    
    #[test]
    fn test_circle_display() {
        let c = Circle::new(3.0);
        assert!(c.to_string().contains("Círculo"));
    }
    
    #[test]
    fn test_circle_default() {
        let c = Circle::default();
        assert!((c.radius - 1.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_circle_scale() {
        let mut c = Circle::new(2.0);
        c.scale(2.0);
        assert!((c.radius - 4.0).abs() < f64::EPSILON);
    }
    
    // --- Tests Rectangle ---
    
    #[test]
    fn test_rectangle_new() {
        let r = Rectangle::new(10.0, 5.0);
        assert!((r.width - 10.0).abs() < f64::EPSILON);
        assert!((r.height - 5.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_rectangle_area() {
        let r = Rectangle::new(4.0, 3.0);
        assert!((r.area() - 12.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_rectangle_perimeter() {
        let r = Rectangle::new(4.0, 3.0);
        assert!((r.perimeter() - 14.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_rectangle_is_square() {
        let square = Rectangle::new(5.0, 5.0);
        let rectangle = Rectangle::new(5.0, 3.0);
        
        assert!(square.is_square());
        assert!(!rectangle.is_square());
    }
    
    #[test]
    fn test_rectangle_default() {
        let r = Rectangle::default();
        assert!((r.width - 1.0).abs() < f64::EPSILON);
        assert!((r.height - 1.0).abs() < f64::EPSILON);
    }
    
    // --- Tests Square ---
    
    #[test]
    fn test_square_new() {
        let c = Square::new(5.0);
        assert!((c.side - 5.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_square_area() {
        let c = Square::new(4.0);
        assert!((c.area() - 16.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_square_perimeter() {
        let c = Square::new(4.0);
        assert!((c.perimeter() - 16.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_square_to_rectangle() {
        let c = Square::new(5.0);
        let r = c.to_rectangle();
        assert!(r.is_square());
        assert!((r.width - 5.0).abs() < f64::EPSILON);
    }
    
    // --- Tests Triangle ---
    
    #[test]
    fn test_triangle_new_valid() {
        let t = Triangle::new(3.0, 4.0, 5.0);
        assert!(t.is_some());
    }
    
    #[test]
    fn test_triangle_new_invalid() {
        let t = Triangle::new(1.0, 2.0, 10.0);
        assert!(t.is_none());
    }
    
    #[test]
    fn test_triangle_equilateral() {
        let t = Triangle::equilateral(5.0);
        assert!(t.is_equilateral());
    }
    
    #[test]
    fn test_triangle_right_triangle() {
        let t = Triangle::right_triangle(3.0, 4.0);
        assert!(t.is_right_triangle());
        assert!((t.side_c - 5.0).abs() < 1e-10); // Hipotenusa 3-4-5
    }
    
    #[test]
    fn test_triangle_area_heron() {
        let t = Triangle::new(3.0, 4.0, 5.0).unwrap();
        // Triángulo 3-4-5 tiene área 6
        assert!((t.area() - 6.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_triangle_perimeter() {
        let t = Triangle::new(3.0, 4.0, 5.0).unwrap();
        assert!((t.perimeter() - 12.0).abs() < f64::EPSILON);
    }
    
    // --- Tests Traits ---
    
    #[test]
    fn test_trait_shape_name() {
        let c = Circle::new(1.0);
        let r = Rectangle::new(1.0, 1.0);
        let t = Triangle::equilateral(1.0);
        
        assert_eq!(c.name(), "Círculo");
        assert_eq!(r.name(), "Rectángulo");
        assert_eq!(t.name(), "Triángulo");
    }
    
    #[test]
    fn test_trait_area_greater_than() {
        let c = Circle::new(10.0);
        assert!(c.area_greater_than(100.0));
        assert!(!c.area_greater_than(1000.0));
    }
    
    #[test]
    fn test_trait_is_larger_than() {
        let large = Circle::new(10.0);
        let small = Circle::new(1.0);
        
        assert!(large.is_larger_than(&small));
        assert!(!small.is_larger_than(&large));
    }
    
    #[test]
    fn test_trait_drawable() {
        let c = Square::new(3.0);
        let drawing = c.draw();
        assert!(!drawing.is_empty());
    }
    
    #[test]
    fn test_trait_transformable_scaled() {
        let c = Circle::new(5.0);
        let c2 = c.scaled(2.0);
        
        assert!((c.radius - 5.0).abs() < f64::EPSILON); // Original sin cambiar
        assert!((c2.radius - 10.0).abs() < f64::EPSILON); // Copia escalada
    }
    
    #[test]
    fn test_trait_positionable() {
        let mut c = Circle::with_position(5.0, 10.0, 20.0);
        assert_eq!(c.position(), (10.0, 20.0));
        
        c.move_by(5.0, -10.0);
        assert_eq!(c.position(), (15.0, 10.0));
    }
    
    // --- Tests Conversions ---
    
    #[test]
    fn test_from_square_to_rectangle() {
        let c = Square::new(5.0);
        let r: Rectangle = c.into();
        assert!((r.width - 5.0).abs() < f64::EPSILON);
        assert!((r.height - 5.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_from_f64_to_circle() {
        let c: Circle = 3.0.into();
        assert!((c.radius - 3.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_from_tuple_to_rectangle() {
        let r: Rectangle = (10.0, 5.0).into();
        assert!((r.width - 10.0).abs() < f64::EPSILON);
        assert!((r.height - 5.0).abs() < f64::EPSILON);
    }
    
    // --- Tests Clone y PartialEq ---
    
    #[test]
    fn test_clone() {
        let c1 = Circle::new(5.0);
        let c2 = c1.clone();
        assert_eq!(c1, c2);
    }
    
    #[test]
    fn test_partial_eq() {
        let c1 = Circle::new(5.0);
        let c2 = Circle::new(5.0);
        let c3 = Circle::new(3.0);
        
        assert_eq!(c1, c2);
        assert_ne!(c1, c3);
    }
}
