//! # Biblioteca de Contenedores Genéricos
//!
//! Esta biblioteca proporciona estructuras de datos genéricas
//! reutilizables para diferentes casos de uso.
//!
//! ## Contenedores disponibles
//!
//! - [`Cola`] - Cola FIFO (First In, First Out)
//! - [`Deque`] - Cola de doble extremo
//! - [`Limitado`] - Contenedor con capacidad fija
//! - [`Cache`] - Caché LRU simplificado

pub mod cache;
pub mod cola;
pub mod deque;
pub mod limitado;

pub use cache::Cache;
pub use cola::Cola;
pub use deque::Deque;
pub use limitado::Limitado;
