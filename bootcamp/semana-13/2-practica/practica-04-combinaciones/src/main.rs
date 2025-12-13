//! Práctica 04: Combinaciones de Smart Pointers
//!
//! En esta práctica aprenderás a:
//! - Combinar Rc<RefCell<T>> para estructuras mutables compartidas
//! - Usar Weak para evitar ciclos
//! - Implementar patrones avanzados

use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn main() {
    println!("=== Práctica 04: Combinaciones de Smart Pointers ===\n");

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
// EJERCICIO 1: Lista Doblemente Enlazada
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
        // TODO: Crear lista vacía
        // ↓ Completa la implementación
        ListaDoble {
            cabeza: None,
            cola: None,
            longitud: 0,
        }
    }
    
    /// Agrega al final de la lista
    pub fn push_back(&mut self, valor: T) {
        // TODO:
        // 1. Crear nuevo nodo con anterior apuntando a cola actual
        // 2. Si hay cola, actualizar su siguiente
        // 3. Si no hay cabeza, establecerla
        // 4. Actualizar cola
        // ↓ Completa la implementación
        let _ = valor;
    }
    
    /// Agrega al inicio de la lista
    pub fn push_front(&mut self, valor: T) {
        // TODO:
        // 1. Crear nuevo nodo con siguiente apuntando a cabeza actual
        // 2. Si hay cabeza, actualizar su anterior
        // 3. Si no hay cola, establecerla
        // 4. Actualizar cabeza
        // ↓ Completa la implementación
        let _ = valor;
    }
    
    /// Remueve y retorna del inicio
    pub fn pop_front(&mut self) -> Option<T> {
        // TODO:
        // 1. Tomar la cabeza actual
        // 2. Actualizar la cabeza al siguiente
        // 3. Si hay nueva cabeza, limpiar su anterior
        // 4. Si lista vacía, limpiar cola
        // ↓ Cambia esto por la implementación correcta
        None
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
// EJERCICIO 2: Sistema de Usuarios y Grupos
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
        // TODO: Crear usuario y agregarlo al sistema
        // ↓ Completa la implementación
        let usuario = Rc::new(RefCell::new(Usuario {
            nombre: nombre.to_string(),
            grupos: Vec::new(),
        }));
        self.usuarios.borrow_mut().push(Rc::clone(&usuario));
        usuario
    }
    
    fn crear_grupo(&self, nombre: &str) -> GrupoRef {
        // TODO: Crear grupo y agregarlo al sistema
        // ↓ Completa la implementación
        let grupo = Rc::new(RefCell::new(Grupo {
            nombre: nombre.to_string(),
            miembros: Vec::new(),
        }));
        self.grupos.borrow_mut().push(Rc::clone(&grupo));
        grupo
    }
    
    fn agregar_a_grupo(&self, usuario: &UsuarioRef, grupo: &GrupoRef) {
        // TODO:
        // 1. Agregar Weak del grupo al usuario
        // 2. Agregar Weak del usuario al grupo
        // ↓ Completa la implementación
        let _ = usuario;
        let _ = grupo;
    }
    
    fn grupos_de(&self, usuario: &UsuarioRef) -> Vec<String> {
        // TODO: Retornar nombres de los grupos del usuario
        // Usar upgrade() para verificar que el grupo existe
        // ↓ Cambia esto por la implementación correcta
        let _ = usuario;
        vec![]
    }
    
    fn miembros_de(&self, grupo: &GrupoRef) -> Vec<String> {
        // TODO: Retornar nombres de los miembros del grupo
        // ↓ Cambia esto por la implementación correcta
        let _ = grupo;
        vec![]
    }
}

// ============================================================
// EJERCICIO 3: Árbol DOM Simplificado
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
        // TODO: Crear nodo DOM
        // ↓ Completa la implementación
        Rc::new(RefCell::new(DomNodeInner {
            tag: tag.to_string(),
            parent: Weak::new(),
            children: Vec::new(),
        }))
    }
    
    fn append_child(padre: &DomNodeRef, hijo: &DomNodeRef) {
        // TODO:
        // 1. Establecer parent del hijo
        // 2. Agregar hijo a children del padre
        // ↓ Completa la implementación
        let _ = padre;
        let _ = hijo;
    }
    
    /// Retorna la ruta desde la raíz (ej: "html > body > p")
    fn path(nodo: &DomNodeRef) -> String {
        // TODO: Navegar hacia arriba construyendo la ruta
        // ↓ Cambia esto por la implementación correcta
        let _ = nodo;
        String::new()
    }
}

// ============================================================
// TESTS
// ============================================================

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
        // Crear estructura con ciclos potenciales
        let sistema = Sistema::new();
        
        let user = sistema.crear_usuario("test");
        let grupo = sistema.crear_grupo("group");
        
        sistema.agregar_a_grupo(&user, &grupo);
        
        // Verificar que Weak no incrementa strong count
        assert_eq!(Rc::strong_count(&user), 2);  // sistema + variable local
        assert_eq!(Rc::strong_count(&grupo), 2);
    }
}
