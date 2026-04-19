//! Tests de integracion para el almacen de usuarios

use practice_02_integration::{UserStore, User};

mod common;

#[test]
fn test_full_flow_add_find() {
    let mut store = UserStore::new();

    // Agregar usuarios
    let id1 = store.add("Juan", "juan@email.com");
    let id2 = store.add("Ana", "ana@email.com");

    // Buscar por ID
    let user1 = store.find(id1).unwrap();
    assert_eq!(user1.name, "Juan");

    let user2 = store.find(id2).unwrap();
    assert_eq!(user2.name, "Ana");
}

#[test]
fn test_find_by_name() {
    let store = common::create_test_store();

    let results = store.find_by_name("garcia");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Ana Garcia");
}

#[test]
fn test_remove_user() {
    let mut store = common::create_test_store();
    let initial_total = store.total();

    let removed = store.remove(1);
    assert!(removed.is_some());
    assert_eq!(store.total(), initial_total - 1);
    assert!(store.find(1).is_none());
}

#[test]
fn test_list_active() {
    let store = common::create_test_store();
    let active = store.list_active();
    assert_eq!(active.len(), 3);
}

#[test]
fn test_user_not_found() {
    let store = UserStore::new();
    assert!(store.find(999).is_none());
}
