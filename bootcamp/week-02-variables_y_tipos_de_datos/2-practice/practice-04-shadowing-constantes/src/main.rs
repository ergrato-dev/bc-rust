// ============================================
// Practice 04: Shadowing and Constants
// ============================================
// Objective: Master shadowing and const
// ============================================

// Constants: defined outside functions
const PI: f64 = 3.14159265358979;
const MAX_ATTEMPTS: u32 = 3;
const WELCOME_MESSAGE: &str = "Welcome to the system!";

fn main() {
    println!("=== Practice 04: Shadowing and Constants ===\n");

    // -----------------------------------------
    // PART 1: Basic Shadowing
    // -----------------------------------------
    println!("--- Basic Shadowing ---");
    
    let x = 5;
    println!("x initial: {}", x);
    
    let x = x + 1;  // Shadowing: new variable
    println!("x after +1: {}", x);
    
    let x = x * 2;  // Shadowing: another new variable
    println!("x after *2: {}", x);
    
    // Comparison with mut
    let mut y = 5;
    y = y + 1;      // Mutation: same variable
    y = y * 2;
    println!("y with mut: {}", y);
    
    // Same result, but different mechanism

    // -----------------------------------------
    // PART 2: Type Change with Shadowing
    // -----------------------------------------
    println!("\n--- Type Change ---");
    
    // This is valid with shadowing
    let spaces = "   ";           // type: &str
    let spaces = spaces.len();    // type: usize
    println!("Number of spaces: {}", spaces);
    
    // With mut it would NOT work:
    // let mut text = "hello";
    // text = text.len();  // Error: different types
    
    // TODO: Use shadowing to convert a number to String
    let number = 42;
    // let number = ...;  // Convert to String
    println!("Number as i32: {}", number);

    // -----------------------------------------
    // PART 3: Shadowing in Scopes
    // -----------------------------------------
    println!("\n--- Shadowing in Scopes ---");
    
    let value = 10;
    println!("Value outside: {}", value);
    
    {
        // Shadow only inside the block
        let value = 99;
        println!("Value inside block: {}", value);
    }
    
    // Outside the block, returns to original
    println!("Value after block: {}", value);
    
    // TODO: Experiment with nested scopes
    // {
    //     let a = 1;
    //     {
    //         let a = 2;
    //         println!("inner a: {}", a);
    //     }
    //     println!("outer a: {}", a);
    // }

    // -----------------------------------------
    // PART 4: Constants
    // -----------------------------------------
    println!("\n--- Constants ---");
    
    println!("PI: {}", PI);
    println!("Max attempts: {}", MAX_ATTEMPTS);
    println!("Welcome: {}", WELCOME_MESSAGE);
    
    // Calculate circle area using constant
    let radius = 5.0;
    let area = PI * radius * radius;
    println!("Circle area (r={}): {:.2}", radius, area);

    println!("\n✅ Practice completed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shadowing() {
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        assert_eq!(x, 12);
    }

    #[test]
    fn test_type_change_shadowing() {
        let s = "hello";
        let s = s.len();
        assert_eq!(s, 5);
    }

    #[test]
    fn test_scope_shadowing() {
        let x = 1;
        {
            let x = 2;
            assert_eq!(x, 2);
        }
        assert_eq!(x, 1);
    }

    #[test]
    fn test_constants() {
        assert!((PI - 3.14159).abs() < 0.0001);
        assert_eq!(MAX_ATTEMPTS, 3);
    }
}
