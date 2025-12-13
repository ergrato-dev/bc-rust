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
    example_basic_mutex();

    // Ejemplo 2: Arc + Mutex
    example_arc_mutex();

    // Ejemplo 3: RwLock
    example_rwlock();

    // Ejecuta los ejercicios
    println!("\n=== Ejercicios ===\n");

    // Descomenta para probar tus soluciones:
    // exercise_1_concurrent_counter();
    // exercise_2_concurrent_cache();
    // exercise_3_bank();
}

// ============================================================================
// EJEMPLOS
// ============================================================================

/// Ejemplo: Mutex básico
fn example_basic_mutex() {
    println!("--- Ejemplo: Mutex Básico ---");

    let data = Mutex::new(vec![1, 2, 3]);

    // Obtener el lock
    {
        let mut guard = data.lock().unwrap();
        guard.push(4);
        println!("  Dentro del lock: {:?}", *guard);
        // guard se dropea aquí, liberando el lock
    }

    // Podemos obtener el lock de nuevo
    let guard = data.lock().unwrap();
    println!("  Después del lock: {:?}", *guard);
    println!();
}

/// Ejemplo: Arc + Mutex para compartir entre threads
fn example_arc_mutex() {
    println!("--- Ejemplo: Arc + Mutex ---");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("  Contador final: {}", *counter.lock().unwrap());
    println!();
}

