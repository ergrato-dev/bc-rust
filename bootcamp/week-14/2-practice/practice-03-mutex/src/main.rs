//! # Práctica 03: Estado Compartido con Mutex
//!
//! En esta práctica aprenderás a:
//! - Usar `Mutex<T>` para exclusión mutua
//! - Combinar `Arc<Mutex<T>>` para compartir entre threads
//! - Usar `RwLock<T>` para múltiples lectores
//! - Evitar deadlocks

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Práctica 03: Mutex y RwLock ===\n");

    // Ejemplo 1: Mutex básico
    ejemplo_mutex_basico();

    // Ejemplo 2: Arc + Mutex
    ejemplo_arc_mutex();

    // Ejemplo 3: RwLock
    ejemplo_rwlock();

    // Ejecuta los ejercicios
    println!("\n=== Ejercicios ===\n");

    // Descomenta para probar tus soluciones:
    // ejercicio_1_contador_concurrente();
    // ejercicio_2_cache_concurrente();
    // ejercicio_3_banco();
}

// ============================================================================
// EJEMPLOS
// ============================================================================

/// Ejemplo: Mutex básico
fn ejemplo_mutex_basico() {
    println!("--- Ejemplo: Mutex Básico ---");

    let datos = Mutex::new(vec![1, 2, 3]);

    // Obtener el lock
    {
        let mut guard = datos.lock().unwrap();
        guard.push(4);
        println!("  Dentro del lock: {:?}", *guard);
        // guard se dropea aquí, liberando el lock
    }

    // Podemos obtener el lock de nuevo
    let guard = datos.lock().unwrap();
    println!("  Después del lock: {:?}", *guard);
    println!();
}

/// Ejemplo: Arc + Mutex para compartir entre threads
fn ejemplo_arc_mutex() {
    println!("--- Ejemplo: Arc + Mutex ---");

    let contador = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let contador_clone = Arc::clone(&contador);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let mut num = contador_clone.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("  Contador final: {}", *contador.lock().unwrap());
    println!();
}

/// Ejemplo: RwLock para múltiples lectores
fn ejemplo_rwlock() {
    println!("--- Ejemplo: RwLock ---");

    let datos = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];

    // Múltiples lectores simultáneos
    for i in 0..3 {
        let datos_clone = Arc::clone(&datos);
        handles.push(thread::spawn(move || {
            let guard = datos_clone.read().unwrap();
            println!("  Lector {}: {:?}", i, *guard);
            thread::sleep(Duration::from_millis(10));
        }));
    }

    // Un escritor (espera a que terminen los lectores)
    let datos_clone = Arc::clone(&datos);
    handles.push(thread::spawn(move || {
        thread::sleep(Duration::from_millis(5));
        let mut guard = datos_clone.write().unwrap();
        guard.push(4);
        println!("  Escritor: agregó 4");
    }));

    for h in handles {
        h.join().unwrap();
    }

    println!("  Final: {:?}", *datos.read().unwrap());
    println!();
}

// ============================================================================
// EJERCICIOS
// ============================================================================

