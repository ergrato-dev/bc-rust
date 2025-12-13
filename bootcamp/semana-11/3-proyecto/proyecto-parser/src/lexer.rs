//! Lexer (Tokenizador)
//!
//! Convierte texto en tokens sin copiar datos.

use crate::ast::Token;

/// Lexer zero-copy que tokeniza texto.
///
/// El lexer mantiene una referencia al texto original y produce
/// tokens que son referencias a porciones de ese texto.
#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    /// Crea un nuevo lexer para el input dado
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer { input, position: 0 }
    }

    /// Retorna el texto restante por procesar
    pub fn remaining(&self) -> &'a str {
        &self.input[self.position..]
    }

    /// Retorna true si no hay más input
    pub fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    /// Avanza la posición y retorna el caracter actual
    fn advance(&mut self) -> Option<char> {
        let remaining = self.remaining();
        let ch = remaining.chars().next()?;
        self.position += ch.len_utf8();
        Some(ch)
    }

    /// Mira el caracter actual sin avanzar
    fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    /// Produce el siguiente token
    pub fn next_token(&mut self) -> Token<'a> {
        if self.is_at_end() {
            return Token::Eof;
        }

        let start = self.position;
        let ch = self.peek().unwrap();

        if ch.is_whitespace() {
            self.consume_whitespace();
            return Token::Whitespace(&self.input[start..self.position]);
        }

        if ch.is_alphabetic() || ch == '_' {
            self.consume_word();
            return Token::Word(&self.input[start..self.position]);
        }

        if ch.is_ascii_digit() {
            self.consume_number();
            return Token::Number(&self.input[start..self.position]);
        }

        // Símbolo (un caracter)
        self.advance();
        Token::Symbol(&self.input[start..self.position])
    }

    /// Consume espacios en blanco
    fn consume_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if !ch.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    /// Consume una palabra (letras, dígitos, guiones bajos)
    fn consume_word(&mut self) {
        while let Some(ch) = self.peek() {
            if !ch.is_alphanumeric() && ch != '_' {
                break;
            }
            self.advance();
        }
    }

    /// Consume un número (dígitos y punto decimal)
    fn consume_number(&mut self) {
        let mut has_dot = false;
        while let Some(ch) = self.peek() {
            if ch == '.' && !has_dot {
                has_dot = true;
                self.advance();
            } else if ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Tokeniza todo el input y retorna un vector de tokens
    pub fn tokenize_all(&mut self) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            if matches!(token, Token::Eof) {
                break;
            }
            tokens.push(token);
        }
        tokens
    }

    /// Tokeniza excluyendo espacios en blanco
    pub fn tokenize_no_whitespace(&mut self) -> Vec<Token<'a>> {
        self.tokenize_all()
            .into_iter()
            .filter(|t| !matches!(t, Token::Whitespace(_)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_empty() {
        let mut lexer = Lexer::new("");
        assert!(matches!(lexer.next_token(), Token::Eof));
    }

    #[test]
    fn test_lexer_word() {
        let mut lexer = Lexer::new("hello");
        assert_eq!(lexer.next_token(), Token::Word("hello"));
        assert!(matches!(lexer.next_token(), Token::Eof));
    }

    #[test]
    fn test_lexer_number() {
        let mut lexer = Lexer::new("123");
        assert_eq!(lexer.next_token(), Token::Number("123"));
    }

    #[test]
    fn test_lexer_decimal() {
        let mut lexer = Lexer::new("3.14");
        assert_eq!(lexer.next_token(), Token::Number("3.14"));
    }

    #[test]
    fn test_lexer_symbol() {
        let mut lexer = Lexer::new("=");
        assert_eq!(lexer.next_token(), Token::Symbol("="));
    }

    #[test]
    fn test_lexer_whitespace() {
        let mut lexer = Lexer::new("  ");
        assert_eq!(lexer.next_token(), Token::Whitespace("  "));
    }

    #[test]
    fn test_lexer_mixed() {
        let mut lexer = Lexer::new("x = 42");
        assert_eq!(lexer.next_token(), Token::Word("x"));
        assert_eq!(lexer.next_token(), Token::Whitespace(" "));
        assert_eq!(lexer.next_token(), Token::Symbol("="));
        assert_eq!(lexer.next_token(), Token::Whitespace(" "));
        assert_eq!(lexer.next_token(), Token::Number("42"));
        assert!(matches!(lexer.next_token(), Token::Eof));
    }

    #[test]
    fn test_tokenize_all() {
        let mut lexer = Lexer::new("a b");
        let tokens = lexer.tokenize_all();
        assert_eq!(tokens.len(), 3); // word, space, word
    }

    #[test]
    fn test_tokenize_no_whitespace() {
        let mut lexer = Lexer::new("a b c");
        let tokens = lexer.tokenize_no_whitespace();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Word("a"));
        assert_eq!(tokens[1], Token::Word("b"));
        assert_eq!(tokens[2], Token::Word("c"));
    }

    #[test]
    fn test_remaining() {
        let mut lexer = Lexer::new("hello world");
        lexer.next_token(); // consume "hello"
        assert_eq!(lexer.remaining(), " world");
    }
}
