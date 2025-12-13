//! # Práctica 01: Vectores
//!
//! Ejercicios para dominar Vec<T> en Rust.

fn main() {
    println!("=== Práctica 01: Vectores ===\n");

    // Ejercicio 1: Operaciones básicas
    demo_operaciones_basicas();

    // Ejercicio 2: Estadísticas
    demo_estadisticas();

    // Ejercicio 3: Filtrado y transformación
    demo_filtrado();

    // Ejercicio 4: Gestión de inventario
    demo_inventario();
}

// ============================================================================
// EJERCICIO 1: Operaciones Básicas
// ============================================================================

/// Crea un vector con los números del 1 al n
fn create_range(n: i32) -> Vec<i32> {
    (1..=n).collect()
}

/// Agrega un elemento al final si no existe
fn add_if_not_exists(vec: &mut Vec<i32>, value: i32) -> bool {
    if vec.contains(&value) {
        false
    } else {
        vec.push(value);
        true
    }
}

/// Elimina la primera ocurrencia de un valor
fn remove_first_occurrence(vec: &mut Vec<i32>, value: i32) -> Option<i32> {
    if let Some(pos) = vec.iter().position(|&x| x == value) {
        Some(vec.remove(pos))
    } else {
        None
    }
}

/// Obtiene el elemento en la posición de forma segura
fn get_safe(vec: &[i32], index: usize) -> Option<i32> {
    vec.get(index).copied()
}

/// Actualiza el elemento en la posición si existe
fn update_at_position(vec: &mut [i32], index: usize, new_value: i32) -> bool {
    if let Some(element) = vec.get_mut(index) {
        *element = new_value;
        true
    } else {
        false
    }
}

fn demo_operaciones_basicas() {
    println!("--- Ejercicio 1: Operaciones Básicas ---");

    let mut v = create_range(5);
    println!("Rango 1-5: {:?}", v);

    let added = add_if_not_exists(&mut v, 6);
    println!("Agregar 6: {} -> {:?}", added, v);

    let not_added = add_if_not_exists(&mut v, 3);
    println!("Agregar 3 (existe): {} -> {:?}", not_added, v);

    let removed = remove_first_occurrence(&mut v, 3);
    println!("Eliminar 3: {:?} -> {:?}", removed, v);

    let value = get_safe(&v, 2);
    println!("Obtener índice 2: {:?}", value);

    update_at_position(&mut v, 0, 100);
    println!("Actualizar índice 0 a 100: {:?}", v);

    println!();
}

// ============================================================================
// EJERCICIO 2: Estadísticas
// ============================================================================

/// Calcula la suma de todos los elementos
fn sum(vec: &[i32]) -> i32 {
    vec.iter().sum()
}

/// Calcula el promedio (devuelve None si está vacío)
fn average(vec: &[i32]) -> Option<f64> {
    if vec.is_empty() {
        None
    } else {
        Some(sum(vec) as f64 / vec.len() as f64)
    }
}

/// Encuentra el valor mínimo
fn minimum(vec: &[i32]) -> Option<i32> {
    vec.iter().min().copied()
}

/// Encuentra el valor máximo
fn maximum(vec: &[i32]) -> Option<i32> {
    vec.iter().max().copied()
}

/// Cuenta cuántos elementos cumplen una condición
fn count_if<F>(vec: &[i32], predicate: F) -> usize
where
    F: Fn(&i32) -> bool,
{
    vec.iter().filter(|x| predicate(x)).count()
}

/// Estructura para almacenar estadísticas
#[derive(Debug)]
struct Statistics {
    sum: i32,
    average: f64,
    minimum: i32,
    maximum: i32,
    count: usize,
}

/// Calcula todas las estadísticas de un vector
fn calculate_statistics(vec: &[i32]) -> Option<Statistics> {
    if vec.is_empty() {
        return None;
    }

    Some(Statistics {
        sum: sum(vec),
        average: average(vec).unwrap(),
        minimum: minimum(vec).unwrap(),
        maximum: maximum(vec).unwrap(),
        count: vec.len(),
    })
}

