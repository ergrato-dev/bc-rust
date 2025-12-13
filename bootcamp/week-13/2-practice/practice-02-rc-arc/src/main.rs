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
    ejercicio_rc_basico();

    // Ejercicio 2: Grafo con nodos compartidos
    println!("\n--- Ejercicio 2: Grafo con Rc ---");
    ejercicio_grafo();

    // Ejercicio 3: Arc en multi-threading
    println!("\n--- Ejercicio 3: Arc Multi-thread ---");
    ejercicio_arc();

    println!("\n✅ Todos los ejercicios completados!");
}

// ============================================================
// EJERCICIO 1: Rc Básico
// ============================================================

fn ejercicio_rc_basico() {
    // Crear un Rc con un vector
    let datos = Rc::new(vec![1, 2, 3, 4, 5]);
    
    println!("Datos originales: {:?}", datos);
    println!("Strong count inicial: {}", Rc::strong_count(&datos));
    
    // ↓ Crea dos clones del Rc
    let _clone1 = Rc::clone(&datos); // ← Descomentar y completar
    let _clone2 = Rc::clone(&datos); // ← Descomentar y completar
    
    println!("Strong count después de clones: {}", Rc::strong_count(&datos));
    
    // ↓ Verifica que todos apuntan a los mismos datos
    // println!("Clone1: {:?}", clone1);
    // println!("Clone2: {:?}", clone2);
    
    // ↓ Drop uno de los clones y verifica el count
    // drop(clone1);
    // println!("Strong count después de drop: {}", Rc::strong_count(&datos));
}

// ============================================================
// EJERCICIO 2: Grafo con Nodos Compartidos
// ============================================================

#[derive(Debug)]
struct Nodo {
    nombre: String,
    conexiones: Vec<Rc<Nodo>>,
}

impl Nodo {
    fn new(nombre: &str) -> Rc<Self> {
        // ↓ Implementa: crea un nodo envuelto en Rc
        Rc::new(Nodo {
            nombre: nombre.to_string(),
            conexiones: vec![],
        })
    }
    
    fn conectar(nodo: &mut Rc<Nodo>, destino: &Rc<Nodo>) {
        // ↓ Implementa: agrega una conexión al nodo
        // Pista: necesitas Rc::get_mut o usar RefCell
        // Por simplicidad, vamos a crear un nuevo nodo con la conexión
        let nuevo = Nodo {
            nombre: Rc::get_mut(nodo).map(|n| n.nombre.clone()).unwrap_or_default(),
            conexiones: vec![Rc::clone(destino)],
        };
        *nodo = Rc::new(nuevo);
    }
}

fn ejercicio_grafo() {
    // Crear nodos
    let mut a = Nodo::new("A");
    let b = Nodo::new("B");
    let c = Nodo::new("C");
    
    println!("Nodo B strong_count: {}", Rc::strong_count(&b));
    
    // Conectar A -> B y A -> C
    Nodo::conectar(&mut a, &b);
    
    println!("Después de conectar A->B:");
    println!("Nodo B strong_count: {}", Rc::strong_count(&b));
    
    // ↓ Crea otro nodo D que también conecte a B
    // let mut d = Nodo::new("D");
    // Nodo::conectar(&mut d, &b);
    // println!("Nodo B strong_count después de D->B: {}", Rc::strong_count(&b));
    
    let _ = c; // Evitar warning
}

// ============================================================
// EJERCICIO 3: Arc en Multi-threading
// ============================================================

fn ejercicio_arc() {
    // Crear datos compartidos entre threads
    let datos = Arc::new(vec![1, 2, 3, 4, 5]);
    
    println!("Datos: {:?}", datos);
    println!("Arc strong_count inicial: {}", Arc::strong_count(&datos));
    
    let mut handles = vec![];
    
    // ↓ Crea 3 threads que lean los datos
    for i in 0..3 {
        let datos_clone = Arc::clone(&datos);
        
        let handle = thread::spawn(move || {
            // ↓ Implementa: calcula algo con los datos
            let suma: i32 = datos_clone.iter().sum();
            println!("Thread {}: suma = {}", i, suma);
            suma
        });
        
        handles.push(handle);
    }
    
    println!("Arc strong_count con threads activos: {}", Arc::strong_count(&datos));
    
    // Esperar a todos los threads
    let mut resultados = vec![];
    for handle in handles {
        resultados.push(handle.join().unwrap());
    }
    
    println!("Resultados de threads: {:?}", resultados);
    println!("Arc strong_count final: {}", Arc::strong_count(&datos));
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
    fn test_nodo_creation() {
        let nodo = Nodo::new("Test");
        assert_eq!(Rc::strong_count(&nodo), 1);
    }
}
