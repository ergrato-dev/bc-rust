// src/shapes/triangle.rs
// Triangle shape implementation

use super::Shape;

/// Represents a triangle defined by its three sides
#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub side_a: f64,
    pub side_b: f64,
    pub side_c: f64,
}

impl Triangle {
    /// Creates a new triangle with the specified sides
    pub fn new(side_a: f64, side_b: f64, side_c: f64) -> Self {
        Self {
            side_a,
            side_b,
            side_c,
        }
    }

    /// Creates an equilateral triangle
    pub fn equilateral(side: f64) -> Self {
        Self {
            side_a: side,
            side_b: side,
            side_c: side,
        }
    }

    /// Creates an isosceles triangle
    pub fn isosceles(base: f64, equal_side: f64) -> Self {
        Self {
            side_a: base,
            side_b: equal_side,
            side_c: equal_side,
        }
    }

    /// Checks if it's equilateral
    pub fn is_equilateral(&self) -> bool {
        let epsilon = 0.0001;
        (self.side_a - self.side_b).abs() < epsilon && (self.side_b - self.side_c).abs() < epsilon
    }

    /// Checks if it's isosceles
    pub fn is_isosceles(&self) -> bool {
        let epsilon = 0.0001;
        (self.side_a - self.side_b).abs() < epsilon
            || (self.side_b - self.side_c).abs() < epsilon
            || (self.side_a - self.side_c).abs() < epsilon
    }

    /// Checks if it's scalene (all sides different)
    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    /// Checks the triangle inequality
    fn satisfies_triangle_inequality(&self) -> bool {
        self.side_a + self.side_b > self.side_c
            && self.side_b + self.side_c > self.side_a
            && self.side_a + self.side_c > self.side_b
    }

    /// Calculates the semi-perimeter (useful for Heron's formula)
    pub fn semi_perimeter(&self) -> f64 {
        (self.side_a + self.side_b + self.side_c) / 2.0
    }
}

impl Shape for Triangle {
    fn name(&self) -> &str {
        if self.is_equilateral() {
            "Equilateral Triangle"
        } else if self.is_isosceles() {
            "Isosceles Triangle"
        } else {
            "Scalene Triangle"
        }
    }

    fn is_valid(&self) -> bool {
        // All sides must be positive and finite
        let sides_positive = self.side_a > 0.0 && self.side_b > 0.0 && self.side_c > 0.0;

        let sides_finite =
            self.side_a.is_finite() && self.side_b.is_finite() && self.side_c.is_finite();

        sides_positive && sides_finite && self.satisfies_triangle_inequality()
    }
}

impl Default for Triangle {
    fn default() -> Self {
        // Unit equilateral triangle
        Self {
            side_a: 1.0,
            side_b: 1.0,
            side_c: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_triangle() {
        let t = Triangle::new(3.0, 4.0, 5.0);
        assert_eq!(t.side_a, 3.0);
        assert_eq!(t.side_b, 4.0);
        assert_eq!(t.side_c, 5.0);
    }

    #[test]
    fn test_equilateral() {
        let t = Triangle::equilateral(5.0);
        assert!(t.is_equilateral());
        assert!(t.is_isosceles()); // An equilateral is also isosceles
    }

    #[test]
    fn test_isosceles() {
        let t = Triangle::isosceles(4.0, 5.0);
        assert!(t.is_isosceles());
        assert!(!t.is_equilateral());
    }

    #[test]
    fn test_scalene() {
        let t = Triangle::new(3.0, 4.0, 5.0);
        assert!(t.is_scalene());
        assert!(!t.is_isosceles());
    }

    #[test]
    fn test_is_valid() {
        let t = Triangle::new(3.0, 4.0, 5.0);
        assert!(t.is_valid());
    }

    #[test]
    fn test_not_valid_triangle_inequality() {
        let t = Triangle::new(1.0, 2.0, 10.0);
        assert!(!t.is_valid());
    }

    #[test]
    fn test_not_valid_negative() {
        let t = Triangle::new(-1.0, 2.0, 3.0);
        assert!(!t.is_valid());
    }

    #[test]
    fn test_semi_perimeter() {
        let t = Triangle::new(3.0, 4.0, 5.0);
        assert_eq!(t.semi_perimeter(), 6.0);
    }

    #[test]
    fn test_name_equilateral() {
        let t = Triangle::equilateral(5.0);
        assert_eq!(t.name(), "Equilateral Triangle");
    }

    #[test]
    fn test_name_isosceles() {
        let t = Triangle::isosceles(4.0, 5.0);
        assert_eq!(t.name(), "Isosceles Triangle");
    }

    #[test]
    fn test_name_scalene() {
        let t = Triangle::new(3.0, 4.0, 5.0);
        assert_eq!(t.name(), "Scalene Triangle");
    }

    #[test]
    fn test_default() {
        let t = Triangle::default();
        assert!(t.is_equilateral());
    }
}
