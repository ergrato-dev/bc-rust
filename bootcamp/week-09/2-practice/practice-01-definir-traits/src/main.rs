//! # Práctica 01: Definir Traits
//!
//! Aprende a definir traits con métodos requeridos y default.

// ============================================
// EJERCICIO 1: Trait Describable
// ============================================
//
// Define un trait `Describable` con:
// - Método requerido: description(&self) -> String
// - Método requerido: type_name(&self) -> &str
// - Método default: info(&self) -> String que combine tipo y descripción

trait Describable {
    /// Retorna una descripción del objeto
    fn description(&self) -> String;
    
    /// Retorna el tipo del objeto
    fn type_name(&self) -> &str;
    
    /// Método default que combina tipo y descripción
    fn info(&self) -> String {
        format!("[{}] {}", self.type_name(), self.description())
    }
}

// ============================================
// EJERCICIO 2: Trait Calculable
// ============================================
//
// Define un trait `Calculable` con:
// - area(&self) -> f64
// - perimeter(&self) -> f64
// - Método default: is_large(&self) -> bool (area > 100)

trait Calculable {
    /// Calcula el área
    fn area(&self) -> f64;
    
    /// Calcula el perímetro
    fn perimeter(&self) -> f64;
    
    /// Verifica si el área es grande (> 100)
    fn is_large(&self) -> bool {
        self.area() > 100.0
    }
}

// ============================================
// EJERCICIO 3: Trait Greetable
// ============================================
//
// Define un trait `Greetable` con:
// - name(&self) -> &str (requerido)
// - greet(&self) -> String (default: "Hola, soy {name}")
// - farewell(&self) -> String (default: "Adiós de {name}")

trait Greetable {
    /// Retorna el nombre
    fn name(&self) -> &str;
    
    /// Saluda con el nombre
    fn greet(&self) -> String {
        format!("Hola, soy {}", self.name())
    }
    
    /// Se despide con el nombre
    fn farewell(&self) -> String {
        format!("Adiós de {}", self.name())
    }
}

// ============================================
// EJERCICIO 4: Trait Validatable
// ============================================
//
// Define un trait `Validatable` con:
// - is_valid(&self) -> bool (requerido)
// - error_message(&self) -> Option<String> (requerido)
// - validate(&self) -> Result<(), String> (default)

trait Validatable {
    /// Verifica si el dato es válido
    fn is_valid(&self) -> bool;
    
    /// Retorna mensaje de error si no es válido
    fn error_message(&self) -> Option<String>;
    
    /// Valida y retorna Result
    fn validate(&self) -> Result<(), String> {
        if self.is_valid() {
            Ok(())
        } else {
            Err(self.error_message().unwrap_or_else(|| "Error desconocido".to_string()))
        }
    }
}

// ============================================
// IMPLEMENTACIONES PARA TESTING
// ============================================

struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Describable for Book {
    fn description(&self) -> String {
        format!("'{}' por {} ({} páginas)", self.title, self.author, self.pages)
    }
    
    fn type_name(&self) -> &str {
        "Book"
    }
}

struct Circle {
    radius: f64,
}

impl Calculable for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Calculable for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

struct Person {
    name: String,
}

impl Greetable for Person {
    fn name(&self) -> &str {
        &self.name
    }
}

struct Robot {
    id: String,
}

impl Greetable for Robot {
    fn name(&self) -> &str {
        &self.id
    }
    
    // Sobrescribimos el método default
    fn greet(&self) -> String {
        format!("BEEP BOOP. Unidad {} en línea.", self.name())
    }
}

struct Email {
    address: String,
}

impl Validatable for Email {
    fn is_valid(&self) -> bool {
        self.address.contains('@') && self.address.contains('.')
    }
    
    fn error_message(&self) -> Option<String> {
        if !self.address.contains('@') {
            Some("Falta el símbolo @".to_string())
        } else if !self.address.contains('.') {
            Some("Falta el dominio".to_string())
        } else {
            None
        }
    }
}

struct Age {
    value: i32,
}

impl Validatable for Age {
    fn is_valid(&self) -> bool {
        self.value >= 0 && self.value <= 150
    }
    
    fn error_message(&self) -> Option<String> {
        if self.value < 0 {
            Some("La edad no puede ser negativa".to_string())
        } else if self.value > 150 {
            Some("La edad no puede ser mayor a 150".to_string())
        } else {
            None
        }
    }
}

