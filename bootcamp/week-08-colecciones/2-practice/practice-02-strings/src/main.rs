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
fn to_string_owned(s: &str) -> String {
    s.to_string()
}

/// Une múltiples &str en un String con separador
fn join_parts(parts: &[&str], separator: &str) -> String {
    parts.join(separator)
}

/// Repite un string n veces
fn repeat_string(s: &str, n: usize) -> String {
    s.repeat(n)
}

/// Crea un String con formato
fn format_greeting(name: &str, age: u32) -> String {
    format!("Hola, {}! Tienes {} años.", name, age)
}

/// Convierte un vector de chars a String
fn chars_to_string(chars: &[char]) -> String {
    chars.iter().collect()
}

fn demo_conversiones() {
    println!("--- Ejercicio 1: Conversiones ---");

    let s = to_string_owned("Hola");
    println!("&str a String: '{}'", s);

    let parts = vec!["Rust", "es", "genial"];
    let joined = join_parts(&parts, " ");
    println!("Unir: '{}'", joined);

    let repeated = repeat_string("🦀", 3);
    println!("Repetir: '{}'", repeated);

    let greeting = format_greeting("Carlos", 25);
    println!("Formato: '{}'", greeting);

    let chars = vec!['H', 'o', 'l', 'a'];
    let from_chars = chars_to_string(&chars);
    println!("Desde chars: '{}'", from_chars);

    println!();
}

// ============================================================================
// EJERCICIO 2: Manipulación
// ============================================================================

/// Convierte a mayúsculas
fn to_uppercase_str(s: &str) -> String {
    s.to_uppercase()
}

/// Convierte a minúsculas
fn to_lowercase_str(s: &str) -> String {
    s.to_lowercase()
}

