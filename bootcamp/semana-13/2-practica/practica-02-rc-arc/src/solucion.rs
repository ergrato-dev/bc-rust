//! Solución - Práctica 02: Rc<T> y Arc<T>

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::sync::Arc;
use std::thread;

fn main() {
    println!("=== Práctica 02: Rc<T> y Arc<T> - SOLUCIÓN ===\n");

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
    
    println!("Strong count de raíz: {}", Rc::strong_count(&raiz));
    
    // Ejercicio 4: Arc con threads
    println!("\n--- Ejercicio 4: Arc con Threads ---");
    let resultado = procesar_en_paralelo(vec![1, 2, 3, 4, 5]);
    println!("Resultado: {:?}", resultado);
    
    println!("\n✅ Práctica completada!");
}

// ============================================================
// EJERCICIO 1: Rc Básico - SOLUCIÓN
// ============================================================

fn crear_datos_compartidos() -> Rc<Vec<i32>> {
    Rc::new(vec![1, 2, 3, 4, 5])
}

// ============================================================
// EJERCICIO 2: Grafo con Nodos Compartidos - SOLUCIÓN
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
    
    fn agregar_nodo(&mut self, id: &str) {
        let nodo = Rc::new(RefCell::new(NodoGrafo {
            id: id.to_string(),
            vecinos: Vec::new(),
        }));
        self.nodos.insert(id.to_string(), nodo);
    }
    
    fn conectar(&self, desde: &str, hacia: &str) {
        if let (Some(nodo_desde), Some(nodo_hacia)) = 
            (self.nodos.get(desde), self.nodos.get(hacia)) 
        {
            nodo_desde.borrow_mut().vecinos.push(Rc::clone(nodo_hacia));
        }
    }
    
    fn vecinos_de(&self, id: &str) -> Vec<String> {
        self.nodos.get(id)
            .map(|nodo| {
                nodo.borrow()
                    .vecinos
                    .iter()
                    .map(|v| v.borrow().id.clone())
                    .collect()
            })
            .unwrap_or_default()
    }
}

// ============================================================
// EJERCICIO 3: Árbol con Parent usando Weak - SOLUCIÓN
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
    fn new(valor: i32) -> NodoArbolRef {
        Rc::new(RefCell::new(NodoArbolInner {
            valor,
            padre: Weak::new(),
            hijos: Vec::new(),
        }))
    }
    
    fn agregar_hijo(padre: &NodoArbolRef, hijo: &NodoArbolRef) {
        // Establecer el padre del hijo
        hijo.borrow_mut().padre = Rc::downgrade(padre);
        // Agregar el hijo a la lista de hijos
        padre.borrow_mut().hijos.push(Rc::clone(hijo));
    }
    
    fn profundidad(nodo: &NodoArbolRef) -> usize {
        match nodo.borrow().padre.upgrade() {
            Some(padre) => 1 + Self::profundidad(&padre),
            None => 0,
        }
    }
}

// ============================================================
// EJERCICIO 4: Arc con Threads - SOLUCIÓN
// ============================================================

fn procesar_en_paralelo(datos: Vec<i32>) -> Vec<i32> {
    let datos = Arc::new(datos);
    
    // Thread 1: Calcular suma
    let datos_suma = Arc::clone(&datos);
    let handle_suma = thread::spawn(move || {
        datos_suma.iter().sum::<i32>()
    });
    
    // Thread 2: Calcular producto de los primeros 3
    let datos_prod = Arc::clone(&datos);
    let handle_prod = thread::spawn(move || {
        datos_prod.iter().take(3).product::<i32>()
    });
    
    // Thread 3: Calcular máximo
    let datos_max = Arc::clone(&datos);
    let handle_max = thread::spawn(move || {
        *datos_max.iter().max().unwrap_or(&0)
    });
    
    // Esperar resultados
    let suma = handle_suma.join().unwrap();
    let producto = handle_prod.join().unwrap();
    let maximo = handle_max.join().unwrap();
    
    vec![suma, producto, maximo]
}

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
        
        assert_eq!(resultado[0], 15);
        assert_eq!(resultado[1], 6);
        assert_eq!(resultado[2], 5);
    }
}
