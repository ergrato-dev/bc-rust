//! Práctica 02: Rc y Arc - Reference Counting
//!
//! En esta práctica aprenderás a:
//! - Compartir datos con múltiples propietarios usando Rc
//! - Usar Arc para contextos multi-thread
//! - Entender strong_count y weak_count

use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    println!("=== Práctica 02: Rc y Arc ===\n");

    // Ejercicio 1: Compartir datos con Rc
    println!("--- Ejercicio 1: Rc Básico ---");
    exercise_rc_basic();

    // Ejercicio 2: Grafo con nodos compartidos
    println!("\n--- Ejercicio 2: Grafo con Rc ---");
    exercise_graph();

    // Ejercicio 3: Arc en multi-threading
    println!("\n--- Ejercicio 3: Arc Multi-thread ---");
    exercise_arc();

    println!("\n✅ Todos los ejercicios completados!");
}

// ============================================================
// EJERCICIO 1: Rc Básico
// ============================================================

fn exercise_rc_basic() {
    // Crear un Rc con un vector
    let data = Rc::new(vec![1, 2, 3, 4, 5]);
    
    println!("Datos originales: {:?}", data);
    println!("Strong count inicial: {}", Rc::strong_count(&data));
    
    // ↓ Crea dos clones del Rc
    let _clone1 = Rc::clone(&data); // ← Descomentar y completar
    let _clone2 = Rc::clone(&data); // ← Descomentar y completar
    
    println!("Strong count después de clones: {}", Rc::strong_count(&data));
    
    // ↓ Verifica que todos apuntan a los mismos datos
    // println!("Clone1: {:?}", clone1);
    // println!("Clone2: {:?}", clone2);
    
    // ↓ Drop uno de los clones y verifica el count
    // drop(clone1);
    // println!("Strong count después de drop: {}", Rc::strong_count(&data));
}

// ============================================================
// EJERCICIO 2: Grafo con Nodos Compartidos
// ============================================================

#[derive(Debug)]
struct Node {
    name: String,
    connections: Vec<Rc<Node>>,
}

impl Node {
    fn new(name: &str) -> Rc<Self> {
        // ↓ Implementa: crea un nodo envuelto en Rc
        Rc::new(Node {
            name: name.to_string(),
            connections: vec![],
        })
    }
    
    fn connect(node: &mut Rc<Node>, dest: &Rc<Node>) {
        // ↓ Implementa: agrega una conexión al nodo
        // Pista: necesitas Rc::get_mut o usar RefCell
        // Por simplicidad, vamos a crear un nuevo nodo con la conexión
        let new_node = Node {
            name: Rc::get_mut(node).map(|n| n.name.clone()).unwrap_or_default(),
            connections: vec![Rc::clone(dest)],
        };
        *node = Rc::new(new_node);
    }
}

fn exercise_graph() {
    // Crear nodos
    let mut a = Node::new("A");
    let b = Node::new("B");
    let c = Node::new("C");
    
    println!("Nodo B strong_count: {}", Rc::strong_count(&b));
    
    // Conectar A -> B y A -> C
    Node::connect(&mut a, &b);
    
    println!("Después de conectar A->B:");
    println!("Nodo B strong_count: {}", Rc::strong_count(&b));
    
    // ↓ Crea otro nodo D que también conecte a B
    // let mut d = Node::new("D");
    // Node::connect(&mut d, &b);
    // println!("Nodo B strong_count después de D->B: {}", Rc::strong_count(&b));
    
    let _ = c; // Evitar warning
}

// ============================================================
// EJERCICIO 3: Arc en Multi-threading
// ============================================================

fn exercise_arc() {
    // Crear datos compartidos entre threads
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    
    println!("Datos: {:?}", data);
    println!("Arc strong_count inicial: {}", Arc::strong_count(&data));
    
    let mut handles = vec![];
    
    // ↓ Crea 3 threads que lean los datos
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        
        let handle = thread::spawn(move || {
            // ↓ Implementa: calcula algo con los datos
            let sum: i32 = data_clone.iter().sum();
            println!("Thread {}: suma = {}", i, sum);
            sum
        });
        
        handles.push(handle);
    }
    
    println!("Arc strong_count con threads activos: {}", Arc::strong_count(&data));
    
    // Esperar a todos los threads
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }
    
    println!("Resultados de threads: {:?}", results);
    println!("Arc strong_count final: {}", Arc::strong_count(&data));
}

// ============================================================
// TESTS
// ============================================================

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
}
