//! Generic FIFO (First In, First Out) Queue
//!
//! # Example
//!
//! ```
//! use proyecto_contenedor::Queue;
//!
//! let mut queue = Queue::new();
//! queue.enqueue(1);
//! queue.enqueue(2);
//! assert_eq!(queue.dequeue(), Some(1));
//! ```

use std::collections::VecDeque;
use std::fmt::Debug;

/// Generic queue implementing the FIFO pattern.
///
/// Elements are added at the back and removed from the front.
pub struct Queue<T> {
    elements: VecDeque<T>,
}

impl<T> Queue<T> {
    /// Creates a new empty queue.
    ///
    /// # Example
    ///
    /// ```
    /// use proyecto_contenedor::Queue;
    /// let queue: Queue<i32> = Queue::new();
    /// assert!(queue.is_empty());
    /// ```
    pub fn new() -> Self {
        Queue {
            elements: VecDeque::new(),
        }
    }

    /// Adds an element to the back of the queue.
    ///
    /// # Example
    ///
    /// ```
    /// use proyecto_contenedor::Queue;
    /// let mut queue = Queue::new();
    /// queue.enqueue("hello");
    /// assert_eq!(queue.len(), 1);
    /// ```
    pub fn enqueue(&mut self, value: T) {
        self.elements.push_back(value);
    }

    /// Removes and returns the front element of the queue.
    ///
    /// Returns `None` if the queue is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use proyecto_contenedor::Queue;
    /// let mut queue = Queue::new();
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    /// assert_eq!(queue.dequeue(), Some(1));
    /// assert_eq!(queue.dequeue(), Some(2));
    /// assert_eq!(queue.dequeue(), None);
    /// ```
    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    /// Returns a reference to the front element without removing it.
    ///
    /// Returns `None` if the queue is empty.
    pub fn front(&self) -> Option<&T> {
        self.elements.front()
    }

    /// Returns the number of elements in the queue.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Checks if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> Debug for Queue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Queue")
            .field("elements", &self.elements)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let queue: Queue<i32> = Queue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_front() {
        let mut queue = Queue::new();
        assert_eq!(queue.front(), None);

        queue.enqueue("a");
        queue.enqueue("b");
        assert_eq!(queue.front(), Some(&"a"));

        queue.dequeue();
        assert_eq!(queue.front(), Some(&"b"));
    }

    #[test]
    fn test_len() {
        let mut queue = Queue::new();
        assert_eq!(queue.len(), 0);

        queue.enqueue(1);
        assert_eq!(queue.len(), 1);

        queue.enqueue(2);
        assert_eq!(queue.len(), 2);

        queue.dequeue();
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn test_fifo_order() {
        let mut queue = Queue::new();
        for i in 0..10 {
            queue.enqueue(i);
        }

        for i in 0..10 {
            assert_eq!(queue.dequeue(), Some(i));
        }
    }

    #[test]
    fn test_default() {
        let queue: Queue<String> = Queue::default();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_debug() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        let debug_str = format!("{:?}", queue);
        assert!(debug_str.contains("Queue"));
    }
}
