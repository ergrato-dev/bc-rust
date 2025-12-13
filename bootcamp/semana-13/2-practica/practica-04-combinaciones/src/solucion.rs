//! Solución - Práctica 04: Combinaciones de Smart Pointers

use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn main() {
    println!("=== Práctica 04: Combinaciones - SOLUCIÓN ===\n");

    // Ejercicio 1: Lista doblemente enlazada
    println!("--- Ejercicio 1: Lista Doblemente Enlazada ---");
    let mut lista = ListaDoble::new();
    
    lista.push_back(1);
    lista.push_back(2);
    lista.push_back(3);
    lista.push_front(0);
    
    println!("Longitud: {}", lista.len());
    
    while let Some(valor) = lista.pop_front() {
        println!("Pop: {}", valor);
    }
    
    // Ejercicio 2: Sistema de usuarios y grupos
    println!("\n--- Ejercicio 2: Usuarios y Grupos ---");
    let sistema = Sistema::new();
    
    let admin = sistema.crear_usuario("admin");
    let user1 = sistema.crear_usuario("user1");
    let user2 = sistema.crear_usuario("user2");
    
    let grupo_dev = sistema.crear_grupo("developers");
    let grupo_admin = sistema.crear_grupo("admins");
    
    sistema.agregar_a_grupo(&admin, &grupo_admin);
    sistema.agregar_a_grupo(&admin, &grupo_dev);
    sistema.agregar_a_grupo(&user1, &grupo_dev);
    sistema.agregar_a_grupo(&user2, &grupo_dev);
    
    println!("Grupos de admin: {:?}", sistema.grupos_de(&admin));
    println!("Miembros de developers: {:?}", sistema.miembros_de(&grupo_dev));
    
    // Ejercicio 3: Árbol DOM simplificado
    println!("\n--- Ejercicio 3: Árbol DOM ---");
    let html = DomNode::new("html");
    let body = DomNode::new("body");
    let div = DomNode::new("div");
    let p = DomNode::new("p");
    
    DomNode::append_child(&html, &body);
    DomNode::append_child(&body, &div);
    DomNode::append_child(&body, &p);
    
    println!("HTML tiene {} hijos", html.borrow().children.len());
    println!("Ruta de p: {}", DomNode::path(&p));
    
    println!("\n✅ Práctica completada!");
}

// ============================================================
// EJERCICIO 1: Lista Doblemente Enlazada - SOLUCIÓN
// ============================================================

type NodoLista<T> = Option<Rc<RefCell<NodoListaInner<T>>>>;
type NodoWeak<T> = Option<Weak<RefCell<NodoListaInner<T>>>>;

struct NodoListaInner<T> {
    valor: T,
    siguiente: NodoLista<T>,
    anterior: NodoWeak<T>,
}

pub struct ListaDoble<T> {
    cabeza: NodoLista<T>,
    cola: NodoWeak<T>,
    longitud: usize,
}

impl<T> ListaDoble<T> {
    pub fn new() -> Self {
        ListaDoble {
            cabeza: None,
            cola: None,
            longitud: 0,
        }
    }
    
    pub fn push_back(&mut self, valor: T) {
        let nuevo = Rc::new(RefCell::new(NodoListaInner {
            valor,
            siguiente: None,
            anterior: self.cola.clone(),
        }));
        
        match self.cola.as_ref().and_then(|w| w.upgrade()) {
            Some(vieja_cola) => {
                vieja_cola.borrow_mut().siguiente = Some(Rc::clone(&nuevo));
            }
            None => {
                self.cabeza = Some(Rc::clone(&nuevo));
            }
        }
        
        self.cola = Some(Rc::downgrade(&nuevo));
        self.longitud += 1;
    }
    
    pub fn push_front(&mut self, valor: T) {
        let nuevo = Rc::new(RefCell::new(NodoListaInner {
            valor,
            siguiente: self.cabeza.take(),
            anterior: None,
        }));
        
        if let Some(ref vieja_cabeza) = self.cabeza {
            vieja_cabeza.borrow_mut().anterior = Some(Rc::downgrade(&nuevo));
        }
        
        if self.cola.is_none() {
            self.cola = Some(Rc::downgrade(&nuevo));
        } else if let Some(ref cabeza) = nuevo.borrow().siguiente {
            cabeza.borrow_mut().anterior = Some(Rc::downgrade(&nuevo));
        }
        
        self.cabeza = Some(nuevo);
        self.longitud += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        self.cabeza.take().map(|vieja_cabeza| {
            match vieja_cabeza.borrow_mut().siguiente.take() {
                Some(nueva_cabeza) => {
                    nueva_cabeza.borrow_mut().anterior = None;
                    self.cabeza = Some(nueva_cabeza);
                }
                None => {
                    self.cola = None;
                }
            }
            self.longitud -= 1;
            Rc::try_unwrap(vieja_cabeza).ok().unwrap().into_inner().valor
        })
    }
    
    pub fn len(&self) -> usize {
        self.longitud
    }
    
    pub fn is_empty(&self) -> bool {
        self.cabeza.is_none()
    }
}

impl<T> Default for ListaDoble<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// EJERCICIO 2: Sistema de Usuarios y Grupos - SOLUCIÓN
// ============================================================

