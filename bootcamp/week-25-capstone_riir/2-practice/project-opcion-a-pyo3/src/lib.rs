//! # capstone-opcion-a: Tokenizador de expresiones matemáticas para Python
//!
//! Expone un lexer/parser de expresiones matemáticas a Python usando PyO3.
//!
//! ## Ejemplo de uso desde Python
//!
//! ```python
//! from capstone_opcion_a import tokenizar, evaluar, Lexer
//!
//! tokens = tokenizar("3 + 4 * 2")
//! # [Token(Numero, "3"), Token(Mas, "+"), ...]
//!
//! resultado = evaluar("(3 + 4) * 2")
//! # 14.0
//! ```
#![deny(missing_docs)]

use pyo3::prelude::*;

/// Tipo de token en una expresión matemática.
#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub enum TipoToken {
    /// Número literal.
    Numero,
    /// Operador `+`.
    Mas,
    /// Operador `-`.
    Menos,
    /// Operador `*`.
    Multiplicacion,
    /// Operador `/`.
    Division,
    /// Paréntesis izquierdo.
    ParenIzq,
    /// Paréntesis derecho.
    ParenDer,
}

/// Unidad léxica de una expresión matemática.
#[pyclass]
#[derive(Debug, Clone)]
pub struct Token {
    /// Tipo del token.
    #[pyo3(get)]
    pub tipo: TipoToken,
    /// Valor textual del token.
    #[pyo3(get)]
    pub valor: String,
}

#[pymethods]
impl Token {
    fn __repr__(&self) -> String {
        format!("Token({:?}, {:?})", self.tipo, self.valor)
    }
}

/// Tokeniza una expresión matemática y retorna la lista de tokens.
///
/// # Errores
/// Retorna un error de Python si la expresión contiene caracteres inválidos.
#[pyfunction]
pub fn tokenizar(expresion: &str) -> PyResult<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = expresion.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' => { chars.next(); }
            '0'..='9' | '.' => {
                let mut num = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() || d == '.' {
                        num.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token { tipo: TipoToken::Numero, valor: num });
            }
            '+' => { tokens.push(Token { tipo: TipoToken::Mas, valor: "+".into() }); chars.next(); }
            '-' => { tokens.push(Token { tipo: TipoToken::Menos, valor: "-".into() }); chars.next(); }
            '*' => { tokens.push(Token { tipo: TipoToken::Multiplicacion, valor: "*".into() }); chars.next(); }
            '/' => { tokens.push(Token { tipo: TipoToken::Division, valor: "/".into() }); chars.next(); }
            '(' => { tokens.push(Token { tipo: TipoToken::ParenIzq, valor: "(".into() }); chars.next(); }
            ')' => { tokens.push(Token { tipo: TipoToken::ParenDer, valor: ")".into() }); chars.next(); }
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    format!("Carácter inválido: '{c}'")
                ));
            }
        }
    }
    Ok(tokens)
}

/// Evalúa una expresión matemática simple (sin precedencia de operadores).
///
/// Para una implementación completa con precedencia, usar un parser Pratt.
#[pyfunction]
pub fn evaluar_simple(expresion: &str) -> PyResult<f64> {
    let tokens = tokenizar(expresion)?;
    if tokens.is_empty() {
        return Ok(0.0);
    }

    let primero: f64 = tokens[0].valor.parse().map_err(|_| {
        pyo3::exceptions::PyValueError::new_err("El primer token debe ser un número")
    })?;

    let mut resultado = primero;
    let mut i = 1;
    while i + 1 < tokens.len() {
        let op = &tokens[i].tipo;
        let operando: f64 = tokens[i + 1].valor.parse().map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("Operando inválido")
        })?;
        match op {
            TipoToken::Mas => resultado += operando,
            TipoToken::Menos => resultado -= operando,
            TipoToken::Multiplicacion => resultado *= operando,
            TipoToken::Division => {
                if operando == 0.0 {
                    return Err(pyo3::exceptions::PyZeroDivisionError::new_err("División por cero"));
                }
                resultado /= operando;
            }
            _ => return Err(pyo3::exceptions::PyValueError::new_err("Operador esperado")),
        }
        i += 2;
    }
    Ok(resultado)
}

/// Módulo Python `capstone_opcion_a`.
#[pymodule]
fn capstone_opcion_a(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<TipoToken>()?;
    m.add_class::<Token>()?;
    m.add_function(wrap_pyfunction!(tokenizar, m)?)?;
    m.add_function(wrap_pyfunction!(evaluar_simple, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::Python;

    #[test]
    fn tokenizar_suma() {
        Python::with_gil(|_py| {
            let tokens = tokenizar("3 + 4").unwrap();
            assert_eq!(tokens.len(), 3);
            assert_eq!(tokens[0].valor, "3");
            assert_eq!(tokens[1].valor, "+");
            assert_eq!(tokens[2].valor, "4");
        });
    }

    #[test]
    fn tokenizar_caracter_invalido() {
        Python::with_gil(|_py| {
            assert!(tokenizar("3 @ 4").is_err());
        });
    }
}
