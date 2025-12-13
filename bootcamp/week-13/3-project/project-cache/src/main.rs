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
struct Node<K, V> {
    key: K,
    value: V,
    prev: RefCell<Weak<Node<K, V>>>,
    next: RefCell<Option<Rc<Node<K, V>>>>,
}

/// Cache LRU con capacidad limitada
struct LruCache<K, V> {
    capacity: usize,
    map: RefCell<HashMap<K, Rc<Node<K, V>>>>,
    head: RefCell<Option<Rc<Node<K, V>>>>,  // Más reciente
    tail: RefCell<Weak<Node<K, V>>>,        // Menos reciente
    hits: RefCell<usize>,
    misses: RefCell<usize>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Rc<Self> {
        Rc::new(Node {
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
    fn new(capacity: usize) -> Self {
        LruCache {
            capacity,
            map: RefCell::new(HashMap::new()),
            head: RefCell::new(None),
            tail: RefCell::new(Weak::new()),
            hits: RefCell::new(0),
            misses: RefCell::new(0),
        }
    }

    /// Obtiene un valor del cache
    fn get(&self, key: &K) -> Option<V> {
        let map = self.map.borrow();
        
        if let Some(node) = map.get(key) {
            *self.hits.borrow_mut() += 1;
            
            // Mover al frente (más reciente)
            let node_clone = Rc::clone(node);
            drop(map); // Liberar borrow antes de modificar
            self.move_to_front(&node_clone);
            
            Some(node_clone.value.clone())
        } else {
            *self.misses.borrow_mut() += 1;
            None
        }
    }

    /// Inserta o actualiza un valor en el cache
    fn put(&self, key: K, value: V) {
        // Si ya existe, remover el nodo viejo
        let existed = {
            let map = self.map.borrow();
            map.contains_key(&key)
        };
        
        if existed {
            if let Some(old_node) = self.map.borrow_mut().remove(&key) {
                self.remove_node(&old_node);
            }
        }
        
        // Verificar capacidad y remover LRU si es necesario
        let needs_evict = {
            self.map.borrow().len() >= self.capacity
        };
        
        if needs_evict {
            // Obtener la key del tail para remover
            let tail_key = {
                self.tail.borrow().upgrade().map(|t| t.key.clone())
            };
            
            if let Some(k) = tail_key {
                if let Some(tail) = self.map.borrow_mut().remove(&k) {
                    self.remove_node(&tail);
                }
            }
        }
        
        // Crear nuevo nodo
        let node = Node::new(key.clone(), value);
        
        // Agregar al frente
        self.add_to_front(&node);
        
        // Agregar al mapa
        self.map.borrow_mut().insert(key, node);
    }

    /// Agrega un nodo al frente (más reciente)
    fn add_to_front(&self, node: &Rc<Node<K, V>>) {
        let current_head = self.head.borrow().clone();
        
        // Nuevo nodo apunta al head actual
        *node.next.borrow_mut() = current_head.clone();
        *node.prev.borrow_mut() = Weak::new();
        
        // Si había un head, su prev apunta al nuevo nodo
        if let Some(ref head) = current_head {
            *head.prev.borrow_mut() = Rc::downgrade(node);
        } else {
            // Lista vacía, tail también apunta al nuevo nodo
            *self.tail.borrow_mut() = Rc::downgrade(node);
        }
        
        // Actualizar head
        *self.head.borrow_mut() = Some(Rc::clone(node));
    }

    /// Remueve un nodo de la lista
    fn remove_node(&self, node: &Rc<Node<K, V>>) {
        let prev = node.prev.borrow().upgrade();
        let next = node.next.borrow().clone();
        
        // Actualizar prev.next
        if let Some(ref prev_node) = prev {
            *prev_node.next.borrow_mut() = next.clone();
        } else {
            // Era el head
            *self.head.borrow_mut() = next.clone();
        }
        
        // Actualizar next.prev
        if let Some(ref next_node) = next {
            *next_node.prev.borrow_mut() = match prev {
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
    fn move_to_front(&self, node: &Rc<Node<K, V>>) {
        // Si ya es el head, no hacer nada
        if self.head.borrow().as_ref().map(|h| Rc::ptr_eq(h, node)).unwrap_or(false) {
            return;
        }
        
        self.remove_node(node);
        self.add_to_front(node);
    }

    /// Remueve el elemento menos recientemente usado
    /// Remueve el elemento menos recientemente usado
    fn remove_lru(&self) {
        if let Some(tail) = self.tail.borrow().upgrade() {
            let key = tail.key.clone();
            self.remove_node(&tail);
            self.map.borrow_mut().remove(&key);
        }
    }

    /// Retorna el número de elementos en el cache
    fn len(&self) -> usize {
        self.map.borrow().len()
    }

    /// Retorna estadísticas del cache
    fn stats(&self) -> (usize, usize, f64) {
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
    fn print_cache(&self) {
        print!("Cache [");
        let mut current = self.head.borrow().clone();
        let mut first = true;
        while let Some(node) = current {
            if !first {
                print!(", ");
            }
            print!("{:?}:{:?}", node.key, node.value);
            first = false;
            current = node.next.borrow().clone();
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
    cache.print_cache();
    
    // Acceder a 'a' lo mueve al frente
    let _ = cache.get(&"a");
    print!("Después de acceder 'a': ");
    cache.print_cache();
    
    // Insertar 'd' elimina 'b' (LRU)
    cache.put("d", 4);
    print!("Después de insertar 'd': ");
    cache.print_cache();
}

fn demo_cache_con_capacidad() {
    println!("\n--- Demo: Cache con Capacidad ---");
    
    let cache: LruCache<i32, String> = LruCache::new(2);
    
    cache.put(1, "uno".to_string());
    cache.put(2, "dos".to_string());
    println!("Capacidad: 2, elementos: {}", cache.len());
    cache.print_cache();
    
    cache.put(3, "tres".to_string());
    println!("Después de insertar 3 (elimina 1):");
    cache.print_cache();
    
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
    
    let (hits, misses, hit_rate) = cache.stats();
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
    fn test_cache_capacity() {
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
    fn test_cache_stats() {
        let cache = LruCache::new(2);
        cache.put("a", 1);
        
        let _ = cache.get(&"a"); // hit
        let _ = cache.get(&"b"); // miss
        let _ = cache.get(&"a"); // hit
        
        let (hits, misses, _) = cache.stats();
        assert_eq!(hits, 2);
        assert_eq!(misses, 1);
    }
}
