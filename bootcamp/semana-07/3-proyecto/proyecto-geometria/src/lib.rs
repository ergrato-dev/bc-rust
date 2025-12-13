//! # Geometry Library
//!
//! A modular library for geometric calculations.
//!
//! ## Basic Usage
//!
//! ```
//! use proyecto_geometria::*;
//!
//! let circle = Circle::new(5.0);
//! let area = calculations::area::circle_area(&circle);
//! println!("Area: {}", area);
//! ```

pub mod calculations;
pub mod format;
pub mod shapes;

// Re-exports for simplified API
pub use shapes::circle::Circle;
pub use shapes::rectangle::Rectangle;
pub use shapes::triangle::Triangle;
pub use shapes::Shape;
