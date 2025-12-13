//! Caché LRU (Least Recently Used) simplificado.
//!
//! Cuando la caché está llena, elimina el elemento menos recientemente usado.
//!
//! # Ejemplo
//!
//! ```
//! use proyecto_contenedor::Cache;
//!
//! let mut cache = Cache::new(2);
//! cache.insertar("a", 1);
//! cache.insertar("b", 2);
//! cache.obtener(&"a"); // "a" ahora es el más reciente
//! cache.insertar("c", 3); // Elimina "b" (LRU)
//! assert!(!cache.contiene(&"b"));
//! ```

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// Caché con política de reemplazo LRU.
///
/// `K` es el tipo de clave, `V` es el tipo de valor.
pub struct Cache<K, V> {
    capacidad: usize,
    datos: HashMap<K, V>,
    orden: Vec<K>, // El último elemento es el más recientemente usado
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
{
    /// Crea una nueva caché con la capacidad especificada.
    ///
    /// # Panics
    ///
    /// Panics si la capacidad es 0.
    pub fn new(capacidad: usize) -> Self {
        assert!(capacidad > 0, "La capacidad debe ser mayor a 0");
        Cache {
            capacidad,
            datos: HashMap::with_capacity(capacidad),
            orden: Vec::with_capacity(capacidad),
        }
    }

    /// Inserta un par clave-valor en la caché.
    ///
    /// Si la caché está llena, elimina el elemento menos recientemente usado.
    /// Si la clave ya existe, actualiza el valor y la marca como reciente.
    pub fn insertar(&mut self, clave: K, valor: V) {
        if self.datos.contains_key(&clave) {
            // Actualizar valor existente y mover al final (más reciente)
            self.datos.insert(clave.clone(), valor);
            self.mover_al_final(&clave);
        } else {
            // Nuevo elemento
            if self.datos.len() >= self.capacidad {
                // Eliminar el menos recientemente usado (primero en orden)
                if let Some(lru_key) = self.orden.first().cloned() {
                    self.datos.remove(&lru_key);
                    self.orden.remove(0);
                }
            }
            self.datos.insert(clave.clone(), valor);
            self.orden.push(clave);
        }
    }

    /// Obtiene una referencia al valor asociado a la clave.
    ///
    /// Si existe, marca la clave como recientemente usada.
    pub fn obtener(&mut self, clave: &K) -> Option<&V> {
        if self.datos.contains_key(clave) {
            self.mover_al_final(clave);
            self.datos.get(clave)
        } else {
            None
        }
    }

    /// Verifica si la caché contiene la clave.
    ///
    /// **Nota**: Esta operación NO actualiza el orden LRU.
    pub fn contiene(&self, clave: &K) -> bool {
        self.datos.contains_key(clave)
    }

    /// Devuelve la cantidad de elementos en la caché.
    pub fn len(&self) -> usize {
        self.datos.len()
    }

    /// Verifica si la caché está vacía.
    pub fn esta_vacia(&self) -> bool {
        self.datos.is_empty()
    }

    /// Devuelve la capacidad máxima de la caché.
    pub fn capacidad(&self) -> usize {
        self.capacidad
    }

    /// Elimina un elemento de la caché.
    pub fn remover(&mut self, clave: &K) -> Option<V> {
        if let Some(valor) = self.datos.remove(clave) {
            self.orden.retain(|k| k != clave);
            Some(valor)
        } else {
            None
        }
    }

    /// Limpia todos los elementos de la caché.
    pub fn limpiar(&mut self) {
        self.datos.clear();
        self.orden.clear();
    }

    // Método auxiliar para mover una clave al final del orden
    fn mover_al_final(&mut self, clave: &K) {
        self.orden.retain(|k| k != clave);
        self.orden.push(clave.clone());
    }
}

impl<K, V> Default for Cache<K, V>
where
    K: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self::new(16) // Capacidad por defecto razonable
    }
}

impl<K, V> Debug for Cache<K, V>
where
    K: Eq + Hash + Clone + Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cache")
            .field("capacidad", &self.capacidad)
            .field("len", &self.datos.len())
            .field("datos", &self.datos)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cache: Cache<String, i32> = Cache::new(10);
        assert!(cache.esta_vacia());
        assert_eq!(cache.capacidad(), 10);
    }

    #[test]
    #[should_panic(expected = "capacidad debe ser mayor a 0")]
    fn test_new_capacidad_cero() {
        let _cache: Cache<i32, i32> = Cache::new(0);
    }

    #[test]
    fn test_insertar_obtener() {
        let mut cache = Cache::new(3);
        cache.insertar("a", 1);
        cache.insertar("b", 2);
        cache.insertar("c", 3);

        assert_eq!(cache.obtener(&"a"), Some(&1));
        assert_eq!(cache.obtener(&"b"), Some(&2));
        assert_eq!(cache.obtener(&"c"), Some(&3));
        assert_eq!(cache.obtener(&"d"), None);
    }

    #[test]
    fn test_lru_eviction() {
        let mut cache = Cache::new(2);
        cache.insertar("a", 1);
        cache.insertar("b", 2);

        // "a" es el LRU, "b" es el más reciente
        cache.insertar("c", 3); // Debería eliminar "a"

        assert!(!cache.contiene(&"a"));
        assert!(cache.contiene(&"b"));
        assert!(cache.contiene(&"c"));
    }

    #[test]
    fn test_lru_access_updates_order() {
        let mut cache = Cache::new(2);
        cache.insertar("a", 1);
        cache.insertar("b", 2);

        // Acceder a "a" lo hace el más reciente
        cache.obtener(&"a");

        // Ahora "b" es el LRU
        cache.insertar("c", 3); // Debería eliminar "b"

        assert!(cache.contiene(&"a"));
        assert!(!cache.contiene(&"b"));
        assert!(cache.contiene(&"c"));
    }

    #[test]
    fn test_update_existing() {
        let mut cache = Cache::new(2);
        cache.insertar("a", 1);
        cache.insertar("b", 2);

        // Actualizar "a" con nuevo valor
        cache.insertar("a", 100);

        assert_eq!(cache.obtener(&"a"), Some(&100));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_contiene() {
        let mut cache = Cache::new(2);
        assert!(!cache.contiene(&"a"));

        cache.insertar("a", 1);
        assert!(cache.contiene(&"a"));
        assert!(!cache.contiene(&"b"));
    }

    #[test]
    fn test_len() {
        let mut cache = Cache::new(5);
        assert_eq!(cache.len(), 0);

        cache.insertar("a", 1);
        assert_eq!(cache.len(), 1);

        cache.insertar("b", 2);
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_remover() {
        let mut cache = Cache::new(3);
        cache.insertar("a", 1);
        cache.insertar("b", 2);

        assert_eq!(cache.remover(&"a"), Some(1));
        assert!(!cache.contiene(&"a"));
        assert_eq!(cache.len(), 1);

        assert_eq!(cache.remover(&"c"), None);
    }

    #[test]
    fn test_limpiar() {
        let mut cache = Cache::new(3);
        cache.insertar("a", 1);
        cache.insertar("b", 2);
        cache.insertar("c", 3);

        cache.limpiar();
        assert!(cache.esta_vacia());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_con_tipos_complejos() {
        let mut cache: Cache<String, Vec<i32>> = Cache::new(2);
        cache.insertar("nums".to_string(), vec![1, 2, 3]);
        cache.insertar("more".to_string(), vec![4, 5]);

        assert_eq!(cache.obtener(&"nums".to_string()), Some(&vec![1, 2, 3]));
    }
}
