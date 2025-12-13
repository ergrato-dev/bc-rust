// ============================================
// Practice 04: Advanced Structs
// ============================================
// Objective: Tuple, Unit and Nested Structs
// ============================================

// -----------------------------------------
// PART 1: Tuple Structs
// -----------------------------------------

/// RGB Color (0-255 each component)
struct Color(u8, u8, u8);

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    fn red() -> Self {
        Self(255, 0, 0)
    }

    fn green() -> Self {
        Self(0, 255, 0)
    }

    fn blue() -> Self {
        Self(0, 0, 255)
    }

    fn white() -> Self {
        Self(255, 255, 255)
    }

    fn black() -> Self {
        Self(0, 0, 0)
    }

    fn display(&self) {
        println!("RGB({}, {}, {})", self.0, self.1, self.2);
    }

    fn is_gray(&self) -> bool {
        self.0 == self.1 && self.1 == self.2
    }
}

/// 2D Point
struct Point(f64, f64);

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self(x, y)
    }

    fn origin() -> Self {
        Self(0.0, 0.0)
    }

    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn distance_to_origin(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }

    fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        (dx * dx + dy * dy).sqrt()
    }
}

// -----------------------------------------
// PART 2: Newtype Pattern
// -----------------------------------------

/// User ID (cannot be confused with ProductId)
struct UserId(u64);

/// Product ID (different type from UserId)
struct ProductId(u64);

impl UserId {
    fn new(id: u64) -> Self {
        Self(id)
    }

    fn value(&self) -> u64 {
        self.0
    }
}

impl ProductId {
    fn new(id: u64) -> Self {
        Self(id)
    }

    fn value(&self) -> u64 {
        self.0
    }
}

// Functions that only accept the correct type
fn get_user(id: UserId) {
    println!("Searching for user with ID: {}", id.value());
}

fn get_product(id: ProductId) {
    println!("Searching for product with ID: {}", id.value());
}

// -----------------------------------------
// PART 3: Nested Structs
// -----------------------------------------

struct Address {
    street: String,
    city: String,
    postal_code: String,
}

impl Address {
    fn new(street: String, city: String, postal_code: String) -> Self {
        Self { street, city, postal_code }
    }

    fn display(&self) {
        println!("{}, {} ({})", self.street, self.city, self.postal_code);
    }
}

struct Person {
    name: String,
    age: u32,
    address: Address,
}

impl Person {
    fn new(name: String, age: u32, address: Address) -> Self {
        Self { name, age, address }
    }

    fn display(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        print!("Address: ");
        self.address.display();
    }

    fn city(&self) -> &str {
        &self.address.city
    }
}

// -----------------------------------------
// PART 4: Unit Struct (Marker)
// -----------------------------------------

struct Validated;
struct NotValidated;

// Could be used for type-state pattern (advanced)

fn main() {
    println!("=== Practice 04: Advanced Structs ===\n");

    // -----------------------------------------
    // Tuple Structs
    // -----------------------------------------
    println!("--- Tuple Structs: Color ---");
    
    let red = Color::red();
    let custom = Color::new(128, 64, 255);
    let gray = Color::new(100, 100, 100);
    
    print!("Red: "); red.display();
    print!("Custom: "); custom.display();
    print!("Gray: "); gray.display();
    println!("Is gray?: {}", gray.is_gray());

    println!("\n--- Tuple Structs: Point ---");
    
    let p1 = Point::new(3.0, 4.0);
    let p2 = Point::new(6.0, 8.0);
    let origin = Point::origin();
    
    println!("P1: ({}, {})", p1.x(), p1.y());
    println!("Distance from P1 to origin: {:.2}", p1.distance_to_origin());
    println!("Distance from P1 to P2: {:.2}", p1.distance_to(&p2));

    // -----------------------------------------
    // Newtype Pattern
    // -----------------------------------------
    println!("\n--- Newtype Pattern ---");
    
    let user_id = UserId::new(42);
    let product_id = ProductId::new(42);
    
    // These are different types even though they contain the same value
    get_user(user_id);
    get_product(product_id);
    
    // This would NOT compile - type safety!
    // get_user(product_id);  // Error!
    // get_product(user_id);  // Error!

    // -----------------------------------------
    // Nested Structs
    // -----------------------------------------
    println!("\n--- Nested Structs ---");
    
    let address = Address::new(
        String::from("123 Main Street"),
        String::from("New York"),
        String::from("10001"),
    );
    
    let person = Person::new(
        String::from("John Doe"),
        30,
        address,
    );
    
    person.display();
    println!("\nCity: {}", person.city());

    // -----------------------------------------
    // Unit Structs
    // -----------------------------------------
    println!("\n--- Unit Structs ---");
    let _validated = Validated;
    let _not_validated = NotValidated;
    println!("Unit structs created (used as markers)");

    println!("\n✅ Practice completed");
}

#[cfg(test)]
mod tests {
    use super::*;

    // Color tests
    #[test]
    fn test_color_red() {
        let c = Color::red();
        assert_eq!(c.0, 255);
        assert_eq!(c.1, 0);
        assert_eq!(c.2, 0);
    }

    #[test]
    fn test_color_is_gray() {
        let gray = Color::new(100, 100, 100);
        let not_gray = Color::new(100, 50, 100);
        
        assert!(gray.is_gray());
        assert!(!not_gray.is_gray());
    }

    // Point tests
    #[test]
    fn test_point_origin() {
        let p = Point::origin();
        assert!((p.x() - 0.0).abs() < 0.001);
        assert!((p.y() - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_point_distance_to_origin() {
        let p = Point::new(3.0, 4.0);
        assert!((p.distance_to_origin() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_point_distance_to() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert!((p1.distance_to(&p2) - 5.0).abs() < 0.001);
    }

    // Newtype tests
    #[test]
    fn test_user_id() {
        let id = UserId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn test_product_id() {
        let id = ProductId::new(100);
        assert_eq!(id.value(), 100);
    }

    // Address and Person tests
    #[test]
    fn test_person_city() {
        let address = Address::new(
            String::from("123 Main"),
            String::from("Boston"),
            String::from("02101"),
        );
        let person = Person::new(String::from("Test"), 25, address);
        
        assert_eq!(person.city(), "Boston");
    }
}