fn main() {
    println!("=== Práctica 01: Definir Traits ===\n");
    
    // Ejercicio 1: Describable
    let book = Book {
        title: String::from("El Libro de Rust"),
        author: String::from("Ferris"),
        pages: 500,
    };
    println!("Describable:");
    println!("  Descripción: {}", book.description());
    println!("  Tipo: {}", book.type_name());
    println!("  Info: {}", book.info());
    
    // Ejercicio 2: Calculable
    println!("\nCalculable:");
    let circle = Circle { radius: 5.0 };
    println!("  Círculo (r=5): área={:.2}, perímetro={:.2}", circle.area(), circle.perimeter());
    
    let rect = Rectangle { width: 15.0, height: 8.0 };
    println!("  Rectángulo (15x8): área={:.2}, is_large={}", rect.area(), rect.is_large());
    
    // Ejercicio 3: Greetable
    println!("\nGreetable:");
    let person = Person { name: String::from("Ana") };
    println!("  Persona: {}", person.greet());
    
    let robot = Robot { id: String::from("R2D2") };
    println!("  Robot: {}", robot.greet());
    
    // Ejercicio 4: Validatable
    println!("\nValidatable:");
    let valid_email = Email { address: String::from("test@ejemplo.com") };
    let invalid_email = Email { address: String::from("invalido") };
    
    println!("  Email válido: {:?}", valid_email.validate());
    println!("  Email inválido: {:?}", invalid_email.validate());
    
    let valid_age = Age { value: 25 };
    let invalid_age = Age { value: -5 };
    
    println!("  Edad válida: {:?}", valid_age.validate());
    println!("  Edad inválida: {:?}", invalid_age.validate());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Tests Ejercicio 1: Describable
    #[test]
    fn test_book_description() {
        let book = Book {
            title: String::from("Rust"),
            author: String::from("Ferris"),
            pages: 100,
        };
        assert!(book.description().contains("Rust"));
        assert!(book.description().contains("Ferris"));
    }
    
    #[test]
    fn test_book_type_name() {
        let book = Book {
            title: String::from("Test"),
            author: String::from("Test"),
            pages: 50,
        };
        assert_eq!(book.type_name(), "Book");
    }
    
    #[test]
    fn test_book_info_default() {
        let book = Book {
            title: String::from("Mi Libro"),
            author: String::from("Autor"),
            pages: 200,
        };
        let info = book.info();
        assert!(info.contains("[Book]"));
        assert!(info.contains("Mi Libro"));
    }
    
    // Tests Ejercicio 2: Calculable
    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 1.0 };
        let area = circle.area();
        assert!((area - std::f64::consts::PI).abs() < 0.001);
    }
    
    #[test]
    fn test_circle_perimeter() {
        let circle = Circle { radius: 1.0 };
        let perimeter = circle.perimeter();
        assert!((perimeter - 2.0 * std::f64::consts::PI).abs() < 0.001);
    }
    
    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle { width: 10.0, height: 5.0 };
        assert_eq!(rect.area(), 50.0);
    }
    
    #[test]
    fn test_rectangle_perimeter() {
        let rect = Rectangle { width: 10.0, height: 5.0 };
        assert_eq!(rect.perimeter(), 30.0);
    }
    
    #[test]
    fn test_is_large_true() {
        let rect = Rectangle { width: 20.0, height: 10.0 };
        assert!(rect.is_large()); // 200 > 100
    }
    
    #[test]
    fn test_is_large_false() {
        let rect = Rectangle { width: 5.0, height: 5.0 };
        assert!(!rect.is_large()); // 25 < 100
    }
    
    // Tests Ejercicio 3: Greetable
    #[test]
    fn test_person_name() {
        let person = Person { name: String::from("Carlos") };
        assert_eq!(person.name(), "Carlos");
    }
    
    #[test]
    fn test_person_greet_default() {
        let person = Person { name: String::from("María") };
        assert_eq!(person.greet(), "Hola, soy María");
    }
    
    #[test]
    fn test_person_farewell_default() {
        let person = Person { name: String::from("Juan") };
        assert_eq!(person.farewell(), "Adiós de Juan");
    }
    
    #[test]
    fn test_robot_greet_override() {
        let robot = Robot { id: String::from("X100") };
        let greeting = robot.greet();
        assert!(greeting.contains("BEEP"));
        assert!(greeting.contains("X100"));
    }
    
    // Tests Ejercicio 4: Validatable
    #[test]
    fn test_email_valid() {
        let email = Email { address: String::from("user@domain.com") };
        assert!(email.is_valid());
        assert!(email.error_message().is_none());
    }
    
    #[test]
    fn test_email_missing_at() {
        let email = Email { address: String::from("invalid.com") };
        assert!(!email.is_valid());
        assert!(email.error_message().unwrap().contains("@"));
    }
    
    #[test]
    fn test_email_validate_ok() {
        let email = Email { address: String::from("test@test.com") };
        assert!(email.validate().is_ok());
    }
    
    #[test]
    fn test_email_validate_err() {
        let email = Email { address: String::from("bad") };
        assert!(email.validate().is_err());
    }
    
    #[test]
    fn test_age_valid() {
        let age = Age { value: 30 };
        assert!(age.is_valid());
        assert!(age.validate().is_ok());
    }
    
    #[test]
    fn test_age_negative() {
        let age = Age { value: -10 };
        assert!(!age.is_valid());
        assert!(age.error_message().unwrap().contains("negativa"));
    }
    
    #[test]
    fn test_age_too_high() {
        let age = Age { value: 200 };
        assert!(!age.is_valid());
        assert!(age.error_message().unwrap().contains("150"));
    }
}
