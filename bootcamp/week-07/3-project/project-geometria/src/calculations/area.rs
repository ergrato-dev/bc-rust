// src/calculations/area.rs
// Area calculations for different shapes

use crate::shapes::circle::Circle;
use crate::shapes::rectangle::Rectangle;
use crate::shapes::triangle::Triangle;

/// Calculates the area of a circle: π × r²
pub fn circle_area(circle: &Circle) -> f64 {
    super::PI * circle.radius.powi(2)
}

/// Calculates the area of a rectangle: width × height
pub fn rectangle_area(rectangle: &Rectangle) -> f64 {
    rectangle.width * rectangle.height
}

/// Calculates the area of a triangle using Heron's formula
/// A = √(s(s-a)(s-b)(s-c)) where s is the semi-perimeter
pub fn triangle_area(triangle: &Triangle) -> f64 {
    let s = triangle.semi_perimeter();
    let a = triangle.side_a;
    let b = triangle.side_b;
    let c = triangle.side_c;

    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area_radius_1() {
        let c = Circle::new(1.0);
        let area = circle_area(&c);
        assert!((area - std::f64::consts::PI).abs() < 0.0001);
    }

    #[test]
    fn test_circle_area_radius_5() {
        let c = Circle::new(5.0);
        let area = circle_area(&c);
        // π × 5² = 78.5398...
        assert!((area - 78.5398).abs() < 0.001);
    }

    #[test]
    fn test_rectangle_area() {
        let r = Rectangle::new(4.0, 3.0);
        assert_eq!(rectangle_area(&r), 12.0);
    }

    #[test]
    fn test_square_area() {
        let s = Rectangle::square(5.0);
        assert_eq!(rectangle_area(&s), 25.0);
    }

    #[test]
    fn test_triangle_area_3_4_5() {
        // Right triangle 3-4-5 has area = (3×4)/2 = 6
        let t = Triangle::new(3.0, 4.0, 5.0);
        let area = triangle_area(&t);
        assert!((area - 6.0).abs() < 0.0001);
    }

    #[test]
    fn test_equilateral_triangle_area() {
        // Area of equilateral with side a = (√3/4) × a²
        let t = Triangle::equilateral(2.0);
        let area = triangle_area(&t);
        let expected = (3.0_f64.sqrt() / 4.0) * 4.0; // √3 ≈ 1.732
        assert!((area - expected).abs() < 0.0001);
    }
}
