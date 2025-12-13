// ============================================
// Practice 03: Text Types
// ============================================
// Objective: Master char, &str, String and bool
// ============================================

fn main() {
    println!("=== Practice 03: Text Types ===\n");

    // -----------------------------------------
    // PART 1: Characters (char)
    // -----------------------------------------
    println!("--- Characters ---");
    
    // char uses single quotes ''
    // Represents a Unicode value (4 bytes)
    let letter: char = 'A';
    let digit: char = '7';
    let symbol: char = '@';
    let emoji: char = '🦀';       // Rust has an emoji!
    let accent: char = 'ñ';
    
    println!("Letter: {}", letter);
    println!("Digit: {}", digit);
    println!("Symbol: {}", symbol);
    println!("Emoji: {}", emoji);
    println!("Accent: {}", accent);
    
    // Useful char methods
    println!("\nChar methods:");
    println!("'A'.is_alphabetic(): {}", 'A'.is_alphabetic());
    println!("'7'.is_numeric(): {}", '7'.is_numeric());
    println!("'a'.is_lowercase(): {}", 'a'.is_lowercase());
    println!("'A'.to_lowercase(): {}", 'A'.to_lowercase());

    // -----------------------------------------
    // PART 2: String Slices (&str)
    // -----------------------------------------
    println!("\n--- String Slices (&str) ---");
    
    // &str are references to text, immutable
    let greeting: &str = "Hello, Rust!";
    let empty: &str = "";
    
    println!("Greeting: {}", greeting);
    println!("Length: {} bytes", greeting.len());
    println!("Is empty?: {}", empty.is_empty());
    
    // TODO: Declare a &str with your favorite quote
    // let quote: &str = ...;
    // println!("Quote: {}", quote);

    // -----------------------------------------
    // PART 3: String (owned)
    // -----------------------------------------
    println!("\n--- String (owned) ---");
    
    // String is text on the heap, mutable
    let mut message = String::from("Hello");
    println!("Initial: {}", message);
    
    // push_str adds a &str
    message.push_str(", world");
    println!("After push_str: {}", message);
    
    // push adds a char
    message.push('!');
    println!("After push: {}", message);
    
    // Another way to create String
    let other = "Rust".to_string();
    println!("With to_string(): {}", other);
    
    // TODO: Create a String and modify it
    // let mut my_string = String::from(...);
    // my_string.push_str(...);

    // -----------------------------------------
    // PART 4: Booleans
    // -----------------------------------------
    println!("\n--- Booleans ---");
    
    let is_rust_cool: bool = true;
    let is_learning: bool = true;
    
    println!("Is Rust cool? {}", is_rust_cool);
    println!("Am I learning? {}", is_learning);
    
    // Boolean operations
    println!("\nBoolean operations:");
    println!("true AND false = {}", true && false);
    println!("true OR false = {}", true || false);
    println!("NOT true = {}", !true);

    println!("\n✅ Practice completed");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_char_methods() {
        assert!('A'.is_alphabetic());
        assert!('7'.is_numeric());
        assert!('a'.is_lowercase());
        assert!('A'.is_uppercase());
    }

    #[test]
    fn test_string_operations() {
        let mut s = String::from("Hello");
        s.push_str(", World");
        s.push('!');
        assert_eq!(s, "Hello, World!");
    }

    #[test]
    fn test_boolean_logic() {
        assert_eq!(true && true, true);
        assert_eq!(true && false, false);
        assert_eq!(true || false, true);
        assert_eq!(!true, false);
    }
}
