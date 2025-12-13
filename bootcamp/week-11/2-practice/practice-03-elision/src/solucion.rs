//! Soluciones de la Práctica 03: Reglas de Elision

fn main() {
    println!("=== Soluciones Práctica 03: Elision ===\n");

    let text = "hola mundo rust";
    println!("Ejercicio 1:");
    println!("  trim: '{}'", simple_trim(text));
    println!("  upper first: '{}'", first_char_str(text));

    let processor = TextProcessor::new("  Texto de prueba  ");
    println!("\nEjercicio 2:");
    println!("  trimmed: '{}'", processor.trimmed());
    println!("  first word: '{}'", processor.first_word());

    let a = "corto";
    let b = "más largo";
    println!("\nEjercicio 3:");
    println!("  min_by_len: '{}'", min_by_len(a, b));
    println!("  either: '{}'", either(a, b, true));

    let data = "key:value";
    println!("\nEjercicio 4:");
    println!("  before_colon: '{}'", before_colon(data));
    println!("  after_char: '{}'", after_char(data, ':'));

    println!("\nEjercicio 5: Análisis de Reglas");
    analyze_elision_rules();

    println!("\n✅ Todas las soluciones funcionan!");
}

// ============================================================
// SOLUCIÓN 1: Funciones simples (Regla 2)
// ============================================================
// NO necesitan lifetime explícito porque:
// - Solo hay UNA referencia de entrada
// - Regla 2: "Si hay exactamente un lifetime de entrada,
//            se aplica a todas las referencias de salida"
// ============================================================

fn simple_trim(s: &str) -> &str {
    // Regla 2 aplica: fn simple_trim<'a>(s: &'a str) -> &'a str
    s.trim()
}

fn first_char_str(s: &str) -> &str {
    // Regla 2 aplica: fn first_char_str<'a>(s: &'a str) -> &'a str
    if s.is_empty() {
        ""  // "" es &'static str, compatible con cualquier lifetime
    } else {
        &s[..s.chars().next().unwrap().len_utf8()]
    }
}

// ============================================================
// SOLUCIÓN 2: Métodos con &self (Regla 3)
// ============================================================

struct TextProcessor<'a> {
    content: &'a str,
}

impl<'a> TextProcessor<'a> {
    fn new(content: &'a str) -> TextProcessor<'a> {
        TextProcessor { content }
    }

    // REGLA 3: &self tiene lifetime implícito que se aplica al retorno
    // Expandido: fn trimmed<'b>(&'b self) -> &'b str
    // Pero el retorno realmente tiene lifetime 'a del contenido
    fn trimmed(&self) -> &str {
        self.content.trim()
    }

    // REGLA 3 aplica igual
    fn first_word(&self) -> &str {
        self.content
            .split_whitespace()
            .next()
            .unwrap_or("")
    }

    // REGLA 3: Aunque hay otro &str (default), el retorno usa
    // lifetime de &self automáticamente.
    // Expandido: fn or_default<'b, 'c>(&'b self, default: &'c str) -> &'b str
    fn or_default(&self, _default: &str) -> &str {
        // IMPORTANTE: Si quisiéramos retornar `default`,
        // necesitaríamos anotación explícita diferente
        self.content
    }
}

// ============================================================
// SOLUCIÓN 3: Funciones que REQUIEREN anotación
// ============================================================
// Estas funciones NECESITAN lifetime porque:
// - Hay DOS referencias de entrada
// - Cualquiera puede ser retornada
// - Regla 2 no aplica (hay más de un input lifetime)
// ============================================================

fn min_by_len<'a>(a: &'a str, b: &'a str) -> &'a str {
    // Al usar 'a para ambos, decimos: "el retorno vive tanto
    // como el más corto de a y b"
    if a.len() <= b.len() {
        a
    } else {
        b
    }
}

fn either<'a>(a: &'a str, b: &'a str, pick_a: bool) -> &'a str {
    if pick_a { a } else { b }
}

// ============================================================
// SOLUCIÓN 4: Casos mixtos
// ============================================================

// NO necesita lifetime explícito
// Solo hay UNA referencia de entrada (s)
// Regla 2 aplica
fn before_colon(s: &str) -> &str {
    s.split(':').next().unwrap_or(s)
}

// NO necesita lifetime explícito
// c es char (valor), no referencia
// Solo s es referencia → Regla 2 aplica
fn after_char(s: &str, c: char) -> &str {
    match s.find(c) {
        Some(pos) => &s[pos + c.len_utf8()..],
        None => "",
    }
}

// ============================================================
// SOLUCIÓN 5: Análisis de Reglas
// ============================================================

fn analyze_elision_rules() {
    println!("  Caso A: fn f(x: &str) -> &str");
    println!("    Regla: 2 (un input → va al output)");
    println!("    ¿Anotación?: NO");
    println!();

    println!("  Caso B: fn f(x: &str, y: &str) -> &str");
    println!("    Regla: Ninguna aplica completamente");
    println!("    ¿Anotación?: SÍ (ambos pueden retornarse)");
    println!();

    println!("  Caso C: fn f(&self) -> &str");
    println!("    Regla: 3 (self → output)");
    println!("    ¿Anotación?: NO");
    println!();

    println!("  Caso D: fn f(&self, x: &str) -> &str");
    println!("    Regla: 3 (self → output)");
    println!("    ¿Anotación?: NO (pero si quieres retornar x, SÍ)");
    println!();

    println!("  Caso E: fn f(x: &str, n: usize) -> &str");
    println!("    Regla: 2 (n no es ref, solo x cuenta)");
    println!("    ¿Anotación?: NO");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(simple_trim("  x  "), "x");
        assert_eq!(first_char_str("ab"), "a");
        assert_eq!(min_by_len("a", "abc"), "a");
        assert_eq!(either("A", "B", false), "B");
        assert_eq!(before_colon("k:v"), "k");
        assert_eq!(after_char("k:v", ':'), "v");
    }
}
