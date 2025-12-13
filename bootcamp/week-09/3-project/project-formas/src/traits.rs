//! # Traits para Formas Geométricas
//!
//! Define los traits que las formas deben implementar.

/// Trait principal para formas geométricas
pub trait Shape {
    /// Calcula el área de la forma
    fn area(&self) -> f64;
    
    /// Calcula el perímetro de la forma
    fn perimeter(&self) -> f64;
    
    /// Retorna el nombre de la forma
    fn name(&self) -> &str;
    
    /// Verifica si el área es mayor que un valor
    fn area_greater_than(&self, value: f64) -> bool {
        self.area() > value
    }
}

/// Trait para comparar formas (separado para mantener Shape dyn-compatible)
pub trait ComparableShape: Shape {
    /// Compara áreas con otra forma
    fn is_larger_than<F: Shape>(&self, other: &F) -> bool {
        self.area() > other.area()
    }
}

/// Trait para formas que se pueden dibujar
pub trait Drawable {
    /// Dibuja la forma en ASCII art
    fn draw(&self) -> String;
    
    /// Dibuja con un carácter específico
    fn draw_with(&self, character: char) -> String;
}

/// Trait para transformaciones geométricas
pub trait Transformable {
    /// Escala la forma por un factor
    fn scale(&mut self, factor: f64);
    
    /// Crea una copia escalada
    fn scaled(&self, factor: f64) -> Self where Self: Sized + Clone {
        let mut copy = self.clone();
        copy.scale(factor);
        copy
    }
}

/// Trait para formas con posición
pub trait Positionable {
    /// Obtiene la posición X
    fn x(&self) -> f64;
    
    /// Obtiene la posición Y
    fn y(&self) -> f64;
    
    /// Mueve la forma
    fn move_by(&mut self, dx: f64, dy: f64);
    
    /// Obtiene la posición como tupla
    fn position(&self) -> (f64, f64) {
        (self.x(), self.y())
    }
}

/// Trait combinado para formas completas
pub trait CompleteShape: Shape + Drawable + Transformable + Clone {}

// Implementación blanket: cualquier tipo que implemente todos los traits
// automáticamente implementa CompleteShape
impl<T> CompleteShape for T 
where 
    T: Shape + Drawable + Transformable + Clone 
{}
