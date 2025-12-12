// Proyecto Semanal: Validador de Configuración
// =============================================
// 
// Construye un sistema robusto para cargar, validar y procesar
// archivos de configuración con manejo completo de errores.
//
// DOMINIO: Sistema de configuración para una aplicación web
//
// FUNCIONALIDADES:
// 1. Leer archivo de configuración
// 2. Parsear formato clave=valor
// 3. Validar campos requeridos y valores
// 4. Proveer valores por defecto
// 5. Reportar todos los errores encontrados

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;

fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║     Validador de Configuración v1.0          ║");
    println!("╚══════════════════════════════════════════════╝\n");

    // Crear archivo de configuración de ejemplo
    crear_config_ejemplo();

    // Cargar y mostrar configuración
    match Config::cargar("app.config") {
        Ok(config) => {
            println!("✓ Configuración cargada exitosamente:\n");
            println!("{}", config);
        }
        Err(errores) => {
            println!("✗ Errores en la configuración:\n");
            for (i, e) in errores.iter().enumerate() {
                println!("  {}. {}", i + 1, e);
            }
        }
    }

    // Limpiar
    let _ = fs::remove_file("app.config");

    // Probar con configuración inválida
    println!("\n--- Probando configuración inválida ---\n");
    crear_config_invalida();
    
    match Config::cargar("bad.config") {
        Ok(config) => println!("Config: {}", config),
        Err(errores) => {
            println!("✗ Se encontraron {} errores:", errores.len());
            for e in &errores {
                println!("  • {}", e);
            }
        }
    }
    
    let _ = fs::remove_file("bad.config");
}

fn crear_config_ejemplo() {
    let contenido = r#"# Configuración de la aplicación
nombre = MiApp
version = 1.0.0
puerto = 8080
host = localhost
max_conexiones = 100
timeout_ms = 5000
debug = true
"#;
    fs::write("app.config", contenido).expect("No se pudo crear archivo");
}

fn crear_config_invalida() {
    let contenido = r#"# Configuración con errores
nombre = 
puerto = abc
max_conexiones = -50
"#;
    fs::write("bad.config", contenido).expect("No se pudo crear archivo");
}

// ============================================================================
// TIPOS DE ERROR
// ============================================================================

#[derive(Debug, Clone)]
pub enum ConfigError {
    /// Error al leer el archivo
    IoError(String),
    /// Línea con formato inválido
    FormatoInvalido { linea: usize, contenido: String },
    /// Campo requerido faltante
    CampoFaltante(String),
    /// Valor vacío para un campo
    ValorVacio(String),
    /// Valor con tipo incorrecto
    TipoIncorrecto { campo: String, esperado: String, encontrado: String },
    /// Valor fuera de rango permitido
    FueraDeRango { campo: String, valor: String, rango: String },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Implementar mensajes descriptivos para cada variante
        match self {
            ConfigError::IoError(msg) => write!(f, "Error de I/O: {}", msg),
            ConfigError::FormatoInvalido { linea, contenido } => {
                write!(f, "Línea {}: formato inválido '{}'", linea, contenido)
            }
            ConfigError::CampoFaltante(campo) => {
                write!(f, "Campo requerido faltante: '{}'", campo)
            }
            ConfigError::ValorVacio(campo) => {
                write!(f, "Valor vacío para campo '{}'", campo)
            }
            ConfigError::TipoIncorrecto { campo, esperado, encontrado } => {
                write!(f, "Campo '{}': esperado {}, encontrado '{}'", campo, esperado, encontrado)
            }
            ConfigError::FueraDeRango { campo, valor, rango } => {
                write!(f, "Campo '{}': valor {} fuera de rango ({})", campo, valor, rango)
            }
        }
    }
}

impl Error for ConfigError {}

// ============================================================================
// CONFIGURACIÓN
// ============================================================================

#[derive(Debug)]
pub struct Config {
    pub nombre: String,
    pub version: String,
    pub puerto: u16,
    pub host: String,
    pub max_conexiones: u32,
    pub timeout_ms: u64,
    pub debug: bool,
}

