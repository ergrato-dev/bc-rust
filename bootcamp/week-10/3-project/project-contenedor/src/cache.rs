//! Simplified LRU (Least Recently Used) Cache.
//!
//! When the cache is full, removes the least recently used element.
//!
//! # Example
//!
//! ```
//! use proyecto_contenedor::Cache;
//!
//! let mut cache = Cache::new(2);
//! cache.insert("a", 1);
//! cache.insert("b", 2);
//! cache.get(&"a"); // "a" is now the most recent
//! cache.insert("c", 3); // Removes "b" (LRU)
//! assert!(!cache.contains(&"b"));
//! ```

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// Cache with LRU replacement policy.
///
/// `K` is the key type, `V` is the value type.
pub struct Cache<K, V> {
    capacity: usize,
    data: HashMap<K, V>,
    order: Vec<K>, // The last element is the most recently used
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
{
    /// Creates a new cache with the specified capacity.
    ///
    /// # Panics
    ///
    /// Panics if capacity is 0.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than 0");
        Cache {
            capacity,
            data: HashMap::with_capacity(capacity),
            order: Vec::with_capacity(capacity),
        }
    }

    /// Inserts a key-value pair into the cache.
    ///
    /// If the cache is full, removes the least recently used element.
    /// If the key already exists, updates the value and marks it as recent.
    pub fn insert(&mut self, key: K, value: V) {
        if self.data.contains_key(&key) {
            // Update existing value and move to end (most recent)
            self.data.insert(key.clone(), value);
            self.move_to_end(&key);
        } else {
            // New element
            if self.data.len() >= self.capacity {
                // Remove the least recently used (first in order)
                if let Some(lru_key) = self.order.first().cloned() {
                    self.data.remove(&lru_key);
                    self.order.remove(0);
                }
            }
            self.data.insert(key.clone(), value);
            self.order.push(key);
        }
    }

    /// Gets a reference to the value associated with the key.
    ///
    /// If it exists, marks the key as recently used.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.data.contains_key(key) {
            self.move_to_end(key);
            self.data.get(key)
        } else {
            None
        }
    }

    /// Checks if the cache contains the key.
    ///
    /// **Note**: This operation does NOT update the LRU order.
    pub fn contains(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    /// Returns the number of elements in the cache.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the maximum capacity of the cache.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Removes an element from the cache.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.data.remove(key) {
            self.order.retain(|k| k != key);
            Some(value)
        } else {
            None
        }
    }

    /// Clears all elements from the cache.
    pub fn clear(&mut self) {
        self.data.clear();
        self.order.clear();
    }

    // Helper method to move a key to the end of the order
    fn move_to_end(&mut self, key: &K) {
        self.order.retain(|k| k != key);
        self.order.push(key.clone());
    }
}

impl<K, V> Default for Cache<K, V>
where
    K: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self::new(16) // Reasonable default capacity
    }
}

impl<K, V> Debug for Cache<K, V>
where
    K: Eq + Hash + Clone + Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cache")
            .field("capacity", &self.capacity)
            .field("len", &self.data.len())
            .field("data", &self.data)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cache: Cache<String, i32> = Cache::new(10);
        assert!(cache.is_empty());
        assert_eq!(cache.capacity(), 10);
    }

    #[test]
    #[should_panic(expected = "Capacity must be greater than 0")]
    fn test_new_zero_capacity() {
        let _cache: Cache<i32, i32> = Cache::new(0);
    }

    #[test]
    fn test_insert_get() {
        let mut cache = Cache::new(3);
        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.insert("c", 3);

        assert_eq!(cache.get(&"a"), Some(&1));
        assert_eq!(cache.get(&"b"), Some(&2));
        assert_eq!(cache.get(&"c"), Some(&3));
        assert_eq!(cache.get(&"d"), None);
    }

    #[test]
    fn test_lru_eviction() {
        let mut cache = Cache::new(2);
        cache.insert("a", 1);
        cache.insert("b", 2);

        // "a" is LRU, "b" is most recent
        cache.insert("c", 3); // Should remove "a"

        assert!(!cache.contains(&"a"));
        assert!(cache.contains(&"b"));
        assert!(cache.contains(&"c"));
    }

    #[test]
    fn test_lru_access_updates_order() {
        let mut cache = Cache::new(2);
        cache.insert("a", 1);
        cache.insert("b", 2);

        // Accessing "a" makes it most recent
        cache.get(&"a");

        // Now "b" is LRU
        cache.insert("c", 3); // Should remove "b"

        assert!(cache.contains(&"a"));
        assert!(!cache.contains(&"b"));
        assert!(cache.contains(&"c"));
    }

    #[test]
    fn test_update_existing() {
        let mut cache = Cache::new(2);
        cache.insert("a", 1);
        cache.insert("b", 2);

        // Update "a" with new value
        cache.insert("a", 100);

        assert_eq!(cache.get(&"a"), Some(&100));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_contains() {
        let mut cache = Cache::new(2);
        assert!(!cache.contains(&"a"));

        cache.insert("a", 1);
        assert!(cache.contains(&"a"));
        assert!(!cache.contains(&"b"));
    }

    #[test]
    fn test_len() {
        let mut cache = Cache::new(5);
        assert_eq!(cache.len(), 0);

        cache.insert("a", 1);
        assert_eq!(cache.len(), 1);

        cache.insert("b", 2);
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_remove() {
        let mut cache = Cache::new(3);
        cache.insert("a", 1);
        cache.insert("b", 2);

        assert_eq!(cache.remove(&"a"), Some(1));
        assert!(!cache.contains(&"a"));
        assert_eq!(cache.len(), 1);

        assert_eq!(cache.remove(&"c"), None);
    }

    #[test]
    fn test_clear() {
        let mut cache = Cache::new(3);
        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.insert("c", 3);

        cache.clear();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_with_complex_types() {
        let mut cache: Cache<String, Vec<i32>> = Cache::new(2);
        cache.insert("nums".to_string(), vec![1, 2, 3]);
        cache.insert("more".to_string(), vec![4, 5]);

        assert_eq!(cache.get(&"nums".to_string()), Some(&vec![1, 2, 3]));
    }
}