type UsuarioRef = Rc<RefCell<Usuario>>;
type GrupoRef = Rc<RefCell<Grupo>>;

struct Usuario {
    nombre: String,
    grupos: Vec<Weak<RefCell<Grupo>>>,
}

struct Grupo {
    nombre: String,
    miembros: Vec<Weak<RefCell<Usuario>>>,
}

struct Sistema {
    usuarios: RefCell<Vec<UsuarioRef>>,
    grupos: RefCell<Vec<GrupoRef>>,
}

impl Sistema {
    fn new() -> Self {
        Sistema {
            usuarios: RefCell::new(Vec::new()),
            grupos: RefCell::new(Vec::new()),
        }
    }
    
    fn crear_usuario(&self, nombre: &str) -> UsuarioRef {
        let usuario = Rc::new(RefCell::new(Usuario {
            nombre: nombre.to_string(),
            grupos: Vec::new(),
        }));
        self.usuarios.borrow_mut().push(Rc::clone(&usuario));
        usuario
    }
    
    fn crear_grupo(&self, nombre: &str) -> GrupoRef {
        let grupo = Rc::new(RefCell::new(Grupo {
            nombre: nombre.to_string(),
            miembros: Vec::new(),
        }));
        self.grupos.borrow_mut().push(Rc::clone(&grupo));
        grupo
    }
    
    fn agregar_a_grupo(&self, usuario: &UsuarioRef, grupo: &GrupoRef) {
        usuario.borrow_mut().grupos.push(Rc::downgrade(grupo));
        grupo.borrow_mut().miembros.push(Rc::downgrade(usuario));
    }
    
    fn grupos_de(&self, usuario: &UsuarioRef) -> Vec<String> {
        usuario.borrow()
            .grupos
            .iter()
            .filter_map(|weak| weak.upgrade())
            .map(|grupo| grupo.borrow().nombre.clone())
            .collect()
    }
    
    fn miembros_de(&self, grupo: &GrupoRef) -> Vec<String> {
        grupo.borrow()
            .miembros
            .iter()
            .filter_map(|weak| weak.upgrade())
            .map(|usuario| usuario.borrow().nombre.clone())
            .collect()
    }
}

// ============================================================
// EJERCICIO 3: Árbol DOM Simplificado - SOLUCIÓN
// ============================================================

type DomNodeRef = Rc<RefCell<DomNodeInner>>;

struct DomNodeInner {
    tag: String,
    parent: Weak<RefCell<DomNodeInner>>,
    children: Vec<DomNodeRef>,
}

struct DomNode;

impl DomNode {
    fn new(tag: &str) -> DomNodeRef {
        Rc::new(RefCell::new(DomNodeInner {
            tag: tag.to_string(),
            parent: Weak::new(),
            children: Vec::new(),
        }))
    }
    
    fn append_child(padre: &DomNodeRef, hijo: &DomNodeRef) {
        hijo.borrow_mut().parent = Rc::downgrade(padre);
        padre.borrow_mut().children.push(Rc::clone(hijo));
    }
    
    fn path(nodo: &DomNodeRef) -> String {
        let mut partes = vec![nodo.borrow().tag.clone()];
        let mut actual = nodo.borrow().parent.upgrade();
        
        while let Some(padre) = actual {
            partes.push(padre.borrow().tag.clone());
            actual = padre.borrow().parent.upgrade();
        }
        
        partes.reverse();
        partes.join(" > ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lista_push_pop() {
        let mut lista = ListaDoble::new();
        
        lista.push_back(1);
        lista.push_back(2);
        lista.push_front(0);
        
        assert_eq!(lista.len(), 3);
        assert_eq!(lista.pop_front(), Some(0));
        assert_eq!(lista.pop_front(), Some(1));
        assert_eq!(lista.pop_front(), Some(2));
        assert_eq!(lista.pop_front(), None);
    }
    
    #[test]
    fn test_sistema_usuarios_grupos() {
        let sistema = Sistema::new();
        
        let user = sistema.crear_usuario("test");
        let grupo = sistema.crear_grupo("testers");
        
        sistema.agregar_a_grupo(&user, &grupo);
        
        let grupos = sistema.grupos_de(&user);
        assert_eq!(grupos, vec!["testers"]);
        
        let miembros = sistema.miembros_de(&grupo);
        assert_eq!(miembros, vec!["test"]);
    }
    
    #[test]
    fn test_dom_tree() {
        let html = DomNode::new("html");
        let body = DomNode::new("body");
        let p = DomNode::new("p");
        
        DomNode::append_child(&html, &body);
        DomNode::append_child(&body, &p);
        
        assert_eq!(html.borrow().children.len(), 1);
        assert_eq!(body.borrow().children.len(), 1);
        
        let path = DomNode::path(&p);
        assert_eq!(path, "html > body > p");
    }
    
    #[test]
    fn test_no_memory_leaks() {
        let sistema = Sistema::new();
        
        let user = sistema.crear_usuario("test");
        let grupo = sistema.crear_grupo("group");
        
        sistema.agregar_a_grupo(&user, &grupo);
        
        assert_eq!(Rc::strong_count(&user), 2);
        assert_eq!(Rc::strong_count(&grupo), 2);
    }
}
