//! Práctica 03: Reglas de Elision de Lifetimes
//!
//! En esta práctica aprenderás a:
//! - Identificar cuándo las reglas de elision aplican
//! - Simplificar firmas de funciones omitiendo lifetimes innecesarios
//! - Reconocer casos donde la anotación es obligatoria

fn main() {
    println!("=== Práctica 03: Reglas de Elision ===\n");

    // Ejercicio 1: Funciones simples
    let text = "hola mundo rust";
    println!("Ejercicio 1:");
    println!("  trim: '{}'", simple_trim(text));
    println!("  upper first: '{}'", first_char_str(text));

    // Ejercicio 2: Métodos con &self
    let processor = TextProcessor::new("  Texto de prueba  ");
    println!("\nEjercicio 2:");
    println!("  trimmed: '{}'", processor.trimmed());
    println!("  first word: '{}'", processor.first_word());

    // Ejercicio 3: Funciones que requieren anotación
    let a = "corto";
    let b = "más largo";
    println!("\nEjercicio 3:");
    println!("  min_by_len: '{}'", min_by_len(a, b));
    println!("  either: '{}'", either(a, b, true));

    // Ejercicio 4: Casos mixtos
    let data = "key:value";
    println!("\nEjercicio 4:");
    println!("  before_colon: '{}'", before_colon(data));
    println!("  after_char: '{}'", after_char(data, ':'));

    // Ejercicio 5: Análisis de elision
    println!("\nEjercicio 5: Ver análisis en código");
    analyze_elision_rules();

    println!("\n✅ Todos los ejercicios completados!");
}

// ============================================================
// EJERCICIO 1: Funciones simples (Regla 2)
// ============================================================
// Estas funciones tienen UNA sola referencia de entrada.
// La Regla 2 de elision aplica automáticamente.
//
// TODO: Verifica que NO necesitan lifetimes explícitos
// ============================================================

/// Elimina espacios al inicio y final
fn simple_trim(s: &str) -> &str {
    s.trim()
}

/// Retorna el primer caracter como &str
fn first_char_str(s: &str) -> &str {
    if s.is_empty() {
        ""
    } else {
        &s[..s.chars().next().unwrap().len_utf8()]
    }
}

// ============================================================
// EJERCICIO 2: Métodos con &self (Regla 3)
// ============================================================
// En métodos, si hay &self, su lifetime se aplica al retorno.
//
// TODO: Identifica qué métodos usan Regla 3
// ============================================================

struct TextProcessor<'a> {
    content: &'a str,
}

impl<'a> TextProcessor<'a> {
    fn new(content: &'a str) -> TextProcessor<'a> {
        TextProcessor { content }
    }

    // ¿Qué regla aplica aquí?
    fn trimmed(&self) -> &str {
        self.content.trim()
    }

    // ¿Y aquí?
    fn first_word(&self) -> &str {
        self.content
            .split_whitespace()
            .next()
            .unwrap_or("")
    }

    // Este método tiene otro parámetro de referencia
    // ¿Qué lifetime tiene el retorno?
    fn or_default(&self, default: &str) -> &str {
        if self.content.is_empty() {
            // NOTA: Esto retorna con lifetime de &self, no de default
            // Por la Regla 3
            self.content
        } else {
            self.content
        }
    }
}

// ============================================================
// EJERCICIO 3: Funciones que REQUIEREN anotación
// ============================================================
// Cuando hay múltiples referencias de entrada que pueden retornarse,
// las reglas de elision NO pueden determinar el lifetime.
//
// TODO: Agrega los lifetimes necesarios
// ============================================================

/// Retorna la cadena más corta
fn min_by_len<'a>(a: &'a str, b: &'a str) -> &'a str {
    // TODO: Implementa - retorna la cadena más corta
    todo!("Implementa min_by_len")
}

/// Retorna a o b basándose en el flag
fn either<'a>(a: &'a str, b: &'a str, pick_a: bool) -> &'a str {
    // TODO: Implementa - si pick_a es true, retorna a
    todo!("Implementa either")
}

// ============================================================
// EJERCICIO 4: Casos mixtos
// ============================================================
// Analiza cada función: ¿necesita lifetime explícito?
// ============================================================

/// Retorna todo antes del primer ':'
fn before_colon(s: &str) -> &str {
    // TODO: ¿Necesita lifetime? ¿Por qué?
    s.split(':').next().unwrap_or(s)
}

/// Retorna todo después del caracter especificado
fn after_char(s: &str, c: char) -> &str {
    // TODO: ¿Necesita lifetime? ¿Por qué?
    // Pista: c es char, no &char
    match s.find(c) {
        Some(pos) => &s[pos + c.len_utf8()..],
        None => "",
    }
}

// ============================================================
// EJERCICIO 5: Análisis de Reglas
// ============================================================
// Para cada firma, indica qué regla aplica y si necesita
// anotación explícita.
// ============================================================

fn analyze_elision_rules() {
    // Caso A: fn f(x: &str) -> &str
    // Regla: __
    // ¿Necesita anotación?: __

    // Caso B: fn f(x: &str, y: &str) -> &str
    // Regla: __
    // ¿Necesita anotación?: __

    // Caso C: fn f(&self) -> &str
    // Regla: __
    // ¿Necesita anotación?: __

    // Caso D: fn f(&self, x: &str) -> &str
    // Regla: __
    // ¿Necesita anotación?: __

    // Caso E: fn f(x: &str, n: usize) -> &str
    // Regla: __
    // ¿Necesita anotación?: __

    println!("  Revisa los comentarios en analyze_elision_rules()");
    println!("  y completa el análisis de cada caso.");
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_trim() {
        assert_eq!(simple_trim("  hola  "), "hola");
        assert_eq!(simple_trim("sin espacios"), "sin espacios");
    }

    #[test]
    fn test_first_char_str() {
        assert_eq!(first_char_str("hola"), "h");
        assert_eq!(first_char_str(""), "");
        assert_eq!(first_char_str("ñ"), "ñ"); // UTF-8
    }

    #[test]
    fn test_processor_trimmed() {
        let p = TextProcessor::new("  texto  ");
        assert_eq!(p.trimmed(), "texto");
    }

    #[test]
    fn test_processor_first_word() {
        let p = TextProcessor::new("uno dos tres");
        assert_eq!(p.first_word(), "uno");
    }

    #[test]
    fn test_min_by_len() {
        assert_eq!(min_by_len("ab", "abcd"), "ab");
        assert_eq!(min_by_len("largo", "xy"), "xy");
    }

    #[test]
    fn test_either() {
        assert_eq!(either("A", "B", true), "A");
        assert_eq!(either("A", "B", false), "B");
    }

    #[test]
    fn test_before_colon() {
        assert_eq!(before_colon("key:value"), "key");
        assert_eq!(before_colon("no colon"), "no colon");
    }

    #[test]
    fn test_after_char() {
        assert_eq!(after_char("key:value", ':'), "value");
        assert_eq!(after_char("no match", ':'), "");
    }
}
