//! Práctica 02: Rc<T> y Arc<T>
//!
//! En esta práctica aprenderás a:
//! - Usar Rc para compartir ownership
//! - Usar Weak para evitar ciclos
//! - Usar Arc para compartir entre threads

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::sync::Arc;
use std::thread;

fn main() {
    println!("=== Práctica 02: Rc<T> y Arc<T> ===\n");

    // Ejercicio 1: Rc básico
    println!("--- Ejercicio 1: Rc Básico ---");
    let datos = crear_datos_compartidos();
    let ref1 = Rc::clone(&datos);
    let ref2 = Rc::clone(&datos);
    
    println!("Strong count: {}", Rc::strong_count(&datos));
    println!("Datos: {:?}", datos);
    
    drop(ref1);
    println!("Después de drop ref1, count: {}", Rc::strong_count(&datos));
    
    drop(ref2);
    println!("Después de drop ref2, count: {}", Rc::strong_count(&datos));
    
    // Ejercicio 2: Grafo con nodos compartidos
    println!("\n--- Ejercicio 2: Grafo ---");
    let mut grafo = Grafo::new();
    grafo.agregar_nodo("A");
    grafo.agregar_nodo("B");
    grafo.agregar_nodo("C");
    
    grafo.conectar("A", "B");
    grafo.conectar("A", "C");
    grafo.conectar("B", "C");
    
    println!("Vecinos de A: {:?}", grafo.vecinos_de("A"));
    println!("Vecinos de B: {:?}", grafo.vecinos_de("B"));
    
    // Ejercicio 3: Árbol con parent (Weak)
    println!("\n--- Ejercicio 3: Árbol con Weak ---");
    let raiz = NodoArbol::new(1);
    let hijo1 = NodoArbol::new(2);
    let hijo2 = NodoArbol::new(3);
    
    NodoArbol::agregar_hijo(&raiz, &hijo1);
    NodoArbol::agregar_hijo(&raiz, &hijo2);
    
    println!("Raíz tiene {} hijos", raiz.borrow().hijos.len());
    println!("Profundidad de hijo1: {}", NodoArbol::profundidad(&hijo1));
    
    // Verificar que Weak no previene liberación
    println!("Strong count de raíz: {}", Rc::strong_count(&raiz));
    
    // Ejercicio 4: Arc con threads
    println!("\n--- Ejercicio 4: Arc con Threads ---");
    let resultado = procesar_en_paralelo(vec![1, 2, 3, 4, 5]);
    println!("Resultado: {:?}", resultado);
    
    println!("\n✅ Práctica completada!");
}

// ============================================================
// EJERCICIO 1: Rc Básico
// ============================================================

/// Crea datos compartidos usando Rc
fn crear_datos_compartidos() -> Rc<Vec<i32>> {
    // TODO: Crear un Rc con un vector [1, 2, 3, 4, 5]
    // ↓ Cambia esto por: Rc::new(vec![1, 2, 3, 4, 5])
    Rc::new(vec![])
}

// ============================================================
// EJERCICIO 2: Grafo con Nodos Compartidos
// ============================================================

type NodoRef = Rc<RefCell<NodoGrafo>>;

#[derive(Debug)]
struct NodoGrafo {
    id: String,
    vecinos: Vec<NodoRef>,
}

struct Grafo {
    nodos: std::collections::HashMap<String, NodoRef>,
}

impl Grafo {
    fn new() -> Self {
        Grafo {
            nodos: std::collections::HashMap::new(),
        }
    }
    
    /// Agrega un nodo al grafo
    fn agregar_nodo(&mut self, id: &str) {
        // TODO: Crear un NodoRef y agregarlo al HashMap
        // ↓ Completa la implementación
        let _ = id;
    }
    
    /// Conecta dos nodos (agrega 'hacia' como vecino de 'desde')
    fn conectar(&self, desde: &str, hacia: &str) {
        // TODO: Obtener ambos nodos y agregar 'hacia' a los vecinos de 'desde'
        // Pista: Usa Rc::clone() para clonar la referencia
        // ↓ Completa la implementación
        let _ = desde;
        let _ = hacia;
    }
    
    /// Retorna los IDs de los vecinos de un nodo
    fn vecinos_de(&self, id: &str) -> Vec<String> {
        // TODO: Obtener el nodo y retornar los IDs de sus vecinos
        // ↓ Cambia esto por la implementación correcta
        let _ = id;
        vec![]
    }
}

