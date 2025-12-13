//! # Sistema de Gestión de Inventario
//!
//! Demostración del uso de colecciones en Rust.

use proyecto_inventario::{Inventario, GeneradorReportes};

fn main() {
    println!("🦀 Sistema de Gestión de Inventario\n");
    println!("═".repeat(50));

    // Crear inventario
    let mut inventario = crear_inventario_demo();

    // Mostrar estado inicial
    let reportes = GeneradorReportes::new(&inventario);
    println!("{}", reportes.reporte_resumen());

    // Realizar algunas operaciones
    println!("\n📦 OPERACIONES DE INVENTARIO");
    println!("─".repeat(50));

    // Entrada de stock
    println!("\n➕ Entrada de stock: 20 laptops");
    inventario.entrada_stock(1, 20, "Pedido #1234");

    // Salida de stock
    println!("➖ Salida de stock: 5 laptops (venta)");
    inventario.salida_stock(1, 5, "Venta cliente ABC");

    println!("➖ Salida de stock: 10 mouses (venta)");
    inventario.salida_stock(2, 10, "Venta mayorista");

    // Búsquedas
    println!("\n🔍 BÚSQUEDAS");
    println!("─".repeat(50));

    println!("\nBuscar 'lap':");
    for p in inventario.buscar_por_nombre("lap") {
        println!("   {}", p);
    }

    println!("\nCategoría 'Electrónica':");
    for p in inventario.buscar_por_categoria("Electrónica") {
        println!("   {}", p);
    }

    // Agregar nuevo producto
    println!("\n➕ Agregando nuevo producto...");
    let nuevo_id = inventario.agregar_producto(
        "Webcam HD",
        "Cámara 1080p con micrófono",
        79.99,
        "Electrónica",
        25,
    );
    println!("   Producto agregado con ID: {}", nuevo_id);

    // Reporte completo
    let reportes = GeneradorReportes::new(&inventario);
    println!("{}", reportes.reporte_completo());

    println!("\n✅ Demo completada");
}

fn crear_inventario_demo() -> Inventario {
    let mut inv = Inventario::new();

    // Electrónica
    inv.agregar_producto(
        "Laptop Pro",
        "Laptop 15\" i7 16GB RAM",
        1299.99,
        "Electrónica",
        10,
    );
    inv.agregar_producto(
        "Mouse Gamer",
        "Mouse RGB 16000 DPI",
        49.99,
        "Electrónica",
        50,
    );
    inv.agregar_producto(
        "Teclado Mecánico",
        "Teclado Cherry MX Blue",
        129.99,
        "Electrónica",
        3, // Stock bajo
    );
    inv.agregar_producto(
        "Monitor 27\"",
        "Monitor 4K IPS",
        399.99,
        "Electrónica",
        8,
    );

    // Muebles
    inv.agregar_producto(
        "Silla Ergonómica",
        "Silla con soporte lumbar",
        299.99,
        "Muebles",
        5,
    );
    inv.agregar_producto(
        "Escritorio Ajustable",
        "Escritorio standing desk",
        449.99,
        "Muebles",
        2, // Stock bajo
    );

    // Oficina
    inv.agregar_producto(
        "Cuaderno A4",
        "Pack de 5 cuadernos",
        9.99,
        "Oficina",
        100,
    );
    inv.agregar_producto(
        "Bolígrafos",
        "Pack de 20 bolígrafos",
        7.99,
        "Oficina",
        0, // Sin stock!
    );

    inv
}
