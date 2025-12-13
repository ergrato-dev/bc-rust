//! # Práctica 02: Implementar Traits
//!
//! Aprende a implementar traits para diferentes tipos.

// ============================================
// EJERCICIO 1: Trait Animal
// ============================================
//
// Implementa el trait Animal para Dog, Cat y Bird
// Cada uno debe tener su propio sonido y forma de moverse

trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> &str;
    fn move_around(&self) -> String;
    
    fn introduce(&self) -> String {
        format!("Soy {}, hago '{}' y me muevo {}", 
            self.name(), 
            self.sound(),
            self.move_around()
        )
    }
}

struct Dog {
    name: String,
    breed: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn sound(&self) -> &str {
        "Guau guau"
    }
    
    fn move_around(&self) -> String {
        format!("corriendo con mis 4 patas de {}", self.breed)
    }
}

struct Cat {
    name: String,
    color: String,
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn sound(&self) -> &str {
        "Miau"
    }
    
    fn move_around(&self) -> String {
        format!("sigilosamente como un gato {}", self.color)
    }
}

struct Bird {
    name: String,
    can_fly: bool,
}

impl Animal for Bird {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn sound(&self) -> &str {
        "Pío pío"
    }
    
    fn move_around(&self) -> String {
        if self.can_fly {
            "volando por el cielo".to_string()
        } else {
            "caminando en el suelo".to_string()
        }
    }
}

// ============================================
// EJERCICIO 2: Múltiples Traits
// ============================================
//
// Implementa varios traits para Employee

trait Nameable {
    fn full_name(&self) -> String;
}

trait WithAge {
    fn age(&self) -> u32;
    fn is_adult(&self) -> bool {
        self.age() >= 18
    }
}

trait Presentable {
    fn introduction(&self) -> String;
}

struct Employee {
    first_name: String,
    last_name: String,
    age: u32,
    position: String,
    salary: f64,
}

impl Nameable for Employee {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

impl WithAge for Employee {
    fn age(&self) -> u32 {
        self.age
    }
}

impl Presentable for Employee {
    fn introduction(&self) -> String {
        format!(
            "Hola, soy {} ({} años), trabajo como {}",
            self.full_name(),
            self.age(),
            self.position
        )
    }
}

// Trait propio del Employee
trait Worker {
    fn monthly_salary(&self) -> f64;
    fn annual_salary(&self) -> f64 {
        self.monthly_salary() * 12.0
    }
}

impl Worker for Employee {
    fn monthly_salary(&self) -> f64 {
        self.salary
    }
}

// ============================================
// EJERCICIO 3: Supertraits
// ============================================
//
// Define un trait que requiera otros traits

use std::fmt::Display;

// Printable requiere que el tipo implemente Display
trait Printable: Display {
    fn print(&self) {
        println!("{}", self);
    }
    
    fn print_decorated(&self) {
        println!("*** {} ***", self);
    }
}

struct Message {
    content: String,
    priority: u8,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let icon = match self.priority {
            0..=3 => "📝",
            4..=6 => "⚠️",
            _ => "🚨",
        };
        write!(f, "{} {}", icon, self.content)
    }
}

impl Printable for Message {}

// ============================================
// EJERCICIO 4: Métodos Asociados (Constructores)
// ============================================
//
// Define traits con métodos asociados (sin self)

trait Creator {
    fn create() -> Self;
    fn create_with_name(name: &str) -> Self;
}

trait Resettable {
    fn reset(&mut self);
}

#[derive(Debug)]
struct Counter {
    name: String,
    value: i32,
}

impl Creator for Counter {
    fn create() -> Self {
        Counter {
            name: String::from("default"),
            value: 0,
        }
    }
    
    fn create_with_name(name: &str) -> Self {
        Counter {
            name: name.to_string(),
            value: 0,
        }
    }
}

impl Resettable for Counter {
    fn reset(&mut self) {
        self.value = 0;
    }
}

impl Counter {
    fn increment(&mut self) {
        self.value += 1;
    }
    
