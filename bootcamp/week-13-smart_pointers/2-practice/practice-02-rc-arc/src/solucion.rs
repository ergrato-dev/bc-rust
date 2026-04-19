//! Solución - Práctica 02: Rc y Arc

use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    println!("=== Práctica 02: Rc y Arc ===\n");

    println!("--- Ejercicio 1: Rc Básico ---");
    exercise_rc_basic();

    println!("\n--- Ejercicio 2: Grafo con Rc ---");
    exercise_graph();

    println!("\n--- Ejercicio 3: Arc Multi-thread ---");
    exercise_arc();

    println!("\n✅ Todos los ejercicios completados!");
}

// EJERCICIO 1: Rc Básico

fn exercise_rc_basic() {
    let data = Rc::new(vec![1, 2, 3, 4, 5]);
    
    println!("Datos originales: {:?}", data);
    println!("Strong count inicial: {}", Rc::strong_count(&data));
    
    let clone1 = Rc::clone(&data);
    let clone2 = Rc::clone(&data);
    
    println!("Strong count después de clones: {}", Rc::strong_count(&data));
    
    println!("Clone1: {:?}", clone1);
    println!("Clone2: {:?}", clone2);
    
    drop(clone1);
    println!("Strong count después de drop: {}", Rc::strong_count(&data));
    
    drop(clone2);
    println!("Strong count final: {}", Rc::strong_count(&data));
}

// EJERCICIO 2: Grafo con Nodos Compartidos

#[derive(Debug)]
struct Node {
    name: String,
    connections: Vec<Rc<Node>>,
}

impl Node {
    fn new(name: &str) -> Rc<Self> {
        Rc::new(Node {
            name: name.to_string(),
            connections: vec![],
        })
    }
    
    fn new_with_connections(name: &str, connections: Vec<Rc<Node>>) -> Rc<Self> {
        Rc::new(Node {
            name: name.to_string(),
            connections,
        })
    }
}

fn exercise_graph() {
    // Crear nodos hoja primero
    let b = Node::new("B");
    let c = Node::new("C");
    
    println!("Nodo B strong_count: {}", Rc::strong_count(&b));
    
    // Crear A conectado a B y C
    let a = Node::new_with_connections("A", vec![Rc::clone(&b), Rc::clone(&c)]);
    
    println!("Después de crear A conectado a B y C:");
    println!("Nodo B strong_count: {}", Rc::strong_count(&b));
    
    // Crear D también conectado a B
    let d = Node::new_with_connections("D", vec![Rc::clone(&b)]);
    
    println!("Nodo B strong_count después de D->B: {}", Rc::strong_count(&b));
    
    println!("Grafo: A={:?}", a.name);
    println!("       D={:?}", d.name);
}

// EJERCICIO 3: Arc en Multi-threading

fn exercise_arc() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    
    println!("Datos: {:?}", data);
    println!("Arc strong_count inicial: {}", Arc::strong_count(&data));
    
    let mut handles = vec![];
    
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        
        let handle = thread::spawn(move || {
            let sum: i32 = data_clone.iter().sum();
            println!("Thread {}: suma = {}", i, sum);
            sum
        });
        
        handles.push(handle);
    }
    
    // El count puede variar aquí dependiendo de timing
    println!("Arc strong_count con threads activos: {}", Arc::strong_count(&data));
    
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }
    
    println!("Resultados de threads: {:?}", results);
    println!("Arc strong_count final: {}", Arc::strong_count(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc_clone_increments_count() {
        let data = Rc::new(42);
        assert_eq!(Rc::strong_count(&data), 1);
        
        let _clone = Rc::clone(&data);
        assert_eq!(Rc::strong_count(&data), 2);
    }

    #[test]
    fn test_rc_drop_decrements_count() {
        let data = Rc::new(42);
        let clone = Rc::clone(&data);
        assert_eq!(Rc::strong_count(&data), 2);
        
        drop(clone);
        assert_eq!(Rc::strong_count(&data), 1);
    }

    #[test]
    fn test_arc_across_threads() {
        let data = Arc::new(vec![1, 2, 3]);
        let data_clone = Arc::clone(&data);
        
        let handle = thread::spawn(move || {
            data_clone.iter().sum::<i32>()
        });
        
        let result = handle.join().unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_node_creation() {
        let node = Node::new("Test");
        assert_eq!(Rc::strong_count(&node), 1);
    }

    #[test]
    fn test_shared_node() {
        let b = Node::new("B");
        let _a = Node::new_with_connections("A", vec![Rc::clone(&b)]);
        let _c = Node::new_with_connections("C", vec![Rc::clone(&b)]);
        
        assert_eq!(Rc::strong_count(&b), 3); // b, a->b, c->b
    }
}
