// Práctica 03: Estructura de Archivos
// Semana 07 - Módulos y Crates
//
// Este archivo es el punto de entrada (crate root)
// Declara los módulos externos

mod productos;
mod ventas;

// Re-importamos para uso más cómodo
use productos::Producto;
use ventas::carrito::Carrito;
use ventas::factura::Factura;

fn main() {
    println!("=== Práctica 03: Estructura de Archivos ===\n");

    // Crear productos
    let mut laptop = Producto::nuevo(1, "Laptop Gaming", 999.99);
    let mut mouse = Producto::nuevo(2, "Mouse Inalámbrico", 29.99);
    let mut teclado = Producto::nuevo(3, "Teclado Mecánico", 79.99);

    // Agregar stock
    laptop.agregar_stock(10);
    mouse.agregar_stock(50);
    teclado.agregar_stock(25);

    println!("--- Catálogo de Productos ---");
    println!("{:?}", laptop);
    println!("{:?}", mouse);
    println!("{:?}", teclado);

    // Crear carrito y agregar productos
    println!("\n--- Carrito de Compras ---");
    let mut carrito = Carrito::nuevo();

    carrito.agregar(laptop.clone(), 1);
    carrito.agregar(mouse.clone(), 2);
    carrito.agregar(teclado.clone(), 1);

    println!("{}", carrito.resumen());

    // Calcular precio con módulo de precios
    println!("\n--- Cálculo de Precios ---");
    let precio_laptop = productos::calcular_precio_final(&laptop, 1);
    println!(
        "Laptop (con {}% IVA): {:.2}€",
        productos::precio::IVA * 100.0,
        precio_laptop
    );

    let precio_con_descuento = productos::precio::aplicar_descuento(precio_laptop, 10.0);
    println!("Laptop (con 10% descuento): {:.2}€", precio_con_descuento);

    // Generar factura
    println!("\n--- Factura ---");
    let factura = Factura::desde_carrito(&carrito, "Cliente Ejemplo");
    println!("{}", factura.generar());

    // Reducir stock después de la venta
    println!("\n--- Actualizando Stock ---");
    laptop.reducir_stock(1).unwrap();
    mouse.reducir_stock(2).unwrap();
    teclado.reducir_stock(1).unwrap();

    println!("Stock laptop: {}", laptop.cantidad);
    println!("Stock mouse: {}", mouse.cantidad);
    println!("Stock teclado: {}", teclado.cantidad);
}
