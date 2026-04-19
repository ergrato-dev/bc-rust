// Practice 02: References
// =========================
// Complete the exercises following the instructions in README.md

fn main() {
    println!("=== Practice 02: References ===\n");
    
    exercise1();
    exercise2();
    exercise3();
    exercise4();
}

// Exercise 1: Immutable Reference
// TODO: Change the parameter to receive a reference
fn length(s: String) -> usize {
    s.len()
}

fn exercise1() {
    println!("--- Exercise 1: Immutable Reference ---");
    
    let text = String::from("Rust is awesome");
    
    // TODO: Call length() without moving text
    let len = length(text.clone()); // <- Modify this line
    
    // Uncomment when it works:
    // println!("'{}' has {} characters", text, len);
    println!("Length: {}", len);
    println!();
}

// Exercise 2: Mutable Reference
// TODO: Change the parameter to receive a mutable reference
fn add_exclamation(s: String) -> String {
    // TODO: Modify to use &mut String
    let mut result = s;
    result.push('!');
    result
}

fn exercise2() {
    println!("--- Exercise 2: Mutable Reference ---");
    
    let mut greeting = String::from("Hello");
    
    // TODO: Modify to use mutable reference
    greeting = add_exclamation(greeting);
    
    println!("Greeting: {}", greeting);
    println!();
}

// Exercise 3: Analyze which blocks compile
fn exercise3() {
    println!("--- Exercise 3: Multiple References ---");
    
    // Block A: Does it compile?
    {
        let s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        println!("  Block A: {} {}", r1, r2);
    }
    
    // Block B: Does it compile? (Uncomment to test)
    // {
    //     let mut s = String::from("hello");
    //     let r1 = &mut s;
    //     let r2 = &mut s;
    //     println!("  Block B: {} {}", r1, r2);
    // }
    
    // Block C: Does it compile? (Uncomment to test)
    // {
    //     let mut s = String::from("hello");
    //     let r1 = &s;
    //     let r2 = &mut s;
    //     println!("  Block C: {}", r1);
    // }
    
    println!();
}

// Exercise 4: Compare strings
// TODO: This function has a lifetime problem
// For now, return String instead of &String
fn longest(s1: &String, s2: &String) -> String {
    if s1.len() >= s2.len() {
        s1.clone()
    } else {
        s2.clone()
    }
}

fn exercise4() {
    println!("--- Exercise 4: Compare Strings ---");
    
    let short = String::from("Rust");
    let long = String::from("Programming");
    
    let result = longest(&short, &long);
    
    println!("  Short: '{}' ({} chars)", short, short.len());
    println!("  Long: '{}' ({} chars)", long, long.len());
    println!("  Longest: '{}'", result);
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_length_with_reference() {
        let s = String::from("hello");
        // If length uses &String, this should compile:
        // let len = length(&s);
        // assert_eq!(len, 5);
        // assert_eq!(s, "hello"); // s still valid
    }
    
    #[test]
    fn test_add_exclamation() {
        let mut s = String::from("test");
        // If add_exclamation uses &mut String:
        // add_exclamation(&mut s);
        // assert_eq!(s, "test!");
    }
    
    #[test]
    fn test_multiple_immutable_refs() {
        let s = String::from("rust");
        let r1 = &s;
        let r2 = &s;
        let r3 = &s;
        
        assert_eq!(*r1, *r2);
        assert_eq!(*r2, *r3);
    }
}
