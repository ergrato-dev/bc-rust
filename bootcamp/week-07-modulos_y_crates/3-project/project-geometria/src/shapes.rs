// src/shapes.rs
// Main module for geometric shapes

pub mod circle;
pub mod rectangle;
pub mod triangle;

/// Trait that defines common behavior for all shapes
pub trait Shape {
    /// Returns the name of the shape
    fn name(&self) -> &str;

    /// Checks if the shape has valid dimensions
    fn is_valid(&self) -> bool;
}
