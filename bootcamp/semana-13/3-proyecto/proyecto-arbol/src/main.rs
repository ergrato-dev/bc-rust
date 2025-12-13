//! Proyecto Semanal: Árbol de Archivos Interactivo
//!
//! Un sistema de árbol de archivos similar al explorador de archivos,
//! donde los nodos pueden ser compartidos y modificados.
//!
//! Conceptos aplicados:
//! - Rc<RefCell<T>> para nodos mutables compartidos
//! - Weak para referencias padre
//! - Box para datos de tamaño variable
//! - Pattern matching con smart pointers

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt;

fn main() {
    println!("=== 🌲 Proyecto: Árbol de Archivos ===\n");

    // Crear estructura de árbol
    let raiz = FileNode::crear_directorio("/");
    
    let home = FileNode::crear_directorio("home");
    let etc = FileNode::crear_directorio("etc");
    let var = FileNode::crear_directorio("var");
    
    FileNode::agregar_hijo(&raiz, &home);
    FileNode::agregar_hijo(&raiz, &etc);
    FileNode::agregar_hijo(&raiz, &var);
    
    let usuario = FileNode::crear_directorio("usuario");
    let documentos = FileNode::crear_directorio("documentos");
    
    FileNode::agregar_hijo(&home, &usuario);
    FileNode::agregar_hijo(&usuario, &documentos);
    
    // Crear algunos archivos
    let readme = FileNode::crear_archivo("README.md", "# Mi Proyecto\n");
    let config = FileNode::crear_archivo(".gitignore", "target/\n*.log\n");
    
    FileNode::agregar_hijo(&documentos, &readme);
    FileNode::agregar_hijo(&documentos, &config);
    
    // Mostrar árbol
    println!("📂 Estructura del árbol:");
    FileNode::imprimir_arbol(&raiz, 0);
    
    // Buscar archivos
    println!("\n🔍 Búsqueda:");
    if let Some(nodo) = FileNode::buscar(&raiz, "README.md") {
        println!("Encontrado: {}", FileNode::ruta_completa(&nodo));
    }
    
    // Estadísticas
    println!("\n📊 Estadísticas:");
    let stats = FileNode::estadisticas(&raiz);
    println!("Directorios: {}", stats.directorios);
    println!("Archivos: {}", stats.archivos);
    println!("Tamaño total: {} bytes", stats.tamanio_total);
    
    // Modificar archivo
    println!("\n✏️ Modificación:");
    if let Some(nodo) = FileNode::buscar(&raiz, "README.md") {
        FileNode::modificar_contenido(&nodo, "# Mi Proyecto Actualizado\n\nDescripción...\n");
        println!("Archivo modificado");
    }
    
    // Mover nodo
    println!("\n📦 Mover archivo:");
    if let Some(gitignore) = FileNode::buscar(&raiz, ".gitignore") {
        FileNode::mover(&gitignore, &usuario);
        println!("Nueva ubicación: {}", FileNode::ruta_completa(&gitignore));
    }
    
    // Mostrar árbol actualizado
    println!("\n📂 Árbol actualizado:");
    FileNode::imprimir_arbol(&raiz, 0);
    
    println!("\n✅ Proyecto completado!");
}

// ============================================================
// TIPOS Y ESTRUCTURAS
// ============================================================

type NodoRef = Rc<RefCell<NodoInner>>;
type NodoWeak = Weak<RefCell<NodoInner>>;

/// Tipo de nodo en el árbol
enum TipoNodo {
    Directorio,
    Archivo { contenido: String },
}

/// Estructura interna del nodo
struct NodoInner {
    nombre: String,
    tipo: TipoNodo,
    padre: NodoWeak,
    hijos: Vec<NodoRef>,
}

/// Estadísticas del árbol
struct Estadisticas {
    directorios: usize,
    archivos: usize,
    tamanio_total: usize,
}

/// API pública para manipular nodos
struct FileNode;

impl FileNode {
    /// Crea un nuevo directorio
    fn crear_directorio(nombre: &str) -> NodoRef {
        // TODO: Crear nodo de tipo directorio
        // ↓ Completa la implementación
        Rc::new(RefCell::new(NodoInner {
            nombre: nombre.to_string(),
            tipo: TipoNodo::Directorio,
            padre: Weak::new(),
            hijos: Vec::new(),
        }))
    }
    
    /// Crea un nuevo archivo con contenido
    fn crear_archivo(nombre: &str, contenido: &str) -> NodoRef {
        // TODO: Crear nodo de tipo archivo
        // ↓ Completa la implementación
        Rc::new(RefCell::new(NodoInner {
            nombre: nombre.to_string(),
            tipo: TipoNodo::Archivo { 
                contenido: contenido.to_string() 
            },
            padre: Weak::new(),
            hijos: Vec::new(),
        }))
    }
    
    /// Agrega un hijo a un directorio
    fn agregar_hijo(padre: &NodoRef, hijo: &NodoRef) {
        // TODO:
        // 1. Verificar que el padre es un directorio
        // 2. Establecer el padre del hijo
        // 3. Agregar el hijo a la lista de hijos
        // ↓ Completa la implementación
        let _ = padre;
        let _ = hijo;
    }
    
