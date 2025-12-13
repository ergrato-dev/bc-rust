//! Árbol de Sintaxis Abstracta (AST)
//!
//! Define los tokens y nodos del AST usando referencias
//! para evitar copias innecesarias.

/// Token que representa una unidad léxica del texto.
///
/// Todos los tokens contienen referencias al texto original,
/// lo que permite parsing zero-copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'a> {
    /// Una palabra (secuencia de letras)
    Word(&'a str),
    /// Un número (secuencia de dígitos, posiblemente con punto)
    Number(&'a str),
    /// Un símbolo (operadores, puntuación)
    Symbol(&'a str),
    /// Espacios en blanco
    Whitespace(&'a str),
    /// Fin del input
    Eof,
}

impl<'a> Token<'a> {
    /// Retorna el texto del token, o "" si es EOF
    pub fn text(&self) -> &'a str {
        match self {
            Token::Word(s) => s,
            Token::Number(s) => s,
            Token::Symbol(s) => s,
            Token::Whitespace(s) => s,
            Token::Eof => "",
        }
    }

    /// Retorna true si es un token de palabra
    pub fn is_word(&self) -> bool {
        matches!(self, Token::Word(_))
    }

    /// Retorna true si es un token numérico
    pub fn is_number(&self) -> bool {
        matches!(self, Token::Number(_))
    }

    /// Retorna true si es un símbolo específico
    pub fn is_symbol(&self, expected: &str) -> bool {
        matches!(self, Token::Symbol(s) if *s == expected)
    }
}

/// Un nodo del AST que representa un par clave-valor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyValueNode<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> KeyValueNode<'a> {
    pub fn new(key: &'a str, value: &'a str) -> Self {
        KeyValueNode { key, value }
    }
}

/// Una línea CSV parseada
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsvRow<'a> {
    pub fields: Vec<&'a str>,
}

impl<'a> CsvRow<'a> {
    pub fn new(fields: Vec<&'a str>) -> Self {
        CsvRow { fields }
    }

    pub fn get(&self, index: usize) -> Option<&'a str> {
        self.fields.get(index).copied()
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_text() {
        let t = Token::Word("hello");
        assert_eq!(t.text(), "hello");
    }

    #[test]
    fn test_token_is_word() {
        assert!(Token::Word("test").is_word());
        assert!(!Token::Number("123").is_word());
    }

    #[test]
    fn test_token_is_symbol() {
        assert!(Token::Symbol("=").is_symbol("="));
        assert!(!Token::Symbol("+").is_symbol("="));
    }

    #[test]
    fn test_key_value_node() {
        let kv = KeyValueNode::new("name", "rust");
        assert_eq!(kv.key, "name");
        assert_eq!(kv.value, "rust");
    }

    #[test]
    fn test_csv_row() {
        let row = CsvRow::new(vec!["a", "b", "c"]);
        assert_eq!(row.len(), 3);
        assert_eq!(row.get(0), Some("a"));
        assert_eq!(row.get(5), None);
    }
}
