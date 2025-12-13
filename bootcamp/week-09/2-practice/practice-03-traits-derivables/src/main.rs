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
struct ProductoId {
    categoria: String,
    numero: u32,
}

impl ProductoId {
    fn new(categoria: &str, numero: u32) -> Self {
        ProductoId {
            categoria: categoria.to_string(),
            numero,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Configuracion {
    debug: bool,
    max_conexiones: u32,
    timeout_ms: u64,
    nombre_app: String,
}

impl Configuracion {
    fn produccion() -> Self {
        Configuracion {
            debug: false,
            max_conexiones: 1000,
            timeout_ms: 30000,
            nombre_app: String::from("MiApp"),
        }
    }
    
    fn desarrollo() -> Self {
        Configuracion {
            debug: true,
            max_conexiones: 10,
            timeout_ms: 60000,
            nombre_app: String::from("MiApp-Dev"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Etiqueta {
    nombre: String,
    color: String,
}

impl Etiqueta {
    fn new(nombre: &str, color: &str) -> Self {
        Etiqueta {
            nombre: nombre.to_string(),
            color: color.to_string(),
        }
    }
}

fn main() {
    println!("=== Práctica 03: Traits Derivables ===\n");
    
    // Ejercicio 1: Debug y Clone
    println!("--- Ejercicio 1: Debug y Clone ---");
    let producto = Producto::new(1, "Laptop", 999.99, 10);
    println!("Producto: {:?}", producto);
    
    let producto_clon = producto.clone();
    println!("Clon: {:?}", producto_clon);
    
    let punto = Punto2D::new(3.0, 4.0);
    let punto2 = punto; // Copy, no move
    println!("Punto original: {:?}", punto);
    println!("Punto copiado: {:?}", punto2);
    println!("Distancia al origen: {:.2}", punto.distancia(&Punto2D::new(0.0, 0.0)));
    
    // Ejercicio 2: PartialEq y Eq
    println!("\n--- Ejercicio 2: PartialEq y Eq ---");
    let user1 = Usuario::new(1, "user@test.com");
    let user2 = Usuario::new(1, "user@test.com");
    let user3 = Usuario::new(2, "other@test.com");
    
    println!("user1 == user2: {}", user1 == user2);
    println!("user1 == user3: {}", user1 == user3);
    println!("user1 != user3: {}", user1 != user3);
    
    let coord1 = Coordenada::new(40.4168, -3.7038);
    let coord2 = Coordenada::new(40.4168, -3.7038);
    println!("Coordenadas iguales: {}", coord1 == coord2);
    
    // Ejercicio 3: PartialOrd y Ord
    println!("\n--- Ejercicio 3: PartialOrd y Ord ---");
    let mut estudiantes = vec![
        Estudiante::new("Carlos", 85),
        Estudiante::new("Ana", 92),
        Estudiante::new("Bob", 85),
        Estudiante::new("Diana", 78),
    ];
    
    estudiantes.sort();
    println!("Estudiantes ordenados:");
    for e in &estudiantes {
        println!("  {}: {}", e.nombre, e.nota);
    }
    
    let mut versiones = vec![
        Version::new(1, 0, 0),
        Version::new(2, 1, 0),
        Version::new(1, 2, 3),
        Version::new(2, 0, 1),
    ];
    versiones.sort();
    println!("\nVersiones ordenadas:");
    for v in &versiones {
        println!("  {}.{}.{}", v.major, v.minor, v.patch);
    }
    
    // Ejercicio 4: Hash y Default
    println!("\n--- Ejercicio 4: Hash y Default ---");
    let mut inventario: HashMap<ProductoId, u32> = HashMap::new();
    inventario.insert(ProductoId::new("ELEC", 1), 50);
    inventario.insert(ProductoId::new("ELEC", 2), 30);
    inventario.insert(ProductoId::new("ROPA", 1), 100);
    
    let id_buscar = ProductoId::new("ELEC", 1);
    println!("Stock de {:?}: {:?}", id_buscar, inventario.get(&id_buscar));
    
    let config_default: Configuracion = Default::default();
    let config_prod = Configuracion::produccion();
    println!("\nConfig default: {:?}", config_default);
    println!("Config producción: {:?}", config_prod);
    
    let mut etiquetas: HashSet<Etiqueta> = HashSet::new();
    etiquetas.insert(Etiqueta::new("rust", "orange"));
    etiquetas.insert(Etiqueta::new("programming", "blue"));
    etiquetas.insert(Etiqueta::new("rust", "orange")); // Duplicado, no se añade
    println!("\nEtiquetas únicas: {}", etiquetas.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Tests Ejercicio 1: Debug y Clone
    #[test]
    fn test_producto_debug() {
        let p = Producto::new(1, "Test", 10.0, 5);
        let debug = format!("{:?}", p);
        assert!(debug.contains("Test"));
        assert!(debug.contains("10"));
    }
    
    #[test]
    fn test_producto_clone() {
        let p1 = Producto::new(1, "Original", 100.0, 10);
        let p2 = p1.clone();
        assert_eq!(p1.id, p2.id);
        assert_eq!(p1.nombre, p2.nombre);
    }
    
    #[test]
    fn test_punto_copy() {
        let p1 = Punto2D::new(1.0, 2.0);
        let p2 = p1; // Copy
        assert_eq!(p1.x, p2.x);
        assert_eq!(p1.y, p2.y);
    }
    
    #[test]
    fn test_punto_distancia() {
        let origen = Punto2D::new(0.0, 0.0);
        let punto = Punto2D::new(3.0, 4.0);
        assert!((punto.distancia(&origen) - 5.0).abs() < 0.001);
    }
    
    // Tests Ejercicio 2: PartialEq y Eq
    #[test]
    fn test_usuario_igualdad() {
        let u1 = Usuario::new(1, "test@test.com");
        let u2 = Usuario::new(1, "test@test.com");
        assert_eq!(u1, u2);
    }
    
    #[test]
    fn test_usuario_desigualdad() {
        let u1 = Usuario::new(1, "a@a.com");
        let u2 = Usuario::new(2, "b@b.com");
        assert_ne!(u1, u2);
    }
    
    #[test]
    fn test_coordenada_igualdad() {
        let c1 = Coordenada::new(1.5, 2.5);
        let c2 = Coordenada::new(1.5, 2.5);
        assert_eq!(c1, c2);
    }
    
    // Tests Ejercicio 3: PartialOrd y Ord
    #[test]
    fn test_estudiante_ordenamiento() {
        let e1 = Estudiante::new("Ana", 90);
        let e2 = Estudiante::new("Bob", 85);
        assert!(e1 > e2); // Mayor nota
    }
    
    #[test]
    fn test_estudiante_ordenamiento_mismo_nota() {
        let e1 = Estudiante::new("Ana", 90);
        let e2 = Estudiante::new("Bob", 90);
        assert!(e1 < e2); // Mismo nota, ordena por nombre
    }
    
    #[test]
    fn test_estudiante_sort() {
        let mut estudiantes = vec![
            Estudiante::new("Carlos", 80),
            Estudiante::new("Ana", 90),
        ];
        estudiantes.sort();
        assert_eq!(estudiantes[0].nombre, "Carlos");
        assert_eq!(estudiantes[1].nombre, "Ana");
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
    fn test_producto_id_hash() {
        let mut map: HashMap<ProductoId, String> = HashMap::new();
        map.insert(ProductoId::new("A", 1), String::from("Producto A1"));
        
        let id = ProductoId::new("A", 1);
        assert_eq!(map.get(&id), Some(&String::from("Producto A1")));
    }
    
    #[test]
    fn test_configuracion_default() {
        let config: Configuracion = Default::default();
        assert!(!config.debug);
        assert_eq!(config.max_conexiones, 0);
        assert_eq!(config.timeout_ms, 0);
        assert_eq!(config.nombre_app, "");
    }
    
    #[test]
    fn test_configuracion_produccion() {
        let config = Configuracion::produccion();
        assert!(!config.debug);
        assert_eq!(config.max_conexiones, 1000);
    }
    
    #[test]
    fn test_configuracion_desarrollo() {
        let config = Configuracion::desarrollo();
        assert!(config.debug);
        assert_eq!(config.max_conexiones, 10);
    }
    
    #[test]
    fn test_etiqueta_hashset() {
        let mut set: HashSet<Etiqueta> = HashSet::new();
        set.insert(Etiqueta::new("rust", "orange"));
        set.insert(Etiqueta::new("rust", "orange")); // Duplicado
        assert_eq!(set.len(), 1);
    }
    
    #[test]
    fn test_etiqueta_default() {
        let etiqueta: Etiqueta = Default::default();
        assert_eq!(etiqueta.nombre, "");
        assert_eq!(etiqueta.color, "");
    }
}
