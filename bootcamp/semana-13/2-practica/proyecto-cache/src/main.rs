//! Proyecto Final: Sistema de Cache LRU con Smart Pointers
//!
//! Implementa un cache LRU (Least Recently Used) usando:
//! - Rc<RefCell<T>> para nodos mutables compartidos
//! - Weak<T> para referencias bidireccionales
//! - HashMap para acceso O(1)

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::{Rc, Weak};

fn main() {
    println!("=== Proyecto: Cache LRU ===\n");

    demo_cache_basico();
    demo_cache_con_capacidad();
    demo_cache_estadisticas();

    println!("\n✅ Proyecto completado!");
}

// ============================================================
// ESTRUCTURAS
// ============================================================

/// Nodo de la lista doblemente enlazada
struct Nodo<K, V> {
    key: K,
    value: V,
    prev: RefCell<Weak<Nodo<K, V>>>,
    next: RefCell<Option<Rc<Nodo<K, V>>>>,
}

/// Cache LRU con capacidad limitada
struct LruCache<K, V> {
    capacidad: usize,
    mapa: RefCell<HashMap<K, Rc<Nodo<K, V>>>>,
    head: RefCell<Option<Rc<Nodo<K, V>>>>,  // Más reciente
    tail: RefCell<Weak<Nodo<K, V>>>,        // Menos reciente
    hits: RefCell<usize>,
    misses: RefCell<usize>,
}

impl<K, V> Nodo<K, V> {
    fn new(key: K, value: V) -> Rc<Self> {
        Rc::new(Nodo {
            key,
            value,
            prev: RefCell::new(Weak::new()),
            next: RefCell::new(None),
        })
    }
}

impl<K, V> LruCache<K, V>
where
    K: Eq + Hash + Clone + Debug,
    V: Clone + Debug,
{
    /// Crea un nuevo cache con capacidad especificada
    fn new(capacidad: usize) -> Self {
        LruCache {
            capacidad,
            mapa: RefCell::new(HashMap::new()),
            head: RefCell::new(None),
            tail: RefCell::new(Weak::new()),
            hits: RefCell::new(0),
            misses: RefCell::new(0),
        }
    }

    /// Obtiene un valor del cache
    fn get(&self, key: &K) -> Option<V> {
        let mapa = self.mapa.borrow();
        
        if let Some(nodo) = mapa.get(key) {
            *self.hits.borrow_mut() += 1;
            
            // Mover al frente (más reciente)
            let nodo_clone = Rc::clone(nodo);
            drop(mapa); // Liberar borrow antes de modificar
            self.mover_al_frente(&nodo_clone);
            
            Some(nodo_clone.value.clone())
        } else {
            *self.misses.borrow_mut() += 1;
            None
        }
    }

    /// Inserta o actualiza un valor en el cache
    fn put(&self, key: K, value: V) {
        // Si ya existe, remover el nodo viejo
        let existia = {
            let mapa = self.mapa.borrow();
            mapa.contains_key(&key)
        };
        
        if existia {
            if let Some(nodo_viejo) = self.mapa.borrow_mut().remove(&key) {
                self.remover_nodo(&nodo_viejo);
            }
        }
        
        // Verificar capacidad y remover LRU si es necesario
        let necesita_evict = {
            self.mapa.borrow().len() >= self.capacidad
        };
        
        if necesita_evict {
            // Obtener la key del tail para remover
            let tail_key = {
                self.tail.borrow().upgrade().map(|t| t.key.clone())
            };
            
            if let Some(k) = tail_key {
                if let Some(tail) = self.mapa.borrow_mut().remove(&k) {
                    self.remover_nodo(&tail);
                }
            }
        }
        
        // Crear nuevo nodo
        let nodo = Nodo::new(key.clone(), value);
        
        // Agregar al frente
        self.agregar_al_frente(&nodo);
        
        // Agregar al mapa
        self.mapa.borrow_mut().insert(key, nodo);
    }

    /// Agrega un nodo al frente (más reciente)
    fn agregar_al_frente(&self, nodo: &Rc<Nodo<K, V>>) {
        let head_actual = self.head.borrow().clone();
        
        // Nuevo nodo apunta al head actual
        *nodo.next.borrow_mut() = head_actual.clone();
        *nodo.prev.borrow_mut() = Weak::new();
        
        // Si había un head, su prev apunta al nuevo nodo
        if let Some(ref head) = head_actual {
            *head.prev.borrow_mut() = Rc::downgrade(nodo);
        } else {
            // Lista vacía, tail también apunta al nuevo nodo
            *self.tail.borrow_mut() = Rc::downgrade(nodo);
        }
        
        // Actualizar head
        *self.head.borrow_mut() = Some(Rc::clone(nodo));
    }

    /// Remueve un nodo de la lista
    fn remover_nodo(&self, nodo: &Rc<Nodo<K, V>>) {
        let prev = nodo.prev.borrow().upgrade();
        let next = nodo.next.borrow().clone();
        
        // Actualizar prev.next
        if let Some(ref prev_nodo) = prev {
            *prev_nodo.next.borrow_mut() = next.clone();
        } else {
            // Era el head
            *self.head.borrow_mut() = next.clone();
        }
        
        // Actualizar next.prev
        if let Some(ref next_nodo) = next {
            *next_nodo.prev.borrow_mut() = match prev {
                Some(ref p) => Rc::downgrade(p),
                None => Weak::new(),
            };
        } else {
            // Era el tail
            *self.tail.borrow_mut() = match prev {
                Some(ref p) => Rc::downgrade(p),
                None => Weak::new(),
            };
        }
    }

    /// Mueve un nodo al frente
    fn mover_al_frente(&self, nodo: &Rc<Nodo<K, V>>) {
        // Si ya es el head, no hacer nada
        if self.head.borrow().as_ref().map(|h| Rc::ptr_eq(h, nodo)).unwrap_or(false) {
            return;
        }
        
        self.remover_nodo(nodo);
        self.agregar_al_frente(nodo);
    }

    /// Remueve el elemento menos recientemente usado
    fn remover_lru(&self) {
        if let Some(tail) = self.tail.borrow().upgrade() {
            let key = tail.key.clone();
            self.remover_nodo(&tail);
            self.mapa.borrow_mut().remove(&key);
        }
    }

    /// Retorna el número de elementos en el cache
    fn len(&self) -> usize {
        self.mapa.borrow().len()
    }

    /// Retorna estadísticas del cache
    fn estadisticas(&self) -> (usize, usize, f64) {
        let hits = *self.hits.borrow();
        let misses = *self.misses.borrow();
        let total = hits + misses;
        let hit_rate = if total > 0 {
            hits as f64 / total as f64 * 100.0
        } else {
            0.0
        };
        (hits, misses, hit_rate)
    }

    /// Imprime el contenido del cache (del más al menos reciente)
    fn imprimir(&self) {
        print!("Cache [");
        let mut current = self.head.borrow().clone();
        let mut first = true;
        while let Some(nodo) = current {
            if !first {
                print!(", ");
            }
            print!("{:?}:{:?}", nodo.key, nodo.value);
            first = false;
            current = nodo.next.borrow().clone();
        }
        println!("]");
    }
}

