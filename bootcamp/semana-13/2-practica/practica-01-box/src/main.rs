//! Práctica 01: Box<T>
//!
//! En esta práctica aprenderás a:
//! - Usar Box para tipos recursivos
//! - Implementar una lista enlazada
//! - Usar Box para trait objects

fn main() {
    println!("=== Práctica 01: Box<T> ===\n");

    // Ejercicio 1: Lista enlazada básica
    println!("--- Ejercicio 1: Lista Enlazada ---");
    let mut lista = Lista::new();
    lista.push(1);
    lista.push(2);
    lista.push(3);
    
    println!("Lista: {:?}", lista);
    println!("Longitud: {}", lista.len());
    println!("Peek: {:?}", lista.peek());
    
    while let Some(valor) = lista.pop() {
        println!("Pop: {}", valor);
    }
    
    // Ejercicio 2: Árbol binario
    println!("\n--- Ejercicio 2: Árbol Binario ---");
    let mut arbol = ArbolBinario::new(5);
    arbol.insertar(3);
    arbol.insertar(7);
    arbol.insertar(1);
    arbol.insertar(9);
    
    println!("Contiene 7: {}", arbol.contiene(&7));
    println!("Contiene 4: {}", arbol.contiene(&4));
    
    // Ejercicio 3: Trait objects
    println!("\n--- Ejercicio 3: Trait Objects ---");
    let figuras: Vec<Box<dyn Figura>> = vec![
        Box::new(Circulo { radio: 5.0 }),
        Box::new(Rectangulo { ancho: 4.0, alto: 3.0 }),
        crear_figura("circulo", 2.0),
    ];
    
    let mut area_total = 0.0;
    for figura in &figuras {
        println!("{}: área = {:.2}", figura.nombre(), figura.area());
        area_total += figura.area();
    }
    println!("Área total: {:.2}", area_total);
    
    println!("\n✅ Práctica completada!");
}

// ============================================================
// EJERCICIO 1: Lista Enlazada con Box
// ============================================================

#[derive(Debug)]
pub struct Lista<T> {
    head: Option<Box<Nodo<T>>>,
}

#[derive(Debug)]
struct Nodo<T> {
    valor: T,
    siguiente: Option<Box<Nodo<T>>>,
}

impl<T> Lista<T> {
    /// Crea una lista vacía
    pub fn new() -> Self {
        // TODO: Implementar
        // ↓ Cambia esto por: Lista { head: None }
        Lista { head: None }
    }
    
    /// Agrega un elemento al inicio de la lista
    pub fn push(&mut self, valor: T) {
        // TODO: Implementar
        // Pista: Usa self.head.take() para obtener ownership del head actual
        // ↓ Completa la implementación
        let _ = valor;
    }
    
    /// Remueve y retorna el primer elemento
    pub fn pop(&mut self) -> Option<T> {
        // TODO: Implementar
        // Pista: Usa map() sobre self.head.take()
        // ↓ Cambia esto por la implementación correcta
        None
    }
    
    /// Retorna una referencia al primer elemento sin removerlo
    pub fn peek(&self) -> Option<&T> {
        // TODO: Implementar
        // Pista: Usa as_ref() y map()
        // ↓ Cambia esto por: self.head.as_ref().map(|nodo| &nodo.valor)
        None
    }
    
    /// Retorna la cantidad de elementos
    pub fn len(&self) -> usize {
        // TODO: Implementar
        // Pista: Itera sobre los nodos contando
        // ↓ Cambia esto por la implementación correcta
        0
    }
    
    /// Verifica si la lista está vacía
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

impl<T> Default for Lista<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// EJERCICIO 2: Árbol Binario de Búsqueda
// ============================================================

#[derive(Debug)]
pub struct ArbolBinario<T> {
    valor: T,
    izquierda: Option<Box<ArbolBinario<T>>>,
    derecha: Option<Box<ArbolBinario<T>>>,
}

impl<T: Ord> ArbolBinario<T> {
    /// Crea un nuevo árbol con un valor raíz
    pub fn new(valor: T) -> Self {
        // TODO: Implementar
        // ↓ Completa la implementación
        ArbolBinario {
            valor,
            izquierda: None,
            derecha: None,
        }
    }
    
