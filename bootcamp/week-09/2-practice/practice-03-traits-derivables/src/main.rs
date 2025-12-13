//! # Práctica 03: Traits Derivables
//!
//! Aprende a usar #[derive()] para implementar traits automáticamente.

use std::collections::{HashMap, HashSet};

// ============================================
// EJERCICIO 1: Debug y Clone
// ============================================
//
// Usa derive para Debug y Clone

#[derive(Debug, Clone)]
struct Product {
    id: u32,
    name: String,
    price: f64,
    stock: u32,
}

impl Product {
    fn new(id: u32, name: &str, price: f64, stock: u32) -> Self {
        Product {
            id,
            name: name.to_string(),
            price,
            stock,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }
    
    fn distance(&self, other: &Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

// ============================================
// EJERCICIO 2: PartialEq y Eq
// ============================================
//
// Deriva igualdad para comparar estructuras

#[derive(Debug, Clone, PartialEq, Eq)]
struct User {
    id: u32,
    email: String,
    active: bool,
}

impl User {
    fn new(id: u32, email: &str) -> Self {
        User {
            id,
            email: email.to_string(),
            active: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Coordinate {
    latitude: f64,
    longitude: f64,
}

impl Coordinate {
    fn new(lat: f64, lon: f64) -> Self {
        Coordinate {
            latitude: lat,
            longitude: lon,
        }
    }
}

// ============================================
// EJERCICIO 3: PartialOrd y Ord
// ============================================
//
// Deriva ordenamiento para ordenar colecciones

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Student {
    grade: u32,      // Primero ordena por nota
    name: String, // Luego por nombre
}

impl Student {
    fn new(name: &str, grade: u32) -> Self {
        Student {
            grade,
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    fn new(major: u32, minor: u32, patch: u32) -> Self {
        Version { major, minor, patch }
    }
    
    fn from_str(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        Some(Version {
            major: parts[0].parse().ok()?,
            minor: parts[1].parse().ok()?,
            patch: parts[2].parse().ok()?,
        })
    }
}

// ============================================
// EJERCICIO 4: Hash y Default
// ============================================
//
// Deriva Hash para HashMap y Default para valores iniciales

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ProductId {
    category: String,
    number: u32,
}

impl ProductId {
    fn new(category: &str, number: u32) -> Self {
        ProductId {
            category: category.to_string(),
            number,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Config {
    debug: bool,
    max_connections: u32,
    timeout_ms: u64,
    app_name: String,
}

impl Config {
    fn production() -> Self {
        Config {
            debug: false,
            max_connections: 1000,
            timeout_ms: 30000,
            app_name: String::from("MiApp"),
        }
    }
    
    fn development() -> Self {
        Config {
            debug: true,
            max_connections: 10,
            timeout_ms: 60000,
            app_name: String::from("MiApp-Dev"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Tag {
    name: String,
    color: String,
}

impl Tag {
    fn new(name: &str, color: &str) -> Self {
        Tag {
            name: name.to_string(),
            color: color.to_string(),
        }
    }
}

fn main() {
    println!("=== Práctica 03: Traits Derivables ===\n");
    
    // Ejercicio 1: Debug y Clone
    println!("--- Ejercicio 1: Debug y Clone ---");
    let product = Product::new(1, "Laptop", 999.99, 10);
    println!("Product: {:?}", product);
    
    let product_clone = product.clone();
    println!("Clone: {:?}", product_clone);
    
    let point = Point2D::new(3.0, 4.0);
    let point2 = point; // Copy, no move
    println!("Original point: {:?}", point);
    println!("Copied point: {:?}", point2);
    println!("Distance to origin: {:.2}", point.distance(&Point2D::new(0.0, 0.0)));
    
    // Ejercicio 2: PartialEq y Eq
    println!("\n--- Ejercicio 2: PartialEq y Eq ---");
    let user1 = User::new(1, "user@test.com");
    let user2 = User::new(1, "user@test.com");
    let user3 = User::new(2, "other@test.com");
    
    println!("user1 == user2: {}", user1 == user2);
    println!("user1 == user3: {}", user1 == user3);
    println!("user1 != user3: {}", user1 != user3);
    
    let coord1 = Coordinate::new(40.4168, -3.7038);
    let coord2 = Coordinate::new(40.4168, -3.7038);
    println!("Coordinates equal: {}", coord1 == coord2);
    
    // Ejercicio 3: PartialOrd y Ord
    println!("\n--- Ejercicio 3: PartialOrd y Ord ---");
    let mut students = vec![
        Student::new("Carlos", 85),
        Student::new("Ana", 92),
        Student::new("Bob", 85),
        Student::new("Diana", 78),
    ];
    
    students.sort();
    println!("Sorted students:");
    for s in &students {
        println!("  {}: {}", s.name, s.grade);
    }
    
    let mut versions = vec![
        Version::new(1, 0, 0),
        Version::new(2, 1, 0),
        Version::new(1, 2, 3),
        Version::new(2, 0, 1),
    ];
    versions.sort();
    println!("\nSorted versions:");
    for v in &versions {
        println!("  {}.{}.{}", v.major, v.minor, v.patch);
    }
    
    // Ejercicio 4: Hash y Default
    println!("\n--- Ejercicio 4: Hash y Default ---");
    let mut inventory: HashMap<ProductId, u32> = HashMap::new();
    inventory.insert(ProductId::new("ELEC", 1), 50);
    inventory.insert(ProductId::new("ELEC", 2), 30);
    inventory.insert(ProductId::new("ROPA", 1), 100);
    
    let id_to_find = ProductId::new("ELEC", 1);
    println!("Stock of {:?}: {:?}", id_to_find, inventory.get(&id_to_find));
    
    let config_default: Config = Default::default();
    let config_prod = Config::production();
    println!("\nDefault config: {:?}", config_default);
    println!("Production config: {:?}", config_prod);
    
    let mut tags: HashSet<Tag> = HashSet::new();
    tags.insert(Tag::new("rust", "orange"));
    tags.insert(Tag::new("programming", "blue"));
    tags.insert(Tag::new("rust", "orange")); // Duplicado, no se añade
    println!("\nUnique tags: {}", tags.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Tests Ejercicio 1: Debug y Clone
    #[test]
    fn test_product_debug() {
        let p = Product::new(1, "Test", 10.0, 5);
        let debug = format!("{:?}", p);
        assert!(debug.contains("Test"));
        assert!(debug.contains("10"));
    }
    
    #[test]
    fn test_product_clone() {
        let p1 = Product::new(1, "Original", 100.0, 10);
        let p2 = p1.clone();
        assert_eq!(p1.id, p2.id);
        assert_eq!(p1.name, p2.name);
    }
    
    #[test]
    fn test_point_copy() {
        let p1 = Point2D::new(1.0, 2.0);
        let p2 = p1; // Copy
        assert_eq!(p1.x, p2.x);
        assert_eq!(p1.y, p2.y);
    }
    
    #[test]
    fn test_point_distance() {
        let origin = Point2D::new(0.0, 0.0);
        let point = Point2D::new(3.0, 4.0);
        assert!((point.distance(&origin) - 5.0).abs() < 0.001);
    }
    
    // Tests Ejercicio 2: PartialEq y Eq
    #[test]
    fn test_user_equality() {
        let u1 = User::new(1, "test@test.com");
        let u2 = User::new(1, "test@test.com");
        assert_eq!(u1, u2);
    }
    
    #[test]
    fn test_user_inequality() {
        let u1 = User::new(1, "a@a.com");
        let u2 = User::new(2, "b@b.com");
        assert_ne!(u1, u2);
    }
    
    #[test]
    fn test_coordinate_equality() {
        let c1 = Coordinate::new(1.5, 2.5);
        let c2 = Coordinate::new(1.5, 2.5);
        assert_eq!(c1, c2);
    }
    
    // Tests Ejercicio 3: PartialOrd y Ord
    #[test]
    fn test_student_ordering() {
        let s1 = Student::new("Ana", 90);
        let s2 = Student::new("Bob", 85);
        assert!(s1 > s2); // Mayor grade
    }
    
    #[test]
    fn test_student_ordering_same_grade() {
        let s1 = Student::new("Ana", 90);
        let s2 = Student::new("Bob", 90);
        assert!(s1 < s2); // Same grade, ordena por name
    }
    
    #[test]
    fn test_student_sort() {
        let mut students = vec![
            Student::new("Carlos", 80),
            Student::new("Ana", 90),
        ];
        students.sort();
        assert_eq!(students[0].name, "Carlos");
        assert_eq!(students[1].name, "Ana");
    }
    
    #[test]
    fn test_version_comparacion() {
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(2, 0, 0);
        assert!(v1 < v2);
    }
    
    #[test]
    fn test_version_comparacion_minor() {
        let v1 = Version::new(1, 5, 0);
        let v2 = Version::new(1, 10, 0);
        assert!(v1 < v2);
    }
    
    #[test]
    fn test_version_from_str() {
        let v = Version::from_str("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }
    
    // Tests Ejercicio 4: Hash y Default
    #[test]
    fn test_product_id_hash() {
        let mut map: HashMap<ProductId, String> = HashMap::new();
        map.insert(ProductId::new("A", 1), String::from("Product A1"));
        
        let id = ProductId::new("A", 1);
        assert_eq!(map.get(&id), Some(&String::from("Product A1")));
    }
    
    #[test]
    fn test_config_default() {
        let config: Config = Default::default();
        assert!(!config.debug);
        assert_eq!(config.max_connections, 0);
        assert_eq!(config.timeout_ms, 0);
        assert_eq!(config.app_name, "");
    }
    
    #[test]
    fn test_config_production() {
        let config = Config::production();
        assert!(!config.debug);
        assert_eq!(config.max_connections, 1000);
    }
    
    #[test]
    fn test_config_development() {
        let config = Config::development();
        assert!(config.debug);
        assert_eq!(config.max_connections, 10);
    }
    
    #[test]
    fn test_tag_hashset() {
        let mut set: HashSet<Tag> = HashSet::new();
        set.insert(Tag::new("rust", "orange"));
        set.insert(Tag::new("rust", "orange")); // Duplicate
        assert_eq!(set.len(), 1);
    }
    
    #[test]
    fn test_tag_default() {
        let tag: Tag = Default::default();
        assert_eq!(tag.name, "");
        assert_eq!(tag.color, "");
    }
}
