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
fn imprimir<T: Display>(valor: T) {
    println!("{}", valor);
}

/// Imprime con formato debug
fn debug<T: Debug>(valor: T) {
    println!("{:?}", valor);
}

/// Encuentra el mayor de dos valores
fn mayor<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

/// Encuentra el menor de dos valores
fn menor<T: PartialOrd>(a: T, b: T) -> T {
    if a <= b { a } else { b }
}

/// Duplica un valor clonable
fn duplicar<T: Clone>(valor: &T) -> (T, T) {
    (valor.clone(), valor.clone())
}

// ============================================
// EJERCICIO 2: Múltiples Bounds
// ============================================
//
// Combina varios traits con +

/// Imprime en ambos formatos
fn imprimir_ambos<T: Display + Debug>(valor: T) {
    println!("Display: {}", valor);
    println!("Debug: {:?}", valor);
}

/// Encuentra el mayor en una lista y lo imprime
fn mayor_en_lista<T: PartialOrd + Display + Clone>(lista: &[T]) -> Option<T> {
    if lista.is_empty() {
        return None;
    }
    
    let mut max = lista[0].clone();
    for item in lista.iter() {
        if item > &max {
            max = item.clone();
        }
    }
    Some(max)
}

/// Ordena y devuelve una copia
fn ordenar_copia<T: Ord + Clone>(lista: &[T]) -> Vec<T> {
    let mut copia = lista.to_vec();
    copia.sort();
    copia
}

/// Cuenta elementos únicos
fn contar_unicos<T: Eq + Hash + Clone>(lista: &[T]) -> usize {
    let set: std::collections::HashSet<T> = lista.iter().cloned().collect();
    set.len()
}

// ============================================
// EJERCICIO 3: Where Clauses
// ============================================
//
// Usa where para bounds complejos

/// Procesa dos tipos diferentes
fn procesar<T, U>(t: T, u: U) -> String
where
    T: Display + Clone,
    U: Debug + Default,
{
    format!("T: {}, U: {:?}", t, u)
}

/// Contenedor genérico con bounds
struct Contenedor<T>
where
    T: Clone + Debug,
{
    valor: T,
    historial: Vec<T>,
}

impl<T> Contenedor<T>
where
    T: Clone + Debug,
{
    fn new(valor: T) -> Self {
        Contenedor {
            valor: valor.clone(),
            historial: vec![valor],
        }
    }
    
    fn actualizar(&mut self, nuevo: T) {
        self.historial.push(self.valor.clone());
        self.valor = nuevo;
    }
    
    fn obtener(&self) -> &T {
        &self.valor
    }
    
    fn historial(&self) -> &[T] {
        &self.historial
    }
}

