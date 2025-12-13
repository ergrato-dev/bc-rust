//! # Práctica 04: Trait Bounds
//!
//! Aprende a usar trait bounds para restringir tipos genéricos.

use std::fmt::{Debug, Display};
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::hash::Hash;

// ============================================
// EJERCICIO 1: Bounds Básicos
// ============================================
//
// Funciones genéricas con trait bounds simples

/// Imprime cualquier tipo que implemente Display
fn print_value<T: Display>(value: T) {
    println!("{}", value);
}

/// Imprime con formato debug
fn print_debug<T: Debug>(value: T) {
    println!("{:?}", value);
}

/// Encuentra el mayor de dos valores
fn larger<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

/// Encuentra el menor de dos valores
fn smaller<T: PartialOrd>(a: T, b: T) -> T {
    if a <= b { a } else { b }
}

/// Duplica un valor clonable
fn duplicate<T: Clone>(value: &T) -> (T, T) {
    (value.clone(), value.clone())
}

// ============================================
// EJERCICIO 2: Múltiples Bounds
// ============================================
//
// Combina varios traits con +

/// Imprime en ambos formatos
fn print_both<T: Display + Debug>(value: T) {
    println!("Display: {}", value);
    println!("Debug: {:?}", value);
}

/// Encuentra el mayor en una lista y lo imprime
fn largest_in_list<T: PartialOrd + Display + Clone>(list: &[T]) -> Option<T> {
    if list.is_empty() {
        return None;
    }
    
    let mut max = list[0].clone();
    for item in list.iter() {
        if item > &max {
            max = item.clone();
        }
    }
    Some(max)
}

/// Ordena y devuelve una copia
fn sort_copy<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    let mut copy = list.to_vec();
    copy.sort();
    copy
}

/// Cuenta elementos únicos
fn count_unique<T: Eq + Hash + Clone>(list: &[T]) -> usize {
    let set: std::collections::HashSet<T> = list.iter().cloned().collect();
    set.len()
}

// ============================================
// EJERCICIO 3: Where Clauses
// ============================================
//
// Usa where para bounds complejos

/// Procesa dos tipos diferentes
fn process<T, U>(t: T, u: U) -> String
where
    T: Display + Clone,
    U: Debug + Default,
{
    format!("T: {}, U: {:?}", t, u)
}

/// Contenedor genérico con bounds
struct Container<T>
where
    T: Clone + Debug,
{
    value: T,
    history: Vec<T>,
}

impl<T> Container<T>
where
    T: Clone + Debug,
{
    fn new(value: T) -> Self {
        Container {
            value: value.clone(),
            history: vec![value],
        }
    }
    
    fn update(&mut self, new_value: T) {
        self.history.push(self.value.clone());
        self.value = new_value;
    }
    
    fn get(&self) -> &T {
        &self.value
    }
    
    fn history(&self) -> &[T] {
        &self.history
    }
}

