//! Solución - Práctica 03: RefCell

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("=== Práctica 03: RefCell ===\n");

    println!("--- Ejercicio 1: RefCell Básico ---");
    ejercicio_refcell_basico();

    println!("\n--- Ejercicio 2: Contador Compartido ---");
    ejercicio_contador();

    println!("\n--- Ejercicio 3: Cache Interno ---");
    ejercicio_cache();

    println!("\n✅ Todos los ejercicios completados!");
}

// EJERCICIO 1: RefCell Básico

fn ejercicio_refcell_basico() {
    let datos = RefCell::new(vec![1, 2, 3]);
    
    {
        let r = datos.borrow();
        println!("Datos: {:?}", *r);
        println!("Longitud: {}", r.len());
    }
    
    {
        let mut r = datos.borrow_mut();
        r.push(4);
        r.push(5);
    }
    
    println!("Después de agregar: {:?}", datos.borrow());
    
    // Demostrar try_borrow_mut
    let r1 = datos.borrow();
    match datos.try_borrow_mut() {
        Ok(mut r) => r.push(6),
        Err(e) => println!("Error esperado: {}", e),
    }
    drop(r1);
    
    // Ahora sí funciona
    {
        let mut r = datos.borrow_mut();
        r.push(6);
    }
    println!("Final: {:?}", datos.borrow());
}

// EJERCICIO 2: Contador Compartido

#[derive(Debug)]
struct Contador {
    valor: i32,
    nombre: String,
}

impl Contador {
    fn new(nombre: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Contador {
            valor: 0,
            nombre: nombre.to_string(),
        }))
    }
    
    fn incrementar(contador: &Rc<RefCell<Self>>) {
        contador.borrow_mut().valor += 1;
    }
    
    fn obtener_valor(contador: &Rc<RefCell<Self>>) -> i32 {
        contador.borrow().valor
    }
    
    fn obtener_nombre(contador: &Rc<RefCell<Self>>) -> String {
        contador.borrow().nombre.clone()
    }
}

fn ejercicio_contador() {
    let contador = Contador::new("Principal");
    
    println!("Contador '{}' inicial: {}", 
             Contador::obtener_nombre(&contador),
             Contador::obtener_valor(&contador));
    
    let ref1 = Rc::clone(&contador);
    let ref2 = Rc::clone(&contador);
    
    Contador::incrementar(&ref1);
    Contador::incrementar(&ref2);
    Contador::incrementar(&contador);
    
    println!("Valor después de 3 incrementos: {}", Contador::obtener_valor(&contador));
    println!("Strong count: {}", Rc::strong_count(&contador));
}

// EJERCICIO 3: Cache con Interior Mutability

struct Calculadora {
    cache: RefCell<Option<i32>>,
    base: i32,
}

impl Calculadora {
    fn new(base: i32) -> Self {
        Calculadora {
            cache: RefCell::new(None),
            base,
        }
    }
    
    fn calcular_costoso(&self) -> i32 {
        if let Some(valor) = *self.cache.borrow() {
            println!("  (usando cache)");
            return valor;
        }
        
        println!("  (calculando...)");
        let resultado = self.base * self.base * 2;
        
        *self.cache.borrow_mut() = Some(resultado);
        
        resultado
    }
    
    fn invalidar_cache(&self) {
        *self.cache.borrow_mut() = None;
    }
}

fn ejercicio_cache() {
    let calc = Calculadora::new(5);
    
    println!("Primera llamada:");
    let r1 = calc.calcular_costoso();
    println!("Resultado: {}", r1);
    
    println!("\nSegunda llamada:");
    let r2 = calc.calcular_costoso();
    println!("Resultado: {}", r2);
    
    println!("\nInvalidar cache y recalcular:");
    calc.invalidar_cache();
    let r3 = calc.calcular_costoso();
    println!("Resultado: {}", r3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refcell_borrow() {
        let cell = RefCell::new(5);
        {
            let r = cell.borrow();
            assert_eq!(*r, 5);
        }
    }

    #[test]
    fn test_refcell_borrow_mut() {
        let cell = RefCell::new(5);
        {
            let mut r = cell.borrow_mut();
            *r = 10;
        }
        assert_eq!(*cell.borrow(), 10);
    }

    #[test]
    fn test_contador_compartido() {
        let contador = Contador::new("Test");
        let ref1 = Rc::clone(&contador);
        
        Contador::incrementar(&contador);
        Contador::incrementar(&ref1);
        
        assert_eq!(Contador::obtener_valor(&contador), 2);
    }

    #[test]
    fn test_cache() {
        let calc = Calculadora::new(3);
        
        let r1 = calc.calcular_costoso();
        let r2 = calc.calcular_costoso();
        
        assert_eq!(r1, r2);
        assert_eq!(r1, 18);
    }

    #[test]
    fn test_cache_invalidacion() {
        let calc = Calculadora::new(4);
        
        let _ = calc.calcular_costoso();
        calc.invalidar_cache();
        
        assert!(calc.cache.borrow().is_none());
    }
}
