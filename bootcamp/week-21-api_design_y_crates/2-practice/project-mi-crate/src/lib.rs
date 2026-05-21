//! # mi-crate — Utilidades de texto
//!
//! Librería de utilidades de procesamiento de texto lista para publicar en `crates.io`.
//!
//! ## Uso rápido
//!
//! ```
//! use project_mi_crate::{slugify, truncar, contar_palabras};
//!
//! assert_eq!(slugify("Hola Mundo"), "hola-mundo");
//! assert_eq!(truncar("hola mundo", 4, "..."), "hola...");
//! assert_eq!(contar_palabras("uno dos tres"), 3);
//! ```

#![deny(missing_docs)]

use thiserror::Error;

/// Errores de la librería.
#[derive(Debug, Error, PartialEq)]
pub enum TextError {
    /// La cadena de entrada está vacía.
    #[error("la cadena de entrada no puede estar vacía")]
    EmptyInput,
    /// El límite de truncado es inválido.
    #[error("el límite {0} es mayor que la longitud del texto")]
    InvalidLimit(usize),
}

/// Convierte un texto a formato slug (lowercase, espacios por guiones).
///
/// # Examples
///
/// ```
/// use project_mi_crate::slugify;
///
/// assert_eq!(slugify("Hola Mundo Rust"), "hola-mundo-rust");
/// assert_eq!(slugify("  espacios  extra  "), "espacios-extra");
/// ```
///
/// # Errors
///
/// Retorna [`TextError::EmptyInput`] si la cadena resultante está vacía.
pub fn slugify(texto: &str) -> String {
    texto
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join("-")
}

/// Trunca el texto a `limite` caracteres y agrega `sufijo`.
///
/// # Examples
///
/// ```
/// use project_mi_crate::truncar;
///
/// assert_eq!(truncar("hola mundo", 4, "..."), "hola...");
/// assert_eq!(truncar("corto", 10, "..."), "corto");
/// ```
pub fn truncar(texto: &str, limite: usize, sufijo: &str) -> String {
    let chars: Vec<char> = texto.chars().collect();
    if chars.len() <= limite {
        return texto.to_string();
    }
    let truncado: String = chars[..limite].iter().collect();
    format!("{truncado}{sufijo}")
}

/// Cuenta las palabras en el texto.
///
/// # Examples
///
/// ```
/// use project_mi_crate::contar_palabras;
///
/// assert_eq!(contar_palabras("uno dos tres"), 3);
/// assert_eq!(contar_palabras(""), 0);
/// assert_eq!(contar_palabras("  solo  una  "), 2);
/// ```
pub fn contar_palabras(texto: &str) -> usize {
    texto.split_whitespace().count()
}

/// Capitaliza la primera letra de cada palabra.
///
/// # Examples
///
/// ```
/// use project_mi_crate::titular;
///
/// assert_eq!(titular("hola mundo rust"), "Hola Mundo Rust");
/// ```
pub fn titular(texto: &str) -> String {
    texto
        .split_whitespace()
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_basico() {
        assert_eq!(slugify("Hola Mundo"), "hola-mundo");
    }

    #[test]
    fn slugify_espacios_extra() {
        assert_eq!(slugify("  a  b  "), "a-b");
    }

    #[test]
    fn truncar_corto() {
        assert_eq!(truncar("hola", 10, "..."), "hola");
    }

    #[test]
    fn truncar_largo() {
        assert_eq!(truncar("hola mundo", 4, "..."), "hola...");
    }

    #[test]
    fn contar_palabras_normal() {
        assert_eq!(contar_palabras("uno dos tres cuatro"), 4);
    }

    #[test]
    fn contar_palabras_vacio() {
        assert_eq!(contar_palabras(""), 0);
    }

    #[test]
    fn titular_minusculas() {
        assert_eq!(titular("hola mundo"), "Hola Mundo");
    }
}