/// Ejemplo: RwLock para múltiples lectores
fn example_rwlock() {
    println!("--- Ejemplo: RwLock ---");

    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];

    // Múltiples lectores simultáneos
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let guard = data_clone.read().unwrap();
            println!("  Lector {}: {:?}", i, *guard);
            thread::sleep(Duration::from_millis(10));
        }));
    }

    // Un escritor (espera a que terminen los lectores)
    let data_clone = Arc::clone(&data);
    handles.push(thread::spawn(move || {
        thread::sleep(Duration::from_millis(5));
        let mut guard = data_clone.write().unwrap();
        guard.push(4);
        println!("  Escritor: agregó 4");
    }));

    for h in handles {
        h.join().unwrap();
    }

    println!("  Final: {:?}", *data.read().unwrap());
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
fn exercise_1_concurrent_counter() {
    println!("--- Ejercicio 1: Contador Concurrente ---");

    let counter = ConcurrentCounter::new(0);
    let mut handles = vec![];

    // 5 threads incrementan 100 veces cada uno
    for _ in 0..5 {
        let c = counter.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                c.increment();
            }
        }));
    }

    // 2 threads decrementan 50 veces cada uno
    for _ in 0..2 {
        let c = counter.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..50 {
                c.decrement();
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let final_value = counter.get();
    println!("  Valor final: {}", final_value);
    assert_eq!(final_value, 400); // 500 - 100
    println!("✓ Ejercicio 1 completado!\n");
}

#[derive(Clone)]
struct ConcurrentCounter {
    // TODO: Agrega el campo necesario (Arc<Mutex<i32>>)
    value: Arc<Mutex<i32>>,
}

impl ConcurrentCounter {
    fn new(initial: i32) -> Self {
        // TODO: Implementa el constructor
        let _ = initial;
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
fn exercise_2_concurrent_cache() {
    println!("--- Ejercicio 2: Cache Concurrente ---");

    let cache: ConcurrentCache<String, i32> = ConcurrentCache::new();
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
struct ConcurrentCache<K, V> {
    // TODO: Agrega el campo necesario (Arc<RwLock<HashMap<K, V>>>)
    data: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> ConcurrentCache<K, V>
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
fn exercise_3_bank() {
    println!("--- Ejercicio 3: Sistema Bancario ---");

    let bank = Bank::new();

    bank.create_account("Alice", 1000);
    bank.create_account("Bob", 500);

    let mut handles = vec![];

    // Múltiples transferencias concurrentes
    for _ in 0..10 {
        let b = bank.clone();
        handles.push(thread::spawn(move || {
            b.transfer("Alice", "Bob", 50);
        }));
    }

    for _ in 0..5 {
        let b = bank.clone();
        handles.push(thread::spawn(move || {
            b.transfer("Bob", "Alice", 30);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let balance_alice = bank.balance("Alice").unwrap();
    let balance_bob = bank.balance("Bob").unwrap();

    println!("  Saldo Alice: {}", balance_alice);
    println!("  Saldo Bob: {}", balance_bob);

    // El total debe ser el mismo (1000 + 500 = 1500)
    assert_eq!(balance_alice + balance_bob, 1500);
    println!("✓ Ejercicio 3 completado!\n");
}

#[derive(Clone)]
struct Bank {
    // TODO: Agrega el campo necesario
    // Hint: Arc<Mutex<HashMap<String, i32>>> para las cuentas
    accounts: Arc<Mutex<HashMap<String, i32>>>,
}

impl Bank {
    fn new() -> Self {
        // TODO: Implementa
        todo!("Implementa new")
    }

    fn create_account(&self, name: &str, initial_balance: i32) {
        // TODO: Implementa
        let _ = (name, initial_balance);
        todo!("Implementa create_account")
    }

    fn deposit(&self, name: &str, amount: i32) -> bool {
        // TODO: Implementa
        let _ = (name, amount);
        todo!("Implementa deposit")
    }

    fn withdraw(&self, name: &str, amount: i32) -> bool {
        // TODO: Retorna false si no hay suficiente saldo
        let _ = (name, amount);
        todo!("Implementa withdraw")
    }

    fn transfer(&self, from: &str, to: &str, amount: i32) -> bool {
        // TODO: Implementa transferencia atómica
        // IMPORTANTE: Usar un solo lock para evitar deadlocks
        let _ = (from, to, amount);
        todo!("Implementa transfer")
    }

    fn balance(&self, name: &str) -> Option<i32> {
        // TODO: Implementa
        let _ = name;
        todo!("Implementa balance")
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_increment() {
        let c = ConcurrentCounter::new(0);
        c.increment();
        c.increment();
        assert_eq!(c.get(), 2);
    }

    #[test]
    fn test_counter_decrement() {
        let c = ConcurrentCounter::new(10);
        c.decrement();
        c.decrement();
        assert_eq!(c.get(), 8);
    }

    #[test]
    fn test_counter_concurrent() {
        let c = ConcurrentCounter::new(0);
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
        let cache: ConcurrentCache<String, i32> = ConcurrentCache::new();
        cache.set("key1".to_string(), 42);
        assert_eq!(cache.get(&"key1".to_string()), Some(42));
    }

    #[test]
    fn test_cache_contains() {
        let cache: ConcurrentCache<String, i32> = ConcurrentCache::new();
        cache.set("exists".to_string(), 1);
        assert!(cache.contains(&"exists".to_string()));
        assert!(!cache.contains(&"not_exists".to_string()));
    }

    #[test]
    fn test_cache_update() {
        let cache: ConcurrentCache<String, i32> = ConcurrentCache::new();
        cache.set("key".to_string(), 1);
        cache.set("key".to_string(), 2);
        assert_eq!(cache.get(&"key".to_string()), Some(2));
    }

    #[test]
    fn test_bank_create_account() {
        let bank = Bank::new();
        bank.create_account("Test", 100);
        assert_eq!(bank.balance("Test"), Some(100));
    }

    #[test]
    fn test_bank_deposit() {
        let bank = Bank::new();
        bank.create_account("Test", 100);
        bank.deposit("Test", 50);
        assert_eq!(bank.balance("Test"), Some(150));
    }

    #[test]
    fn test_bank_withdraw() {
        let bank = Bank::new();
        bank.create_account("Test", 100);
        assert!(bank.withdraw("Test", 50));
        assert_eq!(bank.balance("Test"), Some(50));
        assert!(!bank.withdraw("Test", 100)); // No hay suficiente
    }

    #[test]
    fn test_bank_transfer() {
        let bank = Bank::new();
        bank.create_account("A", 100);
        bank.create_account("B", 50);
        assert!(bank.transfer("A", "B", 30));
        assert_eq!(bank.balance("A"), Some(70));
        assert_eq!(bank.balance("B"), Some(80));
    }

    #[test]
    fn test_bank_transfer_insufficient() {
        let bank = Bank::new();
        bank.create_account("A", 20);
        bank.create_account("B", 50);
        assert!(!bank.transfer("A", "B", 100));
        assert_eq!(bank.balance("A"), Some(20));
        assert_eq!(bank.balance("B"), Some(50));
    }
}
