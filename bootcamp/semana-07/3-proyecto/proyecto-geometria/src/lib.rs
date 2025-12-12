//! # Biblioteca de Geometría
//!
//! Una biblioteca modular para cálculos geométricos.
//!
//! ## Uso básico
//!
//! ```
//! use proyecto_geometria::*;
//!
//! let circulo = Circulo::nuevo(5.0);
//! let area = calculos::area::area_circulo(&circulo);
//! println!("Área: {}", area);
//! ```

pub mod calculos;
pub mod formato;
pub mod formas;

// Re-exports para API simplificada
pub use formas::circulo::Circulo;
pub use formas::rectangulo::Rectangulo;
pub use formas::triangulo::Triangulo;
pub use formas::Forma;
