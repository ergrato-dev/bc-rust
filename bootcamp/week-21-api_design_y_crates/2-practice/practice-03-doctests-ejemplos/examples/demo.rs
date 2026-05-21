//! Ejemplo ejecutable: `cargo run --example demo`
//!
//! Muestra el uso de las funciones matemáticas del crate.

use practice_03_doctests_ejemplos::{es_primo, factorial, fibonacci};

fn main() {
    println!("=== Factoriales ===");
    for n in [0u64, 1, 5, 10, 15, 20] {
        println!("  factorial({n:2}) = {}", factorial(n));
    }

    println!("\n=== Fibonacci ===");
    let serie: Vec<u64> = (0..10).map(fibonacci).collect();
    println!("  fib(0..10) = {serie:?}");

    println!("\n=== Primos hasta 30 ===");
    let primos: Vec<u64> = (2..=30).filter(|&n| es_primo(n)).collect();
    println!("  {primos:?}");
}
