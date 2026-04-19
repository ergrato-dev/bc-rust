//! Tests de integracion para la calculadora

use project_tested_calculator::{Calculator, CalculatorError, factorial, gcd, lcm};

#[test]
fn test_basic_operations_flow() {
    let mut calc = Calculator::new();

    calc.add(100.0);
    calc.subtract(20.0);
    calc.multiply(2.0);
    calc.divide(4.0).unwrap();

    assert_eq!(calc.result(), 40.0);
}

#[test]
fn test_flow_with_errors() {
    let mut calc = Calculator::with_value(100.0);

    calc.divide(2.0).unwrap();
    assert_eq!(calc.result(), 50.0);

    let result = calc.divide(0.0);
    assert!(result.is_err());

    // El valor no cambio despues del error
    assert_eq!(calc.result(), 50.0);
}

#[test]
fn test_complete_history() {
    let mut calc = Calculator::new();

    calc.add(10.0);
    calc.multiply(5.0);
    calc.subtract(25.0);

    let history = calc.history();
    assert_eq!(history.len(), 3);
    assert!(history[0].contains("10"));
    assert!(history[1].contains("5"));
    assert!(history[2].contains("25"));
}

#[test]
fn test_combined_functions() {
    let a = 12u64;
    let b = 8u64;

    let divisor = gcd(a, b);
    let multiple = lcm(a, b);

    // GCD * LCM = a * b
    assert_eq!(divisor * multiple, a * b);
}

#[test]
fn test_scientific_calculator() {
    let mut calc = Calculator::with_value(16.0);

    calc.sqrt().unwrap();
    assert_eq!(calc.result(), 4.0);

    calc.power(3.0);
    assert_eq!(calc.result(), 64.0);
}
