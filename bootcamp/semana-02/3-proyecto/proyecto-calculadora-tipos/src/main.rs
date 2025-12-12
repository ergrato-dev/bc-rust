// ============================================
// Proyecto Semanal: Calculadora de Tipos
// ============================================
// Semana 02: Variables y Tipos de Datos
// ============================================

use std::mem::size_of;

// ============================================
// CONSTANTES
// ============================================
const PI: f64 = 3.14159265358979;
const DIAS_POR_ANIO: u32 = 365;
const HORAS_POR_DIA: u32 = 24;
const MINUTOS_POR_HORA: u32 = 60;

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║   🧮 CALCULADORA DE TIPOS - RUST      ║");
    println!("║      Proyecto Semana 02               ║");
    println!("╚════════════════════════════════════════╝\n");

    // Ejecutar todas las demostraciones
    demo_geometria();
    demo_temperatura();
    demo_estadisticas();
    demo_explorador_tipos();
    demo_calculadora_edad();

    println!("\n✅ Proyecto completado");
}

// ============================================
// NIVEL 1: GEOMETRÍA
// ============================================
fn demo_geometria() {
    println!("═══ 📐 GEOMETRÍA ═══\n");

    // Rectángulo
    let base: f64 = 10.0;
    let altura: f64 = 5.0;
    
    let area_rect = calcular_area_rectangulo(base, altura);
    let perimetro_rect = calcular_perimetro_rectangulo(base, altura);
    
    println!("Rectángulo ({}x{}):", base, altura);
    println!("  Área: {:.2}", area_rect);
    println!("  Perímetro: {:.2}", perimetro_rect);

    // Círculo
    let radio: f64 = 7.0;
    
    let area_circ = calcular_area_circulo(radio);
    let circunferencia = calcular_circunferencia(radio);
    
    println!("\nCírculo (radio {}):", radio);
    println!("  Área: {:.2}", area_circ);
    println!("  Circunferencia: {:.2}", circunferencia);
    println!();
}

fn calcular_area_rectangulo(base: f64, altura: f64) -> f64 {
    base * altura
}

fn calcular_perimetro_rectangulo(base: f64, altura: f64) -> f64 {
    2.0 * (base + altura)
}

fn calcular_area_circulo(radio: f64) -> f64 {
    PI * radio * radio
}

fn calcular_circunferencia(radio: f64) -> f64 {
    2.0 * PI * radio
}

// ============================================
// NIVEL 1: TEMPERATURA
// ============================================
fn demo_temperatura() {
    println!("═══ 🌡️ TEMPERATURA ═══\n");

    let celsius: f64 = 25.0;
    let fahrenheit = celsius_a_fahrenheit(celsius);
    println!("{}°C = {:.1}°F", celsius, fahrenheit);

    let fahrenheit: f64 = 98.6;
    let celsius = fahrenheit_a_celsius(fahrenheit);
    println!("{}°F = {:.1}°C", fahrenheit, celsius);
    println!();
}

fn celsius_a_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fahrenheit_a_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// ============================================
// NIVEL 1: ESTADÍSTICAS
// ============================================
fn demo_estadisticas() {
    println!("═══ 📊 ESTADÍSTICAS ═══\n");

    // Usamos variables mutables para acumular
    let n1: i32 = 10;
    let n2: i32 = 25;
    let n3: i32 = 8;
    let n4: i32 = 42;
    let n5: i32 = 15;

    println!("Números: {}, {}, {}, {}, {}", n1, n2, n3, n4, n5);
    
    let suma = n1 + n2 + n3 + n4 + n5;
    let promedio = suma as f64 / 5.0;
    
    // Encontrar mínimo y máximo manualmente
    let mut minimo = n1;
    if n2 < minimo { minimo = n2; }
    if n3 < minimo { minimo = n3; }
    if n4 < minimo { minimo = n4; }
    if n5 < minimo { minimo = n5; }

    let mut maximo = n1;
    if n2 > maximo { maximo = n2; }
    if n3 > maximo { maximo = n3; }
    if n4 > maximo { maximo = n4; }
    if n5 > maximo { maximo = n5; }

    println!("  Suma: {}", suma);
    println!("  Promedio: {:.2}", promedio);
    println!("  Mínimo: {}", minimo);
    println!("  Máximo: {}", maximo);
    println!();
}

