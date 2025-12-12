// Práctica 02: Propagación de Errores
// ====================================
// Aprende a usar el operador ? para propagar errores de forma elegante.
//
// OBJETIVO:
// - Usar el operador ? para propagar errores
// - Entender la conversión automática con From
// - Encadenar múltiples operaciones que pueden fallar
//
// INSTRUCCIONES:
// 1. Completa cada función usando el operador ?
// 2. Ejecuta el programa para verificar tu solución
// 3. Los tests deben pasar: cargo test

use std::fs;
use std::io;
use std::path::Path;

fn main() {
    println!("=== Práctica: Propagación de Errores ===\n");

    // Ejercicio 1: Leer y parsear
    println!("1. Leer archivo y sumar líneas:");
    match sumar_lineas_archivo("numeros.txt") {
        Ok(suma) => println!("   Suma: {}", suma),
        Err(e) => println!("   Error: {}", e),
    }

    // Crear archivo de prueba
    let _ = fs::write("test_numeros.txt", "10\n20\n30\n");
    match sumar_lineas_archivo("test_numeros.txt") {
        Ok(suma) => println!("   Suma de test_numeros.txt: {}", suma),
        Err(e) => println!("   Error: {}", e),
    }
    let _ = fs::remove_file("test_numeros.txt");

    // Ejercicio 2: Validación encadenada
    println!("\n2. Validar usuario:");
    println!("   'Ana' (25) → {:?}", validar_usuario("Ana", 25));
    println!("   '' (25) → {:?}", validar_usuario("", 25));
    println!("   'Bob' (200) → {:?}", validar_usuario("Bob", 200));

    // Ejercicio 3: Procesar configuración
    println!("\n3. Procesar config:");
    let _ = fs::write("config.txt", "puerto=8080\nhost=localhost\n");
    match obtener_puerto("config.txt") {
        Ok(puerto) => println!("   Puerto: {}", puerto),
        Err(e) => println!("   Error: {}", e),
    }
    let _ = fs::remove_file("config.txt");
}

// ============================================================================
// EJERCICIO 1: Leer archivo y sumar números
// ============================================================================
// Lee un archivo con números (uno por línea) y retorna su suma.
// Usa el operador ? para propagar errores.

fn sumar_lineas_archivo(ruta: &str) -> Result<i64, String> {
    // TODO: Implementar usando ?
    // 1. Leer el archivo con fs::read_to_string(ruta)
    //    - Convertir el error: .map_err(|e| format!("Error leyendo: {}", e))?
    // 2. Para cada línea, parsear a i64 y sumar
    //    - Puedes usar un for loop o .lines().map().sum()
    // 3. Retornar Ok(suma)
    //
    // PISTA: Para el parseo de cada línea:
    //   linea.trim().parse::<i64>().map_err(|_| format!("Línea inválida: {}", linea))?
    todo!("Implementar sumar_lineas_archivo")
}

// ============================================================================
// EJERCICIO 2: Validación encadenada
// ============================================================================
// Valida nombre y edad, propagando el primer error encontrado.

#[derive(Debug, PartialEq)]
struct Usuario {
    nombre: String,
    edad: u8,
}

fn validar_usuario(nombre: &str, edad: i32) -> Result<Usuario, String> {
    // TODO: Implementar usando ?
    // 1. Validar nombre con validar_nombre(nombre)?
    // 2. Validar edad con validar_edad(edad)?
    // 3. Retornar Ok(Usuario { nombre, edad })
    todo!("Implementar validar_usuario")
}

fn validar_nombre(nombre: &str) -> Result<String, String> {
    // TODO: Implementar
    // - Si está vacío, Err("El nombre no puede estar vacío")
    // - Si tiene menos de 2 caracteres, Err("Nombre muy corto")
    // - Si no, Ok(nombre.to_string())
    todo!("Implementar validar_nombre")
}

fn validar_edad(edad: i32) -> Result<u8, String> {
    // TODO: Implementar
    // - Si es negativa, Err("La edad no puede ser negativa")
    // - Si es > 150, Err("Edad fuera de rango")
    // - Si no, Ok(edad as u8)
    todo!("Implementar validar_edad")
}

// ============================================================================
// EJERCICIO 3: Procesar configuración
// ============================================================================
// Lee un archivo de config y extrae el puerto.
// Formato: clave=valor (una por línea)

fn obtener_puerto(ruta: &str) -> Result<u16, String> {
    // TODO: Implementar usando ?
    // 1. Leer archivo
    // 2. Buscar línea que empiece con "puerto="
    // 3. Extraer el valor después del =
    // 4. Parsear a u16
    //
    // PISTA: Usa .lines().find(|l| l.starts_with("puerto="))
    todo!("Implementar obtener_puerto")
}

// ============================================================================
// TESTS
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sumar_lineas_ok() {
        fs::write("test_suma.txt", "1\n2\n3\n4\n5").unwrap();
        assert_eq!(sumar_lineas_archivo("test_suma.txt"), Ok(15));
        fs::remove_file("test_suma.txt").unwrap();
    }

    #[test]
    fn test_sumar_lineas_archivo_no_existe() {
        assert!(sumar_lineas_archivo("no_existe.txt").is_err());
    }

    #[test]
    fn test_sumar_lineas_formato_invalido() {
        fs::write("test_invalid.txt", "1\nabc\n3").unwrap();
        assert!(sumar_lineas_archivo("test_invalid.txt").is_err());
        fs::remove_file("test_invalid.txt").unwrap();
    }

    #[test]
    fn test_validar_usuario_ok() {
        let result = validar_usuario("Ana", 25);
        assert!(result.is_ok());
        let usuario = result.unwrap();
        assert_eq!(usuario.nombre, "Ana");
        assert_eq!(usuario.edad, 25);
    }

    #[test]
    fn test_validar_usuario_nombre_vacio() {
        assert!(validar_usuario("", 25).is_err());
    }

    #[test]
    fn test_validar_usuario_edad_invalida() {
        assert!(validar_usuario("Ana", -5).is_err());
        assert!(validar_usuario("Ana", 200).is_err());
    }

    #[test]
    fn test_obtener_puerto_ok() {
        fs::write("test_config.txt", "host=localhost\npuerto=3000\n").unwrap();
        assert_eq!(obtener_puerto("test_config.txt"), Ok(3000));
        fs::remove_file("test_config.txt").unwrap();
    }

    #[test]
    fn test_obtener_puerto_no_existe() {
        assert!(obtener_puerto("no_existe.txt").is_err());
    }

    #[test]
    fn test_obtener_puerto_sin_puerto() {
        fs::write("test_no_port.txt", "host=localhost\n").unwrap();
        assert!(obtener_puerto("test_no_port.txt").is_err());
        fs::remove_file("test_no_port.txt").unwrap();
    }
}