fn demo_estadisticas() {
    println!("--- Ejercicio 2: Estadísticas ---");

    let numbers = vec![10, 25, 3, 47, 8, 32, 15];
    println!("Números: {:?}", numbers);

    if let Some(stats) = calculate_statistics(&numbers) {
        println!("Estadísticas: {:?}", stats);
    }

    let evens = count_if(&numbers, |x| x % 2 == 0);
    let greater_20 = count_if(&numbers, |x| *x > 20);
    println!("Números pares: {}", evens);
    println!("Mayores a 20: {}", greater_20);

    println!();
}

// ============================================================================
// EJERCICIO 3: Filtrado y Transformación
// ============================================================================

/// Filtra elementos que cumplen una condición
fn filter<F>(vec: &[i32], predicate: F) -> Vec<i32>
where
    F: Fn(&i32) -> bool,
{
    vec.iter().filter(|x| predicate(x)).copied().collect()
}

/// Transforma cada elemento aplicando una función
fn transform<F>(vec: &[i32], transformation: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    vec.iter().map(|&x| transformation(x)).collect()
}

/// Filtra y transforma en una sola operación
fn filter_and_transform<P, T>(vec: &[i32], predicate: P, transformation: T) -> Vec<i32>
where
    P: Fn(&i32) -> bool,
    T: Fn(i32) -> i32,
{
    vec.iter()
        .filter(|x| predicate(x))
        .map(|&x| transformation(x))
        .collect()
}

/// Agrupa números en pares e impares
fn group_even_odd(vec: &[i32]) -> (Vec<i32>, Vec<i32>) {
    vec.iter().partition(|x| *x % 2 == 0)
}

/// Elimina duplicados manteniendo el orden
fn remove_duplicates(vec: &[i32]) -> Vec<i32> {
    let mut seen = Vec::new();
    for &n in vec {
        if !seen.contains(&n) {
            seen.push(n);
        }
    }
    seen
}

fn demo_filtrado() {
    println!("--- Ejercicio 3: Filtrado y Transformación ---");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Original: {:?}", numbers);

    let evens = filter(&numbers, |x| x % 2 == 0);
    println!("Solo pares: {:?}", evens);

    let doubled = transform(&numbers, |x| x * 2);
    println!("Duplicados: {:?}", doubled);

    let evens_squared = filter_and_transform(&numbers, |x| x % 2 == 0, |x| x * x);
    println!("Pares al cuadrado: {:?}", evens_squared);

    let (evens_group, odds_group) = group_even_odd(&numbers);
    println!("Pares: {:?}, Impares: {:?}", evens_group, odds_group);

    let with_duplicates = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let without_duplicates = remove_duplicates(&with_duplicates);
    println!("Sin duplicados: {:?}", without_duplicates);

    println!();
}

// ============================================================================
// EJERCICIO 4: Gestión de Inventario
// ============================================================================

#[derive(Debug, Clone)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    quantity: u32,
}

impl Product {
    fn new(id: u32, name: &str, price: f64, quantity: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            price,
            quantity,
        }
    }

    fn total_value(&self) -> f64 {
        self.price * self.quantity as f64
    }
}

struct Inventory {
    products: Vec<Product>,
    next_id: u32,
}

impl Inventory {
    fn new() -> Self {
        Self {
            products: Vec::new(),
            next_id: 1,
        }
    }

    /// Agrega un nuevo producto
    fn add(&mut self, name: &str, price: f64, quantity: u32) -> u32 {
        let id = self.next_id;
        self.products
            .push(Product::new(id, name, price, quantity));
        self.next_id += 1;
        id
    }

    /// Busca un producto por ID
    fn find(&self, id: u32) -> Option<&Product> {
        self.products.iter().find(|p| p.id == id)
    }

