//! Solución - Práctica 04: Weak

use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    println!("=== Práctica 04: Weak ===\n");

    println!("--- Ejercicio 1: Weak Básico ---");
    exercise_weak_basic();

    println!("\n--- Ejercicio 2: Árbol con Parent ---");
    exercise_tree();

    println!("\n--- Ejercicio 3: Observer Pattern ---");
    exercise_observer();

    println!("\n✅ Todos los ejercicios completados!");
}

fn exercise_weak_basic() {
    let strong_ref = Rc::new(42);
    let weak_ref = Rc::downgrade(&strong_ref);
    
    println!("Valor con Rc: {}", strong_ref);
    println!("Strong count: {}", Rc::strong_count(&strong_ref));
    println!("Weak count: {}", Rc::weak_count(&strong_ref));
    
    match weak_ref.upgrade() {
        Some(value) => println!("Valor via Weak: {}", value),
        None => println!("El valor ya no existe"),
    }
    
    drop(strong_ref);
    
    match weak_ref.upgrade() {
        Some(value) => println!("Valor: {}", value),
        None => println!("Weak::upgrade() retornó None (correcto!)"),
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        })
    }
    
    fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
        parent.children.borrow_mut().push(Rc::clone(child));
        *child.parent.borrow_mut() = Rc::downgrade(parent);
    }
    
    fn get_parent(&self) -> Option<Rc<Node>> {
        self.parent.borrow().upgrade()
    }
}

fn exercise_tree() {
    let root = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);
    let grandchild = Node::new(4);
    
    Node::add_child(&root, &child1);
    Node::add_child(&root, &child2);
    Node::add_child(&child1, &grandchild);
    
    println!("Raíz: {}", root.value);
    println!("  Hijos: {:?}", root.children.borrow().iter().map(|n| n.value).collect::<Vec<_>>());
    
    if let Some(parent) = grandchild.get_parent() {
        println!("Padre de grandchild (4): {}", parent.value);
        
        if let Some(grandparent) = parent.get_parent() {
            println!("Abuelo de grandchild (4): {}", grandparent.value);
        }
    }
    
    println!("\nReference counts:");
    println!("  root - strong: {}, weak: {}", 
             Rc::strong_count(&root), Rc::weak_count(&root));
    println!("  child1 - strong: {}, weak: {}", 
             Rc::strong_count(&child1), Rc::weak_count(&child1));
}

struct Publisher {
    observers: RefCell<Vec<Weak<Observer>>>,
    value: RefCell<String>,
}

struct Observer {
    name: String,
}

impl Publisher {
    fn new() -> Self {
        Publisher {
            observers: RefCell::new(Vec::new()),
            value: RefCell::new(String::new()),
        }
    }
    
    fn subscribe(&self, obs: &Rc<Observer>) {
        self.observers.borrow_mut().push(Rc::downgrade(obs));
    }
    
    fn notify(&self, message: &str) {
        *self.value.borrow_mut() = message.to_string();
        
        let obs = self.observers.borrow();
        let mut alive = 0;
        let mut dead = 0;
        
        for weak_obs in obs.iter() {
            match weak_obs.upgrade() {
                Some(obs) => {
                    println!("  → {} recibió: '{}'", obs.name, message);
                    alive += 1;
                }
                None => {
                    dead += 1;
                }
            }
        }
        
        println!("  Observadores: {} vivos, {} eliminados", alive, dead);
    }
    
    fn cleanup_dead(&self) {
        self.observers.borrow_mut().retain(|weak| weak.upgrade().is_some());
    }
}

impl Observer {
    fn new(name: &str) -> Rc<Self> {
        Rc::new(Observer {
            name: name.to_string(),
        })
    }
}

fn exercise_observer() {
    let publisher = Publisher::new();
    
    let obs1 = Observer::new("Alice");
    let obs2 = Observer::new("Bob");
    
    publisher.subscribe(&obs1);
    publisher.subscribe(&obs2);
    
    println!("Notificación 1:");
    publisher.notify("Hola a todos");
    
    drop(obs2);
    
    println!("\nNotificación 2 (después de drop obs2):");
    publisher.notify("Solo Alice recibirá esto");
    
    publisher.cleanup_dead();
    println!("\nDespués de limpiar: {} observadores", 
             publisher.observers.borrow().len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weak_upgrade() {
        let rc = Rc::new(100);
        let weak = Rc::downgrade(&rc);
        
        assert!(weak.upgrade().is_some());
        drop(rc);
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn test_tree_parent() {
        let root = Node::new(1);
        let child = Node::new(2);
        
        Node::add_child(&root, &child);
        
        let parent = child.get_parent();
        assert!(parent.is_some());
        assert_eq!(parent.unwrap().value, 1);
    }

    #[test]
    fn test_tree_no_memory_leak() {
        let root = Node::new(1);
        let child = Node::new(2);
        
        Node::add_child(&root, &child);
        
        assert_eq!(Rc::strong_count(&root), 1);
        assert_eq!(Rc::strong_count(&child), 2);
        assert_eq!(Rc::weak_count(&root), 1);
    }

    #[test]
    fn test_observer_cleanup() {
        let pub_ = Publisher::new();
        let obs = Observer::new("Test");
        
        pub_.subscribe(&obs);
        assert_eq!(pub_.observers.borrow().len(), 1);
        
        drop(obs);
        pub_.cleanup_dead();
        assert_eq!(pub_.observers.borrow().len(), 0);
    }
}
