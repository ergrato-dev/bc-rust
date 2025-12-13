//! Biblioteca de gestion de usuarios
//!
//! Esta biblioteca proporciona funcionalidades para
//! crear, buscar y gestionar usuarios.

use std::collections::HashMap;

/// Representa un usuario del sistema.
#[derive(Debug, Clone, PartialEq)]
pub struct Usuario {
    pub id: u32,
    pub nombre: String,
    pub email: String,
    pub activo: bool,
}

impl Usuario {
    /// Crea un nuevo usuario.
    pub fn new(id: u32, nombre: &str, email: &str) -> Self {
        Usuario {
            id,
            nombre: nombre.to_string(),
            email: email.to_string(),
            activo: true,
        }
    }

    /// Desactiva el usuario.
    pub fn desactivar(&mut self) {
        self.activo = false;
    }
}

/// Almacen de usuarios en memoria.
#[derive(Debug, Default)]
pub struct AlmacenUsuarios {
    usuarios: HashMap<u32, Usuario>,
    siguiente_id: u32,
}

impl AlmacenUsuarios {
    /// Crea un nuevo almacen vacio.
    pub fn new() -> Self {
        AlmacenUsuarios {
            usuarios: HashMap::new(),
            siguiente_id: 1,
        }
    }

    /// Agrega un usuario y retorna su ID.
    pub fn agregar(&mut self, nombre: &str, email: &str) -> u32 {
        let id = self.siguiente_id;
        let usuario = Usuario::new(id, nombre, email);
        self.usuarios.insert(id, usuario);
        self.siguiente_id += 1;
        id
    }

    /// Busca un usuario por ID.
    pub fn buscar(&self, id: u32) -> Option<&Usuario> {
        self.usuarios.get(&id)
    }

    /// Busca usuarios por nombre (parcial).
    pub fn buscar_por_nombre(&self, nombre: &str) -> Vec<&Usuario> {
        self.usuarios
            .values()
            .filter(|u| u.nombre.to_lowercase().contains(&nombre.to_lowercase()))
            .collect()
    }

    /// Elimina un usuario.
    pub fn eliminar(&mut self, id: u32) -> Option<Usuario> {
        self.usuarios.remove(&id)
    }

    /// Retorna el total de usuarios.
    pub fn total(&self) -> usize {
        self.usuarios.len()
    }

    /// Lista todos los usuarios activos.
    pub fn listar_activos(&self) -> Vec<&Usuario> {
        self.usuarios.values().filter(|u| u.activo).collect()
    }
}

// Tests unitarios
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_usuario() {
        let usuario = Usuario::new(1, "Juan", "juan@email.com");
        assert_eq!(usuario.id, 1);
        assert_eq!(usuario.nombre, "Juan");
        assert!(usuario.activo);
    }

    #[test]
    fn test_desactivar_usuario() {
        let mut usuario = Usuario::new(1, "Juan", "juan@email.com");
        usuario.desactivar();
        assert!(!usuario.activo);
    }

    #[test]
    fn test_almacen_agregar() {
        let mut almacen = AlmacenUsuarios::new();
        let id = almacen.agregar("Ana", "ana@email.com");
        assert_eq!(id, 1);
        assert_eq!(almacen.total(), 1);
    }
}
