// ============================================
// Practice 03: Constructors
// ============================================
// Objective: Associated functions and new()
// ============================================

struct User {
    name: String,
    email: String,
    age: u32,
    active: bool,
}

impl User {
    // -----------------------------------------
    // PART 1: Basic Constructor
    // -----------------------------------------
    
    /// Main constructor
    fn new(name: String, email: String) -> Self {
        Self {
            name,
            email,
            age: 0,
            active: true,
        }
    }

    // -----------------------------------------
    // PART 2: Multiple Constructors
    // -----------------------------------------
    
    /// Constructor with all data
    fn with_age(name: String, email: String, age: u32) -> Self {
        Self {
            name,
            email,
            age,
            active: true,
        }
    }

    /// Constructor for anonymous user
    fn anonymous() -> Self {
        Self {
            name: String::from("Anonymous"),
            email: String::from("anonymous@temp.com"),
            age: 0,
            active: false,
        }
    }

    // TODO: Implement from_email that extracts name from email
    // Example: "john.doe@mail.com" -> name = "john.doe"
    // fn from_email(email: String) -> Self { ... }

    // -----------------------------------------
    // PART 3: Validation (Advanced)
    // -----------------------------------------
    
    /// Constructor with age validation
    /// Returns None if age is greater than 150
    fn validated(name: String, email: String, age: u32) -> Option<Self> {
        if age > 150 {
            None
        } else {
            Some(Self {
                name,
                email,
                age,
                active: true,
            })
        }
    }

    // -----------------------------------------
    // Instance methods
    // -----------------------------------------
    
    fn display(&self) {
        println!(
            "User: {} ({}) - {} years old - {}",
            self.name,
            self.email,
            self.age,
            if self.active { "active" } else { "inactive" }
        );
    }

    fn have_birthday(&mut self) {
        self.age += 1;
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    println!("=== Practice 03: Constructors ===\n");

    // -----------------------------------------
    // Using new()
    // -----------------------------------------
    println!("--- Constructor new() ---");
    
    let mut user1 = User::new(
        String::from("Ana Garcia"),
        String::from("ana@email.com"),
    );
    user1.display();

    // -----------------------------------------
    // Using with_age()
    // -----------------------------------------
    println!("\n--- Constructor with_age() ---");
    
    let user2 = User::with_age(
        String::from("Carlos Lopez"),
        String::from("carlos@email.com"),
        30,
    );
    user2.display();

    // -----------------------------------------
    // Using anonymous()
    // -----------------------------------------
    println!("\n--- Constructor anonymous() ---");
    
    let anonymous = User::anonymous();
    anonymous.display();

    // -----------------------------------------
    // Using validated()
    // -----------------------------------------
    println!("\n--- Constructor validated() ---");
    
    // Valid case
    if let Some(user) = User::validated(
        String::from("Maria"),
        String::from("maria@email.com"),
        25,
    ) {
        println!("User created:");
        user.display();
    }

    // Invalid case
    match User::validated(
        String::from("Impossible"),
        String::from("imp@email.com"),
        200,
    ) {
        Some(u) => u.display(),
        None => println!("Error: invalid age (> 150)"),
    }

    // -----------------------------------------
    // Modify with methods
    // -----------------------------------------
    println!("\n--- Modify User ---");
    
    user1.have_birthday();
    user1.have_birthday();
    println!("After 2 birthdays:");
    user1.display();

    println!("\n✅ Practice completed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let u = User::new(String::from("Test"), String::from("test@t.com"));
        
        assert_eq!(u.name, "Test");
        assert_eq!(u.email, "test@t.com");
        assert_eq!(u.age, 0);
        assert!(u.active);
    }

    #[test]
    fn test_with_age() {
        let u = User::with_age(
            String::from("Test"),
            String::from("test@t.com"),
            25,
        );
        
        assert_eq!(u.age, 25);
    }

    #[test]
    fn test_anonymous() {
        let u = User::anonymous();
        
        assert_eq!(u.name, "Anonymous");
        assert!(!u.active);
    }

    #[test]
    fn test_validated_ok() {
        let result = User::validated(
            String::from("Test"),
            String::from("test@t.com"),
            25,
        );
        
        assert!(result.is_some());
    }

    #[test]
    fn test_validated_fail() {
        let result = User::validated(
            String::from("Test"),
            String::from("test@t.com"),
            200,
        );
        
        assert!(result.is_none());
    }

    #[test]
    fn test_have_birthday() {
        let mut u = User::new(String::from("Test"), String::from("test@t.com"));
        u.have_birthday();
        
        assert_eq!(u.age, 1);
    }

    #[test]
    fn test_deactivate() {
        let mut u = User::new(String::from("Test"), String::from("test@t.com"));
        u.deactivate();
        
        assert!(!u.active);
    }
}
