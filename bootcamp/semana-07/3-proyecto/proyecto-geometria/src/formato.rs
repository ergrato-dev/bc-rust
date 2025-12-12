// src/formato.rs
// Funciones de formateo para resultados geométricos

use crate::formas::Forma;

/// Formatea el resultado de una forma individual
pub fn formatear_resultado(nombre: &str, area: f64, perimetro: f64) -> String {
    format!(
        "{}: Área = {:.4}, Perímetro = {:.4}",
        nombre, area, perimetro
    )
}

/// Formatea el resultado usando el trait Forma
pub fn formatear_forma<T: Forma>(forma: &T, area: f64, perimetro: f64) -> String {
    formatear_resultado(forma.nombre(), area, perimetro)
}

/// Genera una tabla formateada con múltiples formas
pub fn formatear_tabla(formas: &[(&str, f64, f64)]) -> String {
    let mut resultado = String::new();

    // Header
    resultado.push_str("┌──────────────────────────┬──────────────┬──────────────┐\n");
    resultado.push_str("│ Forma                    │ Área         │ Perímetro    │\n");
    resultado.push_str("├──────────────────────────┼──────────────┼──────────────┤\n");

    // Rows
    for (nombre, area, perimetro) in formas {
        resultado.push_str(&format!(
            "│ {:<24} │ {:>12.4} │ {:>12.4} │\n",
            nombre, area, perimetro
        ));
    }

    // Footer
    resultado.push_str("└──────────────────────────┴──────────────┴──────────────┘");

    resultado
}

/// Formatea un valor con unidades
pub fn formatear_con_unidad(valor: f64, unidad: &str) -> String {
    format!("{:.4} {}", valor, unidad)
}

/// Formatea el área con unidades cuadradas
pub fn formatear_area(valor: f64, unidad: &str) -> String {
    format!("{:.4} {}²", valor, unidad)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatear_resultado() {
        let resultado = formatear_resultado("Círculo", 78.5398, 31.4159);
        assert!(resultado.contains("Círculo"));
        assert!(resultado.contains("78.5398"));
        assert!(resultado.contains("31.4159"));
    }

    #[test]
    fn test_formatear_tabla() {
        let formas = vec![
            ("Círculo", 78.5398, 31.4159),
            ("Rectángulo", 12.0, 14.0),
        ];

        let tabla = formatear_tabla(&formas);
        assert!(tabla.contains("Círculo"));
        assert!(tabla.contains("Rectángulo"));
        assert!(tabla.contains("┌"));
        assert!(tabla.contains("└"));
    }

    #[test]
    fn test_formatear_con_unidad() {
        let resultado = formatear_con_unidad(31.4159, "cm");
        assert_eq!(resultado, "31.4159 cm");
    }

    #[test]
    fn test_formatear_area() {
        let resultado = formatear_area(78.5398, "cm");
        assert_eq!(resultado, "78.5398 cm²");
    }
}
