//! Soluciones de la Práctica 02: Lifetimes en Structs

fn main() {
    println!("=== Soluciones Práctica 02: Lifetimes en Structs ===\n");

    let novel = String::from("Llámame Ishmael. Hace algunos años...");
    let excerpt = Excerpt::new(&novel[..16]);
    println!("Ejercicio 1 - Excerpt: '{}'", excerpt.part());

    let key = String::from("database_url");
    let value = String::from("postgres://localhost/db");
    let config = Config::new(&key, &value);
    println!("Ejercicio 2 - Config: {} = {}", config.key(), config.value());

    let text = "primera línea\nsegunda línea\ntercera línea";
    let mut iter = LineIterator::new(text);
    println!("Ejercicio 3 - Líneas:");
    while let Some(line) = iter.next_line() {
        println!("  - {}", line);
    }

    let data = "clave=valor";
    let split = Split::at_char(data, '=');
    println!(
        "Ejercicio 4 - Split: '{}' y '{}'",
        split.left(),
        split.right()
    );

    let category = String::from("Tecnología");
    let article = Article::new("Rust en 2025".to_string(), &category);
    println!("Ejercicio 5 - Article: {} [{}]", article.title(), article.category());

    println!("\n✅ Todas las soluciones funcionan!");
}

// ============================================================
// SOLUCIÓN 1: Excerpt
// ============================================================
// El struct necesita lifetime porque contiene una referencia.
// El lifetime indica: "Excerpt no puede vivir más que el dato
// al que apunta part"
// ============================================================

struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn new(part: &'a str) -> Excerpt<'a> {
        Excerpt { part }
    }

    // part() puede omitir 'a en el retorno gracias a elision (regla 3)
    // pero explícitamente sería -> &'a str
    fn part(&self) -> &str {
        self.part
    }

    fn len(&self) -> usize {
        self.part.len()
    }
}

// ============================================================
// SOLUCIÓN 2: Config
// ============================================================
// Usamos UN solo lifetime porque normalmente key y value
// vienen del mismo contexto (misma configuración).
// Podrían ser dos lifetimes independientes si fuera necesario.
// ============================================================

struct Config<'a> {
    key: &'a str,
    value: &'a str,
}

impl<'a> Config<'a> {
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
// SOLUCIÓN 3: LineIterator
// ============================================================
// content es una referencia al texto original.
// position es un valor owned (usize), no necesita lifetime.
// ============================================================

struct LineIterator<'a> {
    content: &'a str,
    position: usize,
}

impl<'a> LineIterator<'a> {
    fn new(content: &'a str) -> LineIterator<'a> {
        LineIterator {
            content,
            position: 0,
        }
    }

    // El retorno Option<&str> usa el lifetime del struct
    // pero gracias a elision (regla 3) no necesitamos escribirlo
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
// SOLUCIÓN 4: Split
// ============================================================
// Ambas partes vienen del mismo string original,
// así que comparten el mismo lifetime.
// ============================================================

struct Split<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> Split<'a> {
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
// SOLUCIÓN 5: Article
// ============================================================
// Mezcla de datos owned (title: String) y referencias (category).
// Solo category necesita lifetime annotation.
// title es propiedad del struct, vive tanto como el struct.
// ============================================================

struct Article<'a> {
    title: String,       // owned - no necesita lifetime
    category: &'a str,   // referencia - necesita lifetime
}

impl<'a> Article<'a> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_solutions() {
        // Excerpt
        let text = "test";
        let e = Excerpt::new(text);
        assert_eq!(e.part(), "test");

        // Config
        let k = "key";
        let v = "val";
        let c = Config::new(k, v);
        assert_eq!(c.key(), "key");

        // LineIterator
        let mut li = LineIterator::new("a\nb");
        assert_eq!(li.next_line(), Some("a"));

        // Split
        let s = Split::at_char("a=b", '=');
        assert_eq!(s.left(), "a");

        // Article
        let cat = "Tech";
        let a = Article::new("Title".to_string(), cat);
        assert_eq!(a.category(), "Tech");
    }
}
