//! # Práctica 02: Implementar Traits
//!
//! Aprende a implementar traits para diferentes tipos.

// ============================================
// EJERCICIO 1: Trait Animal
// ============================================
//
// Implementa el trait Animal para Perro, Gato y Pajaro
// Cada uno debe tener su propio sonido y forma de moverse

trait Animal {
    fn nombre(&self) -> &str;
    fn sonido(&self) -> &str;
    fn moverse(&self) -> String;
    
    fn presentarse(&self) -> String {
        format!("Soy {}, hago '{}' y me muevo {}", 
            self.nombre(), 
            self.sonido(),
            self.moverse()
        )
    }
}

struct Perro {
    nombre: String,
    raza: String,
}

impl Animal for Perro {
    fn nombre(&self) -> &str {
        &self.nombre
    }
    
    fn sonido(&self) -> &str {
        "Guau guau"
    }
    
    fn moverse(&self) -> String {
        format!("corriendo con mis 4 patas de {}", self.raza)
    }
}

struct Gato {
    nombre: String,
    color: String,
}

impl Animal for Gato {
    fn nombre(&self) -> &str {
        &self.nombre
    }
    
    fn sonido(&self) -> &str {
        "Miau"
    }
    
    fn moverse(&self) -> String {
        format!("sigilosamente como un gato {}", self.color)
    }
}

struct Pajaro {
    nombre: String,
    puede_volar: bool,
}

impl Animal for Pajaro {
    fn nombre(&self) -> &str {
        &self.nombre
    }
    
    fn sonido(&self) -> &str {
        "Pío pío"
    }
    
    fn moverse(&self) -> String {
        if self.puede_volar {
            "volando por el cielo".to_string()
        } else {
            "caminando en el suelo".to_string()
        }
    }
}

// ============================================
// EJERCICIO 2: Múltiples Traits
// ============================================
//
// Implementa varios traits para Empleado

trait Nombrable {
    fn nombre_completo(&self) -> String;
}

trait ConEdad {
    fn edad(&self) -> u32;
    fn es_mayor_de_edad(&self) -> bool {
        self.edad() >= 18
    }
}

trait Presentable {
    fn presentacion(&self) -> String;
}

struct Empleado {
    nombre: String,
    apellido: String,
    edad: u32,
    puesto: String,
    salario: f64,
}

impl Nombrable for Empleado {
    fn nombre_completo(&self) -> String {
        format!("{} {}", self.nombre, self.apellido)
    }
}

impl ConEdad for Empleado {
    fn edad(&self) -> u32 {
        self.edad
    }
}

impl Presentable for Empleado {
    fn presentacion(&self) -> String {
        format!(
            "Hola, soy {} ({} años), trabajo como {}",
            self.nombre_completo(),
            self.edad(),
            self.puesto
        )
    }
}

// Trait propio del Empleado
trait Trabajador {
    fn salario_mensual(&self) -> f64;
    fn salario_anual(&self) -> f64 {
        self.salario_mensual() * 12.0
    }
}

impl Trabajador for Empleado {
    fn salario_mensual(&self) -> f64 {
        self.salario
    }
}

// ============================================
// EJERCICIO 3: Supertraits
// ============================================
//
// Define un trait que requiera otros traits

use std::fmt::Display;

// Imprimible requiere que el tipo implemente Display
trait Imprimible: Display {
    fn imprimir(&self) {
        println!("{}", self);
    }
    
    fn imprimir_decorado(&self) {
        println!("*** {} ***", self);
    }
}

struct Mensaje {
    contenido: String,
    prioridad: u8,
}

impl Display for Mensaje {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let icono = match self.prioridad {
            0..=3 => "📝",
            4..=6 => "⚠️",
            _ => "🚨",
        };
        write!(f, "{} {}", icono, self.contenido)
    }
}

impl Imprimible for Mensaje {}

// ============================================
// EJERCICIO 4: Métodos Asociados (Constructores)
// ============================================
//
// Define traits con métodos asociados (sin self)

trait Creador {
    fn crear() -> Self;
    fn crear_con_nombre(nombre: &str) -> Self;
}

trait Reseteable {
    fn reset(&mut self);
}

#[derive(Debug)]
struct Contador {
    nombre: String,
    valor: i32,
}

impl Creador for Contador {
    fn crear() -> Self {
        Contador {
            nombre: String::from("default"),
            valor: 0,
        }
    }
    
    fn crear_con_nombre(nombre: &str) -> Self {
        Contador {
            nombre: nombre.to_string(),
            valor: 0,
        }
    }
}

