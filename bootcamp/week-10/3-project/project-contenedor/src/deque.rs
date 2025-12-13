//! Generic Deque (Double-Ended Queue)
//!
//! Allows inserting and removing elements from both ends.
//!
//! # Example
//!
//! ```
//! use proyecto_contenedor::Deque;
//!
//! let mut deque = Deque::new();
//! deque.push_front(1);
//! deque.push_back(2);
//! assert_eq!(deque.pop_front(), Some(1));
//! ```

use std::collections::VecDeque;
use std::fmt::Debug;

/// Generic double-ended queue.
///
/// Supports efficient operations at both ends.
pub struct Deque<T> {
    elements: VecDeque<T>,
}

impl<T> Deque<T> {
    /// Creates a new empty deque.
    pub fn new() -> Self {
        Deque {
            elements: VecDeque::new(),
        }
    }

    /// Adds an element to the front of the deque.
    pub fn push_front(&mut self, value: T) {
        self.elements.push_front(value);
    }

    /// Adds an element to the back of the deque.
    pub fn push_back(&mut self, value: T) {
        self.elements.push_back(value);
    }

    /// Removes and returns the front element.
    pub fn pop_front(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    /// Removes and returns the back element.
    pub fn pop_back(&mut self) -> Option<T> {
        self.elements.pop_back()
    }

    /// Returns a reference to the front element.
    pub fn front(&self) -> Option<&T> {
        self.elements.front()
    }

    /// Returns a reference to the back element.
    pub fn back(&self) -> Option<&T> {
        self.elements.back()
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Checks if the deque is empty.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
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
            .field("elements", &self.elements)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let deque: Deque<i32> = Deque::new();
        assert!(deque.is_empty());
    }

    #[test]
    fn test_push_pop_front() {
        let mut deque = Deque::new();
        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);

        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(1));
    }

    #[test]
    fn test_push_pop_back() {
        let mut deque = Deque::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);

        assert_eq!(deque.pop_back(), Some(3));
        assert_eq!(deque.pop_back(), Some(2));
        assert_eq!(deque.pop_back(), Some(1));
    }

    #[test]
    fn test_mixed_operations() {
        let mut deque = Deque::new();
        deque.push_back(2);
        deque.push_front(1);
        deque.push_back(3);

        // Deque: [1, 2, 3]
        assert_eq!(deque.front(), Some(&1));
        assert_eq!(deque.back(), Some(&3));

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_back(), Some(3));
        assert_eq!(deque.pop_front(), Some(2));
    }

    #[test]
    fn test_front_back() {
        let mut deque = Deque::new();
        assert_eq!(deque.front(), None);
        assert_eq!(deque.back(), None);

        deque.push_back(1);
        assert_eq!(deque.front(), Some(&1));
        assert_eq!(deque.back(), Some(&1));

        deque.push_back(2);
        assert_eq!(deque.front(), Some(&1));
        assert_eq!(deque.back(), Some(&2));
    }

    #[test]
    fn test_len() {
        let mut deque = Deque::new();
        assert_eq!(deque.len(), 0);

        deque.push_front(1);
        deque.push_back(2);
        assert_eq!(deque.len(), 2);

        deque.pop_front();
        assert_eq!(deque.len(), 1);
    }

    #[test]
    fn test_default() {
        let deque: Deque<char> = Deque::default();
        assert!(deque.is_empty());
    }
}