/// Capitaliza la primera letra de cada palabra
fn capitalize_words(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Invierte un string (respetando UTF-8)
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

/// Elimina espacios extra (múltiples espacios → uno solo)
fn normalize_spaces(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Trunca un string a n caracteres, agregando "..." si se truncó
fn truncate_string(s: &str, max_chars: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max_chars {
        s.to_string()
    } else {
        let truncated: String = chars[..max_chars].iter().collect();
        format!("{}...", truncated)
    }
}

/// Reemplaza todas las vocales por un carácter
fn replace_vowels(s: &str, replacement: char) -> String {
    s.chars()
        .map(|c| {
            if "aeiouAEIOU".contains(c) {
                replacement
            } else {
                c
            }
        })
        .collect()
}

fn demo_manipulacion() {
    println!("--- Ejercicio 2: Manipulación ---");

    let text = "hola mundo";

    println!("Original: '{}'", text);
    println!("Mayúsculas: '{}'", to_uppercase_str(text));
    println!("Minúsculas: '{}'", to_lowercase_str("HOLA MUNDO"));
    println!("Capitalizar: '{}'", capitalize_words("rust es genial"));
    println!("Invertir: '{}'", reverse_string("Hola 🦀"));
    println!(
        "Normalizar: '{}'",
        normalize_spaces("  muchos   espacios   aquí  ")
    );
    println!("Truncar: '{}'", truncate_string("Este texto es muy largo", 10));
    println!("Sin vocales: '{}'", replace_vowels("Hola Mundo", '*'));

    println!();
}

// ============================================================================
// EJERCICIO 3: Parsing
// ============================================================================

/// Valida si un string es un email básico
fn is_valid_email(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() {
        return false;
    }

    // Debe tener exactamente un @
    let parts: Vec<&str> = s.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    let (user, domain) = (parts[0], parts[1]);

    // Usuario no vacío
    if user.is_empty() {
        return false;
    }

    // Dominio debe tener al menos un punto
    if !domain.contains('.') {
        return false;
    }

    // Dominio no empieza ni termina con punto
    if domain.starts_with('.') || domain.ends_with('.') {
        return false;
    }

    true
}

/// Extrae el dominio de un email
fn extract_domain(email: &str) -> Option<String> {
    if !is_valid_email(email) {
        return None;
    }
    email.split('@').nth(1).map(|s| s.to_string())
}

/// Parsea un string "clave=valor" a tupla
fn parse_key_value_pair(s: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() == 2 {
        Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
    } else {
        None
    }
}

/// Extrae números de un string
fn extract_numbers(s: &str) -> Vec<i32> {
    s.split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter_map(|part| part.parse().ok())
        .collect()
}

/// Valida un número de teléfono (formato: XXX-XXX-XXXX)
fn is_valid_phone(s: &str) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return false;
    }

    parts[0].len() == 3
        && parts[1].len() == 3
        && parts[2].len() == 4
        && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

/// Parsea una fecha simple (DD/MM/YYYY)
fn parse_date(s: &str) -> Option<(u32, u32, u32)> {
    let parts: Vec<&str> = s.split('/').collect();
    if parts.len() != 3 {
        return None;
    }

    let day: u32 = parts[0].parse().ok()?;
    let month: u32 = parts[1].parse().ok()?;
    let year: u32 = parts[2].parse().ok()?;

    if day >= 1 && day <= 31 && month >= 1 && month <= 12 {
        Some((day, month, year))
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
        let valid = is_valid_email(email);
        let symbol = if valid { "✅" } else { "❌" };
        println!("  {} {} ", symbol, email);
    }

    println!("\nExtracción de dominio:");
    println!(
        "  usuario@ejemplo.com → {:?}",
        extract_domain("usuario@ejemplo.com")
    );

    println!("\nParseo clave=valor:");
    println!(
        "  nombre=Juan → {:?}",
        parse_key_value_pair("nombre=Juan")
    );
    println!(
        "  url=https://rust-lang.org → {:?}",
        parse_key_value_pair("url=https://rust-lang.org")
    );

    println!("\nExtracción de números:");
    println!(
        "  'Tengo 3 manzanas y 5 peras' → {:?}",
        extract_numbers("Tengo 3 manzanas y 5 peras")
    );

    println!("\nValidación de teléfono:");
    println!("  123-456-7890 → {}", is_valid_phone("123-456-7890"));
    println!("  12-34-5678 → {}", is_valid_phone("12-34-5678"));

    println!("\nParseo de fecha:");
    println!("  25/12/2024 → {:?}", parse_date("25/12/2024"));
    println!("  32/13/2024 → {:?}", parse_date("32/13/2024"));

    println!();
}

// ============================================================================
// EJERCICIO 4: Procesador de Texto
// ============================================================================

#[derive(Debug)]
struct TextStatistics {
    characters: usize,
    characters_without_spaces: usize,
    words: usize,
    lines: usize,
    sentences: usize,
    longest_word: String,
}

/// Cuenta las palabras en un texto
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Cuenta las oraciones (terminan en . ! ?)
fn count_sentences(text: &str) -> usize {
    text.chars().filter(|c| ".!?".contains(*c)).count()
}

/// Encuentra la palabra más larga
fn find_longest_word(text: &str) -> String {
    text
        .split_whitespace()
        .map(|p| p.trim_matches(|c: char| !c.is_alphabetic()))
        .max_by_key(|p| p.chars().count())
        .unwrap_or("")
        .to_string()
}

/// Cuenta la frecuencia de cada palabra
fn word_frequency(text: &str) -> Vec<(String, usize)> {
    use std::collections::HashMap;

    let mut frequency: HashMap<String, usize> = HashMap::new();

    for word in text.split_whitespace() {
        let clean_word = word
            .trim_matches(|c: char| !c.is_alphabetic())
            .to_lowercase();

        if !clean_word.is_empty() {
            *frequency.entry(clean_word).or_insert(0) += 1;
        }
    }

    let mut result: Vec<_> = frequency.into_iter().collect();
    result.sort_by(|a, b| b.1.cmp(&a.1));
    result
}

/// Analiza un texto y devuelve estadísticas
fn analyze_text(text: &str) -> TextStatistics {
    TextStatistics {
        characters: text.chars().count(),
        characters_without_spaces: text.chars().filter(|c| !c.is_whitespace()).count(),
        words: count_words(text),
        lines: text.lines().count(),
        sentences: count_sentences(text),
        longest_word: find_longest_word(text),
    }
}

/// Genera un resumen del texto (primeras n palabras)
fn generate_summary(text: &str, max_words: usize) -> String {
    let words: Vec<&str> = text.split_whitespace().take(max_words).collect();
    if words.len() < max_words {
        words.join(" ")
    } else {
        format!("{}...", words.join(" "))
    }
}

fn demo_procesador() {
    println!("--- Ejercicio 4: Procesador de Texto ---");

    let text = "Rust es un lenguaje de programación moderno. \
                 Es seguro, rápido y concurrente. \
                 Rust previene errores de memoria sin usar un recolector de basura. \
                 ¡Es fantástico!";

    println!("Texto:");
    println!("  {}", text);

    let stats = analyze_text(text);
    println!("\nEstadísticas:");
    println!("  Caracteres: {}", stats.characters);
    println!(
        "  Caracteres (sin espacios): {}",
        stats.characters_without_spaces
    );
    println!("  Palabras: {}", stats.words);
    println!("  Líneas: {}", stats.lines);
    println!("  Oraciones: {}", stats.sentences);
    println!("  Palabra más larga: '{}'", stats.longest_word);

    println!("\nFrecuencia de palabras (top 5):");
    for (word, count) in word_frequency(text).iter().take(5) {
        println!("  '{}': {}", word, count);
    }

    println!("\nResumen (10 palabras):");
    println!("  {}", generate_summary(text, 10));
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Tests Ejercicio 1
    #[test]
    fn test_to_string_owned() {
        assert_eq!(to_string_owned("hola"), "hola".to_string());
    }

    #[test]
    fn test_join_parts() {
        assert_eq!(join_parts(&["a", "b", "c"], "-"), "a-b-c");
        assert_eq!(join_parts(&["solo"], ","), "solo");
        assert_eq!(join_parts(&[], ","), "");
    }

    #[test]
    fn test_repeat_string() {
        assert_eq!(repeat_string("ab", 3), "ababab");
        assert_eq!(repeat_string("🦀", 2), "🦀🦀");
    }

    #[test]
    fn test_format_greeting() {
        assert_eq!(
            format_greeting("Ana", 30),
            "Hola, Ana! Tienes 30 años."
        );
    }

    #[test]
    fn test_chars_to_string() {
        assert_eq!(chars_to_string(&['H', 'i']), "Hi");
    }

    // Tests Ejercicio 2
    #[test]
    fn test_to_uppercase_str() {
        assert_eq!(to_uppercase_str("hola"), "HOLA");
    }

    #[test]
    fn test_capitalize_words() {
        assert_eq!(capitalize_words("rust es genial"), "Rust Es Genial");
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hola"), "aloh");
        assert_eq!(reverse_string("🦀ab"), "ba🦀");
    }

    #[test]
    fn test_normalize_spaces() {
        assert_eq!(normalize_spaces("  a   b  c  "), "a b c");
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("hola mundo", 4), "hola...");
        assert_eq!(truncate_string("hi", 10), "hi");
    }

    #[test]
    fn test_replace_vowels() {
        assert_eq!(replace_vowels("Hola", '*'), "H*l*");
    }

    // Tests Ejercicio 3
    #[test]
    fn test_is_valid_email() {
        assert!(is_valid_email("user@example.com"));
        assert!(is_valid_email("a.b@c.d.e"));
        assert!(!is_valid_email("invalido"));
        assert!(!is_valid_email("@dominio.com"));
        assert!(!is_valid_email("user@"));
        assert!(!is_valid_email("user@dominio"));
    }

    #[test]
    fn test_extract_domain() {
        assert_eq!(
            extract_domain("user@example.com"),
            Some("example.com".to_string())
        );
        assert_eq!(extract_domain("invalido"), None);
    }

    #[test]
    fn test_parse_key_value_pair() {
        assert_eq!(
            parse_key_value_pair("nombre=Juan"),
            Some(("nombre".to_string(), "Juan".to_string()))
        );
        assert_eq!(
            parse_key_value_pair("url=https://rust-lang.org"),
            Some(("url".to_string(), "https://rust-lang.org".to_string()))
        );
        assert_eq!(parse_key_value_pair("sinigual"), None);
    }

    #[test]
    fn test_extract_numbers() {
        assert_eq!(extract_numbers("tengo 3 gatos y 5 perros"), vec![3, 5]);
        assert_eq!(extract_numbers("sin numeros"), Vec::<i32>::new());
    }

    #[test]
    fn test_is_valid_phone() {
        assert!(is_valid_phone("123-456-7890"));
        assert!(!is_valid_phone("12-34-5678"));
        assert!(!is_valid_phone("1234567890"));
    }

    #[test]
    fn test_parse_date() {
        assert_eq!(parse_date("25/12/2024"), Some((25, 12, 2024)));
        assert_eq!(parse_date("01/01/2000"), Some((1, 1, 2000)));
        assert_eq!(parse_date("32/01/2024"), None);
        assert_eq!(parse_date("01/13/2024"), None);
    }

    // Tests Ejercicio 4
    #[test]
    fn test_count_words() {
        assert_eq!(count_words("una dos tres"), 3);
        assert_eq!(count_words(""), 0);
    }

    #[test]
    fn test_count_sentences() {
        assert_eq!(count_sentences("Hola. ¿Qué tal? ¡Bien!"), 3);
    }

    #[test]
    fn test_find_longest_word() {
        assert_eq!(find_longest_word("el extraordinario caso"), "extraordinario");
    }

    #[test]
    fn test_generate_summary() {
        assert_eq!(
            generate_summary("uno dos tres cuatro cinco", 3),
            "uno dos tres..."
        );
        assert_eq!(generate_summary("uno dos", 5), "uno dos");
    }

    #[test]
    fn test_word_frequency() {
        let freq = word_frequency("hola hola mundo");
        assert_eq!(freq[0], ("hola".to_string(), 2));
    }
}
