//! # Proyecto: Sistema de Gestión de Inventario
//!
//! Este proyecto demuestra el uso de colecciones en Rust:
//! - `Vec<T>` para listas de productos y transacciones
//! - `String` para nombres, descripciones y categorías
//! - `HashMap<K, V>` para índices y búsquedas rápidas
//! - Iteradores para procesamiento de datos

mod producto;
mod transaccion;
mod inventario;
mod reportes;

pub use producto::Producto;
pub use transaccion::{Transaccion, TipoTransaccion};
pub use inventario::Inventario;
pub use reportes::GeneradorReportes;
