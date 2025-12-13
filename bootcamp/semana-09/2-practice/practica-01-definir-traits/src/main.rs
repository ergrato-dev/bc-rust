//! # Práctica 01: Definir Traits
//!
//! Aprende a definir traits con métodos requeridos y default.

// ============================================
// EJERCICIO 1: Trait Describible
// ============================================
//
// Define un trait `Describible` con:
// - Método requerido: descripcion(&self) -> String
// - Método requerido: tipo(&self) -> &str
// - Método default: info(&self) -> String que combine tipo y descripción

trait Describible {
    /// Retorna una descripción del objeto
    fn descripcion(&self) -> String;
    
    /// Retorna el tipo del objeto
    fn tipo(&self) -> &str;
    
    /// Método default que combina tipo y descripción
    fn info(&self) -> String {
        format!("[{}] {}", self.tipo(), self.descripcion())
    }
}

// ============================================
// EJERCICIO 2: Trait Calculable
// ============================================
//
// Define un trait `Calculable` con:
// - area(&self) -> f64
// - perimetro(&self) -> f64
// - Método default: es_grande(&self) -> bool (area > 100)

trait Calculable {
    /// Calcula el área
    fn area(&self) -> f64;
    
    /// Calcula el perímetro
    fn perimetro(&self) -> f64;
    
    /// Verifica si el área es grande (> 100)
    fn es_grande(&self) -> bool {
        self.area() > 100.0
    }
}

// ============================================
// EJERCICIO 3: Trait Saludable
// ============================================
//
// Define un trait `Saludable` con:
// - nombre(&self) -> &str (requerido)
// - saludar(&self) -> String (default: "Hola, soy {nombre}")
// - despedir(&self) -> String (default: "Adiós de {nombre}")

trait Saludable {
    /// Retorna el nombre
    fn nombre(&self) -> &str;
    
    /// Saluda con el nombre
    fn saludar(&self) -> String {
        format!("Hola, soy {}", self.nombre())
    }
    
    /// Se despide con el nombre
    fn despedir(&self) -> String {
        format!("Adiós de {}", self.nombre())
    }
}

// ============================================
// EJERCICIO 4: Trait Validable
// ============================================
//
// Define un trait `Validable` con:
// - es_valido(&self) -> bool (requerido)
// - mensaje_error(&self) -> Option<String> (requerido)
// - validar(&self) -> Result<(), String> (default)

trait Validable {
    /// Verifica si el dato es válido
    fn es_valido(&self) -> bool;
    
    /// Retorna mensaje de error si no es válido
    fn mensaje_error(&self) -> Option<String>;
    
    /// Valida y retorna Result
    fn validar(&self) -> Result<(), String> {
        if self.es_valido() {
            Ok(())
        } else {
            Err(self.mensaje_error().unwrap_or_else(|| "Error desconocido".to_string()))
        }
    }
}

// ============================================
// IMPLEMENTACIONES PARA TESTING
// ============================================

struct Libro {
    titulo: String,
    autor: String,
    paginas: u32,
}

impl Describible for Libro {
    fn descripcion(&self) -> String {
        format!("'{}' por {} ({} páginas)", self.titulo, self.autor, self.paginas)
    }
    
    fn tipo(&self) -> &str {
        "Libro"
    }
}

struct Circulo {
    radio: f64,
}

impl Calculable for Circulo {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radio * self.radio
    }
    
    fn perimetro(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radio
    }
}

struct Rectangulo {
    ancho: f64,
    alto: f64,
}

impl Calculable for Rectangulo {
    fn area(&self) -> f64 {
        self.ancho * self.alto
    }
    
    fn perimetro(&self) -> f64 {
        2.0 * (self.ancho + self.alto)
    }
}

struct Persona {
    nombre: String,
}

impl Saludable for Persona {
    fn nombre(&self) -> &str {
        &self.nombre
    }
}

struct Robot {
    id: String,
}

impl Saludable for Robot {
    fn nombre(&self) -> &str {
        &self.id
    }
    
    // Sobrescribimos el método default
    fn saludar(&self) -> String {
        format!("BEEP BOOP. Unidad {} en línea.", self.nombre())
    }
}

struct Email {
    direccion: String,
}

impl Validable for Email {
    fn es_valido(&self) -> bool {
        self.direccion.contains('@') && self.direccion.contains('.')
    }
    
    fn mensaje_error(&self) -> Option<String> {
        if !self.direccion.contains('@') {
            Some("Falta el símbolo @".to_string())
        } else if !self.direccion.contains('.') {
            Some("Falta el dominio".to_string())
        } else {
            None
        }
    }
}

struct Edad {
    valor: i32,
}

impl Validable for Edad {
    fn es_valido(&self) -> bool {
        self.valor >= 0 && self.valor <= 150
    }
    
    fn mensaje_error(&self) -> Option<String> {
        if self.valor < 0 {
            Some("La edad no puede ser negativa".to_string())
        } else if self.valor > 150 {
            Some("La edad no puede ser mayor a 150".to_string())
        } else {
            None
        }
    }
}

