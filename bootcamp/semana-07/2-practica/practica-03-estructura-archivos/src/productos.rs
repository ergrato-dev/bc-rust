// src/productos.rs
// Módulo principal de productos - declara submódulos

pub mod inventario;
pub mod precio;

// Re-exportar items importantes para uso más simple
pub use inventario::Producto;
pub use precio::calcular_precio_final;
