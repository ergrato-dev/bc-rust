// Práctica 01: Result Básico
// ==========================
// Aprende a usar Result<T, E> para manejar errores recuperables.
//
// OBJETIVO:
// - Crear funciones que retornan Result
// - Usar match para manejar Ok y Err
// - Aplicar métodos como unwrap_or, map, and_then
//
// INSTRUCCIONES:
// 1. Completa cada función según las indicaciones
// 2. Ejecuta el programa para verificar tu solución
// 3. Los tests deben pasar: cargo test

fn main() {
    println!("=== Práctica: Result Básico ===\n");

    // Ejercicio 1: División segura
    println!("1. División segura:");
    match dividir(10.0, 2.0) {
        Ok(resultado) => println!("   10 / 2 = {}", resultado),
        Err(e) => println!("   Error: {}", e),
    }
    match dividir(10.0, 0.0) {
        Ok(resultado) => println!("   10 / 0 = {}", resultado),
        Err(e) => println!("   Error: {}", e),
    }

    // Ejercicio 2: Parsear edad
    println!("\n2. Parsear edad:");
    println!("   '25' → {:?}", parsear_edad("25"));
    println!("   '-5' → {:?}", parsear_edad("-5"));
    println!("   'abc' → {:?}", parsear_edad("abc"));

    // Ejercicio 3: Encadenar operaciones
    println!("\n3. Operación encadenada:");
    println!("   calcular('10', 2.0) → {:?}", calcular("10", 2.0));
    println!("   calcular('abc', 2.0) → {:?}", calcular("abc", 2.0));
    println!("   calcular('10', 0.0) → {:?}", calcular("10", 0.0));

    // Ejercicio 4: Valor por defecto
    println!("\n4. Con valor por defecto:");
    println!("   config_o_default('8080') → {}", config_o_default("8080"));
    println!("   config_o_default('abc') → {}", config_o_default("abc"));
}

// ============================================================================
// EJERCICIO 1: División segura
// ============================================================================
// Implementa una función que divide dos números.
// Retorna Err si el divisor es cero.

fn dividir(dividendo: f64, divisor: f64) -> Result<f64, String> {
    // TODO: Implementar
    // - Si divisor == 0.0, retornar Err("División por cero")
    // - Si no, retornar Ok(dividendo / divisor)
    todo!("Implementar dividir")
}

// ============================================================================
// EJERCICIO 2: Parsear edad con validación
// ============================================================================
// Parsea un string a edad (u8), validando que sea un número válido.
// Una edad válida es entre 0 y 150.

fn parsear_edad(s: &str) -> Result<u8, String> {
    // TODO: Implementar
    // 1. Intentar parsear s a i32 (para detectar negativos)
    //    - Si falla, retornar Err("No es un número válido")
    // 2. Validar rango 0..=150
    //    - Si es negativo, retornar Err("La edad no puede ser negativa")
    //    - Si es > 150, retornar Err("Edad fuera de rango")
    // 3. Convertir a u8 y retornar Ok
    //
    // PISTA: Usa parse::<i32>() primero
    todo!("Implementar parsear_edad")
}

// ============================================================================
// EJERCICIO 3: Encadenar operaciones con and_then
// ============================================================================
// Parsea un string a número y luego divide por el divisor dado.
// Encadena las operaciones usando and_then o el operador ?.

fn calcular(texto: &str, divisor: f64) -> Result<f64, String> {
    // TODO: Implementar
    // 1. Parsear texto a f64
    // 2. Dividir por divisor usando la función dividir
    // 
    // PISTA: Puedes usar:
    //   texto.parse::<f64>().map_err(|_| "...".to_string())?.pipe(|n| dividir(n, divisor))
    // O:
    //   texto.parse::<f64>().map_err(...).and_then(|n| dividir(n, divisor))
    todo!("Implementar calcular")
}

// ============================================================================
// EJERCICIO 4: Valor por defecto con unwrap_or
// ============================================================================
// Intenta parsear un puerto, si falla usa 3000 como default.

fn config_o_default(puerto_str: &str) -> u16 {
    // TODO: Implementar
    // Parsear puerto_str a u16, si falla retornar 3000
    // 
    // PISTA: Usa .unwrap_or(3000)
    todo!("Implementar config_o_default")
}

// ============================================================================
// TESTS
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dividir_ok() {
        assert_eq!(dividir(10.0, 2.0), Ok(5.0));
        assert_eq!(dividir(7.0, 2.0), Ok(3.5));
    }

    #[test]
    fn test_dividir_error() {
        assert!(dividir(10.0, 0.0).is_err());
    }

    #[test]
    fn test_parsear_edad_ok() {
        assert_eq!(parsear_edad("25"), Ok(25));
        assert_eq!(parsear_edad("0"), Ok(0));
        assert_eq!(parsear_edad("150"), Ok(150));
    }

    #[test]
    fn test_parsear_edad_negativa() {
        assert!(parsear_edad("-5").is_err());
    }

    #[test]
    fn test_parsear_edad_invalida() {
        assert!(parsear_edad("abc").is_err());
        assert!(parsear_edad("200").is_err());
    }

    #[test]
    fn test_calcular_ok() {
        assert_eq!(calcular("10", 2.0), Ok(5.0));
    }

    #[test]
    fn test_calcular_parse_error() {
        assert!(calcular("abc", 2.0).is_err());
    }

    #[test]
    fn test_calcular_division_error() {
        assert!(calcular("10", 0.0).is_err());
    }

    #[test]
    fn test_config_o_default() {
        assert_eq!(config_o_default("8080"), 8080);
        assert_eq!(config_o_default("abc"), 3000);
        assert_eq!(config_o_default(""), 3000);
    }
}
