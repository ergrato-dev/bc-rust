//! # Proyecto Formas Geométricas
//!
//! Sistema polimórfico para trabajar con formas geométricas usando traits.
//!
//! ## Módulos
//!
//! - `traits`: Definición de traits (Shape, Drawable, Transformable)
//! - `shapes`: Implementación de formas (Circle, Rectangle, Square, Triangle)
//! - `canvas`: Utilidades para visualización
//!
//! ## Ejemplo
//!
//! ```rust
//! use project_formas::*;
//!
//! let circle = Circle::new(5.0);
//! let rectangle = Rectangle::new(10.0, 5.0);
//!
//! println!("Área del círculo: {:.2}", circle.area());
//! println!("Perímetro del rectángulo: {:.2}", rectangle.perimeter());
//! ```

pub mod traits;
pub mod shapes;
pub mod canvas;

// Re-exportar tipos principales
pub use traits::{Shape, Drawable, Transformable, Positionable, CompleteShape, ComparableShape};
pub use shapes::{Circle, Rectangle, Square, Triangle};
pub use canvas::{Canvas, print_shape, print_shapes, shape_with_largest_area, total_area};
