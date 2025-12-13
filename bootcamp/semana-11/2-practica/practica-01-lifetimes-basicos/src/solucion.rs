//! Soluciones de la Práctica 01: Lifetimes Básicos
//!
//! Este archivo contiene las soluciones completas con explicaciones.

fn main() {
    println!("=== Soluciones Práctica 01: Lifetimes Básicos ===\n");

    let s1 = String::from("Rust es genial");
    let s2 = String::from("Ownership");
    let resultado = longest(&s1, &s2);
    println!("Ejercicio 1 - El más largo: {}", resultado);

    let texto = String::from("Hola mundo Rust");
    let primera = first_word(&texto);
    println!("Ejercicio 2 - Primera palabra: {}", primera);

    let data = String::from("prefijo:valor");
    let prefix = get_prefix(&data, ':');
    println!("Ejercicio 3 - Prefijo: {}", prefix);

    let a = String::from("opción A");
    let b = String::from("opción B");
    let elegido = pick_one(&a, &b, true);
    println!("Ejercicio 4 - Elegido: {}", elegido);

    let linea = String::from("   espacios al inicio");
    let sin_espacios = skip_prefix(&linea, ' ');
    println!("Ejercicio 5 - Sin espacios: '{}'", sin_espacios);

    println!("\n✅ Todas las soluciones funcionan!");
}

// ============================================================
// SOLUCIÓN 1: longest
// ============================================================
// Necesita lifetime explícito porque hay DOS referencias de entrada
// y el compilador no sabe cuál determina el lifetime del retorno.
//
// Al usar el mismo 'a para ambos, indicamos que el retorno vive
// tanto como el MÁS CORTO de los dos inputs.
// ============================================================

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ============================================================
// SOLUCIÓN 2: first_word
// ============================================================
// NO necesita lifetime explícito gracias a la Regla de Elision #2:
// "Si hay exactamente una referencia de entrada, su lifetime
//  se aplica a todas las referencias de salida"
//
// El compilador infiere: fn first_word<'a>(s: &'a str) -> &'a str
// ============================================================

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(pos) => &s[..pos],
        None => s,
    }
}

// ============================================================
// SOLUCIÓN 3: get_prefix
// ============================================================
// NO necesita lifetime explícito.
// Aunque hay dos parámetros (text y delimiter), solo text es referencia.
// delimiter es un char (tipo por valor, no referencia).
//
// La Regla de Elision #2 aplica porque solo hay UNA referencia de entrada.
// ============================================================

fn get_prefix(text: &str, delimiter: char) -> &str {
    match text.find(delimiter) {
        Some(pos) => &text[..pos],
        None => text,
    }
}

// ============================================================
// SOLUCIÓN 4: pick_one
// ============================================================
// NECESITA lifetime explícito porque hay DOS referencias de entrada
// y cualquiera de ellas puede ser retornada.
//
// El compilador no puede determinar automáticamente el lifetime
// del retorno, igual que en longest.
// ============================================================

fn pick_one<'a>(a: &'a str, b: &'a str, pick_first: bool) -> &'a str {
    if pick_first {
        a
    } else {
        b
    }
}

// ============================================================
// SOLUCIÓN 5: skip_prefix
// ============================================================
// NO necesita lifetime explícito.
// Solo hay una referencia de entrada (s).
// skip es un char (valor, no referencia).
//
// Regla de Elision #2 aplica automáticamente.
// ============================================================

fn skip_prefix(s: &str, skip: char) -> &str {
    s.trim_start_matches(skip)
}

// ============================================================
// RESUMEN DE CUÁNDO NECESITAS LIFETIMES EXPLÍCITOS
// ============================================================
//
// ✅ NO necesitas anotación cuando:
//    - Solo hay UNA referencia de entrada (Regla 2)
//    - Hay &self o &mut self y quieres su lifetime (Regla 3)
//
// ❌ SÍ necesitas anotación cuando:
//    - Hay MÚLTIPLES referencias de entrada que pueden retornarse
//    - El retorno tiene un lifetime diferente a &self
//    - En declaraciones de struct (siempre)
//
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("largo", "xy"), "largo");
        assert_eq!(longest("ab", "abcdef"), "abcdef");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hola mundo"), "hola");
        assert_eq!(first_word("palabra"), "palabra");
    }

    #[test]
    fn test_get_prefix() {
        assert_eq!(get_prefix("clave:valor", ':'), "clave");
        assert_eq!(get_prefix("sin_delim", ':'), "sin_delim");
    }

    #[test]
    fn test_pick_one() {
        assert_eq!(pick_one("A", "B", true), "A");
        assert_eq!(pick_one("A", "B", false), "B");
    }

    #[test]
    fn test_skip_prefix() {
        assert_eq!(skip_prefix("   texto", ' '), "texto");
        assert_eq!(skip_prefix("texto", ' '), "texto");
    }
}