// ============================================================
// EJERCICIO 3: Árbol con Parent usando Weak
// ============================================================

type NodoArbolRef = Rc<RefCell<NodoArbolInner>>;

#[derive(Debug)]
struct NodoArbolInner {
    valor: i32,
    padre: Weak<RefCell<NodoArbolInner>>,
    hijos: Vec<NodoArbolRef>,
}

struct NodoArbol;

impl NodoArbol {
    /// Crea un nuevo nodo
    fn new(valor: i32) -> NodoArbolRef {
        // TODO: Crear un Rc<RefCell<NodoArbolInner>>
        // ↓ Completa la implementación
        Rc::new(RefCell::new(NodoArbolInner {
            valor,
            padre: Weak::new(),
            hijos: Vec::new(),
        }))
    }
    
    /// Agrega un hijo al padre
    fn agregar_hijo(padre: &NodoArbolRef, hijo: &NodoArbolRef) {
        // TODO: 
        // 1. Establecer el padre del hijo usando Rc::downgrade()
        // 2. Agregar el hijo a la lista de hijos del padre
        // ↓ Completa la implementación
        let _ = padre;
        let _ = hijo;
    }
    
    /// Calcula la profundidad del nodo (distancia a la raíz)
    fn profundidad(nodo: &NodoArbolRef) -> usize {
        // TODO: Navegar hacia arriba usando upgrade() contando niveles
        // ↓ Cambia esto por la implementación correcta
        let _ = nodo;
        0
    }
}

// ============================================================
// EJERCICIO 4: Arc con Threads
// ============================================================

/// Procesa datos en paralelo usando Arc
fn procesar_en_paralelo(datos: Vec<i32>) -> Vec<i32> {
    // TODO:
    // 1. Crear un Arc con los datos
    // 2. Spawn 3 threads que calculen: suma, producto de los primeros 3, máximo
    // 3. Retornar [suma, producto, maximo]
    // ↓ Cambia esto por la implementación correcta
    let _ = datos;
    vec![0, 0, 0]
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc_basico() {
        let datos = crear_datos_compartidos();
        assert_eq!(Rc::strong_count(&datos), 1);
        
        let clone = Rc::clone(&datos);
        assert_eq!(Rc::strong_count(&datos), 2);
        
        drop(clone);
        assert_eq!(Rc::strong_count(&datos), 1);
    }
    
    #[test]
    fn test_grafo_agregar_nodo() {
        let mut grafo = Grafo::new();
        grafo.agregar_nodo("A");
        grafo.agregar_nodo("B");
        
        assert!(grafo.nodos.contains_key("A"));
        assert!(grafo.nodos.contains_key("B"));
    }
    
    #[test]
    fn test_grafo_conectar() {
        let mut grafo = Grafo::new();
        grafo.agregar_nodo("A");
        grafo.agregar_nodo("B");
        grafo.conectar("A", "B");
        
        let vecinos = grafo.vecinos_de("A");
        assert_eq!(vecinos, vec!["B"]);
    }
    
    #[test]
    fn test_arbol_padre_hijo() {
        let raiz = NodoArbol::new(1);
        let hijo = NodoArbol::new(2);
        
        NodoArbol::agregar_hijo(&raiz, &hijo);
        
        assert_eq!(raiz.borrow().hijos.len(), 1);
        assert!(hijo.borrow().padre.upgrade().is_some());
    }
    
    #[test]
    fn test_arbol_profundidad() {
        let raiz = NodoArbol::new(1);
        let hijo = NodoArbol::new(2);
        let nieto = NodoArbol::new(3);
        
        NodoArbol::agregar_hijo(&raiz, &hijo);
        NodoArbol::agregar_hijo(&hijo, &nieto);
        
        assert_eq!(NodoArbol::profundidad(&raiz), 0);
        assert_eq!(NodoArbol::profundidad(&hijo), 1);
        assert_eq!(NodoArbol::profundidad(&nieto), 2);
    }
    
    #[test]
    fn test_procesar_paralelo() {
        let resultado = procesar_en_paralelo(vec![1, 2, 3, 4, 5]);
        
        assert_eq!(resultado[0], 15);  // suma
        assert_eq!(resultado[1], 6);   // producto de 1*2*3
        assert_eq!(resultado[2], 5);   // máximo
    }
}
