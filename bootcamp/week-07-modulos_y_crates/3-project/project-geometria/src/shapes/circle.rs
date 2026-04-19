// src/shapes/circle.rs
// Circle shape implementation

use super::Shape;

/// Represents a circle defined by its radius
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub radius: f64,
}

impl Circle {
    /// Creates a new circle with the specified radius
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }

    /// Returns the diameter of the circle
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
}

impl Shape for Circle {
    fn name(&self) -> &str {
        "Circle"
    }

    fn is_valid(&self) -> bool {
        self.radius > 0.0 && self.radius.is_finite()
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self { radius: 1.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_circle() {
        let c = Circle::new(5.0);
        assert_eq!(c.radius, 5.0);
    }

    #[test]
    fn test_diameter() {
        let c = Circle::new(3.0);
        assert_eq!(c.diameter(), 6.0);
    }

    #[test]
    fn test_is_valid_positive() {
        let c = Circle::new(5.0);
        assert!(c.is_valid());
    }

    #[test]
    fn test_is_valid_zero() {
        let c = Circle::new(0.0);
        assert!(!c.is_valid());
    }

    #[test]
    fn test_is_valid_negative() {
        let c = Circle::new(-1.0);
        assert!(!c.is_valid());
    }

    #[test]
    fn test_name() {
        let c = Circle::new(1.0);
        assert_eq!(c.name(), "Circle");
    }

    #[test]
    fn test_default() {
        let c = Circle::default();
        assert_eq!(c.radius, 1.0);
    }
}
