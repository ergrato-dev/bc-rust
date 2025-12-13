// Practice 03: Borrow Checker
// ============================
// Learn to resolve borrow checker errors

fn main() {
    println!("=== Practice 03: Borrow Checker ===\n");
    
    exercise1_solution_a();
    exercise1_solution_b();
    exercise2();
    exercise3();
    exercise4();
}

// Exercise 1 - Solution A: Reorganize code
fn exercise1_solution_a() {
    println!("--- Exercise 1A: Reorganize ---");
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // TODO: Reorganize to make it work
    // Hint: use first BEFORE modifying
    
    let first = &numbers[0];
    println!("  First: {}", first);
    // numbers.push(6); // <- Where should this go?
    
    println!("  Vector: {:?}", numbers);
    println!();
}

// Exercise 1 - Solution B: Clone the value
fn exercise1_solution_b() {
    println!("--- Exercise 1B: Clone ---");
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // TODO: Clone the value to not depend on the reference
    let first = numbers[0]; // <- i32 is Copy, doesn't need clone!
    
    numbers.push(6);
    
    println!("  First (copied): {}", first);
    println!("  Vector: {:?}", numbers);
    println!();
}

// Exercise 2: Conflicting Borrows
fn exercise2() {
    println!("--- Exercise 2: Conflicting Borrows ---");
    
    let mut text = String::from("Hello");
    
    // TODO: Reorganize this code to compile
    // The goal is to print the original text AND then modify it
    
    // Version with error:
    // let r1 = &text;
    // let r2 = &text;
    // let r3 = &mut text;  // ERROR: conflict
    // println!("{}, {}", r1, r2);
    // r3.push_str(" world");
    
    // Corrected version:
    {
        let r1 = &text;
        let r2 = &text;
        println!("  Immutable refs: {}, {}", r1, r2);
    } // r1 and r2 end here
    
    {
        let r3 = &mut text;
        r3.push_str(" world");
        println!("  Mutable ref: {}", r3);
    }
    
    println!();
}

// Exercise 3: Return ownership instead of reference
// This function CANNOT return &String to a local variable
fn create_message() -> String {
    let s = String::from("Hello from the function");
    s  // Return ownership, not reference
}

fn exercise3() {
    println!("--- Exercise 3: Return Ownership ---");
    
    let message = create_message();
    println!("  Message: {}", message);
    println!();
}

// Exercise 4: Lifetime Analysis (NLL)
fn exercise4() {
    println!("--- Exercise 4: Non-Lexical Lifetimes ---");
    
    let mut s = String::from("hello");
    
    let r1 = &s;           // Immutable borrow starts
    println!("  r1: {}", r1);  // Last use of r1 → borrow ENDS here
    
    let r2 = &s;           // New immutable borrow
    let r3 = &s;           // Another immutable borrow
    println!("  r2: {}, r3: {}", r2, r3);  // Last use of r2 and r3
    
    // Here r1, r2, r3 no longer exist (NLL)
    
    let r4 = &mut s;       // ✅ OK: no active immutable refs
    r4.push_str("!");
    println!("  r4: {}", r4);
    
    println!("  Final: {}", s);
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_message() {
        let msg = create_message();
        assert_eq!(msg, "Hello from the function");
    }
    
    #[test]
    fn test_nll_allows_sequential() {
        let mut s = String::from("test");
        
        let r1 = &s;
        let _ = r1.len(); // Use r1
        // r1 is no longer used
        
        let r2 = &mut s; // OK thanks to NLL
        r2.push('!');
        
        assert_eq!(s, "test!");
    }
}
