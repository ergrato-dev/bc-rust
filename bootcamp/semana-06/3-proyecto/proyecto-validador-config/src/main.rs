// Weekly Project: Configuration Validator
// =============================================
// 
// Build a robust system to load, validate and process
// configuration files with complete error handling.
//
// DOMAIN: Configuration system for a web application
//
// FEATURES:
// 1. Read configuration file
// 2. Parse key=value format
// 3. Validate required fields and values
// 4. Provide default values
// 5. Report all errors found

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;

fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║     Configuration Validator v1.0             ║");
    println!("╚══════════════════════════════════════════════╝\n");

    // Create example configuration file
    create_example_config();

    // Load and display configuration
    match Config::load("app.config") {
        Ok(config) => {
            println!("✓ Configuration loaded successfully:\n");
            println!("{}", config);
        }
        Err(errors) => {
            println!("✗ Configuration errors:\n");
            for (i, e) in errors.iter().enumerate() {
                println!("  {}. {}", i + 1, e);
            }
        }
    }

    // Cleanup
    let _ = fs::remove_file("app.config");

    // Test with invalid configuration
    println!("\n--- Testing invalid configuration ---\n");
    create_invalid_config();
    
    match Config::load("bad.config") {
        Ok(config) => println!("Config: {}", config),
        Err(errors) => {
            println!("✗ Found {} errors:", errors.len());
            for e in &errors {
                println!("  • {}", e);
            }
        }
    }
    
    let _ = fs::remove_file("bad.config");
}

fn create_example_config() {
    let content = r#"# Application configuration
name = MyApp
version = 1.0.0
port = 8080
host = localhost
max_connections = 100
timeout_ms = 5000
debug = true
"#;
    fs::write("app.config", content).expect("Could not create file");
}

fn create_invalid_config() {
    let content = r#"# Configuration with errors
name = 
port = abc
max_connections = -50
"#;
    fs::write("bad.config", content).expect("Could not create file");
}

// ============================================================================
// ERROR TYPES
// ============================================================================

#[derive(Debug, Clone)]
pub enum ConfigError {
    /// Error reading the file
    IoError(String),
    /// Line with invalid format
    InvalidFormat { line: usize, content: String },
    /// Required field missing
    MissingField(String),
    /// Empty value for a field
    EmptyValue(String),
    /// Value with incorrect type
    WrongType { field: String, expected: String, found: String },
    /// Value outside allowed range
    OutOfRange { field: String, value: String, range: String },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::IoError(msg) => write!(f, "I/O error: {}", msg),
            ConfigError::InvalidFormat { line, content } => {
                write!(f, "Line {}: invalid format '{}'", line, content)
            }
            ConfigError::MissingField(field) => {
                write!(f, "Required field missing: '{}'", field)
            }
            ConfigError::EmptyValue(field) => {
                write!(f, "Empty value for field '{}'", field)
            }
            ConfigError::WrongType { field, expected, found } => {
                write!(f, "Field '{}': expected {}, found '{}'", field, expected, found)
            }
            ConfigError::OutOfRange { field, value, range } => {
                write!(f, "Field '{}': value {} out of range ({})", field, value, range)
            }
        }
    }
}

impl Error for ConfigError {}

// ============================================================================
// CONFIGURATION
// ============================================================================

#[derive(Debug)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub port: u16,
    pub host: String,
    pub max_connections: u32,
    pub timeout_ms: u64,
    pub debug: bool,
}