/// Cache genérico
struct Cache<K, V>
where
    K: Eq + Hash + Clone + Display,
    V: Clone + Debug,
{
    datos: HashMap<K, V>,
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
            datos: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }
    
    fn insertar(&mut self, clave: K, valor: V) {
        self.datos.insert(clave, valor);
    }
    
    fn obtener(&mut self, clave: &K) -> Option<V> {
        match self.datos.get(clave) {
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
    
    fn estadisticas(&self) -> (u32, u32) {
        (self.hits, self.misses)
    }
}

// ============================================
// EJERCICIO 4: impl Trait
// ============================================
//
// Usa impl Trait como parámetro y retorno

/// Acepta cualquier tipo Display
fn mostrar(item: impl Display) {
    println!("Mostrando: {}", item);
}

/// Acepta cualquier iterador de i32
fn sumar_iterador(iter: impl Iterator<Item = i32>) -> i32 {
    iter.sum()
}

/// Retorna un iterador (tipo oculto)
fn crear_rango(inicio: i32, fin: i32) -> impl Iterator<Item = i32> {
    inicio..fin
}

/// Retorna un iterador con filtro
fn numeros_pares(hasta: i32) -> impl Iterator<Item = i32> {
    (0..hasta).filter(|n| n % 2 == 0)
}

/// Retorna un iterador mapeado
fn cuadrados(hasta: i32) -> impl Iterator<Item = i32> {
    (1..=hasta).map(|n| n * n)
}

/// Acepta cualquier cosa que se pueda convertir a String
fn procesar_texto(texto: impl Into<String>) -> String {
    let s: String = texto.into();
    s.to_uppercase()
}

/// Función que acepta múltiples bounds con impl
fn analizar(item: impl Display + Debug + Clone) -> String {
    let clon = item.clone();
    format!("Display: {} | Debug: {:?}", item, clon)
}

fn main() {
    println!("=== Práctica 04: Trait Bounds ===\n");
    
    // Ejercicio 1: Bounds Básicos
    println!("--- Ejercicio 1: Bounds Básicos ---");
    imprimir(42);
    imprimir("Hola mundo");
    
    debug(vec![1, 2, 3]);
    
    println!("Mayor de 10 y 20: {}", mayor(10, 20));
    println!("Menor de 'a' y 'z': {}", menor('a', 'z'));
    
    let (a, b) = duplicar(&String::from("test"));
    println!("Duplicado: ({}, {})", a, b);
    
    // Ejercicio 2: Múltiples Bounds
    println!("\n--- Ejercicio 2: Múltiples Bounds ---");
    imprimir_ambos(42);
    
    let numeros = vec![3, 1, 4, 1, 5, 9, 2, 6];
    if let Some(max) = mayor_en_lista(&numeros) {
        println!("Mayor en lista: {}", max);
    }
    
    let ordenados = ordenar_copia(&numeros);
    println!("Ordenados: {:?}", ordenados);
    
    let con_duplicados = vec![1, 2, 2, 3, 3, 3, 4];
    println!("Elementos únicos: {}", contar_unicos(&con_duplicados));
    
    // Ejercicio 3: Where Clauses
    println!("\n--- Ejercicio 3: Where Clauses ---");
    let resultado = procesar("Hola", vec![1, 2, 3]);
    println!("Procesado: {}", resultado);
    
    let mut contenedor = Contenedor::new(100);
    contenedor.actualizar(200);
    contenedor.actualizar(300);
    println!("Valor actual: {:?}", contenedor.obtener());
    println!("Historial: {:?}", contenedor.historial());
    
    let mut cache: Cache<String, i32> = Cache::new();
    cache.insertar(String::from("uno"), 1);
    cache.insertar(String::from("dos"), 2);
    
    println!("Cache 'uno': {:?}", cache.obtener(&String::from("uno")));
    println!("Cache 'tres': {:?}", cache.obtener(&String::from("tres")));
    println!("Estadísticas (hits, misses): {:?}", cache.estadisticas());
    
    // Ejercicio 4: impl Trait
    println!("\n--- Ejercicio 4: impl Trait ---");
    mostrar(42);
    mostrar("texto");
    
    let suma = sumar_iterador(vec![1, 2, 3, 4, 5].into_iter());
    println!("Suma del iterador: {}", suma);
    
    let rango: Vec<i32> = crear_rango(1, 5).collect();
    println!("Rango: {:?}", rango);
    
    let pares: Vec<i32> = numeros_pares(10).collect();
    println!("Pares hasta 10: {:?}", pares);
    
    let cuads: Vec<i32> = cuadrados(5).collect();
    println!("Cuadrados 1-5: {:?}", cuads);
    
    println!("Texto procesado: {}", procesar_texto("hola"));
    println!("Texto procesado: {}", procesar_texto(String::from("mundo")));
    
    println!("Análisis: {}", analizar(42));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Tests Ejercicio 1: Bounds Básicos
    #[test]
    fn test_mayor_enteros() {
        assert_eq!(mayor(5, 10), 10);
        assert_eq!(mayor(10, 5), 10);
    }
    
    #[test]
    fn test_mayor_strings() {
        assert_eq!(mayor("apple", "banana"), "banana");
    }
    
    #[test]
    fn test_menor_enteros() {
        assert_eq!(menor(5, 10), 5);
        assert_eq!(menor(10, 5), 5);
    }
    
    #[test]
    fn test_duplicar() {
        let (a, b) = duplicar(&42);
        assert_eq!(a, 42);
        assert_eq!(b, 42);
    }
    
    #[test]
    fn test_duplicar_string() {
        let (a, b) = duplicar(&String::from("test"));
        assert_eq!(a, "test");
        assert_eq!(b, "test");
    }
    
    // Tests Ejercicio 2: Múltiples Bounds
    #[test]
    fn test_mayor_en_lista() {
        let lista = vec![1, 5, 3, 9, 2];
        assert_eq!(mayor_en_lista(&lista), Some(9));
    }
    
    #[test]
    fn test_mayor_en_lista_vacia() {
        let lista: Vec<i32> = vec![];
        assert_eq!(mayor_en_lista(&lista), None);
    }
    
    #[test]
    fn test_ordenar_copia() {
        let lista = vec![3, 1, 4, 1, 5];
        let ordenada = ordenar_copia(&lista);
        assert_eq!(ordenada, vec![1, 1, 3, 4, 5]);
    }
    
    #[test]
    fn test_contar_unicos() {
        let lista = vec![1, 2, 2, 3, 3, 3];
        assert_eq!(contar_unicos(&lista), 3);
    }
    
    #[test]
    fn test_contar_unicos_strings() {
        let lista = vec!["a", "b", "a", "c", "b"];
        assert_eq!(contar_unicos(&lista), 3);
    }
    
    // Tests Ejercicio 3: Where Clauses
    #[test]
    fn test_procesar() {
        let resultado = procesar("test", 42);
        assert!(resultado.contains("test"));
        assert!(resultado.contains("42"));
    }
    
    #[test]
    fn test_contenedor_new() {
        let cont = Contenedor::new(100);
        assert_eq!(*cont.obtener(), 100);
    }
    
    #[test]
    fn test_contenedor_actualizar() {
        let mut cont = Contenedor::new(1);
        cont.actualizar(2);
        cont.actualizar(3);
        assert_eq!(*cont.obtener(), 3);
        assert_eq!(cont.historial().len(), 3);
    }
    
    #[test]
    fn test_cache_insertar_obtener() {
        let mut cache: Cache<String, i32> = Cache::new();
        cache.insertar(String::from("key"), 42);
        assert_eq!(cache.obtener(&String::from("key")), Some(42));
    }
    
    #[test]
    fn test_cache_miss() {
        let mut cache: Cache<String, i32> = Cache::new();
        assert_eq!(cache.obtener(&String::from("noexiste")), None);
    }
    
    #[test]
    fn test_cache_estadisticas() {
        let mut cache: Cache<String, i32> = Cache::new();
        cache.insertar(String::from("a"), 1);
        cache.obtener(&String::from("a")); // hit
        cache.obtener(&String::from("b")); // miss
        assert_eq!(cache.estadisticas(), (1, 1));
    }
    
    // Tests Ejercicio 4: impl Trait
    #[test]
    fn test_sumar_iterador() {
        let suma = sumar_iterador(vec![1, 2, 3].into_iter());
        assert_eq!(suma, 6);
    }
    
    #[test]
    fn test_crear_rango() {
        let rango: Vec<i32> = crear_rango(1, 4).collect();
        assert_eq!(rango, vec![1, 2, 3]);
    }
    
    #[test]
    fn test_numeros_pares() {
        let pares: Vec<i32> = numeros_pares(8).collect();
        assert_eq!(pares, vec![0, 2, 4, 6]);
    }
    
    #[test]
    fn test_cuadrados() {
        let cuads: Vec<i32> = cuadrados(4).collect();
        assert_eq!(cuads, vec![1, 4, 9, 16]);
    }
    
    #[test]
    fn test_procesar_texto_str() {
        assert_eq!(procesar_texto("hola"), "HOLA");
    }
    
    #[test]
    fn test_procesar_texto_string() {
        assert_eq!(procesar_texto(String::from("mundo")), "MUNDO");
    }
    
    #[test]
    fn test_analizar() {
        let resultado = analizar(42);
        assert!(resultado.contains("42"));
    }
}
