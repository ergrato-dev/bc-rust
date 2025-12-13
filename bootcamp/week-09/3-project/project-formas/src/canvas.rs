//! # Canvas para Dibujar Formas
//!
//! Proporciona un canvas simple para visualizar formas.

use crate::traits::{Shape, Drawable, ComparableShape};

/// Canvas para organizar y mostrar formas
#[derive(Debug, Clone)]
pub struct Canvas {
    width: usize,
    height: usize,
    title: String,
}

impl Canvas {
    /// Crea un nuevo canvas
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            title: String::from("Canvas"),
        }
    }
    
    /// Crea un canvas con título
    pub fn with_title(width: usize, height: usize, title: &str) -> Self {
        Self {
            width,
            height,
            title: title.to_string(),
        }
    }
    
    /// Muestra información de una forma
    pub fn show_info<F: Shape>(&self, shape: &F) {
        println!("╔{}╗", "═".repeat(self.width - 2));
        println!("║ {} {}", self.title, " ".repeat(self.width - 4 - self.title.len()));
        println!("╠{}╣", "═".repeat(self.width - 2));
        println!("║ Forma: {:<width$}║", shape.name(), width = self.width - 11);
        println!("║ Área: {:<width$.2}║", shape.area(), width = self.width - 10);
        println!("║ Perímetro: {:<width$.2}║", shape.perimeter(), width = self.width - 15);
        println!("╚{}╝", "═".repeat(self.width - 2));
    }
    
    /// Dibuja una forma en el canvas
    pub fn draw<F: Drawable>(&self, shape: &F) -> String {
        let mut result = String::new();
        
        // Borde superior
        result.push_str(&format!("┌{}┐\n", "─".repeat(self.width - 2)));
        result.push_str(&format!("│ {} {}│\n", 
            self.title, 
            " ".repeat(self.width - 4 - self.title.len())));
        result.push_str(&format!("├{}┤\n", "─".repeat(self.width - 2)));
        
        // Contenido del dibujo
        let drawing = shape.draw();
        for line in drawing.lines() {
            let padding = self.width.saturating_sub(line.chars().count() + 4);
            result.push_str(&format!("│ {}{} │\n", line, " ".repeat(padding)));
        }
        
        // Borde inferior
        result.push_str(&format!("└{}┘\n", "─".repeat(self.width - 2)));
        
        result
    }
    
    /// Compara dos formas
    pub fn compare<F1, F2>(shape1: &F1, shape2: &F2) 
    where
        F1: ComparableShape,
        F2: ComparableShape,
    {
        println!("\n📊 Comparación de Formas:");
        println!("─────────────────────────────────────");
        println!("{:<15} │ {:<15} │ {:<15}", "", shape1.name(), shape2.name());
        println!("─────────────────────────────────────");
        println!("{:<15} │ {:>15.2} │ {:>15.2}", "Área", shape1.area(), shape2.area());
        println!("{:<15} │ {:>15.2} │ {:>15.2}", "Perímetro", shape1.perimeter(), shape2.perimeter());
        println!("─────────────────────────────────────");
        
        if shape1.is_larger_than(shape2) {
            println!("✓ {} es más grande", shape1.name());
        } else if shape2.is_larger_than(shape1) {
            println!("✓ {} es más grande", shape2.name());
        } else {
            println!("✓ Tienen la misma área");
        }
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new(40, 20)
    }
}

/// Función helper para imprimir cualquier forma
pub fn print_shape<F: Shape>(shape: &F) {
    println!("📐 {}", shape.name());
    println!("   Área: {:.2}", shape.area());
    println!("   Perímetro: {:.2}", shape.perimeter());
}

/// Función helper para imprimir múltiples formas
pub fn print_shapes(shapes: &[&dyn Shape]) {
    println!("\n📋 Lista de Formas:");
    println!("═══════════════════════════════════════");
    
    for (i, shape) in shapes.iter().enumerate() {
        println!("{}. {} - Área: {:.2}, Perímetro: {:.2}",
            i + 1,
            shape.name(),
            shape.area(),
            shape.perimeter()
        );
    }
    
    println!("═══════════════════════════════════════");
}

/// Encuentra la forma con mayor área
pub fn shape_with_largest_area<'a>(shapes: &[&'a dyn Shape]) -> Option<&'a dyn Shape> {
    shapes.iter()
        .max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap())
        .copied()
}

/// Calcula el área total de todas las formas
pub fn total_area(shapes: &[&dyn Shape]) -> f64 {
    shapes.iter().map(|f| f.area()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::{Circle, Rectangle, Square};
    
    #[test]
    fn test_canvas_new() {
        let canvas = Canvas::new(50, 30);
        assert_eq!(canvas.width, 50);
        assert_eq!(canvas.height, 30);
    }
    
    #[test]
    fn test_canvas_with_title() {
        let canvas = Canvas::with_title(50, 30, "Mi Canvas");
        assert_eq!(canvas.title, "Mi Canvas");
    }
    
    #[test]
    fn test_canvas_draw() {
        let canvas = Canvas::new(30, 10);
        let square = Square::new(3.0);
        let result = canvas.draw(&square);
        assert!(result.contains("Canvas"));
    }
    
    #[test]
    fn test_shape_with_largest_area() {
        let c = Circle::new(1.0);
        let r = Rectangle::new(10.0, 10.0);
        let s = Square::new(5.0);
        
        let shapes: Vec<&dyn Shape> = vec![&c, &r, &s];
        let largest = shape_with_largest_area(&shapes).unwrap();
        
        assert_eq!(largest.name(), "Rectangle");
    }
    
    #[test]
    fn test_total_area() {
        let c = Square::new(2.0); // área = 4
        let r = Rectangle::new(2.0, 3.0); // área = 6
        
        let shapes: Vec<&dyn Shape> = vec![&c, &r];
        let total = total_area(&shapes);
        
        assert!((total - 10.0).abs() < f64::EPSILON);
    }
}