    /// Busca un nodo por nombre (búsqueda recursiva)
    fn buscar(raiz: &NodoRef, nombre: &str) -> Option<NodoRef> {
        // TODO:
        // 1. Si el nombre coincide, retornar este nodo
        // 2. Si no, buscar recursivamente en los hijos
        // ↓ Cambia esto por la implementación correcta
        let _ = raiz;
        let _ = nombre;
        None
    }
    
    /// Retorna la ruta completa del nodo (ej: /home/usuario/documento.txt)
    fn ruta_completa(nodo: &NodoRef) -> String {
        // TODO: Navegar hacia arriba construyendo la ruta
        // ↓ Cambia esto por la implementación correcta
        let _ = nodo;
        String::new()
    }
    
    /// Imprime el árbol con indentación
    fn imprimir_arbol(nodo: &NodoRef, nivel: usize) {
        // TODO:
        // 1. Imprimir este nodo con indentación
        // 2. Imprimir recursivamente los hijos
        // ↓ Completa la implementación
        let _ = nodo;
        let _ = nivel;
    }
    
    /// Calcula estadísticas del árbol
    fn estadisticas(raiz: &NodoRef) -> Estadisticas {
        // TODO:
        // 1. Contar directorios y archivos
        // 2. Sumar tamaño de contenido de archivos
        // 3. Recurrir en hijos
        // ↓ Cambia esto por la implementación correcta
        let _ = raiz;
        Estadisticas {
            directorios: 0,
            archivos: 0,
            tamanio_total: 0,
        }
    }
    
    /// Modifica el contenido de un archivo
    fn modificar_contenido(nodo: &NodoRef, nuevo_contenido: &str) {
        // TODO: Si es archivo, actualizar contenido
        // ↓ Completa la implementación
        let _ = nodo;
        let _ = nuevo_contenido;
    }
    
    /// Mueve un nodo a un nuevo padre
    fn mover(nodo: &NodoRef, nuevo_padre: &NodoRef) {
        // TODO:
        // 1. Remover del padre actual
        // 2. Establecer nuevo padre
        // 3. Agregar a hijos del nuevo padre
        // ↓ Completa la implementación
        let _ = nodo;
        let _ = nuevo_padre;
    }
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_estructura() {
        let raiz = FileNode::crear_directorio("/");
        let home = FileNode::crear_directorio("home");
        
        FileNode::agregar_hijo(&raiz, &home);
        
        assert_eq!(raiz.borrow().hijos.len(), 1);
    }
    
    #[test]
    fn test_busqueda() {
        let raiz = FileNode::crear_directorio("/");
        let home = FileNode::crear_directorio("home");
        let archivo = FileNode::crear_archivo("test.txt", "contenido");
        
        FileNode::agregar_hijo(&raiz, &home);
        FileNode::agregar_hijo(&home, &archivo);
        
        let encontrado = FileNode::buscar(&raiz, "test.txt");
        assert!(encontrado.is_some());
    }
    
    #[test]
    fn test_ruta_completa() {
        let raiz = FileNode::crear_directorio("/");
        let home = FileNode::crear_directorio("home");
        let usuario = FileNode::crear_directorio("usuario");
        
        FileNode::agregar_hijo(&raiz, &home);
        FileNode::agregar_hijo(&home, &usuario);
        
        let ruta = FileNode::ruta_completa(&usuario);
        assert_eq!(ruta, "/home/usuario");
    }
    
    #[test]
    fn test_estadisticas() {
        let raiz = FileNode::crear_directorio("/");
        let archivo1 = FileNode::crear_archivo("a.txt", "123");
        let archivo2 = FileNode::crear_archivo("b.txt", "12345");
        
        FileNode::agregar_hijo(&raiz, &archivo1);
        FileNode::agregar_hijo(&raiz, &archivo2);
        
        let stats = FileNode::estadisticas(&raiz);
        assert_eq!(stats.directorios, 1);
        assert_eq!(stats.archivos, 2);
        assert_eq!(stats.tamanio_total, 8);
    }
    
    #[test]
    fn test_mover_nodo() {
        let raiz = FileNode::crear_directorio("/");
        let dir1 = FileNode::crear_directorio("dir1");
        let dir2 = FileNode::crear_directorio("dir2");
        let archivo = FileNode::crear_archivo("test.txt", "contenido");
        
        FileNode::agregar_hijo(&raiz, &dir1);
        FileNode::agregar_hijo(&raiz, &dir2);
        FileNode::agregar_hijo(&dir1, &archivo);
        
        assert_eq!(dir1.borrow().hijos.len(), 1);
        assert_eq!(dir2.borrow().hijos.len(), 0);
        
        FileNode::mover(&archivo, &dir2);
        
        assert_eq!(dir1.borrow().hijos.len(), 0);
        assert_eq!(dir2.borrow().hijos.len(), 1);
    }
}
