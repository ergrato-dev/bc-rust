// ============================================
// Práctica 04: Shadowing y Constantes
// ============================================
// Objetivo: Dominar shadowing y const
// ============================================

// Constantes: se definen fuera de funciones
const PI: f64 = 3.14159265358979;
const MAX_INTENTOS: u32 = 3;
const MENSAJE_BIENVENIDA: &str = "¡Bienvenido al sistema!";

fn main() {
    println!("=== Práctica 04: Shadowing y Constantes ===\n");

    // -----------------------------------------
    // PARTE 1: Shadowing Básico
    // -----------------------------------------
    println!("--- Shadowing Básico ---");
    
    let x = 5;
    println!("x inicial: {}", x);
    
    let x = x + 1;  // Shadowing: nueva variable
    println!("x después de +1: {}", x);
    
    let x = x * 2;  // Shadowing: otra nueva variable
    println!("x después de *2: {}", x);
    
    // Comparación con mut
    let mut y = 5;
    y = y + 1;      // Mutación: misma variable
    y = y * 2;
    println!("y con mut: {}", y);
    
    // Resultado igual, pero mecanismo diferente

    // -----------------------------------------
    // PARTE 2: Cambio de Tipo con Shadowing
    // -----------------------------------------
    println!("\n--- Cambio de Tipo ---");
    
    // Esto es válido con shadowing
    let espacios = "   ";           // tipo: &str
    let espacios = espacios.len();  // tipo: usize
    println!("Número de espacios: {}", espacios);
    
    // Con mut NO funcionaría:
    // let mut texto = "hola";
    // texto = texto.len();  // Error: tipos diferentes
    
    // TODO: Haz shadowing para convertir un número a String
    let numero = 42;
    // let numero = ...;  // Convierte a String
    println!("Número como i32: {}", numero);

    // -----------------------------------------
    // PARTE 3: Shadowing en Scopes
    // -----------------------------------------
    println!("\n--- Shadowing en Scopes ---");
    
    let valor = 10;
    println!("Valor fuera: {}", valor);
    
    {
        // Shadow solo dentro del bloque
        let valor = 99;
        println!("Valor dentro del bloque: {}", valor);
    }
    
    // Fuera del bloque, vuelve al original
    println!("Valor después del bloque: {}", valor);
    
    // TODO: Experimenta con scopes anidados
    // {
    //     let a = 1;
    //     {
    //         let a = 2;
    //         println!("a interno: {}", a);
    //     }
    //     println!("a externo: {}", a);
    // }

    // -----------------------------------------
    // PARTE 4: Constantes
    // -----------------------------------------
    println!("\n--- Constantes ---");
    
    println!("PI: {}", PI);
    println!("Máximo de intentos: {}", MAX_INTENTOS);
    println!("{}", MENSAJE_BIENVENIDA);
    
    // Usando constantes en cálculos
    let radio = 5.0;
    let area = PI * radio * radio;
    println!("Área del círculo (radio {}): {:.2}", radio, area);
    
    // TODO: Define tu propia constante arriba y úsala aquí
    
    // -----------------------------------------
    // PARTE 5: Cuándo usar cada uno
    // -----------------------------------------
    println!("\n--- Cuándo usar ---");
    
    // const: valores que NUNCA cambian
    // - Configuración del programa
    // - Valores matemáticos (PI, E)
    // - Límites y umbrales
    
    // let: valores que se calculan en runtime
    // - Entrada del usuario
    // - Resultados de funciones
    // - Datos que varían
    
    // let mut: valores que cambian durante la ejecución
    // - Contadores
    // - Acumuladores
    // - Estados

    println!("\n✅ Práctica completada");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shadowing_valor() {
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        assert_eq!(x, 12);
    }

    #[test]
    fn test_shadowing_tipo() {
        let texto = "hola";
        let texto = texto.len();
        assert_eq!(texto, 4);
    }

    #[test]
    fn test_shadowing_scope() {
        let x = 1;
        {
            let x = 2;
            assert_eq!(x, 2);
        }
        assert_eq!(x, 1);  // Fuera del scope, valor original
    }

    #[test]
    fn test_constantes() {
        assert!((PI - 3.14159).abs() < 0.001);
        assert_eq!(MAX_INTENTOS, 3);
    }

    #[test]
    fn test_area_circulo() {
        let radio = 2.0;
        let area = PI * radio * radio;
        assert!((area - 12.566).abs() < 0.01);
    }
}
