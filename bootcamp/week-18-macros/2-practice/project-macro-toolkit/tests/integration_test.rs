// project-macro-toolkit/tests/integration_test.rs
//
// Tests de integración para el proyecto macro toolkit.
// Verifica que los tres macros proc y las dos macros declarativas
// funcionan correctamente en conjunto.

use project_macro_toolkit::{assert_matches, builder, log_call, map, Describe};

// ── Fixtures ─────────────────────────────────────────────────────────────────

#[derive(Describe, Debug)]
struct Punto {
    x: f64,
    y: f64,
}

#[derive(Describe, Debug)]
struct Etiqueta(String);

#[derive(Describe, Debug)]
struct Vacío;

#[builder]
pub struct ServidorConfig {
    host: String,
    puerto: u16,
    max_conexiones: u32,
}

#[log_call]
fn duplicar(n: i32) -> i32 {
    n * 2
}

// ── Tests: Describe ───────────────────────────────────────────────────────────

#[test]
fn test_describe_campos_nombrados() {
    let p = Punto { x: 1.0, y: 2.0 };
    let d = p.describe();
    assert!(d.contains("Punto"), "debe incluir nombre del tipo");
    assert!(d.contains('x'), "debe incluir campo x");
    assert!(d.contains('y'), "debe incluir campo y");
}

#[test]
fn test_describe_campos_posicionales() {
    let e = Etiqueta("hola".to_string());
    let d = e.describe();
    assert!(d.contains("Etiqueta"), "debe incluir nombre del tipo");
    assert!(d.contains('0'), "debe incluir índice 0");
}

#[test]
fn test_describe_unit() {
    let v = Vacío;
    assert_eq!(v.describe(), "Vacío");
}

// ── Tests: log_call ───────────────────────────────────────────────────────────

#[test]
fn test_log_call_valor_correcto() {
    assert_eq!(duplicar(5), 10);
    assert_eq!(duplicar(-3), -6);
    assert_eq!(duplicar(0), 0);
}

// ── Tests: builder ────────────────────────────────────────────────────────────

#[test]
fn test_builder_completo() {
    let config = ServidorConfigBuilder::new()
        .host("127.0.0.1".to_string())
        .puerto(8080)
        .max_conexiones(100)
        .build()
        .expect("debería construirse sin error");

    assert_eq!(config.host, "127.0.0.1");
    assert_eq!(config.puerto, 8080);
    assert_eq!(config.max_conexiones, 100);
}

#[test]
fn test_builder_campo_faltante_retorna_error() {
    let resultado = ServidorConfigBuilder::new()
        .host("localhost".to_string())
        // puerto y max_conexiones no configurados
        .build();

    assert!(resultado.is_err(), "build() sin campos requeridos debe retornar Err");
}

// ── Tests: macros declarativas ────────────────────────────────────────────────

#[test]
fn test_map_vacio() {
    let m: std::collections::HashMap<&str, i32> = map![];
    assert!(m.is_empty());
}

#[test]
fn test_map_con_pares() {
    let m = map!["a" => 1, "b" => 2, "c" => 3];
    assert_eq!(m.len(), 3);
    assert_eq!(m["a"], 1);
    assert_eq!(m["b"], 2);
    assert_eq!(m["c"], 3);
}

#[test]
fn test_assert_matches_ok() {
    let v: Result<i32, &str> = Ok(42);
    assert_matches!(v, Ok(_));
}

#[test]
#[should_panic]
fn test_assert_matches_falla() {
    let v: Result<i32, &str> = Err("error");
    assert_matches!(v, Ok(_));
}

#[test]
fn test_assert_matches_con_guarda() {
    let v = Some(10);
    assert_matches!(v, Some(n) if n > 5);
}
