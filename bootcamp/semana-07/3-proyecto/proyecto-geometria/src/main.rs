// src/main.rs
// Demonstration of the geometry library

use proyecto_geometria::calculations::{area, perimeter};
use proyecto_geometria::format;
use proyecto_geometria::{Circle, Rectangle, Shape, Triangle};

fn main() {
    println!("╔═══════════════════════════════════════════╗");
    println!("║   Modular Geometry Library                ║");
    println!("║   Week 07 - Modules and Crates            ║");
    println!("╚═══════════════════════════════════════════╝\n");

    // Create shapes
    let circle = Circle::new(5.0);
    let rectangle = Rectangle::new(4.0, 3.0);
    let square = Rectangle::square(5.0);
    let triangle = Triangle::new(3.0, 4.0, 5.0);
    let equilateral = Triangle::equilateral(6.0);

    // Show information for each shape
    println!("=== Created Shapes ===\n");

    show_info(&circle);
    show_info(&rectangle);
    show_info(&square);
    show_info(&triangle);
    show_info(&equilateral);

    // Calculate areas and perimeters
    println!("\n=== Calculations ===\n");

    let data = vec![
        (
            circle.name(),
            area::circle_area(&circle),
            perimeter::circle_perimeter(&circle),
        ),
        (
            rectangle.name(),
            area::rectangle_area(&rectangle),
            perimeter::rectangle_perimeter(&rectangle),
        ),
        (
            square.name(),
            area::rectangle_area(&square),
            perimeter::rectangle_perimeter(&square),
        ),
        (
            triangle.name(),
            area::triangle_area(&triangle),
            perimeter::triangle_perimeter(&triangle),
        ),
        (
            equilateral.name(),
            area::triangle_area(&equilateral),
            perimeter::triangle_perimeter(&equilateral),
        ),
    ];

    // Show formatted table
    println!("{}", format::format_table(&data));

    // Show with units
    println!("\n=== Results with Units ===\n");
    let circle_area = area::circle_area(&circle);
    let circle_perimeter = perimeter::circle_perimeter(&circle);

    println!(
        "Circle (radius = 5 cm):",
    );
    println!("  Area: {}", format::format_area(circle_area, "cm"));
    println!(
        "  Circumference: {}",
        format::format_with_unit(circle_perimeter, "cm")
    );

    // Shape validation
    println!("\n=== Shape Validation ===\n");

    let invalid_triangle = Triangle::new(1.0, 2.0, 10.0);
    let invalid_circle = Circle::new(-5.0);

    validate_and_show(&circle);
    validate_and_show(&rectangle);
    validate_and_show(&triangle);
    validate_and_show(&invalid_triangle);
    validate_and_show(&invalid_circle);

    println!("\n=== End of demonstration ===");
}

fn show_info<T: Shape + std::fmt::Debug>(shape: &T) {
    println!("{}: {:?}", shape.name(), shape);
}

fn validate_and_show<T: Shape>(shape: &T) {
    let status = if shape.is_valid() {
        "✓ Valid"
    } else {
        "✗ Invalid"
    };
    println!("{}: {}", shape.name(), status);
}
