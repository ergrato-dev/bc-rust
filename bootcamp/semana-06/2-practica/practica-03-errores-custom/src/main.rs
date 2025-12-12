// Práctica 03: Errores Personalizados
// ====================================
// Aprende a crear tipos de error expresivos y bien estructurados.
//
// OBJETIVO:
// - Definir un enum de error con múltiples variantes
// - Implementar Display y Error
// - Implementar From para conversión automática
//
// INSTRUCCIONES:
// 1. Completa la definición del tipo de error
// 2. Implementa los traits necesarios
// 3. Usa el tipo en las funciones

use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;

fn main() {
    println!("=== Práctica: Errores Personalizados ===\n");

    // Test del tipo de error
    println!("1. Tipos de error:");
    let e1 = ConfigError::ArchivoNoEncontrado("config.toml".to_string());
    println!("   {}", e1);
    
    let e2 = ConfigError::CampoFaltante("puerto".to_string());
    println!("   {}", e2);
    
    let e3 = ConfigError::ValorInvalido {
        campo: "puerto".to_string(),
        valor: "abc".to_string(),
        mensaje: "debe ser un número".to_string(),
    };
    println!("   {}", e3);

    // Test de uso
    println!("\n2. Cargar configuración:");
    
    // Crear archivo de prueba válido
    let _ = fs::write("test_config.toml", "puerto = 8080\nhost = localhost\n");
    match cargar_config("test_config.toml") {
        Ok(config) => println!("   Config: {:?}", config),
        Err(e) => println!("   Error: {}", e),
    }
    let _ = fs::remove_file("test_config.toml");

    // Archivo que no existe
    match cargar_config("no_existe.toml") {
        Ok(config) => println!("   Config: {:?}", config),
        Err(e) => println!("   Error: {}", e),
    }

    // Archivo con valor inválido
    let _ = fs::write("bad_config.toml", "puerto = abc\n");
    match cargar_config("bad_config.toml") {
        Ok(config) => println!("   Config: {:?}", config),
        Err(e) => println!("   Error: {}", e),
    }
    let _ = fs::remove_file("bad_config.toml");
}

// ============================================================================
// EJERCICIO 1: Definir el tipo de error
// ============================================================================

#[derive(Debug)]
enum ConfigError {
    // TODO: Definir las variantes:
    // - ArchivoNoEncontrado(String) - ruta del archivo
    // - ErrorLectura(io::Error) - error de I/O subyacente
    // - CampoFaltante(String) - nombre del campo
    // - ValorInvalido { campo: String, valor: String, mensaje: String }
    ArchivoNoEncontrado(String),
    ErrorLectura(io::Error),
    CampoFaltante(String),
    ValorInvalido {
        campo: String,
        valor: String,
        mensaje: String,
    },
}

// ============================================================================
// EJERCICIO 2: Implementar Display
// ============================================================================

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Implementar mensajes descriptivos para cada variante
        // 
        // Ejemplos de salida:
        // - "Archivo no encontrado: config.toml"
        // - "Error de lectura: Permission denied"
        // - "Campo faltante: puerto"
        // - "Valor inválido en 'puerto': 'abc' - debe ser un número"
        todo!("Implementar Display para ConfigError")
    }
}

// ============================================================================
// EJERCICIO 3: Implementar Error con source()
// ============================================================================

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // TODO: Retornar la causa subyacente si existe
        // - Para ErrorLectura, retornar Some(&io_error)
        // - Para los demás, retornar None
        todo!("Implementar source() para ConfigError")
    }
}

// ============================================================================
// EJERCICIO 4: Implementar From<io::Error>
// ============================================================================

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        // TODO: Convertir io::Error a ConfigError::ErrorLectura
        todo!("Implementar From<io::Error>")
    }
}

impl From<ParseIntError> for ConfigError {
    fn from(_err: ParseIntError) -> Self {
        // Para ParseIntError, necesitamos más contexto
        // Por ahora retornamos un error genérico
        ConfigError::ValorInvalido {
            campo: "desconocido".to_string(),
            valor: "".to_string(),
            mensaje: "error de parseo".to_string(),
        }
    }
}

// ============================================================================
// EJERCICIO 5: Usar el tipo de error
// ============================================================================

#[derive(Debug)]
struct Config {
    puerto: u16,
    host: String,
}

fn cargar_config(ruta: &str) -> Result<Config, ConfigError> {
    // TODO: Implementar
    // 1. Leer el archivo (usa ? - se convertirá automáticamente)
    // 2. Buscar línea con "puerto = "
    //    - Si no existe, retornar Err(ConfigError::CampoFaltante("puerto"))
    // 3. Parsear el valor del puerto
    //    - Si falla, retornar ConfigError::ValorInvalido apropiado
    // 4. Buscar línea con "host = "
    //    - Si no existe, usar "localhost" como default
    // 5. Retornar Ok(Config { puerto, host })
    //
    // Formato del archivo:
    // puerto = 8080
    // host = localhost
    todo!("Implementar cargar_config")
}

// Función auxiliar para extraer valor de una línea "clave = valor"
fn extraer_valor<'a>(linea: &'a str, clave: &str) -> Option<&'a str> {
    if linea.starts_with(clave) {
        linea.split('=').nth(1).map(|v| v.trim())
    } else {
        None
    }
}

// ============================================================================
// TESTS
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let e = ConfigError::ArchivoNoEncontrado("test.toml".to_string());
        assert!(e.to_string().contains("test.toml"));

        let e = ConfigError::CampoFaltante("puerto".to_string());
        assert!(e.to_string().contains("puerto"));
    }

    #[test]
    fn test_error_from_io() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "not found");
        let config_err: ConfigError = io_err.into();
        assert!(matches!(config_err, ConfigError::ErrorLectura(_)));
    }

    #[test]
    fn test_cargar_config_ok() {
        fs::write("test_ok.toml", "puerto = 3000\nhost = example.com\n").unwrap();
        let config = cargar_config("test_ok.toml").unwrap();
        assert_eq!(config.puerto, 3000);
        assert_eq!(config.host, "example.com");
        fs::remove_file("test_ok.toml").unwrap();
    }

    #[test]
    fn test_cargar_config_archivo_no_existe() {
        let result = cargar_config("no_existe_xyz.toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_cargar_config_sin_puerto() {
        fs::write("test_no_port.toml", "host = localhost\n").unwrap();
        let result = cargar_config("test_no_port.toml");
        assert!(matches!(result, Err(ConfigError::CampoFaltante(_))));
        fs::remove_file("test_no_port.toml").unwrap();
    }

    #[test]
    fn test_cargar_config_puerto_invalido() {
        fs::write("test_bad.toml", "puerto = abc\n").unwrap();
        let result = cargar_config("test_bad.toml");
        assert!(matches!(result, Err(ConfigError::ValorInvalido { .. })));
        fs::remove_file("test_bad.toml").unwrap();
    }
}