impl Config {
    /// Campos requeridos que deben estar presentes
    const CAMPOS_REQUERIDOS: &'static [&'static str] = &["nombre", "puerto"];

    /// Carga configuración desde un archivo.
    /// Retorna la configuración o una lista de todos los errores encontrados.
    pub fn cargar(ruta: &str) -> Result<Self, Vec<ConfigError>> {
        let mut errores = Vec::new();

        // Leer archivo
        let contenido = match fs::read_to_string(ruta) {
            Ok(c) => c,
            Err(e) => {
                return Err(vec![ConfigError::IoError(e.to_string())]);
            }
        };

        // Parsear a HashMap
        let valores = Self::parsear_contenido(&contenido, &mut errores);

        // Validar campos requeridos
        Self::validar_requeridos(&valores, &mut errores);

        // Extraer y validar cada campo
        let nombre = Self::extraer_string(&valores, "nombre", &mut errores);
        let version = Self::extraer_string_default(&valores, "version", "0.0.0");
        let puerto = Self::extraer_u16(&valores, "puerto", &mut errores);
        let host = Self::extraer_string_default(&valores, "host", "localhost");
        let max_conexiones = Self::extraer_u32_rango(&valores, "max_conexiones", 1, 10000, 100, &mut errores);
        let timeout_ms = Self::extraer_u64_default(&valores, "timeout_ms", 3000);
        let debug = Self::extraer_bool_default(&valores, "debug", false);

        // Si hay errores, retornarlos
        if !errores.is_empty() {
            return Err(errores);
        }

        Ok(Config {
            nombre,
            version,
            puerto,
            host,
            max_conexiones,
            timeout_ms,
            debug,
        })
    }

    /// Parsea el contenido del archivo a un HashMap
    fn parsear_contenido(contenido: &str, errores: &mut Vec<ConfigError>) -> HashMap<String, String> {
        let mut map = HashMap::new();

        for (num_linea, linea) in contenido.lines().enumerate() {
            let linea = linea.trim();
            
            // Ignorar líneas vacías y comentarios
            if linea.is_empty() || linea.starts_with('#') {
                continue;
            }

            // Buscar el separador =
            if let Some(pos) = linea.find('=') {
                let clave = linea[..pos].trim().to_string();
                let valor = linea[pos + 1..].trim().to_string();
                map.insert(clave, valor);
            } else {
                errores.push(ConfigError::FormatoInvalido {
                    linea: num_linea + 1,
                    contenido: linea.to_string(),
                });
            }
        }

        map
    }

    /// Valida que todos los campos requeridos estén presentes
    fn validar_requeridos(valores: &HashMap<String, String>, errores: &mut Vec<ConfigError>) {
        for campo in Self::CAMPOS_REQUERIDOS {
            if !valores.contains_key(*campo) {
                errores.push(ConfigError::CampoFaltante(campo.to_string()));
            }
        }
    }

    /// Extrae un campo string requerido
    fn extraer_string(
        valores: &HashMap<String, String>,
        campo: &str,
        errores: &mut Vec<ConfigError>,
    ) -> String {
        match valores.get(campo) {
            Some(v) if !v.is_empty() => v.clone(),
            Some(_) => {
                errores.push(ConfigError::ValorVacio(campo.to_string()));
                String::new()
            }
            None => String::new(), // Ya reportado en validar_requeridos
        }
    }

    /// Extrae un campo string con valor por defecto
    fn extraer_string_default(
        valores: &HashMap<String, String>,
        campo: &str,
        default: &str,
    ) -> String {
        valores.get(campo)
            .filter(|v| !v.is_empty())
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    /// Extrae un campo u16
    fn extraer_u16(
        valores: &HashMap<String, String>,
        campo: &str,
        errores: &mut Vec<ConfigError>,
    ) -> u16 {
        match valores.get(campo) {
            Some(v) => match v.parse::<u16>() {
                Ok(n) => n,
                Err(_) => {
                    errores.push(ConfigError::TipoIncorrecto {
                        campo: campo.to_string(),
                        esperado: "número entero 0-65535".to_string(),
                        encontrado: v.clone(),
                    });
                    0
                }
            }
            None => 0, // Ya reportado
        }
    }

    /// Extrae u32 con validación de rango
    fn extraer_u32_rango(
        valores: &HashMap<String, String>,
        campo: &str,
        min: u32,
        max: u32,
        default: u32,
        errores: &mut Vec<ConfigError>,
    ) -> u32 {
        match valores.get(campo) {
            Some(v) => {
                // Primero intentar como i64 para detectar negativos
                match v.parse::<i64>() {
                    Ok(n) if n < 0 => {
                        errores.push(ConfigError::FueraDeRango {
                            campo: campo.to_string(),
                            valor: v.clone(),
                            rango: format!("{}-{}", min, max),
                        });
                        default
                    }
                    Ok(n) if (n as u32) < min || (n as u32) > max => {
                        errores.push(ConfigError::FueraDeRango {
                            campo: campo.to_string(),
                            valor: v.clone(),
                            rango: format!("{}-{}", min, max),
                        });
                        default
                    }
                    Ok(n) => n as u32,
                    Err(_) => {
                        errores.push(ConfigError::TipoIncorrecto {
                            campo: campo.to_string(),
                            esperado: "número entero".to_string(),
                            encontrado: v.clone(),
                        });
                        default
                    }
                }
            }
            None => default,
        }
    }

    /// Extrae u64 con default
    fn extraer_u64_default(
        valores: &HashMap<String, String>,
        campo: &str,
        default: u64,
    ) -> u64 {
        valores.get(campo)
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    /// Extrae bool con default
    fn extraer_bool_default(
        valores: &HashMap<String, String>,
        campo: &str,
        default: bool,
    ) -> bool {
        valores.get(campo)
            .map(|v| matches!(v.to_lowercase().as_str(), "true" | "1" | "yes" | "on"))
            .unwrap_or(default)
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Nombre:          {}", self.nombre)?;
        writeln!(f, "  Versión:         {}", self.version)?;
        writeln!(f, "  Host:            {}", self.host)?;
        writeln!(f, "  Puerto:          {}", self.puerto)?;
        writeln!(f, "  Max conexiones:  {}", self.max_conexiones)?;
        writeln!(f, "  Timeout:         {}ms", self.timeout_ms)?;
        writeln!(f, "  Debug:           {}", self.debug)
    }
}

// ============================================================================
// TESTS
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    fn config_valida() -> &'static str {
        "nombre = TestApp\npuerto = 3000\n"
    }

    fn config_completa() -> &'static str {
        r#"nombre = TestApp
