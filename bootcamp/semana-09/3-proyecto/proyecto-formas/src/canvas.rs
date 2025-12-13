//! # Canvas para Dibujar Formas
//!
//! Proporciona un canvas simple para visualizar formas.

use crate::traits::{Forma, Dibujable, FormaComparable};

/// Canvas para organizar y mostrar formas
#[derive(Debug, Clone)]
pub struct Canvas {
    ancho: usize,
    alto: usize,
    titulo: String,
}

impl Canvas {
    /// Crea un nuevo canvas
    pub fn new(ancho: usize, alto: usize) -> Self {
        Self {
            ancho,
            alto,
            titulo: String::from("Canvas"),
        }
    }
    
    /// Crea un canvas con título
    pub fn con_titulo(ancho: usize, alto: usize, titulo: &str) -> Self {
        Self {
            ancho,
            alto,
            titulo: titulo.to_string(),
        }
    }
    
    /// Muestra información de una forma
    pub fn mostrar_info<F: Forma>(&self, forma: &F) {
        println!("╔{}╗", "═".repeat(self.ancho - 2));
        println!("║ {} {}", self.titulo, " ".repeat(self.ancho - 4 - self.titulo.len()));
        println!("╠{}╣", "═".repeat(self.ancho - 2));
        println!("║ Forma: {:<width$}║", forma.nombre(), width = self.ancho - 11);
        println!("║ Área: {:<width$.2}║", forma.area(), width = self.ancho - 10);
        println!("║ Perímetro: {:<width$.2}║", forma.perimetro(), width = self.ancho - 15);
        println!("╚{}╝", "═".repeat(self.ancho - 2));
    }
    
    /// Dibuja una forma en el canvas
    pub fn dibujar<F: Dibujable>(&self, forma: &F) -> String {
        let mut resultado = String::new();
        
        // Borde superior
        resultado.push_str(&format!("┌{}┐\n", "─".repeat(self.ancho - 2)));
        resultado.push_str(&format!("│ {} {}│\n", 
            self.titulo, 
            " ".repeat(self.ancho - 4 - self.titulo.len())));
        resultado.push_str(&format!("├{}┤\n", "─".repeat(self.ancho - 2)));
        
        // Contenido del dibujo
        let dibujo = forma.dibujar();
        for linea in dibujo.lines() {
            let padding = self.ancho.saturating_sub(linea.chars().count() + 4);
            resultado.push_str(&format!("│ {}{} │\n", linea, " ".repeat(padding)));
        }
        
        // Borde inferior
        resultado.push_str(&format!("└{}┘\n", "─".repeat(self.ancho - 2)));
        
        resultado
    }
    
    /// Compara dos formas
    pub fn comparar<F1, F2>(forma1: &F1, forma2: &F2) 
    where
        F1: FormaComparable,
        F2: FormaComparable,
    {
        println!("\n📊 Comparación de Formas:");
        println!("─────────────────────────────────────");
        println!("{:<15} │ {:<15} │ {:<15}", "", forma1.nombre(), forma2.nombre());
        println!("─────────────────────────────────────");
        println!("{:<15} │ {:>15.2} │ {:>15.2}", "Área", forma1.area(), forma2.area());
        println!("{:<15} │ {:>15.2} │ {:>15.2}", "Perímetro", forma1.perimetro(), forma2.perimetro());
        println!("─────────────────────────────────────");
        
        if forma1.es_mas_grande_que(forma2) {
            println!("✓ {} es más grande", forma1.nombre());
        } else if forma2.es_mas_grande_que(forma1) {
            println!("✓ {} es más grande", forma2.nombre());
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
pub fn imprimir_forma<F: Forma>(forma: &F) {
    println!("📐 {}", forma.nombre());
    println!("   Área: {:.2}", forma.area());
    println!("   Perímetro: {:.2}", forma.perimetro());
}

/// Función helper para imprimir múltiples formas
pub fn imprimir_formas(formas: &[&dyn Forma]) {
    println!("\n📋 Lista de Formas:");
    println!("═══════════════════════════════════════");
    
    for (i, forma) in formas.iter().enumerate() {
        println!("{}. {} - Área: {:.2}, Perímetro: {:.2}",
            i + 1,
            forma.nombre(),
            forma.area(),
            forma.perimetro()
        );
    }
    
    println!("═══════════════════════════════════════");
}

/// Encuentra la forma con mayor área
pub fn forma_mayor_area<'a>(formas: &[&'a dyn Forma]) -> Option<&'a dyn Forma> {
    formas.iter()
        .max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap())
        .copied()
}

/// Calcula el área total de todas las formas
pub fn area_total(formas: &[&dyn Forma]) -> f64 {
    formas.iter().map(|f| f.area()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formas::{Circulo, Rectangulo, Cuadrado};
    
    #[test]
    fn test_canvas_new() {
        let canvas = Canvas::new(50, 30);
        assert_eq!(canvas.ancho, 50);
        assert_eq!(canvas.alto, 30);
    }
    
    #[test]
    fn test_canvas_con_titulo() {
        let canvas = Canvas::con_titulo(50, 30, "Mi Canvas");
        assert_eq!(canvas.titulo, "Mi Canvas");
    }
    
    #[test]
    fn test_canvas_dibujar() {
        let canvas = Canvas::new(30, 10);
        let cuadrado = Cuadrado::new(3.0);
        let resultado = canvas.dibujar(&cuadrado);
        assert!(resultado.contains("Canvas"));
    }
    
    #[test]
    fn test_forma_mayor_area() {
        let c = Circulo::new(1.0);
        let r = Rectangulo::new(10.0, 10.0);
        let s = Cuadrado::new(5.0);
        
        let formas: Vec<&dyn Forma> = vec![&c, &r, &s];
        let mayor = forma_mayor_area(&formas).unwrap();
        
        assert_eq!(mayor.nombre(), "Rectángulo");
    }
    
    #[test]
    fn test_area_total() {
        let c = Cuadrado::new(2.0); // área = 4
        let r = Rectangulo::new(2.0, 3.0); // área = 6
        
        let formas: Vec<&dyn Forma> = vec![&c, &r];
        let total = area_total(&formas);
        
        assert!((total - 10.0).abs() < f64::EPSILON);
    }
}
