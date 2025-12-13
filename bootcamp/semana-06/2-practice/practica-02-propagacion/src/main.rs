// Practice 02: Error Propagation
// ====================================
// Learn to use the ? operator for elegant error propagation.
//
// OBJECTIVE:
// - Use the ? operator to propagate errors
// - Understand automatic conversion with From
// - Chain multiple operations that can fail
//
// INSTRUCTIONS:
// 1. Complete each function using the ? operator
// 2. Run the program to verify your solution
// 3. Tests must pass: cargo test

use std::fs;
use std::io;
use std::path::Path;

fn main() {
    println!("=== Practice: Error Propagation ===\n");

    // Exercise 1: Read and parse
    println!("1. Read file and sum lines:");
    match sum_file_lines("numbers.txt") {
        Ok(sum) => println!("   Sum: {}", sum),
        Err(e) => println!("   Error: {}", e),
    }

    // Create test file
    let _ = fs::write("test_numbers.txt", "10\n20\n30\n");
    match sum_file_lines("test_numbers.txt") {
        Ok(sum) => println!("   Sum of test_numbers.txt: {}", sum),
        Err(e) => println!("   Error: {}", e),
    }
    let _ = fs::remove_file("test_numbers.txt");

    // Exercise 2: Chained validation
    println!("\n2. Validate user:");
    println!("   'Ana' (25) → {:?}", validate_user("Ana", 25));
    println!("   '' (25) → {:?}", validate_user("", 25));
    println!("   'Bob' (200) → {:?}", validate_user("Bob", 200));

    // Exercise 3: Process configuration
    println!("\n3. Process config:");
    let _ = fs::write("config.txt", "port=8080\nhost=localhost\n");
    match get_port("config.txt") {
        Ok(port) => println!("   Port: {}", port),
        Err(e) => println!("   Error: {}", e),
    }
    let _ = fs::remove_file("config.txt");
}

// ============================================================================
// EXERCISE 1: Read file and sum numbers
// ============================================================================
// Read a file with numbers (one per line) and return their sum.
// Use the ? operator to propagate errors.

fn sum_file_lines(path: &str) -> Result<i64, String> {
    // TODO: Implement using ?
    // 1. Read the file with fs::read_to_string(path)
    //    - Convert the error: .map_err(|e| format!("Read error: {}", e))?
    // 2. For each line, parse to i64 and sum
    //    - You can use a for loop or .lines().map().sum()
    // 3. Return Ok(sum)
    //
    // HINT: For parsing each line:
    //   line.trim().parse::<i64>().map_err(|_| format!("Invalid line: {}", line))?
    todo!("Implement sum_file_lines")
}

// ============================================================================
// EXERCISE 2: Chained validation
// ============================================================================
// Validate name and age, propagating the first error found.

#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: u8,
}

fn validate_user(name: &str, age: i32) -> Result<User, String> {
    // TODO: Implement using ?
    // 1. Validate name with validate_name(name)?
    // 2. Validate age with validate_age(age)?
    // 3. Return Ok(User { name, age })
    todo!("Implement validate_user")
}

fn validate_name(name: &str) -> Result<String, String> {
    // TODO: Implement
    // - If empty, Err("Name cannot be empty")
    // - If less than 2 characters, Err("Name too short")
    // - Otherwise, Ok(name.to_string())
    todo!("Implement validate_name")
}

fn validate_age(age: i32) -> Result<u8, String> {
    // TODO: Implement
    // - If negative, Err("Age cannot be negative")
    // - If > 150, Err("Age out of range")
    // - Otherwise, Ok(age as u8)
    todo!("Implement validate_age")
}

// ============================================================================
// EXERCISE 3: Process configuration
// ============================================================================
// Read a config file and extract the port.
// Format: key=value (one per line)

fn get_port(path: &str) -> Result<u16, String> {
    // TODO: Implement using ?
    // 1. Read file
    // 2. Find line that starts with "port="
    // 3. Extract the value after =
    // 4. Parse to u16
    //
    // HINT: Use .lines().find(|l| l.starts_with("port="))
    todo!("Implement get_port")
}

// ============================================================================
// TESTS
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_lines_ok() {
        fs::write("test_sum.txt", "1\n2\n3\n4\n5").unwrap();
        assert_eq!(sum_file_lines("test_sum.txt"), Ok(15));
        fs::remove_file("test_sum.txt").unwrap();
    }

    #[test]
    fn test_sum_lines_file_not_found() {
        assert!(sum_file_lines("does_not_exist.txt").is_err());
    }

    #[test]
    fn test_sum_lines_invalid_format() {
        fs::write("test_invalid.txt", "1\nabc\n3").unwrap();
        assert!(sum_file_lines("test_invalid.txt").is_err());
        fs::remove_file("test_invalid.txt").unwrap();
    }

    #[test]
    fn test_validate_user_ok() {
        let result = validate_user("Ana", 25);
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "Ana");
        assert_eq!(user.age, 25);
    }

    #[test]
    fn test_validate_user_empty_name() {
        assert!(validate_user("", 25).is_err());
    }

    #[test]
    fn test_validate_user_invalid_age() {
        assert!(validate_user("Ana", -5).is_err());
        assert!(validate_user("Ana", 200).is_err());
    }

    #[test]
    fn test_get_port_ok() {
        fs::write("test_config.txt", "host=localhost\nport=3000\n").unwrap();
        assert_eq!(get_port("test_config.txt"), Ok(3000));
        fs::remove_file("test_config.txt").unwrap();
    }

    #[test]
    fn test_get_port_not_found() {
        assert!(get_port("does_not_exist.txt").is_err());
    }

    #[test]
    fn test_get_port_no_port() {
        fs::write("test_no_port.txt", "host=localhost\n").unwrap();
        assert!(get_port("test_no_port.txt").is_err());
        fs::remove_file("test_no_port.txt").unwrap();
    }
}
