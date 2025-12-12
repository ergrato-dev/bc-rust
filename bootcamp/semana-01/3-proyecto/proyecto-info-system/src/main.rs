// ============================================
// 🦀 Bootcamp Rust: Zero to Hero
// Semana 01 - Proyecto: Sistema de Información
// ============================================
//
// OBJETIVO:
// Crear un programa que muestre información
// personal y del bootcamp de forma organizada.
//
// EJECUTAR:
// $ cargo run
//
// ============================================

fn main() {
    mostrar_banner();
    mostrar_info_personal();
    mostrar_info_bootcamp();
    mostrar_estadisticas();
    mostrar_footer();
}

/// Muestra el banner de bienvenida
fn mostrar_banner() {
    println!("╔══════════════════════════════════════════╗");
    println!("║    🦀 BOOTCAMP RUST: ZERO TO HERO 🦀    ║");
    println!("╠══════════════════════════════════════════╣");
}

/// Muestra la información personal del estudiante
fn mostrar_info_personal() {
    // TODO: Personaliza con tu información
    let nombre = "Tu Nombre";
    let rol = "Desarrollador";
    let ubicacion = "Tu Ciudad";

    println!("║  Estudiante: {:<27}║", nombre);
    println!("║  Rol: {:<34}║", rol);
    println!("║  Ubicación: {:<28}║", ubicacion);
    println!("╠══════════════════════════════════════════╣");
}

/// Muestra el progreso del bootcamp
fn mostrar_info_bootcamp() {
    let semana_actual = 1;
    let total_semanas = 16;
    let tema = "Introducción a Rust";

    // Calcular progreso
    let progreso = (semana_actual * 100) / total_semanas;

    // Crear barra de progreso
    let barra = crear_barra_progreso(progreso, 16);

    println!("║  📅 Semana: {:02}/{}                        ║", semana_actual, total_semanas);
    println!("║  📚 Tema: {:<30}║", tema);
    println!("║  📊 Progreso: {} {:>3}%       ║", barra, progreso);
    println!("╠══════════════════════════════════════════╣");
}

/// Muestra estadísticas del día
fn mostrar_estadisticas() {
    let energia = 80;
    let horas_planeadas = 4;

    let barra_energia = crear_barra_progreso(energia, 10);

    println!("║  ⚡ Energía hoy: {} {:>3}%          ║", barra_energia, energia);
    println!("║  ⏰ Horas planeadas: {}                    ║", horas_planeadas);
}

/// Muestra el footer
fn mostrar_footer() {
    println!("╚══════════════════════════════════════════╝");
    println!();
    println!("  ¡A programar! 🚀");
}

/// Crea una barra de progreso visual
fn crear_barra_progreso(porcentaje: i32, longitud: i32) -> String {
    let llenos = (porcentaje * longitud) / 100;
    let vacios = longitud - llenos;

    let mut barra = String::new();

    for _ in 0..llenos {
        barra.push('▓');
    }

    for _ in 0..vacios {
        barra.push('░');
    }

    barra
}
