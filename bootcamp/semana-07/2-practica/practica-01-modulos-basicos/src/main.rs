// Práctica 01: Módulos Básicos
// Semana 07 - Módulos y Crates

// =============================================================================
// EJERCICIO 1: Módulos Inline - Calculadora
// =============================================================================

mod operaciones {
    pub fn sumar(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn restar(a: i32, b: i32) -> i32 {
        a - b
    }

    pub mod avanzadas {
        pub fn multiplicar(a: i32, b: i32) -> i32 {
            a * b
        }

        pub fn dividir(a: i32, b: i32) -> Option<i32> {
            if b == 0 { None } else { Some(a / b) }
        }
    }
}

// =============================================================================
// EJERCICIO 2: Navegación con super y self
// =============================================================================

mod biblioteca {
    pub const NOMBRE: &str = "Biblioteca Rust";

    pub mod libros {
        pub fn titulo_completo(titulo: &str) -> String {
            // TODO: Usar super:: para acceder a NOMBRE del módulo padre
            // Retornar formato: "NOMBRE: titulo"
            format!("{}: {}", super::NOMBRE, titulo)
        }

        mod interno {
            pub fn procesar(titulo: &str) -> String {
                // TODO: Usar super:: para llamar a titulo_completo
                format!("[Procesado] {}", super::titulo_completo(titulo))
            }
        }

        pub fn procesar_publico(titulo: &str) -> String {
            // TODO: Usar self:: para llamar al módulo interno
            self::interno::procesar(titulo)
        }
    }
}

// =============================================================================
// EJERCICIO 3: Árbol de Módulos - Vehículos
// =============================================================================

mod vehiculos {
    pub mod terrestres {
        pub mod automovil {
            pub struct Automovil {
                marca: String,
            }

            impl Automovil {
                pub fn new(marca: &str) -> Self {
                    Self {
                        marca: marca.to_string(),
                    }
                }

                pub fn describir(&self) -> String {
                    format!("Automóvil marca: {}", self.marca)
                }
            }
        }

        pub mod motocicleta {
            pub struct Motocicleta {
                marca: String,
            }

            impl Motocicleta {
                pub fn new(marca: &str) -> Self {
                    Self {
                        marca: marca.to_string(),
                    }
                }

                pub fn describir(&self) -> String {
                    format!("Motocicleta marca: {}", self.marca)
                }
            }
        }
    }

    pub mod aereos {
        pub mod avion {
            pub struct Avion {
                marca: String,
            }

            impl Avion {
                pub fn new(marca: &str) -> Self {
                    Self {
                        marca: marca.to_string(),
                    }
                }

                pub fn describir(&self) -> String {
                    format!("Avión marca: {}", self.marca)
                }
            }
        }
    }
}

// =============================================================================
// FUNCIÓN PRINCIPAL
// =============================================================================

fn main() {
    println!("=== Práctica 01: Módulos Básicos ===\n");

    // Ejercicio 1: Operaciones
    println!("--- Ejercicio 1: Calculadora ---");
    let suma = crate::operaciones::sumar(10, 5);
    let resta = crate::operaciones::restar(10, 5);
    let producto = crate::operaciones::avanzadas::multiplicar(3, 4);
    let division = crate::operaciones::avanzadas::dividir(20, 4);
    let division_cero = crate::operaciones::avanzadas::dividir(10, 0);

    println!("10 + 5 = {}", suma);
    println!("10 - 5 = {}", resta);
    println!("3 * 4 = {}", producto);
    println!("20 / 4 = {:?}", division);
    println!("10 / 0 = {:?}", division_cero);

    // Ejercicio 2: Biblioteca
    println!("\n--- Ejercicio 2: Biblioteca ---");
    let titulo = biblioteca::libros::titulo_completo("El Quijote");
    let procesado = biblioteca::libros::procesar_publico("Don Quijote");

    println!("Título: {}", titulo);
    println!("Procesado: {}", procesado);

    // Ejercicio 3: Vehículos
    println!("\n--- Ejercicio 3: Vehículos ---");
    let auto = vehiculos::terrestres::automovil::Automovil::new("Toyota");
    let moto = vehiculos::terrestres::motocicleta::Motocicleta::new("Honda");
    let avion = vehiculos::aereos::avion::Avion::new("Boeing");

    println!("{}", auto.describir());
    println!("{}", moto.describir());
    println!("{}", avion.describir());
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operaciones_basicas() {
        assert_eq!(operaciones::sumar(5, 3), 8);
        assert_eq!(operaciones::sumar(-5, 5), 0);
        assert_eq!(operaciones::restar(10, 4), 6);
        assert_eq!(operaciones::restar(5, 10), -5);
    }

    #[test]
    fn test_operaciones_avanzadas() {
        assert_eq!(operaciones::avanzadas::multiplicar(6, 7), 42);
        assert_eq!(operaciones::avanzadas::multiplicar(-3, 4), -12);
        assert_eq!(operaciones::avanzadas::dividir(10, 2), Some(5));
        assert_eq!(operaciones::avanzadas::dividir(10, 0), None);
        assert_eq!(operaciones::avanzadas::dividir(7, 2), Some(3)); // División entera
    }

    #[test]
    fn test_biblioteca_titulo() {
        let titulo = biblioteca::libros::titulo_completo("El Quijote");
        assert!(titulo.contains("Biblioteca Rust"));
        assert!(titulo.contains("El Quijote"));
        assert_eq!(titulo, "Biblioteca Rust: El Quijote");
    }

    #[test]
    fn test_biblioteca_procesado() {
        let procesado = biblioteca::libros::procesar_publico("Test");
        assert!(procesado.contains("[Procesado]"));
        assert!(procesado.contains("Biblioteca Rust"));
    }

    #[test]
    fn test_automovil() {
        let auto = vehiculos::terrestres::automovil::Automovil::new("Toyota");
        let descripcion = auto.describir();
        assert!(descripcion.contains("Toyota"));
        assert!(descripcion.contains("Automóvil"));
    }

    #[test]
    fn test_motocicleta() {
        let moto = vehiculos::terrestres::motocicleta::Motocicleta::new("Yamaha");
        let descripcion = moto.describir();
        assert!(descripcion.contains("Yamaha"));
        assert!(descripcion.contains("Motocicleta"));
    }

    #[test]
    fn test_avion() {
        let avion = vehiculos::aereos::avion::Avion::new("Airbus");
        let descripcion = avion.describir();
        assert!(descripcion.contains("Airbus"));
        assert!(descripcion.contains("Avión"));
    }
}