/// Cache genérico
struct Cache<K, V>
where
    K: Eq + Hash + Clone + Display,
    V: Clone + Debug,
{
    data: HashMap<K, V>,
    hits: u32,
    misses: u32,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone + Display,
    V: Clone + Debug,
{
    fn new() -> Self {
        Cache {
            data: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }
    
    fn insert(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }
    
    fn get(&mut self, key: &K) -> Option<V> {
        match self.data.get(key) {
            Some(v) => {
                self.hits += 1;
                Some(v.clone())
            }
            None => {
                self.misses += 1;
                None
            }
        }
    }
    
    fn stats(&self) -> (u32, u32) {
        (self.hits, self.misses)
    }
}

// ============================================
// EJERCICIO 4: impl Trait
// ============================================
//
// Usa impl Trait como parámetro y retorno

/// Acepta cualquier tipo Display
fn show(item: impl Display) {
    println!("Mostrando: {}", item);
}

/// Acepta cualquier iterador de i32
fn sum_iterator(iter: impl Iterator<Item = i32>) -> i32 {
    iter.sum()
}

/// Retorna un iterador (tipo oculto)
fn create_range(start: i32, end: i32) -> impl Iterator<Item = i32> {
    start..end
}

/// Retorna un iterador con filtro
fn even_numbers(up_to: i32) -> impl Iterator<Item = i32> {
    (0..up_to).filter(|n| n % 2 == 0)
}

/// Retorna un iterador mapeado
fn squares(up_to: i32) -> impl Iterator<Item = i32> {
    (1..=up_to).map(|n| n * n)
}

/// Acepta cualquier cosa que se pueda convertir a String
fn process_text(text: impl Into<String>) -> String {
    let s: String = text.into();
    s.to_uppercase()
}

/// Función que acepta múltiples bounds con impl
fn analyze(item: impl Display + Debug + Clone) -> String {
    let clone = item.clone();
    format!("Display: {} | Debug: {:?}", item, clone)
}

fn main() {
    println!("=== Práctica 04: Trait Bounds ===\n");
    
    // Ejercicio 1: Bounds Básicos
    println!("--- Ejercicio 1: Bounds Básicos ---");
    print_value(42);
    print_value("Hola mundo");
    
    print_debug(vec![1, 2, 3]);
    
    println!("Larger of 10 and 20: {}", larger(10, 20));
    println!("Smaller of 'a' and 'z': {}", smaller('a', 'z'));
    
    let (a, b) = duplicate(&String::from("test"));
    println!("Duplicated: ({}, {})", a, b);
    
    // Ejercicio 2: Múltiples Bounds
    println!("\n--- Ejercicio 2: Múltiples Bounds ---");
    print_both(42);
    
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    if let Some(max) = largest_in_list(&numbers) {
        println!("Largest in list: {}", max);
    }
    
    let sorted = sort_copy(&numbers);
    println!("Sorted: {:?}", sorted);
    
    let with_duplicates = vec![1, 2, 2, 3, 3, 3, 4];
    println!("Unique elements: {}", count_unique(&with_duplicates));
    
    // Ejercicio 3: Where Clauses
    println!("\n--- Ejercicio 3: Where Clauses ---");
    let result = process("Hola", vec![1, 2, 3]);
    println!("Processed: {}", result);
    
    let mut container = Container::new(100);
    container.update(200);
    container.update(300);
    println!("Current value: {:?}", container.get());
    println!("History: {:?}", container.history());
    
    let mut cache: Cache<String, i32> = Cache::new();
    cache.insert(String::from("uno"), 1);
    cache.insert(String::from("dos"), 2);
    
    println!("Cache 'uno': {:?}", cache.get(&String::from("uno")));
    println!("Cache 'tres': {:?}", cache.get(&String::from("tres")));
    println!("Stats (hits, misses): {:?}", cache.stats());
    
    // Ejercicio 4: impl Trait
    println!("\n--- Ejercicio 4: impl Trait ---");
    show(42);
    show("texto");
    
    let sum = sum_iterator(vec![1, 2, 3, 4, 5].into_iter());
    println!("Iterator sum: {}", sum);
    
    let range: Vec<i32> = create_range(1, 5).collect();
    println!("Range: {:?}", range);
    
    let evens: Vec<i32> = even_numbers(10).collect();
    println!("Even numbers up to 10: {:?}", evens);
    
    let sqs: Vec<i32> = squares(5).collect();
    println!("Squares 1-5: {:?}", sqs);
    
    println!("Processed text: {}", process_text("hola"));
    println!("Processed text: {}", process_text(String::from("mundo")));
    
    println!("Analysis: {}", analyze(42));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Tests Ejercicio 1: Bounds Básicos
    #[test]
    fn test_larger_integers() {
        assert_eq!(larger(5, 10), 10);
        assert_eq!(larger(10, 5), 10);
    }
    
    #[test]
    fn test_larger_strings() {
        assert_eq!(larger("apple", "banana"), "banana");
    }
    
    #[test]
    fn test_smaller_integers() {
        assert_eq!(smaller(5, 10), 5);
        assert_eq!(smaller(10, 5), 5);
    }
    
    #[test]
    fn test_duplicate() {
        let (a, b) = duplicate(&42);
        assert_eq!(a, 42);
        assert_eq!(b, 42);
    }
    
    #[test]
    fn test_duplicate_string() {
        let (a, b) = duplicate(&String::from("test"));
        assert_eq!(a, "test");
        assert_eq!(b, "test");
    }
    
    // Tests Ejercicio 2: Múltiples Bounds
    #[test]
    fn test_largest_in_list() {
        let list = vec![1, 5, 3, 9, 2];
        assert_eq!(largest_in_list(&list), Some(9));
    }
    
    #[test]
    fn test_largest_in_empty_list() {
        let list: Vec<i32> = vec![];
        assert_eq!(largest_in_list(&list), None);
    }
    
    #[test]
    fn test_sort_copy() {
        let list = vec![3, 1, 4, 1, 5];
        let sorted = sort_copy(&list);
        assert_eq!(sorted, vec![1, 1, 3, 4, 5]);
    }
    
    #[test]
    fn test_count_unique() {
        let list = vec![1, 2, 2, 3, 3, 3];
        assert_eq!(count_unique(&list), 3);
    }
    
    #[test]
    fn test_count_unique_strings() {
        let list = vec!["a", "b", "a", "c", "b"];
        assert_eq!(count_unique(&list), 3);
    }
    
    // Tests Ejercicio 3: Where Clauses
    #[test]
    fn test_process() {
        let result = process("test", 42);
        assert!(result.contains("test"));
        assert!(result.contains("42"));
    }
    
    #[test]
    fn test_container_new() {
        let cont = Container::new(100);
        assert_eq!(*cont.get(), 100);
    }
    
    #[test]
    fn test_container_update() {
        let mut cont = Container::new(1);
        cont.update(2);
        cont.update(3);
        assert_eq!(*cont.get(), 3);
        assert_eq!(cont.history().len(), 3);
    }
    
    #[test]
    fn test_cache_insert_get() {
        let mut cache: Cache<String, i32> = Cache::new();
        cache.insert(String::from("key"), 42);
        assert_eq!(cache.get(&String::from("key")), Some(42));
    }
    
    #[test]
    fn test_cache_miss() {
        let mut cache: Cache<String, i32> = Cache::new();
        assert_eq!(cache.get(&String::from("noexiste")), None);
    }
    
    #[test]
    fn test_cache_stats() {
        let mut cache: Cache<String, i32> = Cache::new();
        cache.insert(String::from("a"), 1);
        cache.get(&String::from("a")); // hit
        cache.get(&String::from("b")); // miss
        assert_eq!(cache.stats(), (1, 1));
    }
    
    // Tests Ejercicio 4: impl Trait
    #[test]
    fn test_sum_iterator() {
        let sum = sum_iterator(vec![1, 2, 3].into_iter());
        assert_eq!(sum, 6);
    }
    
    #[test]
    fn test_create_range() {
        let range: Vec<i32> = create_range(1, 4).collect();
        assert_eq!(range, vec![1, 2, 3]);
    }
    
    #[test]
    fn test_even_numbers() {
        let evens: Vec<i32> = even_numbers(8).collect();
        assert_eq!(evens, vec![0, 2, 4, 6]);
    }
    
    #[test]
    fn test_squares() {
        let sqs: Vec<i32> = squares(4).collect();
        assert_eq!(sqs, vec![1, 4, 9, 16]);
    }
    
    #[test]
    fn test_process_text_str() {
        assert_eq!(process_text("hola"), "HOLA");
    }
    
    #[test]
    fn test_process_text_string() {
        assert_eq!(process_text(String::from("mundo")), "MUNDO");
    }
    
    #[test]
    fn test_analyze() {
        let result = analyze(42);
        assert!(result.contains("42"));
    }
}
