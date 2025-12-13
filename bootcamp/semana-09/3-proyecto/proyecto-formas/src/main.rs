//! # Demo: Sistema de Formas Geométricas
//!
//! Demostración del uso de traits con formas geométricas.

use proyecto_formas::*;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║        🎨 Sistema de Formas Geométricas con Traits         ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    
    // =========================================================================
    // 1. Creación de Formas
    // =========================================================================
    println!("📦 1. CREACIÓN DE FORMAS");
    println!("─────────────────────────────────────────────────────────────\n");
    
    let circulo = Circulo::new(5.0);
    let rectangulo = Rectangulo::new(10.0, 6.0);
    let cuadrado = Cuadrado::new(4.0);
    let triangulo = Triangulo::rectangulo(3.0, 4.0);
    
    println!("Formas creadas:");
    println!("  • {}", circulo);
    println!("  • {}", rectangulo);
    println!("  • {}", cuadrado);
    println!("  • {}", triangulo);
    
    // =========================================================================
    // 2. Trait Forma
    // =========================================================================
    println!("\n📐 2. TRAIT FORMA - Área y Perímetro");
    println!("─────────────────────────────────────────────────────────────\n");
    
    imprimir_forma(&circulo);
    println!();
    imprimir_forma(&rectangulo);
    println!();
    imprimir_forma(&cuadrado);
    println!();
    imprimir_forma(&triangulo);
    
    // =========================================================================
    // 3. Trait Dibujable
    // =========================================================================
    println!("\n🎨 3. TRAIT DIBUJABLE - ASCII Art");
    println!("─────────────────────────────────────────────────────────────\n");
    
    let canvas = Canvas::con_titulo(25, 15, "Cuadrado");
    println!("{}", canvas.dibujar(&cuadrado));
    
    let canvas2 = Canvas::con_titulo(30, 15, "Triángulo");
    println!("{}", canvas2.dibujar(&triangulo));
    
    // =========================================================================
    // 4. Trait Transformable
    // =========================================================================
    println!("🔄 4. TRAIT TRANSFORMABLE - Escalado");
    println!("─────────────────────────────────────────────────────────────\n");
    
    let mut circulo_escalable = Circulo::new(3.0);
    println!("Círculo original: radio = {}", circulo_escalable.radio);
    println!("  Área: {:.2}", circulo_escalable.area());
    
    circulo_escalable.escalar(2.0);
    println!("\nDespués de escalar x2: radio = {}", circulo_escalable.radio);
    println!("  Área: {:.2}", circulo_escalable.area());
    
    // Usando escalada() que no modifica el original
    let original = Cuadrado::new(5.0);
    let escalado = original.escalada(3.0);
    println!("\nCuadrado original: lado = {}", original.lado);
    println!("Cuadrado escalado: lado = {}", escalado.lado);
    
    // =========================================================================
    // 5. Trait Posicionable
    // =========================================================================
    println!("\n📍 5. TRAIT POSICIONABLE - Movimiento");
    println!("─────────────────────────────────────────────────────────────\n");
    
    let mut forma_movil = Circulo::con_posicion(2.0, 0.0, 0.0);
    println!("Posición inicial: {:?}", forma_movil.posicion());
    
    forma_movil.mover(10.0, 5.0);
    println!("Después de mover(10, 5): {:?}", forma_movil.posicion());
    
    forma_movil.mover(-3.0, 2.0);
    println!("Después de mover(-3, 2): {:?}", forma_movil.posicion());
    
    // =========================================================================
    // 6. Polimorfismo con Trait Objects
    // =========================================================================
    println!("\n🔀 6. POLIMORFISMO - Trait Objects");
    println!("─────────────────────────────────────────────────────────────\n");
    
    let formas: Vec<&dyn Forma> = vec![
        &circulo,
        &rectangulo,
        &cuadrado,
        &triangulo,
    ];
    
    imprimir_formas(&formas);
    
    if let Some(mayor) = forma_mayor_area(&formas) {
        println!("\n🏆 Forma con mayor área: {} ({:.2})", 
            mayor.nombre(), 
            mayor.area());
    }
    
    println!("📊 Área total: {:.2}", area_total(&formas));
    
    // =========================================================================
    // 7. Comparación de Formas
    // =========================================================================
    println!("\n⚖️ 7. COMPARACIÓN DE FORMAS");
    println!("─────────────────────────────────────────────────────────────");
    
    Canvas::comparar(&circulo, &rectangulo);
    
    // =========================================================================
    // 8. Conversiones (From/Into)
    // =========================================================================
    println!("\n🔄 8. CONVERSIONES - From/Into");
    println!("─────────────────────────────────────────────────────────────\n");
    
    // Cuadrado -> Rectángulo
    let cuadrado_original = Cuadrado::new(7.0);
    let rectangulo_desde_cuadrado: Rectangulo = cuadrado_original.clone().into();
    println!("Cuadrado(lado=7) -> {}", rectangulo_desde_cuadrado);
    
    // f64 -> Círculo
    let circulo_desde_numero: Circulo = 4.0.into();
    println!("4.0 -> {}", circulo_desde_numero);
    
    // Tupla -> Rectángulo
    let rectangulo_desde_tupla: Rectangulo = (8.0, 3.0).into();
    println!("(8.0, 3.0) -> {}", rectangulo_desde_tupla);
    
    // =========================================================================
    // 9. Tipos de Triángulos
    // =========================================================================
    println!("\n📐 9. TIPOS DE TRIÁNGULOS");
    println!("─────────────────────────────────────────────────────────────\n");
    
    let equilatero = Triangulo::equilatero(5.0);
    let isosceles = Triangulo::isosceles(4.0, 5.0).unwrap();
    let rectangulo_tri = Triangulo::rectangulo(3.0, 4.0);
    
    println!("Equilátero: {} - es_equilatero: {}", equilatero, equilatero.es_equilatero());
    println!("Isósceles: {} - es_isosceles: {}", isosceles, isosceles.es_isosceles());
    println!("Rectángulo: {} - es_rectangulo: {}", rectangulo_tri, rectangulo_tri.es_rectangulo());
    
    // =========================================================================
    // 10. Funciones Genéricas con Trait Bounds
    // =========================================================================
    println!("\n🔧 10. FUNCIONES GENÉRICAS CON TRAIT BOUNDS");
    println!("─────────────────────────────────────────────────────────────\n");
    
    // Función que acepta cualquier Forma
    fn describir<F: Forma>(forma: &F) {
        println!("→ {} con área {:.2}", forma.nombre(), forma.area());
    }
    
    // Función con múltiples bounds
    fn info_completa<F>(forma: &F) 
    where 
        F: Forma + std::fmt::Display 
    {
        println!("→ Display: {}", forma);
        println!("  Área: {:.2}, Perímetro: {:.2}", forma.area(), forma.perimetro());
    }
    
    describir(&circulo);
    describir(&triangulo);
    
    println!();
    info_completa(&cuadrado);
    
    // =========================================================================
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║              ✅ Demostración Completada                     ║");
    println!("╚════════════════════════════════════════════════════════════╝");
}
