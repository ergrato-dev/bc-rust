//! Solución - Práctica 04: Weak

use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    println!("=== Práctica 04: Weak ===\n");

    println!("--- Ejercicio 1: Weak Básico ---");
    ejercicio_weak_basico();

    println!("\n--- Ejercicio 2: Árbol con Parent ---");
    ejercicio_arbol();

    println!("\n--- Ejercicio 3: Observer Pattern ---");
    ejercicio_observer();

    println!("\n✅ Todos los ejercicios completados!");
}

fn ejercicio_weak_basico() {
    let fuerte = Rc::new(42);
    let debil = Rc::downgrade(&fuerte);
    
    println!("Valor con Rc: {}", fuerte);
    println!("Strong count: {}", Rc::strong_count(&fuerte));
    println!("Weak count: {}", Rc::weak_count(&fuerte));
    
    match debil.upgrade() {
        Some(valor) => println!("Valor via Weak: {}", valor),
        None => println!("El valor ya no existe"),
    }
    
    drop(fuerte);
    
    match debil.upgrade() {
        Some(valor) => println!("Valor: {}", valor),
        None => println!("Weak::upgrade() retornó None (correcto!)"),
    }
}

#[derive(Debug)]
struct Nodo {
    valor: i32,
    parent: RefCell<Weak<Nodo>>,
    children: RefCell<Vec<Rc<Nodo>>>,
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
        parent.children.borrow_mut().push(Rc::clone(hijo));
        *hijo.parent.borrow_mut() = Rc::downgrade(parent);
    }
    
    fn obtener_parent(&self) -> Option<Rc<Nodo>> {
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
    
    if let Some(parent) = nieto.obtener_parent() {
        println!("Padre de nieto (4): {}", parent.valor);
        
        if let Some(abuelo) = parent.obtener_parent() {
            println!("Abuelo de nieto (4): {}", abuelo.valor);
        }
    }
    
    println!("\nReference counts:");
    println!("  raiz - strong: {}, weak: {}", 
             Rc::strong_count(&raiz), Rc::weak_count(&raiz));
    println!("  hijo1 - strong: {}, weak: {}", 
             Rc::strong_count(&hijo1), Rc::weak_count(&hijo1));
}

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
        self.observadores.borrow_mut().push(Rc::downgrade(obs));
    }
    
    fn notificar(&self, mensaje: &str) {
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
    
    drop(obs2);
    
    println!("\nNotificación 2 (después de drop obs2):");
    publicador.notificar("Solo Alice recibirá esto");
    
    publicador.limpiar_muertos();
    println!("\nDespués de limpiar: {} observadores", 
             publicador.observadores.borrow().len());
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
        
        assert_eq!(Rc::strong_count(&raiz), 1);
        assert_eq!(Rc::strong_count(&hijo), 2);
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
