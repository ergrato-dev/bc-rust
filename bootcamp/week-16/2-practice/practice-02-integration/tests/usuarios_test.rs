//! Tests de integracion para el almacen de usuarios

use practica_02_integration::{AlmacenUsuarios, Usuario};

mod common;

#[test]
fn test_flujo_completo_agregar_buscar() {
    let mut almacen = AlmacenUsuarios::new();

    // Agregar usuarios
    let id1 = almacen.agregar("Juan", "juan@email.com");
    let id2 = almacen.agregar("Ana", "ana@email.com");

    // Buscar por ID
    let usuario1 = almacen.buscar(id1).unwrap();
    assert_eq!(usuario1.nombre, "Juan");

    let usuario2 = almacen.buscar(id2).unwrap();
    assert_eq!(usuario2.nombre, "Ana");
}

#[test]
fn test_buscar_por_nombre() {
    let almacen = common::crear_almacen_prueba();

    let resultados = almacen.buscar_por_nombre("garcia");
    assert_eq!(resultados.len(), 1);
    assert_eq!(resultados[0].nombre, "Ana Garcia");
}

#[test]
fn test_eliminar_usuario() {
    let mut almacen = common::crear_almacen_prueba();
    let total_inicial = almacen.total();

    let eliminado = almacen.eliminar(1);
    assert!(eliminado.is_some());
    assert_eq!(almacen.total(), total_inicial - 1);
    assert!(almacen.buscar(1).is_none());
}

#[test]
fn test_listar_activos() {
    let almacen = common::crear_almacen_prueba();
    let activos = almacen.listar_activos();
    assert_eq!(activos.len(), 3);
}

#[test]
fn test_usuario_no_encontrado() {
    let almacen = AlmacenUsuarios::new();
    assert!(almacen.buscar(999).is_none());
}
