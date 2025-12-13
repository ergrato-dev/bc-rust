//! Bounded capacity container using const generics.
//!
//! # Example
//!
//! ```
//! use proyecto_contenedor::Bounded;
//!
//! let mut container: Bounded<i32, 3> = Bounded::new();
//! assert!(container.insert(1).is_ok());
//! assert!(container.insert(2).is_ok());
//! assert!(container.insert(3).is_ok());
//! assert!(container.insert(4).is_err()); // Full!
//! ```

use std::fmt::Debug;

/// Container with fixed maximum capacity defined at compile time.
///
/// `N` is the maximum capacity of the container.
pub struct Bounded<T, const N: usize> {
    elements: Vec<T>,
}

impl<T, const N: usize> Bounded<T, N> {
    /// Creates a new empty container with capacity N.
    ///
    /// # Example
    ///
    /// ```
    /// use proyecto_contenedor::Bounded;
    /// let container: Bounded<u8, 10> = Bounded::new();
    /// assert_eq!(container.capacity(), 10);
    /// ```
    pub fn new() -> Self {
        Bounded {
            elements: Vec::with_capacity(N),
        }
    }

    /// Attempts to insert an element into the container.
    ///
    /// Returns `Ok(())` if there is space, or `Err(value)` if full,
    /// returning the value that could not be inserted.
    ///
    /// # Example
    ///
    /// ```
    /// use proyecto_contenedor::Bounded;
    /// let mut container: Bounded<i32, 2> = Bounded::new();
    /// assert_eq!(container.insert(1), Ok(()));
    /// assert_eq!(container.insert(2), Ok(()));
    /// assert_eq!(container.insert(3), Err(3)); // Returns the value
    /// ```
    pub fn insert(&mut self, value: T) -> Result<(), T> {
        if self.elements.len() < N {
            self.elements.push(value);
            Ok(())
        } else {
            Err(value)
        }
    }

    /// Removes and returns the last inserted element.
    ///
    /// Returns `None` if the container is empty.
    pub fn remove(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// Returns the current number of elements.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns the maximum capacity of the container.
    pub fn capacity(&self) -> usize {
        N
    }

    /// Checks if the container is full.
    pub fn is_full(&self) -> bool {
        self.elements.len() >= N
    }

    /// Checks if the container is empty.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns a reference to the element at the given index.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.elements.get(index)
    }
}

impl<T, const N: usize> Default for Bounded<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug, const N: usize> Debug for Bounded<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bounded")
            .field("capacity", &N)
            .field("elements", &self.elements)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let container: Bounded<i32, 5> = Bounded::new();
        assert!(container.is_empty());
        assert_eq!(container.capacity(), 5);
    }

    #[test]
    fn test_insert_ok() {
        let mut container: Bounded<char, 3> = Bounded::new();
        assert_eq!(container.insert('a'), Ok(()));
        assert_eq!(container.insert('b'), Ok(()));
        assert_eq!(container.insert('c'), Ok(()));
        assert_eq!(container.len(), 3);
    }

    #[test]
    fn test_insert_full() {
        let mut container: Bounded<i32, 2> = Bounded::new();
        assert_eq!(container.insert(1), Ok(()));
        assert_eq!(container.insert(2), Ok(()));
        assert_eq!(container.insert(3), Err(3));
        assert_eq!(container.len(), 2);
    }

    #[test]
    fn test_remove() {
        let mut container: Bounded<i32, 3> = Bounded::new();
        container.insert(1).unwrap();
        container.insert(2).unwrap();

        assert_eq!(container.remove(), Some(2));
        assert_eq!(container.remove(), Some(1));
        assert_eq!(container.remove(), None);
    }

    #[test]
    fn test_is_full() {
        let mut container: Bounded<u8, 2> = Bounded::new();
        assert!(!container.is_full());

        container.insert(1).unwrap();
        assert!(!container.is_full());

        container.insert(2).unwrap();
        assert!(container.is_full());
    }

    #[test]
    fn test_get() {
        let mut container: Bounded<&str, 3> = Bounded::new();
        container.insert("a").unwrap();
        container.insert("b").unwrap();

        assert_eq!(container.get(0), Some(&"a"));
        assert_eq!(container.get(1), Some(&"b"));
        assert_eq!(container.get(2), None);
    }

    #[test]
    fn test_different_sizes() {
        let _small: Bounded<i32, 1> = Bounded::new();
        let _medium: Bounded<i32, 100> = Bounded::new();
        let _large: Bounded<i32, 1000> = Bounded::new();

        // Verify each has correct capacity
        assert_eq!(_small.capacity(), 1);
        assert_eq!(_medium.capacity(), 100);
        assert_eq!(_large.capacity(), 1000);
    }

    #[test]
    fn test_default() {
        let container: Bounded<String, 5> = Bounded::default();
        assert!(container.is_empty());
        assert_eq!(container.capacity(), 5);
    }
}