// ============================================
// NIVEL 2: EXPLORADOR DE TIPOS
// ============================================
fn demo_explorador_tipos() {
    println!("═══ 🔍 EXPLORADOR DE TIPOS ═══\n");

    println!("Tamaño en bytes:");
    println!("  bool:  {} byte", size_of::<bool>());
    println!("  char:  {} bytes", size_of::<char>());
    println!("  i8:    {} byte", size_of::<i8>());
    println!("  i16:   {} bytes", size_of::<i16>());
    println!("  i32:   {} bytes", size_of::<i32>());
    println!("  i64:   {} bytes", size_of::<i64>());
    println!("  i128:  {} bytes", size_of::<i128>());
    println!("  f32:   {} bytes", size_of::<f32>());
    println!("  f64:   {} bytes", size_of::<f64>());
    println!("  usize: {} bytes", size_of::<usize>());

    println!("\nRangos de enteros con signo:");
    println!("  i8:  {} a {}", i8::MIN, i8::MAX);
    println!("  i16: {} a {}", i16::MIN, i16::MAX);
    println!("  i32: {} a {}", i32::MIN, i32::MAX);

    println!("\nRangos de enteros sin signo:");
    println!("  u8:  {} a {}", u8::MIN, u8::MAX);
    println!("  u16: {} a {}", u16::MIN, u16::MAX);
    println!("  u32: {} a {}", u32::MIN, u32::MAX);
    println!();
}

// ============================================
// NIVEL 2: CALCULADORA DE EDAD
// ============================================
fn demo_calculadora_edad() {
    println!("═══ 🎂 CALCULADORA DE EDAD ═══\n");

    let edad_anios: u32 = 25;
    
    // Shadowing para conversiones
    let edad_dias = edad_anios * DIAS_POR_ANIO;
    let edad_horas = edad_dias * HORAS_POR_DIA;
    let edad_minutos = edad_horas * MINUTOS_POR_HORA;

    println!("Edad: {} años", edad_anios);
    println!("  En días: {} días", edad_dias);
    println!("  En horas: {} horas", edad_horas);
    println!("  En minutos: {} minutos", edad_minutos);
    
    // Usando u64 para números más grandes
    let edad_segundos: u64 = edad_minutos as u64 * 60;
    println!("  En segundos: {} segundos", edad_segundos);
    println!();
}

// ============================================
// TESTS
// ============================================
#[cfg(test)]
mod tests {
    use super::*;

    // Tests de geometría
    #[test]
    fn test_area_rectangulo() {
        assert!((calcular_area_rectangulo(10.0, 5.0) - 50.0).abs() < 0.001);
    }

    #[test]
    fn test_perimetro_rectangulo() {
        assert!((calcular_perimetro_rectangulo(10.0, 5.0) - 30.0).abs() < 0.001);
    }

    #[test]
    fn test_area_circulo() {
        let area = calcular_area_circulo(1.0);
        assert!((area - PI).abs() < 0.001);
    }

    #[test]
    fn test_circunferencia() {
        let circ = calcular_circunferencia(1.0);
        assert!((circ - 2.0 * PI).abs() < 0.001);
    }

    // Tests de temperatura
    #[test]
    fn test_celsius_a_fahrenheit() {
        assert!((celsius_a_fahrenheit(0.0) - 32.0).abs() < 0.001);
        assert!((celsius_a_fahrenheit(100.0) - 212.0).abs() < 0.001);
    }

    #[test]
    fn test_fahrenheit_a_celsius() {
        assert!((fahrenheit_a_celsius(32.0) - 0.0).abs() < 0.001);
        assert!((fahrenheit_a_celsius(212.0) - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_conversion_ida_vuelta() {
        let original = 25.0;
        let convertido = fahrenheit_a_celsius(celsius_a_fahrenheit(original));
        assert!((original - convertido).abs() < 0.001);
    }

    // Tests de tipos
    #[test]
    fn test_tamano_tipos() {
        assert_eq!(size_of::<i8>(), 1);
        assert_eq!(size_of::<i32>(), 4);
        assert_eq!(size_of::<i64>(), 8);
        assert_eq!(size_of::<char>(), 4);
    }

    #[test]
    fn test_rangos() {
        assert_eq!(i8::MIN, -128);
        assert_eq!(i8::MAX, 127);
        assert_eq!(u8::MIN, 0);
        assert_eq!(u8::MAX, 255);
    }
}