fn main() {
    println!("=== Práctica 01: Definir Traits ===\n");
    
    // Ejercicio 1: Describible
    let libro = Libro {
        titulo: String::from("El Libro de Rust"),
        autor: String::from("Ferris"),
        paginas: 500,
    };
    println!("Describible:");
    println!("  Descripción: {}", libro.descripcion());
    println!("  Tipo: {}", libro.tipo());
    println!("  Info: {}", libro.info());
    
    // Ejercicio 2: Calculable
    println!("\nCalculable:");
    let circulo = Circulo { radio: 5.0 };
    println!("  Círculo (r=5): área={:.2}, perímetro={:.2}", circulo.area(), circulo.perimetro());
    
    let rect = Rectangulo { ancho: 15.0, alto: 8.0 };
    println!("  Rectángulo (15x8): área={:.2}, es_grande={}", rect.area(), rect.es_grande());
    
    // Ejercicio 3: Saludable
    println!("\nSaludable:");
    let persona = Persona { nombre: String::from("Ana") };
    println!("  Persona: {}", persona.saludar());
    
    let robot = Robot { id: String::from("R2D2") };
    println!("  Robot: {}", robot.saludar());
    
    // Ejercicio 4: Validable
    println!("\nValidable:");
    let email_valido = Email { direccion: String::from("test@ejemplo.com") };
    let email_invalido = Email { direccion: String::from("invalido") };
    
    println!("  Email válido: {:?}", email_valido.validar());
    println!("  Email inválido: {:?}", email_invalido.validar());
    
    let edad_valida = Edad { valor: 25 };
    let edad_invalida = Edad { valor: -5 };
    
    println!("  Edad válida: {:?}", edad_valida.validar());
    println!("  Edad inválida: {:?}", edad_invalida.validar());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Tests Ejercicio 1: Describible
    #[test]
    fn test_libro_descripcion() {
        let libro = Libro {
            titulo: String::from("Rust"),
            autor: String::from("Ferris"),
            paginas: 100,
        };
        assert!(libro.descripcion().contains("Rust"));
        assert!(libro.descripcion().contains("Ferris"));
    }
    
    #[test]
    fn test_libro_tipo() {
        let libro = Libro {
            titulo: String::from("Test"),
            autor: String::from("Test"),
            paginas: 50,
        };
        assert_eq!(libro.tipo(), "Libro");
    }
    
    #[test]
    fn test_libro_info_default() {
        let libro = Libro {
            titulo: String::from("Mi Libro"),
            autor: String::from("Autor"),
            paginas: 200,
        };
        let info = libro.info();
        assert!(info.contains("[Libro]"));
        assert!(info.contains("Mi Libro"));
    }
    
    // Tests Ejercicio 2: Calculable
    #[test]
    fn test_circulo_area() {
        let circulo = Circulo { radio: 1.0 };
        let area = circulo.area();
        assert!((area - std::f64::consts::PI).abs() < 0.001);
    }
    
    #[test]
    fn test_circulo_perimetro() {
        let circulo = Circulo { radio: 1.0 };
        let perimetro = circulo.perimetro();
        assert!((perimetro - 2.0 * std::f64::consts::PI).abs() < 0.001);
    }
    
    #[test]
    fn test_rectangulo_area() {
        let rect = Rectangulo { ancho: 10.0, alto: 5.0 };
        assert_eq!(rect.area(), 50.0);
    }
    
    #[test]
    fn test_rectangulo_perimetro() {
        let rect = Rectangulo { ancho: 10.0, alto: 5.0 };
        assert_eq!(rect.perimetro(), 30.0);
    }
    
    #[test]
    fn test_es_grande_true() {
        let rect = Rectangulo { ancho: 20.0, alto: 10.0 };
        assert!(rect.es_grande()); // 200 > 100
    }
    
    #[test]
    fn test_es_grande_false() {
        let rect = Rectangulo { ancho: 5.0, alto: 5.0 };
        assert!(!rect.es_grande()); // 25 < 100
    }
    
    // Tests Ejercicio 3: Saludable
    #[test]
    fn test_persona_nombre() {
        let persona = Persona { nombre: String::from("Carlos") };
        assert_eq!(persona.nombre(), "Carlos");
    }
    
    #[test]
    fn test_persona_saludar_default() {
        let persona = Persona { nombre: String::from("María") };
        assert_eq!(persona.saludar(), "Hola, soy María");
    }
    
    #[test]
    fn test_persona_despedir_default() {
        let persona = Persona { nombre: String::from("Juan") };
        assert_eq!(persona.despedir(), "Adiós de Juan");
    }
    
    #[test]
    fn test_robot_saludar_override() {
        let robot = Robot { id: String::from("X100") };
        let saludo = robot.saludar();
        assert!(saludo.contains("BEEP"));
        assert!(saludo.contains("X100"));
    }
    
    // Tests Ejercicio 4: Validable
    #[test]
    fn test_email_valido() {
        let email = Email { direccion: String::from("user@domain.com") };
        assert!(email.es_valido());
        assert!(email.mensaje_error().is_none());
    }
    
    #[test]
    fn test_email_sin_arroba() {
        let email = Email { direccion: String::from("invalid.com") };
        assert!(!email.es_valido());
        assert!(email.mensaje_error().unwrap().contains("@"));
    }
    
    #[test]
    fn test_email_validar_ok() {
        let email = Email { direccion: String::from("test@test.com") };
        assert!(email.validar().is_ok());
    }
    
    #[test]
    fn test_email_validar_err() {
        let email = Email { direccion: String::from("bad") };
        assert!(email.validar().is_err());
    }
    
    #[test]
    fn test_edad_valida() {
        let edad = Edad { valor: 30 };
        assert!(edad.es_valido());
        assert!(edad.validar().is_ok());
    }
    
    #[test]
    fn test_edad_negativa() {
        let edad = Edad { valor: -10 };
        assert!(!edad.es_valido());
        assert!(edad.mensaje_error().unwrap().contains("negativa"));
    }
    
    #[test]
    fn test_edad_muy_alta() {
        let edad = Edad { valor: 200 };
        assert!(!edad.es_valido());
        assert!(edad.mensaje_error().unwrap().contains("150"));
    }
}
