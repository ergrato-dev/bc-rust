//! # Demo: Sistema de Formas Geométricas
//!
//! Demostración del uso de traits con formas geométricas.

use project_formas::*;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║        🎨 Sistema de Formas Geométricas con Traits         ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    // =========================================================================
    // 1. Creación de Formas
    // =========================================================================
    println!("📦 1. CREACIÓN DE FORMAS");
    println!("─────────────────────────────────────────────────────────────\n");
    
    let circle = Circle::new(5.0);
    let rectangle = Rectangle::new(10.0, 6.0);
    let square = Square::new(4.0);
    let triangle = Triangle::right_triangle(3.0, 4.0);
    
    println!("Formas creadas:");
    println!("  • {}", circle);
    println!("  • {}", rectangle);
    println!("  • {}", square);
    println!("  • {}", triangle);
    
    // =========================================================================
    // 2. Trait Shape
    // =========================================================================
    println!("\n📐 2. TRAIT SHAPE - Área y Perímetro");
    println!("─────────────────────────────────────────────────────────────\n");
    
    print_shape(&circle);
    println!();
    print_shape(&rectangle);
    println!();
    print_shape(&square);
    println!();
    print_shape(&triangle);
    
    // =========================================================================
    // 3. Trait Drawable
    // =========================================================================
    println!("\n🎨 3. TRAIT DRAWABLE - ASCII Art");
    println!("─────────────────────────────────────────────────────────────\n");
    
    let canvas = Canvas::with_title(25, 15, "Cuadrado");
    println!("{}", canvas.draw(&square));
    
    let canvas2 = Canvas::with_title(30, 15, "Triángulo");
    println!("{}", canvas2.draw(&triangle));
    
    // =========================================================================
    // 4. Trait Transformable
    // =========================================================================
    println!("🔄 4. TRAIT TRANSFORMABLE - Escalado");
    println!("─────────────────────────────────────────────────────────────\n");
    
    let mut scalable_circle = Circle::new(3.0);
    println!("Círculo original: radio = {}", scalable_circle.radius);
    println!("  Área: {:.2}", scalable_circle.area());
    
    scalable_circle.scale(2.0);
    println!("\nDespués de escalar x2: radio = {}", scalable_circle.radius);
    println!("  Área: {:.2}", scalable_circle.area());
    
    // Usando scaled() que no modifica el original
    let original = Square::new(5.0);
    let scaled = original.scaled(3.0);
    println!("\nCuadrado original: lado = {}", original.side);
    println!("Cuadrado escalado: lado = {}", scaled.side);
    
    // =========================================================================
    // 5. Trait Positionable
    // =========================================================================
    println!("\n📍 5. TRAIT POSITIONABLE - Movimiento");
    println!("─────────────────────────────────────────────────────────────\n");
    
    let mut movable_shape = Circle::with_position(2.0, 0.0, 0.0);
    println!("Posición inicial: {:?}", movable_shape.position());
    
    movable_shape.move_by(10.0, 5.0);
    println!("Después de move_by(10, 5): {:?}", movable_shape.position());
    
    movable_shape.move_by(-3.0, 2.0);
    println!("Después de move_by(-3, 2): {:?}", movable_shape.position());
    
    // =========================================================================
    // 6. Polimorfismo con Trait Objects
    // =========================================================================
    println!("\n🔀 6. POLIMORFISMO - Trait Objects");
    println!("─────────────────────────────────────────────────────────────\n");
    
    let shapes: Vec<&dyn Shape> = vec![
        &circle,
        &rectangle,
        &square,
        &triangle,
    ];
    
    print_shapes(&shapes);
    
    if let Some(largest) = shape_with_largest_area(&shapes) {
        println!("\n🏆 Forma con mayor área: {} ({:.2})", 
            largest.name(), 
            largest.area());
    }
    
    println!("📊 Área total: {:.2}", total_area(&shapes));
    
    // =========================================================================
    // 7. Comparación de Formas
    // =========================================================================
    println!("\n⚖️ 7. COMPARACIÓN DE FORMAS");
    println!("─────────────────────────────────────────────────────────────");
    
    Canvas::compare(&circle, &rectangle);
    
    // =========================================================================
    // 8. Conversiones (From/Into)
    // =========================================================================
    println!("\n🔄 8. CONVERSIONES - From/Into");
    println!("─────────────────────────────────────────────────────────────\n");
    
    // Square -> Rectangle
    let original_square = Square::new(7.0);
    let rectangle_from_square: Rectangle = original_square.clone().into();
    println!("Cuadrado(lado=7) -> {}", rectangle_from_square);
    
    // f64 -> Circle
    let circle_from_number: Circle = 4.0.into();
    println!("4.0 -> {}", circle_from_number);
    
    // Tupla -> Rectangle
    let rectangle_from_tuple: Rectangle = (8.0, 3.0).into();
    println!("(8.0, 3.0) -> {}", rectangle_from_tuple);
    
    // =========================================================================
    // 9. Tipos de Triángulos
    // =========================================================================
    println!("\n📐 9. TIPOS DE TRIÁNGULOS");
    println!("─────────────────────────────────────────────────────────────\n");
    
    let equilateral = Triangle::equilateral(5.0);
    let isosceles = Triangle::isosceles(4.0, 5.0).unwrap();
    let right_tri = Triangle::right_triangle(3.0, 4.0);
    
    println!("Equilátero: {} - is_equilateral: {}", equilateral, equilateral.is_equilateral());
    println!("Isósceles: {} - is_isosceles: {}", isosceles, isosceles.is_isosceles());
    println!("Rectángulo: {} - is_right_triangle: {}", right_tri, right_tri.is_right_triangle());
    
    // =========================================================================
    // 10. Funciones Genéricas con Trait Bounds
    // =========================================================================
    println!("\n🔧 10. FUNCIONES GENÉRICAS CON TRAIT BOUNDS");
    println!("─────────────────────────────────────────────────────────────\n");
    
    // Función que acepta cualquier Shape
    fn describe<F: Shape>(shape: &F) {
        println!("→ {} con área {:.2}", shape.name(), shape.area());
    }
    
    // Función con múltiples bounds
    fn full_info<F>(shape: &F) 
    where 
        F: Shape + std::fmt::Display 
    {
        println!("→ Display: {}", shape);
        println!("  Área: {:.2}, Perímetro: {:.2}", shape.area(), shape.perimeter());
    }
    
    describe(&circle);
    describe(&triangle);
    
    println!();
    full_info(&square);
    
    // =========================================================================
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║              ✅ Demostración Completada                     ║");
    println!("╚════════════════════════════════════════════════════════════╝");
}
