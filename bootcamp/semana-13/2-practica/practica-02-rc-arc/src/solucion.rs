//! Solución - Práctica 02: Rc y Arc

use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    println!("=== Práctica 02: Rc y Arc ===\n");

    println!("--- Ejercicio 1: Rc Básico ---");
    ejercicio_rc_basico();

    println!("\n--- Ejercicio 2: Grafo con Rc ---");
    ejercicio_grafo();

    println!("\n--- Ejercicio 3: Arc Multi-thread ---");
    ejercicio_arc();

    println!("\n✅ Todos los ejercicios completados!");
}

// EJERCICIO 1: Rc Básico

fn ejercicio_rc_basico() {
    let datos = Rc::new(vec![1, 2, 3, 4, 5]);
    
    println!("Datos originales: {:?}", datos);
    println!("Strong count inicial: {}", Rc::strong_count(&datos));
    
    let clone1 = Rc::clone(&datos);
    let clone2 = Rc::clone(&datos);
    
    println!("Strong count después de clones: {}", Rc::strong_count(&datos));
    
    println!("Clone1: {:?}", clone1);
    println!("Clone2: {:?}", clone2);
    
    drop(clone1);
    println!("Strong count después de drop: {}", Rc::strong_count(&datos));
    
    drop(clone2);
    println!("Strong count final: {}", Rc::strong_count(&datos));
}

// EJERCICIO 2: Grafo con Nodos Compartidos

#[derive(Debug)]
struct Nodo {
    nombre: String,
    conexiones: Vec<Rc<Nodo>>,
}

impl Nodo {
    fn new(nombre: &str) -> Rc<Self> {
        Rc::new(Nodo {
            nombre: nombre.to_string(),
            conexiones: vec![],
        })
    }
    
    fn new_con_conexiones(nombre: &str, conexiones: Vec<Rc<Nodo>>) -> Rc<Self> {
        Rc::new(Nodo {
            nombre: nombre.to_string(),
            conexiones,
        })
    }
}

fn ejercicio_grafo() {
    // Crear nodos hoja primero
    let b = Nodo::new("B");
    let c = Nodo::new("C");
    
    println!("Nodo B strong_count: {}", Rc::strong_count(&b));
    
    // Crear A conectado a B y C
    let a = Nodo::new_con_conexiones("A", vec![Rc::clone(&b), Rc::clone(&c)]);
    
    println!("Después de crear A conectado a B y C:");
    println!("Nodo B strong_count: {}", Rc::strong_count(&b));
    
    // Crear D también conectado a B
    let d = Nodo::new_con_conexiones("D", vec![Rc::clone(&b)]);
    
    println!("Nodo B strong_count después de D->B: {}", Rc::strong_count(&b));
    
    println!("Grafo: A={:?}", a.nombre);
    println!("       D={:?}", d.nombre);
}

// EJERCICIO 3: Arc en Multi-threading

fn ejercicio_arc() {
    let datos = Arc::new(vec![1, 2, 3, 4, 5]);
    
    println!("Datos: {:?}", datos);
    println!("Arc strong_count inicial: {}", Arc::strong_count(&datos));
    
    let mut handles = vec![];
    
    for i in 0..3 {
        let datos_clone = Arc::clone(&datos);
        
        let handle = thread::spawn(move || {
            let suma: i32 = datos_clone.iter().sum();
            println!("Thread {}: suma = {}", i, suma);
            suma
        });
        
        handles.push(handle);
    }
    
    // El count puede variar aquí dependiendo de timing
    println!("Arc strong_count con threads activos: {}", Arc::strong_count(&datos));
    
    let mut resultados = vec![];
    for handle in handles {
        resultados.push(handle.join().unwrap());
    }
    
    println!("Resultados de threads: {:?}", resultados);
    println!("Arc strong_count final: {}", Arc::strong_count(&datos));
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
    fn test_nodo_creation() {
        let nodo = Nodo::new("Test");
        assert_eq!(Rc::strong_count(&nodo), 1);
    }

    #[test]
    fn test_shared_nodo() {
        let b = Nodo::new("B");
        let _a = Nodo::new_con_conexiones("A", vec![Rc::clone(&b)]);
        let _c = Nodo::new_con_conexiones("C", vec![Rc::clone(&b)]);
        
        assert_eq!(Rc::strong_count(&b), 3); // b, a->b, c->b
    }
}
