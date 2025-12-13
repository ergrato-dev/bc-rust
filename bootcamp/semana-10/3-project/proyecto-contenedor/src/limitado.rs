//! Contenedor con capacidad limitada usando const generics.
//!
//! # Ejemplo
//!
//! ```
//! use proyecto_contenedor::Limitado;
//!
//! let mut contenedor: Limitado<i32, 3> = Limitado::new();
//! assert!(contenedor.insertar(1).is_ok());
//! assert!(contenedor.insertar(2).is_ok());
//! assert!(contenedor.insertar(3).is_ok());
//! assert!(contenedor.insertar(4).is_err()); // Lleno!
//! ```

use std::fmt::Debug;

/// Contenedor con capacidad máxima fija definida en tiempo de compilación.
///
/// `N` es la capacidad máxima del contenedor.
pub struct Limitado<T, const N: usize> {
    elementos: Vec<T>,
}

impl<T, const N: usize> Limitado<T, N> {
    /// Crea un nuevo contenedor vacío con capacidad N.
    ///
    /// # Ejemplo
    ///
    /// ```
    /// use proyecto_contenedor::Limitado;
    /// let contenedor: Limitado<u8, 10> = Limitado::new();
    /// assert_eq!(contenedor.capacidad(), 10);
    /// ```
    pub fn new() -> Self {
        Limitado {
            elementos: Vec::with_capacity(N),
        }
    }

    /// Intenta insertar un elemento en el contenedor.
    ///
    /// Devuelve `Ok(())` si hay espacio, o `Err(valor)` si está lleno,
    /// devolviendo el valor que no pudo ser insertado.
    ///
    /// # Ejemplo
    ///
    /// ```
    /// use proyecto_contenedor::Limitado;
    /// let mut contenedor: Limitado<i32, 2> = Limitado::new();
    /// assert_eq!(contenedor.insertar(1), Ok(()));
    /// assert_eq!(contenedor.insertar(2), Ok(()));
    /// assert_eq!(contenedor.insertar(3), Err(3)); // Devuelve el valor
    /// ```
    pub fn insertar(&mut self, valor: T) -> Result<(), T> {
        if self.elementos.len() < N {
            self.elementos.push(valor);
            Ok(())
        } else {
            Err(valor)
        }
    }

    /// Remueve y devuelve el último elemento insertado.
    ///
    /// Devuelve `None` si el contenedor está vacío.
    pub fn remover(&mut self) -> Option<T> {
        self.elementos.pop()
    }

    /// Devuelve la cantidad actual de elementos.
    pub fn len(&self) -> usize {
        self.elementos.len()
    }

    /// Devuelve la capacidad máxima del contenedor.
    pub fn capacidad(&self) -> usize {
        N
    }

    /// Verifica si el contenedor está lleno.
    pub fn esta_lleno(&self) -> bool {
        self.elementos.len() >= N
    }

    /// Verifica si el contenedor está vacío.
    pub fn esta_vacio(&self) -> bool {
        self.elementos.is_empty()
    }

    /// Devuelve una referencia al elemento en el índice dado.
    pub fn obtener(&self, indice: usize) -> Option<&T> {
        self.elementos.get(indice)
    }
}

impl<T, const N: usize> Default for Limitado<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug, const N: usize> Debug for Limitado<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Limitado")
            .field("capacidad", &N)
            .field("elementos", &self.elementos)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let contenedor: Limitado<i32, 5> = Limitado::new();
        assert!(contenedor.esta_vacio());
        assert_eq!(contenedor.capacidad(), 5);
    }

    #[test]
    fn test_insertar_ok() {
        let mut contenedor: Limitado<char, 3> = Limitado::new();
        assert_eq!(contenedor.insertar('a'), Ok(()));
        assert_eq!(contenedor.insertar('b'), Ok(()));
        assert_eq!(contenedor.insertar('c'), Ok(()));
        assert_eq!(contenedor.len(), 3);
    }

    #[test]
    fn test_insertar_lleno() {
        let mut contenedor: Limitado<i32, 2> = Limitado::new();
        assert_eq!(contenedor.insertar(1), Ok(()));
        assert_eq!(contenedor.insertar(2), Ok(()));
        assert_eq!(contenedor.insertar(3), Err(3));
        assert_eq!(contenedor.len(), 2);
    }

    #[test]
    fn test_remover() {
        let mut contenedor: Limitado<i32, 3> = Limitado::new();
        contenedor.insertar(1).unwrap();
        contenedor.insertar(2).unwrap();

        assert_eq!(contenedor.remover(), Some(2));
        assert_eq!(contenedor.remover(), Some(1));
        assert_eq!(contenedor.remover(), None);
    }

    #[test]
    fn test_esta_lleno() {
        let mut contenedor: Limitado<u8, 2> = Limitado::new();
        assert!(!contenedor.esta_lleno());

        contenedor.insertar(1).unwrap();
        assert!(!contenedor.esta_lleno());

        contenedor.insertar(2).unwrap();
        assert!(contenedor.esta_lleno());
    }

    #[test]
    fn test_obtener() {
        let mut contenedor: Limitado<&str, 3> = Limitado::new();
        contenedor.insertar("a").unwrap();
        contenedor.insertar("b").unwrap();

        assert_eq!(contenedor.obtener(0), Some(&"a"));
        assert_eq!(contenedor.obtener(1), Some(&"b"));
        assert_eq!(contenedor.obtener(2), None);
    }

    #[test]
    fn test_diferentes_tamaños() {
        let _pequeño: Limitado<i32, 1> = Limitado::new();
        let _mediano: Limitado<i32, 100> = Limitado::new();
        let _grande: Limitado<i32, 1000> = Limitado::new();

        // Verificamos que cada uno tiene su capacidad correcta
        assert_eq!(_pequeño.capacidad(), 1);
        assert_eq!(_mediano.capacidad(), 100);
        assert_eq!(_grande.capacidad(), 1000);
    }

    #[test]
    fn test_default() {
        let contenedor: Limitado<String, 5> = Limitado::default();
        assert!(contenedor.esta_vacio());
        assert_eq!(contenedor.capacidad(), 5);
    }
}
