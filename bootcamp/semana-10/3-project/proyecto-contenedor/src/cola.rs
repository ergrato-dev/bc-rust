//! Cola genérica FIFO (First In, First Out)
//!
//! # Ejemplo
//!
//! ```
//! use proyecto_contenedor::Cola;
//!
//! let mut cola = Cola::new();
//! cola.encolar(1);
//! cola.encolar(2);
//! assert_eq!(cola.desencolar(), Some(1));
//! ```

use std::collections::VecDeque;
use std::fmt::Debug;

/// Cola genérica que implementa el patrón FIFO.
///
/// Los elementos se agregan al final y se remueven del frente.
pub struct Cola<T> {
    elementos: VecDeque<T>,
}

impl<T> Cola<T> {
    /// Crea una nueva cola vacía.
    ///
    /// # Ejemplo
    ///
    /// ```
    /// use proyecto_contenedor::Cola;
    /// let cola: Cola<i32> = Cola::new();
    /// assert!(cola.esta_vacia());
    /// ```
    pub fn new() -> Self {
        Cola {
            elementos: VecDeque::new(),
        }
    }

    /// Agrega un elemento al final de la cola.
    ///
    /// # Ejemplo
    ///
    /// ```
    /// use proyecto_contenedor::Cola;
    /// let mut cola = Cola::new();
    /// cola.encolar("hola");
    /// assert_eq!(cola.len(), 1);
    /// ```
    pub fn encolar(&mut self, valor: T) {
        self.elementos.push_back(valor);
    }

    /// Remueve y devuelve el elemento del frente de la cola.
    ///
    /// Devuelve `None` si la cola está vacía.
    ///
    /// # Ejemplo
    ///
    /// ```
    /// use proyecto_contenedor::Cola;
    /// let mut cola = Cola::new();
    /// cola.encolar(1);
    /// cola.encolar(2);
    /// assert_eq!(cola.desencolar(), Some(1));
    /// assert_eq!(cola.desencolar(), Some(2));
    /// assert_eq!(cola.desencolar(), None);
    /// ```
    pub fn desencolar(&mut self) -> Option<T> {
        self.elementos.pop_front()
    }

    /// Devuelve una referencia al elemento del frente sin removerlo.
    ///
    /// Devuelve `None` si la cola está vacía.
    pub fn frente(&self) -> Option<&T> {
        self.elementos.front()
    }

    /// Devuelve la cantidad de elementos en la cola.
    pub fn len(&self) -> usize {
        self.elementos.len()
    }

    /// Verifica si la cola está vacía.
    pub fn esta_vacia(&self) -> bool {
        self.elementos.is_empty()
    }
}

impl<T> Default for Cola<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> Debug for Cola<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cola")
            .field("elementos", &self.elementos)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cola: Cola<i32> = Cola::new();
        assert!(cola.esta_vacia());
        assert_eq!(cola.len(), 0);
    }

    #[test]
    fn test_encolar_desencolar() {
        let mut cola = Cola::new();
        cola.encolar(1);
        cola.encolar(2);
        cola.encolar(3);

        assert_eq!(cola.desencolar(), Some(1));
        assert_eq!(cola.desencolar(), Some(2));
        assert_eq!(cola.desencolar(), Some(3));
        assert_eq!(cola.desencolar(), None);
    }

    #[test]
    fn test_frente() {
        let mut cola = Cola::new();
        assert_eq!(cola.frente(), None);

        cola.encolar("a");
        cola.encolar("b");
        assert_eq!(cola.frente(), Some(&"a"));

        cola.desencolar();
        assert_eq!(cola.frente(), Some(&"b"));
    }

    #[test]
    fn test_len() {
        let mut cola = Cola::new();
        assert_eq!(cola.len(), 0);

        cola.encolar(1);
        assert_eq!(cola.len(), 1);

        cola.encolar(2);
        assert_eq!(cola.len(), 2);

        cola.desencolar();
        assert_eq!(cola.len(), 1);
    }

    #[test]
    fn test_fifo_order() {
        let mut cola = Cola::new();
        for i in 0..10 {
            cola.encolar(i);
        }

        for i in 0..10 {
            assert_eq!(cola.desencolar(), Some(i));
        }
    }

    #[test]
    fn test_default() {
        let cola: Cola<String> = Cola::default();
        assert!(cola.esta_vacia());
    }

    #[test]
    fn test_debug() {
        let mut cola = Cola::new();
        cola.encolar(1);
        let debug_str = format!("{:?}", cola);
        assert!(debug_str.contains("Cola"));
    }
}
