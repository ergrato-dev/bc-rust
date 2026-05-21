use wasm_bindgen::prelude::*;

/// Saluda al usuario por su nombre.
///
/// Accesible desde JavaScript como: `greet("Mundo")` → `"¡Hola, Mundo!"`
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("¡Hola, {name}!")
}

/// Suma dos números enteros.
#[wasm_bindgen]
pub fn suma(a: i32, b: i32) -> i32 {
    a + b
}

/// Invierte una cadena de texto.
#[wasm_bindgen]
pub fn invertir(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_basico() {
        assert_eq!(greet("Rust"), "¡Hola, Rust!");
    }

    #[test]
    fn suma_correcta() {
        assert_eq!(suma(2, 3), 5);
    }

    #[test]
    fn invertir_string() {
        assert_eq!(invertir("abc"), "cba");
    }
}

// Tests de wasm-bindgen (se ejecutan en el navegador o con wasm-pack test)
#[cfg(test)]
mod wasm_tests {
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn greet_en_wasm() {
        assert_eq!(super::greet("WASM"), "¡Hola, WASM!");
    }
}
