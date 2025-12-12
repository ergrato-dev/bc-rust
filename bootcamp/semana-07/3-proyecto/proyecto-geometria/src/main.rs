// src/main.rs
// Demostración de la biblioteca de geometría

use proyecto_geometria::calculos::{area, perimetro};
use proyecto_geometria::formato;
use proyecto_geometria::{Circulo, Forma, Rectangulo, Triangulo};

fn main() {
    println!("╔═══════════════════════════════════════════╗");
    println!("║   Biblioteca de Geometría Modular         ║");
    println!("║   Semana 07 - Módulos y Crates            ║");
    println!("╚═══════════════════════════════════════════╝\n");

    // Crear formas
    let circulo = Circulo::nuevo(5.0);
    let rectangulo = Rectangulo::nuevo(4.0, 3.0);
    let cuadrado = Rectangulo::cuadrado(5.0);
    let triangulo = Triangulo::nuevo(3.0, 4.0, 5.0);
    let equilatero = Triangulo::equilatero(6.0);

    // Mostrar información de cada forma
    println!("=== Formas Creadas ===\n");

    mostrar_info(&circulo);
    mostrar_info(&rectangulo);
    mostrar_info(&cuadrado);
    mostrar_info(&triangulo);
    mostrar_info(&equilatero);

    // Calcular áreas y perímetros
    println!("\n=== Cálculos ===\n");

    let datos = vec![
        (
            circulo.nombre(),
            area::area_circulo(&circulo),
            perimetro::perimetro_circulo(&circulo),
        ),
        (
            rectangulo.nombre(),
            area::area_rectangulo(&rectangulo),
            perimetro::perimetro_rectangulo(&rectangulo),
        ),
        (
            cuadrado.nombre(),
            area::area_rectangulo(&cuadrado),
            perimetro::perimetro_rectangulo(&cuadrado),
        ),
        (
            triangulo.nombre(),
            area::area_triangulo(&triangulo),
            perimetro::perimetro_triangulo(&triangulo),
        ),
        (
            equilatero.nombre(),
            area::area_triangulo(&equilatero),
            perimetro::perimetro_triangulo(&equilatero),
        ),
    ];

    // Mostrar tabla formateada
    println!("{}", formato::formatear_tabla(&datos));

    // Mostrar con unidades
    println!("\n=== Resultados con Unidades ===\n");
    let area_circulo = area::area_circulo(&circulo);
    let perim_circulo = perimetro::perimetro_circulo(&circulo);

    println!(
        "Círculo (radio = 5 cm):",
    );
    println!("  Área: {}", formato::formatear_area(area_circulo, "cm"));
    println!(
        "  Circunferencia: {}",
        formato::formatear_con_unidad(perim_circulo, "cm")
    );

    // Validación de formas
    println!("\n=== Validación de Formas ===\n");

    let triangulo_invalido = Triangulo::nuevo(1.0, 2.0, 10.0);
    let circulo_invalido = Circulo::nuevo(-5.0);

    validar_y_mostrar(&circulo);
    validar_y_mostrar(&rectangulo);
    validar_y_mostrar(&triangulo);
    validar_y_mostrar(&triangulo_invalido);
    validar_y_mostrar(&circulo_invalido);

    println!("\n=== Fin de la demostración ===");
}

fn mostrar_info<T: Forma + std::fmt::Debug>(forma: &T) {
    println!("{}: {:?}", forma.nombre(), forma);
}

fn validar_y_mostrar<T: Forma>(forma: &T) {
    let estado = if forma.es_valida() {
        "✓ Válida"
    } else {
        "✗ Inválida"
    };
    println!("{}: {}", forma.nombre(), estado);
}
