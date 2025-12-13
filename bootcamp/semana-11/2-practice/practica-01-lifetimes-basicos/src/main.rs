//! Práctica 01: Lifetimes Básicos en Funciones
//!
//! En esta práctica aprenderás a:
//! - Anotar lifetimes en funciones simples
//! - Relacionar lifetimes de entrada con salida
//! - Entender cuándo Rust requiere anotaciones explícitas

fn main() {
    println!("=== Práctica 01: Lifetimes Básicos ===\n");

    // Ejercicio 1: longest
    let s1 = String::from("Rust es genial");
    let s2 = String::from("Ownership");
    let resultado = longest(&s1, &s2);
    println!("Ejercicio 1 - El más largo: {}", resultado);

    // Ejercicio 2: first_word
    let texto = String::from("Hola mundo Rust");
    let primera = first_word(&texto);
    println!("Ejercicio 2 - Primera palabra: {}", primera);

    // Ejercicio 3: get_prefix
    let data = String::from("prefijo:valor");
    let prefix = get_prefix(&data, ':');
    println!("Ejercicio 3 - Prefijo: {}", prefix);

    // Ejercicio 4: pick_one
    let a = String::from("opción A");
    let b = String::from("opción B");
    let elegido = pick_one(&a, &b, true);
    println!("Ejercicio 4 - Elegido: {}", elegido);

    // Ejercicio 5: skip_prefix
    let linea = String::from("   espacios al inicio");
    let sin_espacios = skip_prefix(&linea, ' ');
    println!("Ejercicio 5 - Sin espacios: '{}'", sin_espacios);

    println!("\n✅ Todos los ejercicios completados!");
}

// ============================================================
// EJERCICIO 1: longest
// ============================================================
// Completa la firma de la función para que compile.
// Esta función retorna la cadena más larga de las dos.
//
// TODO: Agrega las anotaciones de lifetime necesarias
// Pista: Ambos parámetros pueden ser retornados
// ============================================================

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // TODO: Implementa la lógica para retornar el más largo
    // Pista: Compara x.len() con y.len()
    todo!("Implementa longest")
}

// ============================================================
// EJERCICIO 2: first_word
// ============================================================
// Retorna la primera palabra de una cadena (hasta el primer espacio).
// Si no hay espacios, retorna la cadena completa.
//
// TODO: Anota los lifetimes si es necesario
// Pista: ¿Cuántos parámetros de referencia hay?
// ============================================================

fn first_word(s: &str) -> &str {
    // TODO: ¿Necesita lifetime explícito?
    match s.find(' ') {
        Some(pos) => &s[..pos],
        None => s,
    }
}

// ============================================================
// EJERCICIO 3: get_prefix
// ============================================================
// Retorna todo el texto antes del delimitador especificado.
// Si no encuentra el delimitador, retorna la cadena completa.
//
// TODO: Anota los lifetimes correctamente
// Pista: El delimitador (char) no es una referencia
// ============================================================

fn get_prefix(text: &str, delimiter: char) -> &str {
    // TODO: Revisa si necesita lifetime explícito
    match text.find(delimiter) {
        Some(pos) => &text[..pos],
        None => text,
    }
}

// ============================================================
// EJERCICIO 4: pick_one
// ============================================================
// Retorna una de las dos cadenas basándose en el flag.
//
// TODO: Agrega las anotaciones de lifetime
// Pista: Similar a longest - ambas pueden retornarse
// ============================================================

fn pick_one<'a>(a: &'a str, b: &'a str, pick_first: bool) -> &'a str {
    // TODO: Implementa la lógica
    // Pista: Si pick_first es true, retorna a, sino b
    todo!("Implementa pick_one")
}

// ============================================================
// EJERCICIO 5: skip_prefix
// ============================================================
// Salta todos los caracteres iniciales que coincidan con 'skip'.
// Por ejemplo: skip_prefix("   hola", ' ') -> "hola"
//
// TODO: Anota los lifetimes si es necesario
// ============================================================

fn skip_prefix(s: &str, skip: char) -> &str {
    // TODO: ¿Necesita lifetime explícito?
    s.trim_start_matches(skip)
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_first() {
        let s1 = "largo texto aquí";
        let s2 = "corto";
        assert_eq!(longest(s1, s2), "largo texto aquí");
    }

    #[test]
    fn test_longest_second() {
        let s1 = "ab";
        let s2 = "abcdef";
        assert_eq!(longest(s1, s2), "abcdef");
    }

    #[test]
    fn test_longest_equal() {
        let s1 = "igual";
        let s2 = "cinco";
        // Cuando son iguales, retorna el segundo
        assert_eq!(longest(s1, s2), "cinco");
    }

    #[test]
    fn test_first_word_with_space() {
        assert_eq!(first_word("hola mundo"), "hola");
    }

    #[test]
    fn test_first_word_no_space() {
        assert_eq!(first_word("palabra"), "palabra");
    }

    #[test]
    fn test_first_word_multiple_spaces() {
        assert_eq!(first_word("uno dos tres"), "uno");
    }

    #[test]
    fn test_get_prefix_found() {
        assert_eq!(get_prefix("clave:valor", ':'), "clave");
    }

    #[test]
    fn test_get_prefix_not_found() {
        assert_eq!(get_prefix("sin delimitador", ':'), "sin delimitador");
    }

    #[test]
    fn test_get_prefix_at_start() {
        assert_eq!(get_prefix(":valor", ':'), "");
    }

    #[test]
    fn test_pick_one_first() {
        assert_eq!(pick_one("A", "B", true), "A");
    }

    #[test]
    fn test_pick_one_second() {
        assert_eq!(pick_one("A", "B", false), "B");
    }

    #[test]
    fn test_skip_prefix_spaces() {
        assert_eq!(skip_prefix("   texto", ' '), "texto");
    }

    #[test]
    fn test_skip_prefix_no_match() {
        assert_eq!(skip_prefix("texto", ' '), "texto");
    }

    #[test]
    fn test_skip_prefix_all() {
        assert_eq!(skip_prefix("###", '#'), "");
    }
}