    /// Busca un producto por nombre (parcial, case-insensitive)
    fn find_by_name(&self, name: &str) -> Vec<&Product> {
        let name_lower = name.to_lowercase();
        self.products
            .iter()
            .filter(|p| p.name.to_lowercase().contains(&name_lower))
            .collect()
    }

    /// Actualiza la cantidad de un producto
    fn update_quantity(&mut self, id: u32, quantity: u32) -> bool {
        if let Some(product) = self.products.iter_mut().find(|p| p.id == id) {
            product.quantity = quantity;
            true
        } else {
            false
        }
    }

    /// Elimina un producto por ID
    fn delete(&mut self, id: u32) -> Option<Product> {
        if let Some(pos) = self.products.iter().position(|p| p.id == id) {
            Some(self.products.remove(pos))
        } else {
            None
        }
    }

    /// Productos con stock bajo (< umbral)
    fn low_stock(&self, threshold: u32) -> Vec<&Product> {
        self.products
            .iter()
            .filter(|p| p.quantity < threshold)
            .collect()
    }

    /// Valor total del inventario
    fn total_value(&self) -> f64 {
        self.products.iter().map(|p| p.total_value()).sum()
    }

    /// Productos ordenados por precio
    fn sorted_by_price(&self) -> Vec<&Product> {
        let mut products: Vec<_> = self.products.iter().collect();
        products.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
        products
    }

    /// Cantidad total de productos
    fn total_quantity(&self) -> u32 {
        self.products.iter().map(|p| p.quantity).sum()
    }
}

