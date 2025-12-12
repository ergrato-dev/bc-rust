// src/ventas.rs
// Módulo principal de ventas - declara submódulos

pub mod carrito;
pub mod factura;

// Re-exportar items importantes
pub use carrito::Carrito;
pub use factura::Factura;
