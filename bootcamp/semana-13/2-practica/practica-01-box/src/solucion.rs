//! Solución - Práctica 01: Box<T>

fn main() {
    println!("=== Práctica 01: Box<T> - SOLUCIÓN ===\n");

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
// EJERCICIO 1: Lista Enlazada con Box - SOLUCIÓN
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
    pub fn new() -> Self {
        Lista { head: None }
    }
    
    pub fn push(&mut self, valor: T) {
        let nuevo_nodo = Box::new(Nodo {
            valor,
            siguiente: self.head.take(),
        });
        self.head = Some(nuevo_nodo);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|nodo| {
            self.head = nodo.siguiente;
            nodo.valor
        })
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|nodo| &nodo.valor)
    }
    
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(nodo) = current {
            count += 1;
            current = &nodo.siguiente;
        }
        count
    }
    
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
// EJERCICIO 2: Árbol Binario de Búsqueda - SOLUCIÓN
// ============================================================

#[derive(Debug)]
pub struct ArbolBinario<T> {
    valor: T,
    izquierda: Option<Box<ArbolBinario<T>>>,
    derecha: Option<Box<ArbolBinario<T>>>,
}

impl<T: Ord> ArbolBinario<T> {
    pub fn new(valor: T) -> Self {
        ArbolBinario {
            valor,
            izquierda: None,
            derecha: None,
        }
    }
    
    pub fn insertar(&mut self, valor: T) {
        if valor < self.valor {
            match &mut self.izquierda {
                Some(izq) => izq.insertar(valor),
                None => self.izquierda = Some(Box::new(ArbolBinario::new(valor))),
            }
        } else {
            match &mut self.derecha {
                Some(der) => der.insertar(valor),
                None => self.derecha = Some(Box::new(ArbolBinario::new(valor))),
            }
        }
    }
    
    pub fn contiene(&self, valor: &T) -> bool {
        if valor == &self.valor {
            true
        } else if valor < &self.valor {
            self.izquierda.as_ref().is_some_and(|izq| izq.contiene(valor))
        } else {
            self.derecha.as_ref().is_some_and(|der| der.contiene(valor))
        }
    }
}

// ============================================================
// EJERCICIO 3: Trait Objects con Box - SOLUCIÓN
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
        std::f64::consts::PI * self.radio * self.radio
    }
    
    fn nombre(&self) -> &str {
        "Círculo"
    }
}

impl Figura for Rectangulo {
    fn area(&self) -> f64 {
        self.ancho * self.alto
    }
    
    fn nombre(&self) -> &str {
        "Rectángulo"
    }
}

fn crear_figura(tipo: &str, dimension: f64) -> Box<dyn Figura> {
    match tipo {
        "circulo" => Box::new(Circulo { radio: dimension }),
        _ => Box::new(Rectangulo { ancho: dimension, alto: dimension }),
    }
}

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
