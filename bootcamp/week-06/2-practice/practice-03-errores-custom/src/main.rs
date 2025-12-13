// Practice 03: Custom Errors
// ====================================
// Learn to create expressive and well-structured error types.
//
// OBJECTIVE:
// - Define an error enum with multiple variants
// - Implement Display and Error
// - Implement From for automatic conversion
//
// INSTRUCTIONS:
// 1. Complete the error type definition
// 2. Implement necessary traits
// 3. Use the type in functions

use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;

fn main() {
    println!("=== Practice: Custom Errors ===\n");

    // Test error type
    println!("1. Error types:");
    let e1 = ConfigError::FileNotFound("config.toml".to_string());
    println!("   {}", e1);
    
    let e2 = ConfigError::MissingField("port".to_string());
    println!("   {}", e2);
    
    let e3 = ConfigError::InvalidValue {
        field: "port".to_string(),
        value: "abc".to_string(),
        message: "must be a number".to_string(),
    };
    println!("   {}", e3);

    // Test usage
    println!("\n2. Load configuration:");
    
    // Create valid test file
    let _ = fs::write("test_config.toml", "port = 8080\nhost = localhost\n");
    match load_config("test_config.toml") {
        Ok(config) => println!("   Config: {:?}", config),
        Err(e) => println!("   Error: {}", e),
    }
    let _ = fs::remove_file("test_config.toml");

    // File that doesn't exist
    match load_config("does_not_exist.toml") {
        Ok(config) => println!("   Config: {:?}", config),
        Err(e) => println!("   Error: {}", e),
    }

    // File with invalid value
    let _ = fs::write("bad_config.toml", "port = abc\n");
    match load_config("bad_config.toml") {
        Ok(config) => println!("   Config: {:?}", config),
        Err(e) => println!("   Error: {}", e),
    }
    let _ = fs::remove_file("bad_config.toml");
}

// ============================================================================
// EXERCISE 1: Define the error type
// ============================================================================

#[derive(Debug)]
enum ConfigError {
    // TODO: Define variants:
    // - FileNotFound(String) - file path
    // - ReadError(io::Error) - underlying I/O error
    // - MissingField(String) - field name
    // - InvalidValue { field: String, value: String, message: String }
    FileNotFound(String),
    ReadError(io::Error),
    MissingField(String),
    InvalidValue {
        field: String,
        value: String,
        message: String,
    },
}

// ============================================================================
// EXERCISE 2: Implement Display
// ============================================================================

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Implement descriptive messages for each variant
        // 
        // Example outputs:
        // - "File not found: config.toml"
        // - "Read error: Permission denied"
        // - "Missing field: port"
        // - "Invalid value in 'port': 'abc' - must be a number"
        todo!("Implement Display for ConfigError")
    }
}

// ============================================================================
// EXERCISE 3: Implement Error with source()
// ============================================================================

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // TODO: Return underlying cause if exists
        // - For ReadError, return Some(&io_error)
        // - For others, return None
        todo!("Implement source() for ConfigError")
    }
}

// ============================================================================
// EXERCISE 4: Implement From<io::Error>
// ============================================================================

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        // TODO: Convert io::Error to ConfigError::ReadError
        todo!("Implement From<io::Error>")
    }
}

impl From<ParseIntError> for ConfigError {
    fn from(_err: ParseIntError) -> Self {
        // For ParseIntError, we need more context
        // For now return a generic error
        ConfigError::InvalidValue {
            field: "unknown".to_string(),
            value: "".to_string(),
            message: "parse error".to_string(),
        }
    }
}

// ============================================================================
// EXERCISE 5: Use the error type
// ============================================================================

#[derive(Debug)]
struct Config {
    port: u16,
    host: String,
}

fn load_config(path: &str) -> Result<Config, ConfigError> {
    // TODO: Implement
    // 1. Read file (use ? - will convert automatically)
    // 2. Find line with "port = "
    //    - If doesn't exist, return Err(ConfigError::MissingField("port"))
    // 3. Parse port value
    //    - If fails, return appropriate ConfigError::InvalidValue
    // 4. Find line with "host = "
    //    - If doesn't exist, use "localhost" as default
    // 5. Return Ok(Config { port, host })
    //
    // File format:
    // port = 8080
    // host = localhost
    todo!("Implement load_config")
}

// Helper function to extract value from a "key = value" line
fn extract_value<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    if line.starts_with(key) {
        line.split('=').nth(1).map(|v| v.trim())
    } else {
        None
    }
}

// ============================================================================
// TESTS
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let e = ConfigError::FileNotFound("test.toml".to_string());
        assert!(e.to_string().contains("test.toml"));

        let e = ConfigError::MissingField("port".to_string());
        assert!(e.to_string().contains("port"));
    }

    #[test]
    fn test_error_from_io() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "not found");
        let config_err: ConfigError = io_err.into();
        assert!(matches!(config_err, ConfigError::ReadError(_)));
    }

    #[test]
    fn test_load_config_ok() {
        fs::write("test_ok.toml", "port = 3000\nhost = example.com\n").unwrap();
        let config = load_config("test_ok.toml").unwrap();
        assert_eq!(config.port, 3000);
        assert_eq!(config.host, "example.com");
        fs::remove_file("test_ok.toml").unwrap();
    }

    #[test]
    fn test_load_config_missing_port() {
        fs::write("test_no_port.toml", "host = localhost\n").unwrap();
        let result = load_config("test_no_port.toml");
        assert!(matches!(result, Err(ConfigError::MissingField(_))));
        fs::remove_file("test_no_port.toml").unwrap();
    }

    #[test]
    fn test_load_config_invalid_port() {
        fs::write("test_bad_port.toml", "port = abc\n").unwrap();
        let result = load_config("test_bad_port.toml");
        assert!(matches!(result, Err(ConfigError::InvalidValue { .. })));
        fs::remove_file("test_bad_port.toml").unwrap();
    }
}
