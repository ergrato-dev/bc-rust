//! Práctica 04: Weak - Rompiendo Ciclos
//!
//! En esta práctica aprenderás a:
//! - Usar Weak<T> para referencias no-owning
//! - Romper ciclos de referencia
//! - Implementar estructuras padre-hijo

use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    println!("=== Práctica 04: Weak ===\n");

    // Ejercicio 1: Weak básico
    println!("--- Ejercicio 1: Weak Básico ---");
    ejercicio_weak_basico();

    // Ejercicio 2: Árbol con parent
    println!("\n--- Ejercicio 2: Árbol con Parent ---");
    ejercicio_arbol();

    // Ejercicio 3: Observer pattern
    println!("\n--- Ejercicio 3: Observer Pattern ---");
    ejercicio_observer();

    println!("\n✅ Todos los ejercicios completados!");
}

// ============================================================
// EJERCICIO 1: Weak Básico
// ============================================================

fn ejercicio_weak_basico() {
    // Crear Rc y obtener Weak
    let fuerte = Rc::new(42);
    let debil = Rc::downgrade(&fuerte);
    
    println!("Valor con Rc: {}", fuerte);
    println!("Strong count: {}", Rc::strong_count(&fuerte));
    println!("Weak count: {}", Rc::weak_count(&fuerte));
    
    // Usar upgrade() para acceder al valor
    match debil.upgrade() {
        Some(valor) => println!("Valor via Weak: {}", valor),
        None => println!("El valor ya no existe"),
    }
    
    // Simular que el Rc se dropea
    drop(fuerte);
    
    // ↓ Ahora upgrade() retorna None
    match debil.upgrade() {
        Some(valor) => println!("Valor: {}", valor),
        None => println!("Weak::upgrade() retornó None (correcto!)"),
    }
}

// ============================================================
// EJERCICIO 2: Árbol con Referencias al Padre
// ============================================================

#[derive(Debug)]
struct Nodo {
    valor: i32,
    parent: RefCell<Weak<Nodo>>,           // ← Weak para evitar ciclo
    children: RefCell<Vec<Rc<Nodo>>>,       // ← Strong para los hijos
}

impl Nodo {
    fn new(valor: i32) -> Rc<Nodo> {
        Rc::new(Nodo {
            valor,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        })
    }
    
    fn agregar_hijo(parent: &Rc<Nodo>, hijo: &Rc<Nodo>) {
        // ↓ Implementa: agrega hijo a children del parent
        parent.children.borrow_mut().push(Rc::clone(hijo));
        
        // ↓ Implementa: establece el parent del hijo
        *hijo.parent.borrow_mut() = Rc::downgrade(parent);
    }
    
    fn obtener_parent(&self) -> Option<Rc<Nodo>> {
        // ↓ Implementa: retorna el parent si existe
        self.parent.borrow().upgrade()
    }
}

fn ejercicio_arbol() {
    let raiz = Nodo::new(1);
    let hijo1 = Nodo::new(2);
    let hijo2 = Nodo::new(3);
    let nieto = Nodo::new(4);
    
    Nodo::agregar_hijo(&raiz, &hijo1);
    Nodo::agregar_hijo(&raiz, &hijo2);
    Nodo::agregar_hijo(&hijo1, &nieto);
    
    println!("Raíz: {}", raiz.valor);
    println!("  Hijos: {:?}", raiz.children.borrow().iter().map(|n| n.valor).collect::<Vec<_>>());
    
    // Verificar referencia al padre
    if let Some(parent) = nieto.obtener_parent() {
        println!("Padre de nieto (4): {}", parent.valor);
    }
    
    // Verificar counts
    println!("\nReference counts:");
    println!("  raiz - strong: {}, weak: {}", 
             Rc::strong_count(&raiz), Rc::weak_count(&raiz));
    println!("  hijo1 - strong: {}, weak: {}", 
             Rc::strong_count(&hijo1), Rc::weak_count(&hijo1));
}

// ============================================================
// EJERCICIO 3: Observer Pattern (Simplificado)
// ============================================================

struct Publicador {
    observadores: RefCell<Vec<Weak<Observador>>>,
    valor: RefCell<String>,
}

struct Observador {
    nombre: String,
}

impl Publicador {
    fn new() -> Self {
        Publicador {
            observadores: RefCell::new(Vec::new()),
            valor: RefCell::new(String::new()),
        }
    }
    
    fn suscribir(&self, obs: &Rc<Observador>) {
        // ↓ Guarda Weak reference al observador
        self.observadores.borrow_mut().push(Rc::downgrade(obs));
    }
    
    fn notificar(&self, mensaje: &str) {
        // ↓ Implementa: notifica a todos los observadores vivos
        *self.valor.borrow_mut() = mensaje.to_string();
        
        let obs = self.observadores.borrow();
        let mut vivos = 0;
        let mut muertos = 0;
        
        for weak_obs in obs.iter() {
            match weak_obs.upgrade() {
                Some(obs) => {
                    println!("  → {} recibió: '{}'", obs.nombre, mensaje);
                    vivos += 1;
                }
                None => {
                    muertos += 1;
                }
            }
        }
        
        println!("  Observadores: {} vivos, {} eliminados", vivos, muertos);
    }
    
    fn limpiar_muertos(&self) {
        // ↓ Elimina observadores que ya no existen
        self.observadores.borrow_mut().retain(|weak| weak.upgrade().is_some());
    }
}

impl Observador {
    fn new(nombre: &str) -> Rc<Self> {
        Rc::new(Observador {
            nombre: nombre.to_string(),
        })
    }
}

fn ejercicio_observer() {
    let publicador = Publicador::new();
    
    let obs1 = Observador::new("Alice");
    let obs2 = Observador::new("Bob");
    
    publicador.suscribir(&obs1);
    publicador.suscribir(&obs2);
    
    println!("Notificación 1:");
    publicador.notificar("Hola a todos");
    
    // Eliminar un observador
    drop(obs2);
    
    println!("\nNotificación 2 (después de drop obs2):");
    publicador.notificar("Solo Alice recibirá esto");
    
    // Limpiar observadores muertos
    publicador.limpiar_muertos();
    println!("\nDespués de limpiar: {} observadores", 
             publicador.observadores.borrow().len());
}

// ============================================================
// TESTS
// ============================================================

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
    fn test_arbol_parent() {
        let raiz = Nodo::new(1);
        let hijo = Nodo::new(2);
        
        Nodo::agregar_hijo(&raiz, &hijo);
        
        let parent = hijo.obtener_parent();
        assert!(parent.is_some());
        assert_eq!(parent.unwrap().valor, 1);
    }

    #[test]
    fn test_arbol_no_memory_leak() {
        let raiz = Nodo::new(1);
        let hijo = Nodo::new(2);
        
        Nodo::agregar_hijo(&raiz, &hijo);
        
        // raiz tiene 1 strong ref (la variable local)
        assert_eq!(Rc::strong_count(&raiz), 1);
        
        // hijo tiene 2 strong refs (variable local + children del parent)
        assert_eq!(Rc::strong_count(&hijo), 2);
        
        // raiz tiene 1 weak ref (del hijo)
        assert_eq!(Rc::weak_count(&raiz), 1);
    }

    #[test]
    fn test_observer_cleanup() {
        let pub_ = Publicador::new();
        let obs = Observador::new("Test");
        
        pub_.suscribir(&obs);
        assert_eq!(pub_.observadores.borrow().len(), 1);
        
        drop(obs);
        pub_.limpiar_muertos();
        assert_eq!(pub_.observadores.borrow().len(), 0);
    }
}
