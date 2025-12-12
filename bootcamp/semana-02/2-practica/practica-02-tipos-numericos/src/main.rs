// ============================================
// Práctica 02: Tipos Numéricos
// ============================================
// Objetivo: Dominar enteros y flotantes
// ============================================

fn main() {
    println!("=== Práctica 02: Tipos Numéricos ===\n");

    // -----------------------------------------
    // PARTE 1: Enteros con Signo
    // -----------------------------------------
    println!("--- Enteros con Signo ---");
    
    // i8: -128 a 127
    let byte_pequeno: i8 = 100;
    println!("i8: {}", byte_pequeno);
    
    // TODO: Declara variables i16, i32, i64
    // let mediano: i16 = ...;
    // let normal: i32 = ...;     // tipo por defecto
    // let grande: i64 = ...;
    
    // Usando sufijos de tipo
    let con_sufijo = 42i32;
    println!("Con sufijo: {}", con_sufijo);

    // -----------------------------------------
    // PARTE 2: Enteros sin Signo
    // -----------------------------------------
    println!("\n--- Enteros sin Signo ---");
    
    // u8: 0 a 255 (ideal para bytes, colores RGB)
    let color_rojo: u8 = 255;
    let color_verde: u8 = 128;
    let color_azul: u8 = 0;
    println!("RGB: ({}, {}, {})", color_rojo, color_verde, color_azul);
    
    // TODO: Declara un contador u32
    // let contador: u32 = ...;
    
    // usize: tamaño depende de la arquitectura (32/64 bits)
    // Usado para índices y tamaños
    let indice: usize = 0;
    println!("Índice: {}", indice);

    // -----------------------------------------
    // PARTE 3: Flotantes
    // -----------------------------------------
    println!("\n--- Flotantes ---");
    
    // f64 es el tipo por defecto (más precisión)
    let pi: f64 = 3.14159265358979;
    println!("PI (f64): {}", pi);
    
    // f32 tiene menos precisión pero ocupa menos memoria
    let pi_32: f32 = 3.14159265358979;
    println!("PI (f32): {}", pi_32);  // Observa la pérdida de precisión
    
    // TODO: Calcula el área de un círculo con radio 5
    let radio: f64 = 5.0;
    // let area = ...;
    // println!("Área: {}", area);

    // -----------------------------------------
    // PARTE 4: Operaciones
    // -----------------------------------------
    println!("\n--- Operaciones ---");
    
    let a = 10;
    let b = 3;
    
    println!("Suma: {} + {} = {}", a, b, a + b);
    println!("Resta: {} - {} = {}", a, b, a - b);
    println!("Multiplicación: {} * {} = {}", a, b, a * b);
    println!("División entera: {} / {} = {}", a, b, a / b);
    println!("Módulo: {} % {} = {}", a, b, a % b);
    
    // División flotante
    let x = 10.0;
    let y = 3.0;
    println!("División flotante: {} / {} = {}", x, y, x / y);

    // -----------------------------------------
    // PARTE 5: Literales Numéricos
    // -----------------------------------------
    println!("\n--- Literales Numéricos ---");
    
    let decimal = 1_000_000;      // Separador visual
    let hex = 0xFF;               // Hexadecimal
    let octal = 0o77;             // Octal
    let binario = 0b1111_0000;    // Binario
    
    println!("Decimal: {}", decimal);
    println!("Hex 0xFF: {}", hex);
    println!("Octal 0o77: {}", octal);
    println!("Binario: {}", binario);

    println!("\n✅ Práctica completada");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rangos_i8() {
        let min: i8 = -128;
        let max: i8 = 127;
        assert_eq!(min, i8::MIN);
        assert_eq!(max, i8::MAX);
    }

    #[test]
    fn test_rangos_u8() {
        let min: u8 = 0;
        let max: u8 = 255;
        assert_eq!(min, u8::MIN);
        assert_eq!(max, u8::MAX);
    }

    #[test]
    fn test_division_entera() {
        assert_eq!(10 / 3, 3);  // División entera trunca
        assert_eq!(10 % 3, 1);  // Módulo
    }

    #[test]
    fn test_division_flotante() {
        let resultado = 10.0 / 3.0;
        assert!((resultado - 3.333333).abs() < 0.001);
    }

    #[test]
    fn test_area_circulo() {
        let pi = 3.14159;
        let radio = 5.0;
        let area = pi * radio * radio;
        assert!((area - 78.5397).abs() < 0.01);
    }
}
