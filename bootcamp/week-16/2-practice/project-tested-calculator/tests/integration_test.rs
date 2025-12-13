//! Tests de integracion para la calculadora

use proyecto_calculadora_testeada::{Calculadora, CalculadoraError, factorial, mcd, mcm};

#[test]
fn test_flujo_operaciones_basicas() {
    let mut calc = Calculadora::new();

    calc.sumar(100.0);
    calc.restar(20.0);
    calc.multiplicar(2.0);
    calc.dividir(4.0).unwrap();

    assert_eq!(calc.resultado(), 40.0);
}

#[test]
fn test_flujo_con_errores() {
    let mut calc = Calculadora::con_valor(100.0);

    calc.dividir(2.0).unwrap();
    assert_eq!(calc.resultado(), 50.0);

    let result = calc.dividir(0.0);
    assert!(result.is_err());

    // El valor no cambio despues del error
    assert_eq!(calc.resultado(), 50.0);
}

#[test]
fn test_historial_completo() {
    let mut calc = Calculadora::new();

    calc.sumar(10.0);
    calc.multiplicar(5.0);
    calc.restar(25.0);

    let historial = calc.historial();
    assert_eq!(historial.len(), 3);
    assert!(historial[0].contains("10"));
    assert!(historial[1].contains("5"));
    assert!(historial[2].contains("25"));
}

#[test]
fn test_funciones_combinadas() {
    let a = 12u64;
    let b = 8u64;

    let divisor = mcd(a, b);
    let multiplo = mcm(a, b);

    // MCD * MCM = a * b
    assert_eq!(divisor * multiplo, a * b);
}

#[test]
fn test_calculadora_cientifica() {
    let mut calc = Calculadora::con_valor(16.0);

    calc.raiz().unwrap();
    assert_eq!(calc.resultado(), 4.0);

    calc.potencia(3.0);
    assert_eq!(calc.resultado(), 64.0);
}
