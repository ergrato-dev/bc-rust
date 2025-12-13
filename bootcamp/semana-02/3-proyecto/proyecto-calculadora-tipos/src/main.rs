// ============================================
// Weekly Project: Type Calculator
// ============================================
// Week 02: Variables and Data Types
// ============================================

use std::mem::size_of;

// ============================================
// CONSTANTS
// ============================================
const PI: f64 = 3.14159265358979;
const DAYS_PER_YEAR: u32 = 365;
const HOURS_PER_DAY: u32 = 24;
const MINUTES_PER_HOUR: u32 = 60;

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║   🧮 TYPE CALCULATOR - RUST           ║");
    println!("║      Week 02 Project                  ║");
    println!("╚════════════════════════════════════════╝\n");

    // Run all demos
    demo_geometry();
    demo_temperature();
    demo_statistics();
    demo_type_explorer();
    demo_age_calculator();

    println!("\n✅ Project completed");
}

// ============================================
// LEVEL 1: GEOMETRY
// ============================================
fn demo_geometry() {
    println!("═══ 📐 GEOMETRY ═══\n");

    // Rectangle
    let width: f64 = 10.0;
    let height: f64 = 5.0;
    
    let rect_area = calculate_rectangle_area(width, height);
    let rect_perimeter = calculate_rectangle_perimeter(width, height);
    
    println!("Rectangle ({}x{}):", width, height);
    println!("  Area: {:.2}", rect_area);
    println!("  Perimeter: {:.2}", rect_perimeter);

    // Circle
    let radius: f64 = 7.0;
    
    let circle_area = calculate_circle_area(radius);
    let circumference = calculate_circumference(radius);
    
    println!("\nCircle (radius {}):", radius);
    println!("  Area: {:.2}", circle_area);
    println!("  Circumference: {:.2}", circumference);
    println!();
}

fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

fn calculate_rectangle_perimeter(width: f64, height: f64) -> f64 {
    2.0 * (width + height)
}

fn calculate_circle_area(radius: f64) -> f64 {
    PI * radius * radius
}

fn calculate_circumference(radius: f64) -> f64 {
    2.0 * PI * radius
}

