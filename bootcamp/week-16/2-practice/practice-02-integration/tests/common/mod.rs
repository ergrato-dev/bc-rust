//! Helpers compartidos para tests de integracion

use practice_02_integration::UserStore;

/// Crea un almacen con datos de prueba.
pub fn create_test_store() -> UserStore {
    let mut store = UserStore::new();
    store.add("Juan Perez", "juan@email.com");
    store.add("Ana Garcia", "ana@email.com");
    store.add("Carlos Lopez", "carlos@email.com");
    store
}
