//! # Traits para Formas Geométricas
//!
//! Define los traits que las formas deben implementar.

/// Trait principal para formas geométricas
pub trait Forma {
    /// Calcula el área de la forma
    fn area(&self) -> f64;
    
    /// Calcula el perímetro de la forma
    fn perimetro(&self) -> f64;
    
    /// Retorna el nombre de la forma
    fn nombre(&self) -> &str;
    
    /// Verifica si el área es mayor que un valor
    fn area_mayor_que(&self, valor: f64) -> bool {
        self.area() > valor
    }
}

/// Trait para comparar formas (separado para mantener Forma dyn-compatible)
pub trait FormaComparable: Forma {
    /// Compara áreas con otra forma
    fn es_mas_grande_que<F: Forma>(&self, otra: &F) -> bool {
        self.area() > otra.area()
    }
}

/// Trait para formas que se pueden dibujar
pub trait Dibujable {
    /// Dibuja la forma en ASCII art
    fn dibujar(&self) -> String;
    
    /// Dibuja con un carácter específico
    fn dibujar_con(&self, caracter: char) -> String;
}

/// Trait para transformaciones geométricas
pub trait Transformable {
    /// Escala la forma por un factor
    fn escalar(&mut self, factor: f64);
    
    /// Crea una copia escalada
    fn escalada(&self, factor: f64) -> Self where Self: Sized + Clone {
        let mut copia = self.clone();
        copia.escalar(factor);
        copia
    }
}

/// Trait para formas con posición
pub trait Posicionable {
    /// Obtiene la posición X
    fn x(&self) -> f64;
    
    /// Obtiene la posición Y
    fn y(&self) -> f64;
    
    /// Mueve la forma
    fn mover(&mut self, dx: f64, dy: f64);
    
    /// Obtiene la posición como tupla
    fn posicion(&self) -> (f64, f64) {
        (self.x(), self.y())
    }
}

/// Trait combinado para formas completas
pub trait FormaCompleta: Forma + Dibujable + Transformable + Clone {}

// Implementación blanket: cualquier tipo que implemente todos los traits
// automáticamente implementa FormaCompleta
impl<T> FormaCompleta for T 
where 
    T: Forma + Dibujable + Transformable + Clone 
{}
