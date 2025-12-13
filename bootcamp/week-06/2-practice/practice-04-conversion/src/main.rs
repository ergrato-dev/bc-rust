// Practice 04: Error Conversion
// ===================================
// Learn to convert between different error types and use Box<dyn Error>.
//
// OBJECTIVE:
// - Convert errors with map_err
// - Use Box<dyn Error> for heterogeneous errors
// - Add context to errors
//
// INSTRUCTIONS:
// 1. Complete conversion functions
// 2. Practice different handling strategies

use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;

fn main() {
    println!("=== Practice: Error Conversion ===\n");

    // Exercise 1: Basic map_err
    println!("1. Conversion with map_err:");
    println!("   parse_port('8080') → {:?}", parse_port("8080"));
    println!("   parse_port('abc') → {:?}", parse_port("abc"));

    // Exercise 2: Box<dyn Error>
    println!("\n2. Box<dyn Error>:");
    let _ = fs::write("data.txt", "42\n");
    match read_and_parse("data.txt") {
        Ok(n) => println!("   Number: {}", n),
        Err(e) => println!("   Error: {}", e),
    }
    let _ = fs::remove_file("data.txt");

    match read_and_parse("does_not_exist.txt") {
        Ok(n) => println!("   Number: {}", n),
        Err(e) => println!("   Error: {}", e),
    }

    // Exercise 3: Add context
    println!("\n3. With context:");
    let _ = fs::write("config.txt", "100\n");
    match read_with_context("config.txt") {
        Ok(n) => println!("   Value: {}", n),
        Err(e) => println!("   Error: {}", e),
    }
    let _ = fs::remove_file("config.txt");

    // Exercise 4: Result with multiple errors
    println!("\n4. Process multiple files:");
    let _ = fs::write("a.txt", "10\n");
    let _ = fs::write("b.txt", "20\n");
    match sum_files(&["a.txt", "b.txt", "c.txt"]) {
        Ok(sum) => println!("   Sum: {}", sum),
        Err(e) => println!("   Error: {}", e),
    }
    let _ = fs::remove_file("a.txt");
    let _ = fs::remove_file("b.txt");
}

// ============================================================================
// EXERCISE 1: Conversion with map_err
// ============================================================================
// Convert ParseIntError to a descriptive String.

fn parse_port(s: &str) -> Result<u16, String> {
    // TODO: Implement
    // Parse s to u16 and convert the error to a descriptive message
    // Example message: "Invalid port 'abc': invalid digit found in string"
    //
    // HINT: Use .map_err(|e| format!("Invalid port '{}': {}", s, e))
    todo!("Implement parse_port")
}

// ============================================================================
// EXERCISE 2: Box<dyn Error>
// ============================================================================
// Use Box<dyn Error> to handle different error types.

fn read_and_parse(path: &str) -> Result<i32, Box<dyn Error>> {
    // TODO: Implement
    // 1. Read file with fs::read_to_string(path)?
    // 2. Parse content to i32 with content.trim().parse()?
    // 3. Return Ok(number)
    //
    // Note: The ? converts automatically to Box<dyn Error>
    todo!("Implement read_and_parse")
}

// ============================================================================
// EXERCISE 3: Add context
// ============================================================================
// Wrap errors with additional context.

#[derive(Debug)]
struct ErrorWithContext {
    context: String,
    cause: Box<dyn Error>,
}

impl fmt::Display for ErrorWithContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.context, self.cause)
    }
}

impl Error for ErrorWithContext {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.cause.as_ref())
    }
}

// Trait extension to add context
trait WithContext<T> {
    fn context(self, msg: &str) -> Result<T, ErrorWithContext>;
}

impl<T, E: Error + 'static> WithContext<T> for Result<T, E> {
    fn context(self, msg: &str) -> Result<T, ErrorWithContext> {
        self.map_err(|e| ErrorWithContext {
            context: msg.to_string(),
            cause: Box::new(e),
        })
    }
}

fn read_with_context(path: &str) -> Result<i32, ErrorWithContext> {
    // TODO: Implement using the WithContext trait
    // 1. Read file and add context "Reading file"
    // 2. Parse and add context "Parsing number"
    //
    // HINT: 
    //   fs::read_to_string(path).context("Reading file")?
    todo!("Implement read_with_context")
}

// ============================================================================
// EXERCISE 4: Collection of Results
// ============================================================================
// Process multiple files and return the first error or the sum.

fn sum_files(paths: &[&str]) -> Result<i64, String> {
    // TODO: Implement
    // For each path in paths:
    //   1. Read the file
    //   2. Parse to i64
    //   3. Add to total
    // If any operation fails, return the error with context
    //
    // HINT: You can use a for loop with ? or .map().collect()
    todo!("Implement sum_files")
}

// ============================================================================
// BONUS: Collect Results
// ============================================================================
// Collect all results or return the first error.

fn parse_all(strings: &[&str]) -> Result<Vec<i32>, String> {
    // TODO: Implement
    // Parse each string to i32
    // Return Vec<i32> if all are valid
    // Return error if any fails
    //
    // HINT: Use .map(|s| s.parse()).collect()
    // collect() can collect Result<Vec<T>, E>
    todo!("Implement parse_all")
}

// ============================================================================
// TESTS
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_port_ok() {
        assert_eq!(parse_port("8080"), Ok(8080));
        assert_eq!(parse_port("443"), Ok(443));
    }

    #[test]
    fn test_parse_port_error() {
        let result = parse_port("abc");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("abc"));
    }

    #[test]
    fn test_read_and_parse_ok() {
        fs::write("test_num.txt", "123\n").unwrap();
        assert_eq!(read_and_parse("test_num.txt").unwrap(), 123);
        fs::remove_file("test_num.txt").unwrap();
    }

    #[test]
    fn test_read_and_parse_io_error() {
        assert!(read_and_parse("does_not_exist_xyz.txt").is_err());
    }

    #[test]
    fn test_read_and_parse_parse_error() {
        fs::write("test_bad.txt", "abc\n").unwrap();
        assert!(read_and_parse("test_bad.txt").is_err());
        fs::remove_file("test_bad.txt").unwrap();
    }

    #[test]
    fn test_read_with_context() {
        fs::write("test_ctx.txt", "42\n").unwrap();
        assert_eq!(read_with_context("test_ctx.txt").unwrap(), 42);
        fs::remove_file("test_ctx.txt").unwrap();
    }

    #[test]
    fn test_sum_files_ok() {
        fs::write("sum_a.txt", "10\n").unwrap();
        fs::write("sum_b.txt", "20\n").unwrap();
        assert_eq!(sum_files(&["sum_a.txt", "sum_b.txt"]), Ok(30));
        fs::remove_file("sum_a.txt").unwrap();
        fs::remove_file("sum_b.txt").unwrap();
    }

    #[test]
    fn test_sum_files_missing() {
        assert!(sum_files(&["does_not_exist_xyz.txt"]).is_err());
    }

    #[test]
    fn test_parse_all_ok() {
        assert_eq!(parse_all(&["1", "2", "3"]), Ok(vec![1, 2, 3]));
    }

    #[test]
    fn test_parse_all_error() {
        assert!(parse_all(&["1", "abc", "3"]).is_err());
    }
}
