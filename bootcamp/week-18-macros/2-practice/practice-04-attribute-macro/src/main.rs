// practice-04-attribute-macro/src/main.rs
//
// Crate consumidor de los attribute macros `#[log_call]` y `#[rename]`.
// Implementa el crate -impl y este archivo compilará + pasará los tests.

use practice_04_attribute_macro_impl::{log_call, rename};

// ── Funciones anotadas con #[log_call] ───────────────────────────────────────

#[log_call]
fn suma(a: i32, b: i32) -> i32 {
    a + b
}

#[log_call]
fn saludo(nombre: &str) -> String {
    format!("Hola, {}!", nombre)
}

// ── Función renombrada con #[rename] ─────────────────────────────────────────

// Nota: el nombre original en el source es `add`, pero el macro
// lo renombra a `adicion` — la función se llama como `adicion(...)`.
#[rename("adicion")]
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    // log_call debe imprimir antes y después del resultado
    let s = suma(3, 4);
    println!("suma = {s}");

    let g = saludo("mundo");
    println!("{g}");

    let r = adicion(10, 5);
    println!("adicion = {r}");
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Tests de #[log_call] ─────────────────────────────────────────────
    #[test]
    fn test_log_call_retorna_valor_correcto() {
        // El macro no debe cambiar el valor de retorno
        assert_eq!(suma(2, 3), 5);
    }

    #[test]
    fn test_log_call_strings() {
        let r = saludo("Rust");
        assert_eq!(r, "Hola, Rust!");
    }

    #[test]
    fn test_log_call_multiples_llamadas() {
        // Múltiples invocaciones deben funcionar igual
        assert_eq!(suma(0, 0), 0);
        assert_eq!(suma(-1, 1), 0);
        assert_eq!(suma(100, -50), 50);
    }

    // ── Tests de #[rename] ───────────────────────────────────────────────
    #[test]
    fn test_rename_funcion_renombrada() {
        // La función debe ser accesible con el nuevo nombre
        assert_eq!(adicion(3, 4), 7);
    }

    #[test]
    fn test_rename_valor_correcto() {
        assert_eq!(adicion(-5, 5), 0);
        assert_eq!(adicion(100, 200), 300);
    }
}
