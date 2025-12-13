// src/shapes/rectangle.rs
// Rectangle shape implementation

use super::Shape;

/// Represents a rectangle defined by width and height
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    /// Creates a new rectangle with the specified dimensions
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    /// Creates a square (width = height)
    pub fn square(side: f64) -> Self {
        Self {
            width: side,
            height: side,
        }
    }

    /// Checks if it's a square
    pub fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }

    /// Returns the diagonal of the rectangle
    pub fn diagonal(&self) -> f64 {
        (self.width.powi(2) + self.height.powi(2)).sqrt()
    }
}

impl Shape for Rectangle {
    fn name(&self) -> &str {
        if self.is_square() {
            "Square"
        } else {
            "Rectangle"
        }
    }

    fn is_valid(&self) -> bool {
        self.width > 0.0 && self.height > 0.0 && self.width.is_finite() && self.height.is_finite()
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_rectangle() {
        let r = Rectangle::new(4.0, 3.0);
        assert_eq!(r.width, 4.0);
        assert_eq!(r.height, 3.0);
    }

    #[test]
    fn test_square() {
        let s = Rectangle::square(5.0);
        assert_eq!(s.width, 5.0);
        assert_eq!(s.height, 5.0);
        assert!(s.is_square());
    }

    #[test]
    fn test_not_square() {
        let r = Rectangle::new(4.0, 3.0);
        assert!(!r.is_square());
    }

    #[test]
    fn test_diagonal() {
        let r = Rectangle::new(3.0, 4.0);
        assert!((r.diagonal() - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_is_valid() {
        let r = Rectangle::new(4.0, 3.0);
        assert!(r.is_valid());
    }

    #[test]
    fn test_not_valid_zero() {
        let r = Rectangle::new(0.0, 3.0);
        assert!(!r.is_valid());
    }

    #[test]
    fn test_name_rectangle() {
        let r = Rectangle::new(4.0, 3.0);
        assert_eq!(r.name(), "Rectangle");
    }

    #[test]
    fn test_name_square() {
        let s = Rectangle::square(5.0);
        assert_eq!(s.name(), "Square");
    }

    #[test]
    fn test_default() {
        let r = Rectangle::default();
        assert_eq!(r.width, 1.0);
        assert_eq!(r.height, 1.0);
    }
}
