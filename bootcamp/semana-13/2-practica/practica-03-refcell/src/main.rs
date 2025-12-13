//! Práctica 03: RefCell<T>
//!
//! En esta práctica aprenderás a:
//! - Usar RefCell para mutabilidad interior
//! - Implementar patrones como Mock objects y Caché
//! - Evitar panics con try_borrow

use std::cell::RefCell;
use std::collections::HashMap;

fn main() {
    println!("=== Práctica 03: RefCell<T> ===\n");

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
// EJERCICIO 1: Contador con RefCell
// ============================================================

struct Contador {
    valor: RefCell<i32>,
}

impl Contador {
    fn new() -> Self {
        // TODO: Crear contador inicializado en 0
        // ↓ Cambia esto por la implementación correcta
        Contador {
            valor: RefCell::new(0),
        }
    }
    
    /// Incrementa el contador (nota: &self, no &mut self)
    fn incrementar(&self) {
        // TODO: Usar borrow_mut() para incrementar
        // ↓ Cambia esto por: *self.valor.borrow_mut() += 1
    }
    
    /// Retorna el valor actual
    fn valor(&self) -> i32 {
        // TODO: Usar borrow() para leer
        // ↓ Cambia esto por: *self.valor.borrow()
        0
    }
}

// ============================================================
// EJERCICIO 2: Mock Messenger para Testing
// ============================================================

trait Mensajero {
    fn enviar(&self, mensaje: &str);
}

struct MockMensajero {
    mensajes: RefCell<Vec<String>>,
}

impl MockMensajero {
    fn new() -> Self {
        // TODO: Crear mock con vector vacío
        // ↓ Completa la implementación
        MockMensajero {
            mensajes: RefCell::new(Vec::new()),
        }
    }
    
    fn mensajes(&self) -> Vec<String> {
        // TODO: Retornar copia de los mensajes
        // ↓ Cambia esto por: self.mensajes.borrow().clone()
        vec![]
    }
}

impl Mensajero for MockMensajero {
    fn enviar(&self, mensaje: &str) {
        // TODO: Agregar mensaje al vector
        // ↓ Completa la implementación
        let _ = mensaje;
    }
}

// ============================================================
// EJERCICIO 3: Caché con RefCell
// ============================================================

struct Calculadora {
    cache: RefCell<HashMap<u64, u64>>,
    hits: RefCell<u32>,
}

impl Calculadora {
    fn new() -> Self {
        // TODO: Crear calculadora con cache vacío
        // ↓ Completa la implementación
        Calculadora {
            cache: RefCell::new(HashMap::new()),
            hits: RefCell::new(0),
        }
    }
    
    /// Calcula factorial con caché
    fn factorial(&self, n: u64) -> u64 {
        // TODO:
        // 1. Verificar si está en caché
        // 2. Si está, incrementar hits y retornar
        // 3. Si no, calcular, guardar en caché y retornar
        // ↓ Cambia esto por la implementación correcta
        let _ = n;
        1
    }
    
    fn cache_hits(&self) -> u32 {
        // TODO: Retornar número de hits
        // ↓ Cambia esto por: *self.hits.borrow()
        0
    }
    
    /// Calcula factorial de forma recursiva (helper privado)
    fn calcular_factorial(n: u64) -> u64 {
        if n <= 1 { 1 } else { n * Self::calcular_factorial(n - 1) }
    }
}

// ============================================================
// EJERCICIO 4: Patrón Observable
// ============================================================

type Callback = Box<dyn Fn(i32)>;

struct Observable {
    valor: RefCell<i32>,
    observers: RefCell<Vec<Callback>>,
}

impl Observable {
    fn new(valor_inicial: i32) -> Self {
        // TODO: Crear observable con valor inicial y sin observers
        // ↓ Completa la implementación
        Observable {
            valor: RefCell::new(valor_inicial),
            observers: RefCell::new(Vec::new()),
        }
    }
    
    /// Suscribe un callback
    fn suscribir(&self, callback: Callback) {
        // TODO: Agregar callback a la lista
        // ↓ Completa la implementación
        let _ = callback;
    }
    
    /// Establece el valor y notifica observers
    fn set_valor(&self, nuevo: i32) {
        // TODO:
        // 1. Actualizar el valor
        // 2. Llamar a todos los observers con el nuevo valor
        // ↓ Completa la implementación
        let _ = nuevo;
    }
    
    fn get_valor(&self) -> i32 {
        *self.valor.borrow()
    }
}

// ============================================================
// TESTS
// ============================================================

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
