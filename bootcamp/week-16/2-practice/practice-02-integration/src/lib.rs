//! Biblioteca de gestion de usuarios
//!
//! Esta biblioteca proporciona funcionalidades para
//! crear, buscar y gestionar usuarios.

use std::collections::HashMap;

/// Representa un usuario del sistema.
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub active: bool,
}

impl User {
    /// Crea un nuevo usuario.
    pub fn new(id: u32, name: &str, email: &str) -> Self {
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
            active: true,
        }
    }

    /// Desactiva el usuario.
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

/// Almacen de usuarios en memoria.
#[derive(Debug, Default)]
pub struct UserStore {
    users: HashMap<u32, User>,
    next_id: u32,
}

impl UserStore {
    /// Crea un nuevo almacen vacio.
    pub fn new() -> Self {
        UserStore {
            users: HashMap::new(),
            next_id: 1,
        }
    }

    /// Agrega un usuario y retorna su ID.
    pub fn add(&mut self, name: &str, email: &str) -> u32 {
        let id = self.next_id;
        let user = User::new(id, name, email);
        self.users.insert(id, user);
        self.next_id += 1;
        id
    }

    /// Busca un usuario por ID.
    pub fn find(&self, id: u32) -> Option<&User> {
        self.users.get(&id)
    }

    /// Busca usuarios por nombre (parcial).
    pub fn find_by_name(&self, name: &str) -> Vec<&User> {
        self.users
            .values()
            .filter(|u| u.name.to_lowercase().contains(&name.to_lowercase()))
            .collect()
    }

    /// Elimina un usuario.
    pub fn remove(&mut self, id: u32) -> Option<User> {
        self.users.remove(&id)
    }

    /// Retorna el total de usuarios.
    pub fn total(&self) -> usize {
        self.users.len()
    }

    /// Lista todos los usuarios activos.
    pub fn list_active(&self) -> Vec<&User> {
        self.users.values().filter(|u| u.active).collect()
    }
}

// Tests unitarios
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        let user = User::new(1, "Juan", "juan@email.com");
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Juan");
        assert!(user.active);
    }

    #[test]
    fn test_deactivate_user() {
        let mut user = User::new(1, "Juan", "juan@email.com");
        user.deactivate();
        assert!(!user.active);
    }

    #[test]
    fn test_store_add() {
        let mut store = UserStore::new();
        let id = store.add("Ana", "ana@email.com");
        assert_eq!(id, 1);
        assert_eq!(store.total(), 1);
    }
}
