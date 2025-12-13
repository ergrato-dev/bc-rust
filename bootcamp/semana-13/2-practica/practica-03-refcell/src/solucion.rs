//! Solución - Práctica 03: RefCell<T>

use std::cell::RefCell;
use std::collections::HashMap;

fn main() {
    println!("=== Práctica 03: RefCell<T> - SOLUCIÓN ===\n");

    // Ejercicio 1: Contador con RefCell
    println!("--- Ejercicio 1: Contador ---");
    let contador = Contador::new();
    
    contador.incrementar();
    contador.incrementar();
    contador.incrementar();
    
    println!("Valor: {}", contador.valor());
    
    // Ejercicio 2: Mock Messenger
    println!("\n--- Ejercicio 2: Mock Messenger ---");
    let mock = MockMensajero::new();
    
    mock.enviar("Hola");
    mock.enviar("Mundo");
    
    println!("Mensajes enviados: {:?}", mock.mensajes());
    
    // Ejercicio 3: Caché
    println!("\n--- Ejercicio 3: Caché ---");
    let calculadora = Calculadora::new();
    
    println!("Factorial 5: {}", calculadora.factorial(5));
    println!("Factorial 5 (cached): {}", calculadora.factorial(5));
    println!("Factorial 3: {}", calculadora.factorial(3));
    
    println!("Cache hits: {}", calculadora.cache_hits());
    
    // Ejercicio 4: Observable
    println!("\n--- Ejercicio 4: Observable ---");
    let observable = Observable::new(0);
    
    observable.suscribir(Box::new(|v| println!("  Observer 1: {}", v)));
    observable.suscribir(Box::new(|v| println!("  Observer 2: valor * 2 = {}", v * 2)));
    
    println!("Estableciendo valor a 42:");
    observable.set_valor(42);
    
    println!("\n✅ Práctica completada!");
}

// ============================================================
// EJERCICIO 1: Contador con RefCell - SOLUCIÓN
// ============================================================

struct Contador {
    valor: RefCell<i32>,
}

impl Contador {
    fn new() -> Self {
        Contador {
            valor: RefCell::new(0),
        }
    }
    
    fn incrementar(&self) {
        *self.valor.borrow_mut() += 1;
    }
    
    fn valor(&self) -> i32 {
        *self.valor.borrow()
    }
}

// ============================================================
// EJERCICIO 2: Mock Messenger - SOLUCIÓN
// ============================================================

trait Mensajero {
    fn enviar(&self, mensaje: &str);
}

struct MockMensajero {
    mensajes: RefCell<Vec<String>>,
}

impl MockMensajero {
    fn new() -> Self {
        MockMensajero {
            mensajes: RefCell::new(Vec::new()),
        }
    }
    
    fn mensajes(&self) -> Vec<String> {
        self.mensajes.borrow().clone()
    }
}

impl Mensajero for MockMensajero {
    fn enviar(&self, mensaje: &str) {
        self.mensajes.borrow_mut().push(mensaje.to_string());
    }
}

// ============================================================
// EJERCICIO 3: Caché con RefCell - SOLUCIÓN
// ============================================================

struct Calculadora {
    cache: RefCell<HashMap<u64, u64>>,
    hits: RefCell<u32>,
}

impl Calculadora {
    fn new() -> Self {
        Calculadora {
            cache: RefCell::new(HashMap::new()),
            hits: RefCell::new(0),
        }
    }
    
    fn factorial(&self, n: u64) -> u64 {
        // Verificar caché
        if let Some(&resultado) = self.cache.borrow().get(&n) {
            *self.hits.borrow_mut() += 1;
            return resultado;
        }
        
        // Calcular
        let resultado = Self::calcular_factorial(n);
        
        // Guardar en caché
        self.cache.borrow_mut().insert(n, resultado);
        
        resultado
    }
    
    fn cache_hits(&self) -> u32 {
        *self.hits.borrow()
    }
    
    fn calcular_factorial(n: u64) -> u64 {
        if n <= 1 { 1 } else { n * Self::calcular_factorial(n - 1) }
    }
}

// ============================================================
// EJERCICIO 4: Patrón Observable - SOLUCIÓN
// ============================================================

type Callback = Box<dyn Fn(i32)>;

struct Observable {
    valor: RefCell<i32>,
    observers: RefCell<Vec<Callback>>,
}

impl Observable {
    fn new(valor_inicial: i32) -> Self {
        Observable {
            valor: RefCell::new(valor_inicial),
            observers: RefCell::new(Vec::new()),
        }
    }
    
    fn suscribir(&self, callback: Callback) {
        self.observers.borrow_mut().push(callback);
    }
    
    fn set_valor(&self, nuevo: i32) {
        *self.valor.borrow_mut() = nuevo;
        
        for observer in self.observers.borrow().iter() {
            observer(nuevo);
        }
    }
    
    #[allow(dead_code)]
    fn get_valor(&self) -> i32 {
        *self.valor.borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contador() {
        let c = Contador::new();
        assert_eq!(c.valor(), 0);
        
        c.incrementar();
        assert_eq!(c.valor(), 1);
        
        c.incrementar();
        c.incrementar();
        assert_eq!(c.valor(), 3);
    }
    
    #[test]
    fn test_mock_mensajero() {
        let mock = MockMensajero::new();
        
        mock.enviar("Hola");
        mock.enviar("Mundo");
        
        let msgs = mock.mensajes();
        assert_eq!(msgs.len(), 2);
        assert_eq!(msgs[0], "Hola");
        assert_eq!(msgs[1], "Mundo");
    }
    
    #[test]
    fn test_calculadora_factorial() {
        let calc = Calculadora::new();
        
        assert_eq!(calc.factorial(0), 1);
        assert_eq!(calc.factorial(1), 1);
        assert_eq!(calc.factorial(5), 120);
        assert_eq!(calc.factorial(10), 3628800);
    }
    
    #[test]
    fn test_calculadora_cache() {
        let calc = Calculadora::new();
        
        calc.factorial(5);
        assert_eq!(calc.cache_hits(), 0);
        
        calc.factorial(5);
        assert_eq!(calc.cache_hits(), 1);
        
        calc.factorial(5);
        assert_eq!(calc.cache_hits(), 2);
    }
    
    #[test]
    fn test_observable() {
        let obs = Observable::new(0);
        let resultado = RefCell::new(0);
        
        let resultado_clone = resultado.clone();
        obs.suscribir(Box::new(move |v| {
            *resultado_clone.borrow_mut() = v;
        }));
        
        obs.set_valor(42);
        assert_eq!(*resultado.borrow(), 42);
    }
}
