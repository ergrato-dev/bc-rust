//! Solución - Práctica 03: RefCell

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("=== Práctica 03: RefCell ===\n");

    println!("--- Ejercicio 1: RefCell Básico ---");
    exercise_refcell_basic();

    println!("\n--- Ejercicio 2: Contador Compartido ---");
    exercise_counter();

    println!("\n--- Ejercicio 3: Cache Interno ---");
    exercise_cache();

    println!("\n✅ Todos los ejercicios completados!");
}

// EJERCICIO 1: RefCell Básico

fn exercise_refcell_basic() {
    let data = RefCell::new(vec![1, 2, 3]);
    
    {
        let r = data.borrow();
        println!("Datos: {:?}", *r);
        println!("Longitud: {}", r.len());
    }
    
    {
        let mut r = data.borrow_mut();
        r.push(4);
        r.push(5);
    }
    
    println!("Después de agregar: {:?}", data.borrow());
    
    // Demostrar try_borrow_mut
    let r1 = data.borrow();
    match data.try_borrow_mut() {
        Ok(mut r) => r.push(6),
        Err(e) => println!("Error esperado: {}", e),
    }
    drop(r1);
    
    // Ahora sí funciona
    {
        let mut r = data.borrow_mut();
        r.push(6);
    }
    println!("Final: {:?}", data.borrow());
}

// EJERCICIO 2: Contador Compartido

#[derive(Debug)]
struct Counter {
    value: i32,
    name: String,
}

impl Counter {
    fn new(name: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Counter {
            value: 0,
            name: name.to_string(),
        }))
    }
    
    fn increment(counter: &Rc<RefCell<Self>>) {
        counter.borrow_mut().value += 1;
    }
    
    fn get_value(counter: &Rc<RefCell<Self>>) -> i32 {
        counter.borrow().value
    }
    
    fn get_name(counter: &Rc<RefCell<Self>>) -> String {
        counter.borrow().name.clone()
    }
}

fn exercise_counter() {
    let counter = Counter::new("Principal");
    
    println!("Contador '{}' inicial: {}", 
             Counter::get_name(&counter),
             Counter::get_value(&counter));
    
    let ref1 = Rc::clone(&counter);
    let ref2 = Rc::clone(&counter);
    
    Counter::increment(&ref1);
    Counter::increment(&ref2);
    Counter::increment(&counter);
    
    println!("Valor después de 3 incrementos: {}", Counter::get_value(&counter));
    println!("Strong count: {}", Rc::strong_count(&counter));
}

// EJERCICIO 3: Cache con Interior Mutability

struct Calculator {
    cache: RefCell<Option<i32>>,
    base: i32,
}

impl Calculator {
    fn new(base: i32) -> Self {
        Calculator {
            cache: RefCell::new(None),
            base,
        }
    }
    
    fn expensive_calculate(&self) -> i32 {
        if let Some(value) = *self.cache.borrow() {
            println!("  (usando cache)");
            return value;
        }
        
        println!("  (calculando...)");
        let result = self.base * self.base * 2;
        
        *self.cache.borrow_mut() = Some(result);
        
        result
    }
    
    fn invalidate_cache(&self) {
        *self.cache.borrow_mut() = None;
    }
}

fn exercise_cache() {
    let calc = Calculator::new(5);
    
    println!("Primera llamada:");
    let r1 = calc.expensive_calculate();
    println!("Resultado: {}", r1);
    
    println!("\nSegunda llamada:");
    let r2 = calc.expensive_calculate();
    println!("Resultado: {}", r2);
    
    println!("\nInvalidar cache y recalcular:");
    calc.invalidate_cache();
    let r3 = calc.expensive_calculate();
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
    fn test_shared_counter() {
        let counter = Counter::new("Test");
        let ref1 = Rc::clone(&counter);
        
        Counter::increment(&counter);
        Counter::increment(&ref1);
        
        assert_eq!(Counter::get_value(&counter), 2);
    }

    #[test]
    fn test_cache() {
        let calc = Calculator::new(3);
        
        let r1 = calc.expensive_calculate();
        let r2 = calc.expensive_calculate();
        
        assert_eq!(r1, r2);
        assert_eq!(r1, 18);
    }

    #[test]
    fn test_cache_invalidation() {
        let calc = Calculator::new(4);
        
        let _ = calc.expensive_calculate();
        calc.invalidate_cache();
        
        assert!(calc.cache.borrow().is_none());
    }
}
