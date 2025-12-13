//! Helpers compartidos para tests de integracion

use practica_02_integration::AlmacenUsuarios;

/// Crea un almacen con datos de prueba.
pub fn crear_almacen_prueba() -> AlmacenUsuarios {
    let mut almacen = AlmacenUsuarios::new();
    almacen.agregar("Juan Perez", "juan@email.com");
    almacen.agregar("Ana Garcia", "ana@email.com");
    almacen.agregar("Carlos Lopez", "carlos@email.com");
    almacen
}
