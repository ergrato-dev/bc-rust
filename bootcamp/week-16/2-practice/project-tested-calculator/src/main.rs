use project_tested_calculator::{Calculator, factorial, gcd, lcm};

fn main() {
    println!("=== Calculadora Testeada ===\n");

    let mut calc = Calculator::new();

    calc.add(100.0);
    calc.multiply(2.0);
    calc.subtract(50.0);
    calc.divide(3.0).unwrap();

    println!("Resultado: {:.2}", calc.result());
    println!("\nHistorial:");
    for op in calc.history() {
        println!("  {}", op);
    }

    println!("\n--- Funciones matematicas ---");
    println!("Factorial(6) = {}", factorial(6).unwrap());
    println!("GCD(48, 18) = {}", gcd(48, 18));
    println!("LCM(4, 6) = {}", lcm(4, 6));

    println!("\nEjecuta: cargo test");
}
