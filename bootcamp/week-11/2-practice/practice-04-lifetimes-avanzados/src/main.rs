//! Práctica 04: Lifetimes Avanzados
//!
//! En esta práctica aprenderás:
//! - Uso de 'static
//! - Lifetime bounds (T: 'a, 'a: 'b)
//! - Patrones avanzados con lifetimes

fn main() {
    println!("=== Práctica 04: Lifetimes Avanzados ===\n");

    // Ejercicio 1: 'static
    println!("Ejercicio 1 - 'static:");
    let greeting = get_greeting();
    println!("  Greeting: {}", greeting);
    let version = get_version();
    println!("  Version: {}", version);

    // Ejercicio 2: Lifetime bounds en structs
    println!("\nEjercicio 2 - Lifetime Bounds:");
    let value = 42;
    let holder = Holder::new(&value);
    println!("  Holder value: {}", holder.get());

    // Ejercicio 3: Múltiples lifetimes relacionados
    println!("\nEjercicio 3 - Múltiples Lifetimes:");
    let outer = String::from("outer string");
    {
        let inner = String::from("inner");
        let ctx = Context::new(&outer, &inner);
        println!("  Context: {} + {}", ctx.long_lived(), ctx.short_lived());
    }

    // Ejercicio 4: Trait con lifetime
    println!("\nEjercicio 4 - Trait con Lifetime:");
    let text = "palabra1 palabra2 palabra3";
    let tokenizer = SimpleTokenizer;
    let tokens = tokenizer.tokenize(text);
    println!("  Tokens: {:?}", tokens);

    // Ejercicio 5: Funciones genéricas con bounds
    println!("\nEjercicio 5 - Genéricos con Lifetime Bounds:");
    let items = vec!["uno", "dos", "tres"];
    let first = get_first_ref(&items);
    println!("  First: {:?}", first);

    println!("\n✅ Todos los ejercicios completados!");
}

// ============================================================
// EJERCICIO 1: 'static
// ============================================================
// 'static indica que el dato vive durante toda la ejecución.
// Los string literals son 'static.
//
// TODO: Implementa funciones que retornan &'static str
// ============================================================

/// Retorna un saludo estático
fn get_greeting() -> &'static str {
    // TODO: Retorna un string literal
    todo!("Implementar")
}

/// Retorna la versión de la aplicación
fn get_version() -> &'static str {
    // TODO: Usa una constante o literal
    todo!("Implementar")
}

// Constante 'static
const APP_VERSION: &str = "1.0.0";

// ============================================================
// EJERCICIO 2: Lifetime Bounds en Structs
// ============================================================
// T: 'a significa "T vive al menos tanto como 'a"
//
// TODO: Completa el struct Holder con el bound correcto
// ============================================================

struct Holder<'a, T> {
    // TODO: Agrega el bound T: 'a si es necesario
    // (en Rust moderno se infiere, pero practiquemos)
    value: &'a T,
}

impl<'a, T> Holder<'a, T> {
    fn new(value: &'a T) -> Holder<'a, T> {
        Holder { value }
    }

    fn get(&self) -> &T {
        self.value
    }
}

// ============================================================
// EJERCICIO 3: Múltiples Lifetimes Relacionados
// ============================================================
// A veces necesitas que un lifetime sea "más largo" que otro.
// 'a: 'b significa "'a vive al menos tanto como 'b"
//
// TODO: Implementa Context con dos lifetimes
// ============================================================

struct Context<'long, 'short> {
    // TODO: Completa los campos
    // long_data debe vivir al menos tanto como short_data
    long_data: &'long str,
    short_data: &'short str,
}

impl<'long, 'short> Context<'long, 'short> {
    fn new(long_data: &'long str, short_data: &'short str) -> Context<'long, 'short> {
        // TODO: Implementar
        todo!("Implementar")
    }

    fn long_lived(&self) -> &'long str {
        self.long_data
    }

    fn short_lived(&self) -> &'short str {
        self.short_data
    }
}

// ============================================================
// EJERCICIO 4: Trait con Lifetime
// ============================================================
// Los traits pueden tener parámetros de lifetime.
//
// TODO: Implementa el trait Tokenizer
// ============================================================

trait Tokenizer<'a> {
    fn tokenize(&self, input: &'a str) -> Vec<&'a str>;
}

struct SimpleTokenizer;

impl<'a> Tokenizer<'a> for SimpleTokenizer {
    fn tokenize(&self, input: &'a str) -> Vec<&'a str> {
        // TODO: Divide por espacios
        todo!("Implementar")
    }
}

// ============================================================
// EJERCICIO 5: Funciones Genéricas con Lifetime Bounds
// ============================================================
// Combina genéricos y lifetimes.
//
// TODO: Implementa get_first_ref
// ============================================================

/// Retorna una referencia al primer elemento si existe
fn get_first_ref<'a, T>(items: &'a [T]) -> Option<&'a T> {
    // TODO: Implementar
    todo!("Implementar")
}

// ============================================================
// BONUS: Higher-Ranked Trait Bounds (HRTB)
// ============================================================
// for<'a> significa "para cualquier lifetime 'a"
// Esto es avanzado, solo para referencia.

fn apply_to_str<F>(f: F, s: &str) -> &str
where
    F: Fn(&str) -> &str,
{
    f(s)
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_greeting() {
        let g = get_greeting();
        assert!(!g.is_empty());
    }

    #[test]
    fn test_static_version() {
        let v = get_version();
        assert!(v.contains('.'));
    }

    #[test]
    fn test_holder() {
        let x = 100;
        let h = Holder::new(&x);
        assert_eq!(*h.get(), 100);
    }

    #[test]
    fn test_holder_string() {
        let s = String::from("test");
        let h = Holder::new(&s);
        assert_eq!(h.get(), "test");
    }

    #[test]
    fn test_context() {
        let long = String::from("long lived");
        let short = String::from("short");
        let ctx = Context::new(&long, &short);
        assert_eq!(ctx.long_lived(), "long lived");
        assert_eq!(ctx.short_lived(), "short");
    }

    #[test]
    fn test_tokenizer() {
        let t = SimpleTokenizer;
        let tokens = t.tokenize("a b c");
        assert_eq!(tokens, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_tokenizer_empty() {
        let t = SimpleTokenizer;
        let tokens = t.tokenize("");
        assert!(tokens.is_empty() || tokens == vec![""]);
    }

    #[test]
    fn test_get_first_ref() {
        let v = vec![1, 2, 3];
        assert_eq!(get_first_ref(&v), Some(&1));
    }

    #[test]
    fn test_get_first_ref_empty() {
        let v: Vec<i32> = vec![];
        assert_eq!(get_first_ref(&v), None);
    }
}