impl Config {
    /// Required fields that must be present
    const REQUIRED_FIELDS: &'static [&'static str] = &["name", "port"];

    /// Load configuration from a file.
    /// Returns the configuration or a list of all errors found.
    pub fn load(path: &str) -> Result<Self, Vec<ConfigError>> {
        let mut errors = Vec::new();

        // Read file
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                return Err(vec![ConfigError::IoError(e.to_string())]);
            }
        };

        // Parse to HashMap
        let values = Self::parse_content(&content, &mut errors);

        // Validate required fields
        Self::validate_required(&values, &mut errors);

        // Extract and validate each field
        let name = Self::extract_string(&values, "name", &mut errors);
        let version = Self::extract_string_default(&values, "version", "0.0.0");
        let port = Self::extract_u16(&values, "port", &mut errors);
        let host = Self::extract_string_default(&values, "host", "localhost");
        let max_connections = Self::extract_u32_range(&values, "max_connections", 1, 10000, 100, &mut errors);
        let timeout_ms = Self::extract_u64_default(&values, "timeout_ms", 3000);
        let debug = Self::extract_bool_default(&values, "debug", false);

        // If there are errors, return them
        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(Config {
            name,
            version,
            port,
            host,
            max_connections,
            timeout_ms,
            debug,
        })
    }

    /// Parse file content to a HashMap
    fn parse_content(content: &str, errors: &mut Vec<ConfigError>) -> HashMap<String, String> {
        let mut map = HashMap::new();

        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            
            // Ignore empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Find the = separator
            if let Some(pos) = line.find('=') {
                let key = line[..pos].trim().to_string();
                let value = line[pos + 1..].trim().to_string();
                map.insert(key, value);
            } else {
                errors.push(ConfigError::InvalidFormat {
                    line: line_num + 1,
                    content: line.to_string(),
                });
            }
        }

        map
    }

    /// Validate that all required fields are present
    fn validate_required(values: &HashMap<String, String>, errors: &mut Vec<ConfigError>) {
        for field in Self::REQUIRED_FIELDS {
            if !values.contains_key(*field) {
                errors.push(ConfigError::MissingField(field.to_string()));
            }
        }
    }

    /// Extract a required string field
    fn extract_string(
        values: &HashMap<String, String>,
        field: &str,
        errors: &mut Vec<ConfigError>,
    ) -> String {
        match values.get(field) {
            Some(v) if !v.is_empty() => v.clone(),
            Some(_) => {
                errors.push(ConfigError::EmptyValue(field.to_string()));
                String::new()
            }
            None => String::new(), // Already reported in validate_required
        }
    }

    /// Extract a string field with default value
    fn extract_string_default(
        values: &HashMap<String, String>,
        field: &str,
        default: &str,
    ) -> String {
        values.get(field)
            .filter(|v| !v.is_empty())
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    /// Extract a u16 field
    fn extract_u16(
        values: &HashMap<String, String>,
        field: &str,
        errors: &mut Vec<ConfigError>,
    ) -> u16 {
        match values.get(field) {
            Some(v) => match v.parse::<u16>() {
                Ok(n) => n,
                Err(_) => {
                    errors.push(ConfigError::WrongType {
                        field: field.to_string(),
                        expected: "integer 0-65535".to_string(),
                        found: v.clone(),
                    });
                    0
                }
            }
            None => 0, // Already reported
        }
    }

    /// Extract u32 with range validation
    fn extract_u32_range(
        values: &HashMap<String, String>,
        field: &str,
        min: u32,
        max: u32,
        default: u32,
        errors: &mut Vec<ConfigError>,
    ) -> u32 {
        match values.get(field) {
            Some(v) => {
                // First try as i64 to detect negatives
                match v.parse::<i64>() {
                    Ok(n) if n < 0 => {
                        errors.push(ConfigError::OutOfRange {
                            field: field.to_string(),
                            value: v.clone(),
                            range: format!("{}-{}", min, max),
                        });
                        default
                    }
                    Ok(n) if (n as u32) < min || (n as u32) > max => {
                        errors.push(ConfigError::OutOfRange {
                            field: field.to_string(),
                            value: v.clone(),
                            range: format!("{}-{}", min, max),
                        });
                        default
                    }
                    Ok(n) => n as u32,
                    Err(_) => {
                        errors.push(ConfigError::WrongType {
                            field: field.to_string(),
                            expected: "integer".to_string(),
                            found: v.clone(),
                        });
                        default
                    }
                }
            }
            None => default,
        }
    }

    /// Extract u64 with default
    fn extract_u64_default(
        values: &HashMap<String, String>,
        field: &str,
        default: u64,
    ) -> u64 {
        values.get(field)
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    /// Extract bool with default
    fn extract_bool_default(
        values: &HashMap<String, String>,
        field: &str,
        default: bool,
    ) -> bool {
        values.get(field)
            .map(|v| matches!(v.to_lowercase().as_str(), "true" | "1" | "yes" | "on"))
            .unwrap_or(default)
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Name:            {}", self.name)?;
        writeln!(f, "  Version:         {}", self.version)?;
        writeln!(f, "  Host:            {}", self.host)?;
        writeln!(f, "  Port:            {}", self.port)?;
        writeln!(f, "  Max connections: {}", self.max_connections)?;
        writeln!(f, "  Timeout:         {}ms", self.timeout_ms)?;
        writeln!(f, "  Debug:           {}", self.debug)
    }
}

// ============================================================================
// TESTS
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    fn valid_config() -> &'static str {
        "name = TestApp\nport = 3000\n"
    }

    fn complete_config() -> &'static str {
        r#"name = TestApp
