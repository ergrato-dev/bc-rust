//! # Práctica 02: Strings
//!
//! Ejercicios para dominar String y &str en Rust.

fn main() {
    println!("=== Práctica 02: Strings ===\n");

    // Ejercicio 1: Conversiones
    demo_conversiones();

    // Ejercicio 2: Manipulación
    demo_manipulacion();

    // Ejercicio 3: Parsing
    demo_parsing();

    // Ejercicio 4: Procesador de texto
    demo_procesador();
}

// ============================================================================
// EJERCICIO 1: Conversiones
// ============================================================================

/// Convierte &str a String
fn a_string(s: &str) -> String {
    s.to_string()
}

/// Une múltiples &str en un String con separador
fn unir(partes: &[&str], separador: &str) -> String {
    partes.join(separador)
}

/// Repite un string n veces
fn repetir(s: &str, n: usize) -> String {
    s.repeat(n)
}

/// Crea un String con formato
fn formatear_saludo(nombre: &str, edad: u32) -> String {
    format!("Hola, {}! Tienes {} años.", nombre, edad)
}

/// Convierte un vector de chars a String
fn chars_a_string(chars: &[char]) -> String {
    chars.iter().collect()
}

fn demo_conversiones() {
    println!("--- Ejercicio 1: Conversiones ---");

    let s = a_string("Hola");
    println!("&str a String: '{}'", s);

    let partes = vec!["Rust", "es", "genial"];
    let unido = unir(&partes, " ");
    println!("Unir: '{}'", unido);

    let repetido = repetir("🦀", 3);
    println!("Repetir: '{}'", repetido);

    let saludo = formatear_saludo("Carlos", 25);
    println!("Formato: '{}'", saludo);

    let chars = vec!['H', 'o', 'l', 'a'];
    let desde_chars = chars_a_string(&chars);
    println!("Desde chars: '{}'", desde_chars);

    println!();
}

// ============================================================================
// EJERCICIO 2: Manipulación
// ============================================================================

/// Convierte a mayúsculas
fn a_mayusculas(s: &str) -> String {
    s.to_uppercase()
}

/// Convierte a minúsculas
fn a_minusculas(s: &str) -> String {
    s.to_lowercase()
}

