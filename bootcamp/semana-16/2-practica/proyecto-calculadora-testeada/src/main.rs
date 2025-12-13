use proyecto_calculadora_testeada::{Calculadora, factorial, mcd, mcm};

fn main() {
    println!("=== Calculadora Testeada ===\n");

    let mut calc = Calculadora::new();

    calc.sumar(100.0);
    calc.multiplicar(2.0);
    calc.restar(50.0);
    calc.dividir(3.0).unwrap();

    println!("Resultado: {:.2}", calc.resultado());
    println!("\nHistorial:");
    for op in calc.historial() {
        println!("  {}", op);
    }

    println!("\n--- Funciones matematicas ---");
    println!("Factorial(6) = {}", factorial(6).unwrap());
    println!("MCD(48, 18) = {}", mcd(48, 18));
    println!("MCM(4, 6) = {}", mcm(4, 6));

    println!("\nEjecuta: cargo test");
}
