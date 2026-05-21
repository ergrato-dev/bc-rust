#![deny(clippy::all)]

use napi_derive::napi;

/// Suma dos números — expuesta a Node.js.
#[napi]
pub fn suma(a: f64, b: f64) -> f64 {
    a + b
}

/// Convierte una cadena a mayúsculas.
#[napi]
pub fn a_mayusculas(s: String) -> String {
    s.to_uppercase()
}

/// Calcula el factorial de n (n <= 20).
#[napi]
pub fn factorial(n: u32) -> u64 {
    (1..=n as u64).product()
}
