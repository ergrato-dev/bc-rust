// ============================================
// Practice 01: Variable Declaration
// ============================================
// Objective: Practice let vs let mut
// ============================================

fn main() {
    println!("=== Practice 01: Variables ===\n");

    // -----------------------------------------
    // PART 1: Immutable Variables
    // -----------------------------------------
    // TODO: Declare the following immutable variables:
    
    // 1. A variable 'name' with your name (type &str)
    // let name = ...;
    
    // 2. A variable 'age' with your age (type i32)
    // let age = ...;
    
    // 3. A variable 'is_student' with true or false
    // let is_student = ...;
    
    // TODO: Print the variables
    // println!("Name: {}", name);
    // println!("Age: {}", age);
    // println!("Is student: {}", is_student);

    // -----------------------------------------
    // PART 2: Mutable Variables
    // -----------------------------------------
    println!("\n--- Mutable Variables ---");
    
    // TODO: Declare a mutable counter starting at 0
    // let mut counter = ...;
    
    // TODO: Increment the counter 3 times
    // counter = counter + 1;  // or counter += 1;
    // ...
    
    // TODO: Print the final value
    // println!("Final counter: {}", counter);

    // -----------------------------------------
    // PART 3: Experiment with Errors
    // -----------------------------------------
    // Uncomment the following code and observe the error:
    
    // let immutable = 5;
    // immutable = 10;  // Error E0384!
    
    // Now fix it by adding 'mut':
    // let mut mutable = 5;
    // mutable = 10;  // Now it works!

    println!("\n✅ Practice completed");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_immutability() {
        let x = 5;
        // x cannot change
        assert_eq!(x, 5);
    }

    #[test]
    fn test_mutability() {
        let mut y = 0;
        y += 1;
        y += 1;
        y += 1;
        assert_eq!(y, 3);
    }

    #[test]
    fn test_basic_types() {
        let name: &str = "Rust";
        let age: i32 = 10;
        let is_cool: bool = true;
        
        assert_eq!(name, "Rust");
        assert_eq!(age, 10);
        assert!(is_cool);
    }
}