version = 2.0.0
puerto = 8080
host = 0.0.0.0
max_conexiones = 500
timeout_ms = 10000
debug = true
"#
    }

    #[test]
    fn test_cargar_config_minima() {
        fs::write("test_min.config", config_valida()).unwrap();
        let config = Config::cargar("test_min.config").unwrap();
        assert_eq!(config.nombre, "TestApp");
        assert_eq!(config.puerto, 3000);
        assert_eq!(config.host, "localhost"); // default
        fs::remove_file("test_min.config").unwrap();
    }

    #[test]
    fn test_cargar_config_completa() {
        fs::write("test_full.config", config_completa()).unwrap();
        let config = Config::cargar("test_full.config").unwrap();
        assert_eq!(config.nombre, "TestApp");
        assert_eq!(config.version, "2.0.0");
        assert_eq!(config.puerto, 8080);
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.max_conexiones, 500);
        assert_eq!(config.timeout_ms, 10000);
        assert!(config.debug);
        fs::remove_file("test_full.config").unwrap();
    }

    #[test]
    fn test_archivo_no_existe() {
        let result = Config::cargar("no_existe_xyz.config");
        assert!(result.is_err());
        let errores = result.unwrap_err();
        assert!(matches!(errores[0], ConfigError::IoError(_)));
    }

    #[test]
    fn test_campo_faltante() {
        fs::write("test_missing.config", "host = localhost\n").unwrap();
        let result = Config::cargar("test_missing.config");
        assert!(result.is_err());
        let errores = result.unwrap_err();
        assert!(errores.iter().any(|e| matches!(e, ConfigError::CampoFaltante(c) if c == "nombre")));
        fs::remove_file("test_missing.config").unwrap();
    }

    #[test]
    fn test_valor_vacio() {
        fs::write("test_empty.config", "nombre = \npuerto = 8080\n").unwrap();
        let result = Config::cargar("test_empty.config");
        assert!(result.is_err());
        let errores = result.unwrap_err();
        assert!(errores.iter().any(|e| matches!(e, ConfigError::ValorVacio(_))));
        fs::remove_file("test_empty.config").unwrap();
    }

    #[test]
    fn test_tipo_incorrecto() {
        fs::write("test_type.config", "nombre = App\npuerto = abc\n").unwrap();
        let result = Config::cargar("test_type.config");
        assert!(result.is_err());
        let errores = result.unwrap_err();
        assert!(errores.iter().any(|e| matches!(e, ConfigError::TipoIncorrecto { .. })));
        fs::remove_file("test_type.config").unwrap();
    }

    #[test]
    fn test_fuera_de_rango() {
        fs::write("test_range.config", "nombre = App\npuerto = 8080\nmax_conexiones = -10\n").unwrap();
        let result = Config::cargar("test_range.config");
        assert!(result.is_err());
        let errores = result.unwrap_err();
        assert!(errores.iter().any(|e| matches!(e, ConfigError::FueraDeRango { .. })));
        fs::remove_file("test_range.config").unwrap();
    }

    #[test]
    fn test_multiples_errores() {
        fs::write("test_multi.config", "puerto = abc\nmax_conexiones = -5\n").unwrap();
        let result = Config::cargar("test_multi.config");
        assert!(result.is_err());
        let errores = result.unwrap_err();
        assert!(errores.len() >= 2); // Al menos nombre faltante y puerto inválido
        fs::remove_file("test_multi.config").unwrap();
    }

    #[test]
    fn test_comentarios_ignorados() {
        fs::write("test_comments.config", "# Comentario\nnombre = App\n# Otro\npuerto = 80\n").unwrap();
        let config = Config::cargar("test_comments.config").unwrap();
        assert_eq!(config.nombre, "App");
        fs::remove_file("test_comments.config").unwrap();
    }

    #[test]
    fn test_formato_invalido() {
        fs::write("test_format.config", "nombre = App\nlinea_sin_igual\npuerto = 80\n").unwrap();
        let result = Config::cargar("test_format.config");
        assert!(result.is_err());
        let errores = result.unwrap_err();
        assert!(errores.iter().any(|e| matches!(e, ConfigError::FormatoInvalido { .. })));
        fs::remove_file("test_format.config").unwrap();
    }
}
