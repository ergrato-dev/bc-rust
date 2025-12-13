//! # Proyecto Formas Geométricas
//!
//! Sistema polimórfico para trabajar con formas geométricas usando traits.
//!
//! ## Módulos
//!
//! - `traits`: Definición de traits (Forma, Dibujable, Transformable)
//! - `formas`: Implementación de formas (Círculo, Rectángulo, Cuadrado, Triángulo)
//! - `canvas`: Utilidades para visualización
//!
//! ## Ejemplo
//!
//! ```rust
//! use proyecto_formas::*;
//!
//! let circulo = Circulo::new(5.0);
//! let rectangulo = Rectangulo::new(10.0, 5.0);
//!
//! println!("Área del círculo: {:.2}", circulo.area());
//! println!("Perímetro del rectángulo: {:.2}", rectangulo.perimetro());
//! ```

pub mod traits;
pub mod formas;
pub mod canvas;

// Re-exportar tipos principales
pub use traits::{Forma, Dibujable, Transformable, Posicionable, FormaCompleta, FormaComparable};
pub use formas::{Circulo, Rectangulo, Cuadrado, Triangulo};
pub use canvas::{Canvas, imprimir_forma, imprimir_formas, forma_mayor_area, area_total};
