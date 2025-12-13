//! Practica 04: Test-Driven Development
//!
//! Implementa una Pila (Stack) usando TDD.

fn main() {
    println!("=== Practica 04: TDD ===\n");

    let mut pila: Pila<i32> = Pila::new();
    println!("Pila vacia: {}", pila.is_empty());

    pila.push(10);
    pila.push(20);
    pila.push(30);

    println!("Despues de push 10, 20, 30:");
    println!("  len: {}", pila.len());
    println!("  peek: {:?}", pila.peek());

    println!("Pop: {:?}", pila.pop());
    println!("Pop: {:?}", pila.pop());

    println!("len despues de 2 pops: {}", pila.len());
}

/// Una pila generica (LIFO - Last In First Out).
///
/// # Example
///
/// ```
/// use practica_04_tdd::Pila;
///
/// let mut pila = Pila::new();
/// pila.push(1);
/// pila.push(2);
/// assert_eq!(pila.pop(), Some(2));
/// assert_eq!(pila.pop(), Some(1));
/// ```
#[derive(Debug)]
pub struct Pila<T> {
    elementos: Vec<T>,
}

impl<T> Pila<T> {
    /// Crea una pila vacia.
    pub fn new() -> Self {
        Pila {
            elementos: Vec::new(),
        }
    }

    /// Agrega un elemento al tope.
    pub fn push(&mut self, item: T) {
        self.elementos.push(item);
    }

    /// Quita y retorna el elemento del tope.
    pub fn pop(&mut self) -> Option<T> {
        self.elementos.pop()
    }

    /// Retorna referencia al tope sin quitarlo.
    pub fn peek(&self) -> Option<&T> {
        self.elementos.last()
    }

    /// Verifica si la pila esta vacia.
    pub fn is_empty(&self) -> bool {
        self.elementos.is_empty()
    }

    /// Retorna la cantidad de elementos.
    pub fn len(&self) -> usize {
        self.elementos.len()
    }
}

impl<T> Default for Pila<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================
// TESTS (desarrollados con TDD)
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    // Test 1: Pila vacia
    #[test]
    fn test_nueva_pila_esta_vacia() {
        let pila: Pila<i32> = Pila::new();
        assert!(pila.is_empty());
        assert_eq!(pila.len(), 0);
    }

    // Test 2: Push incrementa len
    #[test]
    fn test_push_incrementa_len() {
        let mut pila = Pila::new();
        pila.push(42);
        assert!(!pila.is_empty());
        assert_eq!(pila.len(), 1);
    }

    // Test 3: Push y Pop
    #[test]
    fn test_push_pop() {
        let mut pila = Pila::new();
        pila.push(10);
        pila.push(20);
        
        assert_eq!(pila.pop(), Some(20));
        assert_eq!(pila.pop(), Some(10));
        assert_eq!(pila.pop(), None);
    }

    // Test 4: Pop en pila vacia
    #[test]
    fn test_pop_pila_vacia() {
        let mut pila: Pila<i32> = Pila::new();
        assert_eq!(pila.pop(), None);
    }

    // Test 5: Peek
    #[test]
    fn test_peek() {
        let mut pila = Pila::new();
        pila.push(100);
        
        // Peek no consume
        assert_eq!(pila.peek(), Some(&100));
        assert_eq!(pila.peek(), Some(&100));
        assert_eq!(pila.len(), 1);
    }

    // Test 6: Peek pila vacia
    #[test]
    fn test_peek_pila_vacia() {
        let pila: Pila<String> = Pila::new();
        assert_eq!(pila.peek(), None);
    }

    // Test 7: Orden LIFO
    #[test]
    fn test_orden_lifo() {
        let mut pila = Pila::new();
        pila.push('a');
        pila.push('b');
        pila.push('c');
        
        assert_eq!(pila.pop(), Some('c'));
        assert_eq!(pila.pop(), Some('b'));
        assert_eq!(pila.pop(), Some('a'));
    }

    // Test 8: Multiples tipos
    #[test]
    fn test_pila_strings() {
        let mut pila = Pila::new();
        pila.push(String::from("uno"));
        pila.push(String::from("dos"));
        
        assert_eq!(pila.pop(), Some(String::from("dos")));
    }

    // Test 9: Default
    #[test]
    fn test_default() {
        let pila: Pila<i32> = Pila::default();
        assert!(pila.is_empty());
    }
}
