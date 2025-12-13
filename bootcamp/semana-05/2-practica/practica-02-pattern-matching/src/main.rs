// Practice 02: Pattern Matching
// Week 05 - Enums and Pattern Matching

use std::f64::consts::PI;

// ============================================
// Exercise 1: Basic Match with Currencies
// ============================================

#[derive(Debug, Clone, Copy)]
enum Currency {
    Peso,
    Dollar,
    Euro,
    Pound,
}

// TODO: Implement conversion to pesos
// Rates: Peso = 1, Dollar = 850, Euro = 920, Pound = 1080
fn to_pesos(currency: Currency, amount: f64) -> f64 {
    todo!("Implement with match")
}

// ============================================
// Exercise 2: Enums with Data - Shapes
// ============================================

#[derive(Debug)]
enum Shape {
    Circle(f64),                          // radius
    Rectangle { width: f64, height: f64 },
    Triangle(f64, f64),                   // base, height
}

// TODO: Calculate area of any shape
fn calculate_area(shape: &Shape) -> f64 {
    todo!("Implement by extracting data with match")
}

// TODO: Returns the shape name
fn shape_name(shape: &Shape) -> &str {
    todo!("Implement with match")
}

// ============================================
// Exercise 3: Advanced Patterns
// ============================================

// TODO: Use guards and ranges to classify
fn classify_number(n: i32) -> &'static str {
    todo!("Implement with match, guards, and ranges")
}

// ============================================
// Exercise 4: Tuple Destructuring
// ============================================

// TODO: Classify a point (x, y)
fn classify_point(point: (i32, i32)) -> &'static str {
    // Returns:
    // - "origin" if (0, 0)
    // - "x axis" if y == 0
    // - "y axis" if x == 0
    // - "quadrant I" if x > 0 && y > 0
    // - "quadrant II" if x < 0 && y > 0
    // - "quadrant III" if x < 0 && y < 0
    // - "quadrant IV" if x > 0 && y < 0
    todo!("Implement with match and destructuring")
}

fn main() {
    // Test currencies
    println!("100 dollars = {} pesos", to_pesos(Currency::Dollar, 100.0));
    
    // Test shapes
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle { width: 10.0, height: 5.0 };
    
    println!("{}: area = {:.2}", shape_name(&circle), calculate_area(&circle));
    println!("{}: area = {:.2}", shape_name(&rectangle), calculate_area(&rectangle));
    
    // Test classification
    println!("50 is: {}", classify_number(50));
    println!("-5 is: {}", classify_number(-5));
    
    // Test points
    println!("(3, 4) is in: {}", classify_point((3, 4)));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Currency Tests
    #[test]
    fn test_peso_to_pesos() {
        assert_eq!(to_pesos(Currency::Peso, 100.0), 100.0);
    }

    #[test]
    fn test_dollar_to_pesos() {
        assert_eq!(to_pesos(Currency::Dollar, 1.0), 850.0);
    }

    // Shape Tests
    #[test]
    fn test_circle_area() {
        let c = Shape::Circle(1.0);
        assert!((calculate_area(&c) - PI).abs() < 0.001);
    }

    #[test]
    fn test_rectangle_area() {
        let r = Shape::Rectangle { width: 4.0, height: 5.0 };
        assert_eq!(calculate_area(&r), 20.0);
    }

    #[test]
    fn test_triangle_area() {
        let t = Shape::Triangle(6.0, 4.0);
        assert_eq!(calculate_area(&t), 12.0);
    }

    // Classification Tests
    #[test]
    fn test_classify_zero() {
        assert_eq!(classify_number(0), "zero");
    }

    #[test]
    fn test_classify_small() {
        assert_eq!(classify_number(5), "small");
    }

    #[test]
    fn test_classify_medium() {
        assert_eq!(classify_number(50), "medium");
    }

    #[test]
    fn test_classify_large() {
        assert_eq!(classify_number(200), "large");
    }

    #[test]
    fn test_classify_negative() {
        assert_eq!(classify_number(-10), "negative");
    }

    // Point Tests
    #[test]
    fn test_point_origin() {
        assert_eq!(classify_point((0, 0)), "origin");
    }

    #[test]
    fn test_point_quadrant_i() {
        assert_eq!(classify_point((3, 4)), "quadrant I");
    }

    #[test]
    fn test_point_x_axis() {
        assert_eq!(classify_point((5, 0)), "x axis");
    }
}