fn demo_inventario() {
    println!("--- Ejercicio 4: Gestión de Inventario ---");

    let mut inv = Inventory::new();

    // Agregar productos
    inv.add("Laptop HP", 899.99, 5);
    inv.add("Mouse Logitech", 29.99, 50);
    inv.add("Teclado Mecánico", 79.99, 25);
    inv.add("Monitor 27\"", 299.99, 3);
    inv.add("Laptop Dell", 999.99, 2);

    println!("Inventario inicial:");
    for p in &inv.products {
        println!(
            "  #{} - {} (${:.2} x {} = ${:.2})",
            p.id,
            p.name,
            p.price,
            p.quantity,
            p.total_value()
        );
    }

    println!("\nBúsqueda por ID 3: {:?}", inv.find(3).map(|p| &p.name));

    println!("\nBúsqueda 'laptop':");
    for p in inv.find_by_name("laptop") {
        println!("  - {}", p.name);
    }

    println!("\nStock bajo (< 5 unidades):");
    for p in inv.low_stock(5) {
        println!("  ⚠️  {} ({} unidades)", p.name, p.quantity);
    }

    println!("\nValor total del inventario: ${:.2}", inv.total_value());
    println!("Cantidad total de unidades: {}", inv.total_quantity());

    println!("\nProductos ordenados por precio:");
    for p in inv.sorted_by_price() {
        println!("  ${:.2} - {}", p.price, p.name);
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Tests Ejercicio 1
    #[test]
    fn test_create_range() {
        assert_eq!(create_range(5), vec![1, 2, 3, 4, 5]);
        assert_eq!(create_range(0), vec![]);
        assert_eq!(create_range(1), vec![1]);
    }

    #[test]
    fn test_add_if_not_exists() {
        let mut v = vec![1, 2, 3];
        assert!(add_if_not_exists(&mut v, 4));
        assert!(!add_if_not_exists(&mut v, 2));
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_remove_first_occurrence() {
        let mut v = vec![1, 2, 3, 2, 4];
        assert_eq!(remove_first_occurrence(&mut v, 2), Some(2));
        assert_eq!(v, vec![1, 3, 2, 4]);
        assert_eq!(remove_first_occurrence(&mut v, 5), None);
    }

    #[test]
    fn test_get_safe() {
        let v = vec![10, 20, 30];
        assert_eq!(get_safe(&v, 1), Some(20));
        assert_eq!(get_safe(&v, 10), None);
    }

    #[test]
    fn test_update_at_position() {
        let mut v = vec![1, 2, 3];
        assert!(update_at_position(&mut v, 1, 100));
        assert_eq!(v[1], 100);
        assert!(!update_at_position(&mut v, 10, 999));
    }

    // Tests Ejercicio 2
    #[test]
    fn test_sum() {
        assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum(&[]), 0);
    }

    #[test]
    fn test_average() {
        assert_eq!(average(&[2, 4, 6]), Some(4.0));
        assert_eq!(average(&[]), None);
    }

    #[test]
    fn test_minimum_maximum() {
        let v = vec![3, 1, 4, 1, 5];
        assert_eq!(minimum(&v), Some(1));
        assert_eq!(maximum(&v), Some(5));
        assert_eq!(minimum(&[]), None);
    }

    #[test]
    fn test_count_if() {
        let v = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(count_if(&v, |x| x % 2 == 0), 3);
        assert_eq!(count_if(&v, |x| *x > 10), 0);
    }

    // Tests Ejercicio 3
    #[test]
    fn test_filter() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(filter(&v, |x| x % 2 == 0), vec![2, 4]);
    }

    #[test]
    fn test_transform() {
        let v = vec![1, 2, 3];
        assert_eq!(transform(&v, |x| x * 2), vec![2, 4, 6]);
    }

    #[test]
    fn test_filter_and_transform() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(filter_and_transform(&v, |x| x % 2 == 0, |x| x * 10), vec![20, 40]);
    }

    #[test]
    fn test_group_even_odd() {
        let v = vec![1, 2, 3, 4, 5];
        let (evens, odds) = group_even_odd(&v);
        assert_eq!(evens, vec![2, 4]);
        assert_eq!(odds, vec![1, 3, 5]);
    }

    #[test]
    fn test_remove_duplicates() {
        assert_eq!(remove_duplicates(&[1, 2, 2, 3, 1]), vec![1, 2, 3]);
    }

    // Tests Ejercicio 4
    #[test]
    fn test_inventory_add() {
        let mut inv = Inventory::new();
        let id1 = inv.add("Producto A", 10.0, 5);
        let id2 = inv.add("Producto B", 20.0, 3);
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(inv.products.len(), 2);
    }

    #[test]
    fn test_inventory_find() {
        let mut inv = Inventory::new();
        inv.add("Test", 10.0, 5);
        assert!(inv.find(1).is_some());
        assert!(inv.find(999).is_none());
    }

    #[test]
    fn test_inventory_find_by_name() {
        let mut inv = Inventory::new();
        inv.add("Laptop HP", 899.99, 5);
        inv.add("Laptop Dell", 999.99, 3);
        inv.add("Mouse", 29.99, 10);

        let laptops = inv.find_by_name("laptop");
        assert_eq!(laptops.len(), 2);
    }

    #[test]
    fn test_inventory_total_value() {
        let mut inv = Inventory::new();
        inv.add("A", 10.0, 5);  // 50
        inv.add("B", 20.0, 2);  // 40
        assert!((inv.total_value() - 90.0).abs() < 0.01);
    }

    #[test]
    fn test_inventory_low_stock() {
        let mut inv = Inventory::new();
        inv.add("A", 10.0, 2);
        inv.add("B", 20.0, 10);
        inv.add("C", 30.0, 1);

        let low = inv.low_stock(5);
        assert_eq!(low.len(), 2);
    }

    #[test]
    fn test_inventory_delete() {
        let mut inv = Inventory::new();
        inv.add("Test", 10.0, 5);
        
        let deleted = inv.delete(1);
        assert!(deleted.is_some());
        assert_eq!(deleted.unwrap().name, "Test");
        assert!(inv.products.is_empty());
    }
}