    /// Inserta un valor en el árbol
    pub fn insertar(&mut self, valor: T) {
        // TODO: Implementar
        // Si valor < self.valor, insertar en izquierda
        // Si valor >= self.valor, insertar en derecha
        // ↓ Completa la implementación
        let _ = valor;
    }
    
    /// Verifica si el árbol contiene un valor
    pub fn contiene(&self, valor: &T) -> bool {
        // TODO: Implementar
        // ↓ Cambia esto por la implementación correcta
        let _ = valor;
        false
    }
}

// ============================================================
// EJERCICIO 3: Trait Objects con Box
// ============================================================

trait Figura {
    fn area(&self) -> f64;
    fn nombre(&self) -> &str;
}

struct Circulo {
    radio: f64,
}

struct Rectangulo {
    ancho: f64,
    alto: f64,
}

impl Figura for Circulo {
    fn area(&self) -> f64 {
        // TODO: Implementar
        // ↓ Cambia esto por: std::f64::consts::PI * self.radio * self.radio
        0.0
    }
    
    fn nombre(&self) -> &str {
        "Círculo"
    }
}

impl Figura for Rectangulo {
    fn area(&self) -> f64 {
        // TODO: Implementar
        // ↓ Cambia esto por: self.ancho * self.alto
        0.0
    }
    
    fn nombre(&self) -> &str {
        "Rectángulo"
    }
}

/// Factory function que retorna un trait object
fn crear_figura(tipo: &str, dimension: f64) -> Box<dyn Figura> {
    // TODO: Implementar
    // Si tipo == "circulo", crear Circulo con radio = dimension
    // Si no, crear Rectangulo cuadrado con lado = dimension
    // ↓ Cambia esto por la implementación correcta
    let _ = tipo;
    let _ = dimension;
    Box::new(Circulo { radio: 1.0 })
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lista_push_pop() {
        let mut lista = Lista::new();
        lista.push(1);
        lista.push(2);
        lista.push(3);
        
        assert_eq!(lista.pop(), Some(3));
        assert_eq!(lista.pop(), Some(2));
        assert_eq!(lista.pop(), Some(1));
        assert_eq!(lista.pop(), None);
    }
    
    #[test]
    fn test_lista_peek() {
        let mut lista = Lista::new();
        assert_eq!(lista.peek(), None);
        
        lista.push(1);
        assert_eq!(lista.peek(), Some(&1));
        
        lista.push(2);
        assert_eq!(lista.peek(), Some(&2));
    }
    
    #[test]
    fn test_lista_len() {
        let mut lista = Lista::new();
        assert_eq!(lista.len(), 0);
        
        lista.push(1);
        assert_eq!(lista.len(), 1);
        
        lista.push(2);
        lista.push(3);
        assert_eq!(lista.len(), 3);
    }
    
    #[test]
    fn test_arbol_insertar_contiene() {
        let mut arbol = ArbolBinario::new(5);
        arbol.insertar(3);
        arbol.insertar(7);
        arbol.insertar(1);
        
        assert!(arbol.contiene(&5));
        assert!(arbol.contiene(&3));
        assert!(arbol.contiene(&7));
        assert!(arbol.contiene(&1));
        assert!(!arbol.contiene(&4));
        assert!(!arbol.contiene(&10));
    }
    
    #[test]
    fn test_figura_circulo() {
        let circulo = Circulo { radio: 1.0 };
        let area = circulo.area();
        assert!((area - std::f64::consts::PI).abs() < 0.001);
    }
    
    #[test]
    fn test_figura_rectangulo() {
        let rect = Rectangulo { ancho: 4.0, alto: 3.0 };
        assert_eq!(rect.area(), 12.0);
    }
    
    #[test]
    fn test_crear_figura() {
        let circulo = crear_figura("circulo", 2.0);
        let area = circulo.area();
        let expected = std::f64::consts::PI * 4.0;
        assert!((area - expected).abs() < 0.001);
        
        let cuadrado = crear_figura("cuadrado", 3.0);
        assert_eq!(cuadrado.area(), 9.0);
    }
}