    fn value(&self) -> i32 {
        self.value
    }
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Creator for Point {
    fn create() -> Self {
        Point { x: 0.0, y: 0.0 }
    }
    
    fn create_with_name(_name: &str) -> Self {
        // Los puntos no tienen nombre, pero debemos implementar el método
        Point { x: 0.0, y: 0.0 }
    }
}

impl Resettable for Point {
    fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }
}

impl Point {
    fn move_by(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
    
    fn distance_to_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    println!("=== Práctica 02: Implementar Traits ===\n");
    
    // Ejercicio 1: Animal
    println!("--- Ejercicio 1: Animal ---");
    let dog = Dog { 
        name: String::from("Max"), 
        breed: String::from("Labrador") 
    };
    let cat = Cat { 
        name: String::from("Michi"), 
        color: String::from("naranja") 
    };
    let bird = Bird { 
        name: String::from("Piolín"), 
        can_fly: true 
    };
    
    println!("{}", dog.introduce());
    println!("{}", cat.introduce());
    println!("{}", bird.introduce());
    
    // Ejercicio 2: Múltiples Traits
    println!("\n--- Ejercicio 2: Múltiples Traits ---");
    let employee = Employee {
        first_name: String::from("Ana"),
        last_name: String::from("García"),
        age: 30,
        position: String::from("Desarrolladora Rust"),
        salary: 5000.0,
    };
    
    println!("Nombre: {}", employee.full_name());
    println!("Edad: {} (mayor de edad: {})", employee.age(), employee.is_adult());
    println!("Presentación: {}", employee.introduction());
    println!("Salario anual: ${:.2}", employee.annual_salary());
    
    // Ejercicio 3: Supertraits
    println!("\n--- Ejercicio 3: Supertraits ---");
    let msg_low = Message { content: String::from("Todo bien"), priority: 2 };
    let msg_high = Message { content: String::from("URGENTE"), priority: 9 };
    
    msg_low.print();
    msg_high.print_decorated();
    
    // Ejercicio 4: Métodos Asociados
    println!("\n--- Ejercicio 4: Métodos Asociados ---");
    let mut counter = Counter::create_with_name("my_counter");
    counter.increment();
    counter.increment();
    println!("Counter: {:?}, value: {}", counter, counter.value());
    counter.reset();
    println!("Después de reset: {}", counter.value());
    
    let mut point = Point::create();
    point.move_by(3.0, 4.0);
    println!("Point: {:?}, distancia al origen: {}", point, point.distance_to_origin());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Tests Ejercicio 1: Animal
    #[test]
    fn test_dog_sound() {
        let dog = Dog { name: String::from("Rex"), breed: String::from("Pastor") };
        assert_eq!(dog.sound(), "Guau guau");
    }
    
    #[test]
    fn test_dog_name() {
        let dog = Dog { name: String::from("Rex"), breed: String::from("Pastor") };
        assert_eq!(dog.name(), "Rex");
    }
    
    #[test]
    fn test_dog_move_around() {
        let dog = Dog { name: String::from("Rex"), breed: String::from("Bulldog") };
        assert!(dog.move_around().contains("Bulldog"));
    }
    
    #[test]
    fn test_cat_sound() {
        let cat = Cat { name: String::from("Felix"), color: String::from("negro") };
        assert_eq!(cat.sound(), "Miau");
    }
    
    #[test]
    fn test_cat_move_around() {
        let cat = Cat { name: String::from("Felix"), color: String::from("blanco") };
        assert!(cat.move_around().contains("blanco"));
    }
    
    #[test]
    fn test_bird_flies() {
        let bird = Bird { name: String::from("Tweety"), can_fly: true };
        assert!(bird.move_around().contains("volando"));
    }
    
    #[test]
    fn test_bird_no_fly() {
        let bird = Bird { name: String::from("Pingu"), can_fly: false };
        assert!(bird.move_around().contains("caminando"));
    }
    
    #[test]
    fn test_animal_introduce() {
        let dog = Dog { name: String::from("Buddy"), breed: String::from("Golden") };
        let intro = dog.introduce();
        assert!(intro.contains("Buddy"));
        assert!(intro.contains("Guau"));
    }
    
    // Tests Ejercicio 2: Múltiples Traits
    #[test]
    fn test_employee_full_name() {
        let emp = Employee {
            first_name: String::from("Juan"),
            last_name: String::from("Pérez"),
            age: 25,
            position: String::from("Dev"),
            salary: 3000.0,
        };
        assert_eq!(emp.full_name(), "Juan Pérez");
    }
    
    #[test]
    fn test_employee_age() {
        let emp = Employee {
            first_name: String::from("Test"),
            last_name: String::from("User"),
            age: 17,
            position: String::from("Intern"),
            salary: 1000.0,
        };
        assert_eq!(emp.age(), 17);
        assert!(!emp.is_adult());
    }
    
    #[test]
    fn test_employee_is_adult() {
        let emp = Employee {
            first_name: String::from("Adult"),
            last_name: String::from("Person"),
            age: 25,
            position: String::from("Manager"),
            salary: 5000.0,
        };
        assert!(emp.is_adult());
    }
    
    #[test]
    fn test_employee_annual_salary() {
        let emp = Employee {
            first_name: String::from("Test"),
            last_name: String::from("User"),
            age: 30,
            position: String::from("Dev"),
            salary: 1000.0,
        };
        assert_eq!(emp.annual_salary(), 12000.0);
    }
    
    #[test]
    fn test_employee_introduction() {
        let emp = Employee {
            first_name: String::from("María"),
            last_name: String::from("López"),
            age: 28,
            position: String::from("Designer"),
            salary: 4000.0,
        };
        let intro = emp.introduction();
        assert!(intro.contains("María López"));
        assert!(intro.contains("28"));
        assert!(intro.contains("Designer"));
    }
    
    // Tests Ejercicio 3: Supertraits
    #[test]
    fn test_message_display() {
        let msg = Message { content: String::from("Hola"), priority: 1 };
        let text = format!("{}", msg);
        assert!(text.contains("Hola"));
        assert!(text.contains("📝"));
    }
    
    #[test]
    fn test_message_medium_priority() {
        let msg = Message { content: String::from("Aviso"), priority: 5 };
        let text = format!("{}", msg);
        assert!(text.contains("⚠️"));
    }
    
    #[test]
    fn test_message_high_priority() {
        let msg = Message { content: String::from("Alerta"), priority: 8 };
        let text = format!("{}", msg);
        assert!(text.contains("🚨"));
    }
    
    // Tests Ejercicio 4: Métodos Asociados
    #[test]
    fn test_counter_create() {
        let counter = Counter::create();
        assert_eq!(counter.value(), 0);
        assert_eq!(counter.name, "default");
    }
    
    #[test]
    fn test_counter_create_with_name() {
        let counter = Counter::create_with_name("test");
        assert_eq!(counter.name, "test");
    }
    
    #[test]
    fn test_counter_increment() {
        let mut counter = Counter::create();
        counter.increment();
        counter.increment();
        assert_eq!(counter.value(), 2);
    }
    
    #[test]
    fn test_counter_reset() {
        let mut counter = Counter::create();
        counter.increment();
        counter.increment();
        counter.reset();
        assert_eq!(counter.value(), 0);
    }
    
    #[test]
    fn test_point_create() {
        let point = Point::create();
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 0.0);
    }
    
    #[test]
    fn test_point_move_by() {
        let mut point = Point::create();
        point.move_by(3.0, 4.0);
        assert_eq!(point.x, 3.0);
        assert_eq!(point.y, 4.0);
    }
    
    #[test]
    fn test_point_distance() {
        let mut point = Point::create();
        point.move_by(3.0, 4.0);
        assert!((point.distance_to_origin() - 5.0).abs() < 0.001);
    }
    
    #[test]
    fn test_point_reset() {
        let mut point = Point::create();
        point.move_by(10.0, 20.0);
        point.reset();
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 0.0);
    }
}
