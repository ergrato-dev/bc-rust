// ============================================
// Práctica 02: Métodos
// ============================================
// Objetivo: Implementar métodos con impl
// ============================================

#[derive(Debug)]
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    // -----------------------------------------
    // PARTE 1: Métodos de Lectura (&self)
    // -----------------------------------------
    
    /// Calcula el área del rectángulo
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }

    /// Calcula el perímetro del rectángulo
    fn perimetro(&self) -> u32 {
        2 * (self.ancho + self.alto)
    }

    /// Verifica si es un cuadrado
    fn es_cuadrado(&self) -> bool {
        self.ancho == self.alto
    }

    /// Muestra información del rectángulo
    fn describir(&self) {
        println!(
            "Rectángulo {}x{}, área: {}, es cuadrado: {}",
            self.ancho,
            self.alto,
            self.area(),
            self.es_cuadrado()
        );
    }

    // -----------------------------------------
    // PARTE 2: Métodos de Modificación (&mut self)
    // -----------------------------------------
    
    /// Escala el rectángulo por un factor
    fn escalar(&mut self, factor: u32) {
        self.ancho *= factor;
        self.alto *= factor;
    }

    // TODO: Implementa rotar() que intercambia ancho y alto
    // fn rotar(&mut self) { ... }

    /// Duplica el tamaño
    fn duplicar(&mut self) {
        self.escalar(2);
    }

    // -----------------------------------------
    // PARTE 3: Métodos con Parámetros
    // -----------------------------------------
    
    /// Verifica si este rectángulo puede contener a otro
    fn puede_contener(&self, otro: &Rectangulo) -> bool {
        self.ancho > otro.ancho && self.alto > otro.alto
    }

    // TODO: Implementa es_mayor_que() que compara áreas
    // fn es_mayor_que(&self, otro: &Rectangulo) -> bool { ... }
}

fn main() {
    println!("=== Práctica 02: Métodos ===\n");

    // -----------------------------------------
    // Demostración de métodos de lectura
    // -----------------------------------------
    println!("--- Métodos de Lectura ---");
    
    let rect = Rectangulo { ancho: 10, alto: 5 };
    
    println!("Área: {}", rect.area());
    println!("Perímetro: {}", rect.perimetro());
    println!("¿Es cuadrado?: {}", rect.es_cuadrado());
    rect.describir();

    let cuadrado = Rectangulo { ancho: 5, alto: 5 };
    println!("\n¿Es cuadrado?: {}", cuadrado.es_cuadrado());

    // -----------------------------------------
    // Demostración de métodos de modificación
    // -----------------------------------------
    println!("\n--- Métodos de Modificación ---");
    
    let mut rect_mut = Rectangulo { ancho: 10, alto: 5 };
    println!("Antes de escalar: {}x{}", rect_mut.ancho, rect_mut.alto);
    
    rect_mut.escalar(2);
    println!("Después de escalar x2: {}x{}", rect_mut.ancho, rect_mut.alto);
    
    rect_mut.duplicar();
    println!("Después de duplicar: {}x{}", rect_mut.ancho, rect_mut.alto);

    // -----------------------------------------
    // Demostración de métodos con parámetros
    // -----------------------------------------
    println!("\n--- Métodos con Parámetros ---");
    
    let grande = Rectangulo { ancho: 20, alto: 15 };
    let pequeno = Rectangulo { ancho: 5, alto: 3 };

    if grande.puede_contener(&pequeno) {
        println!("El grande puede contener al pequeño");
    }

    if !pequeno.puede_contener(&grande) {
        println!("El pequeño NO puede contener al grande");
    }

    println!("\n✅ Práctica completada");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let rect = Rectangulo { ancho: 10, alto: 5 };
        assert_eq!(rect.area(), 50);
    }

    #[test]
    fn test_perimetro() {
        let rect = Rectangulo { ancho: 10, alto: 5 };
        assert_eq!(rect.perimetro(), 30);
    }

    #[test]
    fn test_es_cuadrado() {
        let rect = Rectangulo { ancho: 10, alto: 5 };
        let cuadrado = Rectangulo { ancho: 5, alto: 5 };
        
        assert!(!rect.es_cuadrado());
        assert!(cuadrado.es_cuadrado());
    }

    #[test]
    fn test_escalar() {
        let mut rect = Rectangulo { ancho: 10, alto: 5 };
        rect.escalar(3);
        
        assert_eq!(rect.ancho, 30);
        assert_eq!(rect.alto, 15);
    }

    #[test]
    fn test_puede_contener() {
        let grande = Rectangulo { ancho: 20, alto: 15 };
        let pequeno = Rectangulo { ancho: 5, alto: 3 };

        assert!(grande.puede_contener(&pequeno));
        assert!(!pequeno.puede_contener(&grande));
    }

    #[test]
    fn test_duplicar() {
        let mut rect = Rectangulo { ancho: 5, alto: 3 };
        rect.duplicar();
        
        assert_eq!(rect.ancho, 10);
        assert_eq!(rect.alto, 6);
    }
}
