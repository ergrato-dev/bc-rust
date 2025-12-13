//! Práctica 02: Lifetimes en Structs
//!
//! En esta práctica aprenderás a:
//! - Declarar structs que contienen referencias
//! - Implementar métodos en structs con lifetimes
//! - Entender la relación entre el struct y sus datos referenciados

fn main() {
    println!("=== Práctica 02: Lifetimes en Structs ===\n");

    // Ejercicio 1: Excerpt
    let novel = String::from("Llámame Ishmael. Hace algunos años...");
    let excerpt = Excerpt::new(&novel[..16]);
    println!("Ejercicio 1 - Excerpt: '{}'", excerpt.part());

    // Ejercicio 2: Config
    let key = String::from("database_url");
    let value = String::from("postgres://localhost/db");
    let config = Config::new(&key, &value);
    println!("Ejercicio 2 - Config: {} = {}", config.key(), config.value());

    // Ejercicio 3: LineIterator
    let text = "primera línea\nsegunda línea\ntercera línea";
    let mut iter = LineIterator::new(text);
    println!("Ejercicio 3 - Líneas:");
    while let Some(line) = iter.next_line() {
        println!("  - {}", line);
    }

    // Ejercicio 4: Split
    let data = "clave=valor";
    let split = Split::at_char(data, '=');
    println!(
        "Ejercicio 4 - Split: '{}' y '{}'",
        split.left(),
        split.right()
    );

    // Ejercicio 5: Article (mixed owned + ref)
    let category = String::from("Tecnología");
    let article = Article::new("Rust en 2025".to_string(), &category);
    println!("Ejercicio 5 - Article: {} [{}]", article.title(), article.category());

    println!("\n✅ Todos los ejercicios completados!");
}

// ============================================================
// EJERCICIO 1: Excerpt
// ============================================================
// Un struct simple que contiene una referencia a una porción de texto.
//
// TODO: Agrega el lifetime parameter al struct y al campo
// ============================================================

struct Excerpt<'a> {
    // TODO: Agrega 'a al struct y al campo part
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    // TODO: Agrega lifetime a impl y a los métodos si es necesario
    fn new(part: &'a str) -> Excerpt<'a> {
        Excerpt { part }
    }

    fn part(&self) -> &str {
        self.part
    }

    fn len(&self) -> usize {
        self.part.len()
    }
}

// ============================================================
// EJERCICIO 2: Config
// ============================================================
// Struct de configuración con clave y valor como referencias.
//
// TODO: Anota los lifetimes. ¿Usas uno o dos lifetimes?
// ============================================================

struct Config<'a> {
    // TODO: Agrega lifetimes
    key: &'a str,
    value: &'a str,
}

impl<'a> Config<'a> {
    // TODO: Implementa con los lifetimes correctos
    fn new(key: &'a str, value: &'a str) -> Config<'a> {
        Config { key, value }
    }

    fn key(&self) -> &str {
        self.key
    }

    fn value(&self) -> &str {
        self.value
    }
}

// ============================================================
// EJERCICIO 3: LineIterator
// ============================================================
// Un iterador manual sobre líneas de texto.
// Mantiene una referencia al texto original y la posición actual.
//
// TODO: Agrega el lifetime y completa la implementación
// ============================================================

struct LineIterator<'a> {
    // TODO: Agrega lifetime
    content: &'a str,
    position: usize,
}

impl<'a> LineIterator<'a> {
    // TODO: Agrega lifetime al impl
    fn new(content: &'a str) -> LineIterator<'a> {
        LineIterator {
            content,
            position: 0,
        }
    }

    fn next_line(&mut self) -> Option<&str> {
        if self.position >= self.content.len() {
            return None;
        }

        let remaining = &self.content[self.position..];
        match remaining.find('\n') {
            Some(pos) => {
                let line = &remaining[..pos];
                self.position += pos + 1;
                Some(line)
            }
            None => {
                self.position = self.content.len();
                Some(remaining)
            }
        }
    }
}

// ============================================================
// EJERCICIO 4: Split
// ============================================================
// Representa una cadena dividida en dos partes.
//
// TODO: Implementa el struct con lifetimes y métodos
// ============================================================

struct Split<'a> {
    // TODO: Agrega lifetime y campos
    left: &'a str,
    right: &'a str,
}

impl<'a> Split<'a> {
    // TODO: Implementa at_char que divide por un carácter
    fn at_char(s: &'a str, c: char) -> Split<'a> {
        match s.find(c) {
            Some(pos) => Split {
                left: &s[..pos],
                right: &s[pos + c.len_utf8()..],
            },
            None => Split { left: s, right: "" },
        }
    }

    fn left(&self) -> &str {
        self.left
    }

    fn right(&self) -> &str {
        self.right
    }
}

// ============================================================
// EJERCICIO 5: Article (datos owned + referencias)
// ============================================================
// Un struct que mezcla datos propios (String) y referencias.
//
// TODO: Solo category es referencia, title es owned
// ============================================================

struct Article<'a> {
    // TODO: Agrega lifetime solo para category
    title: String,    // owned
    category: &'a str,   // referencia
}

impl<'a> Article<'a> {
    // TODO: Implementa con el lifetime correcto
    fn new(title: String, category: &'a str) -> Article<'a> {
        Article { title, category }
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn category(&self) -> &str {
        self.category
    }
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_excerpt_creation() {
        let text = String::from("Hello, world!");
        let excerpt = Excerpt::new(&text[..5]);
        assert_eq!(excerpt.part(), "Hello");
    }

    #[test]
    fn test_excerpt_len() {
        let text = "Rust";
        let excerpt = Excerpt::new(text);
        assert_eq!(excerpt.len(), 4);
    }

    #[test]
    fn test_config_creation() {
        let k = String::from("key");
        let v = String::from("value");
        let config = Config::new(&k, &v);
        assert_eq!(config.key(), "key");
        assert_eq!(config.value(), "value");
    }

    #[test]
    fn test_line_iterator() {
        let text = "a\nb\nc";
        let mut iter = LineIterator::new(text);
        assert_eq!(iter.next_line(), Some("a"));
        assert_eq!(iter.next_line(), Some("b"));
        assert_eq!(iter.next_line(), Some("c"));
        assert_eq!(iter.next_line(), None);
    }

    #[test]
    fn test_line_iterator_no_newline() {
        let text = "single line";
        let mut iter = LineIterator::new(text);
        assert_eq!(iter.next_line(), Some("single line"));
        assert_eq!(iter.next_line(), None);
    }

    #[test]
    fn test_split_found() {
        let split = Split::at_char("key=value", '=');
        assert_eq!(split.left(), "key");
        assert_eq!(split.right(), "value");
    }

    #[test]
    fn test_split_not_found() {
        let split = Split::at_char("no separator", '=');
        assert_eq!(split.left(), "no separator");
        assert_eq!(split.right(), "");
    }

    #[test]
    fn test_article_creation() {
        let cat = String::from("Tech");
        let article = Article::new("Rust News".to_string(), &cat);
        assert_eq!(article.title(), "Rust News");
        assert_eq!(article.category(), "Tech");
    }
}