/// # Ejercicio 1: Contador Concurrente
///
/// Implementa un contador thread-safe con operaciones increment, decrement y get.
///
/// ## Requisitos:
/// - `increment()` aumenta el contador en 1
/// - `decrement()` disminuye el contador en 1
/// - `get()` retorna el valor actual
/// - Todas las operaciones deben ser thread-safe
#[allow(dead_code)]
fn ejercicio_1_contador_concurrente() {
    println!("--- Ejercicio 1: Contador Concurrente ---");

    let contador = ContadorConcurrente::new(0);
    let mut handles = vec![];

    // 5 threads incrementan 100 veces cada uno
    for _ in 0..5 {
        let c = contador.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                c.increment();
            }
        }));
    }

    // 2 threads decrementan 50 veces cada uno
    for _ in 0..2 {
        let c = contador.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..50 {
                c.decrement();
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let final_value = contador.get();
    println!("  Valor final: {}", final_value);
    assert_eq!(final_value, 400); // 500 - 100
    println!("✓ Ejercicio 1 completado!\n");
}

#[derive(Clone)]
struct ContadorConcurrente {
    // TODO: Agrega el campo necesario (Arc<Mutex<i32>>)
    valor: Arc<Mutex<i32>>,
}

impl ContadorConcurrente {
    fn new(inicial: i32) -> Self {
        // TODO: Implementa el constructor
        let _ = inicial;
        todo!("Implementa new")
    }

    fn increment(&self) {
        // TODO: Incrementa el valor
        todo!("Implementa increment")
    }

    fn decrement(&self) {
        // TODO: Decrementa el valor
        todo!("Implementa decrement")
    }

    fn get(&self) -> i32 {
        // TODO: Retorna el valor actual
        todo!("Implementa get")
    }
}

/// # Ejercicio 2: Cache Concurrente
///
/// Implementa un cache key-value thread-safe usando RwLock.
///
/// ## Requisitos:
/// - `get()` retorna Option<V> - puede tener múltiples lectores
/// - `set()` inserta o actualiza un valor - exclusivo
/// - `contains()` verifica si existe la key - puede tener múltiples lectores
#[allow(dead_code)]
fn ejercicio_2_cache_concurrente() {
    println!("--- Ejercicio 2: Cache Concurrente ---");

    let cache: CacheConcurrente<String, i32> = CacheConcurrente::new();
    let mut handles = vec![];

    // Writers
    for i in 0..5 {
        let c = cache.clone();
        handles.push(thread::spawn(move || {
            c.set(format!("key{}", i), i * 10);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    // Readers
    let mut handles = vec![];
    for i in 0..5 {
        let c = cache.clone();
        handles.push(thread::spawn(move || {
            let key = format!("key{}", i);
            c.get(&key)
        }));
    }

    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("  Valores leídos: {:?}", results);

    assert!(cache.contains(&"key0".to_string()));
    assert!(!cache.contains(&"noexiste".to_string()));
    println!("✓ Ejercicio 2 completado!\n");
}

#[derive(Clone)]
struct CacheConcurrente<K, V> {
    // TODO: Agrega el campo necesario (Arc<RwLock<HashMap<K, V>>>)
    datos: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> CacheConcurrente<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    fn new() -> Self {
        // TODO: Implementa el constructor
        todo!("Implementa new")
    }

    fn get(&self, key: &K) -> Option<V> {
        // TODO: Usa read() para obtener el valor
        let _ = key;
        todo!("Implementa get")
    }

    fn set(&self, key: K, value: V) {
        // TODO: Usa write() para insertar
        let _ = (key, value);
        todo!("Implementa set")
    }

    fn contains(&self, key: &K) -> bool {
        // TODO: Usa read() para verificar
        let _ = key;
        todo!("Implementa contains")
    }
}

/// # Ejercicio 3: Sistema Bancario
///
/// Implementa un banco con cuentas y transferencias thread-safe.
///
/// ## Requisitos:
/// - `crear_cuenta()` crea una cuenta con saldo inicial
/// - `depositar()` agrega dinero a una cuenta
/// - `retirar()` quita dinero (retorna false si no hay suficiente)
/// - `transferir()` mueve dinero entre cuentas atómicamente
/// - `saldo()` retorna el saldo de una cuenta
#[allow(dead_code)]
fn ejercicio_3_banco() {
    println!("--- Ejercicio 3: Sistema Bancario ---");

    let banco = Banco::new();

    banco.crear_cuenta("Alice", 1000);
    banco.crear_cuenta("Bob", 500);

    let mut handles = vec![];

    // Múltiples transferencias concurrentes
    for _ in 0..10 {
        let b = banco.clone();
        handles.push(thread::spawn(move || {
            b.transferir("Alice", "Bob", 50);
        }));
    }

    for _ in 0..5 {
        let b = banco.clone();
        handles.push(thread::spawn(move || {
            b.transferir("Bob", "Alice", 30);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let saldo_alice = banco.saldo("Alice").unwrap();
    let saldo_bob = banco.saldo("Bob").unwrap();

    println!("  Saldo Alice: {}", saldo_alice);
    println!("  Saldo Bob: {}", saldo_bob);

    // El total debe ser el mismo (1000 + 500 = 1500)
    assert_eq!(saldo_alice + saldo_bob, 1500);
    println!("✓ Ejercicio 3 completado!\n");
}

#[derive(Clone)]
struct Banco {
    // TODO: Agrega el campo necesario
    // Hint: Arc<Mutex<HashMap<String, i32>>> para las cuentas
    cuentas: Arc<Mutex<HashMap<String, i32>>>,
}

impl Banco {
    fn new() -> Self {
        // TODO: Implementa
        todo!("Implementa new")
    }

    fn crear_cuenta(&self, nombre: &str, saldo_inicial: i32) {
        // TODO: Implementa
        let _ = (nombre, saldo_inicial);
        todo!("Implementa crear_cuenta")
    }

    fn depositar(&self, nombre: &str, cantidad: i32) -> bool {
        // TODO: Implementa
        let _ = (nombre, cantidad);
        todo!("Implementa depositar")
    }

    fn retirar(&self, nombre: &str, cantidad: i32) -> bool {
        // TODO: Retorna false si no hay suficiente saldo
        let _ = (nombre, cantidad);
        todo!("Implementa retirar")
    }

    fn transferir(&self, desde: &str, hacia: &str, cantidad: i32) -> bool {
        // TODO: Implementa transferencia atómica
        // IMPORTANTE: Usar un solo lock para evitar deadlocks
        let _ = (desde, hacia, cantidad);
        todo!("Implementa transferir")
    }

    fn saldo(&self, nombre: &str) -> Option<i32> {
        // TODO: Implementa
        let _ = nombre;
        todo!("Implementa saldo")
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contador_increment() {
        let c = ContadorConcurrente::new(0);
        c.increment();
        c.increment();
        assert_eq!(c.get(), 2);
    }

    #[test]
    fn test_contador_decrement() {
        let c = ContadorConcurrente::new(10);
        c.decrement();
        c.decrement();
        assert_eq!(c.get(), 8);
    }

    #[test]
    fn test_contador_concurrente() {
        let c = ContadorConcurrente::new(0);
        let mut handles = vec![];

        for _ in 0..10 {
            let c_clone = c.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    c_clone.increment();
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(c.get(), 1000);
    }

    #[test]
    fn test_cache_set_get() {
        let cache: CacheConcurrente<String, i32> = CacheConcurrente::new();
        cache.set("key1".to_string(), 42);
        assert_eq!(cache.get(&"key1".to_string()), Some(42));
    }

    #[test]
    fn test_cache_contains() {
        let cache: CacheConcurrente<String, i32> = CacheConcurrente::new();
        cache.set("exists".to_string(), 1);
        assert!(cache.contains(&"exists".to_string()));
        assert!(!cache.contains(&"not_exists".to_string()));
    }

    #[test]
    fn test_cache_update() {
        let cache: CacheConcurrente<String, i32> = CacheConcurrente::new();
        cache.set("key".to_string(), 1);
        cache.set("key".to_string(), 2);
        assert_eq!(cache.get(&"key".to_string()), Some(2));
    }

    #[test]
    fn test_banco_crear_cuenta() {
        let banco = Banco::new();
        banco.crear_cuenta("Test", 100);
        assert_eq!(banco.saldo("Test"), Some(100));
    }

    #[test]
    fn test_banco_depositar() {
        let banco = Banco::new();
        banco.crear_cuenta("Test", 100);
        banco.depositar("Test", 50);
        assert_eq!(banco.saldo("Test"), Some(150));
    }

    #[test]
    fn test_banco_retirar() {
        let banco = Banco::new();
        banco.crear_cuenta("Test", 100);
        assert!(banco.retirar("Test", 50));
        assert_eq!(banco.saldo("Test"), Some(50));
        assert!(!banco.retirar("Test", 100)); // No hay suficiente
    }

    #[test]
    fn test_banco_transferir() {
        let banco = Banco::new();
        banco.crear_cuenta("A", 100);
        banco.crear_cuenta("B", 50);
        assert!(banco.transferir("A", "B", 30));
        assert_eq!(banco.saldo("A"), Some(70));
        assert_eq!(banco.saldo("B"), Some(80));
    }

    #[test]
    fn test_banco_transferir_insuficiente() {
        let banco = Banco::new();
        banco.crear_cuenta("A", 20);
        banco.crear_cuenta("B", 50);
        assert!(!banco.transferir("A", "B", 100));
        assert_eq!(banco.saldo("A"), Some(20));
        assert_eq!(banco.saldo("B"), Some(50));
    }
}
