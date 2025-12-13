//! Práctica 03: RefCell - Interior Mutability
//!
//! En esta práctica aprenderás a:
//! - Usar RefCell para mutabilidad interior
//! - Combinar Rc<RefCell<T>> para datos mutables compartidos
//! - Manejar errores de borrowing en runtime

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("=== Práctica 03: RefCell ===\n");

    // Ejercicio 1: RefCell básico
    println!("--- Ejercicio 1: RefCell Básico ---");
    exercise_refcell_basic();

    // Ejercicio 2: Contador compartido
    println!("\n--- Ejercicio 2: Contador Compartido ---");
    exercise_counter();

    // Ejercicio 3: Cache con RefCell
    println!("\n--- Ejercicio 3: Cache Interno ---");
    exercise_cache();

    println!("\n✅ Todos los ejercicios completados!");
}

// ============================================================
// EJERCICIO 1: RefCell Básico
// ============================================================

fn exercise_refcell_basic() {
    let data = RefCell::new(vec![1, 2, 3]);
    
    // Borrow inmutable
    {
        let r = data.borrow();
        println!("Datos: {:?}", *r);
        println!("Longitud: {}", r.len());
    } // r se libera aquí
    
    // ↓ Borrow mutable para agregar elementos
    {
        let mut r = data.borrow_mut();
        r.push(4);
        r.push(5);
    }
    
    println!("Después de agregar: {:?}", data.borrow());
    
    // ↓ Intenta usar try_borrow_mut para evitar panic
    // let r1 = data.borrow();
    // match data.try_borrow_mut() {
    //     Ok(mut r) => r.push(6),
    //     Err(e) => println!("Error: {}", e),
    // }
}

// ============================================================
// EJERCICIO 2: Contador Compartido con Rc<RefCell<T>>
// ============================================================

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
        // ↓ Implementa: incrementa el valor en 1
        counter.borrow_mut().value += 1;
    }
    
    fn get_value(counter: &Rc<RefCell<Self>>) -> i32 {
        // ↓ Implementa: retorna el valor actual
        counter.borrow().value
    }
}

fn exercise_counter() {
    let counter = Counter::new("Principal");
    
    println!("Valor inicial: {}", Counter::get_value(&counter));
    
    // Crear múltiples "referencias" al contador
    let ref1 = Rc::clone(&counter);
    let ref2 = Rc::clone(&counter);
    
    // Cada referencia puede incrementar
    Counter::increment(&ref1);
    Counter::increment(&ref2);
    Counter::increment(&counter);
    
    println!("Valor después de 3 incrementos: {}", Counter::get_value(&counter));
    println!("Strong count: {}", Rc::strong_count(&counter));
}

// ============================================================
// EJERCICIO 3: Cache con Interior Mutability
// ============================================================

struct Calculator {
    // Cache interno que se puede mutar aunque Calculadora sea inmutable
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
    
    // Método &self que puede mutar el cache internamente
    fn expensive_calculate(&self) -> i32 {
        // ↓ Implementa: si hay cache, retornarlo; si no, calcular y guardar
        if let Some(value) = *self.cache.borrow() {
            println!("  (usando cache)");
            return value;
        }
        
        println!("  (calculando...)");
        // Simulamos cálculo costoso
        let result = self.base * self.base * 2;
        
        // Guardar en cache
        *self.cache.borrow_mut() = Some(result);
        
        result
    }
    
    fn invalidate_cache(&self) {
        // ↓ Implementa: limpia el cache
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

// ============================================================
// TESTS
// ============================================================

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
        assert_eq!(r1, 18); // 3 * 3 * 2
    }

    #[test]
    fn test_cache_invalidation() {
        let calc = Calculator::new(4);
        
        let _ = calc.expensive_calculate();
        calc.invalidate_cache();
        
        assert!(calc.cache.borrow().is_none());
    }
}