// ============================================================
// DEMOS
// ============================================================

fn demo_cache_basico() {
    println!("--- Demo: Cache Básico ---");
    
    let cache: LruCache<&str, i32> = LruCache::new(3);
    
    cache.put("a", 1);
    cache.put("b", 2);
    cache.put("c", 3);
    
    print!("Después de insertar a, b, c: ");
    cache.imprimir();
    
    // Acceder a 'a' lo mueve al frente
    let _ = cache.get(&"a");
    print!("Después de acceder 'a': ");
    cache.imprimir();
    
    // Insertar 'd' elimina 'b' (LRU)
    cache.put("d", 4);
    print!("Después de insertar 'd': ");
    cache.imprimir();
}

fn demo_cache_con_capacidad() {
    println!("\n--- Demo: Cache con Capacidad ---");
    
    let cache: LruCache<i32, String> = LruCache::new(2);
    
    cache.put(1, "uno".to_string());
    cache.put(2, "dos".to_string());
    println!("Capacidad: 2, elementos: {}", cache.len());
    cache.imprimir();
    
    cache.put(3, "tres".to_string());
    println!("Después de insertar 3 (elimina 1):");
    cache.imprimir();
    
    // Verificar que 1 fue eliminado
    match cache.get(&1) {
        Some(_) => println!("1 encontrado"),
        None => println!("1 no encontrado (correcto - fue evicted)"),
    }
}

fn demo_cache_estadisticas() {
    println!("\n--- Demo: Estadísticas ---");
    
    let cache: LruCache<&str, &str> = LruCache::new(3);
    
    cache.put("user:1", "Alice");
    cache.put("user:2", "Bob");
    cache.put("user:3", "Charlie");
    
    // Simular accesos
    let _ = cache.get(&"user:1"); // hit
    let _ = cache.get(&"user:1"); // hit
    let _ = cache.get(&"user:4"); // miss
    let _ = cache.get(&"user:2"); // hit
    let _ = cache.get(&"user:5"); // miss
    
    let (hits, misses, hit_rate) = cache.estadisticas();
    println!("Hits: {}, Misses: {}, Hit Rate: {:.1}%", hits, misses, hit_rate);
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_put_get() {
        let cache = LruCache::new(3);
        cache.put("a", 1);
        cache.put("b", 2);
        
        assert_eq!(cache.get(&"a"), Some(1));
        assert_eq!(cache.get(&"b"), Some(2));
        assert_eq!(cache.get(&"c"), None);
    }

    #[test]
    fn test_cache_capacidad() {
        let cache = LruCache::new(2);
        cache.put(1, "a");
        cache.put(2, "b");
        cache.put(3, "c"); // Elimina 1
        
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some("b"));
        assert_eq!(cache.get(&3), Some("c"));
    }

    #[test]
    fn test_cache_lru_order() {
        let cache = LruCache::new(3);
        cache.put("a", 1);
        cache.put("b", 2);
        cache.put("c", 3);
        
        // Acceder a 'a' lo mueve al frente
        let _ = cache.get(&"a");
        
        // Insertar 'd' debería eliminar 'b' (ahora es LRU)
        cache.put("d", 4);
        
        assert_eq!(cache.get(&"b"), None); // Eliminado
        assert_eq!(cache.get(&"a"), Some(1)); // Todavía existe
    }

    #[test]
    fn test_cache_update() {
        let cache = LruCache::new(2);
        cache.put("a", 1);
        cache.put("a", 10); // Update
        
        assert_eq!(cache.get(&"a"), Some(10));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_cache_estadisticas() {
        let cache = LruCache::new(2);
        cache.put("a", 1);
        
        let _ = cache.get(&"a"); // hit
        let _ = cache.get(&"b"); // miss
        let _ = cache.get(&"a"); // hit
        
        let (hits, misses, _) = cache.estadisticas();
        assert_eq!(hits, 2);
        assert_eq!(misses, 1);
    }
}