/// Capitaliza la primera letra de cada palabra
fn capitalizar_palabras(s: &str) -> String {
    s.split_whitespace()
        .map(|palabra| {
            let mut chars = palabra.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Invierte un string (respetando UTF-8)
fn invertir(s: &str) -> String {
    s.chars().rev().collect()
}

/// Elimina espacios extra (múltiples espacios → uno solo)
fn normalizar_espacios(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Trunca un string a n caracteres, agregando "..." si se truncó
fn truncar(s: &str, max_chars: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max_chars {
        s.to_string()
    } else {
        let truncado: String = chars[..max_chars].iter().collect();
        format!("{}...", truncado)
    }
}

/// Reemplaza todas las vocales por un carácter
fn reemplazar_vocales(s: &str, reemplazo: char) -> String {
    s.chars()
        .map(|c| {
            if "aeiouAEIOU".contains(c) {
                reemplazo
            } else {
                c
            }
        })
        .collect()
}

fn demo_manipulacion() {
    println!("--- Ejercicio 2: Manipulación ---");

    let texto = "hola mundo";

    println!("Original: '{}'", texto);
    println!("Mayúsculas: '{}'", a_mayusculas(texto));
    println!("Minúsculas: '{}'", a_minusculas("HOLA MUNDO"));
    println!("Capitalizar: '{}'", capitalizar_palabras("rust es genial"));
    println!("Invertir: '{}'", invertir("Hola 🦀"));
    println!(
        "Normalizar: '{}'",
        normalizar_espacios("  muchos   espacios   aquí  ")
    );
    println!("Truncar: '{}'", truncar("Este texto es muy largo", 10));
    println!("Sin vocales: '{}'", reemplazar_vocales("Hola Mundo", '*'));

    println!();
}

// ============================================================================
// EJERCICIO 3: Parsing
// ============================================================================

/// Valida si un string es un email básico
fn es_email_valido(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() {
        return false;
    }

    // Debe tener exactamente un @
    let partes: Vec<&str> = s.split('@').collect();
    if partes.len() != 2 {
        return false;
    }

    let (usuario, dominio) = (partes[0], partes[1]);

    // Usuario no vacío
    if usuario.is_empty() {
        return false;
    }

    // Dominio debe tener al menos un punto
    if !dominio.contains('.') {
        return false;
    }

    // Dominio no empieza ni termina con punto
    if dominio.starts_with('.') || dominio.ends_with('.') {
        return false;
    }

    true
}

/// Extrae el dominio de un email
fn extraer_dominio(email: &str) -> Option<String> {
    if !es_email_valido(email) {
        return None;
    }
    email.split('@').nth(1).map(|s| s.to_string())
}

/// Parsea un string "clave=valor" a tupla
fn parsear_par_clave_valor(s: &str) -> Option<(String, String)> {
    let partes: Vec<&str> = s.splitn(2, '=').collect();
    if partes.len() == 2 {
        Some((partes[0].trim().to_string(), partes[1].trim().to_string()))
    } else {
        None
    }
}

/// Extrae números de un string
fn extraer_numeros(s: &str) -> Vec<i32> {
    s.split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter_map(|part| part.parse().ok())
        .collect()
}

/// Valida un número de teléfono (formato: XXX-XXX-XXXX)
fn es_telefono_valido(s: &str) -> bool {
    let partes: Vec<&str> = s.split('-').collect();
    if partes.len() != 3 {
        return false;
    }

    partes[0].len() == 3
        && partes[1].len() == 3
        && partes[2].len() == 4
        && partes.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

/// Parsea una fecha simple (DD/MM/YYYY)
fn parsear_fecha(s: &str) -> Option<(u32, u32, u32)> {
    let partes: Vec<&str> = s.split('/').collect();
    if partes.len() != 3 {
        return None;
    }

    let dia: u32 = partes[0].parse().ok()?;
    let mes: u32 = partes[1].parse().ok()?;
    let anio: u32 = partes[2].parse().ok()?;

    if dia >= 1 && dia <= 31 && mes >= 1 && mes <= 12 {
        Some((dia, mes, anio))
    } else {
        None
    }
}

fn demo_parsing() {
    println!("--- Ejercicio 3: Parsing ---");

    let emails = vec![
        "usuario@ejemplo.com",
        "invalido",
        "@sinusuario.com",
        "sin@dominio",
    ];

    println!("Validación de emails:");
    for email in emails {
        let valido = es_email_valido(email);
        let simbolo = if valido { "✅" } else { "❌" };
        println!("  {} {} ", simbolo, email);
    }

    println!("\nExtracción de dominio:");
    println!(
        "  usuario@ejemplo.com → {:?}",
        extraer_dominio("usuario@ejemplo.com")
    );

    println!("\nParseo clave=valor:");
    println!(
        "  nombre=Juan → {:?}",
        parsear_par_clave_valor("nombre=Juan")
    );
    println!(
        "  url=https://rust-lang.org → {:?}",
        parsear_par_clave_valor("url=https://rust-lang.org")
    );

    println!("\nExtracción de números:");
    println!(
        "  'Tengo 3 manzanas y 5 peras' → {:?}",
        extraer_numeros("Tengo 3 manzanas y 5 peras")
    );

    println!("\nValidación de teléfono:");
    println!("  123-456-7890 → {}", es_telefono_valido("123-456-7890"));
    println!("  12-34-5678 → {}", es_telefono_valido("12-34-5678"));

    println!("\nParseo de fecha:");
    println!("  25/12/2024 → {:?}", parsear_fecha("25/12/2024"));
    println!("  32/13/2024 → {:?}", parsear_fecha("32/13/2024"));

    println!();
}

// ============================================================================
// EJERCICIO 4: Procesador de Texto
// ============================================================================

#[derive(Debug)]
struct EstadisticasTexto {
    caracteres: usize,
    caracteres_sin_espacios: usize,
    palabras: usize,
    lineas: usize,
    oraciones: usize,
    palabra_mas_larga: String,
}

/// Cuenta las palabras en un texto
fn contar_palabras(texto: &str) -> usize {
    texto.split_whitespace().count()
}

/// Cuenta las oraciones (terminan en . ! ?)
fn contar_oraciones(texto: &str) -> usize {
    texto.chars().filter(|c| ".!?".contains(*c)).count()
}

/// Encuentra la palabra más larga
fn palabra_mas_larga(texto: &str) -> String {
    texto
        .split_whitespace()
        .map(|p| p.trim_matches(|c: char| !c.is_alphabetic()))
        .max_by_key(|p| p.chars().count())
        .unwrap_or("")
        .to_string()
}

/// Cuenta la frecuencia de cada palabra
fn frecuencia_palabras(texto: &str) -> Vec<(String, usize)> {
    use std::collections::HashMap;

    let mut frecuencia: HashMap<String, usize> = HashMap::new();

    for palabra in texto.split_whitespace() {
        let palabra_limpia = palabra
            .trim_matches(|c: char| !c.is_alphabetic())
            .to_lowercase();

        if !palabra_limpia.is_empty() {
            *frecuencia.entry(palabra_limpia).or_insert(0) += 1;
        }
    }

    let mut resultado: Vec<_> = frecuencia.into_iter().collect();
    resultado.sort_by(|a, b| b.1.cmp(&a.1));
    resultado
}

/// Analiza un texto y devuelve estadísticas
fn analizar_texto(texto: &str) -> EstadisticasTexto {
    EstadisticasTexto {
        caracteres: texto.chars().count(),
        caracteres_sin_espacios: texto.chars().filter(|c| !c.is_whitespace()).count(),
        palabras: contar_palabras(texto),
        lineas: texto.lines().count(),
        oraciones: contar_oraciones(texto),
        palabra_mas_larga: palabra_mas_larga(texto),
    }
}

/// Genera un resumen del texto (primeras n palabras)
fn generar_resumen(texto: &str, max_palabras: usize) -> String {
    let palabras: Vec<&str> = texto.split_whitespace().take(max_palabras).collect();
    if palabras.len() < max_palabras {
        palabras.join(" ")
    } else {
        format!("{}...", palabras.join(" "))
    }
}

fn demo_procesador() {
    println!("--- Ejercicio 4: Procesador de Texto ---");

    let texto = "Rust es un lenguaje de programación moderno. \
                 Es seguro, rápido y concurrente. \
                 Rust previene errores de memoria sin usar un recolector de basura. \
                 ¡Es fantástico!";

    println!("Texto:");
    println!("  {}", texto);

    let stats = analizar_texto(texto);
    println!("\nEstadísticas:");
    println!("  Caracteres: {}", stats.caracteres);
    println!(
        "  Caracteres (sin espacios): {}",
        stats.caracteres_sin_espacios
    );
    println!("  Palabras: {}", stats.palabras);
    println!("  Líneas: {}", stats.lineas);
    println!("  Oraciones: {}", stats.oraciones);
    println!("  Palabra más larga: '{}'", stats.palabra_mas_larga);

    println!("\nFrecuencia de palabras (top 5):");
    for (palabra, count) in frecuencia_palabras(texto).iter().take(5) {
        println!("  '{}': {}", palabra, count);
    }

    println!("\nResumen (10 palabras):");
    println!("  {}", generar_resumen(texto, 10));
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Tests Ejercicio 1
    #[test]
    fn test_a_string() {
        assert_eq!(a_string("hola"), "hola".to_string());
    }

    #[test]
    fn test_unir() {
        assert_eq!(unir(&["a", "b", "c"], "-"), "a-b-c");
        assert_eq!(unir(&["solo"], ","), "solo");
        assert_eq!(unir(&[], ","), "");
    }

    #[test]
    fn test_repetir() {
        assert_eq!(repetir("ab", 3), "ababab");
        assert_eq!(repetir("🦀", 2), "🦀🦀");
    }

    #[test]
    fn test_formatear_saludo() {
        assert_eq!(
            formatear_saludo("Ana", 30),
            "Hola, Ana! Tienes 30 años."
        );
    }

    #[test]
    fn test_chars_a_string() {
        assert_eq!(chars_a_string(&['H', 'i']), "Hi");
    }

    // Tests Ejercicio 2
    #[test]
    fn test_a_mayusculas() {
        assert_eq!(a_mayusculas("hola"), "HOLA");
    }

    #[test]
    fn test_capitalizar_palabras() {
        assert_eq!(capitalizar_palabras("rust es genial"), "Rust Es Genial");
    }

    #[test]
    fn test_invertir() {
        assert_eq!(invertir("hola"), "aloh");
        assert_eq!(invertir("🦀ab"), "ba🦀");
    }

    #[test]
    fn test_normalizar_espacios() {
        assert_eq!(normalizar_espacios("  a   b  c  "), "a b c");
    }

    #[test]
    fn test_truncar() {
        assert_eq!(truncar("hola mundo", 4), "hola...");
        assert_eq!(truncar("hi", 10), "hi");
    }

    #[test]
    fn test_reemplazar_vocales() {
        assert_eq!(reemplazar_vocales("Hola", '*'), "H*l*");
    }

    // Tests Ejercicio 3
    #[test]
    fn test_es_email_valido() {
        assert!(es_email_valido("user@example.com"));
        assert!(es_email_valido("a.b@c.d.e"));
        assert!(!es_email_valido("invalido"));
        assert!(!es_email_valido("@dominio.com"));
        assert!(!es_email_valido("user@"));
        assert!(!es_email_valido("user@dominio"));
    }

    #[test]
    fn test_extraer_dominio() {
        assert_eq!(
            extraer_dominio("user@example.com"),
            Some("example.com".to_string())
        );
        assert_eq!(extraer_dominio("invalido"), None);
    }

    #[test]
    fn test_parsear_par_clave_valor() {
        assert_eq!(
            parsear_par_clave_valor("nombre=Juan"),
            Some(("nombre".to_string(), "Juan".to_string()))
        );
        assert_eq!(
            parsear_par_clave_valor("url=https://rust-lang.org"),
            Some(("url".to_string(), "https://rust-lang.org".to_string()))
        );
        assert_eq!(parsear_par_clave_valor("sinigual"), None);
    }

    #[test]
    fn test_extraer_numeros() {
        assert_eq!(extraer_numeros("tengo 3 gatos y 5 perros"), vec![3, 5]);
        assert_eq!(extraer_numeros("sin numeros"), Vec::<i32>::new());
    }

    #[test]
    fn test_es_telefono_valido() {
        assert!(es_telefono_valido("123-456-7890"));
        assert!(!es_telefono_valido("12-34-5678"));
        assert!(!es_telefono_valido("1234567890"));
    }

    #[test]
    fn test_parsear_fecha() {
        assert_eq!(parsear_fecha("25/12/2024"), Some((25, 12, 2024)));
        assert_eq!(parsear_fecha("01/01/2000"), Some((1, 1, 2000)));
        assert_eq!(parsear_fecha("32/01/2024"), None);
        assert_eq!(parsear_fecha("01/13/2024"), None);
    }

    // Tests Ejercicio 4
    #[test]
    fn test_contar_palabras() {
        assert_eq!(contar_palabras("una dos tres"), 3);
        assert_eq!(contar_palabras(""), 0);
    }

    #[test]
    fn test_contar_oraciones() {
        assert_eq!(contar_oraciones("Hola. ¿Qué tal? ¡Bien!"), 3);
    }

    #[test]
    fn test_palabra_mas_larga() {
        assert_eq!(palabra_mas_larga("el extraordinario caso"), "extraordinario");
    }

    #[test]
    fn test_generar_resumen() {
        assert_eq!(
            generar_resumen("uno dos tres cuatro cinco", 3),
            "uno dos tres..."
        );
        assert_eq!(generar_resumen("uno dos", 5), "uno dos");
    }

    #[test]
    fn test_frecuencia_palabras() {
        let freq = frecuencia_palabras("hola hola mundo");
        assert_eq!(freq[0], ("hola".to_string(), 2));
    }
}