// ============================================
// LEVEL 1: TEMPERATURE
// ============================================
fn demo_temperature() {
    println!("═══ 🌡️ TEMPERATURE ═══\n");

    let celsius: f64 = 25.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{}°C = {:.1}°F", celsius, fahrenheit);

    let fahrenheit: f64 = 98.6;
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{}°F = {:.1}°C", fahrenheit, celsius);
    println!();
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// ============================================
// LEVEL 1: STATISTICS
// ============================================
fn demo_statistics() {
    println!("═══ 📊 STATISTICS ═══\n");

    // Using mutable variables to accumulate
    let n1: i32 = 10;
    let n2: i32 = 25;
    let n3: i32 = 8;
    let n4: i32 = 42;
    let n5: i32 = 15;

    println!("Numbers: {}, {}, {}, {}, {}", n1, n2, n3, n4, n5);
    
    let sum = n1 + n2 + n3 + n4 + n5;
    let average = sum as f64 / 5.0;
    
    // Find min and max manually
    let mut minimum = n1;
    if n2 < minimum { minimum = n2; }
    if n3 < minimum { minimum = n3; }
    if n4 < minimum { minimum = n4; }
    if n5 < minimum { minimum = n5; }

    let mut maximum = n1;
    if n2 > maximum { maximum = n2; }
    if n3 > maximum { maximum = n3; }
    if n4 > maximum { maximum = n4; }
    if n5 > maximum { maximum = n5; }

    println!("  Sum: {}", sum);
    println!("  Average: {:.2}", average);
    println!("  Minimum: {}", minimum);
    println!("  Maximum: {}", maximum);
    println!();
}

// ============================================
// LEVEL 2: TYPE EXPLORER
// ============================================
fn demo_type_explorer() {
    println!("═══ 🔍 TYPE EXPLORER ═══\n");

    println!("Size in bytes:");
    println!("  bool:  {} byte", size_of::<bool>());
    println!("  char:  {} bytes", size_of::<char>());
    println!("  i8:    {} byte", size_of::<i8>());
    println!("  i16:   {} bytes", size_of::<i16>());
    println!("  i32:   {} bytes", size_of::<i32>());
    println!("  i64:   {} bytes", size_of::<i64>());
    println!("  i128:  {} bytes", size_of::<i128>());
    println!("  f32:   {} bytes", size_of::<f32>());
    println!("  f64:   {} bytes", size_of::<f64>());
    println!("  usize: {} bytes", size_of::<usize>());

    println!("\nSigned integer ranges:");
    println!("  i8:  {} to {}", i8::MIN, i8::MAX);
    println!("  i16: {} to {}", i16::MIN, i16::MAX);
    println!("  i32: {} to {}", i32::MIN, i32::MAX);

    println!("\nUnsigned integer ranges:");
    println!("  u8:  {} to {}", u8::MIN, u8::MAX);
    println!("  u16: {} to {}", u16::MIN, u16::MAX);
    println!("  u32: {} to {}", u32::MIN, u32::MAX);
    println!();
}

// ============================================
// LEVEL 2: AGE CALCULATOR
// ============================================
fn demo_age_calculator() {
    println!("═══ 🎂 AGE CALCULATOR ═══\n");

    let age_years: u32 = 25;
    
    // Shadowing for conversions
    let age_days = age_years * DAYS_PER_YEAR;
    let age_hours = age_days * HOURS_PER_DAY;
    let age_minutes = age_hours * MINUTES_PER_HOUR;

    println!("Age: {} years", age_years);
    println!("  In days: {} days", age_days);
    println!("  In hours: {} hours", age_hours);
    println!("  In minutes: {} minutes", age_minutes);
    
    // Using u64 for larger numbers
    let age_seconds: u64 = age_minutes as u64 * 60;
    println!("  In seconds: {} seconds", age_seconds);
    println!();
}

// ============================================
// TESTS
// ============================================
#[cfg(test)]
mod tests {
    use super::*;

    // Geometry tests
    #[test]
    fn test_rectangle_area() {
        assert!((calculate_rectangle_area(10.0, 5.0) - 50.0).abs() < 0.001);
    }

    #[test]
    fn test_rectangle_perimeter() {
        assert!((calculate_rectangle_perimeter(10.0, 5.0) - 30.0).abs() < 0.001);
    }

    #[test]
    fn test_circle_area() {
        let area = calculate_circle_area(1.0);
        assert!((area - PI).abs() < 0.001);
    }

    #[test]
    fn test_circumference() {
        let circ = calculate_circumference(1.0);
        assert!((circ - 2.0 * PI).abs() < 0.001);
    }

    // Temperature tests
    #[test]
    fn test_celsius_to_fahrenheit() {
        assert!((celsius_to_fahrenheit(0.0) - 32.0).abs() < 0.001);
        assert!((celsius_to_fahrenheit(100.0) - 212.0).abs() < 0.001);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert!((fahrenheit_to_celsius(32.0) - 0.0).abs() < 0.001);
        assert!((fahrenheit_to_celsius(212.0) - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_round_trip_conversion() {
        let original = 25.0;
        let converted = fahrenheit_to_celsius(celsius_to_fahrenheit(original));
        assert!((original - converted).abs() < 0.001);
    }

    // Type tests
    #[test]
    fn test_type_sizes() {
        assert_eq!(size_of::<i8>(), 1);
        assert_eq!(size_of::<i32>(), 4);
        assert_eq!(size_of::<i64>(), 8);
        assert_eq!(size_of::<char>(), 4);
    }

    #[test]
    fn test_ranges() {
        assert_eq!(i8::MIN, -128);
        assert_eq!(i8::MAX, 127);
        assert_eq!(u8::MIN, 0);
        assert_eq!(u8::MAX, 255);
    }
}
