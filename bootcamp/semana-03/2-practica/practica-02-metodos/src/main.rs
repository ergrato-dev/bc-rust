// ============================================
// Practice 02: Methods
// ============================================
// Objective: Implement methods with impl
// ============================================

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // -----------------------------------------
    // PART 1: Read Methods (&self)
    // -----------------------------------------
    
    /// Calculates the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Calculates the perimeter of the rectangle
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    /// Checks if it's a square
    fn is_square(&self) -> bool {
        self.width == self.height
    }

    /// Shows information about the rectangle
    fn describe(&self) {
        println!(
            "Rectangle {}x{}, area: {}, is square: {}",
            self.width,
            self.height,
            self.area(),
            self.is_square()
        );
    }

    // -----------------------------------------
    // PART 2: Mutation Methods (&mut self)
    // -----------------------------------------
    
    /// Scales the rectangle by a factor
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // TODO: Implement rotate() that swaps width and height
    // fn rotate(&mut self) { ... }

    /// Doubles the size
    fn double(&mut self) {
        self.scale(2);
    }

    // -----------------------------------------
    // PART 3: Methods with Parameters
    // -----------------------------------------
    
    /// Checks if this rectangle can contain another
    fn can_contain(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // TODO: Implement is_larger_than() that compares areas
    // fn is_larger_than(&self, other: &Rectangle) -> bool { ... }
}

fn main() {
    println!("=== Practice 02: Methods ===\n");

    // -----------------------------------------
    // Read methods demonstration
    // -----------------------------------------
    println!("--- Read Methods ---");
    
    let rect = Rectangle { width: 10, height: 5 };
    
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!("Is square?: {}", rect.is_square());
    rect.describe();

    let square = Rectangle { width: 5, height: 5 };
    println!("\nIs square?: {}", square.is_square());

    // -----------------------------------------
    // Mutation methods demonstration
    // -----------------------------------------
    println!("\n--- Mutation Methods ---");
    
    let mut mutable_rect = Rectangle { width: 10, height: 5 };
    println!("Before scaling: {}x{}", mutable_rect.width, mutable_rect.height);
    
    mutable_rect.scale(2);
    println!("After scaling x2: {}x{}", mutable_rect.width, mutable_rect.height);
    
    mutable_rect.double();
    println!("After doubling: {}x{}", mutable_rect.width, mutable_rect.height);

    // -----------------------------------------
    // Methods with parameters demonstration
    // -----------------------------------------
    println!("\n--- Methods with Parameters ---");
    
    let large = Rectangle { width: 20, height: 15 };
    let small = Rectangle { width: 5, height: 3 };

    if large.can_contain(&small) {
        println!("The large one can contain the small one");
    }

    if !small.can_contain(&large) {
        println!("The small one CANNOT contain the large one");
    }

    println!("\n✅ Practice completed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let rect = Rectangle { width: 10, height: 5 };
        assert_eq!(rect.area(), 50);
    }

    #[test]
    fn test_perimeter() {
        let rect = Rectangle { width: 10, height: 5 };
        assert_eq!(rect.perimeter(), 30);
    }

    #[test]
    fn test_is_square() {
        let rect = Rectangle { width: 10, height: 5 };
        let square = Rectangle { width: 5, height: 5 };
        
        assert!(!rect.is_square());
        assert!(square.is_square());
    }

    #[test]
    fn test_scale() {
        let mut rect = Rectangle { width: 10, height: 5 };
        rect.scale(2);
        
        assert_eq!(rect.width, 20);
        assert_eq!(rect.height, 10);
    }

    #[test]
    fn test_double() {
        let mut rect = Rectangle { width: 10, height: 5 };
        rect.double();
        
        assert_eq!(rect.width, 20);
        assert_eq!(rect.height, 10);
    }

    #[test]
    fn test_can_contain() {
        let large = Rectangle { width: 20, height: 15 };
        let small = Rectangle { width: 5, height: 3 };
        
        assert!(large.can_contain(&small));
        assert!(!small.can_contain(&large));
    }
}
