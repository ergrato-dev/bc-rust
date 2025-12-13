//! Demo del Parser de Texto Eficiente
//!
//! Este programa demuestra las capacidades del parser zero-copy.

use proyecto_parser::{Lexer, Parser};
use proyecto_parser::parser::IniParser;

fn main() {
    println!("=== Parser de Texto Eficiente ===\n");

    demo_lexer();
    demo_key_value();
    demo_csv();
    demo_ini();
    demo_extract();

    println!("\n✅ Demo completada!");
}

fn demo_lexer() {
    println!("📝 Demo Lexer (Tokenizador)");
    println!("{}", "-".repeat(40));

    let code = "let x = 42 + 3.14";
    println!("Input: '{}'", code);

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize_no_whitespace();

    println!("Tokens:");
    for token in &tokens {
        println!("  {:?}", token);
    }
    println!();
}

fn demo_key_value() {
    println!("🔑 Demo Key=Value Parser");
    println!("{}", "-".repeat(40));

    // Single key-value
    let single = "database_url = postgres://localhost/db";
    let parser = Parser::new(single);
    if let Some(kv) = parser.parse_key_value('=') {
        println!("Single: {} -> {}", kv.key, kv.value);
    }

    // Multiple key-values
    let config = r#"
host = localhost
port = 5432
# This is a comment
database = mydb
    "#;

    let parser = Parser::new(config);
    let kvs = parser.parse_key_values('=');
    println!("\nConfig file ({} entries):", kvs.len());
    for kv in &kvs {
        println!("  {} = {}", kv.key, kv.value);
    }
    println!();
}

fn demo_csv() {
    println!("📊 Demo CSV Parser");
    println!("{}", "-".repeat(40));

    let csv_data = r#"nombre,edad,ciudad
Alice,30,Madrid
Bob,25,Barcelona
Charlie,35,Valencia"#;

    let parser = Parser::new(csv_data);
    let rows = parser.parse_csv(',');

    println!("CSV con {} filas:", rows.len());
    for (i, row) in rows.iter().enumerate() {
        let fields: Vec<&str> = (0..row.len())
            .filter_map(|j| row.get(j))
            .collect();
        println!("  Fila {}: {:?}", i, fields);
    }
    println!();
}

fn demo_ini() {
    println!("📁 Demo INI Parser");
    println!("{}", "-".repeat(40));

    let ini_content = r#"
[database]
host = localhost
port = 5432
name = myapp

[server]
address = 0.0.0.0
port = 8080

[logging]
level = debug
"#;

    let parser = IniParser::new(ini_content);
    let sections = parser.parse();

    println!("INI con {} secciones:", sections.len());
    for (section, values) in &sections {
        println!("  [{}]", section);
        for kv in values {
            println!("    {} = {}", kv.key, kv.value);
        }
    }
    println!();
}

fn demo_extract() {
    println!("🔍 Demo Extracción");
    println!("{}", "-".repeat(40));

    // Extract words
    let text = "Rust es un lenguaje de programación, ¡increíble!";
    let parser = Parser::new(text);
    let words = parser.extract_words();
    println!("Palabras en '{}...':", &text[..20]);
    println!("  {:?}", words);

    // Extract between delimiters
    let code = "fn main() { println!(\"Hello\"); }";
    let parser = Parser::new(code);
    if let Some(body) = parser.extract_between('{', '}') {
        println!("\nContenido entre {{ }}: '{}'", body.trim());
    }

    // Split demo
    let path = "/usr/local/bin/rust";
    let parser = Parser::new(path);
    let parts: Vec<_> = parser.split('/').filter(|s| !s.is_empty()).collect();
    println!("\nPath '{}' dividido: {:?}", path, parts);
}
