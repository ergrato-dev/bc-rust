// Práctica 04: Conversión de Errores
// ===================================
// Aprende a convertir entre diferentes tipos de error y usar Box<dyn Error>.
//
// OBJETIVO:
// - Convertir errores con map_err
// - Usar Box<dyn Error> para errores heterogéneos
// - Agregar contexto a los errores
//
// INSTRUCCIONES:
// 1. Completa las funciones de conversión
// 2. Practica diferentes estrategias de manejo

use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;

fn main() {
    println!("=== Práctica: Conversión de Errores ===\n");

    // Ejercicio 1: map_err básico
    println!("1. Conversión con map_err:");
    println!("   parsear_puerto('8080') → {:?}", parsear_puerto("8080"));
    println!("   parsear_puerto('abc') → {:?}", parsear_puerto("abc"));

    // Ejercicio 2: Box<dyn Error>
    println!("\n2. Box<dyn Error>:");
    let _ = fs::write("datos.txt", "42\n");
    match leer_y_parsear("datos.txt") {
        Ok(n) => println!("   Número: {}", n),
        Err(e) => println!("   Error: {}", e),
    }
    let _ = fs::remove_file("datos.txt");

    match leer_y_parsear("no_existe.txt") {
        Ok(n) => println!("   Número: {}", n),
        Err(e) => println!("   Error: {}", e),
    }

    // Ejercicio 3: Agregar contexto
    println!("\n3. Con contexto:");
    let _ = fs::write("config.txt", "100\n");
    match leer_con_contexto("config.txt") {
        Ok(n) => println!("   Valor: {}", n),
        Err(e) => println!("   Error: {}", e),
    }
    let _ = fs::remove_file("config.txt");

    // Ejercicio 4: Result con múltiples errores
    println!("\n4. Procesar múltiples archivos:");
    let _ = fs::write("a.txt", "10\n");
    let _ = fs::write("b.txt", "20\n");
    match sumar_archivos(&["a.txt", "b.txt", "c.txt"]) {
        Ok(suma) => println!("   Suma: {}", suma),
        Err(e) => println!("   Error: {}", e),
    }
    let _ = fs::remove_file("a.txt");
    let _ = fs::remove_file("b.txt");
}

// ============================================================================
// EJERCICIO 1: Conversión con map_err
// ============================================================================
// Convierte ParseIntError a un String descriptivo.

fn parsear_puerto(s: &str) -> Result<u16, String> {
    // TODO: Implementar
    // Parsear s a u16 y convertir el error a un mensaje descriptivo
    // Ejemplo de mensaje: "Puerto inválido 'abc': invalid digit found in string"
    //
    // PISTA: Usa .map_err(|e| format!("Puerto inválido '{}': {}", s, e))
    todo!("Implementar parsear_puerto")
}

// ============================================================================
// EJERCICIO 2: Box<dyn Error>
// ============================================================================
// Usa Box<dyn Error> para manejar diferentes tipos de error.

fn leer_y_parsear(ruta: &str) -> Result<i32, Box<dyn Error>> {
    // TODO: Implementar
    // 1. Leer archivo con fs::read_to_string(ruta)?
    // 2. Parsear contenido a i32 con contenido.trim().parse()?
    // 3. Retornar Ok(numero)
    //
    // Nota: El ? convierte automáticamente a Box<dyn Error>
    todo!("Implementar leer_y_parsear")
}

// ============================================================================
// EJERCICIO 3: Agregar contexto
// ============================================================================
// Envuelve errores con contexto adicional.

#[derive(Debug)]
struct ErrorConContexto {
    contexto: String,
    causa: Box<dyn Error>,
}

impl fmt::Display for ErrorConContexto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.contexto, self.causa)
    }
}

impl Error for ErrorConContexto {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.causa.as_ref())
    }
}

// Trait extension para agregar contexto
trait ConContexto<T> {
    fn contexto(self, msg: &str) -> Result<T, ErrorConContexto>;
}