impl Reseteable for Contador {
    fn reset(&mut self) {
        self.valor = 0;
    }
}

impl Contador {
    fn incrementar(&mut self) {
        self.valor += 1;
    }
    
    fn valor(&self) -> i32 {
        self.valor
    }
}

#[derive(Debug)]
struct Punto {
    x: f64,
    y: f64,
}

impl Creador for Punto {
    fn crear() -> Self {
        Punto { x: 0.0, y: 0.0 }
    }
    
    fn crear_con_nombre(_nombre: &str) -> Self {
        // Los puntos no tienen nombre, pero debemos implementar el método
        Punto { x: 0.0, y: 0.0 }
    }
}

impl Reseteable for Punto {
    fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }
}

impl Punto {
    fn mover(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
    
    fn distancia_origen(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    println!("=== Práctica 02: Implementar Traits ===\n");
    
    // Ejercicio 1: Animal
    println!("--- Ejercicio 1: Animal ---");
    let perro = Perro { 
        nombre: String::from("Max"), 
        raza: String::from("Labrador") 
    };
    let gato = Gato { 
        nombre: String::from("Michi"), 
        color: String::from("naranja") 
    };
    let pajaro = Pajaro { 
        nombre: String::from("Piolín"), 
        puede_volar: true 
    };
    
    println!("{}", perro.presentarse());
    println!("{}", gato.presentarse());
    println!("{}", pajaro.presentarse());
    
    // Ejercicio 2: Múltiples Traits
    println!("\n--- Ejercicio 2: Múltiples Traits ---");
    let empleado = Empleado {
        nombre: String::from("Ana"),
        apellido: String::from("García"),
        edad: 30,
        puesto: String::from("Desarrolladora Rust"),
        salario: 5000.0,
    };
    
    println!("Nombre: {}", empleado.nombre_completo());
    println!("Edad: {} (mayor de edad: {})", empleado.edad(), empleado.es_mayor_de_edad());
    println!("Presentación: {}", empleado.presentacion());
    println!("Salario anual: ${:.2}", empleado.salario_anual());
    
    // Ejercicio 3: Supertraits
    println!("\n--- Ejercicio 3: Supertraits ---");
    let msg_bajo = Mensaje { contenido: String::from("Todo bien"), prioridad: 2 };
    let msg_alto = Mensaje { contenido: String::from("URGENTE"), prioridad: 9 };
    
    msg_bajo.imprimir();
    msg_alto.imprimir_decorado();
    
    // Ejercicio 4: Métodos Asociados
    println!("\n--- Ejercicio 4: Métodos Asociados ---");
    let mut contador = Contador::crear_con_nombre("mi_contador");
    contador.incrementar();
    contador.incrementar();
    println!("Contador: {:?}, valor: {}", contador, contador.valor());
    contador.reset();
    println!("Después de reset: {}", contador.valor());
    
    let mut punto = Punto::crear();
    punto.mover(3.0, 4.0);
    println!("Punto: {:?}, distancia al origen: {}", punto, punto.distancia_origen());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Tests Ejercicio 1: Animal
    #[test]
    fn test_perro_sonido() {
        let perro = Perro { nombre: String::from("Rex"), raza: String::from("Pastor") };
        assert_eq!(perro.sonido(), "Guau guau");
    }
    
    #[test]
    fn test_perro_nombre() {
        let perro = Perro { nombre: String::from("Rex"), raza: String::from("Pastor") };
        assert_eq!(perro.nombre(), "Rex");
    }
    
    #[test]
    fn test_perro_moverse() {
        let perro = Perro { nombre: String::from("Rex"), raza: String::from("Bulldog") };
        assert!(perro.moverse().contains("Bulldog"));
    }
    
    #[test]
    fn test_gato_sonido() {
        let gato = Gato { nombre: String::from("Felix"), color: String::from("negro") };
        assert_eq!(gato.sonido(), "Miau");
    }
    
    #[test]
    fn test_gato_moverse() {
        let gato = Gato { nombre: String::from("Felix"), color: String::from("blanco") };
        assert!(gato.moverse().contains("blanco"));
    }
    
    #[test]
    fn test_pajaro_vuela() {
        let pajaro = Pajaro { nombre: String::from("Tweety"), puede_volar: true };
        assert!(pajaro.moverse().contains("volando"));
    }
    
    #[test]
    fn test_pajaro_no_vuela() {
        let pajaro = Pajaro { nombre: String::from("Pingu"), puede_volar: false };
        assert!(pajaro.moverse().contains("caminando"));
    }
    
    #[test]
    fn test_animal_presentarse() {
        let perro = Perro { nombre: String::from("Buddy"), raza: String::from("Golden") };
        let presentacion = perro.presentarse();
        assert!(presentacion.contains("Buddy"));
        assert!(presentacion.contains("Guau"));
    }
    
    // Tests Ejercicio 2: Múltiples Traits
    #[test]
    fn test_empleado_nombre_completo() {
        let emp = Empleado {
            nombre: String::from("Juan"),
            apellido: String::from("Pérez"),
            edad: 25,
            puesto: String::from("Dev"),
            salario: 3000.0,
        };
        assert_eq!(emp.nombre_completo(), "Juan Pérez");
    }
    
    #[test]
    fn test_empleado_edad() {
        let emp = Empleado {
            nombre: String::from("Test"),
            apellido: String::from("User"),
            edad: 17,
            puesto: String::from("Intern"),
            salario: 1000.0,
        };
        assert_eq!(emp.edad(), 17);
        assert!(!emp.es_mayor_de_edad());
    }
    
    #[test]
    fn test_empleado_mayor_edad() {
        let emp = Empleado {
            nombre: String::from("Adult"),
            apellido: String::from("Person"),
            edad: 25,
            puesto: String::from("Manager"),
            salario: 5000.0,
        };
        assert!(emp.es_mayor_de_edad());
    }
    
    #[test]
    fn test_empleado_salario_anual() {
        let emp = Empleado {
            nombre: String::from("Test"),
            apellido: String::from("User"),
            edad: 30,
            puesto: String::from("Dev"),
            salario: 1000.0,
        };
        assert_eq!(emp.salario_anual(), 12000.0);
    }
    
    #[test]
    fn test_empleado_presentacion() {
        let emp = Empleado {
            nombre: String::from("María"),
            apellido: String::from("López"),
            edad: 28,
            puesto: String::from("Designer"),
            salario: 4000.0,
        };
        let pres = emp.presentacion();
        assert!(pres.contains("María López"));
        assert!(pres.contains("28"));
        assert!(pres.contains("Designer"));
    }
    
    // Tests Ejercicio 3: Supertraits
    #[test]
    fn test_mensaje_display() {
        let msg = Mensaje { contenido: String::from("Hola"), prioridad: 1 };
        let texto = format!("{}", msg);
        assert!(texto.contains("Hola"));
        assert!(texto.contains("📝"));
    }
    
    #[test]
    fn test_mensaje_prioridad_media() {
        let msg = Mensaje { contenido: String::from("Aviso"), prioridad: 5 };
        let texto = format!("{}", msg);
        assert!(texto.contains("⚠️"));
    }
    
    #[test]
    fn test_mensaje_prioridad_alta() {
        let msg = Mensaje { contenido: String::from("Alerta"), prioridad: 8 };
        let texto = format!("{}", msg);
        assert!(texto.contains("🚨"));
    }
    
    // Tests Ejercicio 4: Métodos Asociados
    #[test]
    fn test_contador_crear() {
        let contador = Contador::crear();
        assert_eq!(contador.valor(), 0);
        assert_eq!(contador.nombre, "default");
    }
    
    #[test]
    fn test_contador_crear_con_nombre() {
        let contador = Contador::crear_con_nombre("test");
        assert_eq!(contador.nombre, "test");
    }
    
    #[test]
    fn test_contador_incrementar() {
        let mut contador = Contador::crear();
        contador.incrementar();
        contador.incrementar();
        assert_eq!(contador.valor(), 2);
    }
    
    #[test]
    fn test_contador_reset() {
        let mut contador = Contador::crear();
        contador.incrementar();
        contador.incrementar();
        contador.reset();
        assert_eq!(contador.valor(), 0);
    }
    
    #[test]
    fn test_punto_crear() {
        let punto = Punto::crear();
        assert_eq!(punto.x, 0.0);
        assert_eq!(punto.y, 0.0);
    }
    
    #[test]
    fn test_punto_mover() {
        let mut punto = Punto::crear();
        punto.mover(3.0, 4.0);
        assert_eq!(punto.x, 3.0);
        assert_eq!(punto.y, 4.0);
    }
    
    #[test]
    fn test_punto_distancia() {
        let mut punto = Punto::crear();
        punto.mover(3.0, 4.0);
        assert!((punto.distancia_origen() - 5.0).abs() < 0.001);
    }
    
    #[test]
    fn test_punto_reset() {
        let mut punto = Punto::crear();
        punto.mover(10.0, 20.0);
        punto.reset();
        assert_eq!(punto.x, 0.0);
        assert_eq!(punto.y, 0.0);
    }
}