version = 2.0.0
port = 8080
host = 0.0.0.0
max_connections = 500
timeout_ms = 10000
debug = true
"#
    }

    #[test]
    fn test_load_minimal_config() {
        fs::write("test_min.config", valid_config()).unwrap();
        let config = Config::load("test_min.config").unwrap();
        assert_eq!(config.name, "TestApp");
        assert_eq!(config.port, 3000);
        assert_eq!(config.host, "localhost"); // default
        fs::remove_file("test_min.config").unwrap();
    }

    #[test]
    fn test_load_complete_config() {
        fs::write("test_full.config", complete_config()).unwrap();
        let config = Config::load("test_full.config").unwrap();
        assert_eq!(config.name, "TestApp");
        assert_eq!(config.version, "2.0.0");
        assert_eq!(config.port, 8080);
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.max_connections, 500);
        assert_eq!(config.timeout_ms, 10000);
        assert!(config.debug);
        fs::remove_file("test_full.config").unwrap();
    }

    #[test]
    fn test_file_not_found() {
        let result = Config::load("does_not_exist_xyz.config");
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(matches!(errors[0], ConfigError::IoError(_)));
    }

    #[test]
    fn test_missing_field() {
        fs::write("test_missing.config", "host = localhost\n").unwrap();
        let result = Config::load("test_missing.config");
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, ConfigError::MissingField(f) if f == "name")));
        fs::remove_file("test_missing.config").unwrap();
    }

    #[test]
    fn test_empty_value() {
        fs::write("test_empty.config", "name = \nport = 8080\n").unwrap();
        let result = Config::load("test_empty.config");
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, ConfigError::EmptyValue(_))));
        fs::remove_file("test_empty.config").unwrap();
    }

    #[test]
    fn test_wrong_type() {
        fs::write("test_type.config", "name = App\nport = abc\n").unwrap();
        let result = Config::load("test_type.config");
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, ConfigError::WrongType { .. })));
        fs::remove_file("test_type.config").unwrap();
    }

    #[test]
    fn test_out_of_range() {
        fs::write("test_range.config", "name = App\nport = 8080\nmax_connections = -10\n").unwrap();
        let result = Config::load("test_range.config");
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, ConfigError::OutOfRange { .. })));
        fs::remove_file("test_range.config").unwrap();
    }

    #[test]
    fn test_multiple_errors() {
        fs::write("test_multi.config", "port = abc\nmax_connections = -5\n").unwrap();
        let result = Config::load("test_multi.config");
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.len() >= 2); // At least missing name and invalid port
        fs::remove_file("test_multi.config").unwrap();
    }

    #[test]
    fn test_comments_ignored() {
        fs::write("test_comments.config", "# Comment\nname = App\n# Another\nport = 80\n").unwrap();
        let config = Config::load("test_comments.config").unwrap();
        assert_eq!(config.name, "App");
        fs::remove_file("test_comments.config").unwrap();
    }

    #[test]
    fn test_invalid_format() {
        fs::write("test_format.config", "name = App\nline_without_equals\nport = 80\n").unwrap();
        let result = Config::load("test_format.config");
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, ConfigError::InvalidFormat { .. })));
        fs::remove_file("test_format.config").unwrap();
    }
}
