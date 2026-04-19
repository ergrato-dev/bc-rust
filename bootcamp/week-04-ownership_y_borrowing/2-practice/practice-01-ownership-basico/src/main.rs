// Practice 01: Basic Ownership
// ================================
// Complete the exercises following the instructions in README.md

fn main() {
    println!("=== Practice 01: Basic Ownership ===\n");
    
    exercise1();
    exercise2();
    exercise3();
    exercise4();
}

// Exercise 1: Fix using clone()
fn exercise1() {
    println!("--- Exercise 1: Clone ---");
    
    let message = String::from("Hello, Rust!");
    // TODO: Fix this line so message remains valid
    let copy = message; // <- Modify here
    
    // Uncomment when fixed:
    // println!("Original: {}", message);
    // println!("Copy: {}", copy);
    
    println!();
}

// Exercise 2: Fix the function to not take ownership
fn exercise2() {
    println!("--- Exercise 2: Functions ---");
    
    let name = String::from("Ferris");
    print_name(name);
    
    // Uncomment when fixed:
    // println!("Name after: {}", name);
    
    println!();
}

// TODO: Modify this function to not take ownership
fn print_name(n: String) {
    println!("Printing: {}", n);
}

// Exercise 3: Identify the valid variable
fn exercise3() {
    println!("--- Exercise 3: Chain of Moves ---");
    
    let a = String::from("Rust");
    let b = a;
    let c = b;
    let d = c;
    
    // TODO: Uncomment ONLY the line that compiles
    // println!("a = {}", a);
    // println!("b = {}", b);
    // println!("c = {}", c);
    // println!("d = {}", d);
    
    println!();
}

// Exercise 4: Observe the Drop order
fn exercise4() {
    println!("--- Exercise 4: Scope and Drop ---");
    
    struct Resource {
        name: String,
    }

    impl Drop for Resource {
        fn drop(&mut self) {
            println!("  Drop of {}", self.name);
        }
    }

    println!("  Start");
    
    let _r1 = Resource { name: String::from("R1") };
    println!("  Creating R1");
    
    {
        let _r2 = Resource { name: String::from("R2") };
        println!("  Creating R2");
    } // When is drop called for R2?
    
    let _r3 = Resource { name: String::from("R3") };
    println!("  Creating R3");
    
    println!("  End");
    // In what order is drop called for R1 and R3?
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_clone_preserves_original() {
        let original = String::from("test");
        let copy = original.clone();
        
        assert_eq!(original, "test");
        assert_eq!(copy, "test");
    }
    
    #[test]
    fn test_reference_does_not_move() {
        let s = String::from("hello");
        let r = &s;
        
        assert_eq!(s, "hello");
        assert_eq!(*r, "hello");
    }
}
