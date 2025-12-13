// src/format.rs
// Formatting functions for geometric results

use crate::shapes::Shape;

/// Formats the result for an individual shape
pub fn format_result(name: &str, area: f64, perimeter: f64) -> String {
    format!(
        "{}: Area = {:.4}, Perimeter = {:.4}",
        name, area, perimeter
    )
}

/// Formats the result using the Shape trait
pub fn format_shape<T: Shape>(shape: &T, area: f64, perimeter: f64) -> String {
    format_result(shape.name(), area, perimeter)
}

/// Generates a formatted table with multiple shapes
pub fn format_table(shapes: &[(&str, f64, f64)]) -> String {
    let mut result = String::new();

    // Header
    result.push_str("┌──────────────────────────┬──────────────┬──────────────┐\n");
    result.push_str("│ Shape                    │ Area         │ Perimeter    │\n");
    result.push_str("├──────────────────────────┼──────────────┼──────────────┤\n");

    // Rows
    for (name, area, perimeter) in shapes {
        result.push_str(&format!(
            "│ {:<24} │ {:>12.4} │ {:>12.4} │\n",
            name, area, perimeter
        ));
    }

    // Footer
    result.push_str("└──────────────────────────┴──────────────┴──────────────┘");

    result
}

/// Formats a value with units
pub fn format_with_unit(value: f64, unit: &str) -> String {
    format!("{:.4} {}", value, unit)
}

/// Formats the area with squared units
pub fn format_area(value: f64, unit: &str) -> String {
    format!("{:.4} {}²", value, unit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_result() {
        let result = format_result("Circle", 78.5398, 31.4159);
        assert!(result.contains("Circle"));
        assert!(result.contains("78.5398"));
    }

    #[test]
    fn test_format_table() {
        let shapes = vec![
            ("Circle", 78.5398, 31.4159),
            ("Rectangle", 12.0, 14.0),
        ];
        let table = format_table(&shapes);
        assert!(table.contains("Circle"));
        assert!(table.contains("Rectangle"));
        assert!(table.contains("Shape"));
    }

    #[test]
    fn test_format_with_unit() {
        let result = format_with_unit(31.4159, "cm");
        assert_eq!(result, "31.4159 cm");
    }

    #[test]
    fn test_format_area() {
        let result = format_area(78.5398, "cm");
        assert_eq!(result, "78.5398 cm²");
    }
}
