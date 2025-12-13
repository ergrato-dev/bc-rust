//! # Parser de Texto Eficiente
//!
//! Este módulo implementa un parser zero-copy que utiliza lifetimes
//! para evitar duplicación de datos.

pub mod ast;
pub mod lexer;
pub mod parser;

pub use ast::Token;
pub use lexer::Lexer;
pub use parser::{KeyValue, Parser};
