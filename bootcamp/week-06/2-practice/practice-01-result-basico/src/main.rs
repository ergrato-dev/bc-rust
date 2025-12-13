// Practice 01: Basic Result
// ==========================
// Learn to use Result<T, E> to handle recoverable errors.
//
// OBJECTIVE:
// - Create functions that return Result
// - Use match to handle Ok and Err
// - Apply methods like unwrap_or, map, and_then
//
// INSTRUCTIONS:
// 1. Complete each function as indicated
// 2. Run the program to verify your solution
// 3. Tests must pass: cargo test

fn main() {
    println!("=== Practice: Basic Result ===\n");

    // Exercise 1: Safe division
    println!("1. Safe division:");
    match divide(10.0, 2.0) {
        Ok(result) => println!("   10 / 2 = {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    match divide(10.0, 0.0) {
        Ok(result) => println!("   10 / 0 = {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Exercise 2: Parse age
    println!("\n2. Parse age:");
    println!("   '25' → {:?}", parse_age("25"));
    println!("   '-5' → {:?}", parse_age("-5"));
    println!("   'abc' → {:?}", parse_age("abc"));

    // Exercise 3: Chain operations
    println!("\n3. Chained operation:");
    println!("   calculate('10', 2.0) → {:?}", calculate("10", 2.0));
    println!("   calculate('abc', 2.0) → {:?}", calculate("abc", 2.0));
    println!("   calculate('10', 0.0) → {:?}", calculate("10", 0.0));

    // Exercise 4: Default value
    println!("\n4. With default value:");
    println!("   config_or_default('8080') → {}", config_or_default("8080"));
    println!("   config_or_default('abc') → {}", config_or_default("abc"));
}

// ============================================================================
// EXERCISE 1: Safe division
// ============================================================================
// Implement a function that divides two numbers.
// Returns Err if divisor is zero.

fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    // TODO: Implement
    // - If divisor == 0.0, return Err("Division by zero")
    // - Otherwise, return Ok(dividend / divisor)
    todo!("Implement divide")
}

// ============================================================================
// EXERCISE 2: Parse age with validation
// ============================================================================
// Parse a string to age (u8), validating it's a valid number.
// A valid age is between 0 and 150.

fn parse_age(s: &str) -> Result<u8, String> {
    // TODO: Implement
    // 1. Try to parse s to i32 (to detect negatives)
    //    - If fails, return Err("Not a valid number")
    // 2. Validate range 0..=150
    //    - If negative, return Err("Age cannot be negative")
    //    - If > 150, return Err("Age out of range")
    // 3. Convert to u8 and return Ok
    //
    // HINT: Use parse::<i32>() first
    todo!("Implement parse_age")
}

// ============================================================================
// EXERCISE 3: Chain operations with and_then
// ============================================================================
// Parse a string to number and then divide by the given divisor.
// Chain operations using and_then or the ? operator.

fn calculate(text: &str, divisor: f64) -> Result<f64, String> {
    // TODO: Implement
    // 1. Parse text to f64
    // 2. Divide by divisor using the divide function
    // 
    // HINT: You can use:
    //   text.parse::<f64>().map_err(|_| "...".to_string())?.pipe(|n| divide(n, divisor))
    // Or:
    //   text.parse::<f64>().map_err(...).and_then(|n| divide(n, divisor))
    todo!("Implement calculate")
}

// ============================================================================
// EXERCISE 4: Default value with unwrap_or
// ============================================================================
// Try to parse a port, if fails use 3000 as default.

fn config_or_default(port_str: &str) -> u16 {
    // TODO: Implement
    // Parse port_str to u16, if fails return 3000
    // 
    // HINT: Use .unwrap_or(3000)
    todo!("Implement config_or_default")
}

// ============================================================================
// TESTS
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_ok() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert_eq!(divide(7.0, 2.0), Ok(3.5));
    }

    #[test]
    fn test_divide_error() {
        assert!(divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_parse_age_ok() {
        assert_eq!(parse_age("25"), Ok(25));
        assert_eq!(parse_age("0"), Ok(0));
        assert_eq!(parse_age("150"), Ok(150));
    }

    #[test]
    fn test_parse_age_negative() {
        assert!(parse_age("-5").is_err());
    }

    #[test]
    fn test_parse_age_invalid() {
        assert!(parse_age("abc").is_err());
        assert!(parse_age("200").is_err());
    }

    #[test]
    fn test_calculate_ok() {
        assert_eq!(calculate("10", 2.0), Ok(5.0));
    }

    #[test]
    fn test_calculate_parse_error() {
        assert!(calculate("abc", 2.0).is_err());
    }

    #[test]
    fn test_calculate_division_error() {
        assert!(calculate("10", 0.0).is_err());
    }

    #[test]
    fn test_config_or_default_ok() {
        assert_eq!(config_or_default("8080"), 8080);
    }

    #[test]
    fn test_config_or_default_invalid() {
        assert_eq!(config_or_default("abc"), 3000);
    }
}
