//! Practica 04: Test-Driven Development
//!
//! Implementa una Pila (Stack) usando TDD.

fn main() {
    println!("=== Practica 04: TDD ===\n");

    let mut stack: Stack<i32> = Stack::new();
    println!("Stack vacia: {}", stack.is_empty());

    stack.push(10);
    stack.push(20);
    stack.push(30);

    println!("Despues de push 10, 20, 30:");
    println!("  len: {}", stack.len());
    println!("  peek: {:?}", stack.peek());

    println!("Pop: {:?}", stack.pop());
    println!("Pop: {:?}", stack.pop());

    println!("len despues de 2 pops: {}", stack.len());
}

/// Una pila generica (LIFO - Last In First Out).
///
/// # Example
///
/// ```
/// use practice_04_tdd::Stack;
///
/// let mut stack = Stack::new();
/// stack.push(1);
/// stack.push(2);
/// assert_eq!(stack.pop(), Some(2));
/// assert_eq!(stack.pop(), Some(1));
/// ```
#[derive(Debug)]
pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    /// Crea una pila vacia.
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    /// Agrega un elemento al tope.
    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    /// Quita y retorna el elemento del tope.
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// Retorna referencia al tope sin quitarlo.
    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    /// Verifica si la pila esta vacia.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Retorna la cantidad de elementos.
    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

impl<T> Default for Stack<T> {
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

    // Test 1: Stack vacia
    #[test]
    fn test_new_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }

    // Test 2: Push incrementa len
    #[test]
    fn test_push_increments_len() {
        let mut stack = Stack::new();
        stack.push(42);
        assert!(!stack.is_empty());
        assert_eq!(stack.len(), 1);
    }

    // Test 3: Push y Pop
    #[test]
    fn test_push_pop() {
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);
        
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), None);
    }

    // Test 4: Pop en stack vacia
    #[test]
    fn test_pop_empty_stack() {
        let mut stack: Stack<i32> = Stack::new();
        assert_eq!(stack.pop(), None);
    }

    // Test 5: Peek
    #[test]
    fn test_peek() {
        let mut stack = Stack::new();
        stack.push(100);
        
        // Peek no consume
        assert_eq!(stack.peek(), Some(&100));
        assert_eq!(stack.peek(), Some(&100));
        assert_eq!(stack.len(), 1);
    }

    // Test 6: Peek stack vacia
    #[test]
    fn test_peek_empty_stack() {
        let stack: Stack<String> = Stack::new();
        assert_eq!(stack.peek(), None);
    }

    // Test 7: Orden LIFO
    #[test]
    fn test_lifo_order() {
        let mut stack = Stack::new();
        stack.push('a');
        stack.push('b');
        stack.push('c');
        
        assert_eq!(stack.pop(), Some('c'));
        assert_eq!(stack.pop(), Some('b'));
        assert_eq!(stack.pop(), Some('a'));
    }

    // Test 8: Multiples tipos
    #[test]
    fn test_stack_strings() {
        let mut stack = Stack::new();
        stack.push(String::from("uno"));
        stack.push(String::from("dos"));
        
        assert_eq!(stack.pop(), Some(String::from("dos")));
    }

    // Test 9: Default
    #[test]
    fn test_default() {
        let stack: Stack<i32> = Stack::default();
        assert!(stack.is_empty());
    }
}
