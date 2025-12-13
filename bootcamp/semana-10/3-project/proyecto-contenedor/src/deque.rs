//! Deque genérica (Double-Ended Queue)
//!
//! Permite insertar y remover elementos de ambos extremos.
//!
//! # Ejemplo
//!
//! ```
//! use proyecto_contenedor::Deque;
//!
//! let mut deque = Deque::new();
//! deque.push_frente(1);
//! deque.push_atras(2);
//! assert_eq!(deque.pop_frente(), Some(1));
//! ```

use std::collections::VecDeque;
use std::fmt::Debug;

/// Cola de doble extremo genérica.
///
/// Soporta operaciones eficientes en ambos extremos.
pub struct Deque<T> {
    elementos: VecDeque<T>,
}

impl<T> Deque<T> {
    /// Crea una nueva deque vacía.
    pub fn new() -> Self {
        Deque {
            elementos: VecDeque::new(),
        }
    }

    /// Agrega un elemento al frente de la deque.
    pub fn push_frente(&mut self, valor: T) {
        self.elementos.push_front(valor);
    }

    /// Agrega un elemento al final de la deque.
    pub fn push_atras(&mut self, valor: T) {
        self.elementos.push_back(valor);
    }

    /// Remueve y devuelve el elemento del frente.
    pub fn pop_frente(&mut self) -> Option<T> {
        self.elementos.pop_front()
    }

    /// Remueve y devuelve el elemento del final.
    pub fn pop_atras(&mut self) -> Option<T> {
        self.elementos.pop_back()
    }

    /// Devuelve una referencia al elemento del frente.
    pub fn frente(&self) -> Option<&T> {
        self.elementos.front()
    }

    /// Devuelve una referencia al elemento del final.
    pub fn atras(&self) -> Option<&T> {
        self.elementos.back()
    }

    /// Devuelve la cantidad de elementos.
    pub fn len(&self) -> usize {
        self.elementos.len()
    }

    /// Verifica si la deque está vacía.
    pub fn esta_vacia(&self) -> bool {
        self.elementos.is_empty()
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> Debug for Deque<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Deque")
            .field("elementos", &self.elementos)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let deque: Deque<i32> = Deque::new();
        assert!(deque.esta_vacia());
    }

    #[test]
    fn test_push_pop_frente() {
        let mut deque = Deque::new();
        deque.push_frente(1);
        deque.push_frente(2);
        deque.push_frente(3);

        assert_eq!(deque.pop_frente(), Some(3));
        assert_eq!(deque.pop_frente(), Some(2));
        assert_eq!(deque.pop_frente(), Some(1));
    }

    #[test]
    fn test_push_pop_atras() {
        let mut deque = Deque::new();
        deque.push_atras(1);
        deque.push_atras(2);
        deque.push_atras(3);

        assert_eq!(deque.pop_atras(), Some(3));
        assert_eq!(deque.pop_atras(), Some(2));
        assert_eq!(deque.pop_atras(), Some(1));
    }

    #[test]
    fn test_mixed_operations() {
        let mut deque = Deque::new();
        deque.push_atras(2);
        deque.push_frente(1);
        deque.push_atras(3);

        // Deque: [1, 2, 3]
        assert_eq!(deque.frente(), Some(&1));
        assert_eq!(deque.atras(), Some(&3));

        assert_eq!(deque.pop_frente(), Some(1));
        assert_eq!(deque.pop_atras(), Some(3));
        assert_eq!(deque.pop_frente(), Some(2));
    }

    #[test]
    fn test_frente_atras() {
        let mut deque = Deque::new();
        assert_eq!(deque.frente(), None);
        assert_eq!(deque.atras(), None);

        deque.push_atras(1);
        assert_eq!(deque.frente(), Some(&1));
        assert_eq!(deque.atras(), Some(&1));

        deque.push_atras(2);
        assert_eq!(deque.frente(), Some(&1));
        assert_eq!(deque.atras(), Some(&2));
    }

    #[test]
    fn test_len() {
        let mut deque = Deque::new();
        assert_eq!(deque.len(), 0);

        deque.push_frente(1);
        deque.push_atras(2);
        assert_eq!(deque.len(), 2);

        deque.pop_frente();
        assert_eq!(deque.len(), 1);
    }

    #[test]
    fn test_default() {
        let deque: Deque<char> = Deque::default();
        assert!(deque.esta_vacia());
    }
}
