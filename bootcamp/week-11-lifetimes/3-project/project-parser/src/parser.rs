//! Parser
//!
//! Parsea estructuras de datos desde texto usando referencias.

use crate::ast::{CsvRow, KeyValueNode};

/// Par clave-valor usando referencias al texto original
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyValue<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> KeyValue<'a> {
    pub fn new(key: &'a str, value: &'a str) -> Self {
        KeyValue { key, value }
    }
}

/// Parser zero-copy para varias estructuras de texto
#[derive(Debug)]
pub struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    /// Crea un nuevo parser para el input dado
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser { input }
    }

    /// Retorna el input original
    pub fn input(&self) -> &'a str {
        self.input
    }

    /// Parsea un par key=value
    ///
    /// # Ejemplo
    /// ```
    /// use project_parser::Parser;
    ///
    /// let parser = Parser::new("name=rust");
    /// let kv = parser.parse_key_value('=').unwrap();
    /// assert_eq!(kv.key, "name");
    /// assert_eq!(kv.value, "rust");
    /// ```
    pub fn parse_key_value(&self, separator: char) -> Option<KeyValue<'a>> {
        let pos = self.input.find(separator)?;
        let key = self.input[..pos].trim();
        let value = self.input[pos + separator.len_utf8()..].trim();
        Some(KeyValue::new(key, value))
    }

    /// Parsea múltiples pares key=value separados por líneas
    pub fn parse_key_values(&self, separator: char) -> Vec<KeyValue<'a>> {
        self.input
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    return None;
                }
                Parser::new(line).parse_key_value(separator)
            })
            .collect()
    }

    /// Parsea una línea CSV
    pub fn parse_csv_row(&self, delimiter: char) -> CsvRow<'a> {
        let fields: Vec<&'a str> = self.input
            .split(delimiter)
            .map(|s| s.trim())
            .collect();
        CsvRow::new(fields)
    }

    /// Parsea múltiples líneas CSV
    pub fn parse_csv(&self, delimiter: char) -> Vec<CsvRow<'a>> {
        self.input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| Parser::new(line).parse_csv_row(delimiter))
            .collect()
    }

    /// Extrae todas las palabras (sin espacios ni puntuación)
    pub fn extract_words(&self) -> Vec<&'a str> {
        self.input
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Divide el input en líneas
    pub fn lines(&self) -> impl Iterator<Item = &'a str> {
        self.input.lines()
    }

    /// Divide el input por un delimitador
    pub fn split(&self, delimiter: char) -> impl Iterator<Item = &'a str> {
        self.input.split(delimiter)
    }

    /// Encuentra y extrae un fragmento entre delimitadores
    pub fn extract_between(&self, start: char, end: char) -> Option<&'a str> {
        let start_pos = self.input.find(start)? + start.len_utf8();
        let remaining = &self.input[start_pos..];
        let end_pos = remaining.find(end)?;
        Some(&remaining[..end_pos])
    }

    /// Convierte a KeyValueNode del AST
    pub fn to_ast_node(&self, separator: char) -> Option<KeyValueNode<'a>> {
        let kv = self.parse_key_value(separator)?;
        Some(KeyValueNode::new(kv.key, kv.value))
    }
}

/// Parser de configuración estilo INI
#[derive(Debug)]
pub struct IniParser<'a> {
    input: &'a str,
}

impl<'a> IniParser<'a> {
    pub fn new(input: &'a str) -> IniParser<'a> {
        IniParser { input }
    }

    /// Parsea secciones [section] y sus key=value
    pub fn parse(&self) -> Vec<(&'a str, Vec<KeyValue<'a>>)> {
        let mut result = Vec::new();
        let mut current_section = "";
        let mut current_values = Vec::new();

        for line in self.input.lines() {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                // Nueva sección
                if !current_section.is_empty() || !current_values.is_empty() {
                    result.push((current_section, std::mem::take(&mut current_values)));
                }
                current_section = &line[1..line.len() - 1];
            } else if let Some(kv) = Parser::new(line).parse_key_value('=') {
                current_values.push(kv);
            }
        }

        // Última sección
        if !current_section.is_empty() || !current_values.is_empty() {
            result.push((current_section, current_values));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_key_value() {
        let parser = Parser::new("name=rust");
        let kv = parser.parse_key_value('=').unwrap();
        assert_eq!(kv.key, "name");
        assert_eq!(kv.value, "rust");
    }

    #[test]
    fn test_parse_key_value_with_spaces() {
        let parser = Parser::new("  key  =  value  ");
        let kv = parser.parse_key_value('=').unwrap();
        assert_eq!(kv.key, "key");
        assert_eq!(kv.value, "value");
    }

    #[test]
    fn test_parse_key_value_no_separator() {
        let parser = Parser::new("no separator");
        assert!(parser.parse_key_value('=').is_none());
    }

    #[test]
    fn test_parse_key_values() {
        let input = "a=1\nb=2\n# comment\nc=3";
        let parser = Parser::new(input);
        let kvs = parser.parse_key_values('=');
        assert_eq!(kvs.len(), 3);
        assert_eq!(kvs[0].key, "a");
        assert_eq!(kvs[2].value, "3");
    }

    #[test]
    fn test_parse_csv_row() {
        let parser = Parser::new("a,b,c");
        let row = parser.parse_csv_row(',');
        assert_eq!(row.len(), 3);
        assert_eq!(row.get(0), Some("a"));
        assert_eq!(row.get(1), Some("b"));
        assert_eq!(row.get(2), Some("c"));
    }

    #[test]
    fn test_parse_csv() {
        let input = "a,b,c\n1,2,3";
        let parser = Parser::new(input);
        let rows = parser.parse_csv(',');
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].get(0), Some("a"));
        assert_eq!(rows[1].get(0), Some("1"));
    }

    #[test]
    fn test_extract_words() {
        let parser = Parser::new("hello, world! rust_lang 123");
        let words = parser.extract_words();
        assert_eq!(words, vec!["hello", "world", "rust_lang", "123"]);
    }

    #[test]
    fn test_extract_between() {
        let parser = Parser::new("prefix(content)suffix");
        assert_eq!(parser.extract_between('(', ')'), Some("content"));
    }

    #[test]
    fn test_extract_between_not_found() {
        let parser = Parser::new("no brackets");
        assert!(parser.extract_between('(', ')').is_none());
    }

    #[test]
    fn test_ini_parser() {
        let input = r#"
[section1]
key1=value1
key2=value2

[section2]
key3=value3
"#;
        let parser = IniParser::new(input);
        let sections = parser.parse();

        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].0, "section1");
        assert_eq!(sections[0].1.len(), 2);
        assert_eq!(sections[1].0, "section2");
        assert_eq!(sections[1].1.len(), 1);
    }
}
