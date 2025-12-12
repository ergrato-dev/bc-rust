// src/formas.rs
// Módulo principal de formas geométricas

pub mod circulo;
pub mod rectangulo;
pub mod triangulo;

/// Trait que define el comportamiento común de todas las formas
pub trait Forma {
    /// Retorna el nombre de la forma
    fn nombre(&self) -> &str;

    /// Verifica si la forma tiene dimensiones válidas
    fn es_valida(&self) -> bool;
}
