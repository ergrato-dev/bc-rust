// project-macro-toolkit/src/main.rs
//
// Demo del macro toolkit completo.
// Solo compila y ejecuta cuando las tres macros del crate -derive están
// completamente implementadas.

use project_macro_toolkit::{assert_matches, builder, log_call, map, Describe};

// ── Describe ─────────────────────────────────────────────────────────────────

#[derive(Describe, Debug)]
struct Producto {
    nombre: String,
    precio: f64,
    stock: u32,
}

// ── log_call ──────────────────────────────────────────────────────────────────

#[log_call]
fn calcular_total(precio: f64, cantidad: u32) -> f64 {
    precio * cantidad as f64
}

// ── builder ───────────────────────────────────────────────────────────────────

#[builder]
pub struct ConexionConfig {
    host: String,
    puerto: u16,
    timeout_ms: u64,
}

fn main() {
    // 1. Describe
    let p = Producto {
        nombre: "Widget".to_string(),
        precio: 9.99,
        stock: 100,
    };
    println!("Producto: {}", p.describe());

    // 2. log_call
    let total = calcular_total(9.99, 3);
    println!("Total: {:.2}", total);

    // 3. builder
    let config = ConexionConfigBuilder::new()
        .host("localhost".to_string())
        .puerto(5432)
        .timeout_ms(3000)
        .build()
        .expect("Config válida");
    println!("Conectando a {}:{}", config.host, config.puerto);

    // 4. Macros declarativas
    let precios = map![
        "widget" => 9.99_f64,
        "gadget" => 24.99_f64,
    ];
    println!("Widget cuesta {:.2}", precios["widget"]);

    let resultado: Option<i32> = Some(42);
    assert_matches!(resultado, Some(_));
    println!("assert_matches! OK");
}