impl<T, E: Error + 'static> ConContexto<T> for Result<T, E> {
    fn contexto(self, msg: &str) -> Result<T, ErrorConContexto> {
        self.map_err(|e| ErrorConContexto {
            contexto: msg.to_string(),
            causa: Box::new(e),
        })
    }
}

fn leer_con_contexto(ruta: &str) -> Result<i32, ErrorConContexto> {
    // TODO: Implementar usando el trait ConContexto
    // 1. Leer archivo y agregar contexto "Leyendo archivo"
    // 2. Parsear y agregar contexto "Parseando número"
    //
    // PISTA: 
    //   fs::read_to_string(ruta).contexto("Leyendo archivo")?
    todo!("Implementar leer_con_contexto")
}

// ============================================================================
// EJERCICIO 4: Colección de Results
// ============================================================================
// Procesa múltiples archivos y retorna el primer error o la suma.

fn sumar_archivos(rutas: &[&str]) -> Result<i64, String> {
    // TODO: Implementar
    // Para cada ruta en rutas:
    //   1. Leer el archivo
    //   2. Parsear a i64
    //   3. Sumar al total
    // Si cualquier operación falla, retornar el error con contexto
    //
    // PISTA: Puedes usar un for loop con ? o .map().collect()
    todo!("Implementar sumar_archivos")
}

// ============================================================================
// BONUS: Collect Results
// ============================================================================
// Recolecta todos los resultados o retorna el primer error.

fn parsear_todos(strings: &[&str]) -> Result<Vec<i32>, String> {
    // TODO: Implementar
    // Parsear cada string a i32
    // Retornar Vec<i32> si todos son válidos
    // Retornar error si alguno falla
    //
    // PISTA: Usa .map(|s| s.parse()).collect()
    // collect() puede recolectar Result<Vec<T>, E>
    todo!("Implementar parsear_todos")
}

// ============================================================================
// TESTS
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsear_puerto_ok() {
        assert_eq!(parsear_puerto("8080"), Ok(8080));
        assert_eq!(parsear_puerto("443"), Ok(443));
    }

    #[test]
    fn test_parsear_puerto_error() {
        let result = parsear_puerto("abc");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("abc"));
    }

    #[test]
    fn test_leer_y_parsear_ok() {
        fs::write("test_num.txt", "123\n").unwrap();
        assert_eq!(leer_y_parsear("test_num.txt").unwrap(), 123);
        fs::remove_file("test_num.txt").unwrap();
    }

    #[test]
    fn test_leer_y_parsear_io_error() {
        assert!(leer_y_parsear("no_existe_xyz.txt").is_err());
    }

    #[test]
    fn test_leer_y_parsear_parse_error() {
        fs::write("test_bad.txt", "not a number\n").unwrap();
        assert!(leer_y_parsear("test_bad.txt").is_err());
        fs::remove_file("test_bad.txt").unwrap();
    }

    #[test]
    fn test_leer_con_contexto() {
        fs::write("test_ctx.txt", "42\n").unwrap();
        assert_eq!(leer_con_contexto("test_ctx.txt").unwrap(), 42);
        fs::remove_file("test_ctx.txt").unwrap();
    }

    #[test]
    fn test_sumar_archivos_ok() {
        fs::write("sum_a.txt", "10\n").unwrap();
        fs::write("sum_b.txt", "20\n").unwrap();
        assert_eq!(sumar_archivos(&["sum_a.txt", "sum_b.txt"]).unwrap(), 30);
        fs::remove_file("sum_a.txt").unwrap();
        fs::remove_file("sum_b.txt").unwrap();
    }

    #[test]
    fn test_sumar_archivos_error() {
        fs::write("sum_ok.txt", "10\n").unwrap();
        assert!(sumar_archivos(&["sum_ok.txt", "no_existe.txt"]).is_err());
        fs::remove_file("sum_ok.txt").unwrap();
    }

    #[test]
    fn test_parsear_todos_ok() {
        assert_eq!(parsear_todos(&["1", "2", "3"]).unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_parsear_todos_error() {
        assert!(parsear_todos(&["1", "abc", "3"]).is_err());
    }
}
