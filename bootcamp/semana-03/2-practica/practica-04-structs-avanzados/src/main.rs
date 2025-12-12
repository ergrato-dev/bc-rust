// ============================================
// Práctica 04: Structs Avanzados
// ============================================
// Objetivo: Tuple, Unit y Structs Anidados
// ============================================

// -----------------------------------------
// PARTE 1: Tuple Structs
// -----------------------------------------

/// Color RGB (0-255 cada componente)
struct Color(u8, u8, u8);

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    fn rojo() -> Self {
        Self(255, 0, 0)
    }

    fn verde() -> Self {
        Self(0, 255, 0)
    }

    fn azul() -> Self {
        Self(0, 0, 255)
    }

    fn blanco() -> Self {
        Self(255, 255, 255)
    }

    fn negro() -> Self {
        Self(0, 0, 0)
    }

    fn mostrar(&self) {
        println!("RGB({}, {}, {})", self.0, self.1, self.2);
    }

    fn es_gris(&self) -> bool {
        self.0 == self.1 && self.1 == self.2
    }
}

/// Punto en 2D
struct Punto(f64, f64);

impl Punto {
    fn new(x: f64, y: f64) -> Self {
        Self(x, y)
    }

    fn origen() -> Self {
        Self(0.0, 0.0)
    }

    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn distancia_al_origen(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }

    fn distancia_a(&self, otro: &Punto) -> f64 {
        let dx = self.0 - otro.0;
        let dy = self.1 - otro.1;
        (dx * dx + dy * dy).sqrt()
    }
}

// -----------------------------------------
// PARTE 2: Newtype Pattern
// -----------------------------------------

/// ID de usuario (no se puede confundir con ProductId)
struct UserId(u64);

/// ID de producto (tipo distinto a UserId)
struct ProductId(u64);

impl UserId {
    fn new(id: u64) -> Self {
        Self(id)
    }

    fn valor(&self) -> u64 {
        self.0
    }
}

impl ProductId {
    fn new(id: u64) -> Self {
        Self(id)
    }

    fn valor(&self) -> u64 {
        self.0
    }
}

// Funciones que solo aceptan el tipo correcto
fn obtener_usuario(id: UserId) {
    println!("Buscando usuario con ID: {}", id.valor());
}

fn obtener_producto(id: ProductId) {
    println!("Buscando producto con ID: {}", id.valor());
}

// -----------------------------------------
// PARTE 3: Structs Anidados
// -----------------------------------------

struct Direccion {
    calle: String,
    ciudad: String,
    codigo_postal: String,
}

impl Direccion {
    fn new(calle: String, ciudad: String, codigo_postal: String) -> Self {
        Self { calle, ciudad, codigo_postal }
    }

    fn mostrar(&self) {
        println!("{}, {} ({})", self.calle, self.ciudad, self.codigo_postal);
    }
}

struct Persona {
    nombre: String,
    edad: u32,
    direccion: Direccion,
}

impl Persona {
    fn new(nombre: String, edad: u32, direccion: Direccion) -> Self {
        Self { nombre, edad, direccion }
    }

    fn mostrar(&self) {
        println!("Nombre: {}", self.nombre);
        println!("Edad: {}", self.edad);
        print!("Dirección: ");
        self.direccion.mostrar();
    }

    fn ciudad(&self) -> &str {
        &self.direccion.ciudad
    }
}

// -----------------------------------------
// PARTE 4: Unit Struct (Marcador)
// -----------------------------------------

struct Validado;
struct NoValidado;

// Podría usarse para type-state pattern (avanzado)

fn main() {
    println!("=== Práctica 04: Structs Avanzados ===\n");

    // -----------------------------------------
    // Tuple Structs
    // -----------------------------------------
    println!("--- Tuple Structs: Color ---");
    
    let rojo = Color::rojo();
    let custom = Color::new(128, 64, 255);
    let gris = Color::new(100, 100, 100);
    
    print!("Rojo: "); rojo.mostrar();
    print!("Custom: "); custom.mostrar();
    print!("Gris: "); gris.mostrar();
    println!("¿Es gris?: {}", gris.es_gris());

    println!("\n--- Tuple Structs: Punto ---");
    
    let p1 = Punto::new(3.0, 4.0);
    let p2 = Punto::new(6.0, 8.0);
    let origen = Punto::origen();
    
    println!("P1: ({}, {})", p1.x(), p1.y());
    println!("Distancia de P1 al origen: {:.2}", p1.distancia_al_origen());
    println!("Distancia de P1 a P2: {:.2}", p1.distancia_a(&p2));

    // -----------------------------------------
    // Newtype Pattern
    // -----------------------------------------
    println!("\n--- Newtype Pattern ---");
    
    let user_id = UserId::new(12345);
    let product_id = ProductId::new(67890);
    
    obtener_usuario(user_id);
    obtener_producto(product_id);
    
    // Esto NO compilaría (tipos diferentes):
    // obtener_usuario(product_id);  // Error!

    // -----------------------------------------
    // Structs Anidados
    // -----------------------------------------
    println!("\n--- Structs Anidados ---");
    
    let direccion = Direccion::new(
        String::from("Calle Mayor 123"),
        String::from("Madrid"),
        String::from("28001"),
    );

    let persona = Persona::new(
        String::from("María García"),
        28,
        direccion,
    );

    persona.mostrar();
    println!("Vive en: {}", persona.ciudad());

    println!("\n✅ Práctica completada");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        let c = Color::rojo();
        assert_eq!(c.0, 255);
        assert_eq!(c.1, 0);
        assert_eq!(c.2, 0);
    }

    #[test]
    fn test_color_es_gris() {
        let gris = Color::new(50, 50, 50);
        let no_gris = Color::new(50, 60, 70);
        
        assert!(gris.es_gris());
        assert!(!no_gris.es_gris());
    }

    #[test]
    fn test_punto_origen() {
        let p = Punto::origen();
        assert_eq!(p.x(), 0.0);
        assert_eq!(p.y(), 0.0);
    }

    #[test]
    fn test_punto_distancia() {
        let p = Punto::new(3.0, 4.0);
        assert!((p.distancia_al_origen() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_user_id() {
        let id = UserId::new(100);
        assert_eq!(id.valor(), 100);
    }

    #[test]
    fn test_persona_ciudad() {
        let dir = Direccion::new(
            String::from("Calle"),
            String::from("Barcelona"),
            String::from("08001"),
        );
        let persona = Persona::new(String::from("Test"), 25, dir);
        
        assert_eq!(persona.ciudad(), "Barcelona");
    }
}
