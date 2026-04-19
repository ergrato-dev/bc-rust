// src/calculations/perimeter.rs
// Perimeter calculations for different shapes

use crate::shapes::circle::Circle;
use crate::shapes::rectangle::Rectangle;
use crate::shapes::triangle::Triangle;

/// Calculates the perimeter (circumference) of a circle: 2πr
pub fn circle_perimeter(circle: &Circle) -> f64 {
    2.0 * super::PI * circle.radius
}

/// Calculates the perimeter of a rectangle: 2(width + height)
pub fn rectangle_perimeter(rectangle: &Rectangle) -> f64 {
    2.0 * (rectangle.width + rectangle.height)
}

/// Calculates the perimeter of a triangle: a + b + c
pub fn triangle_perimeter(triangle: &Triangle) -> f64 {
    triangle.side_a + triangle.side_b + triangle.side_c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_perimeter_radius_1() {
        let c = Circle::new(1.0);
        let perimeter = circle_perimeter(&c);
        // 2π ≈ 6.2832
        assert!((perimeter - 2.0 * std::f64::consts::PI).abs() < 0.0001);
    }

    #[test]
    fn test_circle_perimeter_radius_5() {
        let c = Circle::new(5.0);
        let perimeter = circle_perimeter(&c);
        // 2π × 5 = 31.4159...
        assert!((perimeter - 31.4159).abs() < 0.001);
    }

    #[test]
    fn test_rectangle_perimeter() {
        let r = Rectangle::new(4.0, 3.0);
        assert_eq!(rectangle_perimeter(&r), 14.0);
    }

    #[test]
    fn test_square_perimeter() {
        let s = Rectangle::square(5.0);
        assert_eq!(rectangle_perimeter(&s), 20.0);
    }

    #[test]
    fn test_triangle_perimeter() {
        let t = Triangle::new(3.0, 4.0, 5.0);
        assert_eq!(triangle_perimeter(&t), 12.0);
    }

    #[test]
    fn test_equilateral_triangle_perimeter() {
        let t = Triangle::equilateral(5.0);
        assert_eq!(triangle_perimeter(&t), 15.0);
    }
}
