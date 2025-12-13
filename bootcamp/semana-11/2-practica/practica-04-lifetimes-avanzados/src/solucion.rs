//! Soluciones de la Práctica 04: Lifetimes Avanzados

fn main() {
    println!("=== Soluciones Práctica 04: Lifetimes Avanzados ===\n");

    println!("Ejercicio 1 - 'static:");
    let greeting = get_greeting();
    println!("  Greeting: {}", greeting);
    let version = get_version();
    println!("  Version: {}", version);

    println!("\nEjercicio 2 - Lifetime Bounds:");
    let value = 42;
    let holder = Holder::new(&value);
    println!("  Holder value: {}", holder.get());

    println!("\nEjercicio 3 - Múltiples Lifetimes:");
    let outer = String::from("outer string");
    {
        let inner = String::from("inner");
        let ctx = Context::new(&outer, &inner);
        println!("  Context: {} + {}", ctx.long_lived(), ctx.short_lived());
    }

    println!("\nEjercicio 4 - Trait con Lifetime:");
    let text = "palabra1 palabra2 palabra3";
    let tokenizer = SimpleTokenizer;
    let tokens = tokenizer.tokenize(text);
    println!("  Tokens: {:?}", tokens);

    println!("\nEjercicio 5 - Genéricos con Lifetime Bounds:");
    let items = vec!["uno", "dos", "tres"];
    let first = get_first_ref(&items);
    println!("  First: {:?}", first);

    println!("\n✅ Todas las soluciones funcionan!");
}

// ============================================================
// SOLUCIÓN 1: 'static
// ============================================================
// String literals viven en el binario del programa.
// Son inherentemente 'static.
// ============================================================

fn get_greeting() -> &'static str {
    // Los string literals son 'static
    "¡Hola, mundo!"
}

const APP_VERSION: &str = "1.0.0";

fn get_version() -> &'static str {
    // Podemos retornar una constante o un literal
    APP_VERSION
    // O directamente: "1.0.0"
}

// ============================================================
// SOLUCIÓN 2: Lifetime Bounds en Structs
// ============================================================
// En Rust moderno, T: 'a se infiere automáticamente cuando
// tienes &'a T. Pero es bueno entender el concepto.
//
// T: 'a significa: "Si T contiene referencias, deben vivir
// al menos tanto como 'a"
// ============================================================

struct Holder<'a, T: 'a> {
    // T: 'a es inferido, pero lo escribimos explícitamente
    // para practicar
    value: &'a T,
}

impl<'a, T: 'a> Holder<'a, T> {
    fn new(value: &'a T) -> Holder<'a, T> {
        Holder { value }
    }

    fn get(&self) -> &T {
        self.value
    }
}

// ============================================================
// SOLUCIÓN 3: Múltiples Lifetimes Relacionados
// ============================================================
// Usamos dos lifetimes independientes.
// En este caso no necesitamos 'long: 'short porque
// no hay dependencia directa en cómo los usamos.
// ============================================================

struct Context<'long, 'short> {
    long_data: &'long str,
    short_data: &'short str,
}

impl<'long, 'short> Context<'long, 'short> {
    fn new(long_data: &'long str, short_data: &'short str) -> Context<'long, 'short> {
        Context {
            long_data,
            short_data,
        }
    }

    fn long_lived(&self) -> &'long str {
        self.long_data
    }

    fn short_lived(&self) -> &'short str {
        self.short_data
    }
}

// ============================================================
// SOLUCIÓN 4: Trait con Lifetime
// ============================================================
// El trait tiene un parámetro de lifetime que permite
// que las implementaciones retornen referencias al input.
// ============================================================

trait Tokenizer<'a> {
    fn tokenize(&self, input: &'a str) -> Vec<&'a str>;
}

struct SimpleTokenizer;

impl<'a> Tokenizer<'a> for SimpleTokenizer {
    fn tokenize(&self, input: &'a str) -> Vec<&'a str> {
        // split_whitespace retorna iterador de &str con el mismo lifetime
        if input.is_empty() {
            Vec::new()
        } else {
            input.split_whitespace().collect()
        }
    }
}

// ============================================================
// SOLUCIÓN 5: Funciones Genéricas con Lifetime Bounds
// ============================================================
// Combinamos genéricos T con lifetimes 'a.
// El slice y su contenido comparten el lifetime.
// ============================================================

fn get_first_ref<'a, T>(items: &'a [T]) -> Option<&'a T> {
    items.first()
}

// ============================================================
// BONUS: Ejemplo de 'a: 'b (outlives)
// ============================================================
// Este ejemplo muestra cuándo necesitas el bound 'a: 'b

#[allow(dead_code)]
fn needs_outlives<'a, 'b>(long: &'a str, _short: &'b str) -> &'b str
where
    'a: 'b, // 'a vive al menos tanto como 'b
{
    // Podemos "acortar" long a 'b porque 'a: 'b
    long
}

// ============================================================
// BONUS: HRTB ejemplo
// ============================================================

#[allow(dead_code)]
fn apply_to_str<F>(f: F, s: &str) -> &str
where
    F: Fn(&str) -> &str,
{
    f(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_solutions() {
        // 'static
        assert!(!get_greeting().is_empty());
        assert!(get_version().contains('.'));

        // Holder
        let x = 42;
        let h = Holder::new(&x);
        assert_eq!(*h.get(), 42);

        // Context
        let l = "long";
        let s = "short";
        let ctx = Context::new(l, s);
        assert_eq!(ctx.long_lived(), "long");

        // Tokenizer
        let t = SimpleTokenizer;
        assert_eq!(t.tokenize("a b"), vec!["a", "b"]);

        // get_first_ref
        assert_eq!(get_first_ref(&[1, 2, 3]), Some(&1));
    }
}
